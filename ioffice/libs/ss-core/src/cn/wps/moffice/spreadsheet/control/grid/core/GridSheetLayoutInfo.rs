use super::super::{
    render::data::IRoGridClientData::IRoGridClientData, theme::NormalGridTheme::NormalGridTheme,
};
use super::{GridCache::GridCache, GridRowCache::GridRowCache};
use super::{GridCoreData::GridCoreData, IHeaderState::IHeaderState};
use crate::cn::wps::moss::app::{IGridSheet::IGridSheet, UnitsConverter::UnitsConverter};
use crate::java_runtime::Rc;
// java public class GridSheetLayoutInfo extends GridCoreData
pub struct GridSheetLayoutInfo {
    pub mGridTheme: NormalGridTheme,
    pub mHeaderState: Option<Rc<dyn IHeaderState>>,
    pub mRoGridClientData: Rc<dyn IRoGridClientData>,
    pub mGridSheet: Option<Rc<dyn IGridSheet>>,
    pub mUnitsConverter: UnitsConverter,
    pub mWidth: i32,
    pub mHeight: i32,
    pub mScrollX: i32,
    pub mScrollY: i32,
    pub mPreScrollX: i32,
    pub mPreScrollY: i32,
    pub mFreezeXOffset: i32,
    pub mFreezeYOffset: i32,
    pub mFreezeLeftX: i32,
    pub mFreezeTopY: i32,
    pub mFreezeOriginXOffset: i32,
    pub mFreezeOriginYOffset: i32,
    pub mFreezeOriginLeftX: i32,
    pub mFreezeOriginTopY: i32,
    pub mMaxRows: i32,
    pub mMaxColumns: i32,
    pub mMaxRowsLimit: i32,
    pub mMaxColumnsLimit: i32,
    pub mDefaultRowHeight: i32,
    pub mDefaultColWidth: i32,
    pub mDefaultRowHeightUnScale: i32,
    pub mDefaultColWidthUnScale: i32,
    pub mCacheDefaultCharWidth: f32,
    pub mCacheDefaultCharWidthUnscale: f32,
    pub mCacheRowHeight: Option<Vec<i32>>,
    pub mCacheColumnWidth: Option<Vec<i32>>,
    pub mGridRowCache: GridRowCache,
    pub mCacheMaxRowsIndex: i32,
    pub mCacheMinRowsIndex: i32,
    pub mCacheMaxColumnIndex: i32,
    pub mDefaultRowDyZero: bool,
    pub mBaseRowHeight: i32,
    pub mRowInfoIsEmpty: bool,
    pub mIgnoreHeadSize: bool,
    pub headerColWidth: i32,
    pub headerRowHeight: i32,
    pub gridXOffset: i32,
    pub gridYOffset: i32,
}
impl GridSheetLayoutInfo {
    // java public int calColumnWidthPixelsIgnoreHidden(int column) {
    // java if (this.mGridSheet != null && column < this.mMaxColumnsLimit) {
    // java if (!this.mGridSheet.isColHidden(column)) {
    // java return this.columnWidthInPixels(column);
    // java } else {
    // java int aColumnWidth = this.mGridSheet.columnWidth(column);
    // java if (aColumnWidth != 2048 && 0 != aColumnWidth) {
    // java aColumnWidth = UnitsConverter.columnUnitsToPixels((float)aColumnWidth, this.mCacheDefaultCharWidth);
    // java } else {
    // java aColumnWidth = this.mDefaultColWidth;
    // java }
    // java
    // java return aColumnWidth;
    // java }
    // java } else {
    // java return 0;
    // java }
    // java }
    pub fn calColumnWidthPixelsIgnoreHidden(&self, col: i32) -> i32 {
        if self.mGridSheet.is_some() && col < self.mMaxColumnsLimit {
            if !self.gridSheet().isColHidden(col) {
                self.columnWidthInPixels(col)
            } else {
                let width = self.gridSheet().columnWidth(col);
                if width != 2048 && width != 0 {
                    UnitsConverter::columnUnitsToPixels(width as f32, self.mCacheDefaultCharWidth)
                } else {
                    self.mDefaultColWidth
                }
            }
        } else {
            0
        }
    }

