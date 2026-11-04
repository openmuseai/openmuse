#![allow(non_snake_case)]
use std::ops::{Deref, DerefMut};
use io_wps_io::cn::wps::io::xwpf::SaxAttributes::SaxAttributes;
use util::base::io::TypeConversion::TypeConversion;
use super::ColorTransform::ColorTransform;

// Java: public class SrgbClr extends ColorTransform
pub struct SrgbClr { pub base: ColorTransform, pub mVal: Option<i32> }
impl Deref for SrgbClr { type Target = ColorTransform; fn deref(&self) -> &Self::Target { &self.base } }
impl DerefMut for SrgbClr { fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base } }
impl SrgbClr {
    // Java: public SrgbClr(Attributes attrs)
    pub fn new(attrs: &dyn SaxAttributes) -> Self {
        // Java: this.mVal = ConvertHelper.getHex4EmptyNamespace(attrs, "val");
        Self { base: ColorTransform::default(), mVal: attrs.getValue_by_uri("", "val").and_then(|s| TypeConversion::hexString2Integer(&s)) }
    }
}
