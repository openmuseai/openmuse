// Java: import android.graphics.Paint
// Java: import cn.wps.text.layout.typo.PaintFontHelper
use super::FontFill::{FontFill, FontFillType};
use super::TextOutline::{TextOutline, TxtOutlnType};
use super::TextShadow::{TextShadow, ShadowType};
use super::TextImage::TextImage;
use super::TextBlur::TextBlur;
use super::GradientAttr::GradType;

// Java field: int AUTO_FONT_COLOR
pub const AUTO_FONT_COLOR: i32 = -16777216;
// Java field: int TEXT_CAP_ALL
pub const TEXT_CAP_ALL: i32 = 0;
// Java field: int TEXT_CAP_NONE
pub const TEXT_CAP_NONE: i32 = 1;
// Java field: int TEXT_CAP_SMALL
pub const TEXT_CAP_SMALL: i32 = 2;
// Java field: byte UNDERLINE_TYPE_NONE
pub const UNDERLINE_TYPE_NONE: u8 = 0;
// Java field: byte UNDERLINE_TYPE_SINGLE
pub const UNDERLINE_TYPE_SINGLE: u8 = 1;
// Java field: byte UNDERLINE_TYPE_DOUBLE
pub const UNDERLINE_TYPE_DOUBLE: u8 = 2;
// Java field: byte UNDERLINE_TYPE_WORDS
pub const UNDERLINE_TYPE_WORDS: u8 = 3;
// Java field: byte UNDERLINE_TYPE_HEAVY
pub const UNDERLINE_TYPE_HEAVY: u8 = 4;
// Java field: byte UNDERLINE_TYPE_DOTTED
pub const UNDERLINE_TYPE_DOTTED: u8 = 5;
// Java field: byte UNDERLINE_TYPE_DASH
pub const UNDERLINE_TYPE_DASH: u8 = 6;
// Java field: byte UNDERLINE_TYPE_DOT_DASH
pub const UNDERLINE_TYPE_DOT_DASH: u8 = 7;
// Java field: byte UNDERLINE_TYPE_DOT_DOT_DASH
pub const UNDERLINE_TYPE_DOT_DOT_DASH: u8 = 8;
// Java field: byte UNDERLINE_TYPE_WAVY
pub const UNDERLINE_TYPE_WAVY: u8 = 9;
// Java field: byte UNDERLINE_TYPE_WAVY_HEAVY
pub const UNDERLINE_TYPE_WAVY_HEAVY: u8 = 10;
// Java field: byte UNDERLINE_TYPE_DOTTED_HEAVY
pub const UNDERLINE_TYPE_DOTTED_HEAVY: u8 = 11;
// Java field: byte UNDERLINE_TYPE_DASH_HEAVY
pub const UNDERLINE_TYPE_DASH_HEAVY: u8 = 12;
// Java field: byte UNDERLINE_TYPE_DOT_DASH_HEAVY
pub const UNDERLINE_TYPE_DOT_DASH_HEAVY: u8 = 13;
// Java field: byte UNDERLINE_TYPE_DOT_DOT_DASH_HEAVY
pub const UNDERLINE_TYPE_DOT_DOT_DASH_HEAVY: u8 = 14;
// Java field: byte UNDERLINE_TYPE_DASH_LONG
pub const UNDERLINE_TYPE_DASH_LONG: u8 = 15;
// Java field: byte UNDERLINE_TYPE_DASH_LONG_HEAVY
pub const UNDERLINE_TYPE_DASH_LONG_HEAVY: u8 = 16;
// Java field: byte UNDERLINE_TYPE_WAVY_DOUBLE
pub const UNDERLINE_TYPE_WAVY_DOUBLE: u8 = 17;
// Java field: byte DELETELINE_TYPE_NONE
pub const DELETELINE_TYPE_NONE: u8 = 100;
// Java field: byte DELETELINE_TYPE_SINGLE
pub const DELETELINE_TYPE_SINGLE: u8 = 101;
// Java field: byte DELETELINE_TYPE_DOUBLE
pub const DELETELINE_TYPE_DOUBLE: u8 = 102;
// Java: public class SpanInfo
#[derive(Clone)]
pub struct SpanInfo {
    // Java field: String mFontName
    pub mFontName: String,
    // Java field: byte mFontFamily
    pub mFontFamily: u8,
    // Java field: byte mCharset
    pub mCharset: u8,
    // Java field: float mFontSize
    pub mFontSize: f32,
    // Java field: float mFontSizeOri
    pub mFontSizeOri: f32,
    // Java field: int mFontColor
    pub mFontColor: i32,
    // Java field: float mColorAlpha
    pub mColorAlpha: f32,
    // Java field: byte mUnderline
    pub mUnderline: u8,
    // Java field: int mUnderlineColor
    pub mUnderlineColor: i32,
    // Java field: FontFill mFontFill
    pub mFontFill: Option<FontFill>,
    // Java field: TextOutline mTextOutline
    pub mTextOutline: Option<TextOutline>,
    // Java field: TextShadow mTextShadow
    pub mTextShadow: Option<TextShadow>,
    // Java field: TextImage mTextImage
    pub mTextImage: Option<TextImage>,
    // Java field: TextBlur mTextBlur
    pub mTextBlur: Option<TextBlur>,
    // Java field: boolean mIsSuperScript
    pub mIsSuperScript: bool,
    // Java field: boolean mIsSubScript
    pub mIsSubScript: bool,
    // Java field: boolean mIsBold
    pub mIsBold: bool,
    // Java field: byte mStrikethrough
    pub mStrikethrough: u8,
    // Java field: boolean mIsItalic
    pub mIsItalic: bool,
    // Java field: boolean mIsAccounting
    pub mIsAccounting: bool,
    // Java field: byte mTextCaps
    pub mTextCaps: u8,
    // Java field: boolean mIsNormalizeH
    pub mIsNormalizeH: bool,
    // Java field: int mStartPos
    pub mStartPos: i32,
    // Java field: int mEndPos
    pub mEndPos: i32,
    // Java field: boolean mArb
    pub mArb: bool,
    // Java field: float mSpc
    pub mSpc: f32,
    // Java field: float mKern
    pub mKern: f32,
    // Java field: float mAscent
    mAscent: f32,
    // Java field: float SUB_SUPER_SCALE
}

