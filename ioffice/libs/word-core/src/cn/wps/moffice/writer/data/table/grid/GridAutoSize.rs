// Java: import cn.wps.moffice.writer.data.table.grid.GridSizeCommit
use crate::cn::wps::moffice::writer::data::table::grid::GridSizeCommit::*;

// java public interface GridAutoSize
pub trait GridAutoSize {
    // java  boolean autoFitSize(int var1)
    fn autoFitSize(&mut self, var1: i32) -> bool;

    // java  void calColFitInfo()
    fn calColFitInfo(&mut self);

    // java  void doFit(int var1)
    fn doFit(&mut self, var1: i32);

    // java  void applyAutoFitChange()
    fn applyAutoFitChange(&mut self);

    // java  int getTotalMinWidth()
    fn getTotalMinWidth(&self) -> i32;

    // java  int getTotalBestWidth()
    fn getTotalBestWidth(&self) -> i32;

    // java  int getParaTotalMax()
    fn getParaTotalMax(&self) -> i32;

    // java  int getTotalFitWidth()
    fn getTotalFitWidth(&self) -> i32;

    // java  boolean checkFitChanged()
    fn checkFitChanged(&mut self) -> bool;

    // java  boolean checkHaveExtrusion()
    fn checkHaveExtrusion(&mut self) -> bool;

    // java  boolean isForceAdapted()
    fn isForceAdapted(&self) -> bool;

}
