// Java: import cn.wps.moffice.writer.core.Document
use crate::cn::wps::moffice::writer::core::Document::*;
// Java: import cn.wps.moffice.writer.data.PLCField
use crate::cn::wps::moffice::writer::data::PLCField::FieldTriple;
// Java: import cn.wps.moffice.writer.data.field.FieldCodeLex
use crate::cn::wps::moffice::writer::data::field::FieldCodeLex::*;
// Java: import cn.wps.moffice.writer.data.field.StrFormatUtil
use crate::cn::wps::moffice::writer::data::field::StrFormatUtil::*;
// Java: import cn.wps.moffice.writer.data.field.parser.BaseParser
use crate::cn::wps::moffice::writer::data::field::parser::BaseParser::*;
// Java: import java.lang.Object
use stubs::java::lang::Object::*;
use stubs::java::lang::StringBuffer::StringBuffer;
use crate::cn::wps::moffice::writer::data::field::parser::IFieldParser::IFieldParser;
use crate::cn::wps::moffice::writer::data::field::KFieldType::KFieldType;

// java public class SectionParse extends BaseParser
#[derive(Default)]
pub struct SectionParse {
    // java BaseParser super_
    pub(crate) super_: BaseParser,
}

// java public class SectionParse extends BaseParser
impl SectionParse {
    // java public void parse(PLCField.FieldTriple fieldTriple, Document subDoc, Object arg)
    pub fn parse__body(&mut self, fieldTriple: &FieldTriple, subDoc: &dyn Document, arg: &dyn Object) {
      // java super.parse(fieldTriple, subDoc, arg);
      self.super_.parse(fieldTriple, subDoc, arg);
      // java if (arg instanceof Integer)
      if let Some(&sectionCount) = arg.as_any().downcast_ref::<i32>() {
        // java int sectionCount = (Integer)arg;
        let mut sectionCount: i32 = sectionCount;
        // java String resultStr = String.valueOf(sectionCount);
        let mut resultStr: String = sectionCount.to_string();
        // java this.mCodeLex = new FieldCodeLex();
        self.super_.mCodeLex = Some(FieldCodeLex::default());
        // java char[] codeBuffer = this.getCodeBuffer();
        let mut codeBuffer: Vec<char> = self.super_.getCodeBuffer();
        // java this.mCodeLex.parse(codeBuffer, codeBuffer.length);
        if let Some(ref mut lex) = self.super_.mCodeLex {
          lex.parse(&codeBuffer, codeBuffer.len() as i32);
        }
        // java int formatCount = this.mCodeLex.getFormatCount();
        let mut formatCount: i32 = self.super_.mCodeLex.as_ref().map(|l| l.getFormatCount()).unwrap_or(0);
        // java if (formatCount > 0)
        if formatCount > 0 {
          // java resultStr = StrFormatUtil.renderByFormat(this.mCodeLex, (long)sectionCount);
          let num: i64 = sectionCount as i64;
          resultStr = StrFormatUtil::renderByFormat(
            self.super_.mCodeLex.as_ref(),
            Some(&num as &dyn Object),
          ).unwrap_or(resultStr);
        }
        // java else
        else {
          // java char[] ch = new char[]{'\u0000'};
          let mut ch: Vec<char> = vec!['\u{0000}'];
          // java String pictureFormat = this.mCodeLex.getPicture(ch);
          let mut pictureFormat: String = self.super_.mCodeLex.as_ref().map(|l| l.getPicture(Some(&mut ch))).unwrap_or_default();
          // java if (!"".equals(pictureFormat))
          if pictureFormat.as_str() != "" {
            // java resultStr = StrFormatUtil.numformat_2(String.valueOf(sectionCount), pictureFormat);
            resultStr = StrFormatUtil::numformat_2(&sectionCount.to_string(), &pictureFormat);
          }
        }
        // java this.mResult = new StringBuffer(resultStr);
        self.super_.mResult = Some(StringBuffer::new__string(&resultStr));
      }
    }

}

impl IFieldParser for SectionParse {
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
        self.super_.getResultRange()
    }
}

impl std::ops::Deref for SectionParse {
    type Target = BaseParser;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for SectionParse {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}
