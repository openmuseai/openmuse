// Java: import android.graphics.Paint
use crate::android::graphics::Paint;
// Java: import cn.wps.text.layout.typo.PaintFontHelper
use wpslayout::cn::wps::text::layout::typo::PaintFontHelper;

// Java field: int AUTO_FONT_COLOR
pub(crate) const AUTO_FONT_COLOR: i32 = /* TODO */;
// Java field: int TEXT_CAP_ALL
pub(crate) const TEXT_CAP_ALL: i32 = /* TODO */;
// Java field: int TEXT_CAP_NONE
pub(crate) const TEXT_CAP_NONE: i32 = /* TODO */;
// Java field: int TEXT_CAP_SMALL
pub(crate) const TEXT_CAP_SMALL: i32 = /* TODO */;
// Java field: byte UNDERLINE_TYPE_NONE
pub(crate) const UNDERLINE_TYPE_NONE: i8 = /* TODO */;
// Java field: byte UNDERLINE_TYPE_SINGLE
pub(crate) const UNDERLINE_TYPE_SINGLE: i8 = /* TODO */;
// Java field: byte UNDERLINE_TYPE_DOUBLE
pub(crate) const UNDERLINE_TYPE_DOUBLE: i8 = /* TODO */;
// Java field: byte UNDERLINE_TYPE_WORDS
pub(crate) const UNDERLINE_TYPE_WORDS: i8 = /* TODO */;
// Java field: byte UNDERLINE_TYPE_HEAVY
pub(crate) const UNDERLINE_TYPE_HEAVY: i8 = /* TODO */;
// Java field: byte UNDERLINE_TYPE_DOTTED
pub(crate) const UNDERLINE_TYPE_DOTTED: i8 = /* TODO */;
// Java field: byte UNDERLINE_TYPE_DASH
pub(crate) const UNDERLINE_TYPE_DASH: i8 = /* TODO */;
// Java field: byte UNDERLINE_TYPE_DOT_DASH
pub(crate) const UNDERLINE_TYPE_DOT_DASH: i8 = /* TODO */;
// Java field: byte UNDERLINE_TYPE_DOT_DOT_DASH
pub(crate) const UNDERLINE_TYPE_DOT_DOT_DASH: i8 = /* TODO */;
// Java field: byte UNDERLINE_TYPE_WAVY
pub(crate) const UNDERLINE_TYPE_WAVY: i8 = /* TODO */;
// Java field: byte UNDERLINE_TYPE_WAVY_HEAVY
pub(crate) const UNDERLINE_TYPE_WAVY_HEAVY: i8 = /* TODO */;
// Java field: byte UNDERLINE_TYPE_DOTTED_HEAVY
pub(crate) const UNDERLINE_TYPE_DOTTED_HEAVY: i8 = /* TODO */;
// Java field: byte UNDERLINE_TYPE_DASH_HEAVY
pub(crate) const UNDERLINE_TYPE_DASH_HEAVY: i8 = /* TODO */;
// Java field: byte UNDERLINE_TYPE_DOT_DASH_HEAVY
pub(crate) const UNDERLINE_TYPE_DOT_DASH_HEAVY: i8 = /* TODO */;
// Java field: byte UNDERLINE_TYPE_DOT_DOT_DASH_HEAVY
pub(crate) const UNDERLINE_TYPE_DOT_DOT_DASH_HEAVY: i8 = /* TODO */;
// Java field: byte UNDERLINE_TYPE_DASH_LONG
pub(crate) const UNDERLINE_TYPE_DASH_LONG: i8 = /* TODO */;
// Java field: byte UNDERLINE_TYPE_DASH_LONG_HEAVY
pub(crate) const UNDERLINE_TYPE_DASH_LONG_HEAVY: i8 = /* TODO */;
// Java field: byte UNDERLINE_TYPE_WAVY_DOUBLE
pub(crate) const UNDERLINE_TYPE_WAVY_DOUBLE: i8 = /* TODO */;
// Java field: byte DELETELINE_TYPE_NONE
pub(crate) const DELETELINE_TYPE_NONE: i8 = /* TODO */;
// Java field: byte DELETELINE_TYPE_SINGLE
pub(crate) const DELETELINE_TYPE_SINGLE: i8 = /* TODO */;
// Java field: byte DELETELINE_TYPE_DOUBLE
pub(crate) const DELETELINE_TYPE_DOUBLE: i8 = /* TODO */;
// Java field: float SUB_SUPER_SCALE
pub(crate) const SUB_SUPER_SCALE: f32 = /* TODO */;

