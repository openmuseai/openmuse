use super::super::super::tools::MRUIntKeyCache::MRUIntKeyCache;
use crate::cn::wps::moss::framework::{EtFontOp::INSTANCE as ET_FONT_OP, FontInfo::FontInfo};
static FONT_HEIGHT_CACHE: std::sync::LazyLock<MRUIntKeyCache<i32>> =
    std::sync::LazyLock::new(|| MRUIntKeyCache::new(512));
use super::super::super::{
    core::GridSheetLayoutInfo::GridSheetLayoutInfo,
    render::graphics::{Paint, Rect},
};
use super::super::{
    cache::NumFmtResultCache::NumFmtResultCache,
    data::{CellTextData::CellTextData, TextData::TextData},
    typo::{PaintFontHelper::PaintFontHelper, TypoLayoutGen::TypoLayoutGen},
};
use crate::cn::wps::{
    moss::{app::UnitsConverter::UnitsConverter, util::KmoRect::KmoRect},
    text::layout::typo::layout::Layout::Layout,
};
// java public class TextLayoutMeasure
pub struct TextLayoutMeasure;
impl TextLayoutMeasure {
    // java public static int getCellTextEndCol(GridSheetLayoutInfo layoutInfo, int startRow, int endRow, int endCol) {
    // java if (endCol < 0) {
    // java endCol = 0;
    // java }
    // java
    // java float maxOverTextWidth = 0.0F;
    // java int maxTextRowIdx = 0;
    // java int maxTextColIdx = 0;
    // java short alignment = 1;
    // java KmoSheet sheet = layoutInfo.mGridSheet.coreSheet();
    // java CellTextData textData = new CellTextData();
    // java textData.clear();
    // java
    // java for(int row = startRow; row < endRow; ++row) {
    // java int col = 0;
    // java
    // java for(col = endCol; col > -1 && !textData.updateCellType(sheet, row, col); --col) {
    // java }
    // java
    // java if (textData.mCellType == 2) {
    // java int colWidth = layoutInfo.columnWidthInPixels(col);
    // java KmoXF xf = sheet.getCellXF(row, col);
    // java textData.updateCellPara(xf, layoutInfo.mUnitsConverter);
    // java if (textData.mPara.mHAlignment != 3 && !textData.mPara.mIsShrinkToFit && !textData.mPara.isAlignFill() && !textData.mPara.mIsWrapText && !textData.mPara.mIsVerticalText) {
    // java int rowHeight = layoutInfo.rowHeightInPixels(row);
    // java if (textData.updateCellSpan(layoutInfo, xf, colWidth, rowHeight)) {
    // java float textOverWidth = measureTypoWidth(textData, colWidth, rowHeight) - (float)colWidth;
    // java if (textData.mPara.mHAlignment == 2) {
    // java textOverWidth *= 0.5F;
    // java }
    // java
    // java if (textOverWidth > maxOverTextWidth) {
    // java int dif = 0;
    // java if (col < maxTextColIdx) {
    // java for(int i = col + 1; i < maxTextColIdx; ++i) {
    // java dif += layoutInfo.columnWidthInPixels(i);
    // java }
    // java }
    // java
    // java if (textOverWidth - (float)dif > maxOverTextWidth) {
    // java maxOverTextWidth = textOverWidth;
    // java maxTextRowIdx = row;
    // java maxTextColIdx = col;
    // java alignment = textData.mPara.mHAlignment;
    // java }
    // java }
    // java
    // java textData.clear();
    // java }
    // java }
    // java }
    // java }
    // java
    // java int needspace = (int)(maxOverTextWidth + 0.5F);
    // java if (needspace < 1) {
    // java return endCol;
    // java } else {
    // java return findEndCol(alignment, maxTextRowIdx, maxTextColIdx, needspace, layoutInfo);
    // java }
    // java }
    pub fn getCellTextEndCol(
        l: &GridSheetLayoutInfo,
        startRow: i32,
        endRow: i32,
        mut endCol: i32,
    ) -> i32 {
        if endCol < 0 {
            endCol = 0;
        }
        let mut max = 0.0f32;
        let (mut maxRow, mut maxCol) = (0, 0);
        let mut align = 1i16;
        let core = l.gridSheet().coreSheet();
        let mut d = CellTextData::default();
        d.clear();
        for row in startRow..endRow {
            let mut col = endCol;
            while col > -1 && !d.updateCellType(&core.borrow(), row, col) {
                col -= 1;
            }
            if d.mCellType == 2 {
                let width = l.columnWidthInPixels(col);
                let xf = core.borrow().getCellXF__2(row, col).expect("cell XF");
                d.updateCellPara(&xf, &l.mUnitsConverter);
                if d.mPara.mHAlignment != 3
                    && !d.mPara.mIsShrinkToFit
                    && !d.mPara.isAlignFill()
                    && !d.mPara.mIsWrapText
                    && !d.mPara.mIsVerticalText
                {
                    let height = l.rowHeightInPixels(row);
                    if d.updateCellSpan(l, &xf, width, height) {
                        let mut over = Self::measureTypoWidth(&d, width, height) - width as f32;
                        if d.mPara.mHAlignment == 2 {
                            over *= 0.5;
                        }
                        if over > max {
                            let mut diff = 0;
                            if col < maxCol {
                                for c in col + 1..maxCol {
                                    diff += l.columnWidthInPixels(c);
                                }
                            }
                            if over - diff as f32 > max {
                                max = over;
                                maxRow = row;
                                maxCol = col;
                                align = d.mPara.mHAlignment;
                            }
                        }
                        d.clear();
                    }
                }
            }
        }
        let need = (max + 0.5) as i32;
        if need < 1 {
            endCol
        } else {
            Self::findEndCol(align, maxRow, maxCol, need, l)
        }
    }
    // java private static int findEndCol(short alignment, int rowIdx, int colIdx, int needSpace, GridSheetLayoutInfo layoutInfo) {
    // java int endCol = 0;
    // java switch (alignment) {
    // java case 1:
    // java endCol = forwardSpace(needSpace, rowIdx, colIdx, layoutInfo);
    // java break;
    // java case 2:
    // java int halfSpace = (int)((float)needSpace * 0.5F);
    // java endCol = forwardSpace(halfSpace + 1, rowIdx, colIdx, layoutInfo);
    // java }
    // java
    // java return endCol;
    // java }
    fn findEndCol(alignment: i16, row: i32, col: i32, need: i32, l: &GridSheetLayoutInfo) -> i32 {
        match alignment {
            1 => Self::forwardSpace(need, row, col, l),
            2 => Self::forwardSpace((need as f32 * 0.5) as i32 + 1, row, col, l),
            _ => 0,
        }
    }
    // java private static int forwardSpace(int needSpace, int rowIdx, int colIdx, GridSheetLayoutInfo layoutInfo) {
    // java int freeSpace = 0;
    // java ++colIdx;
    // java
    // java while(colIdx < layoutInfo.mMaxColumnsLimit) {
    // java if (stopFindSpace(layoutInfo.mGridSheet, rowIdx, colIdx)) {
    // java return colIdx - 1;
    // java }
    // java
    // java freeSpace += layoutInfo.columnWidthInPixels(colIdx);
    // java if (freeSpace >= needSpace) {
    // java return colIdx;
    // java }
    // java
    // java ++colIdx;
    // java }
    // java
    // java return layoutInfo.mMaxColumnsLimit - 1;
    // java }
    fn forwardSpace(need: i32, row: i32, mut col: i32, l: &GridSheetLayoutInfo) -> i32 {
        let mut free = 0;
        col += 1;
        while col < l.mMaxColumnsLimit {
            if Self::stopFindSpace(l.gridSheet().as_ref(), row, col) {
                return col - 1;
            }
            free += l.columnWidthInPixels(col);
            if free >= need {
                return col;
            }
            col += 1;
        }
        l.mMaxColumnsLimit - 1
    }
    // java private static final boolean stopFindSpace(IGridSheet sheet, int rowIdx, int colIdx) {
    // java return sheet.getCellType(rowIdx, colIdx, (KmoSheet.CellTypeInfo)null) != 0 || sheet.getMergedRange(rowIdx, colIdx) != null;
    // java }
    fn stopFindSpace(
        sheet: &dyn crate::cn::wps::moss::app::IGridSheet::IGridSheet,
        row: i32,
        col: i32,
    ) -> bool {
        sheet.getCellType(row, col, None) != 0 || sheet.getMergedRange(row, col).is_some()
    }
    // java private static float measureAngleTextHeight(CellTextData data, int cellWidth, int cellHeight) {
    // java float typoHeight = 0.0F;
    // java BoringLayout.Metrics metrics = sBoringMetrics;
    // java PaintFontHelper.measureBoringMetrics(data.mSourceText, metrics, data.mFirstSpan, data.mOtherSpans);
    // java float sumTextWidth = (float)metrics.width;
    // java float radians = (float)Math.toRadians((double)Math.abs(data.mPara.mAngle));
    // java float sina = (float)Math.sin((double)radians);
    // java float perLineWidth = (float)cellHeight / sina;
    // java int lineCount = (int)(sumTextWidth / perLineWidth) + 1;
    // java if (lineCount < 2) {
    // java typoHeight = sumTextWidth * sina;
    // java } else {
    // java typoHeight = (float)cellHeight;
    // java }
    // java
    // java return typoHeight;
    // java }
    fn measureAngleTextHeight(d: &CellTextData, _width: i32, height: i32) -> f32 {
        let fm = PaintFontHelper::measureBoringMetrics(d.source(), &d.mFirstSpan, &d.mOtherSpans);
        let sum = fm.width as f32;
        let r = ((d.mPara.mAngle as f64).abs() * 0.017453292519943295) as f32;
        let sin = (r as f64).sin() as f32;
        let per = height as f32 / sin;
        let count = ((sum / per) as i32).wrapping_add(1);
        if count < 2 {
            sum * sin
        } else {
            height as f32
        }
    }

