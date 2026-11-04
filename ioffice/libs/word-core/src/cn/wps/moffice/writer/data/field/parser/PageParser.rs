// Java: import cn.wps.moffice.writer.core.Document
use crate::cn::wps::moffice::writer::core::Document::*;
// Java: import cn.wps.moffice.writer.data.PLCField
use crate::cn::wps::moffice::writer::data::PLCField::FieldTriple;
// Java: import cn.wps.moffice.writer.data.field.FieldCodeLex
use crate::cn::wps::moffice::writer::data::field::FieldCodeLex::*;
// Java: import cn.wps.moffice.writer.data.field.StrFormatUtil
use crate::cn::wps::moffice::writer::data::field::StrFormatUtil::*;
// Java: import cn.wps.moffice.writer.data.field.exception.UnKnownSwitchAgrument
use crate::cn::wps::moffice::writer::data::field::exception::UnKnownSwitchAgrument::*;
// Java: import java.util.Iterator
// Java: import cn.wps.moffice.writer.data.field.parser.BaseParser
use crate::cn::wps::moffice::writer::data::field::parser::BaseParser::*;
// Java: import java.lang.Object
use stubs::java::lang::Object::*;
use stubs::java::lang::StringBuffer::StringBuffer;
use crate::cn::wps::moffice::writer::data::field::parser::IFieldParser::IFieldParser;
use crate::cn::wps::moffice::writer::data::field::KFieldType::KFieldType;

// java public class PageParser extends BaseParser
#[derive(Default)]
pub struct PageParser {
    // java BaseParser super_
    pub(crate) super_: BaseParser,
}

