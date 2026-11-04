#![allow(non_snake_case)]
use std::ops::{Deref, DerefMut};
use io_wps_io::cn::wps::io::{dom::io::WMap::WMap, xwpf::SaxAttributes::SaxAttributes};
use super::ColorChoice::ColorChoice;
use crate::cn::wps::moffice::writer::io::reader::helper::IOHelper::IOHelper;

// Java: public class Gs extends ColorChoice
pub struct Gs { pub base: ColorChoice, pub mPos: Option<f32> }
impl Deref for Gs { type Target = ColorChoice; fn deref(&self) -> &Self::Target { &self.base } }
impl DerefMut for Gs { fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base } }
impl Gs {
    // Java: public Gs(Attributes attrs)
    pub fn new(attrs: &dyn SaxAttributes) -> Self {
        // Java: ConvertHelper.getString(attrs, "pos") -> IOHelper.PositiveFixedPercentage2Float(value)
        let value = if attrs.getLength() == 0 { None } else { attrs.getValue_by_uri(WMap::MAIN_NAMESPACING(), "pos") };
        Self { base: ColorChoice::new(), mPos: IOHelper::PositiveFixedPercentage2Float(value.as_deref()) }
    }
}
