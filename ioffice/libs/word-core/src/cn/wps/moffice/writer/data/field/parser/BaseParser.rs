// Java: import cn.wps.moffice.writer.core.Document
use crate::cn::wps::moffice::writer::core::Document::*;
// Java: import cn.wps.moffice.writer.core.TextDocument
use crate::cn::wps::moffice::writer::core::TextDocument::{self as TextDocumentMod, TextDocument};
// Java: import cn.wps.moffice.writer.core.metadata.Metadata
use crate::cn::wps::moffice::writer::core::metadata::Metadata::*;
// Java: import cn.wps.moffice.writer.core.metadata.PIDSI
use crate::cn::wps::moffice::writer::core::metadata::PIDSI::*;
// Java: import cn.wps.moffice.writer.data.PLCField
use crate::cn::wps::moffice::writer::data::PLCField::{FieldTriple, FieldNode};
// Java: import cn.wps.moffice.writer.data.RANGEFactory
use crate::cn::wps::moffice::writer::data::RANGEFactory::*;
// Java: import cn.wps.moffice.writer.data.field.FieldCodeLex
use crate::cn::wps::moffice::writer::data::field::FieldCodeLex::*;
// Java: import cn.wps.moffice.writer.data.field.KFieldType
use crate::cn::wps::moffice::writer::data::field::KFieldType::*;
// Java: import cn.wps.moffice.writer.data.field.StrFormatUtil
use crate::cn::wps::moffice::writer::data::field::StrFormatUtil::*;
// Java: import cn.wps.moffice.writer.data.field.parser.IFieldParser
use crate::cn::wps::moffice::writer::data::field::parser::IFieldParser::*;
// Java: import java.lang.Object
use stubs::java::lang::Object::*;
use stubs::java::lang::StringBuilder::StringBuilder;
use stubs::java::lang::StringBuffer::StringBuffer;