// Java: public class SpanInfo
pub struct SpanInfo {
    // Java field: String mFontName
    pub(crate) mFontName: Arc<Mutex<String>>,
    // Java field: byte mFontFamily
    pub(crate) mFontFamily: i8,
    // Java field: byte mCharset
    pub(crate) mCharset: i8,
    // Java field: float mFontSize
    pub(crate) mFontSize: f32,
    // Java field: float mFontSizeOri
    pub(crate) mFontSizeOri: f32,
    // Java field: int mFontColor
    pub(crate) mFontColor: i32,
    // Java field: float mColorAlpha
    pub(crate) mColorAlpha: f32,
    // Java field: byte mUnderline
    pub(crate) mUnderline: i8,
    // Java field: int mUnderlineColor
    pub(crate) mUnderlineColor: i32,
    // Java field: FontFill mFontFill
    pub(crate) mFontFill: Arc<Mutex<FontFill>>,
    // Java field: TextOutline mTextOutline
    pub(crate) mTextOutline: Arc<Mutex<TextOutline>>,
    // Java field: TextShadow mTextShadow
    pub(crate) mTextShadow: Arc<Mutex<TextShadow>>,
    // Java field: TextImage mTextImage
    pub(crate) mTextImage: Arc<Mutex<TextImage>>,
    // Java field: TextBlur mTextBlur
    pub(crate) mTextBlur: Arc<Mutex<TextBlur>>,
    // Java field: boolean mIsSuperScript
    pub(crate) mIsSuperScript: bool,
    // Java field: boolean mIsSubScript
    pub(crate) mIsSubScript: bool,
    // Java field: boolean mIsBold
    pub(crate) mIsBold: bool,
    // Java field: byte mStrikethrough
    pub(crate) mStrikethrough: i8,
    // Java field: boolean mIsItalic
    pub(crate) mIsItalic: bool,
    // Java field: boolean mIsAccounting
    pub(crate) mIsAccounting: bool,
    // Java field: byte mTextCaps
    pub(crate) mTextCaps: i8,
    // Java field: boolean mIsNormalizeH
    pub(crate) mIsNormalizeH: bool,
    // Java field: int mStartPos
    pub(crate) mStartPos: i32,
    // Java field: int mEndPos
    pub(crate) mEndPos: i32,
    // Java field: boolean mArb
    pub(crate) mArb: bool,
    // Java field: float mSpc
    pub(crate) mSpc: f32,
    // Java field: float mKern
    pub(crate) mKern: f32,
    // Java field: float mAscent
    pub(crate) mAscent: f32,
}

// Java: public class SpanInfo
impl SpanInfo {
    // Java: SpanInfo()
    pub fn SpanInfo(&self) {
      // Java: super();
      // TODO_RUST_CODE
      // Java: this.mFontName = "times New Roman";
      // TODO_RUST_CODE
      // Java: this.mFontColor = -16777216;
      // TODO_RUST_CODE
      // Java: this.mFontSize = 40.0F;
      // TODO_RUST_CODE
      // Java: this.mStartPos = 0;
      // TODO_RUST_CODE
      // Java: this.mEndPos = 0;
      // TODO_RUST_CODE
      // Java: this.mUnderline = 0;
      // TODO_RUST_CODE
      // Java: this.mUnderlineColor = this.mFontColor;
      // TODO_RUST_CODE
      // Java: this.mFontSizeOri = this.mFontSize;
      // TODO_RUST_CODE
    }

