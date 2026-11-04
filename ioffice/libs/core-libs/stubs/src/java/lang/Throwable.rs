#![allow(non_snake_case)]
#[derive(Default, Debug)]
pub struct Throwable {}
impl Throwable {
    pub fn getMessage(&self) -> String { String::new() }
}
