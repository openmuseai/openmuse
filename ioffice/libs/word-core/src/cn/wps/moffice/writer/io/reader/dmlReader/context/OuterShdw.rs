#![allow(non_snake_case)]
use std::ops::{Deref, DerefMut};
use io_wps_io::cn::wps::io::xwpf::SaxAttributes::SaxAttributes;
use super::Shadow::Shadow;

// Java: public class OuterShdw extends Shadow
pub struct OuterShdw { pub base: Shadow, pub mRotWithShape: Option<bool> }
impl Deref for OuterShdw { type Target = Shadow; fn deref(&self) -> &Self::Target { &self.base } }
impl DerefMut for OuterShdw { fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base } }
impl OuterShdw {
    // Java: public OuterShdw(Attributes attrs)
    pub fn new(attrs: &dyn SaxAttributes) -> Self {
        // Java: ConvertHelper.toBoolean(attrs, "rotWithShape") uses empty namespace.
        let mRotWithShape = attrs.getValue_by_uri("", "rotWithShape").map(|s| {
            !(s.eq_ignore_ascii_case("off") || s == "0" || s.eq_ignore_ascii_case("false") || s.eq_ignore_ascii_case("f"))
        });
        Self { base: Shadow::new(attrs), mRotWithShape }
    }
}
