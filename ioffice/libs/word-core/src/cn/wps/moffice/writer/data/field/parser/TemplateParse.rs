// Java: import cn.wps.moffice.kfs.File
use kfs_kfs_app::cn::wps::moffice::kfs::File::*;
// Java: import cn.wps.moffice.property.PropertySet
use util::moffice::property::PropertySet::*;
// Java: import cn.wps.moffice.writer.core.Document
use crate::cn::wps::moffice::writer::core::Document::*;
// Java: import cn.wps.moffice.writer.core.TextDocument
use crate::cn::wps::moffice::writer::core::TextDocument::*;
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

// java static final boolean DEBUG = false;
pub(crate) const DEBUG: bool = false;
// java private static final char customSwitch = 'p';
const customSwitch: char = 'p';

// java public class TemplateParse extends BaseParser
#[derive(Default)]
pub struct TemplateParse {
    // java BaseParser super_
    pub(crate) super_: BaseParser,
    // java private String fileNameValue = "";
    fileNameValue: Option<String>,
}

// java public class TemplateParse extends BaseParser
impl TemplateParse {
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
        // java TextDocument textDoc = this.mSubDocument.getTextDocument();
        let textDoc = self.super_.subDocument().and_then(|p| p.getTextDocument());
        // java PropertySet pro = textDoc.getProperty();
        let pro: Option<PropertySet> = textDoc.map(|td| td.getProperty());
        // java this.fileNameValue = this.metaProperty.getTemplate();
        self.fileNameValue = self.super_.metaProperty.as_ref().map(|m| m.getTemplate());
        // java if (pro.contains(421))
        if pro.as_ref().map(|p| p.contains(421)).unwrap_or(false) {
          // java this.fileNameValue = pro.getString(421) + File.separator + this.fileNameValue;
          let sep = std::path::MAIN_SEPARATOR.to_string();
          let prefix = pro.as_ref().map(|p| p.getString(421)).unwrap_or_default();
          let name = self.fileNameValue.clone().unwrap_or_default();
          self.fileNameValue = Some(format!("{}{}{}", prefix, sep, name));
          let _ = DEBUG;
        }
      }
      // java else
      else {
        // java this.fileNameValue = this.metaProperty.getTemplate();
        self.fileNameValue = self.super_.metaProperty.as_ref().map(|m| m.getTemplate());
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

impl IFieldParser for TemplateParse {
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

impl std::ops::Deref for TemplateParse {
    type Target = BaseParser;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for TemplateParse {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}
