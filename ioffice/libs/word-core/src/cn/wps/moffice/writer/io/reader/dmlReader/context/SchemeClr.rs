#![allow(non_snake_case)]
use std::ops::{Deref, DerefMut};
use io_wps_io::cn::wps::io::xwpf::SaxAttributes::SaxAttributes;
use io_wps_io::cn::wps::io::dom::io::WMap::WMap;
use super::ColorTransform::ColorTransform;

// Java: public class SchemeClr extends ColorTransform
pub struct SchemeClr { pub base: ColorTransform, pub mVal: Option<i32> }
impl Deref for SchemeClr { type Target = ColorTransform; fn deref(&self) -> &Self::Target { &self.base } }
impl DerefMut for SchemeClr { fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base } }
impl SchemeClr {
    // Java: public SchemeClr(Attributes attributes)
    pub fn new(attributes: &dyn SaxAttributes) -> Self {
        // Java: String tmp = ConvertHelper.getString(attributes, "val"); if (null != tmp) this.mVal = tmp.hashCode();
        let tmp = if attributes.getLength() == 0 { None } else { attributes.getValue_by_uri(WMap::MAIN_NAMESPACING(), "val") };
        Self { base: ColorTransform::default(), mVal: tmp.as_deref().map(java_hash) }
    }
}
pub(super) fn java_hash(value: &str) -> i32 {
    value.encode_utf16().fold(0_i32, |hash, ch| hash.wrapping_mul(31).wrapping_add(ch as i32))
}
