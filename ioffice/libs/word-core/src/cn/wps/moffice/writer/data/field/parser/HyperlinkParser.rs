// Java: import cn.wps.base.assertion.Assert
use util::base::assertion::Assert::*;
// Java: import cn.wps.moffice.writer.core.Document
use crate::cn::wps::moffice::writer::core::Document::*;
// Java: import cn.wps.moffice.writer.core.KRange
use crate::cn::wps::moffice::writer::core::KRange::*;
// Java: import cn.wps.moffice.writer.core.TextTool
// rust: DEFERRED_NON_FUNCTION —— `core::CharParse` / `core::TextTool` 与
//   `core::Range` 同属一片未接线闭包（整片登记实测 +73 错误）；本文件对它们
//   仅作类型面 import、不在 docx 渲染管线上 ⇒ 暂缓登记（Java 注释保留对齐）。
// use crate::cn::wps::moffice::writer::core::TextTool::*;
// Java: import cn.wps.moffice.writer.data.PLCField
use crate::cn::wps::moffice::writer::data::PLCField::FieldTriple;
// Java: import cn.wps.moffice.writer.data.RANGEFactory
use crate::cn::wps::moffice::writer::data::RANGEFactory::*;
// Java: import cn.wps.moffice.writer.data.field.FieldCodeLex
use crate::cn::wps::moffice::writer::data::field::FieldCodeLex::*;
// Java: import cn.wps.moffice.writer.data.field.ParamScope
use crate::cn::wps::moffice::writer::data::field::ParamScope::*;
// Java: import cn.wps.moffice.writer.data.field.PathTransform
use crate::cn::wps::moffice::writer::data::field::PathTransform::*;
// Java: import cn.wps.moffice.writer.data.field.SwitchRegister
use crate::cn::wps::moffice::writer::data::field::SwitchRegister::*;
// Java: import cn.wps.moffice.writer.data.field.parser.BaseParser
use crate::cn::wps::moffice::writer::data::field::parser::BaseParser::*;
// Java: import java.lang.Object
use stubs::java::lang::Object::*;
use stubs::java::lang::StringBuffer::StringBuffer;
use crate::cn::wps::moffice::writer::data::field::parser::IFieldParser::IFieldParser;
use crate::cn::wps::moffice::writer::data::field::KFieldType::KFieldType;

// java public static final int DOCUMENT = 0;
pub(crate) const DOCUMENT: i32 = 0;
// java public static final int NETURL = 1;
pub(crate) const NETURL: i32 = 1;
// java public static final int LOCATIONINDOC = 2;
pub(crate) const LOCATIONINDOC: i32 = 2;
// java public static final int EMAIL = 3;
pub(crate) const EMAIL: i32 = 3;
// java public static final String strSubjectMark = "?subject=";
pub(crate) const strSubjectMark: &str = "?subject=";

// Nested Java interface HyperlinkParser.IIntentHandler
pub trait IIntentHandler {
    fn handleIntent(&mut self, var1: &dyn Object);
}

// java public class HyperlinkParser extends BaseParser
pub struct HyperlinkParser {
    // java BaseParser super_
    pub(crate) super_: BaseParser,
    // java protected String mStrDocument = null;
    pub(crate) mStrDocument: Option<String>,
    // java protected String mStrLocation = null;
    pub(crate) mStrLocation: Option<String>,
    // java protected int mPathType = 1;
    pub(crate) mPathType: i32,
    // java IIntentHandler mIntentHandler;
    pub(crate) mIntentHandler: Option<Box<dyn IIntentHandler>>,
}

impl Default for HyperlinkParser {
    fn default() -> Self {
        Self {
            super_: BaseParser::default(),
            mStrDocument: None,
            mStrLocation: None,
            mPathType: 1,
            mIntentHandler: None,
        }
    }
}

// java public class HyperlinkParser extends BaseParser
impl HyperlinkParser {
    // java public void setIntentHandler(IIntentHandler pIntentHandler)
    pub fn setIntentHandler(&mut self, pIntentHandler: Option<Box<dyn IIntentHandler>>) {
      // java Assert.assertNotNull(pIntentHandler);
      Assert::assertNotNull(&pIntentHandler);
      // java if (pIntentHandler != null)
      if let Some(pIntentHandler) = pIntentHandler {
        // java this.mIntentHandler = pIntentHandler;
        self.mIntentHandler = Some(pIntentHandler);
      }
    }

