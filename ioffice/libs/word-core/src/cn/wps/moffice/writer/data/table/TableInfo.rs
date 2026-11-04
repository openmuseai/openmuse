// Java: import cn.wps.moffice.drawing.IPos
use drawing::cn::wps::moffice::drawing::IPos::*;
// Java: import cn.wps.moffice.drawing.graphics.SHD
use drawing::cn::wps::moffice::drawing::graphics::SHD::*;
// Java: import cn.wps.moffice.property.PropertySet
use util::moffice::property::PropertySet::*;
// Java: import cn.wps.moffice.writer.data.table.grid.GridCols
use crate::cn::wps::moffice::writer::data::table::grid::GridCols::*;
// Java: import cn.wps.moffice.writer.data.value.BRC
use crate::cn::wps::moffice::writer::data::value::BRC::*;
// Java: import cn.wps.moffice.writer.data.value.TableBorders
use crate::cn::wps::moffice::writer::data::value::TableBorders::*;
// Java: import cn.wps.moffice.writer.data.value.TableLook
use crate::cn::wps::moffice::writer::data::value::TableLook::*;
// Java: import cn.wps.moffice.writer.data.table.RowInfo
use crate::cn::wps::moffice::writer::data::table::RowInfo::*;
// Java: import cn.wps.moffice.writer.data.table.CellInfo
use crate::cn::wps::moffice::writer::data::table::CellInfo::*;
// Java: import cn.wps.moffice.writer.data.table.TableJCData
use crate::cn::wps::moffice::writer::data::table::TableJCData::*;
// Java: import cn.wps.moffice.writer.data.table.Data
use crate::cn::wps::moffice::writer::data::table::Data::*;

// java public interface TableInfo
pub trait TableInfo {
    // java  int getLevel()
    fn getLevel(&self) -> i32;

    // java  long getRange()
    fn getRange(&self) -> i64;

    // java  int getStartCP()
    fn getStartCP(&self) -> i32;

    // java  int getEndCP()
    fn getEndCP(&self) -> i32;

    // java  int size()
    fn size(&self) -> i32;

    // java  RowInfo getRowByIndex(int var1)
    fn getRowByIndex(&self, var1: i32) -> Box<dyn RowInfo>;

    // java  RowInfo getRowByCP(int var1)
    fn getRowByCP(&self, var1: i32) -> Box<dyn RowInfo>;

    // java  CellInfo getParentCell()
    fn getParentCell(&self) -> Box<dyn CellInfo>;

    // java  TableJCData getJC()
    fn getJC(&self) -> TableJCData;

    // java  GridCols getGrid()
    fn getGrid(&self) -> Box<dyn GridCols>;

    // java  void requestAutoFitDelay()
    fn requestAutoFitDelay(&mut self);

    // java  IPos getPos()
    fn getPos(&mut self) -> Box<dyn IPos>;

    // java  SHD getSHD()
    fn getSHD(&mut self) -> SHD;

    // java  TableLook getTableLook()
    fn getTableLook(&mut self) -> TableLook;

    // java  TableBorders getTableBorders()
    fn getTableBorders(&mut self) -> TableBorders;

    // java  int getStyleId()
    fn getStyleId(&self) -> i32;

    // java  BRC getBrcTop()
    fn getBrcTop(&mut self) -> BRC;

    // java  BRC getBrcLeft()
    fn getBrcLeft(&mut self) -> BRC;

    // java  BRC getBrcBottom()
    fn getBrcBottom(&mut self) -> BRC;

    // java  BRC getBrcRight()
    fn getBrcRight(&mut self) -> BRC;

    // java  boolean isRTL()
    fn isRTL(&self) -> bool;

    // java  boolean isComplete()
    fn isComplete(&self) -> bool;

    // java  boolean isTableFit()
    fn isTableFit(&self) -> bool;

    // java  PropertySet getTableProperty()
    fn getTableProperty(&self) -> &PropertySet;

    // java  Data.TableData getData()
    fn getData(&mut self) -> &mut crate::cn::wps::moffice::writer::data::table::Data::TableData;

    // java  void invalid()
    fn invalid(&mut self);

}
