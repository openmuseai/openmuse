// Java: public class ColorFilter
pub struct ColorFilter {
    // Java field: Integer cellXFRef
    pub cellXFRef: Option<i32>,
    // Java field: boolean isApply
    pub isApply: bool,
}

// Java: public class ColorFilter
impl ColorFilter {
    // Java: ColorFilter()
    pub fn ColorFilter() -> Self {
      // Java: super();
      ColorFilter {
          cellXFRef: None,
          isApply: true,
      }
    }

}
