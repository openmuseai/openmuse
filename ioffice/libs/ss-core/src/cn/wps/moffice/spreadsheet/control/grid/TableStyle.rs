// java package cn.wps.moffice.spreadsheet.control.grid;
// java
// java import cn.wps.moffice.spreadsheet.control.grid.layout.data.SpanInfo;
// java
// java public abstract class TableStyle {
// java public static final boolean isAutomaticColor(Integer color) {
// java return color == null || (-16777216 & color) == 0;
// java }
// java
// java public abstract Integer borderColor(int var1, int var2);
// java
// java public abstract Integer fillColor(int var1, int var2);
// java
// java public abstract FontStyle fontStyle(int var1, int var2);
// java
// java public interface FontStyle {
// java boolean isEmpty();
// java
// java void replace(SpanInfo var1);
// java }
// java }
use super::layout::data::SpanInfo::SpanInfo;
pub trait FontStyle {
    fn isEmpty(&self) -> bool;
    fn replace(&self, s: &mut SpanInfo);
}
pub trait TableStyle {
    fn borderColor(&self, row: i32, col: i32) -> Option<i32>;
    fn fillColor(&self, row: i32, col: i32) -> Option<i32>;
    fn fontStyle(&self, row: i32, col: i32) -> Option<&dyn FontStyle>;
}
pub fn isAutomaticColor(c: Option<i32>) -> bool {
    c.is_none_or(|c| c & -16777216 == 0)
}
