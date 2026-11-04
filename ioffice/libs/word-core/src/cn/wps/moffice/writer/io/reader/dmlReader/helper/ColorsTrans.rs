#![allow(non_snake_case, dead_code)]

use super::{ColorsByName::ColorsByName, HSLHelper::HSLHelper};
use crate::cn::wps::moffice::writer::io::tools::ColorTransform::ColorTransform as IOColorTransform;
use crate::cn::wps::moffice::writer::io::reader::dmlReader::context::{
    ColorChoice::ColorChoice, ColorTransform::ColorTransform, PrstClr::PrstClr,
    SchemeClr::SchemeClr, SrgbClr::SrgbClr, SysClr::SysClr, ThemeContext::ThemeContext,
};

// Java: public class ColorsTrans
pub struct ColorsTrans;

fn argb(r: i32, g: i32, b: i32, alpha: i32) -> i32 {
    IOColorTransform::argb(r, g, b, alpha)
}
fn red(clr: i32) -> i32 { IOColorTransform::getRed(clr) as i32 }
fn green(clr: i32) -> i32 { IOColorTransform::getGreen(clr) as i32 }
fn blue(clr: i32) -> i32 { IOColorTransform::getBlue(clr) as i32 }
fn setRed(clr: i32, r: i32) -> i32 { IOColorTransform::setRed(clr, r) }
fn setGreen(clr: i32, g: i32) -> i32 { IOColorTransform::setGreen(clr, g) }
fn setBlue(clr: i32, b: i32) -> i32 { IOColorTransform::setBlue(clr, b) }

// Java: srgbClr -> prstClr -> schemeClr, selecting the first non-null transform.
fn pick<T: Copy>(s: Option<&SrgbClr>, p: Option<&PrstClr>, c: Option<&SchemeClr>, field: impl Fn(&ColorTransform) -> Option<T>) -> Option<T> {
    s.and_then(|v| field(&v.base)).or_else(|| p.and_then(|v| field(&v.base))).or_else(|| c.and_then(|v| field(&v.base)))
}

impl ColorsTrans {
    // Java: public static Float toAlpha(ColorChoice clrChoice, float sDefaultAlpha)
    pub fn toAlpha(clrChoice: &ColorChoice, sDefaultAlpha: f32) -> Option<f32> {
        let s = clrChoice.mSrgbClr.as_ref();
        let p = clrChoice.mPrstClr.as_ref();
        let c = clrChoice.mSchemeClr.as_ref();
        if s.is_none() && p.is_none() && c.is_none() && clrChoice.mSysClr.is_none() { return None; }
        let alpha = Self::alpha(s, p, c, sDefaultAlpha);
        let alphaOff = Self::alphaOff(s, p, c, sDefaultAlpha);
        let alphaMod = Self::alphaMod(s, p, c, sDefaultAlpha);
        Some(alpha * alphaMod + alphaOff)
    }

    // Java: public static Integer toColor4Theme(ColorChoice clrChoice)
    pub fn toColor4Theme(clrChoice: &ColorChoice) -> Option<i32> { Self::toColor(None, clrChoice, None) }

    // Java: public static Integer toColor(Integer phClr, ColorChoice clrChoice, ThemeContext themeCtx)
    pub fn toColor(phClr: Option<i32>, clrChoice: &ColorChoice, themeCtx: Option<&ThemeContext>) -> Option<i32> {
        let s = clrChoice.mSrgbClr.as_ref();
        let p = clrChoice.mPrstClr.as_ref();
        let c = clrChoice.mSchemeClr.as_ref();
        if s.is_none() && p.is_none() && c.is_none() && clrChoice.mSysClr.is_none() { return None; }
        let mut clr = s.and_then(|v| v.mVal);
        if clr.is_none() { clr = Self::toColorPrstClr(p); }
        if clr.is_none() { clr = Self::toColorSchemeClr(phClr, c, themeCtx); }
        if clr.is_none() { clr = Self::toColorSysClr(clrChoice.mSysClr.as_ref()); }
        let mut clr = clr?;
        if themeCtx.is_some() {
            clr = Self::modifyLum(clr, s, p, c);
            clr = Self::modifyHue(clr, s, p, c);
            clr = Self::modifyRed(clr, s, p, c);
            clr = Self::modifyGreen(clr, s, p, c);
            clr = Self::modifyBlue(clr, s, p, c);
            clr = Self::modifyShade(clr, s, p, c);
            clr = Self::modifyTint(clr, s, p, c);
            clr = Self::modifySat(clr, s, p, c);
        }
        Some(clr)
    }

