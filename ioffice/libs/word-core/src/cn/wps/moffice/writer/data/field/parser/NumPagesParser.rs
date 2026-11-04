// Java: import cn.wps.moffice.writer.core.Document
use crate::cn::wps::moffice::writer::core::Document::*;
// Java: import cn.wps.moffice.writer.data.PLCField
use crate::cn::wps::moffice::writer::data::PLCField::FieldTriple;
// Java: import cn.wps.moffice.writer.data.field.FieldCodeLex
use crate::cn::wps::moffice::writer::data::field::FieldCodeLex::*;
// Java: import cn.wps.moffice.writer.data.field.KFieldType
use crate::cn::wps::moffice::writer::data::field::KFieldType::*;
// Java: import cn.wps.moffice.writer.data.field.parser.NumFieldParse
use crate::cn::wps::moffice::writer::data::field::parser::NumFieldParse::*;
// Java: import java.lang.Object
use stubs::java::lang::Object::*;
use stubs::java::lang::StringBuffer::StringBuffer;
use crate::cn::wps::moffice::writer::data::field::parser::IFieldParser::IFieldParser;

// java public class NumPagesParser extends NumFieldParse
#[derive(Default)]
pub struct NumPagesParser {
    // java NumFieldParse super_
    pub(crate) super_: NumFieldParse,
    // java private boolean isWebMode;
    isWebMode: bool,
}

// java public class NumPagesParser extends NumFieldParse
impl NumPagesParser {
    // java public void parse(PLCField.FieldTriple fieldTriple, Document subDoc, Object arg)
    pub fn parse__body(&mut self, fieldTriple: &FieldTriple, subDoc: &dyn Document, arg: &dyn Object) {
      // java super.parse(fieldTriple, subDoc, arg);
      self.super_.parse(fieldTriple, subDoc, arg);
      // java if (arg instanceof Integer)
      if let Some(&pageCount) = arg.as_any().downcast_ref::<i32>() {
        // java int pageCount = (Integer)arg;
        let mut pageCount: i32 = pageCount;
        // java int msoNfc = 0;
        let mut msoNfc: i32 = 0;
        // java this.formatResult(pageCount, msoNfc, this.getCodeLex());
        let codeLex = self.super_.getCodeLex();
        self.formatResult(pageCount, msoNfc, &codeLex);
      }
      // java else if (arg instanceof Boolean)
      else if let Some(&is_web) = arg.as_any().downcast_ref::<bool>() {
        // java this.isWebMode = (Boolean)arg;
        self.isWebMode = is_web;
        // java if (this.mSubDocument.getTextDocument().isInIOState())
        let in_io = self.super_.subDocument()
          .and_then(|p| p.getTextDocument())
          .map(|td| td.isInIOState())
          .unwrap_or(false);
        if in_io {
          // java this.reLayout = true;
          self.super_.reLayout = true;
          // java this.mResult = new StringBuffer("");
          self.super_.mResult = Some(StringBuffer::new__string(""));
          // java return;
          return;
        }
        // java char[] codeBuffer = this.getCodeBuffer();
        let mut codeBuffer: Vec<char> = self.super_.getCodeBuffer();
        // java this.numparse(codeBuffer);
        self.registerSwitch();
        let field_num = self.setFieldValue();
        self.super_.numparse_with_value(&codeBuffer, field_num);
      }
    }

    // java public void parseByCharBuffer(char[] codeBuffer, Document subDoc, KFieldType fieldType)
    pub fn parseByCharBuffer__body(&mut self, codeBuffer: &[char], subDoc: &dyn Document, fieldType: KFieldType) {
      // java super.parseByCharBuffer(codeBuffer, subDoc, fieldType);
      self.super_.parseByCharBuffer(codeBuffer, subDoc, fieldType);
      // java this.numparse(codeBuffer);
      let buf: Vec<char> = codeBuffer.to_vec();
      self.registerSwitch();
      let field_num = self.setFieldValue();
      self.super_.numparse_with_value(&buf, field_num);
    }

