// Java field: int RUBY_ALIGN_CENTER
pub(crate) const RUBY_ALIGN_CENTER: i32 = -1364013995;
// Java field: int RUBY_ALIGN_DISTRIBUTE_LETTER
pub(crate) const RUBY_ALIGN_DISTRIBUTE_LETTER: i32 = -1063614105;
// Java field: int RUBY_ALIGN_DISTRIBUTE_SPACE
pub(crate) const RUBY_ALIGN_DISTRIBUTE_SPACE: i32 = -997367899;
// Java field: int RUBY_ALIGN_LEFT
pub(crate) const RUBY_ALIGN_LEFT: i32 = 3317767;
// Java field: int RUBY_ALIGN_RIGHT
pub(crate) const RUBY_ALIGN_RIGHT: i32 = 108511772;
// Java field: int RUBY_ALIGN_RIGHT_VERTICAL
pub(crate) const RUBY_ALIGN_RIGHT_VERTICAL: i32 = 1084797362;

// Java: public class RubyPr
pub struct RubyPr {
    // Java field: Integer mTmpRubyAlign
    pub mTmpRubyAlign: Option<i32>,
    // Java field: Integer mTmpHpsRaise
    pub mTmpHpsRaise: Option<i32>,
    // Java field: String mTmpHps
    pub mTmpHps: Option<String>,
}

// Java: public class RubyPr
impl RubyPr {
    // Java: RubyPr()
    pub fn new() -> Self {
        // Java: super();
        let mut pr = RubyPr {
            mTmpRubyAlign: None,
            mTmpHpsRaise: None,
            mTmpHps: None,
        };
        // Java: this.reset();
        pr.reset();
        pr
    }

    // Java: void reset()
    pub fn reset(&mut self) {
        // Java: this.mTmpRubyAlign = null;
        self.mTmpRubyAlign = None;
        // Java: this.mTmpHpsRaise = null;
        self.mTmpHpsRaise = None;
        // Java: this.mTmpHps = null;
        self.mTmpHps = None;
    }
}
