#![allow(non_snake_case)]
use std::ops::{Deref, DerefMut};
use io_wps_io::cn::wps::io::xwpf::SaxAttributes::SaxAttributes;
use util::base::io::TypeConversion::TypeConversion;
use super::ColorChoice::ColorChoice;

// Java: public class InnerShdw extends ColorChoice
pub struct InnerShdw {
    pub base: ColorChoice,
    pub mBlurRad: Option<i64>,
    pub mDist: Option<i64>,
    pub mDir: Option<i32>,
}
impl Deref for InnerShdw { type Target = ColorChoice; fn deref(&self) -> &Self::Target { &self.base } }
impl DerefMut for InnerShdw { fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base } }
impl InnerShdw {
    // Java: public InnerShdw(Attributes attrs)
    pub fn new(attrs: &dyn SaxAttributes) -> Self {
        Self {
            base: ColorChoice::new(),
            mBlurRad: attrs.getValue_by_uri("", "blurRad").and_then(|s| TypeConversion::string2Long(&s)),
            mDist: attrs.getValue_by_uri("", "dist").and_then(|s| TypeConversion::string2Long(&s)),
            mDir: attrs.getValue_by_uri("", "dir").and_then(|s| TypeConversion::string2Integer(&s)),
        }
    }
}
