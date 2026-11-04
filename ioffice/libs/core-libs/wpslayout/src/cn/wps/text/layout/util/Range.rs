use super::TextRangeType::TextRangeType;

// Java: public class Range
pub struct Range {
    // Java field: int mStart
    pub(crate) mStart: i32,
    // Java field: int mEnd
    pub(crate) mEnd: i32,
    // Java field: TextRangeType mType
    pub(crate) mType: TextRangeType,
}

// Java: public class Range
impl Range {
    // Java: Range(int start, int end, TextRangeType type)
    pub fn new(start: i32, end: i32, r#type: TextRangeType) -> Self {
      // Java: super();
      // Java: this.mStart = start;
      // Java: this.mEnd = end;
      // Java: this.mType = type;
      Range {
          mStart: start,
          mEnd: end,
          mType: r#type,
      }
    }

    // Java: boolean contains(int number)
    pub fn contains(&self, number: i32) -> bool {
      // Java: return number >= this.mStart && number <= this.mEnd;
      number >= self.mStart && number <= self.mEnd
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

    // Java: TextRangeType type()
    pub fn r#type(&self) -> TextRangeType {
      // Java: return this.mType;
      self.mType
    }

}
