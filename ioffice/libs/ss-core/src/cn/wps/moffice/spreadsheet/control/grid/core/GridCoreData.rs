// java package cn.wps.moffice.spreadsheet.control.grid.core;
// java
// java import cn.wps.moss.app.IGridSheet;
// java
// java public abstract class GridCoreData {
// java public void reset(IGridSheet gridSheet) {
// java this.resetSheet(gridSheet);
// java this.refreshData();
// java }
// java
// java protected abstract void resetSheet(IGridSheet var1);
// java
// java protected abstract void refreshData();
// java }
use crate::{cn::wps::moss::app::IGridSheet::IGridSheet, java_runtime::Rc};
pub trait GridCoreData {
    fn resetSheet(&mut self, g: Rc<dyn IGridSheet>);
    fn refreshData(&mut self);
    fn reset(&mut self, g: Rc<dyn IGridSheet>) {
        self.resetSheet(g);
        self.refreshData();
    }
}
