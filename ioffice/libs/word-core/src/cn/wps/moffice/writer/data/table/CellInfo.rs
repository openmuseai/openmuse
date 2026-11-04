// Java: import cn.wps.moffice.drawing.graphics.SHD
use drawing::cn::wps::moffice::drawing::graphics::SHD::*;
// Java: import cn.wps.moffice.writer.data.value.BRC
use crate::cn::wps::moffice::writer::data::value::BRC::*;
// Java: import cn.wps.moffice.writer.data.table.RowInfo
use crate::cn::wps::moffice::writer::data::table::RowInfo::*;
// Java: import cn.wps.moffice.writer.data.table.TableInfo
use crate::cn::wps::moffice::writer::data::table::TableInfo::*;
// Java: import cn.wps.moffice.writer.data.table.Data
use crate::cn::wps::moffice::writer::data::table::Data::*;

// java public interface CellInfo
pub trait CellInfo {
    // java  int size()
    fn size(&self) -> i32;

    // java  int getIndex()
    fn getIndex(&self) -> i32;

    // java  long getRange()
    fn getRange(&self) -> i64;

    // java  int getStartCP()
    fn getStartCP(&self) -> i32;

    // java  int getEndCP()
    fn getEndCP(&self) -> i32;

    // java  CellInfo getNextCell()
    fn getNextCell(&self) -> Option<&dyn CellInfo>;

    // java  CellInfo getPrevCell()
    fn getPrevCell(&self) -> Option<&dyn CellInfo>;

    // java  RowInfo getParent()
    fn getParent(&self) -> &dyn RowInfo;

    // java  TableInfo getTableByIndex(int var1)
    fn getTableByIndex(&self, var1: i32) -> Option<&dyn TableInfo>;

    // java  TableInfo getTableByCP(int var1)
    fn getTableByCP(&self, var1: i32) -> Option<&dyn TableInfo>;

    // java  CellInfo getPrevVertMergeCell()
    fn getPrevVertMergeCell(&mut self) -> Option<&dyn CellInfo>;

    // java  CellInfo getNextVertMergeCell()
    fn getNextVertMergeCell(&mut self) -> Option<&dyn CellInfo>;

    // java  boolean isVertMergeCell()
    fn isVertMergeCell(&mut self) -> bool;

    // java  boolean isFirstVertMergeCell()
    fn isFirstVertMergeCell(&mut self) -> bool;

    // java  boolean isLastVertMergeCell()
    fn isLastVertMergeCell(&mut self) -> bool;

    // java  int getHoriMerge()
    fn getHoriMerge(&mut self) -> i32;

    // java  int getVertMerge()
    fn getVertMerge(&mut self) -> i32;

    // java  int getVertAlign()
    fn getVertAlign(&mut self) -> i32;

    // java  int getTextFlow()
    fn getTextFlow(&mut self) -> i32;

    // java  boolean getFitText()
    fn getFitText(&mut self) -> bool;

    // java  boolean getNoWrap()
    fn getNoWrap(&mut self) -> bool;

    // java  int getRgdxaLeft()
    fn getRgdxaLeft(&mut self) -> i32;

    // java  int getRgdxaRight()
    fn getRgdxaRight(&mut self) -> i32;

    // java  int getWidth()
    fn getWidth(&mut self) -> i32;

    // java  SHD getSHD()
    fn getSHD(&mut self) -> Option<&SHD>;

    // java  int getMarginTop()
    fn getMarginTop(&mut self) -> i32;

    // java  int getMarginLeft()
    fn getMarginLeft(&mut self) -> i32;

    // java  int getMarginBottom()
    fn getMarginBottom(&mut self) -> i32;

    // java  int getMarginRight()
    fn getMarginRight(&mut self) -> i32;

    // java  BRC getBrcTop()
    fn getBrcTop(&mut self) -> Option<&BRC>;

    // java  BRC getBrcLeft()
    fn getBrcLeft(&mut self) -> Option<&BRC>;

    // java  BRC getBrcBottom()
    fn getBrcBottom(&mut self) -> Option<&BRC>;

    // java  BRC getBrcRight()
    fn getBrcRight(&mut self) -> Option<&BRC>;

    // java  BRC getBrcFromTopLeftToBottomRight()
    fn getBrcFromTopLeftToBottomRight(&mut self) -> Option<&BRC>;

    // java  BRC getBrcFromTopRightToBottomLeft()
    fn getBrcFromTopRightToBottomLeft(&mut self) -> Option<&BRC>;

    // java  boolean isAutoWidth()
    fn isAutoWidth(&mut self) -> bool;

    // java  Data.CellData getData()
    fn getData(&mut self) -> &mut crate::cn::wps::moffice::writer::data::table::Data::CellData;

}
