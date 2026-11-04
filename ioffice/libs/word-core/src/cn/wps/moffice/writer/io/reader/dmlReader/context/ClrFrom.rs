#![allow(non_snake_case)]
use std::ops::{Deref, DerefMut};
use super::ColorChoice::ColorChoice;

// Java: public class ClrFrom extends ColorChoice
#[derive(Default)]
pub struct ClrFrom { pub base: ColorChoice }
impl Deref for ClrFrom { type Target = ColorChoice; fn deref(&self) -> &Self::Target { &self.base } }
impl DerefMut for ClrFrom { fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base } }
impl ClrFrom {
    // Java: implicit ClrFrom()
    pub fn new() -> Self { Self::default() }
}