    // Java: SpanInfo(SpanInfo span)
    pub fn SpanInfo(&self, SpanInfo span) {
      // Java: super();
      // TODO_RUST_CODE
      // Java: this.mFontName = span.mFontName;
      // TODO_RUST_CODE
      // Java: this.mFontFamily = span.mFontFamily;
      // TODO_RUST_CODE
      // Java: this.mCharset = span.mCharset;
      // TODO_RUST_CODE
      // Java: this.mFontSize = span.mFontSize;
      // TODO_RUST_CODE
      // Java: this.mFontSizeOri = span.mFontSize;
      // TODO_RUST_CODE
      // Java: this.mFontColor = span.mFontColor;
      // TODO_RUST_CODE
      // Java: this.mUnderline = span.mUnderline;
      // TODO_RUST_CODE
      // Java: this.mUnderlineColor = span.mUnderlineColor;
      // TODO_RUST_CODE
      // Java: this.mIsSuperScript = span.mIsSuperScript;
      // TODO_RUST_CODE
      // Java: this.mIsSubScript = span.mIsSubScript;
      // TODO_RUST_CODE
      // Java: this.mIsBold = span.mIsBold;
      // TODO_RUST_CODE
      // Java: this.mStrikethrough = span.mStrikethrough;
      // TODO_RUST_CODE
      // Java: this.mIsItalic = span.mIsItalic;
      // TODO_RUST_CODE
      // Java: this.mIsAccounting = span.mIsAccounting;
      // TODO_RUST_CODE
      // Java: this.mTextCaps = span.mTextCaps;
      // TODO_RUST_CODE
      // Java: this.mIsNormalizeH = span.mIsNormalizeH;
      // TODO_RUST_CODE
      // Java: this.mStartPos = span.mStartPos;
      // TODO_RUST_CODE
      // Java: this.mEndPos = span.mEndPos;
      // TODO_RUST_CODE
      // Java: this.adjustPos();
      // TODO_RUST_CODE
      // Java: this.mAscent = span.mAscent;
      // TODO_RUST_CODE
      // Java: this.mFontFill = span.mFontFill;
      // TODO_RUST_CODE
      // Java: this.mTextOutline = span.mTextOutline;
      // TODO_RUST_CODE
      // Java: this.mTextShadow = span.mTextShadow;
      // TODO_RUST_CODE
      // Java: this.mTextImage = span.mTextImage;
      // TODO_RUST_CODE
      // Java: this.mTextBlur = span.mTextBlur;
      // TODO_RUST_CODE
      // Java: this.mSpc = span.mSpc;
      // TODO_RUST_CODE
      // Java: this.mKern = span.mKern;
      // TODO_RUST_CODE
      // Java: this.mArb = span.mArb;
      // TODO_RUST_CODE
    }

    // Java: SpanInfo(String fontName, byte fontFamily, byte charset, float fontSize, int fontColor, byte underline, boolean isSuperScript, boolean isSubScript, boolean isBold, byte strikethrough, boolean isItalic, int startPos, int endPos, int underlineColor, FontFill fontFill, TextOutline txtOutline, TextShadow txtShadow, TextImage txtImage, TextBlur txtBlur)
    pub fn SpanInfo(&self, String fontName, byte fontFamily, byte charset, float fontSize, int fontColor, byte underline, boolean isSuperScript, boolean isSubScript, boolean isBold, byte strikethrough, boolean isItalic, int startPos, int endPos, int underlineColor, FontFill fontFill, TextOutline txtOutline, TextShadow txtShadow, TextImage txtImage, TextBlur txtBlur) {
      // Java: super();
      // TODO_RUST_CODE
      // Java: this.mFontName = fontName;
      // TODO_RUST_CODE
      // Java: this.mFontFamily = fontFamily;
      // TODO_RUST_CODE
      // Java: this.mCharset = charset;
      // TODO_RUST_CODE
      // Java: this.mFontSize = fontSize;
      // TODO_RUST_CODE
      // Java: this.mFontSizeOri = fontSize;
      // TODO_RUST_CODE
      // Java: this.mFontColor = fontColor;
      // TODO_RUST_CODE
      // Java: this.mUnderline = underline;
      // TODO_RUST_CODE
      // Java: this.mUnderlineColor = underlineColor;
      // TODO_RUST_CODE
      // Java: this.mIsSuperScript = isSuperScript;
      // TODO_RUST_CODE
      // Java: this.mIsSubScript = isSubScript;
      // TODO_RUST_CODE
      // Java: this.mIsBold = isBold;
      // TODO_RUST_CODE
      // Java: this.mStrikethrough = strikethrough;
      // TODO_RUST_CODE
      // Java: this.mIsItalic = isItalic;
      // TODO_RUST_CODE
      // Java: this.mFontFill = fontFill;
      // TODO_RUST_CODE
      // Java: this.mTextOutline = txtOutline;
      // TODO_RUST_CODE
      // Java: this.mTextShadow = txtShadow;
      // TODO_RUST_CODE
      // Java: this.mTextImage = txtImage;
      // TODO_RUST_CODE
      // Java: this.mTextBlur = txtBlur;
      // TODO_RUST_CODE
      // Java: this.mStartPos = startPos;
      // TODO_RUST_CODE
      // Java: this.mEndPos = endPos;
      // TODO_RUST_CODE
      // Java: this.adjustPos();
      // TODO_RUST_CODE
    }

