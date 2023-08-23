# `allow-until`

Allows an item until a specified semver version, and then errors on compilation.

[![github]](https://github.com/DexterHill0/allow-until)&ensp;[![crates-io]](https://crates.io/crates/allow-until)&ensp;[![docs-rs]](https://docs.rs/allow-until)

[github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
[crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
[docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs

```rust
#[allow_until(version = ">= 1.0.x", reason = "struct is deprecated from version 1.0.x onwards")]
struct MyStruct {
    //....
}
```

Or with the derive macro:

```rust
#[derive(AllowUntil)]
struct MyStruct {
    #[allow_until(version = ">= 1.0.x", reason = "member is deprecated from version 1.0.x onwards")]
    foo: usize
}
```

Once the `CARGO_PKG_VERSION` matches the given semver predicate, the macro will cause a compilation error, therefore reminding you to update/remove the code.
