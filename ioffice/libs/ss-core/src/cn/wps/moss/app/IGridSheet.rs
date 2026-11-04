// java package cn.wps.moss.app;

// java import cn.wps.moss.util.KmoRect;
use crate::java_runtime::RefCell;
use crate::java_runtime::Rc;

use super::KmoSheet::{CellTypeInfo, KmoSheet};

// java public interface IGridSheet
pub trait IGridSheet {
    // java KmoSheet coreSheet();
    fn coreSheet(&self) -> Rc<RefCell<KmoSheet>>;
    // java boolean isDrawHeader();
    fn isDrawHeader(&self) -> bool;
    // java boolean isDrawGrid();
    fn isDrawGrid(&self) -> bool;
    // java int rowHeight(int var1);
    fn rowHeight(&self, row: i32) -> i32;
    // java boolean isRowHidden(int var1);
    fn isRowHidden(&self, row: i32) -> bool;
    // java int columnWidth(int var1);
    fn columnWidth(&self, col: i32) -> i32;
    // java boolean isColHidden(int var1);
    fn isColHidden(&self, col: i32) -> bool;
    // java int getCellType(int var1, int var2, KmoSheet.CellTypeInfo var3);
    fn getCellType(&self, row: i32, col: i32, info: Option<&mut CellTypeInfo>) -> i32;
    // java int maxRows();
    fn maxRows(&self) -> i32;
    // java int maxColumnsLimit();
    fn maxColumnsLimit(&self) -> i32;
    // java int maxRowsLimit();
    fn maxRowsLimit(&self) -> i32;
    // java int maxColumns();
    fn maxColumns(&self) -> i32;
    // java boolean isEmptySheet();
    fn isEmptySheet(&self) -> bool;
    // java boolean isCellBlank(int var1, int var2);
    fn isCellBlank(&self, row: i32, col: i32) -> bool;
    // java int getDefRowHeight();
    fn getDefRowHeight(&self) -> i32;
    // java boolean isDefaultRowDyZero();
    fn isDefaultRowDyZero(&self) -> bool;
    // java KmoXF getCellXF(int var1, int var2);
    fn getCellXF(&self,row:i32,col:i32)->Option<Rc<super::xf::KmoXF::KmoXF>>{self.coreSheet().borrow().getCellXF__2(row,col)}
    // java NumFmtResult getCellValue(int var1, int var2, int var3);
    fn getCellValue(&self,row:i32,col:i32,max_width:i32)->Rc<RefCell<kfc::cn::wps::kfc::numfmt::format::NumFmtResult::NumFmtResult>> {self.coreSheet().borrow().getNumFormatResult(row,col,max_width)}
    // java UnicodeString getRunsString(int var1, int var2);
    fn getRunsString(&self,row:i32,col:i32)->Option<Rc<crate::cn::wps::moss::filefmt::biff8::record::common::UnicodeString::UnicodeString>>{let sheet=self.coreSheet();let sheet=sheet.borrow();let data=sheet.getCellData(row,col);let result=sheet.getCellValueStore().borrow_mut().runsStringValue(data.id);result}
    // java String getRowName(int var1);
    fn getRowName(&self, _row: i32) {
        todo!("DEFERRED_NON_FUNCTION");
    }
    // java String getColName(int var1);
    fn getColName(&self, _col: i32) {
        todo!("DEFERRED_NON_FUNCTION");
    }
    // java boolean isRowFormatted(int var1);
    fn isRowFormatted(&self, _row: i32) -> bool {
        todo!("DEFERRED_NON_FUNCTION");
    }
    // java KmoXF getRowXF(int var1);
    fn getRowXF(&self, _row: i32) {
        todo!("DEFERRED_NON_FUNCTION");
    }
    // java KmoPalette getPalette();
    fn getPalette(&self)->Rc<crate::cn::wps::moss::core::book::KmoPalette::KmoPalette>{self.coreSheet().borrow().getBook().getPalette()}
    // java KmoRect getMergedRange(int var1, int var2);
    fn getMergedRange(&self,row:i32,col:i32)->Option<crate::cn::wps::moss::util::KmoRect::KmoRect>{self.coreSheet().borrow().getMergedRange(row,col)}
    // java void createFrozenPane(int var1, int var2, int var3, int var4);
    fn createFrozenPane(&self, _top_row: i32, _left_col: i32, _row: i32, _col: i32) {
        todo!("DEFERRED_NON_FUNCTION");
    }
    // java void removeFrozenPane();
    fn removeFrozenPane(&self) {
        todo!("DEFERRED_NON_FUNCTION");
    }
    // java void scrollPane(short var1, short var2);
    fn scrollPane(&self, _top_row: i16, _leftmost_column: i16) {
        todo!("DEFERRED_NON_FUNCTION");
    }
    // java boolean hasFrozen();
    fn hasFrozen(&self)->bool{self.coreSheet().borrow().hasFrozen()}
    // java int getPaneColSplit();
    fn getPaneColSplit(&self)->i32{self.coreSheet().borrow().getPaneColSplit()}
    // java int getPaneRowSplit();
    fn getPaneRowSplit(&self)->i32{self.coreSheet().borrow().getPaneRowSplit()}
    // java int getPaneTopRow();
    fn getPaneTopRow(&self)->i32{self.coreSheet().borrow().getPaneTopRow()}
    // java int getPaneLeftCol();
    fn getPaneLeftCol(&self)->i32{self.coreSheet().borrow().getPaneLeftCol()}
    // java int getActivePane();
    fn getActivePane(&self)->i32{self.coreSheet().borrow().getActivePane()}
    // java KmoXFManager getXFMgr();
    fn getXFMgr(&self)->Option<Rc<RefCell<super::xf::KmoXFManager::KmoXFManager>>>{Some(self.coreSheet().borrow().getBook().xfManagerHandle())}
    // java void setSelection(KmoRect var1, int var2, int var3);
    fn setSelection(&self, _rect: (), _active_row: i32, _active_col: i32) {
        todo!("DEFERRED_NON_FUNCTION");
    }
    // java KmoRect getSelction();
    fn getSelction(&self) {
        todo!("DEFERRED_NON_FUNCTION");
    }
    // java int getActiveRow();
    fn getActiveRow(&self) -> i32 {
        todo!("DEFERRED_NON_FUNCTION");
    }
    // java int getActiveCol();
    fn getActiveCol(&self) -> i32 {
        todo!("DEFERRED_NON_FUNCTION");
    }
    // java int minRowInfoIndex();
    fn minRowInfoIndex(&self) -> i32 {
        todo!("DEFERRED_NON_FUNCTION");
    }
    // java int maxRowInfoIndex();
    fn maxRowInfoIndex(&self) -> i32 {
        todo!("DEFERRED_NON_FUNCTION");
    }
    // java int maxColInfoIndex();
    fn maxColInfoIndex(&self) -> i32 {
        todo!("DEFERRED_NON_FUNCTION");
    }
    // java void setScale(short var1, short var2);
    fn setScale(&self, _nscl: i16, _dscl: i16) {
        todo!("DEFERRED_NON_FUNCTION");
    }
    // java short getNScale();
    fn getNScale(&self) -> i16 {
        todo!("DEFERRED_NON_FUNCTION");
    }
    // java short getDScale();
    fn getDScale(&self) -> i16 {
        todo!("DEFERRED_NON_FUNCTION");
    }
    // java int getDefColWidth();
    fn getDefColWidth(&self) -> i32 {
        todo!("DEFERRED_NON_FUNCTION");
    }
    // java int getLeftTopRow();
    fn getLeftTopRow(&self)->i32{self.coreSheet().borrow().leftTopRow()}
    // java void setLeftTopRow(int var1);
    fn setLeftTopRow(&self, _row: i32) {
        todo!("DEFERRED_NON_FUNCTION");
    }
    // java int getLeftTopCol();
    fn getLeftTopCol(&self)->i32{self.coreSheet().borrow().leftTopCol()}
    // java SheetDrawingAgg getSheetDrawingAgg();
    fn getSheetDrawingAgg(&self) {
        todo!("DEFERRED_NON_FUNCTION");
    }
    // java void setLeftTopCol(int var1);
    fn setLeftTopCol(&self, _col: i32) {
        todo!("DEFERRED_NON_FUNCTION");
    }
    // java void currentGridRange(int var1, int var2);
    fn currentGridRange(&self,first:i32,last:i32){let sheet=self.coreSheet();let reader=sheet.borrow().getBook().getBookReader();if let Some(reader)=reader{reader.currentGridRange(sheet,first,last);}}

    // java boolean isSingleCell(int var1, int var2, int var3, int var4);
    fn isSingleCell(&self, _first_row: i32, _last_row: i32, _first_col: i32, _last_col: i32) -> bool {
        todo!("DEFERRED_NON_FUNCTION");
    }
    // java IndexMapIterator<KmoColInfo> colInfosIterator();
    fn colInfosIterator(&self) {
        todo!("DEFERRED_NON_FUNCTION");
    }
    // java KmoRect getLastSelection();
    fn getLastSelection(&self) {
        todo!("DEFERRED_NON_FUNCTION");
    }
}
