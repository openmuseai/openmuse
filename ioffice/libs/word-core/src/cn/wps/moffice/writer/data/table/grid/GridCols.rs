// Java: import cn.wps.moffice.writer.data.table.TableInfo
use crate::cn::wps::moffice::writer::data::table::TableInfo::*;
// Java: import java.util.Iterator
// Java: import cn.wps.moffice.writer.data.table.grid.GridCol
use crate::cn::wps::moffice::writer::data::table::grid::GridCol::*;
// Java: import cn.wps.moffice.writer.data.table.grid.GridAutoSize
use crate::cn::wps::moffice::writer::data::table::grid::GridAutoSize::*;

// java public interface GridCols
pub trait GridCols {
    // java  int columSize()
    fn columSize(&mut self) -> i32;

    // java  int rowSize()
    fn rowSize(&mut self) -> i32;

    // java  GridCol getColByIndex(int var1)
    fn getColByIndex(&self, var1: i32) -> &(dyn GridCol);

    // java  Iterator<GridCol> getIterator()
    fn getIterator(&self) -> Vec<Box<dyn GridCol>>;

    // java  GridCol getColByOffset(float var1)
    fn getColByOffset(&self, var1: f32) -> &(dyn GridCol);

    // java  int indexOf(GridCol var1)
    fn indexOf(&mut self, var1: &dyn GridCol) -> i32;

    // java  TableInfo getTableInfo()
    fn getTableInfo(&self) -> &dyn TableInfo;

    // java  int getGridBefore(int var1)
    fn getGridBefore(&self, var1: i32) -> i32;

    // java  int getGridAfter(int var1)
    fn getGridAfter(&self, var1: i32) -> i32;

    // java  int getWidthBefore(int var1, boolean var2)
    fn getWidthBefore(&self, var1: i32, var2: bool) -> i32;

    // java  int getWidthAfter(int var1, boolean var2)
    fn getWidthAfter(&self, var1: i32, var2: bool) -> i32;

    // java  int getTotalGridWidth()
    fn getTotalGridWidth(&self) -> i32;

    // java  int getTotalOriWidth()
    fn getTotalOriWidth(&self) -> i32;

    // java  GridAutoSize getAutoSize()
    fn getAutoSize(&self) -> &(dyn GridAutoSize);

    // java  void invalid()
    fn invalid(&mut self);

}
