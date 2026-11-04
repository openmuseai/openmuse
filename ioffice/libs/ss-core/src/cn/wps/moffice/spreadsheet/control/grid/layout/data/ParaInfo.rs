use crate::cn::wps::moss::{
    app::{xf::KmoXF::KmoXF, UnitsConverter::UnitsConverter},
    util::KmoCellAlignUtils::KmoCellAlignUtils,
};
use serde::{Deserialize, Serialize};
// java public class ParaInfo
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ParaInfo {
    pub mVAlignment: i16,
    pub mHAlignment: i16,
    #[serde(skip_serializing)]
    pub mRawHAlign: i16,
    pub mIndent: i16,
    pub mAngle: i16,
    pub mIsWrapText: bool,
    pub mIsShrinkToFit: bool,
    pub mIsVerticalText: bool,
    pub mIsRightToLeft: bool,
}
impl ParaInfo {
    // java public boolean reset(KmoXF xf, int cellType, UnitsConverter converter) {
    // java this.mIndent = (short)((int)((float)xf.getIndent() * converter.PointsToPixels(16.0F)));
    // java this.mAngle = xf.getRotation();
    // java if (this.mAngle == 255) {
    // java this.mAngle = 0;
    // java this.mIsVerticalText = true;
    // java } else {
    // java this.mIsVerticalText = false;
    // java }
    // java
    // java this.mVAlignment = xf.getVerticalAlignment();
    // java this.mRawHAlign = xf.getAlignment();
    // java this.mHAlignment = KmoCellAlignUtils.calcFinalHAlign(this.mRawHAlign, this.mAngle, cellType);
    // java this.mIsWrapText = xf.getWrapText();
    // java this.mIsShrinkToFit = xf.getShrinkToFit();
    // java if (this.mIsVerticalText && !this.mIsWrapText) {
    // java this.mIsWrapText = true;
    // java } else if (this.mIsWrapText && this.mHAlignment == 4) {
    // java this.mIsWrapText = false;
    // java }
    // java
    // java return true;
    // java }
    pub fn reset(&mut self, xf: &KmoXF, cellType: i32, converter: &UnitsConverter) -> bool {
        self.mIndent = (xf.getIndent() as f32 * converter.PointsToPixels(16.0)) as i32 as i16;
        self.mAngle = xf.getRotation();
        if self.mAngle == 255 {
            self.mAngle = 0;
            self.mIsVerticalText = true;
        } else {
            self.mIsVerticalText = false;
        }
        self.mVAlignment = xf.getVerticalAlignment();
        self.mRawHAlign = xf.getAlignment();
        self.mHAlignment =
            KmoCellAlignUtils::calcFinalHAlign(self.mRawHAlign, self.mAngle, cellType);
        self.mIsWrapText = xf.getWrapText();
        self.mIsShrinkToFit = xf.getShrinkToFit();
        if self.mIsVerticalText && !self.mIsWrapText {
            self.mIsWrapText = true;
        } else if self.mIsWrapText && self.mHAlignment == 4 {
            self.mIsWrapText = false;
        }
        true
    }
    // java public final boolean isAlignFill() {
    // java return this.mHAlignment == 4;
    // java }
    pub fn isAlignFill(&self) -> bool {
        self.mHAlignment == 4
    }
    // java public final boolean isCenterCross() {
    // java return this.mHAlignment == 6;
    // java }
    pub fn isCenterCross(&self) -> bool {
        self.mHAlignment == 6
    }
    // java public final void setRightToLeft(boolean isRightToLeft) {
    // java this.mIsRightToLeft = isRightToLeft;
    // java if (isRightToLeft && this.mAngle == 0 && this.mRawHAlign == 0) {
    // java this.mHAlignment = 3;
    // java }
    // java
    // java }
    pub fn setRightToLeft(&mut self, v: bool) {
        self.mIsRightToLeft = v;
        if v && self.mAngle == 0 && self.mRawHAlign == 0 {
            self.mHAlignment = 3;
        }
    }
    // java public final void modifyHAlignment(boolean numericAsText) {
    // java if (this.mRawHAlign == 0 && numericAsText) {
    // java this.mHAlignment = 1;
    // java }
    // java
    // java }
    pub fn modifyHAlignment(&mut self, v: bool) {
        if self.mRawHAlign == 0 && v {
            self.mHAlignment = 1;
        }
    }
}