pub const SUB_SUPER_SCALE: f32 = 0.75;

// Java: public class SpanInfo
impl SpanInfo {
    // Java: SpanInfo()
    pub fn new() -> Self {
        // Java: super();
        // Java: this.mFontName = "times New Roman";
        // Java: this.mFontColor = -16777216;
        // Java: this.mFontSize = 40.0F;
        // Java: this.mStartPos = 0;
        // Java: this.mEndPos = 0;
        // Java: this.mUnderline = 0;
        // Java: this.mUnderlineColor = this.mFontColor;
        // Java: this.mFontSizeOri = this.mFontSize;
        SpanInfo {
            mFontName: String::from("times New Roman"),
            mFontFamily: 0,
            mCharset: 0,
            mFontSize: 40.0,
            mFontSizeOri: 40.0,
            mFontColor: -16777216,
            mColorAlpha: 1.0,
            mUnderline: 0,
            mUnderlineColor: -16777216,
            mFontFill: None,
            mTextOutline: None,
            mTextShadow: None,
            mTextImage: None,
            mTextBlur: None,
            mIsSuperScript: false,
            mIsSubScript: false,
            mIsBold: false,
            mStrikethrough: 0,
            mIsItalic: false,
            mIsAccounting: false,
            mTextCaps: 1,
            mIsNormalizeH: false,
            mStartPos: 0,
            mEndPos: 0,
            mArb: false,
            mSpc: 0.0,
            mKern: 0.0,
            mAscent: 0.0,
        }
    }