    // java public boolean formatResult(int pageIndex, int msoNfc, FieldCodeLex codeLex)
    pub fn formatResult(&mut self, pageIndex: i32, msoNfc: i32, codeLex: &FieldCodeLex) -> bool {
      // java int nErr = codeLex.getLastError();
      let mut nErr: i32 = codeLex.getLastError();
      // java if (nErr != 0)
      if nErr != 0 {
        // java return false;
        return false;
      }
      // java else
      else {
        // java this.mResult = new StringBuffer();
        self.super_.mResult = Some(StringBuffer::new());
        // java char[] ch = new char[]{'\u0000'};
        let mut ch: Vec<char> = vec!['\u{0000}'];
        // java String strFmt = codeLex.getPicture(ch);
        let mut strFmt: String = codeLex.getPicture(Some(&mut ch));
        // java if (strFmt.length() > 0)
        if (strFmt.len() as i32) > 0 {
          // java if (ch[0] == '#')
          if ch[0usize] == '#' {
            // java this.formatNumByString2(pageIndex, strFmt, this.mResult);
            // rust: 缓冲区借自 `self.super_`，而该方法取 `&mut self` ⇒ 先 `take()` 取出再放回
            //   （Java 直接传同一对象，无借用规则）。
            if let Some(mut buf) = self.super_.mResult.take() {
              self.super_.formatNumByString2(pageIndex, &strFmt, &mut buf);
              self.super_.mResult = Some(buf);
            }
          }
          // java else if (ch[0] == '*')
          else if ch[0usize] == '*' {
            // java this.formatNumByString(pageIndex, strFmt, this.mResult);
            // rust: 缓冲区借自 `self.super_`，而该方法取 `&mut self` ⇒ 先 `take()` 取出再放回
            //   （Java 直接传同一对象，无借用规则）。
            if let Some(mut buf) = self.super_.mResult.take() {
              self.super_.formatNumByString__int_string_stringbuffer(pageIndex, &strFmt, &mut buf);
              self.super_.mResult = Some(buf);
            }
          }
          // java else
          else {
            // java this.formatNumByString(pageIndex, 0, this.mResult);
            // rust: 缓冲区借自 `self.super_`，而该方法取 `&mut self` ⇒ 先 `take()` 取出再放回
            //   （Java 直接传同一对象，无借用规则）。
            if let Some(mut buf) = self.super_.mResult.take() {
              self.super_.formatNumByString__int_int_stringbuffer(pageIndex, 0, &mut buf);
              self.super_.mResult = Some(buf);
            }
          }
        }
        // java else
        else {
          // java this.formatNumByString(pageIndex, msoNfc, this.mResult);
          // rust: 缓冲区借自 `self.super_`，而该方法取 `&mut self` ⇒ 先 `take()` 取出再放回
          //   （Java 直接传同一对象，无借用规则）。
          if let Some(mut buf) = self.super_.mResult.take() {
            self.super_.formatNumByString__int_int_stringbuffer(pageIndex, msoNfc, &mut buf);
            self.super_.mResult = Some(buf);
          }
        }
        // java return true;
        return true;
      }
    }

    // java public int setFieldValue()
    pub fn setFieldValue(&mut self) -> i32 {
          // java int numPage;
          let mut numPage: i32;
          // java if (this.isWebMode)
          if self.isWebMode {
            // java numPage = this.metaProperty.getPageCount();
            numPage = self.super_.metaProperty.as_ref().map(|m| m.getPageCount()).unwrap_or(0);
          }
          // java else
          else {
            // java numPage = 1;
            numPage = 1;
          }
          // java return numPage;
          return numPage;
          // java Exception var2
          #[allow(unreachable_code)]
          {
            // java return 0;
            return 0;
          }
    }

    // java public void registerSwitch()
    pub fn registerSwitch(&mut self) {
    }

}

impl IFieldParser for NumPagesParser {
    fn parse(&mut self, fieldTriple: &FieldTriple, subDoc: &dyn Document, arg: &dyn Object) {
        self.parse__body(fieldTriple, subDoc, arg);
    }
    fn parseByCharBuffer(&mut self, codeBuffer: &[char], subDoc: &dyn Document, fieldType: KFieldType) {
        self.parseByCharBuffer__body(codeBuffer, subDoc, fieldType);
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
        self.super_.getResultRange()
    }
}

impl std::ops::Deref for NumPagesParser {
    type Target = NumFieldParse;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for NumPagesParser {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}
