// Java: import cn.wps.moffice.writer.core.Document
use crate::cn::wps::moffice::writer::core::Document::*;
// Java: import cn.wps.moffice.writer.data.PLCField
use crate::cn::wps::moffice::writer::data::PLCField::FieldTriple;
// Java: import cn.wps.moffice.writer.data.field.FieldCodeLex
use crate::cn::wps::moffice::writer::data::field::FieldCodeLex::*;
// Java: import cn.wps.moffice.writer.data.field.KFieldType
use crate::cn::wps::moffice::writer::data::field::KFieldType::*;
// Java: import cn.wps.moffice.writer.data.field.ParamScope
use crate::cn::wps::moffice::writer::data::field::ParamScope::*;
// Java: import cn.wps.moffice.writer.data.field.StrFormatUtil
use crate::cn::wps::moffice::writer::data::field::StrFormatUtil::*;
// Java: import cn.wps.moffice.writer.data.field.parser.BaseParser
use crate::cn::wps::moffice::writer::data::field::parser::BaseParser::*;
// Java: import java.lang.Object
use stubs::java::lang::Object::*;
use stubs::java::lang::StringBuffer::StringBuffer;
use crate::cn::wps::moffice::writer::data::field::parser::IFieldParser::IFieldParser;

// java public class FieldStringResultParser extends BaseParser
#[derive(Default)]
pub struct FieldStringResultParser {
    // java BaseParser super_
    pub(crate) super_: BaseParser,
    // java private String filedStringResult = "";
    filedStringResult: Option<String>,
}

