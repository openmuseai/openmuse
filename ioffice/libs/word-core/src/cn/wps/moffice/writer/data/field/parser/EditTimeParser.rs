// Java: import cn.wps.moffice.writer.core.Document
use crate::cn::wps::moffice::writer::core::Document::*;
// Java: import cn.wps.moffice.writer.data.PLCField
use crate::cn::wps::moffice::writer::data::PLCField::FieldTriple;
// Java: import cn.wps.moffice.writer.data.field.KFieldType
use crate::cn::wps::moffice::writer::data::field::KFieldType::*;
// Java: import cn.wps.moffice.writer.data.field.parser.NumFieldParse
use crate::cn::wps::moffice::writer::data::field::parser::NumFieldParse::*;
// Java: import java.lang.Object
use stubs::java::lang::Object::*;
use stubs::java::lang::StringBuffer::StringBuffer;
use crate::cn::wps::moffice::writer::data::field::parser::IFieldParser::IFieldParser;
use crate::cn::wps::moffice::writer::data::field::FieldCodeLex::FieldCodeLex;

// java public class EditTimeParser extends NumFieldParse
#[derive(Default)]
pub struct EditTimeParser {
    // java NumFieldParse super_
    pub(crate) super_: NumFieldParse,
}

// java public class EditTimeParser extends NumFieldParse
impl EditTimeParser {
    // java public void parse(PLCField.FieldTriple fieldTriple, Document subDoc, Object arg)
    pub fn parse__body(&mut self, fieldTriple: &FieldTriple, subDoc: &dyn Document, arg: &dyn Object) {
      // java super.parse(fieldTriple, subDoc, arg);
      self.super_.parse(fieldTriple, subDoc, arg);
      // java if (arg instanceof String && "parse".equals(arg))
      if arg.as_any().downcast_ref::<String>().map(|s| s.as_str() == "parse").unwrap_or(false) {
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
        self.super_.mCodeLex = Some(FieldCodeLex::default());
        self.registerSwitch();
        if let Some(ref mut lex) = self.super_.mCodeLex {
          lex.parse(&codeBuffer, codeBuffer.len() as i32);
        }
        let field_num = self.setFieldValue();
        self.super_.format_field_num(field_num);
      }
    }

    // java public void parseByCharBuffer(char[] codeBuffer, Document subDoc, KFieldType fieldType)
    pub fn parseByCharBuffer__body(&mut self, codeBuffer: &[char], subDoc: &dyn Document, fieldType: KFieldType) {
      // java super.parseByCharBuffer(codeBuffer, subDoc, fieldType);
      self.super_.parseByCharBuffer(codeBuffer, subDoc, fieldType);
      // java this.numparse(codeBuffer);
      let buf: Vec<char> = codeBuffer.to_vec();
      self.super_.mCodeLex = Some(FieldCodeLex::default());
      self.registerSwitch();
      if let Some(ref mut lex) = self.super_.mCodeLex {
        lex.parse(&buf, buf.len() as i32);
      }
      let field_num = self.setFieldValue();
      self.super_.format_field_num(field_num);
    }

    // java public int setFieldValue()
    pub fn setFieldValue(&mut self) -> i32 {
      // java long editTime = this.metaProperty.getEditTime();
      let mut editTime: i64 = self.super_.metaProperty.as_ref().map(|m| m.getEditTime()).unwrap_or(0);
      // java return (int)editTime / '\uea60';
      return (editTime as i32) / (0xEA60 as i32);
    }

    // java public void registerSwitch()
    pub fn registerSwitch(&mut self) {
    }

}

impl IFieldParser for EditTimeParser {
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

impl std::ops::Deref for EditTimeParser {
    type Target = NumFieldParse;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for EditTimeParser {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}
