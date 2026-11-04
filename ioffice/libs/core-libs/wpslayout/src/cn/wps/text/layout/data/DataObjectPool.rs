// Java: import cn.wps.text.layout.pool.ObjectPool
// Java: import cn.wps.text.layout.pool.PoolableObjectFactory
use super::TextData::TextData;
use super::SpanInfo::SpanInfo;

// Java field: int MAX_LENGTH
pub(crate) const MAX_LENGTH: i32 = 512;
// Java field: int STEP
pub(crate) const STEP: i32 = 32;

// Java: public class DataObjectPool
pub struct DataObjectPool {
}

// Java: public class CharArray
pub struct CharArray {
    // Java field: char[] mValue
    pub(crate) mValue: Vec<char>,
    // Java field: int mCount
    pub(crate) mCount: i32,
}

// Java: public class DataObjectPool
impl DataObjectPool {
    // Java: DataObjectPool()
    pub fn new() -> Self {
      // Java: super();
      DataObjectPool {}
    }

    // Java: static void destroy()
    pub fn destroy() {
      // Java: textDataObjectPool.clear();
      // Java: spanInfoObjectPool.clear();
      // Java: charArrayObjectPool.clear();
      // Java: stringBufferPool.clear();
    }

    pub fn borrow_text_data() -> TextData { TextData::new() }
    pub fn return_text_data(_data: TextData) {}
    pub fn borrow_span_info() -> SpanInfo { SpanInfo::new() }
    pub fn return_span_info(_span: SpanInfo) {}
    pub fn borrow_char_array() -> CharArray { CharArray::new() }
    pub fn return_char_array(_arr: CharArray) {}
}

// Java: public class CharArray
impl CharArray {
    // Java: CharArray()
    pub fn new() -> Self {
         // Java: super();
         CharArray { mValue: vec!['\0'; 32], mCount: 0 }
    }

    // Java: char[] toCharArray(String src, int start, int len)
    pub fn toCharArray(&mut self, src: &str, start: usize, len: usize) -> &[char] {
         // Java: if (len > 512)
         if len > 512
         {
            // Java: this.clear();
            self.clear();
            // Java: char[] values = new char[len];
            let chars: Vec<char> = src.chars().skip(start).take(len).collect();
            // Java: src.getChars(start, start + len, values, 0);
            self.mValue = chars;
            self.mCount = len as i32;
            // Java: return values;
            return &self.mValue[..len];
         }
         // Java: else
         {
            // Java: this.mCount = len;
            self.mCount = len as i32;
            // Java: if (this.mValue.length < len)
            if self.mValue.len() < len
            {
               // Java: int size = len / 32 * 32 + (len % 32 > 0 ? 32 : 0);
               let size = len / 32 * 32 + if len % 32 > 0 { 32 } else { 0 };
               // Java: this.mValue = new char[size];
               self.mValue = vec!['\0'; size];
            }
            // Java: src.getChars(start, start + len, this.mValue, 0);
            for (i, c) in src.chars().skip(start).take(len).enumerate() {
                self.mValue[i] = c;
            }
            // Java: return this.mValue;
            &self.mValue[..len]
         }
    }

    // Java: int length()
    pub fn length(&self) -> i32 {
         // Java: return this.mCount;
         self.mCount
    }

    // Java: void clear()
    pub fn clear(&mut self) {
         // Java: this.mCount = 0;
         self.mCount = 0;
         // Java: this.mValue[0] = 0;
         if !self.mValue.is_empty() { self.mValue[0] = '\0'; }
    }

}
