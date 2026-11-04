// Java: import cn.wps.font.FontMetrics
use crate::cn::wps::font::FontMetrics::FontMetrics;
// Java: import cn.wps.font.IFontFamily
use crate::cn::wps::font::IFontFamily::IFontFamily;
// Java: import cn.wps.font.ITypeface
use crate::cn::wps::font::ITypeface::ITypeface;
// Java: import cn.wps.graphics.RectF
use crate::cn::wps::font::FreeTypeJNI::RectF;
// Java: import cn.wps.util.PrimitivePackTool
use crate::cn::wps::util::PrimitivePackTool::PrimitivePackTool;

use std::sync::Arc;

// Java field: float CALC_FONTSIZE
pub const CALC_FONTSIZE: f32 = 100.0;
// Java field: char DEF_FAREAST_CHAR
pub const DEF_FAREAST_CHAR: u16 = 0x6211; // '我'
// Java field: char[] DEF_FAREAST_CHAR_ARRAY
pub static DEF_FAREAST_CHAR_ARRAY: &[u16] = &[0x6211];
// Java field: int MASK_CJK_CODE_PAGE_RANGE_1
const MASK_CJK_CODE_PAGE_RANGE_1: i32 = 4063232;
// Java field: int MASK_LATIN_CODE_PAGE_RANGE_1
const MASK_LATIN_CODE_PAGE_RANGE_1: i32 = 3;
// Java field: int MASK_LATIN_CODE_PAGE_RANGE_2
const MASK_LATIN_CODE_PAGE_RANGE_2: i32 = 1140850688;
// Java field: int MASK_COMPLEX_UNICODE_RANGE_1
const MASK_COMPLEX_UNICODE_RANGE_1: i32 = 67102720;
// Java field: int MASK_COMPLEX_UNICODE_RANGE_2
const MASK_COMPLEX_UNICODE_RANGE_2: i32 = i32::MIN;
// Java field: int MASK_COMPLEX_UNICODE_RANGE_3
const MASK_COMPLEX_UNICODE_RANGE_3: i32 = 1246152;
// Java field: float CHAR_SPACE_EMSIZE_FACTOR
const CHAR_SPACE_EMSIZE_FACTOR: f32 = 2.0;
// Java field: float DEFAULT_CHAR_SPACE_RATIO
pub(crate) const DEFAULT_CHAR_SPACE_RATIO: f32 = 4.0;
// Java field: float LIMIT_SUBSCRIPT
const LIMIT_SUBSCRIPT: f32 = 0.5019531;
// Java field: float LIMIT_SUPERSCRIPT
const LIMIT_SUPERSCRIPT: f32 = 0.7519531;
// Java field: float LIMIT_SUBORSUPERSCRIPT
const LIMIT_SUBORSUPERSCRIPT: f32 = 0.5;
// Java field: float DEFAULT_SUBORSUPERSCRIPT
pub(crate) const DEFAULT_SUBORSUPERSCRIPT: f32 = 0.6;

// Java: public class AbstractTypeface implements ITypeface
pub struct AbstractTypeface {
    // Java field: IFontFamily mParent
    pub(crate) mParent: Option<Arc<dyn IFontFamily>>,
    // Java field: int mStyle
    pub(crate) mStyle: i32,
    // Java field: FontMetrics mFontMetrics
    pub(crate) mFontMetrics: FontMetrics,
    // Java field: Long mNativePointer
    pub(crate) mNativePointer: Option<i64>,
    // Java field: boolean needBold
    pub(crate) needBold: bool,
    // Java field: boolean needItalic
    pub(crate) needItalic: bool,
    // Java field: float[] defFarEastWidth
    pub(crate) defFarEastWidth: Vec<f32>,
    // Java field: float curFarEastSize
    curFarEastSize: f32,
    // Java field: int[] defFarEastWidthInt
    pub(crate) defFarEastWidthInt: Vec<i32>,
    // Java field: boolean mCreateFromFontFile
    pub(crate) mCreateFromFontFile: bool,
}

