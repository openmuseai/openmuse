#![allow(non_snake_case)]
use std::ops::{Deref, DerefMut};
use super::{PrstClr::PrstClr, SchemeClr::SchemeClr, ScrgbClr::ScrgbClr,
    SrgbClr::SrgbClr, SysClr::SysClr};

// Java: public class ColorTransform
#[derive(Default, Clone)]
pub struct ColorTransform {
    pub mTint: Option<f32>,
    pub mShade: Option<f32>,
    pub mComp: Option<f32>,
    pub mInv: Option<f32>,
    pub mGray: Option<f32>,
    pub mAlpha: Option<f32>,
    pub mAlphaOff: Option<f32>,
    pub mAlphaMod: Option<f32>,
    pub mRed: Option<f32>,
    pub mRedOff: Option<f32>,
    pub mRedMod: Option<f32>,
    pub mGreen: Option<f32>,
    pub mGreenOff: Option<f32>,
    pub mGreenMod: Option<f32>,
    pub mBlue: Option<f32>,
    pub mBlueOff: Option<f32>,
    pub mBlueMod: Option<f32>,
    pub mHue: Option<i32>,
    pub mHueOff: Option<i32>,
    pub mHueMod: Option<f32>,
    pub mSat: Option<f32>,
    pub mSatOff: Option<f32>,
    pub mSatMod: Option<f32>,
    pub mLum: Option<f32>,
    pub mLumOff: Option<f32>,
    pub mLumMod: Option<f32>,
    pub mGamma: Option<f32>,
    pub mInvGamma: Option<f32>,
}
impl ColorTransform {
    // Java: implicit ColorTransform()
    pub fn new() -> Self { Self::default() }
}

// Java: DmlShapeCtxt.mClrTransform is a ColorTransform reference whose runtime
// object is one of the five derived color classes. Preserve that runtime type
// until the corresponding set*Clr method casts and transfers the object.
pub enum ColorTransformValue {
    SysClr(SysClr),
    PrstClr(PrstClr),
    SchemeClr(SchemeClr),
    SrgbClr(SrgbClr),
    ScrgbClr(ScrgbClr),
}
impl Deref for ColorTransformValue {
    type Target = ColorTransform;
    fn deref(&self) -> &Self::Target {
        match self {
            Self::SysClr(v) => &v.base, Self::PrstClr(v) => &v.base,
            Self::SchemeClr(v) => &v.base, Self::SrgbClr(v) => &v.base,
            Self::ScrgbClr(v) => &v.base,
        }
    }
}
impl DerefMut for ColorTransformValue {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            Self::SysClr(v) => &mut v.base, Self::PrstClr(v) => &mut v.base,
            Self::SchemeClr(v) => &mut v.base, Self::SrgbClr(v) => &mut v.base,
            Self::ScrgbClr(v) => &mut v.base,
        }
    }
}
