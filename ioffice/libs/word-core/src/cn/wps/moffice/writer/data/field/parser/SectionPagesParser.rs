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

// java public class SectionPagesParser extends BaseParser
#[derive(Default)]
pub struct SectionPagesParser {
    // java BaseParser super_
    pub(crate) super_: BaseParser,
}

// java public class SectionPagesParser extends BaseParser
impl SectionPagesParser {
    // java public void parse(PLCField.FieldTriple fieldTriple, Document subDoc, Object arg)
    pub fn parse__body(&mut self, fieldTriple: &FieldTriple, subDoc: &dyn Document, arg: &dyn Object) {
      // java super.parse(fieldTriple, subDoc, arg);
      self.super_.parse(fieldTriple, subDoc, arg);
      // java if (arg instanceof Integer[])
      if let Some(args) = arg.as_any().downcast_ref::<Vec<Option<i32>>>() {
        // java Integer[] args = (Integer[])arg;
        // java int pageCount = args[0];
        let mut pageCount: i32 = args.get(0).and_then(|x| *x).unwrap_or(0);
        // java int msoNfc = 0;
        let mut msoNfc: i32 = 0;
        // java if (args[1] != null)
        if args.get(1).and_then(|x| x.as_ref()).is_some() {
          // java msoNfc = args[1];
          msoNfc = args[1].unwrap_or(0);
        }
        // java this.formatResult(pageCount, msoNfc, this.getCodeLex());
        let codeLex = self.super_.getCodeLex();
        self.formatResult(pageCount, msoNfc, &codeLex);
      } else if let Some(args) = arg.as_any().downcast_ref::<Vec<i32>>() {
        let mut pageCount: i32 = *args.get(0).unwrap_or(&0);
        let mut msoNfc: i32 = *args.get(1).unwrap_or(&0);
        let codeLex = self.super_.getCodeLex();
        self.formatResult(pageCount, msoNfc, &codeLex);
      }
      // java else
      else {
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
        // java int sectionCount = (Integer)arg;
        let mut sectionCount: i32 = arg.as_any().downcast_ref::<i32>().copied().unwrap_or(0);
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
          let section_long: i64 = sectionCount as i64;
          resultStr = StrFormatUtil::renderByFormat(
            self.super_.mCodeLex.as_ref(),
            Some(&section_long as &dyn Object),
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

}

impl IFieldParser for SectionPagesParser {
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

impl std::ops::Deref for SectionPagesParser {
    type Target = BaseParser;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for SectionPagesParser {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}