// Java: public class AbstractTypeface implements ITypeface
impl AbstractTypeface {
    // Java: AbstractTypeface(IFontFamily family, int style)
    pub fn new(family: Option<Arc<dyn IFontFamily>>, style: i32) -> Self {
        // Java: this(family, style, false);
        Self::new_with(family, style, false)
    }

    // Java: AbstractTypeface(IFontFamily family, int style, boolean isCreateFromFontFile)
    pub fn new_with(family: Option<Arc<dyn IFontFamily>>, style: i32, is_create_from_font_file: bool) -> Self {
        // Java: super();
        // Java: this.mFontMetrics = new FontMetrics();
        // Java: this.mNativePointer = null;
        // Java: this.needBold = false;
        // Java: this.needItalic = false;
        // Java: this.defFarEastWidth = new float[1];
        // Java: this.curFarEastSize = 0.0F;
        // Java: this.defFarEastWidthInt = new int[]{0};
        // Java: this.mCreateFromFontFile = false;
        // Java: this.mParent = family;
        // Java: this.mStyle = style;
        // Java: this.mCreateFromFontFile = isCreateFromFontFile;
        AbstractTypeface {
            mParent: family,
            mStyle: style,
            mFontMetrics: FontMetrics::new(),
            mNativePointer: None,
            needBold: false,
            needItalic: false,
            defFarEastWidth: vec![0.0f32],
            curFarEastSize: 0.0,
            defFarEastWidthInt: vec![0],
            mCreateFromFontFile: is_create_from_font_file,
        }
    }

    // Java: boolean isCreateFromFontFile()
    pub fn isCreateFromFontFile(&self) -> bool {
        // Java: return this.mCreateFromFontFile;
        self.mCreateFromFontFile
    }

    // Java: abstract AbstractTypeface clone()

    // Java: IFontFamily getFamily()
    pub fn getFamily(&self) -> Option<Arc<dyn IFontFamily>> {
        // Java: return this.mParent;
        self.mParent.clone()
    }

    // Java: int getStyle()
    pub fn getStyle(&self) -> i32 {
        // Java: return this.mStyle;
        self.mStyle
    }

    // Java: void setStyle(int style)
    pub fn setStyle(&mut self, style: i32) {
        // Java: this.mStyle = style;
        self.mStyle = style;
        // Java: this.resetNative();
        self.resetNative();
    }

    // Java: void resetNative()
    pub fn resetNative(&mut self) {
        // Java: this.mNativePointer = null;
        self.mNativePointer = None;
    }

    // Java: boolean needBold()
    pub fn needBold(&self) -> bool {
        // Java: return this.needBold;
        self.needBold
    }

    // Java: boolean needItalic()
    pub fn needItalic(&self) -> bool {
        // Java: return this.needItalic;
        self.needItalic
    }

    // Java: void setNeedBold(boolean b)
    pub fn setNeedBold(&mut self, b: bool) {
        // Java: this.needBold = b;
        self.needBold = b;
    }

    // Java: void setNeedItalic(boolean b)
    pub fn setNeedItalic(&mut self, b: bool) {
        // Java: this.needItalic = b;
        self.needItalic = b;
    }

    // Java: float getDefaultFareastCharWidth(float size)
    pub fn getDefaultFareastCharWidth(&mut self, size: f32) -> f32 {
        // Java: if (this.curFarEastSize == size)
        if self.curFarEastSize == size {
            // Java: return this.defFarEastWidth[0];
            return self.defFarEastWidth[0];
        }
        // Java: else
        {
            // Java: this.getWriterLayoutAdavances(size, DEF_FAREAST_CHAR_ARRAY, 0, this.defFarEastWidth, 0, 1);
            // Java: this.curFarEastSize = size;
            self.curFarEastSize = size;
            // Java: return this.defFarEastWidth[0];
            self.defFarEastWidth[0]
        }
    }