// java public class FieldStringResultParser extends BaseParser
impl FieldStringResultParser {
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
        // java KFieldType fieldType = KFieldType.getKFieldType(fieldTriple.getFieldType());
        let mut fieldType: KFieldType = KFieldType::getKFieldType(fieldTriple.getFieldType()).unwrap_or(KFieldType::FieldAuthor);
        // java this.commonParse(codeBuffer, fieldType);
        self.commonParse(&codeBuffer, fieldType);
      }
    }

    // java public void parseByCharBuffer(char[] codeBuffer, Document subDoc, KFieldType fieldType)
    pub fn parseByCharBuffer__body(&mut self, codeBuffer: &[char], subDoc: &dyn Document, fieldType: KFieldType) {
      // java super.parseByCharBuffer(codeBuffer, subDoc, fieldType);
      self.super_.parseByCharBuffer(codeBuffer, subDoc, fieldType);
      // java this.commonParse(codeBuffer, fieldType);
      let buf: Vec<char> = codeBuffer.to_vec();
      self.commonParse(&buf, fieldType);
    }

    // java private void commonParse(char[] codeBuffer, KFieldType fieldType)
    pub fn commonParse(&mut self, codeBuffer: &Vec<char>, fieldType: KFieldType) {
      // java this.mCodeLex = new FieldCodeLex();
      self.super_.mCodeLex = Some(FieldCodeLex::default());
      // java this.mCodeLex.parse(codeBuffer, codeBuffer.length);
      if let Some(ref mut lex) = self.super_.mCodeLex {
        lex.parse(codeBuffer, codeBuffer.len() as i32);
      }
      // java this.filedStringResult = this.setFieldStr(this.filedStringResult, fieldType);
      let cur = self.filedStringResult.clone().unwrap_or_default();
      self.filedStringResult = Some(self.setFieldStr(cur, fieldType));
      // java if (KFieldType.FieldComments == KFieldType.getKFieldType(this.mFieldTriple.getFieldType()) && this.filedStringResult.length() > 255)
      let ft = self.super_.mFieldTriple.as_ref().and_then(|t| KFieldType::getKFieldType(t.getFieldType()));
      if ft == Some(KFieldType::FieldComments) && self.filedStringResult.as_ref().map(|s| s.len()).unwrap_or(0) > 255 {
        // java this.filedStringResult = this.filedStringResult.substring(0, 255);
        if let Some(ref mut s) = self.filedStringResult {
          *s = s.chars().take(255).collect();
        }
      }
      // java this.filedStringResult = StrFormatUtil.renderByFormat(this.mCodeLex, this.filedStringResult.toCharArray());
      let chars: Vec<char> = self.filedStringResult.as_ref().map(|s| s.chars().collect()).unwrap_or_default();
      self.filedStringResult = StrFormatUtil::renderByFormat(
        self.super_.mCodeLex.as_ref(),
        Some(&chars as &dyn Object),
      ).or_else(|| self.filedStringResult.clone());
      // java this.mResult = new StringBuffer(this.filedStringResult);
      self.super_.mResult = Some(StringBuffer::new__string(self.filedStringResult.as_deref().unwrap_or("")));
    }

    // java private String setFieldStr(String filedStringResult_1, KFieldType fieldType)
    pub fn setFieldStr(&mut self, mut filedStringResult_1: String, fieldType: KFieldType) -> String {
      // java int paramCount = this.mCodeLex.getParamCount();
      let mut paramCount: i32 = self.super_.mCodeLex.as_ref().map(|l| l.getParamCount()).unwrap_or(0);
      // java if (paramCount > 0)
      if paramCount > 0 {
        // java ParamScope paramScope = this.mCodeLex.getParam(0);
        let mut paramScope: Option<&ParamScope> = self.super_.mCodeLex.as_ref().and_then(|l| l.getParam(0));
        // java filedStringResult_1 = paramScope.mStr;
        filedStringResult_1 = paramScope.and_then(|p| p.mStr.clone()).unwrap_or_default();
        // java this.backFillVal(filedStringResult_1);
        self.backFillVal(&filedStringResult_1);
      }
      // java else if (this.metaProperty != null)
      else if self.super_.metaProperty.is_some() {
        // java filedStringResult_1 = this.getFieldResult(fieldType);
        filedStringResult_1 = self.getFieldResult(fieldType);
      }
      // java else
      else {
        // java filedStringResult_1 = "";
        filedStringResult_1 = String::new();
      }
      // java return filedStringResult_1;
      return filedStringResult_1;
    }

    // java private String getFieldResult(KFieldType fieldType)
    pub fn getFieldResult(&self, fieldType: KFieldType) -> String {
      let meta = self.super_.metaProperty.as_ref();
      // java switch (fieldType)
      match fieldType {
        // java case FieldAuthor:
        KFieldType::FieldAuthor => {
          // java return this.metaProperty.getAuthor() != null ? this.metaProperty.getAuthor() : "";
          return meta.map(|m| m.getAuthor()).unwrap_or_default();
        }
        // java case FieldComments:
        KFieldType::FieldComments => {
          // java return this.metaProperty.getComments() != null ? this.metaProperty.getComments() : "";
          return meta.map(|m| m.getComments()).unwrap_or_default();
        }
        // java case FieldLastSavedBy:
        KFieldType::FieldLastSavedBy => {
          // java return this.metaProperty.getLastAuthor() != null ? this.metaProperty.getLastAuthor() : "";
          return meta.map(|m| m.getLastAuthor()).unwrap_or_default();
        }
        // java case FieldSubject:
        KFieldType::FieldSubject => {
          // java return this.metaProperty.getSubject() != null ? this.metaProperty.getSubject() : "";
          return meta.map(|m| m.getSubject()).unwrap_or_default();
        }
        // java case FieldTitle:
        KFieldType::FieldTitle => {
          // java return this.metaProperty.getTitle() != null ? this.metaProperty.getTitle() : "";
          return meta.map(|m| m.getTitle()).unwrap_or_default();
        }
        // java case FieldKeyWord:
        KFieldType::FieldKeyWord => {
          // java return this.metaProperty.getKeyWords() != null ? this.metaProperty.getKeyWords() : "";
          return meta.map(|m| m.getKeyWords()).unwrap_or_default();
        }
        // java case FieldUserAddress:
        KFieldType::FieldUserAddress => {
          // java return "";
          return String::new();
        }
        // java case FieldUserInitials:
        KFieldType::FieldUserInitials => {
          // java return "";
          return String::new();
        }
        // java case FieldUserName:
        KFieldType::FieldUserName => {
          // java return "";
          return String::new();
        }
        // java default:
        _ => {
          // java return "";
          return String::new();
        }
      }
    }

    // java private void backFillVal(String fieldValue)
    pub fn backFillVal(&mut self, fieldValue: &str) {
      // java KFieldType fieldType = KFieldType.getKFieldType(this.mFieldTriple.getFieldType());
      let mut fieldType: KFieldType = self.super_.mFieldTriple.as_ref()
        .and_then(|t| KFieldType::getKFieldType(t.getFieldType()))
        .unwrap_or(KFieldType::FieldAuthor);
      // java switch (fieldType)
      match fieldType {
        // java case FieldAuthor:
        KFieldType::FieldAuthor => {
          // java this.metaProperty.setAuthor(fieldValue);
          if let Some(ref mut m) = self.super_.metaProperty { m.setAuthor(fieldValue); }
        }
        // java case FieldComments:
        KFieldType::FieldComments => {
          // java this.metaProperty.setComments(fieldValue);
          if let Some(ref mut m) = self.super_.metaProperty { m.setComments(fieldValue); }
        }
        // java case FieldLastSavedBy:
        KFieldType::FieldLastSavedBy => {
          // java this.metaProperty.setLastAuthor(fieldValue);
          if let Some(ref mut m) = self.super_.metaProperty { m.setLastAuthor(fieldValue); }
        }
        // java case FieldSubject:
        KFieldType::FieldSubject => {
          // java this.metaProperty.setSubject(fieldValue);
          if let Some(ref mut m) = self.super_.metaProperty { m.setSubject(fieldValue); }
        }
        // java case FieldTitle:
        KFieldType::FieldTitle => {
          // java this.metaProperty.setTitle(fieldValue);
          if let Some(ref mut m) = self.super_.metaProperty { m.setTitle(fieldValue); }
        }
        // java case FieldKeyWord:
        KFieldType::FieldKeyWord => {
          // java this.metaProperty.setKeyWords(fieldValue);
          if let Some(ref mut m) = self.super_.metaProperty { m.setKeyWords(fieldValue); }
        }
        // java default:
        _ => {
        }
      }
    }

}

impl IFieldParser for FieldStringResultParser {
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

impl std::ops::Deref for FieldStringResultParser {
    type Target = BaseParser;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for FieldStringResultParser {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}
