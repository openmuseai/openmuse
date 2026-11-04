use crate::cn::wps::moss::{
    app::{xf::KmoFont::KmoFont, UnitsConverter::UnitsConverter},
    core::book::KmoPalette::KmoPalette,
    util::KmoColor::KmoColor,
};
use serde::{Deserialize, Serialize};
// java public class SpanInfo
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct SpanInfo {
    pub mFontName: String,
    pub mFontFamily: i8,
    pub mCharset: i8,
    pub mRawFontSize: i32,
    #[serde(serialize_with = "crate::java_runtime::serialize_java_float")]
    pub mFontSize: f32,
    pub mFontColor: i32,
    pub mUnderline: i8,
    pub mIsSuperScript: bool,
    pub mIsSubScript: bool,
    pub mIsBold: bool,
    pub mIsStrikeout: bool,
    pub mIsItalic: bool,
    pub mStartPos: i32,
    pub mEndPos: i32,
}
impl SpanInfo {
    pub const AUTO_FONT_COLOR: i32 = -16777216;
    // java public boolean reset(UnitsConverter converter, KmoPalette palette, KmoFont kmoFont, int startPos, int endPos) {
    // java this.mFontName = kmoFont.getFontName();
    // java this.mFontFamily = kmoFont.getFamily();
    // java this.mRawFontSize = kmoFont.getFontHeight();
    // java this.mFontSize = converter.TwipsToPixels((float)this.mRawFontSize);
    // java this.mFontColor = kmoFont.getColorPaletteIndex();
    // java if (ColorToolKit.isPaletteIndex(this.mFontColor)) {
    // java this.mFontColor = palette.getRGBColor((short)this.mFontColor);
    // java }
    // java
    // java if (KmoColor.isAutomaticColor(this.mFontColor)) {
    // java this.mFontColor = -16777216;
    // java }
    // java
    // java this.mUnderline = kmoFont.getUnderline();
    // java short script = kmoFont.getSuperSubScript();
    // java this.mIsSubScript = script == 2;
    // java this.mIsSuperScript = script == 1;
    // java this.mIsBold = kmoFont.getBoldWeight() == 700;
    // java this.mIsStrikeout = kmoFont.isStrikeout();
    // java this.mIsItalic = kmoFont.isItalic();
    // java this.mStartPos = startPos;
    // java this.mEndPos = endPos;
    // java if (this.mIsSubScript || this.mIsSuperScript) {
    // java this.mFontSize *= 0.75F;
    // java }
    // java
    // java return true;
    // java }
    pub fn reset(
        &mut self,
        c: &UnitsConverter,
        p: &KmoPalette,
        f: &KmoFont,
        start: i32,
        end: i32,
    ) -> bool {
        self.mFontName = f.getFontName();
        self.mFontFamily = f.getFamily();
        self.mRawFontSize = f.getFontHeight() as i32;
        self.mFontSize = c.TwipsToPixels(self.mRawFontSize as f32);
        self.mFontColor = f.getColorPaletteIndex();
        if (self.mFontColor >> 24) & 255 == 0 {
            self.mFontColor = p.getRGBColor(self.mFontColor as i16);
        }
        if KmoColor::isAutomaticColor(self.mFontColor) {
            self.mFontColor = -16777216;
        }
        self.mUnderline = f.getUnderline();
        let script = f.getSuperSubScript();
        self.mIsSubScript = script == 2;
        self.mIsSuperScript = script == 1;
        self.mIsBold = f.getBoldWeight() == 700;
        self.mIsStrikeout = f.isStrikeout();
        self.mIsItalic = f.isItalic();
        self.mStartPos = start;
        self.mEndPos = end;
        if self.mIsSubScript || self.mIsSuperScript {
            self.mFontSize *= 0.75;
        }
        true
    }
    // java public final boolean hasUnderline() {
    // java return this.mUnderline != 0;
    // java }
    pub fn hasUnderline(&self) -> bool {
        self.mUnderline != 0
    }
    // java public final boolean isAcounting() {
    // java return this.mUnderline == 33 || this.mUnderline == 34;
    // java }
    pub fn isAcounting(&self) -> bool {
        self.mUnderline == 33 || self.mUnderline == 34
    }
    // java public final boolean isSUnderline() {
    // java return this.mUnderline == 1 || this.mUnderline == 33;
    // java }
    pub fn isSUnderline(&self) -> bool {
        self.mUnderline == 1 || self.mUnderline == 33
    }
    // java public final boolean isDUnderline() {
    // java return this.mUnderline == 2 || this.mUnderline == 34;
    // java }
    pub fn isDUnderline(&self) -> bool {
        self.mUnderline == 2 || self.mUnderline == 34
    }
    // java public final boolean contains(int pos) {
    // java return pos >= this.mStartPos && pos < this.mEndPos && this.mStartPos != this.mEndPos;
    // java }
    pub fn contains(&self, pos: i32) -> bool {
        pos >= self.mStartPos && pos < self.mEndPos && self.mStartPos != self.mEndPos
    }
    // java public boolean equalsAttr(SpanInfo spanInfo) {
    // java return this.mFontName.equals(spanInfo.mFontName) && this.mFontFamily == spanInfo.mFontFamily && this.mRawFontSize == spanInfo.mRawFontSize && this.mFontColor == spanInfo.mFontColor && this.mUnderline == spanInfo.mUnderline && this.mIsBold == spanInfo.mIsBold && this.mIsItalic == spanInfo.mIsItalic && this.mIsStrikeout == spanInfo.mIsStrikeout && this.mIsSubScript == spanInfo.mIsSubScript;
    // java }
    pub fn equalsAttr(&self, s: &Self) -> bool {
        self.mFontName == s.mFontName
            && self.mFontFamily == s.mFontFamily
            && self.mRawFontSize == s.mRawFontSize
            && self.mFontColor == s.mFontColor
            && self.mUnderline == s.mUnderline
            && self.mIsBold == s.mIsBold
            && self.mIsItalic == s.mIsItalic
            && self.mIsStrikeout == s.mIsStrikeout
            && self.mIsSubScript == s.mIsSubScript
    }
    // java public final float getAscent() {
    // java Paint.FontMetricsInt fontMetrics = new Paint.FontMetricsInt();
    // java PaintFontHelper.getFontMetricsInt(fontMetrics, this.mFontName, this.mFontSize, this.mIsBold, this.mIsItalic);
    // java return (float)fontMetrics.ascent;
    // java }
    pub fn getAscent(&self) -> f32 {
        crate::cn::wps::text::layout::typo::PaintFontHelper::PaintFontHelper::getFontMetricsInt(
            &self.mFontName,
            self.mFontSize,
            self.mIsBold,
            self.mIsItalic,
        )
        .ascent as f32
    }
}