    // java public static void destroy() {
    // java mFontHeightCache.clear();
    // java }
    pub fn destroy() {
        FONT_HEIGHT_CACHE.clear();
    }
    // java public static float measureEmptyTypoHeight(GridSheetLayoutInfo layoutInfo, FontInfo font) {
    // java int hashCode = font.hashCode();
    // java Integer integer = mFontHeightCache.get(font.hashCode());
    // java if (integer != null) {
    // java return (float)integer;
    // java } else {
    // java short fontHeight = (short)(font.getFontSize() * 20);
    // java if (fontHeight < 240) {
    // java fontHeight = 240;
    // java }
    // java
    // java float fontSize = layoutInfo.mUnitsConverter.TwipsToPixels((float)fontHeight);
    // java String fontName = font.getFontName();
    // java Paint.FontMetricsInt metricsInt = sMetricsInt;
    // java PaintFontHelper.getFontMetricsInt(metricsInt, fontName, fontSize, false, false);
    // java int typoHeight = metricsInt.descent - metricsInt.ascent;
    // java mFontHeightCache.put(hashCode, typoHeight);
    // java return (float)typoHeight;
    // java }
    // java }
    pub fn measureEmptyTypoHeight(l: &GridSheetLayoutInfo, font: &FontInfo) -> f32 {
        let hash = font.hashCode();
        if let Some(h) = FONT_HEIGHT_CACHE.get(hash) {
            return h as f32;
        }
        let mut height = font.getFontSize().wrapping_mul(20) as i16;
        if height < 240 {
            height = 240;
        }
        let size = l.mUnitsConverter.TwipsToPixels(height as f32);
        let fm = PaintFontHelper::getFontMetricsInt__font(font.getFontName(), size, false, false);
        let h = fm.descent - fm.ascent;
        FONT_HEIGHT_CACHE.put(hash, Some(h));
        h as f32
    }
    // java public static float getAdjustedCellHeight(GridSheetLayoutInfo layoutInfo, int rowIdx, int colIdx, boolean bestFit) {
    // java KmoSheet sheet = layoutInfo.mGridSheet.coreSheet();
    // java FontInfo font = EtFontOp.INSTANCE.getFont(sheet, rowIdx, colIdx);
    // java return getAdjustedCellHeight(layoutInfo, rowIdx, colIdx, font, bestFit);
    // java }
    pub fn getAdjustedCellHeight(
        l: &GridSheetLayoutInfo,
        row: i32,
        col: i32,
        bestFit: bool,
    ) -> f32 {
        let font = ET_FONT_OP.getFont(&l.gridSheet().coreSheet().borrow(), row, col);
        Self::getAdjustedCellHeight__font(l, row, col, font.as_ref(), bestFit)
    }
    // java public static float getAdjustedCellHeight(GridSheetLayoutInfo layoutInfo, int rowIdx, int colIdx, FontInfo font, boolean bestFit) {
    // java KmoRect mergedRange = layoutInfo.mGridSheet.getMergedRange(rowIdx, colIdx);
    // java if (mergedRange != null && mergedRange.br.row > mergedRange.tl.row) {
    // java return -1.0F;
    // java } else {
    // java int colWidth = mergedRange == null ? layoutInfo.calColumnWidthPixelsIgnoreHidden(colIdx) : getRangeWidth(mergedRange, layoutInfo);
    // java float adjustedHeight = -1.0F;
    // java CellTextData data = getData(layoutInfo, rowIdx, colIdx, colWidth, 0);
    // java if (data.mIsValid) {
    // java int rowHeight = layoutInfo.rowHeightInPixels(rowIdx);
    // java int bestHeight = !data.mPara.mIsVerticalText ? (int)measureTypoHeight(layoutInfo.mGridSheet.coreSheet(), data, colWidth, rowHeight) : (int)measureTypoWidth(data, rowHeight, colWidth);
    // java if (bestHeight >= rowHeight || bestFit && bestHeight < rowHeight) {
    // java adjustedHeight = (float)bestHeight;
    // java }
    // java } else {
    // java float emptyHeight = measureEmptyTypoHeight(layoutInfo, font);
    // java int rowHeight = layoutInfo.rowHeightInPixels(rowIdx);
    // java if (emptyHeight >= (float)rowHeight || bestFit && emptyHeight < (float)rowHeight) {
    // java adjustedHeight = emptyHeight;
    // java }
    // java }
    // java
    // java data.clear();
    // java return adjustedHeight;
    // java }
    // java }
    pub fn getAdjustedCellHeight__font(
        l: &GridSheetLayoutInfo,
        row: i32,
        col: i32,
        font: Option<&FontInfo>,
        bestFit: bool,
    ) -> f32 {
        let merged = l.gridSheet().getMergedRange(row, col);
        if merged.as_ref().is_some_and(|r| r.br.row > r.tl.row) {
            return -1.0;
        }
        let width = merged
            .as_ref()
            .map(|r| Self::getRangeWidth(r, l))
            .unwrap_or_else(|| l.calColumnWidthPixelsIgnoreHidden(col));
        Self::adjustedHeight(l, row, col, font, bestFit, width)
    }
    // java public static float getAdjustedCellHeightByCoreWidth(GridSheetLayoutInfo layoutInfo, int rowIdx, int colIdx, boolean bestFit) {
    // java KmoSheet sheet = layoutInfo.mGridSheet.coreSheet();
    // java FontInfo font = EtFontOp.INSTANCE.getFont(sheet, rowIdx, colIdx);
    // java int coreWidth = layoutInfo.mGridSheet.coreSheet().getColWidth(colIdx);
    // java coreWidth = UnitsConverter.columnUnitsToPixels((float)coreWidth, layoutInfo.defaultCharWidthInPixels());
    // java return getAdjustedCellHeightByCoreWidth(layoutInfo, rowIdx, colIdx, font, bestFit, coreWidth);
    // java }
    pub fn getAdjustedCellHeightByCoreWidth(
        l: &GridSheetLayoutInfo,
        row: i32,
        col: i32,
        bestFit: bool,
    ) -> f32 {
        let core = l.gridSheet().coreSheet();
        let sheet = core.borrow();
        let font = ET_FONT_OP.getFont(&sheet, row, col);
        let coreWidth = UnitsConverter::columnUnitsToPixels(
            sheet.getColWidth(col) as f32,
            l.defaultCharWidthInPixels(),
        );
        drop(sheet);
        Self::getAdjustedCellHeightByCoreWidth__font(l, row, col, font.as_ref(), bestFit, coreWidth)
    }
    // java public static float getAdjustedCellHeightByCoreWidth(GridSheetLayoutInfo layoutInfo, int rowIdx, int colIdx, FontInfo font, boolean bestFit, int coreColWidth) {
    // java KmoRect mergedRange = layoutInfo.mGridSheet.getMergedRange(rowIdx, colIdx);
    // java if (mergedRange != null && mergedRange.br.row > mergedRange.tl.row) {
    // java return -1.0F;
    // java } else {
    // java int colWidth = mergedRange == null ? coreColWidth : getRangeCoreWidth(mergedRange, layoutInfo);
    // java float adjustedHeight = -1.0F;
    // java CellTextData data = getData(layoutInfo, rowIdx, colIdx, colWidth, 0);
    // java if (data.mIsValid) {
    // java int rowHeight = layoutInfo.rowHeightInPixels(rowIdx);
    // java int bestHeight = !data.mPara.mIsVerticalText ? (int)measureTypoHeight(layoutInfo.mGridSheet.coreSheet(), data, colWidth, rowHeight) : (int)measureTypoWidth(data, rowHeight, colWidth);
    // java if (bestHeight >= rowHeight || bestFit && bestHeight < rowHeight) {
    // java adjustedHeight = (float)bestHeight;
    // java }
    // java } else {
    // java float emptyHeight = measureEmptyTypoHeight(layoutInfo, font);
    // java int rowHeight = layoutInfo.rowHeightInPixels(rowIdx);
    // java if (emptyHeight >= (float)rowHeight || bestFit && emptyHeight < (float)rowHeight) {
    // java adjustedHeight = emptyHeight;
    // java }
    // java }
    // java
    // java data.clear();
    // java return adjustedHeight;
    // java }
    // java }
    pub fn getAdjustedCellHeightByCoreWidth__font(
        l: &GridSheetLayoutInfo,
        row: i32,
        col: i32,
        font: Option<&FontInfo>,
        bestFit: bool,
        coreWidth: i32,
    ) -> f32 {
        let merged = l.gridSheet().getMergedRange(row, col);
        if merged.as_ref().is_some_and(|r| r.br.row > r.tl.row) {
            return -1.0;
        }
        let width = merged
            .as_ref()
            .map(|r| Self::getRangeCoreWidth(r, l))
            .unwrap_or(coreWidth);
        Self::adjustedHeight(l, row, col, font, bestFit, width)
    }
    // rust: Shared native body of the two identical original adjusted-height branches, after each overload computes its own width.
    fn adjustedHeight(
        l: &GridSheetLayoutInfo,
        row: i32,
        col: i32,
        font: Option<&FontInfo>,
        bestFit: bool,
        width: i32,
    ) -> f32 {
        let mut adjusted = -1.0;
        let mut data = Self::getData(l, row, col, width, 0);
        let rowHeight = l.rowHeightInPixels(row);
        if data.mIsValid {
            let best = if !data.mPara.mIsVerticalText {
                Self::measureTypoHeight(&data, width, rowHeight) as i32
            } else {
                Self::measureTypoWidth(&data, rowHeight, width) as i32
            };
            if best >= rowHeight || bestFit && best < rowHeight {
                adjusted = best as f32;
            }
        } else {
            let empty = Self::measureEmptyTypoHeight(l, font.expect("NullPointerException font"));
            if empty >= rowHeight as f32 || bestFit && empty < (rowHeight as f32) {
                adjusted = empty;
            }
        }
        data.clear();
        adjusted
    }

