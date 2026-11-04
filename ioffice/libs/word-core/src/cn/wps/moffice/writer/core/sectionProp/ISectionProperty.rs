// Java: import cn.wps.moffice.writer.core.KRange
use crate::cn::wps::moffice::writer::core::KRange::*;
// Java: import cn.wps.moffice.writer.core.SelectionType
use crate::cn::wps::moffice::writer::core::SelectionType::*;
// Java: import cn.wps.moffice.writer.data.value.DxaColumn
use crate::cn::wps::moffice::writer::data::value::DxaColumn::*;
// Java: import cn.wps.moffice.writer.data.value.PageSetup
use crate::cn::wps::moffice::writer::data::value::PageSetup::*;

// java public interface ISectionProperty
pub trait ISectionProperty {
    // java  PageSetup getPageSetup(int var1)
    fn getPageSetup(&self, var1: i32) -> &PageSetup;

    // java  void setPageSetup(int var1, PageSetup var2)
    fn setPageSetup(&mut self, var1: i32, var2: &PageSetup);

    // java  Integer getOrientation(int var1)
    fn getOrientation(&self, var1: i32) -> i32;

    // java  DxaColumn[] getDxaColumns(int var1)
    fn getDxaColumns(&self, var1: i32) -> Vec<DxaColumn>;

    // java  float getMinFixedColumnWdith(int var1)
    fn getMinFixedColumnWdith(&self, var1: i32) -> f32;

    // java  int getTextFlow(int var1)
    fn getTextFlow(&self, var1: i32) -> i32;

    // java  boolean setFixedDxaColumns(SelectionType var1, KRange var2, int var3)
    fn setFixedDxaColumns__selectiontype_krange_int(&mut self, var1: &SelectionType, var2: &KRange, var3: i32) -> bool;

    // java  boolean setFixedDxaColumns(SelectionType var1, KRange var2, boolean var3)
    fn setFixedDxaColumns__selectiontype_krange_boolean(&mut self, var1: &SelectionType, var2: &KRange, var3: bool) -> bool;

    // java  void beginSectionChange()
    fn beginSectionChange(&mut self);

    // java  void endSectionChange()
    fn endSectionChange(&mut self);

}
