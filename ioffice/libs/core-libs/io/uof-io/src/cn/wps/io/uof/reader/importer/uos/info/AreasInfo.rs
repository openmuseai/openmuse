// Java: import java.util.ArrayList
// Java: import java.util.List

// Java: public class AreasInfo
pub struct AreasInfo {
    // Java field: List<String> areas
    pub areas: Vec<String>,
}

// Java: public class AreasInfo
impl AreasInfo {
    // Java: AreasInfo()
    pub fn AreasInfo() -> Self {
      // Java: super();
      AreasInfo {
          areas: Vec::new(),
      }
    }

}
