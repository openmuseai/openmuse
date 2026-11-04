// java public class ParamScope
#[derive(Default)]
pub struct ParamScope {
    // java public int mBegin;
    pub(crate) mBegin: i32,
    // java public int mEnd;
    pub(crate) mEnd: i32,
    // java public String mStr;
    pub(crate) mStr: Option<String>,
}

// java public class ParamScope
impl ParamScope {
    // java public  ParamScope(String string, int nStartPos, int nCurPos)
    pub fn ParamScope(string: Option<&str>, nStartPos: i32, nCurPos: i32) -> Self {
      // java this.mBegin = nStartPos;
      Self {
        mBegin: nStartPos,
        mEnd: nCurPos,
        mStr: string.map(|s| s.to_string()),
      }
      // java this.mEnd = nCurPos;
      // java this.mStr = string;
    }

}
