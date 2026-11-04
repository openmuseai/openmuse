// java package cn.wps.moss.app;

// java import cn.wps.moss.engine.grid.KmoCellData;
// java import cn.wps.moss.util.KmoRect;
use crate::java_runtime::RefCell;
use crate::java_runtime::Rc;

use super::IGridSheet::IGridSheet;
use super::KmoSheet::{CellTypeInfo, KmoSheet};

// java class GridSheetData implements IGridSheet
pub struct GridSheetData {
    // java private KmoSheet sheet;
    sheet: Rc<RefCell<KmoSheet>>,
}

// java class GridSheetData implements IGridSheet
impl GridSheetData {
    // java public GridSheetData(KmoSheet sheet)
    pub fn GridSheetData(sheet: Rc<RefCell<KmoSheet>>) -> Self {
        // java this.sheet = sheet;
        Self { sheet }
    }
}

impl IGridSheet for GridSheetData {
    // java public KmoSheet coreSheet()
    fn coreSheet(&self) -> Rc<RefCell<KmoSheet>> {
        // java return this.sheet;
        Rc::clone(&self.sheet)
    }

    // java public boolean isDrawHeader()
    fn isDrawHeader(&self)->bool{self.sheet.borrow().isDrawHeader()}

    // java public boolean isDrawGrid()
    fn isDrawGrid(&self)->bool{self.sheet.borrow().isDrawGridLines()}

    // java public int rowHeight(int row)
    fn rowHeight(&self, row: i32) -> i32 {
        // java return this.sheet.getRowHeight(row);
        self.sheet.borrow().getRowHeight(row)
    }

    // java public boolean isRowHidden(int row)
    fn isRowHidden(&self,row:i32)->bool{self.sheet.borrow().isRowHidden(row)}

    // java public int columnWidth(int col)
    fn columnWidth(&self, col: i32) -> i32 {
        // Java: return this.sheet.getColWidth((short)col);
        self.sheet.borrow().getColWidth((col as i16) as i32)
    }

    // java public boolean isColHidden(int col)
    fn isColHidden(&self,col:i32)->bool{self.sheet.borrow().isColHidden(col)}

    // java public int getCellType(int row, int col, KmoSheet.CellTypeInfo info)
    fn getCellType(&self, row: i32, col: i32, info: Option<&mut CellTypeInfo>) -> i32 {
        // java return this.sheet.getCellValueType(row, col, info);
        self.sheet.borrow().getCellValueType(row, col, info)
    }

    // java public int maxRows()
    fn maxRows(&self) -> i32 {
        // java return this.sheet.getUsedRange().br.row;
        self.sheet.borrow().getUsedRange().br.row
    }

    // java public int maxColumns()
    fn maxColumns(&self) -> i32 {
        // java return this.sheet.getUsedRange().br.col;
        self.sheet.borrow().getUsedRange().br.col
    }

    // java public int maxColumnsLimit()
    fn maxColumnsLimit(&self) -> i32 {
        // java return this.sheet.getBook().getSpreadsheetVersion().getMaxColumns();
        self.sheet.borrow().getMaxColumns()
    }

    // java public int maxRowsLimit()
    fn maxRowsLimit(&self) -> i32 {
        // java return this.sheet.getBook().getSpreadsheetVersion().getMaxRows();
        self.sheet.borrow().getMaxRows()
    }

    // java public int getDefRowHeight()
    fn getDefRowHeight(&self) -> i32 {
        // java return this.sheet.getDefRowHeight();
        self.sheet.borrow().getDefRowHeight()
    }

    // java public boolean isDefaultRowDyZero()
    fn isDefaultRowDyZero(&self) -> bool {
        // java return this.sheet.isDefaultRowDyZero();
        self.sheet.borrow().isDefaultRowDyZero()
    }

    // java public boolean isEmptySheet()
    fn isEmptySheet(&self) -> bool {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java return this.sheet.isEmptySheet()
        */
    }

    // java public boolean isCellBlank(int row, int col)
    fn isCellBlank(&self, _row: i32, _col: i32) -> bool {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java return this.sheet.isCellBlank(row, col)
        */
    }
}

impl GridSheetData {
    // java public KmoXF getCellXF(int row, int col)
    pub fn getCellXF(&self,row:i32,col:i32)->Option<Rc<super::xf::KmoXF::KmoXF>>{self.sheet.borrow().getCellXF__2(row,col)}

    // java public NumFmtResult getCellValue(int row, int col, int maxWidth)
    pub fn getCellValue(&self,row:i32,col:i32,maxWidth:i32)->Rc<RefCell<kfc::cn::wps::kfc::numfmt::format::NumFmtResult::NumFmtResult>> {
self.sheet.borrow().getNumFormatResult(row,col,maxWidth)
    }

    // java public UnicodeString getRunsString(int row, int col)
    pub fn getRunsString(&self,row:i32,col:i32)->Option<Rc<crate::cn::wps::moss::filefmt::biff8::record::common::UnicodeString::UnicodeString>>{<Self as IGridSheet>::getRunsString(self,row,col)}

    // java public String getRowName(int row)
    pub fn getRowName(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java return AppLogicHelper.getRowName(row)
        */
    }

    // java public String getColName(int col)
    pub fn getColName(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java return AppLogicHelper.getColName(col)
        */
    }

