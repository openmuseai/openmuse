// Java: import cn.wps.moffice.writer.data.table.Data
use crate::cn::wps::moffice::writer::data::table::Data::*;
// Java: import cn.wps.moffice.writer.data.table.grid.GridCols
use crate::cn::wps::moffice::writer::data::table::grid::GridCols::*;
// Java: import cn.wps.moffice.writer.data.table.grid.GridCell
use crate::cn::wps::moffice::writer::data::table::grid::GridCell::*;

// java public interface GridCol
pub trait GridCol {
    // java  GridCols getParent()
    fn getParent(&self) -> &(dyn GridCols);

    // java  GridCol getPre()
    fn getPre(&self) -> &(dyn GridCol);

    // java  GridCol getNext()
    fn getNext(&self) -> &(dyn GridCol);

    // java  int getLeftPos()
    fn getLeftPos(&self) -> i32;

    // java  int getWidth()
    fn getWidth(&self) -> i32;

    // java  int getRightPos()
    fn getRightPos(&self) -> i32;

    // java  int index()
    fn index(&mut self) -> i32;

    // java  int size()
    fn size(&self) -> i32;

    // java  GridCell getCell(int var1)
    fn getCell(&self, var1: i32) -> &(dyn GridCell);

    // java  int indexOf(GridCell var1)
    fn indexOf(&mut self, var1: &dyn GridCell) -> i32;

    // java  void setCell(int var1, GridCell var2)
    fn setCell(&mut self, var1: i32, var2: &dyn GridCell);

    // java  GridCell getLongestCellNoSpand()
    fn getLongestCellNoSpand(&self) -> &(dyn GridCell);

    // java  GridCell getFirstNotAutoWidth()
    fn getFirstNotAutoWidth(&self) -> &(dyn GridCell);

    // java  GridCell getFirstCellBeginInRow()
    fn getFirstCellBeginInRow(&self) -> &(dyn GridCell);

    // java  GridCell getFirstCellAtSpandBegin()
    fn getFirstCellAtSpandBegin(&self) -> &(dyn GridCell);

    // java  GridCell getFirstCellAtSpandEnd()
    fn getFirstCellAtSpandEnd(&self) -> &(dyn GridCell);

    // java  Data.GridColData getData()
    // rust: Java 嵌套类 `Data.GridColData` 在 Rust 侧是同模块的 `GridColData`。
    fn getData(&self) -> crate::cn::wps::moffice::writer::data::table::Data::GridColData;

}
