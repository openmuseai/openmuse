// Java: import cn.wps.moffice.writer.core.Document
use crate::cn::wps::moffice::writer::core::Document::*;
// Java: import cn.wps.moffice.writer.data.PLCField
use crate::cn::wps::moffice::writer::data::PLCField::FieldTriple;
// Java: import cn.wps.moffice.writer.data.RANGEFactory
use crate::cn::wps::moffice::writer::data::RANGEFactory::*;
// Java: import cn.wps.moffice.writer.data.field.parser.BaseParser
use crate::cn::wps::moffice::writer::data::field::parser::BaseParser::*;
// Java: import java.lang.Object
use stubs::java::lang::Object::*;
use crate::cn::wps::moffice::writer::data::field::parser::IFieldParser::IFieldParser;
use crate::cn::wps::moffice::writer::data::field::KFieldType::KFieldType;

// java public class MacroBtnParser extends BaseParser
#[derive(Default)]
pub struct MacroBtnParser {
    // java BaseParser super_
    pub(crate) super_: BaseParser,
    // java int start = 0;
    pub(crate) start: i32,
    // java int end = 0;
    pub(crate) end: i32,
}

// java public class MacroBtnParser extends BaseParser
impl MacroBtnParser {
    // java public void parse(PLCField.FieldTriple fieldTriple, Document subDoc, Object arg)
    pub fn parse__body(&mut self, fieldTriple: &FieldTriple, subDoc: &dyn Document, arg: &dyn Object) {
      // java super.parse(fieldTriple, subDoc, arg);
      self.super_.parse(fieldTriple, subDoc, arg);
      // java char[] codeBuffer = this.getCodeBuffer();
      let mut codeBuffer: Vec<char> = self.super_.getCodeBuffer();
      // java int next_pos = this.skipChar(codeBuffer, 0, ' ');
      let mut next_pos: i32 = self.skipChar(&codeBuffer, 0, ' ');
      // java int src_len = codeBuffer.length;
      let mut src_len: i32 = codeBuffer.len() as i32;
      // java int space_num = 0;
      let mut space_num: i32 = 0;
      // java this.start = 0;
      self.start = 0;
      // java this.end = 0;
      self.end = 0;
      let mut idx: i32 = next_pos;
      // java for(int idx = next_pos; idx < src_len ++idx)
      while idx < src_len {
        // java if (codeBuffer[idx] == 19)
        if codeBuffer[(idx) as usize] == '\u{0013}' {
          // java this.start = idx + 1 + fieldTriple.getStartCP();
          self.start = idx + 1 + fieldTriple.getStartCP();
          // java this.end = src_len - idx - 1 + this.start;
          self.end = src_len - idx - 1 + self.start;
          // java return;
          return;
        }
        // java if (codeBuffer[idx] == ' ')
        if codeBuffer[(idx) as usize] == ' ' {
          // java int skip_num = this.skipChar(codeBuffer, idx, ' ');
          let mut skip_num: i32 = self.skipChar(&codeBuffer, idx, ' ');
          // java if (space_num != 0)
          if space_num != 0 {
            // java this.start = idx + skip_num + 1 + fieldTriple.getStartCP();
            self.start = idx + skip_num + 1 + fieldTriple.getStartCP();
            // java this.end = this.get_ResultLen(codeBuffer, idx + skip_num) + this.start;
            self.end = self.get_ResultLen(&codeBuffer, idx + skip_num) + self.start;
            // java if (this.end >= 0)
            if self.end >= 0 {
              // java return;
              return;
            }
            // java return;
            return;
          }
          // java idx = idx + skip_num - 1;
          idx = idx + skip_num - 1;
          // java ++space_num;
          space_num += 1;
        }
        idx += 1;
      }
    }

    // java public Long getResultRange()
    pub fn getResultRange__body(&self) -> Option<i64> {
      // java return this.end <= this.start ? null : RANGEFactory.getRANGEObject(this.start, this.end);
      return if self.end <= self.start { None } else { Some(RANGEFactory::getRANGEObject(self.start, self.end)) };
    }

    // java private int skipChar(char[] codeBuffer, int pos, char ch)
    pub fn skipChar(&self, codeBuffer: &Vec<char>, pos: i32, ch: char) -> i32 {
      // java int old;
      let mut old: i32;
      old = pos;
      let mut pos = pos;
      // java for(old = pos pos < codeBuffer.length && ch == codeBuffer[pos] ++pos)
      while pos < (codeBuffer.len() as i32) && ch == codeBuffer[(pos) as usize] {
        pos += 1;
      }
      // java return pos - old;
      return pos - old;
    }

    // java private int get_ResultLen(char[] codeBuffer, int pos)
    pub fn get_ResultLen(&self, codeBuffer: &Vec<char>, pos: i32) -> i32 {
      let mut pos = pos;
      // java while (pos < codeBuffer.length)
      while pos < (codeBuffer.len() as i32) {
        // java if (codeBuffer[pos] == '\r')
        if codeBuffer[(pos) as usize] == '\r' {
          // java return -1;
          return -1;
        }
        // java ++pos;
        pos += 1;
      }
      // java return codeBuffer.length;
      return codeBuffer.len() as i32;
    }

}

impl IFieldParser for MacroBtnParser {
    fn parse(&mut self, fieldTriple: &FieldTriple, subDoc: &dyn Document, arg: &dyn Object) {
        self.parse__body(fieldTriple, subDoc, arg);
    }
    fn parseByCharBuffer(&mut self, codeBuffer: &[char], subDoc: &dyn Document, fieldType: KFieldType) {
        self.super_.parseByCharBuffer(codeBuffer, subDoc, fieldType);
    }
    fn isReLayout(&self) -> bool {
        self.super_.isReLayout()
    }
    fn dispose(&mut self) {
        self.super_.dispose();
    }
    fn getResult(&self) -> Option<String> {
        self.super_.getResult()
    }
    fn getResultRange(&self) -> Option<i64> {
        self.getResultRange__body()
    }
}

impl std::ops::Deref for MacroBtnParser {
    type Target = BaseParser;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for MacroBtnParser {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}
