// java package cn.wps.moffice.spreadsheet.control.grid;
// java
// java public class EmptyTableStyle extends TableStyle {
// java private static TableStyle instance = new EmptyTableStyle();
// java
// java private EmptyTableStyle() {
// java }
// java
// java public static TableStyle getInstance() {
// java return instance;
// java }
// java
// java public Integer borderColor(int rowIndex, int colIndex) {
// java return null;
// java }
// java
// java public Integer fillColor(int rowIndex, int colIndex) {
// java return null;
// java }
// java
// java public TableStyle.FontStyle fontStyle(int rowIdx, int colIdx) {
// java return null;
// java }
// java }
use super::TableStyle::{FontStyle, TableStyle};
pub struct EmptyTableStyle;
impl EmptyTableStyle {
    pub fn getInstance() -> &'static Self {
        static INSTANCE: EmptyTableStyle = EmptyTableStyle;
        &INSTANCE
    }
}
impl TableStyle for EmptyTableStyle {
    fn borderColor(&self, _r: i32, _c: i32) -> Option<i32> {
        None
    }
    fn fillColor(&self, _r: i32, _c: i32) -> Option<i32> {
        None
    }
    fn fontStyle(&self, _r: i32, _c: i32) -> Option<&dyn FontStyle> {
        None
    }
}
