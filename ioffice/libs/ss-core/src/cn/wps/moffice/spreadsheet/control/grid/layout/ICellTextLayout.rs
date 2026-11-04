// java package cn.wps.moffice.spreadsheet.control.grid.layout;
// java
// java import android.graphics.Canvas;
// java import android.graphics.Rect;
// java import cn.wps.moffice.spreadsheet.control.grid.core.GridSheetLayoutInfo;
// java import cn.wps.moffice.spreadsheet.control.grid.layout.data.CellTextData;
// java import cn.wps.text.layout.ITextEdit;
// java
// java public interface ICellTextLayout extends ITextEdit {
// java void typo(GridSheetLayoutInfo var1, CellTextData var2, int var3, int var4, boolean var5);
// java
// java Rect getTextRect();
// java
// java void renderer(Canvas var1);
// java
// java void clear();
// java }
use super::super::{
    core::GridSheetLayoutInfo::GridSheetLayoutInfo,
    render::graphics::{Canvas, PointF, Rect},
};
use super::data::CellTextData::CellTextData;
pub trait ICellTextLayout {
    fn typo(
        &mut self,
        l: &GridSheetLayoutInfo,
        data: &mut CellTextData,
        w: i32,
        h: i32,
        force: bool,
    );
    fn getTextRect(&self) -> Rect;
    fn getTypoOffset(&self) -> PointF;
    fn renderer(&mut self, c: &mut Canvas);
    fn clear(&mut self);
    fn className(&self) -> &'static str;
}
