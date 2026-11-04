#![allow(non_snake_case)]
use std::ops::{Deref, DerefMut};
use super::Cxn::Cxn;

// Java: public class Cxnlst extends ArrayList<Cxn>
#[derive(Default)]
pub struct Cxnlst(pub Vec<Cxn>);
impl Deref for Cxnlst { type Target = Vec<Cxn>; fn deref(&self) -> &Self::Target { &self.0 } }
impl DerefMut for Cxnlst { fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 } }
impl Cxnlst { pub fn new() -> Self { Self::default() } }
