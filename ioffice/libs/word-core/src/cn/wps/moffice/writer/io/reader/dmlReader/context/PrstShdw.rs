#![allow(non_snake_case)]
use std::ops::{Deref, DerefMut};
use io_wps_io::cn::wps::io::{dom::io::WMap::WMap, xwpf::SaxAttributes::SaxAttributes};
use super::{OuterShdw::OuterShdw, SchemeClr::java_hash};

// Java: public class PrstShdw extends OuterShdw
pub struct PrstShdw { pub base: OuterShdw, pub mPrst: Option<i32> }
impl Deref for PrstShdw { type Target = OuterShdw; fn deref(&self) -> &Self::Target { &self.base } }
impl DerefMut for PrstShdw { fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base } }
impl PrstShdw {
    // Java: public PrstShdw(Attributes attrs)
    pub fn new(attrs: &dyn SaxAttributes) -> Self {
        let prst = if attrs.getLength() == 0 { None } else { attrs.getValue_by_uri(WMap::MAIN_NAMESPACING(), "prst") };
        Self { base: OuterShdw::new(attrs), mPrst: prst.as_deref().map(java_hash) }
    }
}