    // java public boolean isRowFormatted(int row)
    pub fn isRowFormatted(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java return this.sheet.isRowFormatted(row)
        */
    }

    // java public KmoXF getRowXF(int row)
    pub fn getRowXF(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java return this.sheet.getRowXF(row)
        */
    }

    // java public KmoPalette getPalette()
    pub fn getPalette(&self)->Rc<crate::cn::wps::moss::core::book::KmoPalette::KmoPalette>{self.sheet.borrow().getBook().getPalette()}

    // java public KmoRect getMergedRange(int row, int col)
    pub fn getMergedRange(&self,row:i32,col:i32)->Option<crate::cn::wps::moss::util::KmoRect::KmoRect>{
      // java return this.sheet.getMergedRange(row, col);
      self.sheet.borrow().getMergedRange(row,col)
    }

    // java public void createFrozenPane(int topRow, int leftCol, int row, int col)
    pub fn createFrozenPane(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java this.sheet.createFrozenPane(topRow, leftCol, row, col)
        */
    }

    // java public void removeFrozenPane()
    pub fn removeFrozenPane(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java this.sheet.removeFrozenPane()
        */
    }

    // java public void scrollPane(short topRow, short leftmostColumn)
    pub fn scrollPane(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java this.sheet.scrollPane(topRow, leftmostColumn)
        */
    }

    // java public boolean hasFrozen()
    pub fn hasFrozen(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java return this.sheet.hasFrozen()
        */
    }

    // java public int getPaneColSplit()
    pub fn getPaneColSplit(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java return this.sheet.getPaneColSplit()
        */
    }

    // java public int getPaneRowSplit()
    pub fn getPaneRowSplit(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java return this.sheet.getPaneRowSplit()
        */
    }

    // java public int getPaneTopRow()
    pub fn getPaneTopRow(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java return this.sheet.getPaneTopRow()
        */
    }

    // java public int getPaneLeftCol()
    pub fn getPaneLeftCol(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java return this.sheet.getPaneLeftCol()
        */
    }

    // java public int getActivePane()
    pub fn getActivePane(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java return this.sheet.getActivePane()
        */
    }

    // java public KmoXFManager getXFMgr()
    pub fn getXFMgr(&self)->Rc<RefCell<super::xf::KmoXFManager::KmoXFManager>>{self.sheet.borrow().getBook().xfManagerHandle()}

    // java public void setSelection(KmoRect rect, int active_row, int active_col)
    pub fn setSelection(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java this.sheet.setSelection(rect, active_row, active_col)
        */
    }

    // java public KmoRect getSelction()
    pub fn getSelction(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java return this.sheet.getSelectionRect()
        */
    }

    // java public int getActiveRow()
    pub fn getActiveRow(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java return this.sheet.getSelection().activeRow()
        */
    }

    // java public int getActiveCol()
    pub fn getActiveCol(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java return this.sheet.getSelection().activeCol()
        */
    }

    // java public int minRowInfoIndex()
    pub fn minRowInfoIndex(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java return this.sheet.minRowInfoIndex()
        */
    }

    // java public int maxRowInfoIndex()
    pub fn maxRowInfoIndex(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java return this.sheet.maxRowInfoIndex()
        */
    }

    // java public int maxColInfoIndex()
    pub fn maxColInfoIndex(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java return this.sheet.maxColumnInfoIndex()
        */
    }

    // java public void setScale(short nscl, short dscl)
    pub fn setScale(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java this.sheet.setScale(nscl, dscl)
        */
    }

    // java public short getNScale()
    pub fn getNScale(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java return this.sheet.getNScale()
        */
    }

    // java public short getDScale()
    pub fn getDScale(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java return this.sheet.getDScale()
        */
    }

    // java public int getDefColWidth()
    pub fn getDefColWidth(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java return this.sheet.getDefColWidth()
        */
    }

    // java public int getLeftTopRow()
    pub fn getLeftTopRow(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java return this.sheet.leftTopRow()
        */
    }

    // java public int getLeftTopCol()
    pub fn getLeftTopCol(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java return this.sheet.leftTopCol()
        */
    }

    // java public SheetDrawingAgg getSheetDrawingAgg()
    pub fn getSheetDrawingAgg(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java return this.sheet.getSheetDrawingAgg()
        */
    }

    // java public void setLeftTopRow(int row)
    pub fn setLeftTopRow(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java this.sheet.setLeftTopRow(row)
        */
    }

    // java public void setLeftTopCol(int col)
    pub fn setLeftTopCol(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java this.sheet.setLeftTopCol(col)
        */
    }

    // java public void currentGridRange(int beginRow, int endRow)
    pub fn currentGridRange(&self,first:i32,last:i32){<Self as IGridSheet>::currentGridRange(self,first,last)}


    // java public boolean isSingleCell(int firstRow, int lastRow, int firstCol, int lastCol)
    pub fn isSingleCell(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java return this.sheet.isSingleCell(firstRow, lastRow, firstCol, lastCol)
        */
    }

    // java public IndexMapIterator<KmoColInfo> colInfosIterator()
    pub fn colInfosIterator(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java KmoColInfos colInfos = this.sheet.getColInfos()
        // java return colInfos.iterator()
        */
    }

    // java public KmoRect getLastSelection()
    pub fn getLastSelection(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java return this.sheet.getLastSelection()
        */
    }
}