    pub const STD_CHARS: &'static str = "0";
    pub const CELL_TEXT_MARGIN: i16 = 8;
    // java private static CellTextData getData(GridSheetLayoutInfo layoutInfo, int rowIdx, int colIdx, int width, int height) {
    // java if (width <= 0) {
    // java width = Integer.MAX_VALUE;
    // java }
    // java
    // java if (height <= 0) {
    // java height = Integer.MAX_VALUE;
    // java }
    // java
    // java boolean hasCFRule = !layoutInfo.mGridSheet.coreSheet().getCFormatter().isEmpty();
    // java sTextData.updateData(layoutInfo, width, height, rowIdx, colIdx, hasCFRule, (CellXFCache)null);
    // java return sTextData;
    // java }
    fn getData(l: &GridSheetLayoutInfo, row: i32, col: i32, w: i32, h: i32) -> CellTextData {
        let w = if w <= 0 { i32::MAX } else { w };
        let h = if h <= 0 { i32::MAX } else { h };
        let core = l.gridSheet().coreSheet();
        let hasCF = !core.borrow().getCFormatter().isEmpty();
        let mut data = CellTextData::default();
        data.updateData(l, w, h, row, col, hasCF, None);
        data
    }
    // java private static float measureSLineWidth(CellTextData data) {
    // java if (data.mCellType != 1) {
    // java return (float)Math.ceil((double)PaintFontHelper.measureCharWidth(data, 0, data.mSourceText.length()));
    // java } else {
    // java Matcher m = numericCharPattern.matcher(data.mSourceText);
    // java buffer.setLength(0);
    // java
    // java while(m.find()) {
    // java m.appendReplacement(buffer, "0");
    // java }
    // java
    // java m.appendTail(buffer);
    // java return (float)Math.ceil((double)PaintFontHelper.getCharWidth(buffer.toString(), data.mFirstSpan));
    // java }
    // java }
    fn measureSLineWidth(d: &CellTextData) -> f32 {
        if d.mCellType != 1 {
            PaintFontHelper::measureCharWidth(d, 0, d.source().len()).ceil()
        } else {
            let chars = ".,:/'1fijklrtvxy ".encode_utf16().collect::<Vec<_>>();
            let text = d
                .source()
                .iter()
                .map(|c| if chars.contains(c) { 48 } else { *c })
                .collect::<Vec<_>>();
            PaintFontHelper::getCharWidth__span(&text, &d.mFirstSpan).ceil()
        }
    }
    // java private static float measureTypoWidth(CellTextData data, int cellWidth, int cellHeight) {
    // java float typoWidth = 0.0F;
    // java float typoHeight = 0.0F;
    // java if (data.mCellType != 2) {
    // java typoWidth = measureSLineWidth(data) + 8.0F;
    // java typoWidth += (float)data.mPara.mIndent;
    // java if (data.isAngledText()) {
    // java Paint.FontMetricsInt metricsInt = sMetricsInt;
    // java PaintFontHelper.getFontMetricsInt(metricsInt, data.mFirstSpan);
    // java typoHeight = (float)(metricsInt.bottom - metricsInt.top);
    // java }
    // java } else if (!data.mPara.mIsWrapText) {
    // java BoringLayout.Metrics metrics = sBoringMetrics;
    // java PaintFontHelper.measureBoringMetrics(data.mSourceText, metrics, data.mFirstSpan, data.mOtherSpans);
    // java typoWidth = (float)(metrics.width + data.mPara.mIndent);
    // java typoHeight = (float)(metrics.bottom - metrics.top);
    // java } else {
    // java if (sTextPaint == null) {
    // java sTextPaint = new TextPaint();
    // java }
    // java
    // java Rect rect = new Rect(0, 0, cellWidth, 0);
    // java if (data.isAngledText()) {
    // java return measureAngleTextWidth(data, cellWidth, cellHeight);
    // java }
    // java
    // java Layout layout = typoLayoutGen.getLayout(rect, sTextPaint, data, true);
    // java typoWidth = getRealLayoutWidth(layout, data) + (float)data.mPara.mIndent;
    // java typoHeight = layout.getHeight();
    // java }
    // java
    // java if (data.isAngledText()) {
    // java if (data.mPara.mAngle != 90 && data.mPara.mAngle != -90) {
    // java float radians = (float)Math.toRadians((double)Math.abs(data.mPara.mAngle));
    // java float cos = (float)Math.cos((double)radians);
    // java typoWidth = (typoWidth + typoHeight * (float)Math.tan((double)radians)) * cos;
    // java } else {
    // java typoWidth = typoHeight;
    // java }
    // java }
    // java
    // java return typoWidth;
    // java }
    pub fn measureTypoWidth(d: &CellTextData, w: i32, h: i32) -> f32 {
        let (mut tw, mut th) = (0.0f32, 0.0f32);
        if d.mCellType != 2 {
            tw = Self::measureSLineWidth(d) + 8.0;
            tw += d.mPara.mIndent as f32;
            if d.isAngledText() {
                let fm = PaintFontHelper::getFontMetricsInt(&d.mFirstSpan);
                th = (fm.bottom - fm.top) as f32;
            }
        } else if !d.mPara.mIsWrapText {
            let fm =
                PaintFontHelper::measureBoringMetrics(d.source(), &d.mFirstSpan, &d.mOtherSpans);
            tw = (fm.width + d.mPara.mIndent as i32) as f32;
            th = (fm.bottom - fm.top) as f32;
        } else {
            if d.isAngledText() {
                return Self::measureAngleTextWidth(d, w, h);
            }
            let mut paint = Paint::default();
            let l = TypoLayoutGen::getLayout(&Rect::new(0, 0, w, 0), &mut paint, d, true);
            tw = Self::getRealLayoutWidth(&l, d) + d.mPara.mIndent as f32;
            th = l.getHeight();
        }
        if d.isAngledText() {
            if d.mPara.mAngle != 90 && d.mPara.mAngle != -90 {
                let radians = ((d.mPara.mAngle as f64).abs() * 0.017453292519943295) as f32;
                let cos = (radians as f64).cos() as f32;
                tw = (tw + th * (radians as f64).tan() as f32) * cos;
            } else {
                tw = th;
            }
        }
        tw
    }
    // java private static float getRealLayoutWidth(Layout layout, TextData data) {
    // java int lineCount = layout.getLineCount();
    // java float maxLineWidth = 0.0F;
    // java int maxLine = 0;
    // java
    // java for(int i = 0; i < lineCount; ++i) {
    // java float width = layout.getLineWidth(i) + 8.0F;
    // java if (width > maxLineWidth) {
    // java maxLineWidth = width;
    // java maxLine = i;
    // java }
    // java }
    // java
    // java if (!data.mIsRichText && data.mFirstSpan.mIsItalic) {
    // java float lineHeight = layout.getLineDescent(maxLine) - layout.getLineAscent(maxLine);
    // java maxLineWidth += lineHeight * 0.1F;
    // java }
    // java
    // java int layoutWidth = layout.getWidth();
    // java if (Math.abs(layoutWidth - (int)(maxLineWidth + 0.5F)) <= 8) {
    // java maxLineWidth = (float)layoutWidth;
    // java }
    // java
    // java return maxLineWidth;
    // java }
    fn getRealLayoutWidth(l: &dyn Layout, d: &TextData) -> f32 {
        let (mut max, mut line) = (0.0f32, 0);
        for i in 0..l.getLineCount() {
            let width = l.getLineWidth(i) + 8.0;
            if width > max {
                max = width;
                line = i;
            }
        }
        if !d.mIsRichText && d.mFirstSpan.mIsItalic {
            let height = l.getLineDescent(line) - (l.getLineTop(line) - l.getLineBaseline(line));
            max += height * 0.1;
        }
        let width = l.getWidth();
        if (width - (max + 0.5) as i32).abs() <= 8 {
            max = width as f32;
        }
        max
    }
    // java private static float measureTypoHeight(KmoSheet sheet, CellTextData data, int cellWidth, int cellHeight) {
    // java float typoHeight = 0.0F;
    // java float typoWidth = 0.0F;
    // java if (data.mCellType != 2) {
    // java Paint.FontMetricsInt metricsInt = sMetricsInt;
    // java PaintFontHelper.getFontMetricsInt(metricsInt, data.mFirstSpan);
    // java typoHeight = (float)(metricsInt.bottom - metricsInt.top);
    // java if (data.isAngledText()) {
    // java typoWidth = measureSLineWidth(data);
    // java }
    // java
    // java if (data.mFirstSpan.mIsSubScript || data.mFirstSpan.mIsSuperScript) {
    // java typoHeight += typoHeight * 0.1F;
    // java }
    // java } else {
    // java if (sTextPaint == null) {
    // java sTextPaint = new TextPaint();
    // java }
    // java
    // java Rect rect = new Rect(0, 0, cellWidth, 0);
    // java rect.right -= data.mPara.mIndent;
    // java if (rect.right < rect.left) {
    // java rect.right = rect.left + 1;
    // java }
    // java
    // java Layout layout = typoLayoutGen.getLayout(rect, sTextPaint, data, true);
    // java if (!data.isAngledText()) {
    // java typoHeight = layout.getHeight();
    // java typoWidth = getRealLayoutWidth(layout, data);
    // java } else {
    // java typoHeight = (float)layout.getAngleRectHeight();
    // java typoWidth = (float)layout.getAngleRectWidth();
    // java }
    // java }
    // java
    // java if (data.isAngledText()) {
    // java if (data.mPara.mAngle != 90 && data.mPara.mAngle != -90) {
    // java float radians = (float)Math.toRadians((double)Math.abs(data.mPara.mAngle));
    // java float sin = (float)Math.sin((double)radians);
    // java typoHeight = (typoWidth + typoHeight / (float)Math.tan((double)radians)) * sin;
    // java } else {
    // java typoHeight = typoWidth;
    // java }
    // java }
    // java
    // java return typoHeight;
    // java }
    pub fn measureTypoHeight(d: &CellTextData, w: i32, _h: i32) -> f32 {
        let (mut th, mut tw) = (0.0f32, 0.0f32);
        if d.mCellType != 2 {
            let fm = PaintFontHelper::getFontMetricsInt(&d.mFirstSpan);
            th = (fm.bottom - fm.top) as f32;
            if d.isAngledText() {
                tw = Self::measureSLineWidth(d);
            }
            if d.mFirstSpan.mIsSubScript || d.mFirstSpan.mIsSuperScript {
                th += th * 0.1;
            }
        } else {
            let mut rect = Rect::new(0, 0, w, 0);
            rect.right -= d.mPara.mIndent as i32;
            if rect.right < rect.left {
                rect.right = rect.left + 1;
            }
            let mut p = Paint::default();
            let l = TypoLayoutGen::getLayout(&rect, &mut p, d, true);
            if !d.isAngledText() {
                th = l.getHeight();
                tw = Self::getRealLayoutWidth(&l, d);
            } else {
                th = l.mAngleRectHeight as f32;
                tw = l.mAngleRectWidth as f32;
            }
        }
        if d.isAngledText() {
            if d.mPara.mAngle != 90 && d.mPara.mAngle != -90 {
                let r = ((d.mPara.mAngle as f64).abs() * 0.017453292519943295) as f32;
                let sin = (r as f64).sin() as f32;
                th = (tw + th / (r as f64).tan() as f32) * sin;
            } else {
                th = tw;
            }
        }
        th
    }
    // java private static float measureAngleTextWidth(CellTextData data, int cellWidth, int cellHeight) {
    // java float typoWidth = 0.0F;
    // java BoringLayout.Metrics metrics = sBoringMetrics;
    // java PaintFontHelper.measureBoringMetrics(data.mSourceText, metrics, data.mFirstSpan, data.mOtherSpans);
    // java float preLineHeight = (float)(metrics.bottom - metrics.top);
    // java float sumTextWidth = (float)metrics.width;
    // java float radians = (float)Math.toRadians((double)Math.abs(data.mPara.mAngle));
    // java float sina = (float)Math.sin((double)radians);
    // java float perLineWidth = (float)cellHeight / sina;
    // java int lineCount = (int)(sumTextWidth / perLineWidth) + 1;
    // java float perlineTypoWidth = preLineHeight / sina;
    // java if (sumTextWidth < perLineWidth) {
    // java perLineWidth = sumTextWidth;
    // java }
    // java
    // java typoWidth = (float)((double)(perlineTypoWidth * (float)(lineCount - 1)) + (double)(perLineWidth + preLineHeight * (float)Math.tan((double)radians)) * Math.cos((double)radians));
    // java return typoWidth;
    // java }
    fn measureAngleTextWidth(d: &CellTextData, _w: i32, h: i32) -> f32 {
        let fm = PaintFontHelper::measureBoringMetrics(d.source(), &d.mFirstSpan, &d.mOtherSpans);
        let pre = (fm.bottom - fm.top) as f32;
        let sum = fm.width as f32;
        let r = ((d.mPara.mAngle as f64).abs() * 0.017453292519943295) as f32;
        let sin = (r as f64).sin() as f32;
        let mut per = h as f32 / sin;
        let count = ((sum / per) as i32).wrapping_add(1);
        let typow = pre / sin;
        if sum < per {
            per = sum;
        }
        ((typow * count.wrapping_sub(1) as f32) as f64
            + ((per + pre * (r as f64).tan() as f32) as f64) * (r as f64).cos()) as f32
    }
    // java public static float getAdjustedCellWidth(GridSheetLayoutInfo layoutInfo, int rowIdx, int colIdx, boolean bestFit) {
    // java NumFmtResultCache.remove(layoutInfo.mGridSheet.coreSheet().getSheetUid(), rowIdx, colIdx);
    // java KmoRect mergedRange = layoutInfo.mGridSheet.getMergedRange(rowIdx, colIdx);
    // java if (mergedRange != null && mergedRange.br.col > mergedRange.tl.col) {
    // java return -1.0F;
    // java } else {
    // java int rowHeight = mergedRange == null ? layoutInfo.rowHeightInPixels(rowIdx) : getRangeHeight(mergedRange, layoutInfo);
    // java CellTextData data = getData(layoutInfo, rowIdx, colIdx, 0, rowHeight);
    // java float adjustedWidth = -1.0F;
    // java if (data.mIsValid && !data.mPara.isCenterCross()) {
    // java if (data.mCellType == 1 && data.mSourceText.equals(NumFmtUtil.invalidNumberSignString)) {
    // java return -1.0F;
    // java }
    // java
    // java int colWidth = layoutInfo.columnWidthInPixels(colIdx);
    // java int bestWidth = !data.mPara.mIsVerticalText ? (int)measureTypoWidth(data, colWidth, rowHeight) : (int)measureTypoHeight(layoutInfo.mGridSheet.coreSheet(), data, rowHeight, colWidth);
    // java if (bestWidth >= colWidth || bestFit && bestWidth < colWidth) {
    // java adjustedWidth = (float)bestWidth;
    // java }
    // java }
    // java
    // java data.clear();
    // java return adjustedWidth;
    // java }
    // java }
    pub fn getAdjustedCellWidth(l: &GridSheetLayoutInfo, row: i32, col: i32, bestFit: bool) -> f32 {
        let core = l.gridSheet().coreSheet();
        NumFmtResultCache::remove(core.borrow().getSheetUid(), row, col);
        let merged = core.borrow().getMergedRange(row, col);
        if merged.as_ref().is_some_and(|m| m.br.col > m.tl.col) {
            return -1.0;
        }
        let height = merged
            .as_ref()
            .map(|m| Self::getRangeHeight(m, l))
            .unwrap_or_else(|| l.rowHeightInPixels(row));
        let mut d = Self::getData(l, row, col, 0, height);
        let mut adjusted = -1.0;
        if d.mIsValid && !d.mPara.isCenterCross() {
            if d.mCellType == 1
                && String::from_utf16_lossy(d.source())
                    == kfc::cn::wps::kfc::numfmt::util::NumFmtUtil::invalidNumberSignString
            {
                return -1.0;
            }
            let width = l.columnWidthInPixels(col);
            let best = if !d.mPara.mIsVerticalText {
                Self::measureTypoWidth(&d, width, height) as i32
            } else {
                Self::measureTypoHeight(&d, height, width) as i32
            };
            if best >= width || bestFit && best < width {
                adjusted = best as f32;
            }
        }
        d.clear();
        adjusted
    }
    // java private static int getRangeWidth(KmoRect kmoRect, GridSheetLayoutInfo layoutInfo) {
    // java int sumWidth = 0;
    // java
    // java for(int i = kmoRect.tl.col; i <= kmoRect.br.col; ++i) {
    // java sumWidth += layoutInfo.columnWidthInPixels(i);
    // java }
    // java
    // java return sumWidth;
    // java }
    pub fn getRangeWidth(r: &KmoRect, l: &GridSheetLayoutInfo) -> i32 {
        (r.tl.col..=r.br.col)
            .map(|c| l.columnWidthInPixels(c))
            .sum()
    }
    // java private static int getRangeHeight(KmoRect kmoRect, GridSheetLayoutInfo layoutInfo) {
    // java int sumHeight = 0;
    // java
    // java for(int i = kmoRect.tl.row; i <= kmoRect.br.row; ++i) {
    // java sumHeight += layoutInfo.rowHeightInPixels(i);
    // java }
    // java
    // java return sumHeight;
    // java }
    pub fn getRangeHeight(r: &KmoRect, l: &GridSheetLayoutInfo) -> i32 {
        (r.tl.row..=r.br.row).map(|i| l.rowHeightInPixels(i)).sum()
    }
    // java private static int getRangeCoreWidth(KmoRect kmoRect, GridSheetLayoutInfo layoutInfo) {
    // java int sumWidth = 0;
    // java
    // java for(int i = kmoRect.tl.col; i <= kmoRect.br.col; ++i) {
    // java sumWidth += layoutInfo.mGridSheet.coreSheet().getColWidth(i);
    // java }
    // java
    // java return UnitsConverter.columnUnitsToPixels((float)sumWidth, layoutInfo.defaultCharWidthInPixels());
    // java }
    pub fn getRangeCoreWidth(r: &KmoRect, l: &GridSheetLayoutInfo) -> i32 {
        {
            let sh = l.gridSheet().coreSheet();
            let sh = sh.borrow();
            let sum: i32 = (r.tl.col..=r.br.col).map(|c| sh.getColWidth(c)).sum();
            UnitsConverter::columnUnitsToPixels(sum as f32, l.defaultCharWidthInPixels())
        }
    }
    // java public static float measureEmptyTypoWidth(GridSheetLayoutInfo layoutInfo, int rowIdx, int colIdx) {
    // java float typoWidth = 0.0F;
    // java KmoXF xf = layoutInfo.mGridSheet.getCellXF(rowIdx, colIdx);
    // java KmoFont font = xf.getFont();
    // java float fontSize = layoutInfo.mUnitsConverter.TwipsToPixels((float)font.getFontHeight());
    // java String fontName = font.getFontName();
    // java typoWidth = PaintFontHelper.getCharWidth("01234567", fontName, fontSize, false, false);
    // java return typoWidth;
    // java }
    pub fn measureEmptyTypoWidth(l: &GridSheetLayoutInfo, row: i32, col: i32) -> f32 {
        let xf = l.gridSheet().getCellXF(row, col).expect("cell XF");
        let font = xf.getFont().expect("cell font");
        let size = l.mUnitsConverter.TwipsToPixels(font.getFontHeight() as f32);
        PaintFontHelper::getCharWidth__text(
            &"01234567".encode_utf16().collect::<Vec<_>>(),
            &font.getFontName(),
            size,
            false,
            false,
        )
    }
}
