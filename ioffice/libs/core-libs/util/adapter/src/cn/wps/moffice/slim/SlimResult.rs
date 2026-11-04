// Java: public class SlimResult
pub struct SlimResult {
    // Java field: int mType
    pub(crate) mType: i32,
    // Java field: long mSlimSize
    pub(crate) mSlimSize: i64,
}

// Java: public class SlimResult
impl SlimResult {
    // Java: SlimResult(int type)
    pub fn new(_type: i32) -> Self {
      // Java: super();
      // Java: this.mType = type;
      // Java: this.mSlimSize = 0L;
      SlimResult {
          mType: _type,
          mSlimSize: 0i64,
      }
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
      // Java: StringBuilder sb = new StringBuilder();
      let mut sb = String::new();
      // Java: sb.append("SlimResult: \n");
      sb.push_str("SlimResult: \n");
      // Java: sb.append("[type: ]" + this.mType);
      sb.push_str(&format!("[type: ]{}", self.mType));
      // Java: sb.append("[slimSize: ]" + this.mSlimSize);
      sb.push_str(&format!("[slimSize: ]{}", self.mSlimSize));
      // Java: return sb.toString();
      sb
    }

}
