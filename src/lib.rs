//! # `allow-until`
//!
//! Allows an item until a specified semver version, and then errors on compilation.
//!
//! [![github]](https://github.com/DexterHill0/allow-until)&ensp;[![crates-io]](https://crates.io/crates/allow-until)&ensp;[![docs-rs]](https://docs.rs/allow-until)
//!
//! [github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
//! [crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
//! [docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs
//!
//! ```rust
//! #[allow_until(version = ">= 1.0.x", reason = "struct is deprecated from version 1.0.x onwards")]
//! struct MyStruct {
//!     //....
//! }
//! ```
//! Or with the derive macro:
//! ```rust
//! #[derive(AllowUntil)]
//! struct MyStruct {
//!     #[allow_until(version = ">= 1.0.x", reason = "member is deprecated from version 1.0.x onwards")]
//!     foo: usize
//! }
//! ```

#![feature(proc_macro_diagnostic, proc_macro_span)]

use proc_macro::{TokenTree as TT, *};
use semver::{Version, VersionReq};

struct Args {
    pub version: VersionReq,
    pub reason: Option<String>,
}

fn parse_arguments(args: TokenStream) -> Result<Args, Diagnostic> {
    let mut toks = args.into_iter().peekable();

    let mut version = None;
    let mut reason = None;

    while let Some(tok) = toks.next() {
        let ident = match tok {
            TT::Ident(ident) => ident,
            t => {
                return Err(t
                    .span()
                    .error("expected ident")
                    .help("valid arguments are `version` and `reason`"));
            }
        };

        match toks.next() {
            Some(TT::Punct(p)) if p.as_char() == '=' => (),
            Some(t) => return Err(t.span().error("expected `=`")),
            None => {
                return Err(Span::call_site()
                    .error("unexpected end of tokens")
                    .help("expected `=`"))
            }
        }

        let lit = match toks.next() {
            Some(TT::Literal(lit)) => lit,
            Some(t) => return Err(t.span().error("expected literal")),
            None => {
                return Err(Span::call_site()
                    .error("unexpected end of tokens")
                    .help("expected literal"))
            }
        };

        match &ident.to_string()[..] {
            "version" => {
                let lit_str = lit.to_string();

                let v = lit_str
                    .get(1..lit_str.len() - 1)
                    .ok_or(lit.span().error("expected string literal"))?;

                version = Some(
                    VersionReq::parse(v).map_err(|_| lit.span().error("invalid semver version"))?,
                );
            }
            "reason" => {
                let lit_str = lit.to_string();

                let v = lit_str
                    .get(1..lit_str.len() - 1)
                    .ok_or(lit.span().error("expected string literal"))?;

                reason = Some(v.into());
            }
            _ => {
                return Err(lit
                    .span()
                    .error("unknown argument")
                    .help("valid arguments are `version` and `reason`"))
            }
        }

        match toks.peek() {
            Some(TT::Punct(p)) if p.as_char() == ',' => {
                toks.next();
            }
            Some(t) => {
                return Err(t
                    .span()
                    .error("unexpected token")
                    .help("expected end of tokens or `,`"))
            }
            None => {}
        }
    }

    if version.is_none() {
        return Err(Span::call_site().error("missing required `version` argument"));
    }

    Ok(Args {
        reason,
        version: version.unwrap(),
    })
}

fn emit_error_version_match(pred: VersionReq, reason: Option<String>, at: Span) {
    if let Ok(pkg_ver) = std::env::var("CARGO_PKG_VERSION") {
        let version = Version::parse(&pkg_ver).expect("invalid cargo semver ver");

        if pred.matches(&version) {
            at.error(reason.map_or(
                format!("item not allowed! (version {} matches {})", version, pred),
                |r| format!("{} (version {} matches {})", r, version, pred),
            ))
            .emit();
        }
    }
}

fn recurse_find_attr(group: Group) {
    let mut toks = group.stream().into_iter();

    loop {
        match toks.next() {
            Some(TT::Group(g)) => recurse_find_attr(g),
            Some(TT::Punct(hash)) if hash.as_char() == '#' => match toks.next() {
                Some(TT::Group(inner_g)) => {
                    let mut toks = inner_g.stream().into_iter();

                    match toks.next() {
                        Some(TT::Ident(ident)) if &ident.to_string()[..] == "allow_until" => {
                            match toks.next() {
                                Some(TT::Group(g)) => {
                                    let args = parse_arguments(g.stream());
                                    let args = match args {
                                        Err(e) => {
                                            e.emit();
                                            return;
                                        }
                                        Ok(a) => a,
                                    };

                                    emit_error_version_match(
                                        args.version,
                                        args.reason,
                                        hash.span()
                                            .join(inner_g.span())
                                            .unwrap()
                                            .join(ident.span())
                                            .unwrap(),
                                    );

                                    continue;
                                }
                                _ => continue,
                            }
                        }
                        _ => continue,
                    }
                }
                _ => continue,
            },
            None => break,
            _ => continue,
        }
    }
}

/// Allows an item until a specified semver version, and then errors on compilation.
///
/// ```rust
/// #[allow_until(version = ">= 1.0.x", reason = "struct is deprecated from version 1.0.x onwards")]
/// struct MyStruct {
///     //....
/// }
/// ```
#[proc_macro_attribute]
pub fn allow_until(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_arguments(args);

    let args = match args {
        Err(e) => {
            e.emit();
            return input;
        }
        Ok(a) => a,
    };

    emit_error_version_match(args.version, args.reason, Span::call_site());

    input
}

/// Allows an item until a specified semver version, and then errors on compilation.
///
/// ```rust
/// #[derive(AllowUntil)]
/// struct MyStruct {
///     #[allow_until(version = ">= 1.0.x", reason = "member is deprecated from version 1.0.x onwards")]
///     foo: usize
/// }
/// ```
#[proc_macro_derive(AllowUntil, attributes(allow_until))]
pub fn allow_until_derive(stream: TokenStream) -> TokenStream {
    let toks = stream.into_iter();

    for tok in toks {
        match tok {
            TT::Group(g) => recurse_find_attr(g),
            _ => continue,
        }
    }

    TokenStream::new()
}
