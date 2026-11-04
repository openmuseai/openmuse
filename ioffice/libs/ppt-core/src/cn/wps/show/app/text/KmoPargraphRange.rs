#![allow(dead_code, non_snake_case, unused_variables, unused_imports, unused_mut)]

// Java: package cn.wps.show.app.text
// Java: public class KmoPargraphRange
//
// 100% 直译：本类无外部依赖（纯四个 `int` 字段 + 读写器），默认构建下全量真实实现，0 处 DEFERRED。
// owner: show.app.text    wave: M-PPT-KMOSLIDE-SP3
pub struct KmoPargraphRange {
    // Java field: int startIndex = 0
    pub(crate) startIndex: i32,
    // Java field: int endIndex = 0
    pub(crate) endIndex: i32,
    // Java field: int startOffset = 0
    pub(crate) startOffset: i32,
    // Java field: int endOffset = 0
    pub(crate) endOffset: i32,
}

// Java: KmoPargraphRange()（隐式无参构造器，字段初始化为 0）
impl Default for KmoPargraphRange {
    fn default() -> Self {
        KmoPargraphRange {
            // Java field initializer: int startIndex = 0
            startIndex: 0,
            // Java field initializer: int endIndex = 0
            endIndex: 0,
            // Java field initializer: int startOffset = 0
            startOffset: 0,
            // Java field initializer: int endOffset = 0
            endOffset: 0,
        }
    }
}

// Java: public class KmoPargraphRange
impl KmoPargraphRange {
    // Java: KmoPargraphRange()
    pub fn new() -> Self {
      // Java: （隐式无参构造器：四个字段默认 0）
      Self::default()
    }

    // Java: public int startIndex()
    pub fn startIndex(&self) -> i32 {
      // Java: return this.startIndex;
      self.startIndex
    }

    // Java: public void setStartIndex(int startIndex)
    pub fn setStartIndex(&mut self, startIndex: i32) {
      // Java: this.startIndex = startIndex;
      self.startIndex = startIndex;
    }

    // Java: public int endIndex()
    pub fn endIndex(&self) -> i32 {
      // Java: return this.endIndex;
      self.endIndex
    }

    // Java: public void setEndIndex(int endIndex)
    pub fn setEndIndex(&mut self, endIndex: i32) {
      // Java: this.endIndex = endIndex;
      self.endIndex = endIndex;
    }

    // Java: public int startOffset()
    pub fn startOffset(&self) -> i32 {
      // Java: return this.startOffset;
      self.startOffset
    }

    // Java: public void setStartOffset(int startOffset)
    pub fn setStartOffset(&mut self, startOffset: i32) {
      // Java: this.startOffset = startOffset;
      self.startOffset = startOffset;
    }

    // Java: public int endOffset()
    pub fn endOffset(&self) -> i32 {
      // Java: return this.endOffset;
      self.endOffset
    }

    // Java: public void setEndOffset(int endOffset)
    pub fn setEndOffset(&mut self, endOffset: i32) {
      // Java: this.endOffset = endOffset;
      self.endOffset = endOffset;
    }
}
