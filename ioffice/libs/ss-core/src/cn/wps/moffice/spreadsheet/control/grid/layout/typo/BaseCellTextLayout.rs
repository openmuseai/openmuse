use super::super::super::{
    core::GridSheetLayoutInfo::GridSheetLayoutInfo,
    render::graphics::{Paint, PointF, Rect},
};
use super::super::data::{
    CellTextData::CellTextData, TextData::TextData, UnderlineCache::UnderlineCache,
};
use super::super::layouter::BidiTools::BidiTools;
use super::PaintFontHelper::PaintFontHelper;
// java public abstract class BaseCellTextLayout implements ICellTextLayout
#[derive(Default)]
pub struct BaseCellTextLayout {
    pub mTypoOffset: PointF,
    pub mTextPaint: Paint,
    pub mTextRect: Rect,
    pub rtl: bool,
    pub mTextData: Option<TextData>,
    pub mUnderlineCache: UnderlineCache,
}
impl BaseCellTextLayout {
    // java public void typo(GridSheetLayoutInfo layoutInfo, CellTextData data, int width, int height, boolean forceTypo) {
    // java this.mTextData = data;
    // java this.mTextRect.set(0, 0, width, height);
    // java this.rtl = layoutInfo.isSheetRightToLeft() && BidiTools.needRtl(data.mSourceText);
    // java Rect var10000 = this.mTextRect;
    // java var10000.right += data.mCenterCrossWidth;
    // java if (data.mPara.isAlignFill()) {
    // java this.modifyAlignFillText(data, this.mTextRect.width());
    // java }
    // java
    // java }
    pub fn typo(
        &mut self,
        l: &GridSheetLayoutInfo,
        d: &mut CellTextData,
        w: i32,
        h: i32,
        _force: bool,
    ) {
        self.mTextRect = Rect::new(0, 0, w, h);
        self.rtl = l.isSheetRightToLeft() && BidiTools::needRtl(d.source());
        self.mTextRect.right += d.mCenterCrossWidth;
        if d.mPara.isAlignFill() {
            self.modifyAlignFillText(d, self.mTextRect.width());
        }
        self.mTextData = Some(d.base.clone());
    }
    // java protected void modifyAlignFillText(CellTextData data, int typoWidth) {
    // java SpanInfo span = data.mFirstSpan;
    // java int textWidth = (int)PaintFontHelper.getCharWidth(data.mSourceText, span.mFontName, span.mFontSize, span.mIsBold, span.mIsItalic);
    // java int sumTxtWidth = textWidth * 2;
    // java if (sumTxtWidth < typoWidth && textWidth > 0) {
    // java StringBuffer buffer = new StringBuffer(data.mSourceText);
    // java
    // java do {
    // java buffer.append(data.mSourceText);
    // java sumTxtWidth += textWidth;
    // java } while(sumTxtWidth <= typoWidth);
    // java
    // java data.mSourceText = buffer.toString();
    // java }
    // java
    // java }
    fn modifyAlignFillText(&self, d: &mut CellTextData, w: i32) {
        let textWidth = PaintFontHelper::getCharWidth__span(d.source(), &d.mFirstSpan) as i32;
        let mut sum = textWidth.wrapping_mul(2);
        if sum < w && textWidth > 0 {
            let source = d.source().to_vec();
            let mut buffer = source.clone();
            loop {
                buffer.extend(&source);
                sum = sum.wrapping_add(textWidth);
                if sum > w {
                    break;
                }
            }
            d.mSourceText = Some(buffer);
        }
    }
    // java public void clear() {
    // java this.mTextData = null;
    // java }
    pub fn clear(&mut self) {
        self.mTextData = None;
    }
}