    // Java: SpanInfo(SpanInfo span)
    pub fn new_from(span: &SpanInfo) -> Self {
        // Java: super();
        // Java: this.mFontName = span.mFontName;
        // Java: this.mFontFamily = span.mFontFamily;
        // Java: this.mCharset = span.mCharset;
        // Java: this.mFontSize = span.mFontSize;
        // Java: this.mFontSizeOri = span.mFontSize;
        // Java: this.mFontColor = span.mFontColor;
        // Java: this.mUnderline = span.mUnderline;
        // Java: this.mUnderlineColor = span.mUnderlineColor;
        // Java: this.mIsSuperScript = span.mIsSuperScript;
        // Java: this.mIsSubScript = span.mIsSubScript;
        // Java: this.mIsBold = span.mIsBold;
        // Java: this.mStrikethrough = span.mStrikethrough;
        // Java: this.mIsItalic = span.mIsItalic;
        // Java: this.mIsAccounting = span.mIsAccounting;
        // Java: this.mTextCaps = span.mTextCaps;
        // Java: this.mIsNormalizeH = span.mIsNormalizeH;
        // Java: this.mStartPos = span.mStartPos;
        // Java: this.mEndPos = span.mEndPos;
        // Java: this.adjustPos();
        // Java: this.mAscent = span.mAscent;
        // Java: this.mFontFill = span.mFontFill;
        // Java: this.mTextOutline = span.mTextOutline;
        // Java: this.mTextShadow = span.mTextShadow;
        // Java: this.mTextImage = span.mTextImage;
        // Java: this.mTextBlur = span.mTextBlur;
        // Java: this.mSpc = span.mSpc;
        // Java: this.mKern = span.mKern;
        // Java: this.mArb = span.mArb;
        let mut s = SpanInfo {
            mFontName: span.mFontName.clone(),
            mFontFamily: span.mFontFamily,
            mCharset: span.mCharset,
            mFontSize: span.mFontSize,
            mFontSizeOri: span.mFontSize,
            mFontColor: span.mFontColor,
            mColorAlpha: span.mColorAlpha,
            mUnderline: span.mUnderline,
            mUnderlineColor: span.mUnderlineColor,
            mFontFill: None,
            mTextOutline: None,
            mTextShadow: None,
            mTextImage: None,
            mTextBlur: None,
            mIsSuperScript: span.mIsSuperScript,
            mIsSubScript: span.mIsSubScript,
            mIsBold: span.mIsBold,
            mStrikethrough: span.mStrikethrough,
            mIsItalic: span.mIsItalic,
            mIsAccounting: span.mIsAccounting,
            mTextCaps: span.mTextCaps,
            mIsNormalizeH: span.mIsNormalizeH,
            mStartPos: span.mStartPos,
            mEndPos: span.mEndPos,
            mArb: span.mArb,
            mSpc: span.mSpc,
            mKern: span.mKern,
            mAscent: span.mAscent,
        };
        s.adjustPos();
        s
    }

