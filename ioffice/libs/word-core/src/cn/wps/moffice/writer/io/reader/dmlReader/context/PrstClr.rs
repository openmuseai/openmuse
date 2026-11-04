#![allow(non_snake_case)]
use std::ops::{Deref, DerefMut};
use io_wps_io::cn::wps::io::xwpf::SaxAttributes::SaxAttributes;
use io_wps_io::cn::wps::io::dom::io::WMap::WMap;
use super::{ColorTransform::ColorTransform, SchemeClr::java_hash};

// Java: public class PrstClr extends ColorTransform
pub struct PrstClr { pub base: ColorTransform, pub mVal: Option<i32> }
impl Deref for PrstClr { type Target = ColorTransform; fn deref(&self) -> &Self::Target { &self.base } }
impl DerefMut for PrstClr { fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base } }
impl PrstClr {
    // Java: public PrstClr(Attributes attributes)
    pub fn new(attributes: &dyn SaxAttributes) -> Self {
        // Java: String tmp = ConvertHelper.getString(attributes, "val"); this.mVal = null == tmp ? null : tmp.hashCode();
        let tmp = if attributes.getLength() == 0 { None } else { attributes.getValue_by_uri(WMap::MAIN_NAMESPACING(), "val") };
        Self { base: ColorTransform::default(), mVal: tmp.as_deref().map(java_hash) }
    }
}