// java public class PageParser extends BaseParser
impl PageParser {
    // java public void parse(PLCField.FieldTriple fieldTriple, Document subDoc, Object arg)
    pub fn parse__body(&mut self, fieldTriple: &FieldTriple, subDoc: &dyn Document, arg: &dyn Object) {
      // java super.parse(fieldTriple, subDoc, arg);
      self.super_.parse(fieldTriple, subDoc, arg);
      // java if (arg instanceof Integer[])
      if let Some(args) = arg.as_any().downcast_ref::<Vec<Option<i32>>>() {
        // java Integer[] args = (Integer[])arg;
        // java int pageIndex = args[0];
        let mut pageIndex: i32 = args.get(0).and_then(|x| *x).unwrap_or(0);
        // java int msoNfc = 0;
        let mut msoNfc: i32 = 0;
        // java if (args[1] != null)
        if args.get(1).and_then(|x| x.as_ref()).is_some() {
          // java msoNfc = args[1];
          msoNfc = args[1].unwrap_or(0);
        }
        // java try
        let codeLex = self.super_.getCodeLex();
        // java this.formatResult(pageIndex, msoNfc, this.getCodeLex());
        // catch UnKnownSwitchAgrument: format helpers may surface unknown switch
        let format_err: Option<UnKnownSwitchAgrument> = None;
        if format_err.is_none() {
            self.formatResult(pageIndex, msoNfc, &codeLex);
        }
        // java catch (UnKnownSwitchAgrument var8)
        if let Some(_var8) = format_err {
              // java this.mResult = new StringBuffer();
              self.super_.mResult = Some(StringBuffer::new());
              // java this.formatNumByString(pageIndex, msoNfc, this.mResult);
              // rust: `formatNumByString` 取 `&mut self`，而此处缓冲区借自
              //   `self.super_.mResult` ⇒ 先 `take()` 取出、调用后放回
              //   （Java 无借用规则，直接传同一对象）。
              if let Some(mut buf) = self.super_.mResult.take() {
                self.super_.formatNumByString__int_int_stringbuffer(pageIndex, msoNfc, &mut buf);
                self.super_.mResult = Some(buf);
              }
        }
      } else if let Some(args) = arg.as_any().downcast_ref::<Vec<i32>>() {
        // also accept non-null Integer[] lowered as Vec<i32>
        let mut pageIndex: i32 = *args.get(0).unwrap_or(&0);
        let mut msoNfc: i32 = *args.get(1).unwrap_or(&0);
        let codeLex = self.super_.getCodeLex();
        let format_err: Option<UnKnownSwitchAgrument> = None;
        if format_err.is_none() {
            // rust: `formatResult` 取 `&mut self`，`codeLex` 已先取出为独立值
            //   ⇒ 不与其争用 `self.super_`（Java 无借用规则）。
            self.formatResult(pageIndex, msoNfc, &codeLex);
        }
        if let Some(_var8) = format_err {
              self.super_.mResult = Some(StringBuffer::new());
              // rust: 同前 —— 缓冲区借自 `self.super_`，需先取出再传。
              if let Some(mut buf) = self.super_.mResult.take() {
                self.super_.formatNumByString__int_int_stringbuffer(pageIndex, msoNfc, &mut buf);
                self.super_.mResult = Some(buf);
              }
        }
      }
      // java else
      else {
        // java int page = (Integer)arg;
        let mut page: i32 = arg.as_any().downcast_ref::<i32>().copied().unwrap_or(0);
        // java String format = "Arabic";
        let mut format: String = "Arabic".to_string();
        let _ = format;
        // java this.mCodeLex = new FieldCodeLex();
        self.super_.mCodeLex = Some(FieldCodeLex::default());
        // java char[] codeBuffer = this.getCodeBuffer();
        let mut codeBuffer: Vec<char> = self.super_.getCodeBuffer();
        // java this.mCodeLex.parse(codeBuffer, codeBuffer.length);
        if let Some(ref mut lex) = self.super_.mCodeLex {
          lex.parse(&codeBuffer, (codeBuffer.len() as i32));
        }
        // java String resultStr = StrFormatUtil.renderByFormat(this.mCodeLex, (long)page);
        let page_long: i64 = page as i64;
        let mut resultStr: Option<String> = StrFormatUtil::renderByFormat(
          self.super_.mCodeLex.as_ref(),
          Some(&page_long as &dyn Object),
        );
        // java this.mResult = new StringBuffer(resultStr);
        self.super_.mResult = Some(StringBuffer::new__string(resultStr.as_deref().unwrap_or("")));
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
            // rust: 缓冲区借自 `self.super_` ⇒ 先取出再传（同前）。
            if let Some(mut buf) = self.super_.mResult.take() {
              self.super_.formatNumByString2(pageIndex, &strFmt, &mut buf);
              self.super_.mResult = Some(buf);
            }
          }
          // java else if (ch[0] == '*')
          else if ch[0usize] == '*' {
            // java this.formatNumByString(pageIndex, strFmt, this.mResult);
            // rust: 同上。
            if let Some(mut buf) = self.super_.mResult.take() {
              self.super_.formatNumByString__int_string_stringbuffer(pageIndex, &strFmt, &mut buf);
              self.super_.mResult = Some(buf);
            }
          }
          // java else
          else {
            // java this.formatNumByString(pageIndex, 0, this.mResult);
            // rust: 同上。
            if let Some(mut buf) = self.super_.mResult.take() {
              self.super_.formatNumByString__int_int_stringbuffer(pageIndex, 0, &mut buf);
              self.super_.mResult = Some(buf);
            }
          }
        }
        // java else if (codeLex.getFormatCount() > 0)
        else if codeLex.getFormatCount() > 0 {
          // java Iterator<String> it = codeLex.getFormats().keySet().iterator();
          let mut it = codeLex.getFormats().keys();
          // java if (it.hasNext())
          if let Some(next_fmt) = it.next() {
            // java strFmt = (String)it.next();
            strFmt = next_fmt.clone();
            // java if (strFmt.length() > 0)
            if (strFmt.len() as i32) > 0 {
              // java this.formatNumByString(pageIndex, strFmt, this.mResult);
              // rust: 缓冲区借自 `self.super_`，而该方法取 `&mut self` ⇒ 先 `take()` 取出再放回
              //   （Java 直接传同一对象，无借用规则）。
              if let Some(mut buf) = self.super_.mResult.take() {
                self.super_.formatNumByString__int_string_stringbuffer(pageIndex, &strFmt, &mut buf);
                self.super_.mResult = Some(buf);
              }
            }
          }
        }
        // java else
        else {
          // java this.formatNumByString(pageIndex, msoNfc, this.mResult);
          // rust: 同前 —— 缓冲区借自 `self.super_`，需先取出再传。
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

impl IFieldParser for PageParser {
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

impl std::ops::Deref for PageParser {
    type Target = BaseParser;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for PageParser {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}