    // rust: Rust borrow adapter for the nullable Java field.
    pub fn gridSheet(&self) -> &Rc<dyn IGridSheet> {
        self.mGridSheet
            .as_ref()
            .expect("NullPointerException mGridSheet")
    }
    pub const DEFAULT_COLUMN_WIDTH: i32 = 2048;
    pub const DEFAULT_ROW_HEIGHT: i32 = 325;
    pub const INVALID_VALUE: i32 = -1;
    // java public GridSheetLayoutInfo(IGridSheet gridSheet, UnitsConverter unitsConverter, IHeaderState headerState, IRoGridClientData roGridClientData) {
    // java this.mUnitsConverter = unitsConverter;
    // java this.mGridTheme = new NormalGridTheme(gridSheet, unitsConverter);
    // java this.mHeaderState = headerState;
    // java this.mRoGridClientData = roGridClientData;
    // java }
    pub fn new(
        grid: Rc<dyn IGridSheet>,
        converter: UnitsConverter,
        headerState: Option<Rc<dyn IHeaderState>>,
        client: Rc<dyn IRoGridClientData>,
    ) -> Self {
        let theme = NormalGridTheme::new(grid.as_ref(), &converter);
        Self {
            mGridTheme: theme,
            mHeaderState: headerState,
            mRoGridClientData: client,
            mGridSheet: None,
            mUnitsConverter: converter,
            mWidth: -1,
            mHeight: -1,
            mScrollX: 0,
            mScrollY: 0,
            mPreScrollX: 0,
            mPreScrollY: 0,
            mFreezeXOffset: 0,
            mFreezeYOffset: 0,
            mFreezeLeftX: 0,
            mFreezeTopY: 0,
            mFreezeOriginXOffset: 0,
            mFreezeOriginYOffset: 0,
            mFreezeOriginLeftX: 0,
            mFreezeOriginTopY: 0,
            mMaxRows: 0,
            mMaxColumns: 0,
            mMaxRowsLimit: 0,
            mMaxColumnsLimit: 0,
            mDefaultRowHeight: -1,
            mDefaultColWidth: -1,
            mDefaultRowHeightUnScale: -1,
            mDefaultColWidthUnScale: -1,
            mCacheDefaultCharWidth: -1.0,
            mCacheDefaultCharWidthUnscale: -1.0,
            mCacheRowHeight: None,
            mCacheColumnWidth: None,
            mGridRowCache: GridRowCache::new(),
            mCacheMaxRowsIndex: 0,
            mCacheMinRowsIndex: 0,
            mCacheMaxColumnIndex: 0,
            mDefaultRowDyZero: false,
            mBaseRowHeight: 0,
            mRowInfoIsEmpty: false,
            mIgnoreHeadSize: false,
            headerColWidth: -1,
            headerRowHeight: -1,
            gridXOffset: 0,
            gridYOffset: 0,
        }
    }
    // java public void refreshData() {
    // java this.mGridTheme.resetTheme(this.mGridSheet, this.mUnitsConverter);
    // java this.recalculate();
    // java }
    pub fn refreshData(&mut self) {
        let grid = self.gridSheet().clone();
        self.mGridTheme
            .resetTheme(grid.as_ref(), &self.mUnitsConverter);
        self.recalculate();
    }
    // java public void reset(IGridSheet gridSheet) { this.resetSheet(gridSheet); this.refreshData(); }
    pub fn reset(&mut self, g: Rc<dyn IGridSheet>) {
        GridCoreData::reset(self, g);
    }
    // java public void resetSheet(IGridSheet gridSheet) {
    // java this.mGridSheet = gridSheet;
    // java }
    // java private void initializeFreezeScroll() {
    // java if (this.mGridSheet != null && this.mGridSheet.hasFrozen()) {
    // java int splitRows = this.mGridSheet.getPaneRowSplit();
    // java int splitCols = this.mGridSheet.getPaneColSplit();
    // java if (splitCols > 0) {
    // java int leftTopCol = this.mGridSheet.getLeftTopCol();
    // java this.mFreezeLeftX = this.columnX(leftTopCol);
    // java this.mFreezeOriginLeftX = this.mFreezeLeftX;
    // java this.mFreezeXOffset = this.columnX(splitCols + leftTopCol) - this.mFreezeLeftX;
    // java this.mFreezeOriginXOffset = this.mFreezeXOffset;
    // java }
    // java
    // java if (splitRows > 0) {
    // java int leftTopRow = this.mGridSheet.getLeftTopRow();
    // java this.mFreezeTopY = this.rowY(leftTopRow);
    // java this.mFreezeOriginTopY = this.mFreezeTopY;
    // java this.mFreezeYOffset = this.rowY(splitRows + leftTopRow) - this.mFreezeTopY;
    // java this.mFreezeOriginYOffset = this.mFreezeYOffset;
    // java }
    // java
    // java }
    // java }
    pub fn initializeFreezeScroll(&mut self) {
        let grid = self.gridSheet().clone();
        if grid.hasFrozen() {
            let rows = grid.getPaneRowSplit();
            let cols = grid.getPaneColSplit();
            if cols > 0 {
                let left = grid.getLeftTopCol();
                self.mFreezeLeftX = self.columnX(left);
                self.mFreezeOriginLeftX = self.mFreezeLeftX;
                self.mFreezeXOffset = self.columnX(cols + left) - self.mFreezeLeftX;
                self.mFreezeOriginXOffset = self.mFreezeXOffset;
            }
            if rows > 0 {
                let top = grid.getLeftTopRow();
                self.mFreezeTopY = self.rowY(top);
                self.mFreezeOriginTopY = self.mFreezeTopY;
                self.mFreezeYOffset = self.rowY(rows + top) - self.mFreezeTopY;
                self.mFreezeOriginYOffset = self.mFreezeYOffset;
            }
        }
    }
    pub fn resetSheet(&mut self, grid: Rc<dyn IGridSheet>) {
        self.mGridSheet = Some(grid);
    }
    // java private synchronized void recalculate() {
    // java this.mFreezeLeftX = 0;
    // java this.mFreezeTopY = 0;
    // java this.mFreezeXOffset = 0;
    // java this.mFreezeYOffset = 0;
    // java this.mFreezeOriginXOffset = 0;
    // java this.mFreezeOriginYOffset = 0;
    // java this.mFreezeOriginLeftX = 0;
    // java this.mFreezeOriginTopY = 0;
    // java this.mMaxRows = 0;
    // java this.mMaxColumns = 0;
    // java this.mDefaultRowHeight = -1;
    // java this.mDefaultColWidth = -1;
    // java this.mDefaultRowHeightUnScale = -1;
    // java this.mDefaultColWidthUnScale = -1;
    // java this.mCacheMaxScrollX = -1;
    // java this.mCacheMaxScrollY = -1;
    // java this.mCacheMaxContentX = -1;
    // java this.mCacheMaxContentY = -1;
    // java this.mCacheGridX = -1;
    // java this.mCacheGridY = -1;
    // java this.mCacheHeadingRowHeight = -1;
    // java this.mCacheHeadingColWidth = -1;
    // java this.mCacheDefaultCharWidth = -1.0F;
    // java this.mCacheDefaultCharWidthUnscale = -1.0F;
    // java this.mMaxRowsLimit = this.mGridSheet.maxRowsLimit();
    // java this.mMaxColumnsLimit = this.mGridSheet.maxColumnsLimit();
    // java this.mCacheMaxRowsIndex = this.mGridSheet.maxRowInfoIndex();
    // java this.mCacheMinRowsIndex = this.mGridSheet.minRowInfoIndex();
    // java this.mCacheMaxColumnIndex = this.mGridSheet.maxColInfoIndex();
    // java this.mDefaultRowDyZero = this.mGridSheet.isDefaultRowDyZero();
    // java KmoSheet sheet = this.mGridSheet.coreSheet();
    // java if (sheet == null) {
    // java this.mRowInfoIsEmpty = false;
    // java this.mMaxRows = 0;
    // java this.mMaxColumns = 0;
    // java } else {
    // java KmoRowInfos coreRowInfos = sheet.getRowInfos();
    // java this.mRowInfoIsEmpty = coreRowInfos.getFirstIndex() < 0 && coreRowInfos.getLastIndex() < 0;
    // java KmoRect kmoRect = SharedObjects.kmoRectPool.borrowObject();
    // java sheet.getUsedRange(kmoRect);
    // java this.mMaxRows = kmoRect.br.row;
    // java this.mMaxColumns = kmoRect.br.col;
    // java SharedObjects.kmoRectPool.returnObject(kmoRect);
    // java }
    // java
    // java this.initializeDefaultValue();
    // java this.initializeRowHeightCache();
    // java this.initializeColumnWidthCache();
    // java this.initializeFreezeScroll();
    // java this.initializeContentScroll();
    // java }
    pub fn recalculate(&mut self) {
        self.headerColWidth = -1;
        self.headerRowHeight = -1;
        self.mCacheDefaultCharWidth = -1.0;
        self.mCacheDefaultCharWidthUnscale = -1.0;
        self.mFreezeLeftX = 0;
        self.mFreezeTopY = 0;
        self.mFreezeXOffset = 0;
        self.mFreezeYOffset = 0;
        self.mFreezeOriginXOffset = 0;
        self.mFreezeOriginYOffset = 0;
        self.mFreezeOriginLeftX = 0;
        self.mFreezeOriginTopY = 0;
        self.mMaxRowsLimit = self.gridSheet().maxRowsLimit();
        self.mMaxColumnsLimit = self.gridSheet().maxColumnsLimit();
        let core = self.gridSheet().coreSheet();
        let sheet = core.borrow();
        self.mCacheMaxRowsIndex = sheet.maxRowInfoIndex();
        self.mCacheMinRowsIndex = sheet.minRowInfoIndex();
        self.mCacheMaxColumnIndex = sheet.maxColumnInfoIndex();
        self.mDefaultRowDyZero = self.gridSheet().isDefaultRowDyZero();
        let rows = sheet.getRowInfos();
        self.mRowInfoIsEmpty = rows.getFirstRow() < 0 && rows.getLastRow() < 0;
        let used = sheet.getUsedRange();
        self.mMaxRows = used.br.row;
        self.mMaxColumns = used.br.col;
        drop(rows);
        drop(sheet);
        self.initializeDefaultValue();
        self.initializeRowHeightCache();
        self.initializeColumnWidthCache();
        self.initializeFreezeScroll();
    }
    // java private void initializeDefaultValue() {
    // java this.defaultCharWidthInPixels();
    // java this.defaultCharWidthInPixelsUnscale();
    // java int defColWidth = 2048;
    // java if (this.mGridSheet.coreSheet() != null) {
    // java defColWidth = this.mGridSheet.coreSheet().emptyColWidth();
    // java }
    // java
    // java this.mDefaultColWidth = UnitsConverter.columnUnitsToPixels((float)defColWidth, this.mCacheDefaultCharWidth);
    // java this.mDefaultRowHeight = (int)this.mUnitsConverter.TwipsToPixelsY((float)this.mGridSheet.getDefRowHeight());
    // java this.mDefaultRowHeightUnScale = (int)this.mUnitsConverter.twipsToPixelsYUnScale((float)this.mGridSheet.getDefRowHeight());
    // java this.mDefaultColWidthUnScale = UnitsConverter.columnUnitsToPixels((float)defColWidth, this.mCacheDefaultCharWidthUnscale);
    // java this.mBaseRowHeight = this.mDefaultRowDyZero ? 0 : this.mDefaultRowHeight;
    // java }
    pub fn initializeDefaultValue(&mut self) {
        self.mCacheDefaultCharWidth = self.mGridTheme.getNormalCharWidth();
        self.mCacheDefaultCharWidthUnscale = self.mGridTheme.getNormalCharWidthUnScale();
        let def = self.gridSheet().coreSheet().borrow().emptyColWidth();
        self.mDefaultColWidth =
            UnitsConverter::columnUnitsToPixels(def as f32, self.mCacheDefaultCharWidth);
        self.mDefaultRowHeight =
            self.mUnitsConverter
                .TwipsToPixelsY(self.gridSheet().getDefRowHeight() as f32) as i32;
        self.mDefaultRowHeightUnScale = self
            .mUnitsConverter
            .twipsToPixelsYUnScale(self.gridSheet().getDefRowHeight() as f32)
            as i32;
        self.mDefaultColWidthUnScale =
            UnitsConverter::columnUnitsToPixels(def as f32, self.mCacheDefaultCharWidthUnscale);
        self.mBaseRowHeight = if self.mDefaultRowDyZero {
            0
        } else {
            self.mDefaultRowHeight
        };
    }
    // java private void initializeRowHeightCache() {
    // java if (this.mGridSheet != null && !(this.mGridSheet instanceof EmptyGridSheet)) {
    // java int cacheMaxIdx = this.mCacheMaxRowsIndex + 1;
    // java this.mCacheRowHeight = new int[cacheMaxIdx + 1];
    // java int aSumRowsHeight = 0;
    // java int aRowHeight = 0;
    // java
    // java for(int i = 0; i < cacheMaxIdx; ++i) {
    // java this.mCacheRowHeight[i] = aSumRowsHeight;
    // java aRowHeight = this.currentRowHeightPixels(i, false);
    // java aSumRowsHeight += aRowHeight;
    // java }
    // java
    // java this.mCacheRowHeight[cacheMaxIdx] = aSumRowsHeight;
    // java this.mGridRowCache.reset(this.mMaxRowsLimit, this.mCacheRowHeight, this.mDefaultRowDyZero ? 0 : this.mDefaultRowHeight);
    // java }
    // java }
    pub fn initializeRowHeightCache(&mut self) {
        let max = self.mCacheMaxRowsIndex + 1;
        let mut rows = vec![0; (max + 1) as usize];
        let mut sum = 0i32;
        for i in 0..max {
            rows[i as usize] = sum;
            sum = sum.wrapping_add(self.currentRowHeightPixels(i, false));
        }
        rows[max as usize] = sum;
        self.mGridRowCache.reset(
            self.mMaxRowsLimit,
            Some(&rows),
            if self.mDefaultRowDyZero {
                0
            } else {
                self.mDefaultRowHeight
            },
        );
        self.mCacheRowHeight = Some(rows);
    }
    // java private void initializeColumnWidthCache() {
    // java if (this.mGridSheet != null && !(this.mGridSheet instanceof EmptyGridSheet)) {
    // java IndexMapIterator<KmoColInfo> iterator = this.mGridSheet.colInfosIterator();
    // java if (null != iterator && iterator.hasNext()) {
    // java int maxColumnCache = this.mCacheMaxColumnIndex + 1;
    // java this.mCacheColumnWidth = new int[maxColumnCache + 1];
    // java int aSumColumnsWidth = 0;
    // java int aColumnWidth = 0;
    // java int initIndex = 0;
    // java
    // java while(iterator.hasNext()) {
    // java KmoColInfo colInfo = iterator.next();
    // java aColumnWidth = colInfo.getWidth();
    // java if (colInfo.isHidden()) {
    // java aColumnWidth = 0;
    // java } else {
    // java aColumnWidth = UnitsConverter.columnUnitsToPixels((float)aColumnWidth, this.mCacheDefaultCharWidth);
    // java }
    // java
    // java short colMinIndex = colInfo.getColIndex();
    // java if (initIndex > colMinIndex && colMinIndex >= 0) {
    // java initIndex = colMinIndex;
    // java if (colMinIndex < this.mCacheColumnWidth.length) {
    // java aSumColumnsWidth = this.mCacheColumnWidth[colMinIndex];
    // java }
    // java
    // java for(int i = colMinIndex; i < this.mCacheColumnWidth.length; ++i) {
    // java this.mCacheColumnWidth[i] = 0;
    // java }
    // java }
    // java
    // java for(int endIndex = Math.min(colMinIndex, this.mCacheColumnWidth.length); initIndex < endIndex; aSumColumnsWidth += this.mDefaultColWidth) {
    // java this.mCacheColumnWidth[initIndex++] = aSumColumnsWidth;
    // java }
    // java
    // java for(short maxColIndex = colInfo.getMaxColIndex(); initIndex <= maxColIndex && initIndex < this.mCacheColumnWidth.length; ++initIndex) {
    // java this.mCacheColumnWidth[initIndex] = aSumColumnsWidth;
    // java aSumColumnsWidth += aColumnWidth;
    // java }
    // java }
    // java
    // java this.mCacheColumnWidth[maxColumnCache] = aSumColumnsWidth;
    // java } else {
    // java this.mCacheColumnWidth = null;
    // java }
    // java }
    // java }
    pub fn initializeColumnWidthCache(&mut self) {
        let core = self.gridSheet().coreSheet();
        let sheet = core.borrow();
        let cols = sheet.getColInfos().borrow();
        if cols.iterator().next().is_none() {
            self.mCacheColumnWidth = None;
            return;
        }
        let max = self.mCacheMaxColumnIndex + 1;
        let mut cache = vec![0; (max + 1) as usize];
        let mut sum = 0i32;
        let mut index = 0i32;
        for col in cols.iterator() {
            let width = if col.isHidden() {
                0
            } else {
                UnitsConverter::columnUnitsToPixels(
                    col.getWidth() as f32,
                    self.mCacheDefaultCharWidth,
                )
            };
            let min = col.getColIndex() as i32;
            if index > min && min >= 0 {
                index = min;
                if min < cache.len() as i32 {
                    sum = cache[min as usize];
                }
                for v in &mut cache[min as usize..] {
                    *v = 0;
                }
            }
            let end = min.min(cache.len() as i32);
            while index < end {
                cache[index as usize] = sum;
                index += 1;
                sum = sum.wrapping_add(self.mDefaultColWidth);
            }
            while index <= col.getMaxColIndex() as i32 && index < cache.len() as i32 {
                cache[index as usize] = sum;
                sum = sum.wrapping_add(width);
                index += 1;
            }
        }
        cache[max as usize] = sum;
        self.mCacheColumnWidth = Some(cache);
    }
    // java private int coreRowHeightPixels(int row) {
    // java int aRowHeight = 0;
    // java KmoRowInfo rowInfo = null;
    // java
    // java try {
    // java rowInfo = this.mGridSheet.coreSheet().getRowInfos().getRowInfo(row);
    // java } catch (Exception var5) {
    // java rowInfo = null;
    // java }
    // java
    // java if (rowInfo == null) {
    // java return this.mDefaultRowDyZero ? 0 : this.mDefaultRowHeight;
    // java } else if (rowInfo.isHidden()) {
    // java return 0;
    // java } else {
    // java aRowHeight = rowInfo.getHeight();
    // java return aRowHeight >= 2 && aRowHeight <= 8192 ? (int)this.mUnitsConverter.TwipsToPixelsY((float)aRowHeight) : this.mDefaultRowHeight;
    // java }
    // java }
    pub fn coreRowHeightPixels(&self, row: i32) -> i32 {
        let core = self.gridSheet().coreSheet();
        let sheet = core.borrow();
        let rows = sheet.getRowInfos();
        let Some(info) = rows.getRowInfo(row) else {
            return if self.mDefaultRowDyZero {
                0
            } else {
                self.mDefaultRowHeight
            };
        };
        if info.isHidden() {
            0
        } else {
            let h = info.getHeight();
            if h >= 2 && h <= 8192 {
                self.mUnitsConverter.TwipsToPixelsY(h as f32) as i32
            } else {
                self.mDefaultRowHeight
            }
        }
    }
    // java private int currentRowHeightPixels(int row, boolean defaultHeight) {
    // java if (this.mGridSheet != null && row < this.mMaxRowsLimit && row >= 0) {
    // java return !defaultHeight || row <= this.mCacheMaxRowsIndex && row >= this.mCacheMinRowsIndex ? this.coreRowHeightPixels(row) : this.mBaseRowHeight;
    // java } else {
    // java return 0;
    // java }
    // java }
    pub fn currentRowHeightPixels(&self, row: i32, default: bool) -> i32 {
        if row < self.mMaxRowsLimit && row >= 0 {
            if !default || row <= self.mCacheMaxRowsIndex && row >= self.mCacheMinRowsIndex {
                self.coreRowHeightPixels(row)
            } else {
                self.mBaseRowHeight
            }
        } else {
            0
        }
    }
    // java private int currentColumnWidthPixels(int column, boolean defaultWidth) {
    // java if (this.mGridSheet != null && column >= 0 && column < this.mMaxColumnsLimit && !this.mGridSheet.isColHidden(column)) {
    // java if (defaultWidth) {
    // java return this.mDefaultColWidth;
    // java } else {
    // java int aColumnWidth = this.mGridSheet.columnWidth(column);
    // java if (aColumnWidth == 2048) {
    // java aColumnWidth = this.mDefaultColWidth;
    // java } else {
    // java aColumnWidth = UnitsConverter.columnUnitsToPixels((float)aColumnWidth, this.mCacheDefaultCharWidth);
    // java }
    // java
    // java return aColumnWidth;
    // java }
    // java } else {
    // java return 0;
    // java }
    // java }
    pub fn currentColumnWidthPixels(&self, col: i32, default: bool) -> i32 {
        if col >= 0 && col < self.mMaxColumnsLimit && !self.gridSheet().isColHidden(col) {
            if default {
                self.mDefaultColWidth
            } else {
                let w = self.gridSheet().columnWidth(col);
                if w == 2048 {
                    self.mDefaultColWidth
                } else {
                    UnitsConverter::columnUnitsToPixels(w as f32, self.mCacheDefaultCharWidth)
                }
            }
        } else {
            0
        }
    }
    // java public int rowHeightInPixels(int row) {
    // java if (row < 0) {
    // java return 0;
    // java } else {
    // java int aRowHeight = 0;
    // java int[] cacheRowHeight = this.mCacheRowHeight;
    // java if (cacheRowHeight == null) {
    // java aRowHeight = this.currentRowHeightPixels(row, true);
    // java } else if (row >= cacheRowHeight.length - 1) {
    // java aRowHeight = this.currentRowHeightPixels(row, true);
    // java } else {
    // java aRowHeight = cacheRowHeight[row + 1] - cacheRowHeight[row];
    // java }
    // java
    // java return aRowHeight;
    // java }
    // java }
    pub fn rowHeightInPixels(&self, row: i32) -> i32 {
        if row < 0 {
            return 0;
        }
        match &self.mCacheRowHeight {
            Some(c) if row < (c.len() as i32 - 1) => {
                c[row as usize + 1].wrapping_sub(c[row as usize])
            }
            _ => self.currentRowHeightPixels(row, true),
        }
    }
    // java public int columnWidthInPixels(int column) {
    // java if (column < 0) {
    // java return 0;
    // java } else {
    // java int aColumnWidth = 0;
    // java int[] cacheColumnWidth = this.mCacheColumnWidth;
    // java if (cacheColumnWidth == null) {
    // java aColumnWidth = this.currentColumnWidthPixels(column, true);
    // java } else if (column >= cacheColumnWidth.length - 1) {
    // java aColumnWidth = this.currentColumnWidthPixels(column, true);
    // java } else {
    // java aColumnWidth = cacheColumnWidth[column + 1] - cacheColumnWidth[column];
    // java }
    // java
    // java return aColumnWidth;
    // java }
    // java }
    pub fn columnWidthInPixels(&self, column: i32) -> i32 {
        if column < 0 {
            return 0;
        }
        match &self.mCacheColumnWidth {
            Some(c) if column < (c.len() as i32 - 1) => {
                c[column as usize + 1].wrapping_sub(c[column as usize])
            }
            _ => self.currentColumnWidthPixels(column, true),
        }
    }
    // java private int sumRowNotCacheDefHeight(int rowFrom, int rowTo) {
    // java if (!this.mRowInfoIsEmpty && rowFrom <= this.mCacheMaxRowsIndex && rowTo >= this.mCacheMinRowsIndex) {
    // java int sum = 0;
    // java int coreRowFrom = this.mCacheMinRowsIndex;
    // java int coreRowTo = this.mCacheMaxRowsIndex;
    // java if (rowFrom < this.mCacheMinRowsIndex) {
    // java sum += this.mBaseRowHeight * (this.mCacheMinRowsIndex - rowFrom);
    // java } else {
    // java coreRowFrom = rowFrom;
    // java }
    // java
    // java if (rowTo <= this.mCacheMaxRowsIndex) {
    // java coreRowTo = rowTo;
    // java } else {
    // java sum += this.mBaseRowHeight * (rowTo - this.mCacheMaxRowsIndex);
    // java }
    // java
    // java for(int i = coreRowFrom; i <= coreRowTo; ++i) {
    // java sum += this.coreRowHeightPixels(i);
    // java }
    // java
    // java return sum;
    // java } else {
    // java return this.mBaseRowHeight * (rowTo - rowFrom + 1);
    // java }
    // java }
    pub fn sumRowNotCacheDefHeight(&self, from: i32, to: i32) -> i32 {
        if !self.mRowInfoIsEmpty && from <= self.mCacheMaxRowsIndex && to >= self.mCacheMinRowsIndex
        {
            let mut sum = 0i32;
            let first = if from < self.mCacheMinRowsIndex {
                sum = sum.wrapping_add(
                    self.mBaseRowHeight
                        .wrapping_mul(self.mCacheMinRowsIndex - from),
                );
                self.mCacheMinRowsIndex
            } else {
                from
            };
            let last = if to <= self.mCacheMaxRowsIndex {
                to
            } else {
                sum = sum.wrapping_add(
                    self.mBaseRowHeight
                        .wrapping_mul(to - self.mCacheMaxRowsIndex),
                );
                self.mCacheMaxRowsIndex
            };
            for i in first..=last {
                sum = sum.wrapping_add(self.coreRowHeightPixels(i));
            }
            sum
        } else {
            self.mBaseRowHeight.wrapping_mul(to - from + 1)
        }
    }
    // java private int rowYNotCacheDefHeight(int row) {
    // java int aRowY = 0;
    // java if (this.mRowInfoIsEmpty) {
    // java return this.mBaseRowHeight * row;
    // java } else if (row < this.mCacheMinRowsIndex) {
    // java return this.mBaseRowHeight * row;
    // java } else {
    // java aRowY += this.mBaseRowHeight * row;
    // java if (row <= this.mCacheMaxRowsIndex) {
    // java for(int i = this.mCacheMinRowsIndex; i <= row; ++i) {
    // java aRowY += this.coreRowHeightPixels(i);
    // java }
    // java
    // java return aRowY;
    // java } else {
    // java for(int i = this.mCacheMinRowsIndex; i <= this.mCacheMaxRowsIndex; ++i) {
    // java aRowY += this.coreRowHeightPixels(i);
    // java }
    // java
    // java aRowY += this.mBaseRowHeight * (row - this.mCacheMaxRowsIndex);
    // java return aRowY;
    // java }
    // java }
    // java }
    pub fn rowYNotCacheDefHeight(&self, row: i32) -> i32 {
        if self.mRowInfoIsEmpty || row < self.mCacheMinRowsIndex {
            self.mBaseRowHeight.wrapping_mul(row)
        } else {
            let mut y = self.mBaseRowHeight.wrapping_mul(row);
            for i in self.mCacheMinRowsIndex..=row.min(self.mCacheMaxRowsIndex) {
                y = y.wrapping_add(self.coreRowHeightPixels(i));
            }
            if row > self.mCacheMaxRowsIndex {
                y = y.wrapping_add(
                    self.mBaseRowHeight
                        .wrapping_mul(row - self.mCacheMaxRowsIndex),
                );
            }
            y
        }
    }
    // java public int rowY(int row) {
    // java if (row < 0) {
    // java return this.mDefaultRowHeight;
    // java } else if (row == 0) {
    // java return 0;
    // java } else {
    // java int aRowY = 0;
    // java int[] cacheRowHeight = this.mCacheRowHeight;
    // java if (cacheRowHeight == null) {
    // java row = Math.max(row, this.mMaxRowsLimit) - 1;
    // java aRowY = this.rowYNotCacheDefHeight(row);
    // java } else {
    // java int rowHeightCacheLength = cacheRowHeight.length;
    // java if (row >= rowHeightCacheLength) {
    // java int aCacheRowIndex = this.mGridRowCache.startIndex(row);
    // java int aCacheRowY = this.mGridRowCache.startPos(row);
    // java if (aCacheRowY < cacheRowHeight[rowHeightCacheLength - 1]) {
    // java aCacheRowY = cacheRowHeight[rowHeightCacheLength - 1];
    // java aCacheRowIndex = cacheRowHeight.length - 1;
    // java }
    // java
    // java aCacheRowY += this.sumRowNotCacheDefHeight(aCacheRowIndex, row - 1);
    // java aRowY = aCacheRowY;
    // java } else {
    // java aRowY = cacheRowHeight[row];
    // java }
    // java }
    // java
    // java return aRowY;
    // java }
    // java }
    pub fn rowY(&self, row: i32) -> i32 {
        if row < 0 {
            return self.mDefaultRowHeight;
        }
        if row == 0 {
            return 0;
        }
        let Some(c) = &self.mCacheRowHeight else {
            return self.rowYNotCacheDefHeight(row.max(self.mMaxRowsLimit) - 1);
        };
        if row >= c.len() as i32 {
            let mut i = self.mGridRowCache.startIndex(row);
            let mut y = self.mGridRowCache.startPos(row);
            if y < c[c.len() - 1] {
                y = c[c.len() - 1];
                i = c.len() as i32 - 1;
            }
            y.wrapping_add(self.sumRowNotCacheDefHeight(i, row - 1))
        } else {
            c[row as usize]
        }
    }
    // java public int columnX(int column) {
    // java if (column < 0) {
    // java return this.mDefaultColWidth;
    // java } else {
    // java int aColumnX = 0;
    // java int[] cacheColumnWidth = this.mCacheColumnWidth;
    // java if (cacheColumnWidth == null) {
    // java for(int i = 0; i < column; ++i) {
    // java aColumnX += this.columnWidthInPixels(i);
    // java }
    // java } else if (column >= cacheColumnWidth.length) {
    // java int aCacheColumnIndex = cacheColumnWidth.length - 1;
    // java
    // java int aCacheColumnX;
    // java for(aCacheColumnX = cacheColumnWidth[aCacheColumnIndex]; aCacheColumnIndex < column; ++aCacheColumnIndex) {
    // java aCacheColumnX += this.columnWidthInPixels(aCacheColumnIndex);
    // java }
    // java
    // java aColumnX = aCacheColumnX;
    // java } else {
    // java aColumnX = cacheColumnWidth[column];
    // java }
    // java
    // java return aColumnX;
    // java }
    // java }
    pub fn columnX(&self, col: i32) -> i32 {
        if col < 0 {
            return self.mDefaultColWidth;
        }
        let mut x = 0i32;
        if let Some(c) = &self.mCacheColumnWidth {
            if col >= c.len() as i32 {
                let mut i = c.len() as i32 - 1;
                x = c[i as usize];
                while i < col {
                    x = x.wrapping_add(self.columnWidthInPixels(i));
                    i += 1;
                }
            } else {
                x = c[col as usize];
            }
        } else {
            for i in 0..col {
                x = x.wrapping_add(self.columnWidthInPixels(i));
            }
        }
        x
    }
    // java public int findColumnByXEx(float x) {
    // java if (x < 0.0F) {
    // java return 0;
    // java } else {
    // java int aColNum = 0;
    // java int[] cacheColumnWidth = this.mCacheColumnWidth;
    // java if (cacheColumnWidth != null && !(x > (float)cacheColumnWidth[cacheColumnWidth.length - 1])) {
    // java aColNum = GridCache.search((int)x, cacheColumnWidth);
    // java } else {
    // java int startCol = 0;
    // java int endCol = this.mMaxColumnsLimit;
    // java int aPosX = 0;
    // java int aIndex = 0;
    // java if (cacheColumnWidth != null) {
    // java startCol = cacheColumnWidth.length - 1;
    // java aPosX = cacheColumnWidth[cacheColumnWidth.length - 1];
    // java }
    // java
    // java for(aIndex = startCol; (float)aPosX < x && aIndex < endCol; ++aIndex) {
    // java aPosX += this.currentColumnWidthPixels(aIndex, true);
    // java }
    // java
    // java if ((float)aPosX == x) {
    // java aColNum = aIndex;
    // java } else {
    // java aColNum = aIndex - 1;
    // java }
    // java }
    // java
    // java return aColNum;
    // java }
    // java }
    pub fn findColumnByXEx(&self, x: f32) -> i32 {
        if x < 0.0 {
            return 0;
        }
        if let Some(c) = &self.mCacheColumnWidth {
            if x <= c[c.len() - 1] as f32 {
                return GridCache::search__static(x as i32, c);
            }
        }
        let (mut i, mut pos) = (0, 0i32);
        if let Some(c) = &self.mCacheColumnWidth {
            i = c.len() as i32 - 1;
            pos = c[c.len() - 1];
        }
        while (pos as f32) < x && i < self.mMaxColumnsLimit {
            pos = pos.wrapping_add(self.currentColumnWidthPixels(i, true));
            i += 1;
        }
        if pos as f32 == x {
            i
        } else {
            i - 1
        }
    }
    // java public int findRowByYEx(float y) {
    // java if (y < 0.0F) {
    // java return 0;
    // java } else {
    // java int aRowNum = 0;
    // java int[] cacheRowHeight = this.mCacheRowHeight;
    // java if (cacheRowHeight != null && !(y > (float)cacheRowHeight[cacheRowHeight.length - 1])) {
    // java aRowNum = GridCache.search((int)y, cacheRowHeight);
    // java } else {
    // java int startRow = 0;
    // java int endRow = this.mMaxRowsLimit;
    // java int aPosY = 0;
    // java GridCache.CacheInfo rowRange = GridCache.CacheInfo.obtain();
    // java this.mGridRowCache.search((int)y, rowRange);
    // java if (rowRange.mStartIndex > -1) {
    // java startRow = rowRange.mStartIndex;
    // java endRow = rowRange.mEndIndex < this.mMaxRowsLimit ? rowRange.mEndIndex : this.mMaxRowsLimit;
    // java aPosY = rowRange.mStartPos;
    // java }
    // java
    // java rowRange.recycle();
    // java int aIndex = 0;
    // java
    // java for(aIndex = startRow; (float)aPosY < y && aIndex < endRow; ++aIndex) {
    // java aPosY += this.currentRowHeightPixels(aIndex, true);
    // java }
    // java
    // java if ((float)aPosY == y) {
    // java aRowNum = aIndex;
    // java } else {
    // java aRowNum = aIndex - 1;
    // java }
    // java }
    // java
    // java return aRowNum;
    // java }
    // java }
    pub fn findRowByYEx(&self, y: f32) -> i32 {
        if y < 0.0 {
            return 0;
        }
        if let Some(c) = &self.mCacheRowHeight {
            if y <= c[c.len() - 1] as f32 {
                return GridCache::search__static(y as i32, c);
            }
        }
        let info = self.mGridRowCache.search(y as i32);
        let (mut i, mut end, mut pos) = (0, self.mMaxRowsLimit, 0i32);
        if info.mStartIndex > -1 {
            i = info.mStartIndex;
            end = info.mEndIndex.min(self.mMaxRowsLimit);
            pos = info.mStartPos;
        }
        while (pos as f32) < y && i < end {
            pos = pos.wrapping_add(self.currentRowHeightPixels(i, true));
            i += 1;
        }
        if pos as f32 == y {
            i
        } else {
            i - 1
        }
    }
    // java public int clientOriginX() {
    // java int x = this.mRoGridClientData.getGridXOffset();
    // java if (this.mIgnoreHeadSize) {
    // java return x;
    // java } else {
    // java if (this.mHeaderState != null && !this.mHeaderState.isNormalHeader()) {
    // java x += this.mRoGridClientData.getClientsXOffset();
    // java } else {
    // java x += this.headingColWidthInPixels();
    // java }
    // java
    // java return x;
    // java }
    // java }
    pub fn clientOriginX(&self) -> i32 {
        {
            let x = self.mRoGridClientData.getGridXOffset();
            if self.mIgnoreHeadSize {
                x
            } else if self
                .mHeaderState
                .as_ref()
                .is_some_and(|h| !h.isNormalHeader())
            {
                x + self.mRoGridClientData.getClientsXOffset()
            } else {
                x + self.headingColWidthInPixels()
            }
        }
    }
    // java public int clientOriginY() {
    // java int y = this.mRoGridClientData.getGridYOffset();
    // java if (this.mIgnoreHeadSize) {
    // java return y;
    // java } else {
    // java if (this.mHeaderState != null && !this.mHeaderState.isNormalHeader()) {
    // java y += this.mRoGridClientData.getClientsYOffset();
    // java } else {
    // java y += this.headingRowHeightInPixels();
    // java }
    // java
    // java return y;
    // java }
    // java }
    pub fn clientOriginY(&self) -> i32 {
        {
            let y = self.mRoGridClientData.getGridYOffset();
            if self.mIgnoreHeadSize {
                y
            } else if self
                .mHeaderState
                .as_ref()
                .is_some_and(|h| !h.isNormalHeader())
            {
                y + self.mRoGridClientData.getClientsYOffset()
            } else {
                y + self.headingRowHeightInPixels()
            }
        }
    }
    // java public int logicalColXPos(int column) {
    // java int xPos = this.columnX(column);
    // java xPos += this.clientOriginX();
    // java return xPos;
    // java }
    pub fn logicalColXPos(&self, col: i32) -> i32 {
        self.columnX(col) + self.clientOriginX()
    }
    // java public int logicalRowYPos(int row) {
    // java int yPos = this.rowY(row);
    // java yPos += this.clientOriginY();
    // java return yPos;
    // java }
    pub fn logicalRowYPos(&self, row: i32) -> i32 {
        self.rowY(row) + self.clientOriginY()
    }
    // java public int deviceColXPos(int column) {
    // java int xPos = this.columnX(column) - this.mScrollX;
    // java xPos += this.clientOriginX();
    // java return xPos;
    // java }
    pub fn deviceColXPos(&self, col: i32) -> i32 {
        self.columnX(col) - self.mScrollX + self.clientOriginX()
    }
    // java public int deviceRowYPos(int row) {
    // java int yPos = this.rowY(row) - this.mScrollY;
    // java yPos += this.clientOriginY();
    // java return yPos;
    // java }
    pub fn deviceRowYPos(&self, row: i32) -> i32 {
        self.rowY(row) - self.mScrollY + self.clientOriginY()
    }
    // java public int rowHeightInPixelsWithoutCache(int row) {
    // java return this.currentRowHeightPixels(row, false);
    // java }
    pub fn rowHeightInPixelsWithoutCache(&self, row: i32) -> i32 {
        self.currentRowHeightPixels(row, false)
    }
    // java public int columnWidthInPixelsWithoutCache(int column) {
    // java return this.currentColumnWidthPixels(column, false);
    // java }
    pub fn columnWidthInPixelsWithoutCache(&self, col: i32) -> i32 {
        self.currentColumnWidthPixels(col, false)
    }
    // java public int headingRowHeightInPixels() {
    // java if (this.mCacheHeadingRowHeight == -1) {
    // java this.mCacheHeadingRowHeight = this.mGridTheme.getHeaderRowHeight();
    // java }
    // java
    // java return this.mCacheHeadingRowHeight;
    // java }
    pub fn headingRowHeightInPixels(&self) -> i32 {
        if self.headerRowHeight == -1 {
            self.mGridTheme.getHeaderRowHeight()
        } else {
            self.headerRowHeight
        }
    }
    // java public int headingColWidthInPixels() {
    // java if (this.mCacheHeadingColWidth == -1) {
    // java this.mCacheHeadingColWidth = this.mGridTheme.getHeaderColWidth();
    // java }
    // java
    // java return this.mCacheHeadingColWidth;
    // java }
    pub fn headingColWidthInPixels(&self) -> i32 {
        if self.headerColWidth == -1 {
            self.mGridTheme.getHeaderColWidth()
        } else {
            self.headerColWidth
        }
    }
    // java public float defaultCharWidthInPixels() {
    // java if (this.mCacheDefaultCharWidth == -1.0F) {
    // java this.mCacheDefaultCharWidth = this.mGridTheme.getNormalCharWidth();
    // java }
    // java
    // java return this.mCacheDefaultCharWidth;
    // java }
    pub fn defaultCharWidthInPixels(&self) -> f32 {
        if self.mCacheDefaultCharWidth == -1.0 {
            self.mGridTheme.getNormalCharWidth()
        } else {
            self.mCacheDefaultCharWidth
        }
    }
    // java public void setIgnoreHeadSize(boolean b) {
    // java this.mIgnoreHeadSize = b;
    // java }
    pub fn setIgnoreHeadSize(&mut self, v: bool) {
        self.mIgnoreHeadSize = v;
    }
}

