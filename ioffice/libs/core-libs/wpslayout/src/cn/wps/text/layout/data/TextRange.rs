// Java: public class TextRange
pub struct TextRange {
    // Java field: int mStart
    pub(crate) mStart: i32,
    // Java field: int mEnd
    pub(crate) mEnd: i32,
}

// Java: public class TextRange
impl TextRange {
    // Java: TextRange()
    pub fn new() -> Self {
      // Java: super();
      TextRange { mStart: 0, mEnd: 0 }
    }

    // Java: void putStart(int start)
    pub fn putStart(&mut self, start: i32) {
      // Java: this.mStart = start;
      self.mStart = start;
    }

    // Java: void putEnd(int end)
    pub fn putEnd(&mut self, end: i32) {
      // Java: this.mEnd = end;
      self.mEnd = end;
    }

    // Java: int start()
    pub fn start(&self) -> i32 {
      // Java: return this.mStart;
      self.mStart
    }

    // Java: int end()
    pub fn end(&self) -> i32 {
      // Java: return this.mEnd;
      self.mEnd
    }

}