    // Java: SpanInfo(String fontName, byte fontFamily, byte charset, float fontSize, int fontColor, byte underline, boolean isSuperScript, boolean isSubScript, boolean isBold, byte strikethrough, boolean isItalic, int startPos, int endPos, int underlineColor, FontFill fontFill, TextOutline txtOutline, TextShadow txtShadow, TextImage txtImage, TextBlur txtBlur)
    pub fn new_full(fontName: String, fontFamily: u8, charset: u8, fontSize: f32, fontColor: i32, underline: u8, isSuperScript: bool, isSubScript: bool, isBold: bool, strikethrough: u8, isItalic: bool, startPos: i32, endPos: i32, underlineColor: i32, fontFill: Option<FontFill>, txtOutline: Option<TextOutline>, txtShadow: Option<TextShadow>, txtImage: Option<TextImage>, txtBlur: Option<TextBlur>) -> Self {
        // Java: super();
        // Java: this.mFontName = fontName;
        // Java: this.mFontFamily = fontFamily;
        // Java: this.mCharset = charset;
        // Java: this.mFontSize = fontSize;
        // Java: this.mFontSizeOri = fontSize;
        // Java: this.mFontColor = fontColor;
        // Java: this.mUnderline = underline;
        // Java: this.mUnderlineColor = underlineColor;
        // Java: this.mIsSuperScript = isSuperScript;
        // Java: this.mIsSubScript = isSubScript;
        // Java: this.mIsBold = isBold;
        // Java: this.mStrikethrough = strikethrough;
        // Java: this.mIsItalic = isItalic;
        // Java: this.mFontFill = fontFill;
        // Java: this.mTextOutline = txtOutline;
        // Java: this.mTextShadow = txtShadow;
        // Java: this.mTextImage = txtImage;
        // Java: this.mTextBlur = txtBlur;
        // Java: this.mStartPos = startPos;
        // Java: this.mEndPos = endPos;
        // Java: this.adjustPos();
        let mut s = SpanInfo {
            mFontName: fontName,
            mFontFamily: fontFamily,
            mCharset: charset,
            mFontSize: fontSize,
            mFontSizeOri: fontSize,
            mFontColor: fontColor,
            mColorAlpha: 1.0,
            mUnderline: underline,
            mUnderlineColor: underlineColor,
            mFontFill: fontFill,
            mTextOutline: txtOutline,
            mTextShadow: txtShadow,
            mTextImage: txtImage,
            mTextBlur: txtBlur,
            mIsSuperScript: isSuperScript,
            mIsSubScript: isSubScript,
            mIsBold: isBold,
            mStrikethrough: strikethrough,
            mIsItalic: isItalic,
            mIsAccounting: false,
            mTextCaps: 1,
            mIsNormalizeH: false,
            mStartPos: startPos,
            mEndPos: endPos,
            mArb: false,
            mSpc: 0.0,
            mKern: 0.0,
            mAscent: 0.0,
        };
        s.adjustPos();
        s
    }

    // Java: boolean equalsAttr(SpanInfo spanInfo)
    pub fn equalsAttr(&self, spanInfo: Option<&SpanInfo>) -> bool {
        // Java: if (spanInfo == null)
        let spanInfo = match spanInfo {
            None => {
                // Java: return false;
                return false;
            }
            Some(s) => s,
        };
        // Java: else
        // Java: return this.mFontName.equals(spanInfo.mFontName) && this.mFontFamily == spanInfo.mFontFamily && this.mCharset == spanInfo.mCharset && this.mFontSize == spanInfo.mFontSize && this.mFontSizeOri == spanInfo.mFontSizeOri && this.mFontColor == spanInfo.mFontColor && this.mUnderline == spanInfo.mUnderline && this.mIsBold == spanInfo.mIsBold && this.mIsItalic == spanInfo.mIsItalic && this.mStrikethrough == spanInfo.mStrikethrough && this.mIsSuperScript == spanInfo.mIsSuperScript && this.mIsSubScript == spanInfo.mIsSubScript && this.mUnderlineColor == spanInfo.mUnderlineColor && this.mTextCaps == spanInfo.mTextCaps && this.mIsNormalizeH == spanInfo.mIsNormalizeH && this.mArb == spanInfo.mArb && (this.mFontFill == null && spanInfo.mFontFill == null || this.mFontFill != null && this.mFontFill.equals(spanInfo.mFontFill)) && (this.mTextOutline == null && spanInfo.mTextOutline == null || this.mTextOutline != null && this.mTextOutline.equals(spanInfo.mTextOutline)) && (this.mTextShadow == null && spanInfo.mTextShadow == null || this.mTextShadow != null && this.mTextShadow.equals(spanInfo.mTextShadow)) && (this.mTextImage == null && spanInfo.mTextImage == null || this.mTextImage != null && this.mTextImage.equals(spanInfo.mTextImage)) && (this.mTextBlur == null && spanInfo.mTextBlur == null || this.mTextBlur != null && this.mTextBlur.equals(spanInfo.mTextBlur)) && this.mSpc == spanInfo.mSpc && this.mKern == spanInfo.mKern;
        self.mFontName == spanInfo.mFontName
            && self.mFontFamily == spanInfo.mFontFamily
            && self.mCharset == spanInfo.mCharset
            && self.mFontSize == spanInfo.mFontSize
            && self.mFontSizeOri == spanInfo.mFontSizeOri
            && self.mFontColor == spanInfo.mFontColor
            && self.mUnderline == spanInfo.mUnderline
            && self.mIsBold == spanInfo.mIsBold
            && self.mIsItalic == spanInfo.mIsItalic
            && self.mStrikethrough == spanInfo.mStrikethrough
            && self.mIsSuperScript == spanInfo.mIsSuperScript
            && self.mIsSubScript == spanInfo.mIsSubScript
            && self.mUnderlineColor == spanInfo.mUnderlineColor
            && self.mTextCaps == spanInfo.mTextCaps
            && self.mIsNormalizeH == spanInfo.mIsNormalizeH
            && self.mArb == spanInfo.mArb
            && self.mSpc == spanInfo.mSpc
            && self.mKern == spanInfo.mKern
    }