    // Java: boolean equalsAttr(SpanInfo spanInfo)
    pub fn equalsAttr(&self, SpanInfo spanInfo) -> bool {
      // Java: if (spanInfo == null)
        // TODO_RUST_CODE
      {
         // Java: return false;
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: return this.mFontName.equals(spanInfo.mFontName) && this.mFontFamily == spanInfo.mFontFamily && this.mCharset == spanInfo.mCharset && this.mFontSize == spanInfo.mFontSize && this.mFontSizeOri == spanInfo.mFontSizeOri && this.mFontColor == spanInfo.mFontColor && this.mUnderline == spanInfo.mUnderline && this.mIsBold == spanInfo.mIsBold && this.mIsItalic == spanInfo.mIsItalic && this.mStrikethrough == spanInfo.mStrikethrough && this.mIsSuperScript == spanInfo.mIsSuperScript && this.mIsSubScript == spanInfo.mIsSubScript && this.mUnderlineColor == spanInfo.mUnderlineColor && this.mTextCaps == spanInfo.mTextCaps && this.mIsNormalizeH == spanInfo.mIsNormalizeH && this.mArb == spanInfo.mArb && (this.mFontFill == null && spanInfo.mFontFill == null || this.mFontFill != null && this.mFontFill.equals(spanInfo.mFontFill)) && (this.mTextOutline == null && spanInfo.mTextOutline == null || this.mTextOutline != null && this.mTextOutline.equals(spanInfo.mTextOutline)) && (this.mTextShadow == null && spanInfo.mTextShadow == null || this.mTextShadow != null && this.mTextShadow.equals(spanInfo.mTextShadow)) && (this.mTextImage == null && spanInfo.mTextImage == null || this.mTextImage != null && this.mTextImage.equals(spanInfo.mTextImage)) && (this.mTextBlur == null && spanInfo.mTextBlur == null || this.mTextBlur != null && this.mTextBlur.equals(spanInfo.mTextBlur)) && this.mSpc == spanInfo.mSpc && this.mKern == spanInfo.mKern;
         // TODO_RUST_CODE
      }
    }

    // Java: boolean hasSpc()
    pub fn hasSpc(&self) -> bool {
      // Java: return this.mSpc != 0.0F;
      // TODO_RUST_CODE
    }

    // Java: float getSpc()
    pub fn getSpc(&self) -> f32 {
      // Java: return this.mSpc;
      // TODO_RUST_CODE
    }

    // Java: boolean hasFill()
    pub fn hasFill(&self) -> bool {
      // Java: return this.mFontFill != null && this.mFontFill.mFillType != FontFill.FontFillType.NONE || this.mFontFill == null;
      // TODO_RUST_CODE
    }

    // Java: boolean hasArtFill()
    pub fn hasArtFill(&self) -> bool {
      // Java: return this.mFontFill != null && this.mFontFill.mFillType != FontFill.FontFillType.COLORFILL;
      // TODO_RUST_CODE
    }

    // Java: boolean hasRectShapeGradFill()
    pub fn hasRectShapeGradFill(&self) -> bool {
      // Java: if (this.mFontFill != null && this.mFontFill.mFillType == FontFill.FontFillType.GRADFILL && this.mFontFill.mGradFill != null && this.mFontFill.mGradFill.getGradAttr() != null)
        // TODO_RUST_CODE
      {
         // Java: GradientAttr.GradType type = this.mFontFill.mGradFill.getGradAttr().gradType();
         // TODO_RUST_CODE
         // Java: return type == GradientAttr.GradType.RECT || type == GradientAttr.GradType.SHAPE;
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: return false;
         // TODO_RUST_CODE
      }
    }

    // Java: boolean hasOutline()
    pub fn hasOutline(&self) -> bool {
      // Java: return this.mTextOutline != null && this.mTextOutline.getTxtOutlnType() != TextOutline.TxtOutlnType.NONE;
      // TODO_RUST_CODE
    }

