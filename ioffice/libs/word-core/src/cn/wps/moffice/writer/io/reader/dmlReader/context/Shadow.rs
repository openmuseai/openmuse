#![allow(non_snake_case)]
use std::ops::{Deref, DerefMut};
use io_wps_io::cn::wps::io::{dom::io::WMap::WMap, xwpf::SaxAttributes::SaxAttributes};
use super::InnerShdw::InnerShdw;

// Java: public class Shadow extends InnerShdw
pub struct Shadow {
    pub base: InnerShdw,
    pub mSX: Option<String>, pub mSY: Option<String>,
    pub mKX: Option<String>, pub mKY: Option<String>, pub mAlgn: Option<String>,
}
impl Deref for Shadow { type Target = InnerShdw; fn deref(&self) -> &Self::Target { &self.base } }
impl DerefMut for Shadow { fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base } }
impl Shadow {
    // Java: public Shadow(Attributes attrs)
    pub fn new(attrs: &dyn SaxAttributes) -> Self {
        // Java: ConvertHelper.getString(attrs, name) reads MAIN_NAMESPACING.
        let get = |name| if attrs.getLength() == 0 { None } else { attrs.getValue_by_uri(WMap::MAIN_NAMESPACING(), name) };
        Self { base: InnerShdw::new(attrs), mSX: get("sx"), mSY: get("sy"), mKX: get("kx"), mKY: get("ky"), mAlgn: get("algn") }
    }
}