impl GridCoreData for GridSheetLayoutInfo {
    fn resetSheet(&mut self, g: Rc<dyn IGridSheet>) {
        self.resetSheet(g)
    }
    fn refreshData(&mut self) {
        self.refreshData()
    }
} // java public boolean isSheetRightToLeft() {
  // java return this.mGridSheet != null && this.mGridSheet.coreSheet() != null ? this.mGridSheet.coreSheet().isSheetRightToLeft() : false;
  // java }
impl GridSheetLayoutInfo {
    pub fn isSheetRightToLeft(&self) -> bool {
        self.mGridSheet
            .as_ref()
            .is_some_and(|g| g.coreSheet().borrow().isSheetRightToLeft())
    }
}

impl GridSheetLayoutInfo {
    // java public void convertRtl(Canvas canvas, Rect rect) {
    // java if (this.isSheetRightToLeft()) {
    // java canvas.scale(-1.0F, 1.0F, (float)rect.centerX(), (float)rect.centerY());
    // java }
    // java
    // java }
    pub fn convertRtl(
        &self,
        c: &mut super::super::render::graphics::Canvas,
        r: &super::super::render::graphics::Rect,
    ) {
        if self.isSheetRightToLeft() {
            c.scale(
                -1.0,
                1.0,
                ((r.left.wrapping_add(r.right)) >> 1) as f32,
                ((r.top.wrapping_add(r.bottom)) >> 1) as f32,
            );
        }
    }
    // java public boolean isDrawGridline() {
    // java return this.mGridSheet.isDrawGrid();
    // java }
    pub fn isDrawGridline(&self) -> bool {
        self.gridSheet().isDrawGrid()
    }
}