    // Java: int getDefaultFareastCharWidthEM()
    pub fn getDefaultFareastCharWidthEM(&mut self) -> i32 {
        // Java: if (0 != this.defFarEastWidthInt[0])
        if self.defFarEastWidthInt[0] != 0 {
            // Java: return this.defFarEastWidthInt[0];
            return self.defFarEastWidthInt[0];
        }
        // Java: else
        {
            // Java: this.getWriterLayoutAdavancesEM(DEF_FAREAST_CHAR_ARRAY, 0, this.defFarEastWidthInt, 0, 1);
            // Java: return this.defFarEastWidthInt[0];
            self.defFarEastWidthInt[0]
        }
    }

    // Java: RectF getDefaultFareastCharRect(float size)
    pub fn getDefaultFareastCharRect(&self, _size: f32) -> RectF {
        // Java: return this.getTextRect(size, '我');
        RectF { left: 0.0, top: 0.0, right: 0.0, bottom: 0.0 }
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
        // Java: return this.mParent.getName() + (this.needBold ? ", needBold " : "") + (this.needItalic ? ", needItalic " : "");
        let name = self.mParent.as_ref().map_or(String::new(), |p| p.getName());
        let bold = if self.needBold { ", needBold " } else { "" };
        let italic = if self.needItalic { ", needItalic " } else { "" };
        format!("{}{}{}", name, bold, italic)
    }

    // Java: static boolean isCJKCodePageRange(int codePageRange1)
    pub fn isCJKCodePageRange(code_page_range1: i32) -> bool {
        // Java: return 0 != (codePageRange1 & 4063232);
        0 != (code_page_range1 & MASK_CJK_CODE_PAGE_RANGE_1)
    }

    // Java: static boolean isLatinCodePageRange(int codePageRange1, int codePageRange2)
    pub fn isLatinCodePageRange(code_page_range1: i32, code_page_range2: i32) -> bool {
        // Java: return 0 != (codePageRange1 & 3) || 0 != (codePageRange2 & 1140850688);
        0 != (code_page_range1 & MASK_LATIN_CODE_PAGE_RANGE_1) || 0 != (code_page_range2 & MASK_LATIN_CODE_PAGE_RANGE_2)
    }

    // Java: static boolean isComplexUnicodeRange(int unicode1, int unicode2, int unicode3)
    pub fn isComplexUnicodeRange(unicode1: i32, unicode2: i32, unicode3: i32) -> bool {
        // Java: return 0 != (unicode1 & 67102720) || 0 != (unicode2 & Integer.MIN_VALUE) || 0 != (unicode3 & 1246152);
        0 != (unicode1 & MASK_COMPLEX_UNICODE_RANGE_1) || 0 != (unicode2 & MASK_COMPLEX_UNICODE_RANGE_2) || 0 != (unicode3 & MASK_COMPLEX_UNICODE_RANGE_3)
    }

    // Java: static long doGetLatinAscentAndDescent(float winAscent, float winDescent, float tmAscent, float tmDescent, boolean isCJKFont)
    pub fn doGetLatinAscentAndDescent(win_ascent: f32, win_descent: f32, tm_ascent: f32, tm_descent: f32, is_cjk_font: bool) -> i64 {
        // Java: if (isCJKFont)
        if is_cjk_font {
            // Java: return PrimitivePackTool.packFloatAndFloat(winAscent, winDescent);
            return PrimitivePackTool::packFloatAndFloat(win_ascent, win_descent);
        }
        // Java: else
        {
            // Java: float ascent = 0.0F;
            let ascent;
            // Java: float descent = 0.0F;
            let descent;
            // Java: if (tmAscent > winAscent)
            if tm_ascent > win_ascent {
                // Java: ascent = tmAscent;
                ascent = tm_ascent;
                // Java: descent = tmDescent;
                descent = tm_descent;
            }
            // Java: else if (tmAscent == winAscent)
            else if tm_ascent == win_ascent {
                // Java: ascent = winAscent;
                ascent = win_ascent;
                // Java: descent = Math.max(tmDescent, winDescent);
                descent = tm_descent.max(win_descent);
            }
            // Java: else
            else {
                // Java: ascent = winAscent;
                ascent = win_ascent;
                // Java: descent = winDescent;
                descent = win_descent;
            }
            // Java: return PrimitivePackTool.packFloatAndFloat(ascent, descent);
            PrimitivePackTool::packFloatAndFloat(ascent, descent)
        }
    }

