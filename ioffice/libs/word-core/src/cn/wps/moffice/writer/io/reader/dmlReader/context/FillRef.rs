#![allow(non_snake_case)]
use std::ops::{Deref, DerefMut};
use io_wps_io::cn::wps::io::xwpf::SaxAttributes::SaxAttributes;
use util::base::io::TypeConversion::TypeConversion;
use super::ColorChoice::ColorChoice;

// Java: public class FillRef extends ColorChoice
pub struct FillRef { pub base: ColorChoice, pub mIdx: Option<i32> }
impl Deref for FillRef { type Target = ColorChoice; fn deref(&self) -> &Self::Target { &self.base } }
impl DerefMut for FillRef { fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base } }
impl FillRef {
    // Java: FillRef(Attributes attrs)
    pub fn new(attrs: &dyn SaxAttributes) -> Self {
        // Java: ConvertHelper.getInt4EmptyNamespace(attrs, "idx")
        let mIdx = attrs.getValue_by_uri("", "idx").and_then(|s| TypeConversion::string2Integer(&s));
        Self { base: ColorChoice::new(), mIdx }
    }
}
