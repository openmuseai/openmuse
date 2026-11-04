#![allow(non_snake_case)]
use std::ops::{Deref, DerefMut};
use super::ColorChoice::ColorChoice;

// Java: public class FgClr extends ColorChoice
#[derive(Default)]
pub struct FgClr { pub base: ColorChoice }
impl Deref for FgClr { type Target = ColorChoice; fn deref(&self) -> &Self::Target { &self.base } }
impl DerefMut for FgClr { fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base } }
impl FgClr {
    // Java: implicit FgClr()
    pub fn new() -> Self { Self::default() }
}