    // Java: boolean hasSpc()
    pub fn hasSpc(&self) -> bool {
        // Java: return this.mSpc != 0.0F;
        self.mSpc != 0.0
    }

    // Java: float getSpc()
    pub fn getSpc(&self) -> f32 {
        // Java: return this.mSpc;
        self.mSpc
    }

    // Java: boolean hasFill()
    pub fn hasFill(&self) -> bool {
        // Java: return this.mFontFill != null && this.mFontFill.mFillType != FontFill.FontFillType.NONE || this.mFontFill == null;
        match &self.mFontFill {
            Some(ff) => ff.mFillType != FontFillType::NONE,
            None => true,
        }
    }

    // Java: boolean hasArtFill()
    pub fn hasArtFill(&self) -> bool {
        // Java: return this.mFontFill != null && this.mFontFill.mFillType != FontFill.FontFillType.COLORFILL;
        match &self.mFontFill {
            Some(ff) => ff.mFillType != FontFillType::COLORFILL,
            None => false,
        }
    }

    // Java: boolean hasRectShapeGradFill()
    pub fn hasRectShapeGradFill(&self) -> bool {
        // Java: if (this.mFontFill != null && this.mFontFill.mFillType == FontFill.FontFillType.GRADFILL && this.mFontFill.mGradFill != null && this.mFontFill.mGradFill.getGradAttr() != null)
        {
            // Java: GradientAttr.GradType type = this.mFontFill.mGradFill.getGradAttr().gradType();
            // Java: return type == GradientAttr.GradType.RECT || type == GradientAttr.GradType.SHAPE;
        }
        // Java: else
        {
            // Java: return false;
            false
        }
    }

    // Java: boolean hasOutline()
    pub fn hasOutline(&self) -> bool {
        // Java: return this.mTextOutline != null && this.mTextOutline.getTxtOutlnType() != TextOutline.TxtOutlnType.NONE;
        match &self.mTextOutline {
            Some(to) => to.getTxtOutlnType() != TxtOutlnType::NONE,
            None => false,
        }
    }

    // Java: boolean hasBlur()
    pub fn hasBlur(&self) -> bool {
        // Java: return this.mTextBlur != null && (this.hasFill() || this.hasOutline());
        self.mTextBlur.is_some() && (self.hasFill() || self.hasOutline())
    }

    // Java: boolean hasTextImage()
    pub fn hasTextImage(&self) -> bool {
        // Java: return this.mTextImage != null && (this.hasFill() || this.hasOutline());
        self.mTextImage.is_some() && (self.hasFill() || self.hasOutline())
    }

    // Java: boolean hasOuterShdw()
    pub fn hasOuterShdw(&self) -> bool {
        // Java: return this.mTextShadow != null && this.mTextShadow.getShdwType() == TextShadow.ShadowType.OUTER && (this.hasFill() || this.hasOutline());
        match &self.mTextShadow {
            Some(ts) => ts.getShdwType() == Some(ShadowType::OUTER) && (self.hasFill() || self.hasOutline()),
            None => false,
        }
    }

    // Java: boolean hasInnerShdw()
    pub fn hasInnerShdw(&self) -> bool {
        // Java: return this.mTextShadow != null && this.mTextShadow.getShdwType() == TextShadow.ShadowType.INNER && (this.hasFill() || this.hasOutline());
        match &self.mTextShadow {
            Some(ts) => ts.getShdwType() == Some(ShadowType::INNER) && (self.hasFill() || self.hasOutline()),
            None => false,
        }
    }

