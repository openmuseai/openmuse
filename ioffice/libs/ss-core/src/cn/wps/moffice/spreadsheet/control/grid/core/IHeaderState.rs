// java package cn.wps.moffice.spreadsheet.control.grid.core;
// java
// java public interface IHeaderState {
// java float getHeaderFactor();
// java
// java boolean isNormalHeader();
// java
// java boolean isDrawHeader();
// java }
pub trait IHeaderState {
    fn getHeaderFactor(&self) -> f32;
    fn isNormalHeader(&self) -> bool;
    fn isDrawHeader(&self) -> bool;
}