    // Java: boolean hasBlur()
    pub fn hasBlur(&self) -> bool {
      // Java: return this.mTextBlur != null && (this.hasFill() || this.hasOutline());
      // TODO_RUST_CODE
    }

    // Java: boolean hasTextImage()
    pub fn hasTextImage(&self) -> bool {
      // Java: return this.mTextImage != null && (this.hasFill() || this.hasOutline());
      // TODO_RUST_CODE
    }

    // Java: boolean hasOuterShdw()
    pub fn hasOuterShdw(&self) -> bool {
      // Java: return this.mTextShadow != null && this.mTextShadow.getShdwType() == TextShadow.ShadowType.OUTER && (this.hasFill() || this.hasOutline());
      // TODO_RUST_CODE
    }

    // Java: boolean hasInnerShdw()
    pub fn hasInnerShdw(&self) -> bool {
      // Java: return this.mTextShadow != null && this.mTextShadow.getShdwType() == TextShadow.ShadowType.INNER && (this.hasFill() || this.hasOutline());
      // TODO_RUST_CODE
    }

    // Java: boolean hasStrikethroughLine()
    pub fn hasStrikethroughLine(&self) -> bool {
      // Java: return this.mStrikethrough != 100;
      // TODO_RUST_CODE
    }

    // Java: boolean hasUnderline()
    pub fn hasUnderline(&self) -> bool {
      // Java: return this.mUnderline != 0;
      // TODO_RUST_CODE
    }

    // Java: boolean isAcounting()
    pub fn isAcounting(&self) -> bool {
      // Java: return this.mIsAccounting;
      // TODO_RUST_CODE
    }

    // Java: boolean isUnderlineHeavy()
    pub fn isUnderlineHeavy(&self) -> bool {
      // Java: return 4 == this.mUnderline || 10 == this.mUnderline || 11 == this.mUnderline || 12 == this.mUnderline || 13 == this.mUnderline || 14 == this.mUnderline || 16 == this.mUnderline;
      // TODO_RUST_CODE
    }

    // Java: boolean contains(int pos)
    pub fn contains(&self, int pos) -> bool {
      // Java: return pos >= this.mStartPos && pos < this.mEndPos && this.mStartPos != this.mEndPos;
      // TODO_RUST_CODE
    }

    // Java: float getAscent()
    pub fn getAscent(&self) -> f32 {
      // Java: if (0.0F == this.mAscent)
        // TODO_RUST_CODE
      {
         // Java: Paint.FontMetricsInt fontMetrics = new Paint.FontMetricsInt();
         // TODO_RUST_CODE
         // Java: PaintFontHelper.getFontMetricsInt(fontMetrics, this);
         // TODO_RUST_CODE
         // Java: this.mAscent = (float)fontMetrics.ascent;
         // TODO_RUST_CODE
      }
      // Java: return this.mAscent;
      // TODO_RUST_CODE
    }

    // Java: void resetAscent()
    pub fn resetAscent(&self) {
      // Java: this.mAscent = 0.0F;
      // TODO_RUST_CODE
    }

    // Java: void adjustPos()
    pub fn adjustPos(&self) {
      // Java: this.mEndPos = Math.max(this.mEndPos, 0);
      // TODO_RUST_CODE
      // Java: this.mStartPos = Math.max(this.mStartPos, 0);
      // TODO_RUST_CODE
      // Java: this.mStartPos = Math.min(this.mStartPos, this.mEndPos);
      // TODO_RUST_CODE
    }

