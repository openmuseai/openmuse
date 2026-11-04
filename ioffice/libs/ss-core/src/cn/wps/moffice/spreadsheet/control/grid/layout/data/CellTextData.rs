use super::super::{
    super::core::GridSheetLayoutInfo::GridSheetLayoutInfo,
    cache::NumFmtResultCache::NumFmtResultCache, typo::PaintFontHelper::PaintFontHelper,
};
use super::TextData::TextData;
use crate::cn::wps::moss::app::{
    xf::KmoXF::KmoXF,
    KmoSheet::{CellTypeInfo, KmoSheet},
    UnitsConverter::UnitsConverter,
};
// java public class CellTextData extends TextData
pub struct CellTextData {
    pub base: TextData,
    pub mRowIdx: i32,
    pub mColIdx: i32,
    pub mCellType: i32,
    pub mIsMergedCell: bool,
    pub mCenterCrossWidth: i32,
    pub mIsValid: bool,
    mTypeInfo: CellTypeInfo,
    mCondfmtValid: bool,
}
impl std::ops::Deref for CellTextData {
    type Target = TextData;
    fn deref(&self) -> &TextData {
        &self.base
    }
}
impl std::ops::DerefMut for CellTextData {
    fn deref_mut(&mut self) -> &mut TextData {
        &mut self.base
    }
}
impl Default for CellTextData {
    fn default() -> Self {
        Self::new(true)
    }
}
impl CellTextData {
    // java public CellTextData(boolean condfmtValid) {
    // java this.mIsValid = false;
    // java this.mTypeInfo = new KmoSheet.CellTypeInfo();
    // java this.mCondfmtValid = condfmtValid;
    // java this.mOtherSpans = new ArrayList(2);
    // java }
    pub fn new(valid: bool) -> Self {
        Self {
            base: TextData::default(),
            mRowIdx: 0,
            mColIdx: 0,
            mCellType: 0,
            mIsMergedCell: false,
            mCenterCrossWidth: 0,
            mIsValid: false,
            mTypeInfo: CellTypeInfo::CellTypeInfo(),
            mCondfmtValid: valid,
        }
    }
    // java public boolean updateCellType(KmoSheet sheet, int rowIdx, int colIdx) {
    // java this.mRowIdx = rowIdx;
    // java this.mColIdx = colIdx;
    // java this.mIsValid = true;
    // java this.mCellType = sheet.getCellValueType(rowIdx, colIdx, this.mTypeInfo);
    // java this.mIsRichText = this.mTypeInfo.hasRun;
    // java if (this.mCellType == 0) {
    // java this.mIsValid = false;
    // java }
    // java
    // java return this.mIsValid;
    // java }
    pub fn updateCellType(&mut self, s: &KmoSheet, row: i32, col: i32) -> bool {
        self.mRowIdx = row;
        self.mColIdx = col;
        self.mIsValid = true;
        self.mCellType = s.getCellValueType(row, col, Some(&mut self.mTypeInfo));
        self.mIsRichText = self.mTypeInfo.hasRun;
        if self.mCellType == 0 {
            self.mIsValid = false;
        }
        self.mIsValid
    }
    // java public void updateCellPara(KmoXF xf, UnitsConverter converter) {
    // java this.mPara.reset(xf, this.mCellType, converter);
    // java }
    pub fn updateCellPara(&mut self, xf: &KmoXF, c: &UnitsConverter) {
        let typ = self.mCellType;
        self.mPara.reset(xf, typ, c);
    }
    // java public boolean updateData(GridSheetLayoutInfo layoutInfo, int width, int height, int rowIdx, int colIdx, boolean hasCFRule, CellXFCache cellXfCache) {
    // java IGridSheet gridSheet = layoutInfo.mGridSheet;
    // java KmoSheet sheet = gridSheet.coreSheet();
    // java this.updateCellType(sheet, rowIdx, colIdx);
    // java if (this.mIsValid) {
    // java if (hasCFRule && !sheet.getCFormatter().isShowValue(rowIdx, colIdx)) {
    // java this.mIsValid = false;
    // java } else {
    // java KmoXF xf = cellXfCache != null ? cellXfCache.get(gridSheet, rowIdx, colIdx) : sheet.getCellXF(rowIdx, colIdx);
    // java if (null == xf) {
    // java this.mIsValid = false;
    // java return this.mIsValid;
    // java }
    // java
    // java this.updateCellPara(xf, layoutInfo.mUnitsConverter);
    // java this.mIsValid = this.updateCellSpan(layoutInfo, xf, width, height);
    // java }
    // java }
    // java
    // java if (this.mIsValid) {
    // java this.insertThaiPlaceHolder();
    // java }
    // java
    // java return this.mIsValid;
    // java }
    pub fn updateData(
        &mut self,
        l: &GridSheetLayoutInfo,
        width: i32,
        height: i32,
        row: i32,
        col: i32,
        hasCF: bool,
        cellXfCache: Option<&super::super::cache::CellXFCache::CellXFCache>,
    ) -> bool {
        let core = l.gridSheet().coreSheet();
        let sheet = core.borrow();
        self.updateCellType(&sheet, row, col);
        if self.mIsValid {
            if hasCF && !sheet.getCFormatter().isShowValue(row, col) {
                self.mIsValid = false;
            } else {
                let Some(xf) = (if let Some(cache) = cellXfCache {
                    cache.get(l.gridSheet().as_ref(), row, col)
                } else {
                    sheet.getCellXF__2(row, col)
                }) else {
                    self.mIsValid = false;
                    return false;
                };
                self.updateCellPara(&xf, &l.mUnitsConverter);
                self.mIsValid = self.updateCellSpan(l, &xf, width, height);
            }
        }
        if self.mIsValid {
            self.insertThaiPlaceHolder();
        }
        self.mIsValid
    }
    // java public boolean updateCellSpan(GridSheetLayoutInfo layoutInfo, KmoXF xf, int width, int height) {
    // java KmoSheet kmoSheet = layoutInfo.mGridSheet.coreSheet();
    // java KmoTableManager tableManager = kmoSheet.getTableManager();
    // java KmoXF newXF = xf;
    // java if (null != tableManager) {
    // java newXF = tableManager.getMergeXF(this.mRowIdx, this.mColIdx, xf, (byte)2);
    // java }
    // java
    // java KmoFont mergedFont = this.mCondfmtValid ? kmoSheet.getCFormatter().getMergeXF(this.mRowIdx, this.mColIdx, newXF, (byte)2).getFont() : newXF.getFont();
    // java if (mergedFont == null) {
    // java return false;
    // java } else {
    // java KmoPalette palette = kmoSheet.getBook().getPalette();
    // java UnitsConverter converter = layoutInfo.mUnitsConverter;
    // java this.mIsMergedCell = width > layoutInfo.columnWidthInPixels(this.mColIdx) || height > layoutInfo.rowHeightInPixels(this.mRowIdx);
    // java this.mCenterCrossWidth = this.calcCenterCrossWidth(layoutInfo, this.mRowIdx, this.mColIdx);
    // java this.mFirstSpan.reset(converter, palette, mergedFont, 0, 0);
    // java NumFmtResultCache.FmtStrResult result = NumFmtResultCache.getFmtResult(kmoSheet, this.mRowIdx, this.mColIdx);
    // java if (result == null) {
    // java int charCount = 256;
    // java int realWidth = this.mPara.mAngle != 0 && Math.abs(this.mPara.mAngle) == 90 ? height : width + this.mCenterCrossWidth;
    // java if (!this.mPara.mIsShrinkToFit) {
    // java charCount = (int)((float)realWidth / PaintFontHelper.getCharWidth("0", this.mFirstSpan) + 0.5F);
    // java }
    // java
    // java NumFmtResult formatResult = kmoSheet.getNumFormatResult(this.mRowIdx, this.mColIdx, charCount);
    // java if (formatResult.mFillPos > -1) {
    // java this.fillFmtResult(realWidth, formatResult, charCount);
    // java }
    // java
    // java result = NumFmtResultCache.put(formatResult, this.mRowIdx, this.mColIdx);
    // java }
    // java
    // java KmoCellData cellData = SharedObjects.kmoCellDataPool.borrowObject();
    // java kmoSheet.getCellData(this.mRowIdx, this.mColIdx, cellData);
    // java if (cellData.isPicInCell) {
    // java KmoCellPic kmoCellPic = kmoSheet.getCellPic(this.mRowIdx, this.mColIdx);
    // java if (kmoCellPic != null && !kmoCellPic.isValid()) {
    // java result.mNumFmtStr = kmoCellPic.getValue();
    // java this.mCellType = 6;
    // java }
    // java }
    // java
    // java SharedObjects.kmoCellDataPool.returnObject(cellData);
    // java this.mSourceText = result.mNumFmtStr;
    // java if (this.mSourceText != null && this.mSourceText.length() != 0) {
    // java if (this.mCellType == 2) {
    // java boolean isRightToLeft = TypoLayoutGen.isRightToLeftTypo(this.mSourceText);
    // java this.mPara.setRightToLeft(isRightToLeft);
    // java if (isRightToLeft && KmoLang.systemIsLang((short)1025)) {
    // java this.mSourceText = ArabicTypoUtil.replaceArabicDigit(this.mSourceText);
    // java }
    // java } else if (this.mCellType == 1) {
    // java this.mPara.modifyHAlignment(result.mNumericAsText);
    // java }
    // java
    // java this.mFirstSpan.mEndPos = this.mSourceText.length();
    // java if (this.mIsRichText && (result.mRichRunInvalid || this.mPara.isAlignFill())) {
    // java this.mIsRichText = true;
    // java }
    // java
    // java boolean isSymbolFont = SymbolConverter.isSymbolFont(this.mFirstSpan.mFontName);
    // java if (this.mIsRichText) {
    // java DataAllocator allocator = DataAllocator.getInstance();
    // java UnicodeString unString = layoutInfo.mGridSheet.getRunsString(this.mRowIdx, this.mColIdx);
    // java if (unString != null) {
    // java List<FormatRun> runs = unString.getFormatRuns();
    // java SpanInfo preSpan = this.mFirstSpan;
    // java KmoXFManager xfManager = kmoSheet.getBook().getXFManager();
    // java int prePos = 0;
    // java int sourceTextLength = this.mSourceText.length();
    // java int i = 0;
    // java
    // java for(int size = runs.size(); i < size; ++i) {
    // java FormatRun run = (FormatRun)runs.get(i);
    // java short pos = run.getCharacterPos();
    // java if (prePos > pos || pos > sourceTextLength) {
    // java for(int j = this.mOtherSpans.size() - 1; j > -1; --j) {
    // java allocator.disposeSpan((SpanInfo)this.mOtherSpans.get(j));
    // java }
    // java
    // java this.mOtherSpans.clear();
    // java this.mFirstSpan.mEndPos = sourceTextLength;
    // java break;
    // java }
    // java
    // java preSpan.mEndPos = pos;
    // java prePos = pos;
    // java SpanInfo span = allocator.allocSpan();
    // java span.reset(converter, palette, xfManager.getFont(run.getFontIndex()), preSpan.mEndPos, preSpan.mEndPos);
    // java preSpan = span;
    // java this.mOtherSpans.add(span);
    // java if (!isSymbolFont) {
    // java isSymbolFont = SymbolConverter.isSymbolFont(span.mFontName);
    // java }
    // java }
    // java
    // java preSpan.mEndPos = sourceTextLength;
    // java }
    // java }
    // java
    // java if (result.mNumFmtColorIdx != null && xf.getFont().getColorPaletteIndex() == mergedFont.getColorPaletteIndex()) {
    // java int formatColor = palette.getFormatColor(result.mNumFmtColorIdx);
    // java this.mFirstSpan.mFontColor = formatColor;
    // java
    // java for(int i = this.mOtherSpans.size() - 1; i > -1; --i) {
    // java ((SpanInfo)this.mOtherSpans.get(i)).mFontColor = formatColor;
    // java }
    // java }
    // java
    // java this.mSourceText = this.modifySpecialText(this.mSourceText, !this.mPara.mIsWrapText);
    // java if (isSymbolFont) {
    // java this.mSourceText = this.convertSymbolText(this.mSourceText);
    // java }
    // java
    // java return true;
    // java } else {
    // java this.mIsValid = false;
    // java return this.mIsValid;
    // java }
    // java }
    // java }
    pub fn updateCellSpan(
        &mut self,
        l: &GridSheetLayoutInfo,
        xf: &crate::java_runtime::Rc<KmoXF>,
        width: i32,
        height: i32,
    ) -> bool {
        let core = l.gridSheet().coreSheet();
        let sheet = core.borrow();
        let merged = if self.mCondfmtValid {
            sheet
                .getCFormatter()
                .getMergeXF(self.mRowIdx, self.mColIdx, xf.clone(), 2)
        } else {
            xf.clone()
        };
        let Some(font) = merged.getFont() else {
            return false;
        };
        self.mIsMergedCell = width > l.columnWidthInPixels(self.mColIdx)
            || height > l.rowHeightInPixels(self.mRowIdx);
        self.mCenterCrossWidth = self.calcCenterCrossWidth(l, self.mRowIdx, self.mColIdx);
        self.mFirstSpan.reset(
            &l.mUnitsConverter,
            &sheet.getBook().getPalette(),
            &font,
            0,
            0,
        );
        let result = if let Some(r) =
            NumFmtResultCache::getFmtResult(&sheet, self.mRowIdx, self.mColIdx)
        {
            r
        } else {
            let mut chars = 256;
            let real = if self.mPara.mAngle != 0 && self.mPara.mAngle.abs() == 90 {
                height
            } else {
                width + self.mCenterCrossWidth
            };
            if !self.mPara.mIsShrinkToFit {
                chars = (real as f32 / PaintFontHelper::getCharWidth__span(&[48], &self.mFirstSpan)
                    + 0.5) as i32;
            }
            let r = sheet.getNumFormatResult(self.mRowIdx, self.mColIdx, chars);
            let mut r = r.borrow_mut();
            if r.mFillPos > -1 {
                self.fillFmtResult(real, &mut r, chars);
            }
            NumFmtResultCache::put(&r, self.mRowIdx, self.mColIdx)
        };
        let r = result.borrow();
        self.mSourceText = Some(r.mNumFmtStr.clone());
        if self.source().is_empty() {
            self.mIsValid = false;
            return false;
        }
        if self.mCellType == 2 {
            let rtl = super::super::layouter::BidiTools::BidiTools::needRtl(self.source());
            self.mPara.setRightToLeft(rtl);
            if rtl {
                panic!("Unported M3 Arabic/Bidi text normalization");
            }
        } else if self.mCellType == 1 {
            self.mPara.modifyHAlignment(r.mNumericAsText);
        }
        self.mFirstSpan.mEndPos = self.source().len() as i32;
        if self.mIsRichText {
            if let Some(unicode) = l.gridSheet().getRunsString(self.mRowIdx, self.mColIdx) {
                let book = sheet.getBook();
                let mgr = book.getXFManager().borrow();
                let palette = book.getPalette();
                let mut prePos = 0i32;
                let len = self.source().len() as i32;
                for run in unicode.getFormatRuns() {
                    let pos = run.getCharacterPos() as i32;
                    if prePos > pos || pos > len {
                        self.mOtherSpans.clear();
                        self.mFirstSpan.mEndPos = len;
                        break;
                    }
                    if let Some(pre) = self.mOtherSpans.last_mut() {
                        pre.mEndPos = pos;
                    } else {
                        self.mFirstSpan.mEndPos = pos;
                    }
                    prePos = pos;
                    let mut span = super::SpanInfo::SpanInfo::default();
                    span.reset(
                        &l.mUnitsConverter,
                        &palette,
                        &mgr.getFont(run.getFontIndex() as i32),
                        pos,
                        pos,
                    );
                    self.mOtherSpans.push(span);
                }
                if let Some(pre) = self.mOtherSpans.last_mut() {
                    pre.mEndPos = len;
                } else {
                    self.mFirstSpan.mEndPos = len;
                }
            }
        }
        if r.mNumFmtColorIdx.is_some()
            && xf.getFont().unwrap().getColorPaletteIndex() == font.getColorPaletteIndex()
        {
            self.mFirstSpan.mFontColor = sheet
                .getBook()
                .getPalette()
                .getFormatColor(r.mNumFmtColorIdx.unwrap());
            let color = self.mFirstSpan.mFontColor;
            for span in &mut self.mOtherSpans {
                span.mFontColor = color;
            }
        }
        let source = self.source().to_vec();
        let filtered = self.modifySpecialText(&source, !self.mPara.mIsWrapText);
        self.mSourceText = Some(filtered);
        true
    }
    // java private void fillFmtResult(int sumWidth, NumFmtResult result, int chars) {
    // java String source = result.numfmtStr();
    // java float width = PaintFontHelper.getCharWidth(source, this.mFirstSpan.mFontName, this.mFirstSpan.mFontSize, this.mFirstSpan.mIsBold, this.mFirstSpan.mIsItalic);
    // java float sub = (float)sumWidth - width - (float)this.mPara.mIndent;
    // java if (sub > 0.0F) {
    // java FILL_CHAR[0] = result.mFillChar;
    // java float charWidth = PaintFontHelper.getCharWidth(new String(FILL_CHAR), this.mFirstSpan.mFontName, this.mFirstSpan.mFontSize, this.mFirstSpan.mIsBold, this.mFirstSpan.mIsItalic);
    // java int charCount = (int)(sub / (charWidth > 1.0F ? charWidth : 1.0F));
    // java if (charCount > 0) {
    // java char[] fills = NumFmtUtil.fillChar(FILL_CHAR[0], charCount);
    // java result.mNumFmtStr.insert(result.mFillPos, fills);
    // java }
    // java } else if (sub < 0.0F) {
    // java result.mNumFmtStr.setLength(0);
    // java char[] fills = NumFmtUtil.fillChar('#', chars);
    // java result.mNumFmtStr.append(fills);
    // java }
    // java
    // java }
    fn fillFmtResult(
        &mut self,
        total: i32,
        r: &mut kfc::cn::wps::kfc::numfmt::format::NumFmtResult::NumFmtResult,
        chars: i32,
    ) {
        let source = r.numfmtStr().encode_utf16().collect::<Vec<_>>();
        let width = PaintFontHelper::getCharWidth__span(&source, &self.mFirstSpan);
        let sub = total as f32 - width - self.mPara.mIndent as f32;
        if sub > 0.0 {
            let cw = PaintFontHelper::getCharWidth__span(&[r.mFillChar as u16], &self.mFirstSpan);
            let count = (sub / if cw > 1.0 { cw } else { 1.0 }) as i32;
            if count > 0 {
                let mut units = r.mNumFmtStr.encode_utf16().collect::<Vec<_>>();
                units.splice(
                    r.mFillPos as usize..r.mFillPos as usize,
                    std::iter::repeat_n(r.mFillChar as u16, count as usize),
                );
                r.mNumFmtStr =
                    String::from_utf16(&units).expect("Unpaired UTF16 in NumFmtResult adapter");
            }
        } else if sub < 0.0 {
            r.mNumFmtStr.clear();
            r.mNumFmtStr.push_str(&"#".repeat(chars.max(0) as usize));
        }
    }
    // java private int calcCenterCrossWidth(GridSheetLayoutInfo layoutInfo, int row, int col) {
    // java if (!this.mIsMergedCell && this.mPara.mHAlignment == 6 && !this.mPara.mIsVerticalText && this.mPara.mAngle == 0) {
    // java int width = 0;
    // java KmoSheet sheet = layoutInfo.mGridSheet.coreSheet();
    // java
    // java for(int colIdx = col + 1; colIdx < layoutInfo.mMaxColumns; ++colIdx) {
    // java KmoXF xf = sheet.getCellXF(row, colIdx);
    // java if (!sheet.isBlankCellValue(row, colIdx) || xf.getAlignment() != 6) {
    // java break;
    // java }
    // java
    // java width += layoutInfo.columnWidthInPixels(colIdx);
    // java }
    // java
    // java return width;
    // java } else {
    // java return 0;
    // java }
    // java }
    pub fn calcCenterCrossWidth(&self, l: &GridSheetLayoutInfo, row: i32, col: i32) -> i32 {
        if !self.mIsMergedCell
            && self.mPara.mHAlignment == 6
            && !self.mPara.mIsVerticalText
            && self.mPara.mAngle == 0
        {
            let core = l.gridSheet().coreSheet();
            let s = core.borrow();
            let mut width = 0i32;
            for c in col + 1..l.mMaxColumns {
                let xf = s.getCellXF__2(row, c).unwrap();
                if !s.isBlankCellValue(row, c) || xf.getAlignment() != 6 {
                    break;
                }
                width = width.wrapping_add(l.columnWidthInPixels(c));
            }
            width
        } else {
            0
        }
    }
    // java public final boolean isVerticalText() {
    // java return false;
    // java }
    pub fn isVerticalText(&self) -> bool {
        false
    }
    // java public final boolean isAngledText() {
    // java return this.mPara.mAngle != 0;
    // java }
    pub fn isAngledText(&self) -> bool {
        self.mPara.mAngle != 0
    }
    // java public void clear() {
    // java this.mIsValid = false;
    // java this.mIsMergedCell = false;
    // java this.mRowIdx = -1;
    // java this.mColIdx = -1;
    // java super.clear();
    // java }
    pub fn clear(&mut self) {
        self.mIsValid = false;
        self.mIsMergedCell = false;
        self.mRowIdx = -1;
        self.mColIdx = -1;
        self.base.clear();
    }
    // rust: Fail explicitly until the corresponding language dependency is present.
    pub fn insertThaiPlaceHolder(&mut self) {
        if self.source().iter().any(|c| (0x0e00..=0x0e7f).contains(c)) {
            panic!("Unported M3 Thai correction");
        }
    }
    // java private boolean isJPFontType(String fontName) {
    // java int i = 0;
    // java
    // java int len;
    // java for(len = JP_FONTS.length; i < len && !JP_FONTS[i].equals(fontName); ++i) {
    // java }
    // java
    // java return i < len;
    // java }
    fn isJPFontType(font: &str) -> bool {
        [
            "ＭＳ 明朝",
            "ＭＳ Ｐ明朝",
            "ＭＳ ゴシック",
            "ＭＳ Ｐゴシック",
            "MS UI Gothic",
            "MS Mincho",
            "MS PMincho",
            "MS Gothic",
            "MS PGothic",
            "Meiryo UI",
        ]
        .contains(&font)
    }
    // java private String modifySpecialText(String sourceText, boolean filterReturn) {
    // java boolean hasReturn = sourceText.indexOf(10) != -1;
    // java boolean hasSpecial = sourceText.indexOf(59411) != -1 || sourceText.indexOf(8206) != -1 || sourceText.indexOf(8207) != -1 || UnprintableCodeUtil.hasUnprintableASCIIControlCode(sourceText);
    // java boolean hasJpFontText = sourceText.indexOf(92) != -1;
    // java if ((!hasReturn || !filterReturn) && !hasSpecial && !hasJpFontText) {
    // java return sourceText;
    // java } else {
    // java int spanIdx = -1;
    // java SpanInfo spanInfo = this.mFirstSpan;
    // java boolean isJpFont = hasJpFontText && this.isJPFontType(spanInfo.mFontName);
    // java StringBuffer buffer = new StringBuffer(sourceText);
    // java int charIdx = 0;
    // java
    // java for(int len = buffer.length(); charIdx < len; ++charIdx) {
    // java char c = buffer.charAt(charIdx);
    // java if (hasJpFontText && charIdx >= spanInfo.mEndPos) {
    // java ++spanIdx;
    // java spanInfo = (SpanInfo)this.mOtherSpans.get(spanIdx);
    // java isJpFont = this.isJPFontType(spanInfo.mFontName);
    // java }
    // java
    // java if ((!filterReturn || c != '\n') && c != 8206 && c != 8207 && !UnprintableCodeUtil.isUnprintableASCIIControlCode(c)) {
    // java if (c == '\ue813') {
    // java buffer.setCharAt(charIdx, '　');
    // java } else if (isJpFont && c == '\\') {
    // java buffer.setCharAt(charIdx, '¥');
    // java }
    // java } else {
    // java buffer.deleteCharAt(charIdx);
    // java if (charIdx < this.mFirstSpan.mEndPos) {
    // java --this.mFirstSpan.mEndPos;
    // java
    // java for(SpanInfo span : this.mOtherSpans) {
    // java --span.mStartPos;
    // java --span.mEndPos;
    // java }
    // java } else {
    // java for(int i = 0; i < this.mOtherSpans.size(); ++i) {
    // java SpanInfo span = (SpanInfo)this.mOtherSpans.get(i);
    // java if (charIdx < span.mEndPos) {
    // java --span.mEndPos;
    // java
    // java for(int j = i + 1; j < this.mOtherSpans.size(); ++j) {
    // java span = (SpanInfo)this.mOtherSpans.get(j);
    // java --span.mStartPos;
    // java --span.mEndPos;
    // java }
    // java break;
    // java }
    // java }
    // java }
    // java
    // java --len;
    // java --charIdx;
    // java }
    // java }
    // java
    // java return buffer.toString();
    // java }
    // java }
    fn modifySpecialText(&mut self, source: &[u16], filterReturn: bool) -> Vec<u16> {
        let hasReturn = source.contains(&10);
        let special = source
            .iter()
            .any(|c| [59411, 8206, 8207, 30, 31].contains(c));
        let hasJp = source.contains(&92);
        if (!hasReturn || !filterReturn) && !special && !hasJp {
            return source.to_vec();
        }
        let mut buffer = source.to_vec();
        let mut spanIndex = -1i32;
        let mut isJp = hasJp && Self::isJPFontType(&self.mFirstSpan.mFontName);
        let mut i = 0usize;
        while i < buffer.len() {
            let c = buffer[i];
            let span = if spanIndex < 0 {
                &self.mFirstSpan
            } else {
                &self.mOtherSpans[spanIndex as usize]
            };
            if hasJp && i as i32 >= span.mEndPos {
                spanIndex += 1;
                isJp = Self::isJPFontType(&self.mOtherSpans[spanIndex as usize].mFontName);
            }
            if (!filterReturn || c != 10) && c != 8206 && c != 8207 && c != 30 && c != 31 {
                if c == 59411 {
                    buffer[i] = 12288;
                } else if isJp && c == 92 {
                    buffer[i] = 165;
                }
                i += 1;
            } else {
                buffer.remove(i);
                if (i as i32) < self.mFirstSpan.mEndPos {
                    self.mFirstSpan.mEndPos -= 1;
                    for span in &mut self.mOtherSpans {
                        span.mStartPos -= 1;
                        span.mEndPos -= 1;
                    }
                } else {
                    for j in 0..self.mOtherSpans.len() {
                        if (i as i32) < self.mOtherSpans[j].mEndPos {
                            self.mOtherSpans[j].mEndPos -= 1;
                            for span in &mut self.mOtherSpans[j + 1..] {
                                span.mStartPos -= 1;
                                span.mEndPos -= 1;
                            }
                            break;
                        }
                    }
                }
            }
        }
        buffer
    }
}
