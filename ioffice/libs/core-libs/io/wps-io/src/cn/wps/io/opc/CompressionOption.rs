// Java: public enum CompressionOption
pub enum CompressionOption {
    FAST,
    MAXIMUM,
    NORMAL,
    NOT_COMPRESSED,
}

// Java: public enum CompressionOption
impl CompressionOption {
    // Java field: int value

    // Java: CompressionOption(int value)
    pub fn new(value: i32) -> Self {
      // Java: this.value = value;
      match value {
          1 => CompressionOption::FAST,
          9 => CompressionOption::MAXIMUM,
          0 => CompressionOption::NOT_COMPRESSED,
          _ => CompressionOption::NORMAL,
      }
    }

    // Java: int value()
    pub fn value(&self) -> i32 {
      // Java: return this.value;
      match self {
          CompressionOption::FAST => 1,
          CompressionOption::MAXIMUM => 9,
          CompressionOption::NORMAL => -1,
          CompressionOption::NOT_COMPRESSED => 0,
      }
    }
}
