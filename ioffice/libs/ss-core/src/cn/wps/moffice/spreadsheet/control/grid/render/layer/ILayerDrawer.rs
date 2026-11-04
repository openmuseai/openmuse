use crate::cn::wps::moffice::spreadsheet::control::grid;
use crate::java_runtime::{Cell, Rc, RefCell, Weak};
use grid::{
    core::{GridRange::GridRange, GridSheetLayoutInfo::GridSheetLayoutInfo},
    render::{
        data::GridClientData::{ClientRect, ClientRects, GridClientData},
        graphics::{Canvas, Paint, Path, Rect},
    },
};
// java package cn.wps.moffice.spreadsheet.control.grid.render.layer;
// java
// java import android.graphics.Canvas;
// java import android.graphics.Paint;
// java import cn.wps.moffice.spreadsheet.control.grid.core.GridRange;
// java import cn.wps.moffice.spreadsheet.control.grid.core.GridSheetLayoutInfo;
// java
// java public interface ILayerDrawer {
// java boolean draw(Canvas var1, Paint var2, GridRange var3, GridSheetLayoutInfo var4);
// java
// java void destroy();
// java }
pub trait ILayerDrawer {
    fn draw(
        &mut self,
        c: &mut Canvas,
        p: &mut Paint,
        r: &GridRange,
        l: &GridSheetLayoutInfo,
    ) -> bool;
    fn destroy(&mut self);
    fn className(&self) -> &'static str;
}
