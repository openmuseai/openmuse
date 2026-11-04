#![allow(non_snake_case, dead_code, unused_variables, non_upper_case_globals, unused_imports)]

// Java: import cn.wps.font.impl.BuildinGlyphMetrics
use crate::cn::wps::font::r#impl::BuildinGlyphMetrics::{self, IMetricsGetter};
// Java: import cn.wps.graphics.RectF
use crate::cn::wps::font::FreeTypeJNI::RectF;

use std::sync::Arc;
use crate::cn::wps::font::IFontFamily::IFontFamily;
use crate::cn::wps::font::FreeTypeJNI::TTFHeader;

// Java field: int NORMAL
pub const NORMAL: i32 = 0;
// Java field: int BOLD
pub const BOLD: i32 = 1;
// Java field: int ITALIC
pub const ITALIC: i32 = 2;
// Java field: int UNDERLINE
pub const UNDERLINE: i32 = 4;
// Java field: int STRIKEOUT
pub const STRIKEOUT: i32 = 8;

pub struct FontMetrics {
    pub mMaxAscent: f32,
    pub mMaxDescent: f32,
    pub mBaseline: f32,
    pub mHeight: f32,
    pub mLeading: f32,
}

impl FontMetrics {
    pub fn new() -> Self {
        FontMetrics {
            mMaxAscent: 0.0,
            mMaxDescent: 0.0,
            mBaseline: 0.0,
            mHeight: 0.0,
            mLeading: 0.0,
        }
    }
}

// Java: public interface ITypeface
pub trait ITypeface: Send + Sync {
    // Java: abstract IFontFamily getFamily()
    fn getFamily(&self) -> Option<Arc<dyn IFontFamily>>;

    // Java: abstract int getStyle()
    fn getStyle(&self) -> i32;

    // Java: abstract void getFontMetrics(float var1, FontMetrics var2)
    fn getFontMetrics(&self, var1: f32, var2: &mut FontMetrics);

    // Java: abstract void getTextWidths(float var1, char[] var2, int var3, float[] var4, int var5, int var6)
    fn getTextWidths_float(&self, var1: f32, var2: &[u16], var3: i32, var4: &mut [f32], var5: i32, var6: i32);

    // Java: abstract void getTextWidths(int var1, char[] var2, int var3, int[] var4, int var5, int var6)
    fn getTextWidths_int(&self, var1: i32, var2: &[u16], var3: i32, var4: &mut [i32], var5: i32, var6: i32);

    // Java: abstract void getTextWidths(int var1, int[] var2, int var3, int[] var4, int var5, int var6)
    fn getTextWidths_int_int(&self, var1: i32, var2: &[i32], var3: i32, var4: &mut [i32], var5: i32, var6: i32);

    // Java: abstract void getTextWidthsEM(char[] var1, int var2, int[] var3, int var4, int var5)
    fn getTextWidthsEM(&self, var1: &[u16], var2: i32, var3: &mut [i32], var4: i32, var5: i32);

    // Java: abstract float getTextWidth(float var1, char[] var2, int var3, int var4)
    fn getTextWidth(&self, var1: f32, var2: &[u16], var3: i32, var4: i32) -> f32;

    // Java: abstract int getTextWidthEM(char[] var1, int var2, int var3)
    fn getTextWidthEM(&self, var1: &[u16], var2: i32, var3: i32) -> i32;

    // Java: abstract RectF getTextRect(float var1, char var2)
    fn getTextRect_char(&self, var1: f32, var2: u16) -> RectF;

    // Java: abstract RectF getTextRectEM(char var1)
    fn getTextRectEM_char(&self, var1: u16) -> RectF;

    // Java: abstract RectF getTextRect(float var1, char[] var2, int var3, int var4)
    fn getTextRect_chars(&self, var1: f32, var2: &[u16], var3: i32, var4: i32) -> RectF;

    // Java: abstract RectF getTextRectEM(char[] var1, int var2, int var3)
    fn getTextRectEM_chars(&self, var1: &[u16], var2: i32, var3: i32) -> RectF;

    // Java: abstract Object getNativeTypeface()
    fn getNativeTypeface(&self) -> i64;

    // Java: abstract boolean needBold()
    fn needBold(&self) -> bool;

    // Java: abstract boolean needItalic()
    fn needItalic(&self) -> bool;

    // Java: abstract float getDefaultFareastCharWidth(float var1)
    fn getDefaultFareastCharWidth(&self, var1: f32) -> f32;

    // Java: abstract int getDefaultFareastCharWidthEM()
    fn getDefaultFareastCharWidthEM(&self) -> i32;

    // Java: abstract RectF getDefaultFareastCharRect(float var1)
    fn getDefaultFareastCharRect(&self, var1: f32) -> RectF;

