#![allow(non_snake_case)]
#[derive(Default, Debug, Clone)]
pub struct IllegalArgumentException;
impl IllegalArgumentException {
    pub fn new() -> Self { Self }
}