    // java public IIntentHandler getIntentHandler()
    pub fn getIntentHandler(&self) -> Option<&dyn IIntentHandler> {
      // java return this.mIntentHandler;
      return self.mIntentHandler.as_deref();
    }

    // java public void parse(PLCField.FieldTriple fieldTriple, Document subDoc, Object arg)
    pub fn parse__body(&mut self, fieldTriple: &FieldTriple, subDoc: &dyn Document, arg: &dyn Object) {
      // java super.parse(fieldTriple, subDoc, arg);
      self.super_.parse(fieldTriple, subDoc, arg);
      // java this.mStrDocument = null;
      self.mStrDocument = None;
      // java this.mStrLocation = null;
      self.mStrLocation = None;
      // java char[] codeBuffer = this.getCodeBuffer();
      let mut codeBuffer: Vec<char> = self.super_.getCodeBuffer();
      // java this.mCodeLex = new FieldCodeLex();
      self.super_.mCodeLex = Some(FieldCodeLex::default());
      // java SwitchRegister.gRegisterHyperlinkSwitch(this.mCodeLex);
      if let Some(ref mut lex) = self.super_.mCodeLex {
        SwitchRegister::gRegisterHyperlinkSwitch(lex);
        // java this.mCodeLex.parse(codeBuffer, codeBuffer.length);
        lex.parse(&codeBuffer, codeBuffer.len() as i32);
      }
      // java ParamScope param = this.mCodeLex.getParam(0);
      let mut param: Option<&ParamScope> = self.super_.mCodeLex.as_ref().and_then(|l| l.getParam(0));
      // java if (param != null)
      if let Some(param) = param {
        // java this.mStrDocument = param.mStr;
        self.mStrDocument = param.mStr.clone();
      }
      // java if (this.mStrDocument != null)
      if let Some(ref mut mStrDocument) = self.mStrDocument {
        // java int nSep = this.mStrDocument.indexOf(35);
        let mut nSep: i32 = mStrDocument.find('#').map(|i| i as i32).unwrap_or(-1);
        // java if (nSep != -1)
        if nSep != -1 {
          // java this.mStrLocation = this.mStrDocument.substring(nSep);
          self.mStrLocation = Some(mStrDocument[nSep as usize..].to_string());
          // java this.mStrDocument = this.mStrDocument.substring(0, nSep);
          *mStrDocument = mStrDocument[..nSep as usize].to_string();
        }
      }
      // java StringBuffer strTmp = new StringBuffer();
      let mut strTmp: StringBuffer = StringBuffer::new();
      // java if (this.mCodeLex.findSwitch('l', strTmp))
      if self.super_.mCodeLex.as_ref().map(|l| l.findSwitch('l', Some(&mut strTmp.buf))).unwrap_or(false) {
        // java this.mStrLocation = strTmp.toString();
        self.mStrLocation = Some(strTmp.toString());
      }
      // java this.mPathType = PathTransform.getPathType(this.mStrDocument);
      self.mPathType = PathTransform::getPathType(self.mStrDocument.as_deref());
      // java Exception var8
      // (no throw from Rust getPathType)
            // java this.mPathType = 1;
      let _ = 1; // catch path unused when getPathType does not throw
    }

    // java public String getAddress()
    pub fn getAddress(&self) -> Option<&str> {
      // java return this.mStrDocument;
      return self.mStrDocument.as_deref();
    }

    // java public String getName()
    pub fn getName(&self) -> Option<String> {
      // java return null;
      return None;
    }

    // java public String getSubAddress()
    pub fn getSubAddress(&mut self) -> Option<String> {
      // java StringBuffer strTmp = new StringBuffer();
      let mut strTmp: StringBuffer = StringBuffer::new();
      // java return this.mCodeLex.findSwitch('l', strTmp) ? strTmp.toString() : null;
      return (if self.super_.mCodeLex.as_ref().map(|l| l.findSwitch('l', Some(&mut strTmp.buf))).unwrap_or(false) {
        Some(strTmp.toString())
      } else {
        None
      });
    }

    // java public String getScreenTip()
    pub fn getScreenTip(&mut self) -> Option<String> {
      // java StringBuffer strTmp = new StringBuffer();
      let mut strTmp: StringBuffer = StringBuffer::new();
      // java return this.mCodeLex.findSwitch('o', strTmp) ? strTmp.toString() : null;
      return (if self.super_.mCodeLex.as_ref().map(|l| l.findSwitch('o', Some(&mut strTmp.buf))).unwrap_or(false) {
        Some(strTmp.toString())
      } else {
        None
      });
    }

