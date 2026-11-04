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
// Java: import cn.wps.moffice.writer.data.field.exception.UnExpectedEndFormatException
use crate::cn::wps::moffice::writer::data::field::exception::UnExpectedEndFormatException::*;
// Java: import cn.wps.moffice.writer.data.field.parser.BaseParser
use crate::cn::wps::moffice::writer::data::field::parser::BaseParser::*;
// Java: import java.lang.Object
use stubs::java::lang::Object::*;
use stubs::java::lang::StringBuffer::StringBuffer;
use crate::cn::wps::moffice::writer::data::field::parser::IFieldParser::IFieldParser;
use crate::cn::wps::moffice::writer::data::field::parser::formula::ExtractFieldUtil::ExtractFieldUtil;
use crate::cn::wps::moffice::writer::data::field::parser::formula::FormulaCompre::FormulaCompre;

// java public class FormulaParser extends BaseParser
#[derive(Default)]
pub struct FormulaParser {
    // java BaseParser super_
    pub(crate) super_: BaseParser,
    // java private String resultStr;
    resultStr: Option<String>,
}

// java public class FormulaParser extends BaseParser
impl FormulaParser {
    // java public void parse(PLCField.FieldTriple fieldTriple, Document subDoc, Object arg)
    pub fn parse__body(&mut self, fieldTriple: &FieldTriple, subDoc: &dyn Document, arg: &dyn Object) {
      // java super.parse(fieldTriple, subDoc, arg);
      self.super_.parse(fieldTriple, subDoc, arg);
      // java char[] codeBuffer = this.getCodeBuffer();
      let mut codeBuffer: Vec<char> = self.super_.getCodeBuffer();
      // java this.commonParse(codeBuffer, subDoc);
      self.commonParse(&codeBuffer, subDoc);
    }

    // java public void parseByCharBuffer(char[] codeBuffer, Document subDoc, KFieldType fieldType)
    pub fn parseByCharBuffer__body(&mut self, codeBuffer: &[char], subDoc: &dyn Document, fieldType: KFieldType) {
      // java super.parseByCharBuffer(codeBuffer, subDoc, fieldType);
      self.super_.parseByCharBuffer(codeBuffer, subDoc, fieldType);
      // java if (ExtractFieldUtil.checkContainField(codeBuffer))
      if ExtractFieldUtil::checkContainField(Some(codeBuffer)) {
        // java this.resultStr = new String(ExtractFieldUtil.extractField(codeBuffer, subDoc));
        self.resultStr = ExtractFieldUtil::extractField(Some(codeBuffer), subDoc)
          .map(|v| v.into_iter().collect());
        // java this.mResult = new StringBuffer(this.resultStr);
        self.super_.mResult = Some(StringBuffer::new__string(self.resultStr.as_deref().unwrap_or("")));
      }
      // java else
      else {
        // java this.commonParse(codeBuffer, subDoc);
        self.commonParse(codeBuffer, subDoc);
      }
    }

    // java private synchronized void commonParse(char[] codeBuffer, Document subDoc)
    pub fn commonParse(&mut self, codeBuffer: &[char], _subDoc: &dyn Document) {
      // java this.mCodeLex = new FieldCodeLex();
      self.super_.mCodeLex = Some(FieldCodeLex::default());
      // java this.mCodeLex.parse(codeBuffer, codeBuffer.length);
      if let Some(ref mut lex) = self.super_.mCodeLex {
        let buf: Vec<char> = codeBuffer.to_vec();
        lex.parse(&buf, codeBuffer.len() as i32);
      }
      // java StringBuilder formulaStr = new StringBuilder();
      let mut formulaStr: String = String::new();
      // java char[] ch = new char[]{'\u0000'};
      let mut ch: Vec<char> = vec!['\u{0000}'];
      // java int paramsCount = this.mCodeLex.getParamCount();
      let mut paramsCount: i32 = self.super_.mCodeLex.as_ref().map(|l| l.getParamCount()).unwrap_or(0);
      // java if (paramsCount <= 0)
      if paramsCount <= 0 {
        // java throw new UnExpectedEndFormatException();
        panic!("{}", UnExpectedEndFormatException::new());
      }
      // java else
      else {
        let mut i: i32 = 0;
        // java for(int i = 0; i < paramsCount; ++i)
        while i < paramsCount {
          // java formulaStr.append(this.mCodeLex.getParam(i).mStr);
          if let Some(param) = self.super_.mCodeLex.as_ref().and_then(|l| l.getParam(i)) {
            if let Some(ref s) = param.mStr {
              formulaStr.push_str(s);
            }
          }
          i += 1;
        }
        // java this.resultStr = FormulaCompre.parseFunction(formulaStr.toString());
        self.resultStr = FormulaCompre::parseFunction(Some(&formulaStr));
        // java String formatStr = this.mCodeLex.getPicture(ch);
        let mut formatStr: String = self.super_.mCodeLex.as_ref().map(|l| l.getPicture(Some(&mut ch))).unwrap_or_default();
        // java if (!"".equals(formatStr))
        if formatStr.as_str() != "" {
          // java this.resultStr = StrFormatUtil.numformat_2(this.resultStr, formatStr);
          if let Some(ref rs) = self.resultStr {
            self.resultStr = Some(StrFormatUtil::numformat_2(rs, &formatStr));
          }
        }
        // java else
        else {
          // java double num = Double.parseDouble(this.resultStr);
          let mut num: f64 = self.resultStr.as_ref().and_then(|s| s.parse().ok()).unwrap_or(0.0);
          // java if (num % (double)1.0F == (double)0.0F)
          if num % 1.0 == 0.0 {
            // java this.resultStr = StrFormatUtil.numformat_2(this.resultStr, "0");
            if let Some(ref rs) = self.resultStr {
              self.resultStr = Some(StrFormatUtil::numformat_2(rs, "0"));
            }
          }
        }
        // java this.mResult = new StringBuffer(this.resultStr);
        self.super_.mResult = Some(StringBuffer::new__string(self.resultStr.as_deref().unwrap_or("")));
      }
    }

}

impl IFieldParser for FormulaParser {
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

impl std::ops::Deref for FormulaParser {
    type Target = BaseParser;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for FormulaParser {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}
