// Java: import cn.wps.io.uof.reader.importer.uos.info.ProtectPasswordInfo
use crate::cn::wps::io::uof::reader::importer::uos::info::ProtectPasswordInfo::ProtectPasswordInfo;

// Java: public class ProtectionInfo
pub struct ProtectionInfo {
    // Java field: ProtectPasswordInfo passwordInfo
    pub passwordInfo: ProtectPasswordInfo,
    // Java field: boolean isProtect
    pub isProtect: bool,
    // Java field: boolean selectLockedCells
    pub selectLockedCells: bool,
    // Java field: boolean selectUnlockedCells
    pub selectUnlockedCells: bool,
    // Java field: boolean formatCells
    pub formatCells: bool,
    // Java field: boolean formatColumns
    pub formatColumns: bool,
    // Java field: boolean formatRows
    pub formatRows: bool,
    // Java field: boolean insertColumns
    pub insertColumns: bool,
    // Java field: boolean insertRows
    pub insertRows: bool,
    // Java field: boolean insertHyperlinks
    pub insertHyperlinks: bool,
    // Java field: boolean deleteColumns
    pub deleteColumns: bool,
    // Java field: boolean deleteRows
    pub deleteRows: bool,
    // Java field: boolean sort
    pub sort: bool,
    // Java field: boolean autoFilter
    pub autoFilter: bool,
    // Java field: boolean pivotTables
    pub pivotTables: bool,
    // Java field: boolean objects
    pub objects: bool,
    // Java field: boolean scenarios
    pub scenarios: bool,
}

// Java: public class ProtectionInfo
impl ProtectionInfo {
    // Java: ProtectionInfo()
    pub fn ProtectionInfo() -> Self {
      // Java: super();
      ProtectionInfo {
          passwordInfo: ProtectPasswordInfo::ProtectPasswordInfo(),
          isProtect: false,
          selectLockedCells: false,
          selectUnlockedCells: false,
          formatCells: true,
          formatColumns: true,
          formatRows: true,
          insertColumns: true,
          insertRows: true,
          insertHyperlinks: true,
          deleteColumns: true,
          deleteRows: true,
          sort: true,
          autoFilter: true,
          pivotTables: true,
          objects: false,
          scenarios: false,
      }
    }

}
