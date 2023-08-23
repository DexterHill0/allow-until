use allow_until::{allow_until, AllowUntil};

#[allow(unused)]
#[test]
fn test() {
    #[derive(AllowUntil)]
    struct Foo {
        #[allow_until(version = ">=1.0.x")]
        a: usize,
        b: usize,
    }

    #[allow_until(version = ">=1.0.x", reason = "for fun!")]
    struct Bar {
        a: usize,
        b: usize,
    }
}
