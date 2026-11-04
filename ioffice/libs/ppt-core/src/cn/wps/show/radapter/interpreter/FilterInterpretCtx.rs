#![allow(non_snake_case, non_upper_case_globals, dead_code)]

// Java field: int FILTER_NONE
pub(crate) const FILTER_NONE: i32 = 0;
// Java field: int FILTER_MASK
pub(crate) const FILTER_MASK: i32 = 4096;
// Java field: int FILTER_BLIP_FILL
pub(crate) const FILTER_BLIP_FILL: i32 = 4097;
// Java field: int FILTER_PATTERN_FILL
pub(crate) const FILTER_PATTERN_FILL: i32 = 4098;

// Java: public class FilterInterpretCtx
pub struct FilterInterpretCtx {
    // Java field: int mInterpretFlag
    pub(crate) mInterpretFlag: i32,
}

// Java: public class FilterInterpretCtx
impl FilterInterpretCtx {
    pub fn new() -> Self {
        FilterInterpretCtx { mInterpretFlag: 0 }
    }

    // Java: void reset()
    pub fn reset(&mut self) {
      // Java: this.mInterpretFlag = 0;
      self.mInterpretFlag = 0;
    }

    // Java: boolean hasFilter()
    pub fn hasFilter(&self) -> bool {
      // Java: return this.mInterpretFlag != 0;
      self.mInterpretFlag != 0
    }

    // Java: void setFilterFlag(int flag)
    pub fn setFilterFlag(&mut self, flag: i32) {
      // Java: this.mInterpretFlag |= flag;
      self.mInterpretFlag |= flag;
    }

    // Java: boolean isFilterBlipFill()
    pub fn isFilterBlipFill(&self) -> bool {
      // Java: return (this.mInterpretFlag & 4097) != 0;
      (self.mInterpretFlag & 4097) != 0
    }

    // Java: boolean isFilterPatternFill()
    pub fn isFilterPatternFill(&self) -> bool {
      // Java: return (this.mInterpretFlag & 4098) != 0;
      (self.mInterpretFlag & 4098) != 0
    }

}
