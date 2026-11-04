pub struct Foo {
    x: i32,
}

impl Foo {
    fn add(&self) -> i32 {
        let a = 10;
        let b = self.x;
        a + b
    }
}