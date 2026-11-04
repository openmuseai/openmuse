// Java: public class ColorInfo
pub struct ColorInfo {
    // Java field: String themeColorRef
    pub themeColorRef: Option<String>,
    // Java field: String hexColor
    pub hexColor: Option<String>,
}

// Java: public class ColorInfo
impl ColorInfo {
    // Java: ColorInfo()
    pub fn ColorInfo() -> Self {
      // Java: super();
      ColorInfo {
          themeColorRef: None,
          hexColor: None,
      }
    }

}
