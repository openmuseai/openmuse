#![allow(non_snake_case)]
#![allow(dead_code)]

// Java: public class NumFmtResult
pub struct NumFmtResult {
    // Java field: StringBuffer mNumFmtStr
    pub mNumFmtStr: String,
    // Java field: StringBuffer mNumRealValue
    pub mNumRealValue: String,
    // Java field: Integer mNumFmtColorIdx
    pub mNumFmtColorIdx: Option<i32>,
    // Java field: boolean richFormatInvalid
    pub richFormatInvalid: bool,
    // Java field: boolean mNumericAsText
    pub mNumericAsText: bool,
    // Java field: int mFillPos
    pub mFillPos: i32,
    // Java field: char mFillChar
    pub mFillChar: char,
}

// Java: public class NumFmtResult
impl NumFmtResult {
    // Java: NumFmtResult()
    pub fn new() -> Self {
      // Java: super();
      NumFmtResult {
          mNumFmtStr: String::new(),
          mNumRealValue: String::new(),
          mNumFmtColorIdx: None,
          richFormatInvalid: false,
          mNumericAsText: false,
          mFillPos: -1,
          mFillChar: '\0',
      }
    }

    // Java: void set(String formatResult, Integer formatColorIdx)
    pub fn set(&mut self, formatResult: &str, formatColorIdx: Option<i32>) {
      // Java: this.clear();
      self.clear();
      // Java: this.mNumFmtStr.append(formatResult);
      self.mNumFmtStr.push_str(formatResult);
      // Java: this.mNumFmtColorIdx = formatColorIdx;
      self.mNumFmtColorIdx = formatColorIdx;
    }

    // Java: void clear()
    pub fn clear(&mut self) {
      // Java: this.mNumFmtStr.setLength(0);
      self.mNumFmtStr.clear();
      // Java: this.mNumRealValue.setLength(0);
      self.mNumRealValue.clear();
      // Java: this.mNumFmtColorIdx = null;
      self.mNumFmtColorIdx = None;
      // Java: this.richFormatInvalid = false;
      self.richFormatInvalid = false;
      // Java: this.mNumericAsText = false;
      self.mNumericAsText = false;
      // Java: this.mFillPos = -1;
      self.mFillPos = -1;
    }

    // Java: String numfmtStr()
    pub fn numfmtStr(&self) -> &str {
      // Java: return this.mNumFmtStr.toString();
      &self.mNumFmtStr
    }

    // Java: void correctResult(String oldSeparator, String newSeparator)
    pub fn correctResult(&mut self, oldSeparator: &str, newSeparator: &str) {
      // Java: String numFmtStr = this.numfmtStr();
      let numFmtStr = self.mNumFmtStr.clone();
      // Java: if (numFmtStr != null && !numFmtStr.isEmpty())
      if true && !numFmtStr.is_empty() {
      // LLM Rust Null-Safety: numFmtStr is always non-null String in Rust, first condition is always true
         // Java: int length = numFmtStr.length();
         let length = numFmtStr.len();
         // Java: String replaceStr = numFmtStr.replaceAll(oldSeparator, newSeparator);
         let replaceStr = numFmtStr.replace(oldSeparator, newSeparator);
         // Java: this.mNumFmtStr.replace(0, length, replaceStr);
         let _ = length;
         self.mNumFmtStr = replaceStr;
      }
    }

}