    // Java: private static int modifyTint(...)
    fn modifyTint(clr: i32, s: Option<&SrgbClr>, p: Option<&PrstClr>, c: Option<&SchemeClr>) -> i32 {
        let Some(tint) = pick(s,p,c,|v| v.mTint) else { return clr; };
        let tmpTint = 1.0 - tint;
        let mut r = red(clr) as f32 / 255.0;
        let mut g = green(clr) as f32 / 255.0;
        let mut b = blue(clr) as f32 / 255.0;
        r = Self::gammaCorrect4(r, 2.2222223, tint, tmpTint) as f32;
        g = Self::gammaCorrect4(g, 2.2222223, tint, tmpTint) as f32;
        b = Self::gammaCorrect4(b, 2.2222223, tint, tmpTint) as f32;
        r = Self::gammaCorrect2(r, 0.45) as f32;
        g = Self::gammaCorrect2(g, 0.45) as f32;
        b = Self::gammaCorrect2(b, 0.45) as f32;
        argb(r as i32, g as i32, b as i32, 255)
    }
    fn modifyShade(mut clr: i32, s: Option<&SrgbClr>, p: Option<&PrstClr>, c: Option<&SchemeClr>) -> i32 {
        let Some(shade) = pick(s,p,c,|v| v.mShade) else { return clr; };
        let r = ((red(clr) as f64) * (shade as f64).powf(0.45_f32 as f64)) as i32;
        let g = ((green(clr) as f64) * (shade as f64).powf(0.45_f32 as f64)) as i32;
        let b = ((blue(clr) as f64) * (shade as f64).powf(0.45_f32 as f64)) as i32;
        clr = argb(r,g,b,255);
        clr
    }
    fn modifyHue(mut clr: i32, s: Option<&SrgbClr>, p: Option<&PrstClr>, c: Option<&SchemeClr>) -> i32 {
        let mut hsl = HSLHelper::new(clr);
        if let Some(hue) = pick(s,p,c,|v| v.mHue) {
            let hue = hue / 216; // Java integer division before float conversion.
            hsl.setH((hue as f32).min(1.0));
            clr = hsl.toRGB();
        }
        clr = Self::modifyHueByMod(clr, &mut hsl, s,p,c);
        clr = Self::modifyHueByOff(clr, &mut hsl, s,p,c);
        clr
    }
    fn modifyHueByMod(clr: i32, hsl: &mut HSLHelper, s: Option<&SrgbClr>, p: Option<&PrstClr>, c: Option<&SchemeClr>) -> i32 {
        let Some(hueMod) = pick(s,p,c,|v| v.mHueMod) else { return clr; };
        hsl.setH((hsl.H() * hueMod).min(1.0)); hsl.toRGB()
    }
    fn modifyHueByOff(clr: i32, hsl: &mut HSLHelper, s: Option<&SrgbClr>, p: Option<&PrstClr>, c: Option<&SchemeClr>) -> i32 {
        let Some(hueOff) = pick(s,p,c,|v| v.mHueOff) else { return clr; };
        let hueOff = hueOff / 216;
        hsl.setH((hsl.H() + hueOff as f32).min(1.0)); hsl.toRGB()
    }
    fn modifySat(mut clr: i32, s: Option<&SrgbClr>, p: Option<&PrstClr>, c: Option<&SchemeClr>) -> i32 {
        let mut hsl = HSLHelper::new(clr);
        if let Some(sat) = pick(s,p,c,|v| v.mSat) { hsl.setS(sat.min(1.0)); clr = hsl.toRGB(); }
        clr = Self::modifySatByMod(clr, &mut hsl, s,p,c);
        clr = Self::modifySatByOff(clr, &mut hsl, s,p,c);
        clr
    }
    fn modifySatByMod(clr: i32, hsl: &mut HSLHelper, s: Option<&SrgbClr>, p: Option<&PrstClr>, c: Option<&SchemeClr>) -> i32 {
        let Some(satMod) = pick(s,p,c,|v| v.mSatMod) else { return clr; };
        hsl.setS(hsl.S() * satMod); hsl.toRGB()
    }
    fn modifySatByOff(clr: i32, hsl: &mut HSLHelper, s: Option<&SrgbClr>, p: Option<&PrstClr>, c: Option<&SchemeClr>) -> i32 {
        let Some(satOff) = pick(s,p,c,|v| v.mSatOff) else { return clr; };
        hsl.setS((hsl.S() + satOff).min(1.0)); hsl.toRGB()
    }
    fn modifyLum(mut clr: i32, s: Option<&SrgbClr>, p: Option<&PrstClr>, c: Option<&SchemeClr>) -> i32 {
        let mut hsl = HSLHelper::new(clr);
        if let Some(lum) = pick(s,p,c,|v| v.mLum) { hsl.setL(lum.min(1.0)); clr = hsl.toRGB(); }
        clr = Self::modifyLumByMod(clr, &mut hsl, s,p,c);
        clr = Self::modifyLumByOff(clr, &mut hsl, s,p,c);
        clr
    }
    fn modifyLumByMod(clr: i32, hsl: &mut HSLHelper, s: Option<&SrgbClr>, p: Option<&PrstClr>, c: Option<&SchemeClr>) -> i32 {
        let Some(lumMod) = pick(s,p,c,|v| v.mLumMod) else { return clr; };
        hsl.setL((hsl.L() * lumMod).min(1.0)); hsl.toRGB()
    }
    fn modifyLumByOff(clr: i32, hsl: &mut HSLHelper, s: Option<&SrgbClr>, p: Option<&PrstClr>, c: Option<&SchemeClr>) -> i32 {
        let Some(lumOff) = pick(s,p,c,|v| v.mLumOff) else { return clr; };
        hsl.setL((hsl.L() + lumOff).min(1.0)); hsl.toRGB()
    }

