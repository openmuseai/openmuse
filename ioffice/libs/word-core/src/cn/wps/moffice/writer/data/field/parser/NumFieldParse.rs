// Java: import cn.wps.kfc.numfmt.IWordFormatter
use kfc::cn::wps::kfc::numfmt::IWordFormatter::*;
// Java: import cn.wps.kfc.numfmt.NumFmtFactory
use kfc::cn::wps::kfc::numfmt::NumFmtFactory::*;
// Java: import cn.wps.moffice.writer.data.field.FieldCodeLex
use crate::cn::wps::moffice::writer::data::field::FieldCodeLex::*;
// Java: import cn.wps.moffice.writer.data.field.StrFormatUtil
use crate::cn::wps::moffice::writer::data::field::StrFormatUtil::*;
// Java: import cn.wps.moffice.writer.data.field.parser.BaseParser
use crate::cn::wps::moffice::writer::data::field::parser::BaseParser::*;
use stubs::java::lang::Object::Object;
use stubs::java::lang::StringBuffer::StringBuffer;

// java public abstract class NumFieldParse extends BaseParser
pub struct NumFieldParse {
    // java BaseParser super_
    pub(crate) super_: BaseParser,
    // java private String fieldValue = "";
    fieldValue: Option<String>,
    // java private IWordFormatter mWordFormatter = NumFmtFactory.createWordFormatter((short)0);
    mWordFormatter: Option<Box<dyn IWordFormatter>>,
}

impl Default for NumFieldParse {
    fn default() -> Self {
        Self {
            super_: BaseParser::default(),
            fieldValue: Some(String::new()),
            mWordFormatter: Some(Box::new(NumFmtFactory::createWordFormatter(0))),
        }
    }
}

// java public abstract class NumFieldParse extends BaseParser
impl NumFieldParse {
    // java public void numparse(char[] codeBuffer)
    pub fn numparse(&mut self, codeBuffer: &Vec<char>) {
      // java this.mCodeLex = new FieldCodeLex();
      self.super_.mCodeLex = Some(FieldCodeLex::default());
      // java this.registerSwitch();
      self.registerSwitch();
      // java this.mCodeLex.parse(codeBuffer, codeBuffer.length);
      if let Some(ref mut lex) = self.super_.mCodeLex {
        lex.parse(codeBuffer, codeBuffer.len() as i32);
      }
      // java long fieldNum = (long)this.setFieldValue();
      let mut fieldNum: i64 = self.setFieldValue() as i64;
      // java this.fieldValue = Long.toString(fieldNum);
      self.fieldValue = Some(fieldNum.to_string());
      // java int formatCount = this.mCodeLex.getFormatCount();
      let mut formatCount: i32 = self.super_.mCodeLex.as_ref().map(|l| l.getFormatCount()).unwrap_or(0);
      // java if (formatCount > 0)
      if formatCount > 0 {
        // java this.fieldValue = StrFormatUtil.renderByFormat(this.mCodeLex, fieldNum);
        self.fieldValue = StrFormatUtil::renderByFormat(
          self.super_.mCodeLex.as_ref(),
          Some(&fieldNum as &dyn Object),
        );
      }
      // java else
      else {
        // java char[] ch = new char[]{'\u0000'};
        let mut ch: Vec<char> = vec!['\u{0000}'];
        // java String pictureFormat = this.mCodeLex.getPicture(ch);
        let mut pictureFormat: String = self.super_.mCodeLex.as_ref().map(|l| l.getPicture(Some(&mut ch))).unwrap_or_default();
        // java if (!"".equals(pictureFormat))
        if pictureFormat.as_str() != "" {
          // java this.fieldValue = StrFormatUtil.numformat_2(this.fieldValue, pictureFormat);
          let cur = self.fieldValue.clone().unwrap_or_default();
          self.fieldValue = Some(StrFormatUtil::numformat_2(&cur, &pictureFormat));
        }
      }
      // java this.mResult = new StringBuffer(this.fieldValue);
      self.super_.mResult = Some(StringBuffer::new__string(self.fieldValue.as_deref().unwrap_or("")));
    }

    /// Shared numparse that uses an externally computed field value (subclass setFieldValue).
    /// Creates a fresh FieldCodeLex and parses — does not call subclass registerSwitch.
    pub fn numparse_with_value(&mut self, codeBuffer: &Vec<char>, field_num: i32) {
      self.super_.mCodeLex = Some(FieldCodeLex::default());
      if let Some(ref mut lex) = self.super_.mCodeLex {
        lex.parse(codeBuffer, codeBuffer.len() as i32);
      }
      self.format_field_num(field_num);
    }

    /// Format `field_num` into mResult using the current mCodeLex (must already be parsed).
    pub fn format_field_num(&mut self, field_num: i32) {
      let mut fieldNum: i64 = field_num as i64;
      self.fieldValue = Some(fieldNum.to_string());
      let mut formatCount: i32 = self.super_.mCodeLex.as_ref().map(|l| l.getFormatCount()).unwrap_or(0);
      if formatCount > 0 {
        self.fieldValue = StrFormatUtil::renderByFormat(
          self.super_.mCodeLex.as_ref(),
          Some(&fieldNum as &dyn Object),
        );
      } else {
        let mut ch: Vec<char> = vec!['\u{0000}'];
        let mut pictureFormat: String = self.super_.mCodeLex.as_ref().map(|l| l.getPicture(Some(&mut ch))).unwrap_or_default();
        if pictureFormat.as_str() != "" {
          let cur = self.fieldValue.clone().unwrap_or_default();
          self.fieldValue = Some(StrFormatUtil::numformat_2(&cur, &pictureFormat));
        }
      }
      self.super_.mResult = Some(StringBuffer::new__string(self.fieldValue.as_deref().unwrap_or("")));
    }

    // java public abstract int setFieldValue()
    pub fn setFieldValue(&mut self) -> i32 {
      0
    }

    // java public abstract void registerSwitch()
    pub fn registerSwitch(&mut self) {
    }

}

impl std::ops::Deref for NumFieldParse {
    type Target = BaseParser;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for NumFieldParse {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}