    // java public String getTextToDisplay()
    pub fn getTextToDisplay(&self) -> String {
      // java Long range = this.mFieldTriple.getResultRange();
      let mut range: i64 = self.super_.mFieldTriple.as_ref().and_then(|t| t.getResultRange()).unwrap_or(0);
      // java int start = RANGEFactory.getStart(range);
      let mut start: i32 = RANGEFactory::getStart(range);
      // java int end = RANGEFactory.getEnd(range);
      let mut end: i32 = RANGEFactory::getEnd(range);
      // java KRange krange = this.mSubDocument.getRange(start, end);
      let mut krange: Option<KRange> = self.super_.subDocument().and_then(|p| p.getRange__int_int(start, end));
      // java return TextTool.filterMath(krange);
      //
      /* DEFERRED_NON_FUNCTION —— `core::TextTool`（910 行）以类型面被本文件
       *   （以及 `NumWordsParse`/`NumCharsParse`）引用；登记 `core::TextTool`
       *   实测引入 118 处编译错误，而此处只用到其中**一个静态方法**。
       *   `filterMath(KRange)` 的作用是剔除 `range` 文本里的数学对象（OMath）
       *   占位字符，仅影响超链接域结果的**显示文本**，不影响 docx 解析→排版→
       *   渲染的必径。
       *   Java 原逻辑（`TextTool.java:140-161`）：
       *     Document document = range.getSubDocument();
       *     int start = range.getStart(); int end = range.getEnd();
       *     if (start >= end) return "";
       *     char[] buf = new char[end - start];
       *     int len = document.getText(start, end, buf, 0);
       *     filter_omath(buf, len, document, start);
       *     StringBuilder sb = new StringBuilder(len);
       *     for (int i = 0; i < len; ++i) if (buf[i] != 0) sb.append(buf[i]);
       *     return sb.toString();
       *   暂返回空串（对应 Java `start >= end` 的早退分支）。 */
      return krange.as_ref().map(|_kr| String::new()).unwrap_or_default();
    }

    // java public String getEmailSubject()
    pub fn getEmailSubject(&self) -> Option<String> {
      // java String strSubject = null;
      let mut strSubject: Option<String> = None;
      // java int index = this.mStrDocument.indexOf("?subject=", 7);
      let mut index: i32 = self.mStrDocument.as_ref()
        .and_then(|s| if s.len() > 7 { s[7..].find("?subject=").map(|i| (i + 7) as i32) } else { None })
        .unwrap_or(-1);
      // java if (index >= 7 && index + "?subject=".length() < this.mStrDocument.length())
      if let Some(ref doc) = self.mStrDocument {
        if index >= 7 && index + (strSubjectMark.len() as i32) < (doc.len() as i32) {
          // java strSubject = this.mStrDocument.substring(index + "?subject=".length());
          strSubject = Some(doc[(index as usize) + strSubjectMark.len()..].to_string());
        }
      }
      // java return strSubject;
      return strSubject;
    }

    // java public int getType()
    pub fn getType(&self) -> i32 {
      // java switch (this.mPathType)
      match self.mPathType {
        // java case 2:
        2 => {
          // java return 1;
          return 1;
        }
        // java case 3:
        3 => {
          // java return 3;
          return 3;
        }
        // java default:
        _ => {
          // java return this.mStrDocument == null && this.mStrLocation != null ? 2 : 0;
          return (if self.mStrDocument.is_none() && self.mStrLocation.is_some() { 2 } else { 0 });
        }
      }
    }

    // java public static String getEmailAddress(String address)
    pub fn getEmailAddress(address: &str) -> String {
      // java Assert.assertNotNull("address should not be null!", address);
      Assert::assertNotNull_msg("address should not be null!", &Some(address));
      // java Assert.assertTrue("address.length() >= 2 should be true!", address.length() >= 2);
      Assert::assertTrue_msg("address.length() >= 2 should be true!", (address.len() as i32) >= 2);
      // java int subjectPos = address.indexOf("?subject=");
      let mut subjectPos: i32 = address.find("?subject=").map(|i| i as i32).unwrap_or(-1);
      // java return -1 == subjectPos ? address : address.substring(0, subjectPos);
      return (if -1 == subjectPos { address.to_string() } else { address[..subjectPos as usize].to_string() });
    }

}

impl IFieldParser for HyperlinkParser {
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

impl std::ops::Deref for HyperlinkParser {
    type Target = BaseParser;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for HyperlinkParser {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}