// java public class BaseParser implements IFieldParser
#[derive(Default)]
pub struct BaseParser {
    // java protected FieldCodeLex mCodeLex = null;
    pub(crate) mCodeLex: Option<FieldCodeLex>,
    // java protected PLCField.FieldTriple mFieldTriple = null;
    pub(crate) mFieldTriple: Option<FieldTriple>,
    // java protected Document mSubDocument = null;
    // L4: Java stores Document ref — match CPieceTable / PLC holder pattern
    // rust: 存 `*mut (dyn Document + 'static)` —— 仓内既有写法
    //   （`data::PLC.rs:32` 的 `mDocument`），`+ 'static` 显式标注使裸指针
    //   不携带调用方生命周期（Java 直接存引用，无生命周期概念）。
    pub(crate) mSubDocument: Option<*mut (dyn Document + 'static)>,
    // java protected StringBuffer mResult = null;
    pub(crate) mResult: Option<StringBuffer>,
    // java protected boolean reLayout = false;
    pub(crate) reLayout: bool,
    // java protected PIDSI metaProperty = null;
    pub(crate) metaProperty: Option<PIDSI>,
}

// rust-only: 把任意生命周期的 `&dyn Document` 转成不带生命周期的裸指针，
//   供 `mSubDocument` 字段保存（Java 无生命周期概念，直接存引用）。
pub(crate) fn erase_lifetime<'a>(d: &'a dyn Document) -> *mut (dyn Document + 'static) {
  // rust: `*mut (dyn Document + 'static)` 要求 `'static`；Java 直接存引用、
  //   无生命周期概念 ⇒ 先把胖指针拆成 `(数据指针, vtable 指针)` 再重组，
  //   从而绕过赋值处的 `'static` 推导（与仓内裸指针持有语义一致）。
  unsafe {
    let raw: *const dyn Document = d as *const dyn Document;
    let (data, meta): (*const (), *const ()) = std::mem::transmute(raw);
    std::mem::transmute::<(*const (), *const ()), *mut (dyn Document + 'static)>((data, meta))
  }
}

// java public interface IFieldParser
impl IFieldParser for BaseParser {
    // java public void parse(PLCField.FieldTriple fieldTriple, Document subDoc, Object arg)
    fn parse(&mut self, fieldTriple: &FieldTriple, subDoc: &dyn Document, _arg: &dyn Object) {
      // java this.mFieldTriple = fieldTriple;
      self.mFieldTriple = Some(fieldTriple.clone());
      // java this.mSubDocument = subDoc;
      // rust: `*mut dyn Document` 隐含 `'static`，而 `subDoc: &dyn Document` 带生命周期
      //   （Java 无此区分，直接存引用）⇒ 经裸指针消除生命周期约束。
      self.mSubDocument = Some(erase_lifetime(subDoc));
      // java this.setMetaProperty();
      self.setMetaProperty();
    }

    // java public void parseByCharBuffer(char[] codeBuffer, Document subDoc, KFieldType fieldType)
    fn parseByCharBuffer(&mut self, _codeBuffer: &[char], subDoc: &dyn Document, _fieldType: KFieldType) {
      // java this.mSubDocument = subDoc;
      // rust: `*mut dyn Document` 隐含 `'static`，而 `subDoc: &dyn Document` 带生命周期
      //   （Java 无此区分，直接存引用）⇒ 经裸指针消除生命周期约束。
      self.mSubDocument = Some(erase_lifetime(subDoc));
      // java this.setMetaProperty();
      self.setMetaProperty();
    }

    // java public boolean isReLayout()
    fn isReLayout(&self) -> bool {
      // java return this.reLayout;
      return self.reLayout;
    }

    // java public void dispose()
    fn dispose(&mut self) {
      // java this.mResult = null;
      self.mResult = None;
      // java this.mCodeLex = null;
      self.mCodeLex = None;
      // java this.mSubDocument = null;
      self.mSubDocument = None;
      // java this.mFieldTriple = null;
      self.mFieldTriple = None;
      // java this.metaProperty = null;
      self.metaProperty = None;
    }

    // java public String getResult()
    fn getResult(&self) -> Option<String> {
      // java return this.mResult != null ? this.mResult.toString() : null;
      return (if self.mResult.is_some() { Some(self.mResult.as_ref().unwrap().toString()) } else { None });
    }

    // java public Long getResultRange()
    fn getResultRange(&self) -> Option<i64> {
      // java return null;
      return None;
    }

}

// java public class BaseParser implements IFieldParser
impl BaseParser {
    // java public PLCField.FieldTriple getFieldTriple()
    pub fn getFieldTriple(&self) -> Option<FieldTriple> {
      // java return this.mFieldTriple;
      return self.mFieldTriple.clone();
    }

    // java public Document getDocument()
    pub fn getDocument(&self) -> Option<&dyn Document> {
      // java return this.mSubDocument;
      // rust: 从擦除指针还原 `&dyn Document`（见字段声明处说明）。
      self.mSubDocument.map(|p| unsafe { &*p })
    }

    // rust-only: 供子类（各 field parser）读取 `mSubDocument` 的类型化访问器，
    //   避免每个调用点重复写裸指针还原（字段已擦除为 `*mut ()`）。
    pub(crate) fn subDocument(&self) -> Option<&dyn Document> {
      self.mSubDocument.map(|p| unsafe { &*p })
    }

    // java protected FieldCodeLex getCodeLex()
    pub fn getCodeLex(&self) -> FieldCodeLex {
      // java char[] result = this.getCodeBuffer();
      let mut result: Vec<char> = self.getCodeBuffer();
      // java FieldCodeLex codeLex = new FieldCodeLex();
      let mut codeLex: FieldCodeLex = FieldCodeLex::default();
      // java if (result.length > 0)
      if (result.len() as i32) > 0 {
        // java codeLex.parse(result, result.length);
        codeLex.parse(&result, (result.len() as i32));
      }
      // java return codeLex;
      return codeLex;
    }

    // java protected char[] getCodeBuffer()
    pub fn getCodeBuffer(&self) -> Vec<char> {
      // java long range = this.mFieldTriple.getCodeRange();
      let mut range: i64 = self.mFieldTriple.as_ref().map(|t| t.getCodeRange()).unwrap_or(0);
      // java int start = RANGEFactory.getStart(range);
      let mut start: i32 = RANGEFactory::getStart(range);
      // java int end = RANGEFactory.getEnd(range);
      let mut end: i32 = RANGEFactory::getEnd(range);
      // java char[] buf = new char[end - start];
      let mut buf: Vec<char> = vec![Default::default(); ((end - start).max(0)) as usize];
      // java this.mSubDocument.getText(start, end, buf, 0);
      if let Some(doc) = self.getDocument() {
        doc.getText(start, end, &mut buf, 0);
      }
      // java return buf;
      return buf;
    }

    // java protected boolean formatNumByString2(int nVal, String szFormat, StringBuffer strResult)
    pub fn formatNumByString2(&mut self, nVal: i32, szFormat: &str, strResult: &mut StringBuffer) -> bool {
      // java StringBuilder strFormat = new StringBuilder();
      let mut strFormat: StringBuilder = StringBuilder::new();
      // java int nQuoteCount = 0;
      let mut nQuoteCount: i32 = 0;
      // java int i = 0;
      let mut i: i32 = 0;
      let mut nLen: i32 = (szFormat.len() as i32);
      // java for(int nLen = szFormat.length(); i < nLen; ++i)
      while i < nLen {
        // java if (szFormat.charAt(i) == '\'')
        if szFormat.chars().nth(i as usize).unwrap_or('\0') == '\'' {
          // java ++nQuoteCount;
          nQuoteCount += 1;
        }
        // java else
        else {
          // java strFormat.append(szFormat.charAt(i));
          strFormat.append(szFormat.chars().nth(i as usize).unwrap_or('\0'));
        }
        i += 1;
      }
      // java if (nQuoteCount % 2 != 0)
      if nQuoteCount % 2 != 0 {
        // java strResult.setLength(0);
        strResult.setLength(0);
        // java return false;
        return false;
      }
      // java else
      else {
        // java i = StrFormatUtil.txNfcString2MsoNfc(strFormat.toString());
        i = StrFormatUtil::txNfcString2MsoNfc(&strFormat.toString());
        // java return StrFormatUtil.txFormatNum(nVal, i, strResult);
        return StrFormatUtil::txFormatNum__int_int_stringbuffer(nVal, i, Some(strResult));
      }
    }

    // java protected boolean formatNumByString(int nVal, String strNfc, StringBuffer strResult)
    pub fn formatNumByString__int_string_stringbuffer(&mut self, nVal: i32, strNfc: &str, strResult: &mut StringBuffer) -> bool {
      // java int msoNfc = StrFormatUtil.txNfcString2MsoNfc(strNfc);
      let mut msoNfc: i32 = StrFormatUtil::txNfcString2MsoNfc(strNfc);
      // java return StrFormatUtil.txFormatNum(nVal, msoNfc, strResult);
      return StrFormatUtil::txFormatNum__int_int_stringbuffer(nVal, msoNfc, Some(strResult));
    }

    // java protected boolean formatNumByString(int nVal, int msoNfc, StringBuffer strResult)
    pub fn formatNumByString__int_int_stringbuffer(&mut self, nVal: i32, msoNfc: i32, strResult: &mut StringBuffer) -> bool {
      // java return StrFormatUtil.txFormatNum(nVal, msoNfc, strResult);
      return StrFormatUtil::txFormatNum__int_int_stringbuffer(nVal, msoNfc, Some(strResult));
    }

    // java private void setMetaProperty()
    pub fn setMetaProperty(&mut self) {
      // java if (this.mSubDocument != null)
      if let Some(doc) = self.getDocument() {
        // java TextDocument tdox = this.mSubDocument.getTextDocument();
        let tdox = doc.getTextDocument();
        // java if (tdox != null)
        if let Some(tdox) = tdox {
          // java Metadata metaData = tdox.getMetadata();
          let mut metaData: Metadata = tdox.getMetadata();
          // java if (metaData != null)
          // Metadata is a concrete struct from getMetadata
          {
            // java this.metaProperty = metaData.getPIDSI();
            self.metaProperty = Some(metaData.getPIDSI());
          }
        }
      }
    }

}
