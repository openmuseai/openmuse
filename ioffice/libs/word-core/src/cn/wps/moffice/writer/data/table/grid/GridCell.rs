// Java: import cn.wps.moffice.writer.data.table.CellInfo
use crate::cn::wps::moffice::writer::data::table::CellInfo::*;
// Java: import cn.wps.moffice.writer.data.table.grid.GridCol
use crate::cn::wps::moffice::writer::data::table::grid::GridCol::*;

// java public interface GridCell
pub trait GridCell {
    // java  GridCol getParent()
    fn getParent(&self) -> &(dyn GridCol);

    // java  int getIndex()
    fn getIndex(&self) -> i32;

    // java  CellInfo cell()
    fn cell(&mut self) -> &dyn CellInfo;

    // java  GridCell firstSpan()
    fn firstSpan(&mut self) -> &(dyn GridCell);

    // java  GridCell getPreSpan()
    fn getPreSpan(&self) -> &(dyn GridCell);

    // java  GridCell getNextSpan()
    fn getNextSpan(&self) -> &(dyn GridCell);

    // java  GridCell getPreGridCell()
    fn getPreGridCell(&self) -> &(dyn GridCell);

    // java  int getSpanIndex()
    fn getSpanIndex(&self) -> i32;

    // java  int getSpanCount()
    fn getSpanCount(&self) -> i32;

}
