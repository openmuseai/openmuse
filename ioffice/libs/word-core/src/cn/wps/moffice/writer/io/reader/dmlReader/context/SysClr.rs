#![allow(non_snake_case)]
use std::ops::{Deref, DerefMut};
use io_wps_io::cn::wps::io::xwpf::SaxAttributes::SaxAttributes;
use io_wps_io::cn::wps::io::dom::io::WMap::WMap;
use util::base::io::TypeConversion::TypeConversion;
use super::ColorTransform::ColorTransform;

// Java: public class SysClr extends ColorTransform
pub struct SysClr { pub base: ColorTransform, pub mSysClr: Option<String>, pub mSysLastClr: Option<i32> }
impl Deref for SysClr { type Target = ColorTransform; fn deref(&self) -> &Self::Target { &self.base } }
impl DerefMut for SysClr { fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base } }
impl SysClr {
    // Java: public SysClr(Attributes attrs)
    pub fn new(attrs: &dyn SaxAttributes) -> Self {
        // Java: this.mSysClr = ConvertHelper.getString(attrs, "val");
        let mSysClr = if attrs.getLength() == 0 { None } else { attrs.getValue_by_uri(WMap::MAIN_NAMESPACING(), "val") };
        // Java: this.mSysLastClr = ConvertHelper.getHex4EmptyNamespace(attrs, "lastClr");
        let mSysLastClr = attrs.getValue_by_uri("", "lastClr").and_then(|s| TypeConversion::hexString2Integer(&s));
        Self { base: ColorTransform::default(), mSysClr, mSysLastClr }
    }
}
