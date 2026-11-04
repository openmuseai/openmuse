use super::super::super::{
    core::GridSheetLayoutInfo::GridSheetLayoutInfo,
    render::graphics::{Canvas, FontMetricsInt, Paint, PointF, Rect},
};
use super::super::layouter::BidiTools::BidiTools;
use super::super::{
    data::{CellTextData::CellTextData, ParaInfo::ParaInfo},
    ICellTextLayout::ICellTextLayout,
};
use super::BaseCellTextLayout::BaseCellTextLayout;
use super::PaintFontHelper::PaintFontHelper;
use super::TypoLayoutGen::TypoLayoutGen;
use crate::cn::wps::text::layout::{layouter::WpsLayout::WpsLayout, typo::layout::Layout::Layout};
#[derive(Default)]
pub struct CellRichTextLayout {
    pub base: BaseCellTextLayout,
    pub mTypoCache: Option<WpsLayout>,
}
impl std::ops::Deref for CellRichTextLayout {
    type Target = BaseCellTextLayout;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
impl std::ops::DerefMut for CellRichTextLayout {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}
impl CellRichTextLayout {
    // java private void doTypo(GridSheetLayoutInfo layoutInfo, CellTextData data, int width, int height, boolean forceTypo, boolean scaleToFit) {
    // java super.typo(layoutInfo, data, width, height, forceTypo);
    // java Rect var10000 = this.mTextRect;
    // java var10000.right -= data.mPara.mIndent;
    // java if (this.mTextRect.right < this.mTextRect.left) {
    // java this.mTextRect.right = this.mTextRect.left + 1;
    // java }
    // java
    // java this.mTypoCache = this.mTypoLayoutGen.getLayout(this.mTextRect, this.mTextPaint, data, forceTypo);
    // java this.setTextAlignment(data.mPara, this.mTextRect, this.mTextPaint, (float)this.mTypoCache.getWidth(), this.mTypoCache.getHeight());
    // java int overflow;
    // java if (!data.mPara.mIsWrapText && data.mPara.mAngle == 0) {
    // java overflow = (int)(this.mTypoCache.getLineWidth(0) - (float)this.mTextRect.width());
    // java } else {
    // java overflow = this.mTypoCache.getWidth() - this.mTextRect.width();
    // java }
    // java
    // java if (data.mPara.mIsShrinkToFit && overflow > 2 && scaleToFit) {
    // java float scale = (float)this.mTextRect.width() / (float)this.mTypoCache.getWidth();
    // java SpanInfo var12 = data.mFirstSpan;
    // java var12.mFontSize *= scale;
    // java if (data.mOtherSpans != null) {
    // java for(SpanInfo otherSpan : data.mOtherSpans) {
    // java otherSpan.mFontSize *= scale;
    // java }
    // java }
    // java
    // java this.doTypo(layoutInfo, data, width, height, forceTypo, false);
    // java } else {
    // java if (overflow > 2 && !data.mIsMergedCell && !data.mPara.isAlignFill()) {
    // java this.modifyTypoSpace(this.mTextRect, overflow, data, layoutInfo);
    // java } else {
    // java var10000 = this.mTextRect;
    // java var10000.right += data.mPara.mIndent;
    // java }
    // java
    // java this.mUnderlineCache.calcUnderline(this.mTypoCache, data);
    // java }
    // java }
    pub fn doTypo(
        &mut self,
        l: &GridSheetLayoutInfo,
        d: &mut CellTextData,
        w: i32,
        h: i32,
        force: bool,
        scaleToFit: bool,
    ) {
        self.base.typo(l, d, w, h, force);
        self.base.mTextRect.right -= d.mPara.mIndent as i32;
        if self.mTextRect.right < self.mTextRect.left {
            self.base.mTextRect.right = self.mTextRect.left + 1;
        }
        let rect = self.mTextRect;
        self.mTypoCache = Some(TypoLayoutGen::getLayout(
            &rect,
            &mut self.base.mTextPaint,
            d,
            force,
        ));
        let cache = self.mTypoCache.as_ref().unwrap();
        let (tw, th) = if d.mPara.mAngle != 0 {
            (cache.mAngleRectWidth as f32, cache.mAngleRectHeight as f32)
        } else {
            (cache.getWidth() as f32, cache.getHeight())
        };
        self.base.mTypoOffset = PointF {
            x: self.mTextRect.left as f32
                + self.calcHorizAlign(d, self.mTextRect.width() as f32, tw),
            y: self.mTextRect.top as f32
                + self.calcVertAlign(d, self.mTextRect.height() as f32, th),
        };
        let cache = self.mTypoCache.as_ref().unwrap();
        let overflow = if !d.mPara.mIsWrapText && d.mPara.mAngle == 0 {
            (cache.getLineWidth(0) - self.mTextRect.width() as f32) as i32
        } else {
            cache.getWidth() - self.mTextRect.width()
        };
        if d.mPara.mIsShrinkToFit && overflow > 2 && scaleToFit {
            let scale = self.mTextRect.width() as f32 / cache.getWidth() as f32;
            d.mFirstSpan.mFontSize *= scale;
            for s in &mut d.mOtherSpans {
                s.mFontSize *= scale;
            }
            self.doTypo(l, d, w, h, force, false);
        } else {
            if overflow > 2 && !d.mIsMergedCell && !d.mPara.isAlignFill() {
                self.modifyTypoSpace(overflow, d, l);
            } else {
                self.base.mTextRect.right += d.mPara.mIndent as i32;
            }
            self.base
                .mUnderlineCache
                .calcUnderline__layout(self.mTypoCache.as_ref().unwrap(), d);
        }
    }
    // java protected float calcHorizAlign(ParaInfo paraInfo, Paint paint, float cellWidth, float typoWidth) {
    // java float offset = 0.0F;
    // java switch (paraInfo.mHAlignment) {
    // java case 1:
    // java case 5:
    // java offset = (float)(2 + paraInfo.mIndent);
    // java break;
    // java case 2:
    // java case 6:
    // java offset = (cellWidth - typoWidth) / 2.0F;
    // java break;
    // java case 3:
    // java offset = -2.0F;
    // java offset += cellWidth - typoWidth;
    // java float italicFontSize = this.mTextData.italicFontSize();
    // java if (italicFontSize > 0.0F) {
    // java offset -= italicFontSize * 0.2F;
    // java }
    // java case 4:
    // java }
    // java
    // java return offset;
    // java }
    fn calcHorizAlign(&self, d: &CellTextData, w: f32, tw: f32) -> f32 {
        match d.mPara.mHAlignment {
            1 | 5 => (2 + d.mPara.mIndent as i32) as f32,
            2 | 6 => (w - tw) / 2.0,
            3 => {
                let mut o = -2.0;
                o += w - tw;
                let size = d.italicFontSize();
                if size > 0.0 {
                    o -= size * 0.2;
                }
                o
            }
            _ => 0.0,
        }
    }
    // java protected float calcVertAlign(ParaInfo paraInfo, Paint paint, float cellHeight, float typoHeight) {
    // java float offset = 0.0F;
    // java switch (paraInfo.mVAlignment) {
    // java case 0:
    // java case 3:
    // java offset = 0.0F;
    // java break;
    // java case 1:
    // java case 4:
    // java if (cellHeight - typoHeight > 0.0F) {
    // java offset = cellHeight * 0.5F;
    // java if (!this.mTextData.mIsRichText) {
    // java if (this.mTextData.mFirstSpan.mIsSuperScript) {
    // java offset -= typoHeight;
    // java if (offset < 0.0F) {
    // java offset = 0.0F;
    // java }
    // java } else if (this.mTextData.mFirstSpan.mIsSubScript) {
    // java offset -= typoHeight * 0.2F;
    // java if (offset > cellHeight - typoHeight) {
    // java offset -= typoHeight * 0.1F;
    // java }
    // java } else {
    // java offset -= typoHeight * 0.5F;
    // java }
    // java } else {
    // java offset -= typoHeight * 0.5F;
    // java }
    // java }
    // java break;
    // java case 2:
    // java offset = cellHeight - typoHeight;
    // java if (offset < 0.0F) {
    // java offset = 0.0F;
    // java }
    // java }
    // java
    // java return offset;
    // java }
    fn calcVertAlign(&self, d: &CellTextData, h: f32, th: f32) -> f32 {
        match d.mPara.mVAlignment {
            1 | 4 => {
                let mut o = 0.0;
                if h - th > 0.0 {
                    o = h * 0.5;
                    if !d.mIsRichText {
                        if d.mFirstSpan.mIsSuperScript {
                            o -= th;
                            if o < 0.0 {
                                o = 0.0;
                            }
                        } else if d.mFirstSpan.mIsSubScript {
                            o -= th * 0.2;
                            if o > h - th {
                                o -= th * 0.1;
                            }
                        } else {
                            o -= th * 0.5;
                        }
                    } else {
                        o -= th * 0.5;
                    }
                }
                o
            }
            2 => (h - th).max(0.0),
            _ => 0.0,
        }
    }
    // java private int modifyTypoSpace(Rect textRect, int needSpace, CellTextData data, GridSheetLayoutInfo layoutInfo) {
    // java int freeSpace = 0;
    // java switch (data.mPara.mHAlignment) {
    // java case 1:
    // java freeSpace = this.forwardSpace(needSpace, data.mRowIdx, data.mColIdx, layoutInfo, data.mSourceText);
    // java textRect.right += freeSpace;
    // java textRect.right += data.mPara.mIndent + 2;
    // java break;
    // java case 2:
    // java case 6:
    // java int halfSpace = (int)((float)needSpace * 0.5F);
    // java int backwardSpace = this.backwardSpace(halfSpace, data.mRowIdx, data.mColIdx, layoutInfo);
    // java int forwardSpace = 0;
    // java if (data.mPara.mHAlignment != 6) {
    // java forwardSpace = this.forwardSpace(halfSpace + 1, data.mRowIdx, data.mColIdx, layoutInfo, data.mSourceText);
    // java }
    // java
    // java freeSpace = forwardSpace + backwardSpace;
    // java textRect.left -= backwardSpace;
    // java textRect.right += forwardSpace;
    // java break;
    // java case 3:
    // java freeSpace = this.backwardSpace(needSpace, data.mRowIdx, data.mColIdx, layoutInfo);
    // java textRect.left -= freeSpace + 2;
    // java case 4:
    // java case 5:
    // java }
    // java
    // java return freeSpace;
    // java }
    fn modifyTypoSpace(&mut self, need: i32, d: &CellTextData, l: &GridSheetLayoutInfo) -> i32 {
        let mut free = 0;
        match d.mPara.mHAlignment {
            1 => {
                free = self.forwardSpace(need, d.mRowIdx, d.mColIdx, l, d.source());
                self.base.mTextRect.right += free;
                self.base.mTextRect.right += d.mPara.mIndent as i32 + 2;
            }
            2 | 6 => {
                let half = (need as f32 * 0.5) as i32;
                let back = self.backwardSpace(half, d.mRowIdx, d.mColIdx, l);
                let forward = if d.mPara.mHAlignment != 6 {
                    self.forwardSpace(half + 1, d.mRowIdx, d.mColIdx, l, d.source())
                } else {
                    0
                };
                free = forward + back;
                self.base.mTextRect.left -= back;
                self.base.mTextRect.right += forward;
            }
            3 => {
                free = self.backwardSpace(need, d.mRowIdx, d.mColIdx, l);
                self.base.mTextRect.left -= free + 2;
            }
            _ => {}
        }
        free
    }
    // java private int forwardSpace(int needSpace, int rowIdx, int colIdx, GridSheetLayoutInfo layoutInfo, String mSourceText) {
    // java int freeSpace = 0;
    // java KmoSheet sheet = layoutInfo.mGridSheet.coreSheet();
    // java if (layoutInfo.isSheetRightToLeft() && !BidiTools.needRtl(mSourceText)) {
    // java --colIdx;
    // java
    // java while(colIdx > 0 && !this.stopFindSpace(sheet, rowIdx, colIdx)) {
    // java freeSpace += layoutInfo.columnWidthInPixels(colIdx);
    // java if (freeSpace >= needSpace) {
    // java return needSpace;
    // java }
    // java
    // java --colIdx;
    // java }
    // java
    // java if (colIdx == 0) {
    // java freeSpace = needSpace + 16;
    // java }
    // java } else {
    // java ++colIdx;
    // java
    // java while(colIdx < layoutInfo.mMaxColumns && !this.stopFindSpace(sheet, rowIdx, colIdx)) {
    // java freeSpace += layoutInfo.columnWidthInPixels(colIdx);
    // java if (freeSpace >= needSpace) {
    // java return needSpace;
    // java }
    // java
    // java ++colIdx;
    // java }
    // java
    // java if (colIdx == layoutInfo.mMaxColumns) {
    // java freeSpace = needSpace + 16;
    // java }
    // java }
    // java
    // java return freeSpace;
    // java }
    fn forwardSpace(
        &self,
        need: i32,
        row: i32,
        mut col: i32,
        l: &GridSheetLayoutInfo,
        text: &[u16],
    ) -> i32 {
        let mut free = 0;
        let core = l.gridSheet().coreSheet();
        let sh = core.borrow();
        if l.isSheetRightToLeft() && !BidiTools::needRtl(text) {
            col -= 1;
            while col > 0 && !Self::stopFindSpace(&sh, row, col) {
                free += l.columnWidthInPixels(col);
                if free >= need {
                    return need;
                }
                col -= 1;
            }
            if col == 0 {
                free = need + 16;
            }
        } else {
            col += 1;
            while col < l.mMaxColumns && !Self::stopFindSpace(&sh, row, col) {
                free += l.columnWidthInPixels(col);
                if free >= need {
                    return need;
                }
                col += 1;
            }
            if col == l.mMaxColumns {
                free = need + 16;
            }
        }
        free
    }
    // java private int backwardSpace(int needSpace, int rowIdx, int colIdx, GridSheetLayoutInfo layoutInfo) {
    // java int freeSpace = 0;
    // java KmoSheet sheet = layoutInfo.mGridSheet.coreSheet();
    // java --colIdx;
    // java
    // java while(colIdx > -1 && !this.stopFindSpace(sheet, rowIdx, colIdx)) {
    // java freeSpace += layoutInfo.columnWidthInPixels(colIdx);
    // java if (freeSpace >= needSpace) {
    // java return needSpace;
    // java }
    // java
    // java --colIdx;
    // java }
    // java
    // java if (colIdx == -1) {
    // java freeSpace = needSpace;
    // java }
    // java
    // java return freeSpace;
    // java }
    fn backwardSpace(&self, need: i32, row: i32, mut col: i32, l: &GridSheetLayoutInfo) -> i32 {
        let core = l.gridSheet().coreSheet();
        let sh = core.borrow();
        let mut free = 0;
        col -= 1;
        while col > -1 && !Self::stopFindSpace(&sh, row, col) {
            free += l.columnWidthInPixels(col);
            if free >= need {
                return need;
            }
            col -= 1;
        }
        if col == -1 {
            free = need;
        }
        free
    }
    // java private final boolean stopFindSpace(KmoSheet sheet, int rowIdx, int colIdx) {
    // java boolean isBlankCell = sheet.isBlankCellValue(rowIdx, colIdx);
    // java return !isBlankCell || sheet.getMergedRange(rowIdx, colIdx) != null;
    // java }
    fn stopFindSpace(s: &crate::cn::wps::moss::app::KmoSheet::KmoSheet, r: i32, c: i32) -> bool {
        !s.isBlankCellValue(r, c) || s.getMergedRange(r, c).is_some()
    }
}
impl ICellTextLayout for CellRichTextLayout {
    // java public void typo(GridSheetLayoutInfo layoutInfo, CellTextData data, int width, int height, boolean forceTypo) {
    // java this.doTypo(layoutInfo, data, width, height, forceTypo, true);
    // java }
    fn typo(&mut self, l: &GridSheetLayoutInfo, d: &mut CellTextData, w: i32, h: i32, f: bool) {
        self.doTypo(l, d, w, h, f, true);
    }
    // java public void renderer(Canvas canvas) {
    // java canvas.save();
    // java
    // java try {
    // java canvas.clipRect(this.mTextRect);
    // java canvas.translate(this.mTypoOffset.x, this.mTypoOffset.y);
    // java this.mTypoCache.draw(canvas, (Path)null, (Paint)null, 0);
    // java this.mUnderlineCache.draw(canvas);
    // java } catch (Exception e) {
    // java e.printStackTrace();
    // java }
    // java
    // java canvas.restore();
    // java }
    fn renderer(&mut self, c: &mut Canvas) {
        c.save();
        c.clipRect(&self.mTextRect);
        c.translate(self.mTypoOffset.x, self.mTypoOffset.y);
        self.mTypoCache.as_ref().expect("layout").draw(c);
        self.base.mUnderlineCache.draw(c);
        c.restore();
    }
    fn getTextRect(&self) -> Rect {
        self.mTextRect
    }
    fn getTypoOffset(&self) -> PointF {
        self.mTypoOffset
    }
    fn clear(&mut self) {
        self.base.clear();
        self.mTypoCache = None;
    }
    fn className(&self) -> &'static str {
        "CellRichTextLayout"
    }
}
