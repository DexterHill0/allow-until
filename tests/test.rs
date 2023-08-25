use allow_until::{allow_until as allow_until_attr, AllowUntil};

#[allow(unused)]
#[test]
fn test() {
    #[derive(AllowUntil)]
    struct Foo {
        #[allow_until(version = ">=1.0.x")]
        a: usize,
        b: usize,
    }

    #[allow_until_attr(version = ">=1.0.x", reason = "for fun!")]
    struct Bar {
        a: usize,
        b: usize,
    }

    #[derive(AllowUntil)]
    enum FooBar {
        #[allow_until(version = ">=1.0.x")]
        A,
        B(#[allow_until(version = ">=1.0.x")] usize),
        C {
            #[allow_until(version = ">=1.0.x")]
            bar: bool,
        },
    }
}