    // Java: void copyFrom(SpanInfo span)
    pub fn copyFrom(&self, SpanInfo span) {
      // Java: this.mFontName = span.mFontName;
      // TODO_RUST_CODE
      // Java: this.mFontFamily = span.mFontFamily;
      // TODO_RUST_CODE
      // Java: this.mCharset = span.mCharset;
      // TODO_RUST_CODE
      // Java: this.mFontSize = span.mFontSize;
      // TODO_RUST_CODE
      // Java: this.mFontSizeOri = span.mFontSizeOri;
      // TODO_RUST_CODE
      // Java: this.mFontColor = span.mFontColor;
      // TODO_RUST_CODE
      // Java: this.mUnderline = span.mUnderline;
      // TODO_RUST_CODE
      // Java: this.mUnderlineColor = span.mUnderlineColor;
      // TODO_RUST_CODE
      // Java: this.mIsSuperScript = span.mIsSuperScript;
      // TODO_RUST_CODE
      // Java: this.mIsSubScript = span.mIsSubScript;
      // TODO_RUST_CODE
      // Java: this.mIsBold = span.mIsBold;
      // TODO_RUST_CODE
      // Java: this.mStrikethrough = span.mStrikethrough;
      // TODO_RUST_CODE
      // Java: this.mIsItalic = span.mIsItalic;
      // TODO_RUST_CODE
      // Java: this.mIsAccounting = span.mIsAccounting;
      // TODO_RUST_CODE
      // Java: this.mTextCaps = span.mTextCaps;
      // TODO_RUST_CODE
      // Java: this.mIsNormalizeH = span.mIsNormalizeH;
      // TODO_RUST_CODE
      // Java: this.mStartPos = span.mStartPos;
      // TODO_RUST_CODE
      // Java: this.mEndPos = span.mEndPos;
      // TODO_RUST_CODE
      // Java: this.adjustPos();
      // TODO_RUST_CODE
      // Java: this.mAscent = span.mAscent;
      // TODO_RUST_CODE
      // Java: this.mFontFill = span.mFontFill;
      // TODO_RUST_CODE
      // Java: this.mTextOutline = span.mTextOutline;
      // TODO_RUST_CODE
      // Java: this.mTextShadow = span.mTextShadow;
      // TODO_RUST_CODE
      // Java: this.mTextImage = span.mTextImage;
      // TODO_RUST_CODE
      // Java: this.mTextBlur = span.mTextBlur;
      // TODO_RUST_CODE
      // Java: this.mSpc = span.mSpc;
      // TODO_RUST_CODE
      // Java: this.mArb = span.mArb;
      // TODO_RUST_CODE
    }

    // Java: void clear()
    pub fn clear(&self) {
      // Java: this.mFontName = "times New Roman";
      // TODO_RUST_CODE
      // Java: this.mFontFamily = 0;
      // TODO_RUST_CODE
      // Java: this.mCharset = 0;
      // TODO_RUST_CODE
      // Java: this.mFontColor = -16777216;
      // TODO_RUST_CODE
      // Java: this.mFontSize = 40.0F;
      // TODO_RUST_CODE
      // Java: this.mStartPos = 0;
      // TODO_RUST_CODE
      // Java: this.mEndPos = 0;
      // TODO_RUST_CODE
      // Java: this.mUnderline = 0;
      // TODO_RUST_CODE
      // Java: this.mUnderlineColor = this.mFontColor;
      // TODO_RUST_CODE
      // Java: this.mFontSizeOri = this.mFontSize;
      // TODO_RUST_CODE
      // Java: this.mIsSuperScript = false;
      // TODO_RUST_CODE
      // Java: this.mIsSubScript = false;
      // TODO_RUST_CODE
      // Java: this.mIsBold = false;
      // TODO_RUST_CODE
      // Java: this.mStrikethrough = 100;
      // TODO_RUST_CODE
      // Java: this.mIsItalic = false;
      // TODO_RUST_CODE
      // Java: this.mIsAccounting = false;
      // TODO_RUST_CODE
      // Java: this.mTextCaps = 1;
      // TODO_RUST_CODE
      // Java: this.mIsNormalizeH = false;
      // TODO_RUST_CODE
      // Java: this.mFontFill = null;
      // TODO_RUST_CODE
      // Java: this.mTextOutline = null;
      // TODO_RUST_CODE
      // Java: this.mTextShadow = null;
      // TODO_RUST_CODE
      // Java: this.mTextImage = null;
      // TODO_RUST_CODE
      // Java: this.mTextBlur = null;
      // TODO_RUST_CODE
      // Java: this.mAscent = 0.0F;
      // TODO_RUST_CODE
      // Java: this.mSpc = 0.0F;
      // TODO_RUST_CODE
      // Java: this.mKern = 0.0F;
      // TODO_RUST_CODE
      // Java: this.mArb = false;
      // TODO_RUST_CODE
    }

    // Java: boolean hasKern()
    pub fn hasKern(&self) -> bool {
      // Java: return this.mKern != 0.0F;
      // TODO_RUST_CODE
    }

    // Java: float getKern()
    pub fn getKern(&self) -> f32 {
      // Java: return this.mKern;
      // TODO_RUST_CODE
    }

}
