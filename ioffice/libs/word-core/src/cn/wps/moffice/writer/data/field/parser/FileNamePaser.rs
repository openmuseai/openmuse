// Java: import cn.wps.moffice.writer.core.Document
use crate::cn::wps::moffice::writer::core::Document::*;
// Java: import cn.wps.moffice.writer.data.PLCField
use crate::cn::wps::moffice::writer::data::PLCField::FieldTriple;
// Java: import cn.wps.moffice.writer.data.field.FieldCodeLex
use crate::cn::wps::moffice::writer::data::field::FieldCodeLex::*;
// Java: import cn.wps.moffice.writer.data.field.KFieldType
use crate::cn::wps::moffice::writer::data::field::KFieldType::*;
// Java: import cn.wps.moffice.writer.data.field.StrFormatUtil
use crate::cn::wps::moffice::writer::data::field::StrFormatUtil::*;
// Java: import cn.wps.moffice.writer.data.field.parser.BaseParser
use crate::cn::wps::moffice::writer::data::field::parser::BaseParser::*;
// Java: import java.lang.Object
use stubs::java::lang::Object::*;
use stubs::java::lang::StringBuffer::StringBuffer;
use crate::cn::wps::moffice::writer::data::field::parser::IFieldParser::IFieldParser;

// java private static final char customSwitch = 'p';
const customSwitch: char = 'p';

// java public class FileNamePaser extends BaseParser
#[derive(Default)]
pub struct FileNamePaser {
    // java BaseParser super_
    pub(crate) super_: BaseParser,
    // java private String fileNameValue = "";
    fileNameValue: Option<String>,
}

// java public class FileNamePaser extends BaseParser
impl FileNamePaser {
    // java public void parse(PLCField.FieldTriple fieldTriple, Document subDoc, Object arg)
    pub fn parse__body(&mut self, fieldTriple: &FieldTriple, subDoc: &dyn Document, arg: &dyn Object) {
      // java super.parse(fieldTriple, subDoc, arg);
      self.super_.parse(fieldTriple, subDoc, arg);
      // java if (arg instanceof String && "parse".equals(arg))
      if arg.as_any().downcast_ref::<String>().map(|s| s.as_str() == "parse").unwrap_or(false) {
        // java char[] codeBuffer = this.getCodeBuffer();
        let mut codeBuffer: Vec<char> = self.super_.getCodeBuffer();
        // java this.commonParse(codeBuffer);
        self.commonParse(&codeBuffer);
      }
    }

    // java public void parseByCharBuffer(char[] codeBuffer, Document subDoc, KFieldType fieldType)
    pub fn parseByCharBuffer__body(&mut self, codeBuffer: &[char], subDoc: &dyn Document, fieldType: KFieldType) {
      // java super.parseByCharBuffer(codeBuffer, subDoc, fieldType);
      self.super_.parseByCharBuffer(codeBuffer, subDoc, fieldType);
      // java this.commonParse(codeBuffer);
      let buf: Vec<char> = codeBuffer.to_vec();
      self.commonParse(&buf);
    }

    // java private void commonParse(char[] codeBuffer)
    pub fn commonParse(&mut self, codeBuffer: &Vec<char>) {
      // java this.mCodeLex = new FieldCodeLex();
      self.super_.mCodeLex = Some(FieldCodeLex::default());
      // java this.mCodeLex.registerPrivateSwitch('p', false);
      if let Some(ref mut lex) = self.super_.mCodeLex {
        lex.registerPrivateSwitch__char_boolean(customSwitch, false);
        // java this.mCodeLex.parse(codeBuffer, codeBuffer.length);
        lex.parse(codeBuffer, codeBuffer.len() as i32);
      }
      // java if (this.mCodeLex.findSwitch('p', (StringBuffer)null))
      if self.super_.mCodeLex.as_ref().map(|l| l.findSwitch(customSwitch, None)).unwrap_or(false) {
        // java this.fileNameValue = this.mSubDocument.getTextDocument().getPath();
        self.fileNameValue = self.super_.subDocument()
          .and_then(|p| p.getTextDocument())
          .map(|td| td.getPath());
      }
      // java else
      else {
        // java this.fileNameValue = this.mSubDocument.getTextDocument().getName();
        self.fileNameValue = self.super_.subDocument()
          .and_then(|p| p.getTextDocument())
          .map(|td| td.getName());
      }
      // java this.fileNameValue = StrFormatUtil.renderByFormat(this.mCodeLex, this.fileNameValue.toCharArray());
      let chars: Vec<char> = self.fileNameValue.as_ref().map(|s| s.chars().collect()).unwrap_or_default();
      self.fileNameValue = StrFormatUtil::renderByFormat(
        self.super_.mCodeLex.as_ref(),
        Some(&chars as &dyn Object),
      ).or_else(|| self.fileNameValue.clone());
      // java this.mResult = new StringBuffer(this.fileNameValue);
      self.super_.mResult = Some(StringBuffer::new__string(self.fileNameValue.as_deref().unwrap_or("")));
    }

}

impl IFieldParser for FileNamePaser {
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

impl std::ops::Deref for FileNamePaser {
    type Target = BaseParser;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for FileNamePaser {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}