    // Java: boolean hasStrikethroughLine()
    pub fn hasStrikethroughLine(&self) -> bool {
        // Java: return this.mStrikethrough != 100;
        self.mStrikethrough != 100
    }

    // Java: boolean hasUnderline()
    pub fn hasUnderline(&self) -> bool {
        // Java: return this.mUnderline != 0;
        self.mUnderline != 0
    }

    // Java: boolean isAcounting()
    pub fn isAcounting(&self) -> bool {
        // Java: return this.mIsAccounting;
        self.mIsAccounting
    }

    // Java: boolean isUnderlineHeavy()
    pub fn isUnderlineHeavy(&self) -> bool {
        // Java: return 4 == this.mUnderline || 10 == this.mUnderline || 11 == this.mUnderline || 12 == this.mUnderline || 13 == this.mUnderline || 14 == this.mUnderline || 16 == this.mUnderline;
        self.mUnderline == 4 || self.mUnderline == 10 || self.mUnderline == 11 || self.mUnderline == 12 || self.mUnderline == 13 || self.mUnderline == 14 || self.mUnderline == 16
    }

    // Java: boolean contains(int pos)
    pub fn contains(&self, pos: i32) -> bool {
        // Java: return pos >= this.mStartPos && pos < this.mEndPos && this.mStartPos != this.mEndPos;
        pos >= self.mStartPos && pos < self.mEndPos && self.mStartPos != self.mEndPos
    }

    // Java: float getAscent()
    pub fn getAscent(&self) -> f32 {
        // Java: if (0.0F == this.mAscent)
        // Java: Paint.FontMetricsInt fontMetrics = new Paint.FontMetricsInt();
        // Java: PaintFontHelper.getFontMetricsInt(fontMetrics, this);
        // Java: this.mAscent = (float)fontMetrics.ascent;
        // Java: return this.mAscent;
        self.mAscent
    }

    // Java: void resetAscent()
    pub fn resetAscent(&mut self) {
        // Java: this.mAscent = 0.0F;
        self.mAscent = 0.0;
    }

    // Java: void adjustPos()
    pub fn adjustPos(&mut self) {
        // Java: this.mEndPos = Math.max(this.mEndPos, 0);
        self.mEndPos = self.mEndPos.max(0);
        // Java: this.mStartPos = Math.max(this.mStartPos, 0);
        self.mStartPos = self.mStartPos.max(0);
        // Java: this.mStartPos = Math.min(this.mStartPos, this.mEndPos);
        self.mStartPos = self.mStartPos.min(self.mEndPos);
    }

