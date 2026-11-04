// java package cn.wps.moffice.spreadsheet.control.grid.render.data;
// java
// java import cn.wps.moffice.spreadsheet.control.grid.core.GridRange;
// java import cn.wps.moss.util.KmoRect;
// java import java.util.List;
// java
// java public interface IRoGridClientData {
// java int getGridXOffset();
// java
// java int getGridYOffset();
// java
// java int getClientsXOffset();
// java
// java int getClientsYOffset();
// java
// java int getCellsOffsetX();
// java
// java int getCellsOffsetY();
// java
// java int getValidCurClientCount();
// java
// java boolean isOnlyOneClient();
// java
// java GridClientData.ClientRect[] getClientRects();
// java
// java GridClientData.ClientRect getMainClient();
// java
// java List<GridRange> getClientsRange();
// java
// java List<KmoRect> getClientRangeRect();
// java }
pub trait IRoGridClientData {
    fn getGridXOffset(&self) -> i32;
    fn getGridYOffset(&self) -> i32;
    fn getClientsXOffset(&self) -> i32;
    fn getClientsYOffset(&self) -> i32;
    fn getCellsOffsetX(&self) -> i32;
    fn getCellsOffsetY(&self) -> i32;
    fn getMainClient(
        &self,
    ) -> crate::java_runtime::Rc<crate::java_runtime::RefCell<super::GridClientData::ClientRect>>;
    fn getClientRects(&self) -> super::GridClientData::ClientRects;
    fn getValidCurClientCount(&self) -> i32;
    fn isOnlyOneClient(&self) -> bool;
    fn getClientsRange(&self) -> Vec<super::super::super::core::GridRange::GridRange>;
    fn getClientRangeRect(&self) -> Vec<crate::cn::wps::moss::util::KmoRect::KmoRect>;
}