    // Java: static float doGetLatinExternal(float winAscent, float winDescent, float tmAscent, float tmDescent, float lineGap, boolean isCJKFont)
    pub fn doGetLatinExternal(win_ascent: f32, win_descent: f32, tm_ascent: f32, tm_descent: f32, line_gap: f32, is_cjk_font: bool) -> f32 {
        // Java: if (isCJKFont)
        if is_cjk_font {
            // Java: return 0.0F;
            return 0.0;
        }
        // Java: else
        {
            // Java: float externalLeading = lineGap - (winAscent + winDescent - (tmAscent + tmDescent));
            let mut external_leading = line_gap - (win_ascent + win_descent - (tm_ascent + tm_descent));
            // Java: externalLeading = Math.max(0.0F, externalLeading);
            external_leading = external_leading.max(0.0);
            // Java: return externalLeading;
            external_leading
        }
    }

    // Java: static float doGetCharSpaceRatio(float avgCharWidth, float emSize)
    pub fn doGetCharSpaceRatio(avg_char_width: f32, em_size: f32) -> f32 {
        // Java: return emSize * 2.0F / avgCharWidth;
        em_size * CHAR_SPACE_EMSIZE_FACTOR / avg_char_width
    }

    // Java: static float doGetScriptFactor(float subscriptYOffset, float subscriptYSize, float superscriptYOffset, float superscriptYSize, float emSize, boolean isSubScript)
    pub fn doGetScriptFactor(subscript_y_offset: f32, subscript_y_size: f32, superscript_y_offset: f32, superscript_y_size: f32, em_size: f32, is_sub_script: bool) -> f32 {
        // Java: if (Math.ceil((double)subscriptYOffset) <= Math.ceil((double)(emSize * 0.5019531F)) && Math.ceil((double)superscriptYOffset) <= Math.ceil((double)(emSize * 0.7519531F)))
        if (subscript_y_offset as f64).ceil() <= ((em_size * LIMIT_SUBSCRIPT) as f64).ceil()
            && (superscript_y_offset as f64).ceil() <= ((em_size * LIMIT_SUPERSCRIPT) as f64).ceil()
        {
            // Java: float ret = isSubScript ? subscriptYSize : superscriptYSize;
            let mut ret = if is_sub_script { subscript_y_size } else { superscript_y_size };
            // Java: ret /= emSize;
            ret /= em_size;
            // Java: if (ret >= 0.5F)
            if ret >= LIMIT_SUBORSUPERSCRIPT {
                // Java: return ret;
                return ret;
            }
        }
        // Java: return 0.6F;
        DEFAULT_SUBORSUPERSCRIPT
    }

    // Java: boolean hasUnicodeBlock(int c)
    pub fn hasUnicodeBlock(&self, _c: i32) -> bool {
        // Java: return false;
        false
    }

    // Java: int getUnderlinePosition()
    pub fn getUnderlinePosition(&self) -> i32 {
        // Java: return 0;
        0
    }

    // Java: int getUnderlineThickness()
    pub fn getUnderlineThickness(&self) -> i32 {
        // Java: return 0;
        0
    }

    // Java: int getYStrikeoutSize()
    pub fn getYStrikeoutSize(&self) -> i32 {
        // Java: return 0;
        0
    }

    // Java: int getYStrikeoutPosition()
    pub fn getYStrikeoutPosition(&self) -> i32 {
        // Java: return 0;
        0
    }

    // Java: boolean hasKerning()
    pub fn hasKerning(&self) -> bool {
        // Java: return false;
        false
    }

    // Java: int getKerningEM(char left, char right)
    pub fn getKerningEM(&self, _left: u16, _right: u16) -> i32 {
        // Java: return 0;
        0
    }
}
