// Java field: int textanchorTop
pub(crate) const textanchorTop: i32 = 0;
// Java field: int textanchorMiddle
pub(crate) const textanchorMiddle: i32 = 1;
// Java field: int textanchorBottom
pub(crate) const textanchorBottom: i32 = 2;
// Java field: int textanchorTopCentered
pub(crate) const textanchorTopCentered: i32 = 3;
// Java field: int textanchorMiddleCentered
pub(crate) const textanchorMiddleCentered: i32 = 4;
// Java field: int textanchorBottomCentered
pub(crate) const textanchorBottomCentered: i32 = 5;
// Java field: int textanchorTopBaseline
pub(crate) const textanchorTopBaseline: i32 = 6;
// Java field: int textanchorBottomBaseline
pub(crate) const textanchorBottomBaseline: i32 = 7;
// Java field: int textanchorTopCenteredBaseline
pub(crate) const textanchorTopCenteredBaseline: i32 = 8;
// Java field: int textanchorBottomCenteredBaseline
pub(crate) const textanchorBottomCenteredBaseline: i32 = 9;

// Java: public class TextAnchorV
pub struct TextAnchorV {
}

// Java: public class TextAnchorV
impl TextAnchorV {
    // Java: TextAnchorV()
    pub fn new() -> Self {
      // Java: super();
      TextAnchorV {}
    }

    // Java: String getNameByID(int id)
    pub fn getNameByID(&self, id: i32) -> String {
      // Java: return "top";
      "top".to_string()
    }

    // Java: int getIDByName(String name)
    pub fn getIDByName(&self, name: &str) -> i32 {
      // Java: return 0;
      0
    }

}