    // Java: abstract ITypeface deriveFallbackTypeface(IFontFamily var1)
    fn deriveFallbackTypeface(&self, var1: Arc<dyn IFontFamily>) -> Option<Arc<dyn ITypeface>>;

    // Java: abstract boolean isCJKFont()
    fn isCJKFont(&self) -> bool;

    // Java: abstract boolean isLatinFont()
    fn isLatinFont(&self) -> bool;

    // Java: abstract boolean isComplexFont()
    fn isComplexFont(&self) -> bool;

    // Java: abstract boolean hasUnicodeBlock(int var1)
    fn hasUnicodeBlock(&self, var1: i32) -> bool;

    // Java: abstract float getWriterLayoutEMSize()
    fn getWriterLayoutEMSize(&self) -> f32;

    // Java: abstract float getSrcEmSize()
    fn getSrcEmSize(&self) -> f32;

    // Java: abstract void setSrcEmSize(float var1)
    fn setSrcEmSize(&self, var1: f32);

    // Java: abstract float getWriterLayoutAscent()
    fn getWriterLayoutAscent(&self) -> f32;

    // Java: abstract float getWriterLayoutDescent()
    fn getWriterLayoutDescent(&self) -> f32;

    // Java: abstract float getWriterLayoutLatinExternal()
    fn getWriterLayoutLatinExternal(&self) -> f32;

    // Java: abstract float getWriterLayoutCharSpaceRatio()
    fn getWriterLayoutCharSpaceRatio(&self) -> f32;

    // Java: abstract float getWriterLayoutSubscriptFactor()
    fn getWriterLayoutSubscriptFactor(&self) -> f32;

    // Java: abstract float getWriterLayoutSuperscriptFactor()
    fn getWriterLayoutSuperscriptFactor(&self) -> f32;

    // Java: abstract void getWriterLayoutAdavances(float var1, char[] var2, int var3, float[] var4, int var5, int var6)
    fn getWriterLayoutAdavances_float(&self, var1: f32, var2: &[u16], var3: i32, var4: &mut [f32], var5: i32, var6: i32);

    // Java: abstract void getWriterLayoutAdavances(int var1, char[] var2, int var3, int[] var4, int var5, int var6)
    fn getWriterLayoutAdavances_int(&self, var1: i32, var2: &[u16], var3: i32, var4: &mut [i32], var5: i32, var6: i32);

    // Java: abstract void getWriterLayoutAdavances(int var1, int[] var2, int var3, int[] var4, int var5, int var6)
    fn getWriterLayoutAdavances_int_int(&self, var1: i32, var2: &[i32], var3: i32, var4: &mut [i32], var5: i32, var6: i32);

    // Java: abstract void getWriterLayoutAdavancesEM(char[] var1, int var2, int[] var3, int var4, int var5)
    fn getWriterLayoutAdavancesEM_chars(&self, var1: &[u16], var2: i32, var3: &mut [i32], var4: i32, var5: i32);

    // Java: abstract int getCharWidth(char var1, boolean var2)
    fn getCharWidth(&self, var1: u16, var2: bool) -> i32;

    // Java: abstract RectF getCharRect(char var1, boolean var2)
    fn getCharRect(&self, var1: u16, var2: bool) -> RectF;

    // Java: abstract FreeTypeJNI.TTFHeader getTTFHeader(boolean var1)
    fn getTTFHeader(&self, var1: bool) -> Option<TTFHeader>;

    // Java: abstract void getWriterLayoutAdavancesEM(int[] var1, int var2, int[] var3, int var4, int var5)
    fn getWriterLayoutAdavancesEM_ints(&self, var1: &[i32], var2: i32, var3: &mut [i32], var4: i32, var5: i32);

    // Java: abstract int getUnderlinePosition()
    fn getUnderlinePosition(&self) -> i32;

    // Java: abstract int getUnderlineThickness()
    fn getUnderlineThickness(&self) -> i32;

    // Java: abstract int getYStrikeoutSize()
    fn getYStrikeoutSize(&self) -> i32;

    // Java: abstract int getYStrikeoutPosition()
    fn getYStrikeoutPosition(&self) -> i32;

    // Java: abstract boolean hasKerning()
    fn hasKerning(&self) -> bool;

    // Java: abstract int getKerningEM(char var1, char var2)
    fn getKerningEM(&self, var1: u16, var2: u16) -> i32;

    // Java: abstract BuildinGlyphMetrics.IMetricsGetter getMetricsGetter()
    fn getMetricsGetter(&self) -> Option<Arc<dyn BuildinGlyphMetrics::IMetricsGetterTrait>>;

    // Java: abstract boolean isCreateFromFontFile()
    fn isCreateFromFontFile(&self) -> bool;

    // Java: abstract float getWppLayoutHeight()
    fn getWppLayoutHeight(&self) -> f32;

    // Java: abstract float getWppLayoutStakckedHeight()
    fn getWppLayoutStakckedHeight(&self) -> f32;
}
