#![allow(non_snake_case)]
use std::ops::{Deref, DerefMut};
use io_wps_io::cn::wps::io::xwpf::SaxAttributes::SaxAttributes;
use util::base::io::{MetricUtility::MetricUtility, TypeConversion::TypeConversion};
use super::ColorChoice::ColorChoice;

// Java: public class Glow extends ColorChoice
pub struct Glow { pub base: ColorChoice, pub mRad: Option<i32> }
impl Deref for Glow { type Target = ColorChoice; fn deref(&self) -> &Self::Target { &self.base } }
impl DerefMut for Glow { fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base } }
impl Glow {
    // Java: public Glow(Attributes attrs)
    pub fn new(attrs: &dyn SaxAttributes) -> Self {
        // Java: ConvertHelper.getLong4EmptyNamespace(attrs, "rad")
        let valueLong = attrs.getValue_by_uri("", "rad").and_then(|s| TypeConversion::string2Long(&s));
        // Java: (int)MetricUtility.EMU2Points((float)valueLong)
        let mRad = valueLong.map(|v| MetricUtility::EMU2Points(v as f32) as i32);
        Self { base: ColorChoice::new(), mRad }
    }
}