    fn modifyGreen(clr: i32, s: Option<&SrgbClr>, p: Option<&PrstClr>, c: Option<&SchemeClr>) -> i32 {
        let value = pick(s,p,c,|v| v.mGreen).map(|v| Self::gammaCorrect2(v,0.45));
        let mut channel = value.map(|v| v as i32).unwrap_or_else(|| green(clr));
        channel = Self::modifyGreenByMod(channel,s,p,c);
        channel = Self::modifyGreenByOff(channel,s,p,c);
        channel = (channel as f32).min(255.0) as i32;
        setGreen(clr,channel)
    }
    fn modifyGreenByMod(channel: i32, s: Option<&SrgbClr>, p: Option<&PrstClr>, c: Option<&SchemeClr>) -> i32 {
        pick(s,p,c,|v| v.mGreenMod).map(|v| Self::gammaCorrect3(v,0.45,channel as f32) as i32).unwrap_or(channel)
    }
    fn modifyGreenByOff(channel: i32, s: Option<&SrgbClr>, p: Option<&PrstClr>, c: Option<&SchemeClr>) -> i32 {
        pick(s,p,c,|v| v.mGreenOff).map(|v| Self::gammaCorrect2(Self::adjustOff(v,channel),0.45) as i32).unwrap_or(channel)
    }
    fn modifyBlue(clr: i32, s: Option<&SrgbClr>, p: Option<&PrstClr>, c: Option<&SchemeClr>) -> i32 {
        let value = pick(s,p,c,|v| v.mBlue).map(|v| Self::gammaCorrect2(v,0.45));
        let mut channel = value.map(|v| v as i32).unwrap_or_else(|| blue(clr));
        channel = Self::modifyBlueByMod(channel,s,p,c);
        channel = Self::modifyBlueByOff(channel,s,p,c);
        channel = (channel as f32).min(255.0) as i32;
        setBlue(clr,channel)
    }
    fn modifyBlueByMod(channel: i32, s: Option<&SrgbClr>, p: Option<&PrstClr>, c: Option<&SchemeClr>) -> i32 {
        pick(s,p,c,|v| v.mBlueMod).map(|v| Self::gammaCorrect3(v,0.45,channel as f32) as i32).unwrap_or(channel)
    }
    fn modifyBlueByOff(channel: i32, s: Option<&SrgbClr>, p: Option<&PrstClr>, c: Option<&SchemeClr>) -> i32 {
        pick(s,p,c,|v| v.mBlueOff).map(|v| Self::gammaCorrect2(Self::adjustOff(v,channel),0.45) as i32).unwrap_or(channel)
    }
    fn modifyRed(clr: i32, s: Option<&SrgbClr>, p: Option<&PrstClr>, c: Option<&SchemeClr>) -> i32 {
        let value = pick(s,p,c,|v| v.mRed).map(|v| Self::gammaCorrect2(v,0.45));
        let mut channel = value.map(|v| v as i32).unwrap_or_else(|| red(clr));
        channel = Self::modifyRedByMod(channel,s,p,c);
        channel = Self::modifyRedByOff(channel,s,p,c);
        channel = (channel as f32).min(255.0) as i32;
        setRed(clr,channel)
    }
    fn modifyRedByMod(channel: i32, s: Option<&SrgbClr>, p: Option<&PrstClr>, c: Option<&SchemeClr>) -> i32 {
        pick(s,p,c,|v| v.mRedMod).map(|v| Self::gammaCorrect3(v,0.45,channel as f32) as i32).unwrap_or(channel)
    }
    fn modifyRedByOff(channel: i32, s: Option<&SrgbClr>, p: Option<&PrstClr>, c: Option<&SchemeClr>) -> i32 {
        pick(s,p,c,|v| v.mRedOff).map(|v| Self::gammaCorrect2(Self::adjustOff(v,channel),0.45) as i32).unwrap_or(channel)
    }
    // Java: private static float adjustOff(float off, int clr)
    fn adjustOff(off: f32, clr: i32) -> f32 { ((off as f64) + ((clr as f32 / 255.0) as f64).powf(2.2222223_f32 as f64)) as f32 }
    fn alphaMod(s: Option<&SrgbClr>, p: Option<&PrstClr>, c: Option<&SchemeClr>, _sDefaultAlpha: f32) -> f32 { pick(s,p,c,|v| v.mAlphaMod).unwrap_or(1.0) }
    fn alphaOff(s: Option<&SrgbClr>, p: Option<&PrstClr>, c: Option<&SchemeClr>, _sDefaultAlpha: f32) -> f32 { pick(s,p,c,|v| v.mAlphaOff).unwrap_or(0.0) }
    fn alpha(s: Option<&SrgbClr>, p: Option<&PrstClr>, c: Option<&SchemeClr>, sDefaultAlpha: f32) -> f32 { pick(s,p,c,|v| v.mAlpha).unwrap_or(sDefaultAlpha) }

