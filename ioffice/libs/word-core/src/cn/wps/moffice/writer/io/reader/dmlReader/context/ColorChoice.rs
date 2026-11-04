#![allow(non_snake_case)]
use super::{ScrgbClr::ScrgbClr, SrgbClr::SrgbClr, SysClr::SysClr, SchemeClr::SchemeClr, PrstClr::PrstClr};

// Java: public class ColorChoice
#[derive(Default)]
pub struct ColorChoice {
    pub mScrgbClr: Option<ScrgbClr>,
    pub mSrgbClr: Option<SrgbClr>,
    pub mSysClr: Option<SysClr>,
    pub mSchemeClr: Option<SchemeClr>,
    pub mPrstClr: Option<PrstClr>,
}
impl ColorChoice {
    // Java: implicit ColorChoice()
    pub fn new() -> Self { Self::default() }
}
