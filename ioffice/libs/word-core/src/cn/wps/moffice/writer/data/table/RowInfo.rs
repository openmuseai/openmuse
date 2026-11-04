// Java: import cn.wps.moffice.property.PropertySet
use util::moffice::property::PropertySet::*;
// Java: import cn.wps.moffice.writer.data.table.TableInfo
use crate::cn::wps::moffice::writer::data::table::TableInfo::*;
// Java: import cn.wps.moffice.writer.data.table.CellInfo
use crate::cn::wps::moffice::writer::data::table::CellInfo::*;
// Java: import cn.wps.moffice.writer.data.table.Data
use crate::cn::wps::moffice::writer::data::table::Data::*;

// java public interface RowInfo
pub trait RowInfo {
    // java  TableInfo getParent()
    fn getParent(&self) -> &dyn TableInfo;

    // java  int getIndex()
    fn getIndex(&self) -> i32;

    // java  long getRange()
    fn getRange(&self) -> i64;

    // java  int getStartCP()
    fn getStartCP(&self) -> i32;

    // java  int getEndCP()
    fn getEndCP(&self) -> i32;

    // java  RowInfo getNextRow()
    fn getNextRow(&self) -> Option<&dyn RowInfo>;

    // java  RowInfo getPrevRow()
    fn getPrevRow(&self) -> Option<&dyn RowInfo>;

    // java  int getRowHeight()
    fn getRowHeight(&mut self) -> i32;

    // java  int getRgdxaCount()
    fn getRgdxaCount(&mut self) -> i32;

    // java  int getRgdxa(int var1)
    fn getRgdxa(&mut self, var1: i32) -> i32;

    // java  int size()
    fn size(&self) -> i32;

    // java  int getCellWidth(int var1)
    fn getCellWidth(&mut self, var1: i32) -> i32;

    // java  CellInfo getCellByIndex(int var1)
    fn getCellByIndex(&self, var1: i32) -> Option<&dyn CellInfo>;

    // java  CellInfo getCellByCP(int var1)
    fn getCellByCP(&self, var1: i32) -> Option<&dyn CellInfo>;

    // java  boolean cantSplit()
    fn cantSplit(&mut self) -> bool;

    // java  PropertySet getRowProperty()
    fn getRowProperty(&self) -> &PropertySet;

    // java  boolean isRevisionDelete()
    fn isRevisionDelete(&mut self) -> bool;

    // java  boolean isRevisionInsert()
    fn isRevisionInsert(&mut self) -> bool;

    // java  boolean isFormatChange()
    fn isFormatChange(&mut self) -> bool;

    // java  PropertySet getOriginalProperty()
    fn getOriginalProperty(&self) -> &PropertySet;

    // java  PropertySet getFinalProperty()
    fn getFinalProperty(&self) -> &PropertySet;

    // java  Data.RowData getData()
    fn getData(&mut self) -> &mut crate::cn::wps::moffice::writer::data::table::Data::RowData;

}
