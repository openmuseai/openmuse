#![allow(non_snake_case)]
use std::ops::{Deref, DerefMut};
use super::ColorChoice::ColorChoice;

// Java: public class BgClr extends ColorChoice
#[derive(Default)]
pub struct BgClr { pub base: ColorChoice }
impl Deref for BgClr { type Target = ColorChoice; fn deref(&self) -> &Self::Target { &self.base } }
impl DerefMut for BgClr { fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base } }
impl BgClr {
    // Java: implicit BgClr()
    pub fn new() -> Self { Self::default() }
}
