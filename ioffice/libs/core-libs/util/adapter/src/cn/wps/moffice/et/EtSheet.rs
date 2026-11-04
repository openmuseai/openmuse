// Java: import java.util.ArrayList
// Java: import java.util.List

use super::EtCell::EtCell;
use super::EtRowInfo::EtRowInfo;
use super::EtColInfo::EtColInfo;
use super::EtMerge::EtMerge;

// Java: public class EtSheet
pub struct EtSheet {
    // Java field: int rows
    pub(crate) rows: i32,
    // Java field: int cols
    pub(crate) cols: i32,
    // Java field: List<EtCell> cells
    pub(crate) cells: Vec<EtCell>,
    // Java field: List<EtRowInfo> rowInfos
    pub(crate) rowInfos: Vec<EtRowInfo>,
    // Java field: List<EtColInfo> colInfos
    pub(crate) colInfos: Vec<EtColInfo>,
    // Java field: List<EtMerge> merges
    pub(crate) merges: Vec<EtMerge>,
}

// Java: public class EtSheet
impl EtSheet {
    // Java: EtSheet()
    pub fn new() -> Self {
      // Java: super();
      EtSheet {
          rows: 0,
          cols: 0,
          cells: Vec::new(),
          rowInfos: Vec::new(),
          colInfos: Vec::new(),
          merges: Vec::new(),
      }
    }

}
