#![allow(non_snake_case)]
use std::ops::{Deref, DerefMut};
use io_wps_io::cn::wps::io::xwpf::SaxAttributes::SaxAttributes;
use util::base::io::TypeConversion::TypeConversion;
use super::ColorTransform::ColorTransform;

// Java: public class ScrgbClr extends ColorTransform
pub struct ScrgbClr {
    pub base: ColorTransform,
    pub mRed: Option<i32>,
    pub mGreen: Option<i32>,
    pub mBlue: Option<i32>,
}
impl Deref for ScrgbClr { type Target = ColorTransform; fn deref(&self) -> &Self::Target { &self.base } }
impl DerefMut for ScrgbClr { fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base } }
impl ScrgbClr {
    // Java: public ScrgbClr(Attributes attrs)
    pub fn new(attrs: &dyn SaxAttributes) -> Self {
        // Java: ConvertHelper.getHex4EmptyNamespace(attrs, "r"/"g"/"b")
        let hex = |name| attrs.getValue_by_uri("", name).and_then(|s| TypeConversion::hexString2Integer(&s));
        Self { base: ColorTransform::default(), mRed: hex("r"), mGreen: hex("g"), mBlue: hex("b") }
    }
}
