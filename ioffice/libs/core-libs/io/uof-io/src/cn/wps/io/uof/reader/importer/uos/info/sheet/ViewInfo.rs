// Java: public class ViewInfo
pub struct ViewInfo {
    // Java field: boolean isSelected
    pub isSelected: bool,
    // Java field: Integer topCell
    pub topCell: Option<i32>,
    // Java field: Integer leftCell
    pub leftCell: Option<i32>,
    // Java field: String view
    pub view: Option<String>,
    // Java field: boolean showGridLines
    pub showGridLines: bool,
    // Java field: boolean showFormulas
    pub showFormulas: bool,
    // Java field: Double sheetScale
    pub sheetScale: Option<f64>,
    // Java field: Double scale
    pub scale: Option<f64>,
    // Java field: String sqref
    pub sqref: Option<String>,
    // Java field: boolean showRowColHeaders
    pub showRowColHeaders: bool,
    // Java field: boolean showZeros
    pub showZeros: bool,
    // Java field: boolean showOutlineSymbols
    pub showOutlineSymbols: bool,
    // Java field: Integer activeRow
    pub activeRow: Option<i32>,
    // Java field: Integer activeCol
    pub activeCol: Option<i32>,
}

// Java: public class ViewInfo
impl ViewInfo {
    // Java: ViewInfo()
    pub fn ViewInfo() -> Self {
      // Java: super();
      ViewInfo {
          isSelected: false,
          topCell: None,
          leftCell: None,
          view: None,
          showGridLines: true,
          showFormulas: false,
          sheetScale: None,
          scale: None,
          sqref: None,
          showRowColHeaders: true,
          showZeros: true,
          showOutlineSymbols: true,
          activeRow: None,
          activeCol: None,
      }
    }

}