    // Java: private static Integer toColor(Integer phClr, SchemeClr schemeClr, ThemeContext themeCtx)
    fn toColorSchemeClr(phClr: Option<i32>, schemeClr: Option<&SchemeClr>, themeCtx: Option<&ThemeContext>) -> Option<i32> {
        let (Some(schemeClr), Some(themeCtx)) = (schemeClr, themeCtx) else { return None; };
        let val = schemeClr.mVal?;
        match val {
            -1177623385..=-1177623380 | 97420 | 97421 | 99466 | 99467 | 107433 | 107434 | 115245 | 115246 | 99368034 | 268452191 => themeCtx.mClrScheme.get(&val).copied(),
            106600465 => phClr,
            _ => None,
        }
    }
    // Java: public static Integer toColor(PrstClr prstClr)
    pub fn toColorPrstClr(prstClr: Option<&PrstClr>) -> Option<i32> { prstClr.and_then(|v| v.mVal).and_then(ColorsByName::toClr) }
    // Java: private static Integer toColor(SysClr sysClr)
    fn toColorSysClr(sysClr: Option<&SysClr>) -> Option<i32> { sysClr.and_then(|v| v.mSysLastClr) }
    fn gammaCorrect4(input: f32, gamma: f32, f: f32, cp: f32) -> f64 { (f as f64) * (input as f64).powf(gamma as f64) + cp as f64 }
    fn gammaCorrect3(input: f32, gamma: f32, f: f32) -> f64 { Self::gammaCorrect4(input,gamma,f,0.5) }
    fn gammaCorrect2(input: f32, gamma: f32) -> f64 { Self::gammaCorrect3(input,gamma,255.0) }
    // Java: public static int rgb2Argb(int rgb, float alpha)
    pub fn rgb2Argb(rgb: i32, alpha: f32) -> i32 {
        let _alpha = (255.0_f32 * alpha) as i32 as i16;
        (rgb & 0x00ff_ffff) | ((_alpha as i32) << 24)
    }
    // Java: public static float toTransparency(ColorChoice clrChoice, boolean isText)
    pub fn toTransparency(clrChoice: &ColorChoice, isText: bool) -> f32 {
        let defaultAlpha = if isText { 0.0 } else { 1.0 };
        let mut alpha = Self::toAlpha(clrChoice, defaultAlpha).unwrap_or(defaultAlpha);
        if !isText { alpha = (1.0_f64 - alpha as f64) as f32; }
        alpha
    }
}
