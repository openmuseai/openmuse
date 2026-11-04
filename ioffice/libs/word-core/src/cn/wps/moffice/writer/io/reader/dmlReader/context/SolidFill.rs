#![allow(non_snake_case)]
use std::ops::{Deref, DerefMut};
use super::ColorChoice::ColorChoice;

// Java: public class SolidFill extends ColorChoice
#[derive(Default)]
pub struct SolidFill { pub base: ColorChoice }
impl Deref for SolidFill { type Target = ColorChoice; fn deref(&self) -> &Self::Target { &self.base } }
impl DerefMut for SolidFill { fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base } }
impl SolidFill {
    // Java: implicit SolidFill()
    pub fn new() -> Self { Self::default() }
}