    // Java: void copyFrom(SpanInfo span)
    pub fn copyFrom(&mut self, span: &SpanInfo) {
        // Java: this.mFontName = span.mFontName;
        self.mFontName = span.mFontName.clone();
        // Java: this.mFontFamily = span.mFontFamily;
        self.mFontFamily = span.mFontFamily;
        // Java: this.mCharset = span.mCharset;
        self.mCharset = span.mCharset;
        // Java: this.mFontSize = span.mFontSize;
        self.mFontSize = span.mFontSize;
        // Java: this.mFontSizeOri = span.mFontSizeOri;
        self.mFontSizeOri = span.mFontSizeOri;
        // Java: this.mFontColor = span.mFontColor;
        self.mFontColor = span.mFontColor;
        // Java: this.mUnderline = span.mUnderline;
        self.mUnderline = span.mUnderline;
        // Java: this.mUnderlineColor = span.mUnderlineColor;
        self.mUnderlineColor = span.mUnderlineColor;
        // Java: this.mIsSuperScript = span.mIsSuperScript;
        self.mIsSuperScript = span.mIsSuperScript;
        // Java: this.mIsSubScript = span.mIsSubScript;
        self.mIsSubScript = span.mIsSubScript;
        // Java: this.mIsBold = span.mIsBold;
        self.mIsBold = span.mIsBold;
        // Java: this.mStrikethrough = span.mStrikethrough;
        self.mStrikethrough = span.mStrikethrough;
        // Java: this.mIsItalic = span.mIsItalic;
        self.mIsItalic = span.mIsItalic;
        // Java: this.mIsAccounting = span.mIsAccounting;
        self.mIsAccounting = span.mIsAccounting;
        // Java: this.mTextCaps = span.mTextCaps;
        self.mTextCaps = span.mTextCaps;
        // Java: this.mIsNormalizeH = span.mIsNormalizeH;
        self.mIsNormalizeH = span.mIsNormalizeH;
        // Java: this.mStartPos = span.mStartPos;
        self.mStartPos = span.mStartPos;
        // Java: this.mEndPos = span.mEndPos;
        self.mEndPos = span.mEndPos;
        // Java: this.adjustPos();
        self.adjustPos();
        // Java: this.mAscent = span.mAscent;
        self.mAscent = span.mAscent;
        // Java: this.mFontFill = span.mFontFill;
        // Java: this.mTextOutline = span.mTextOutline;
        // Java: this.mTextShadow = span.mTextShadow;
        // Java: this.mTextImage = span.mTextImage;
        // Java: this.mTextBlur = span.mTextBlur;
        // Java: this.mSpc = span.mSpc;
        self.mSpc = span.mSpc;
        // Java: this.mArb = span.mArb;
        self.mArb = span.mArb;
    }

    // Java: void clear()
    pub fn clear(&mut self) {
        // Java: this.mFontName = "times New Roman";
        self.mFontName = String::from("times New Roman");
        // Java: this.mFontFamily = 0;
        self.mFontFamily = 0;
        // Java: this.mCharset = 0;
        self.mCharset = 0;
        // Java: this.mFontColor = -16777216;
        self.mFontColor = -16777216;
        // Java: this.mFontSize = 40.0F;
        self.mFontSize = 40.0;
        // Java: this.mStartPos = 0;
        self.mStartPos = 0;
        // Java: this.mEndPos = 0;
        self.mEndPos = 0;
        // Java: this.mUnderline = 0;
        self.mUnderline = 0;
        // Java: this.mUnderlineColor = this.mFontColor;
        self.mUnderlineColor = self.mFontColor;
        // Java: this.mFontSizeOri = this.mFontSize;
        self.mFontSizeOri = self.mFontSize;
        // Java: this.mIsSuperScript = false;
        self.mIsSuperScript = false;
        // Java: this.mIsSubScript = false;
        self.mIsSubScript = false;
        // Java: this.mIsBold = false;
        self.mIsBold = false;
        // Java: this.mStrikethrough = 100;
        self.mStrikethrough = 100;
        // Java: this.mIsItalic = false;
        self.mIsItalic = false;
        // Java: this.mIsAccounting = false;
        self.mIsAccounting = false;
        // Java: this.mTextCaps = 1;
        self.mTextCaps = 1;
        // Java: this.mIsNormalizeH = false;
        self.mIsNormalizeH = false;
        // Java: this.mFontFill = null;
        self.mFontFill = None;
        // Java: this.mTextOutline = null;
        self.mTextOutline = None;
        // Java: this.mTextShadow = null;
        self.mTextShadow = None;
        // Java: this.mTextImage = null;
        self.mTextImage = None;
        // Java: this.mTextBlur = null;
        self.mTextBlur = None;
        // Java: this.mAscent = 0.0F;
        self.mAscent = 0.0;
        // Java: this.mSpc = 0.0F;
        self.mSpc = 0.0;
        // Java: this.mKern = 0.0F;
        self.mKern = 0.0;
        // Java: this.mArb = false;
        self.mArb = false;
    }

    // Java: boolean hasKern()
    pub fn hasKern(&self) -> bool {
        // Java: return this.mKern != 0.0F;
        self.mKern != 0.0
    }

    // Java: float getKern()
    pub fn getKern(&self) -> f32 {
        // Java: return this.mKern;
        self.mKern
    }
}
