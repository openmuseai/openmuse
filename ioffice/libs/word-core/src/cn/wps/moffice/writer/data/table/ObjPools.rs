// Rust stub for ObjPools (auto-generated)
// Original: cn/wps/moffice/writer/data/table/ObjPools.java
use crate::*;

pub struct ObjPools {
    // TODO: fields
}

impl ObjPools {
    pub fn new() -> Self { todo!() }
    pub fn getInstance() -> ObjPools { todo!() }
    pub fn obtainGridCell(&self) -> GridCellImpl { todo!() }
    pub fn recycleGridCell(&self, cell: GridCellImpl) { todo!() }
    pub fn obtainGridCol(&self) -> GridColImpl { todo!() }
    pub fn recycleGridCol(&self, col: GridColImpl) { todo!() }
    pub fn recycleTableRow(&self, row: TableManagerUtil::KTableRow) { todo!() }
    pub fn recycleTableCell(&self, cell: TableManagerUtil::KTableCell) { todo!() }
}
