// Java: import cn.wps.moffice.kfs.File
use kfs_kfs_app::cn::wps::moffice::kfs::File::*;
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
// Java: import java.text.SimpleDateFormat
use stubs::java::text::SimpleDateFormat::*;
// Java: import java.util.Date
use stubs::java::util::Date::*;
// Java: import java.util.HashMap
// Java: import cn.wps.moffice.writer.data.field.parser.BaseParser
use crate::cn::wps::moffice::writer::data::field::parser::BaseParser::*;
// Java: import java.lang.Object
use stubs::java::lang::Object::*;
use stubs::java::lang::StringBuffer::StringBuffer;
use crate::cn::wps::moffice::writer::data::field::parser::IFieldParser::IFieldParser;
use std::collections::HashMap;

// java private static final int AUTHOR = 1;
const AUTHOR: i32 = 1;
// java private static final int COMMENTS = 2;
const COMMENTS: i32 = 2;
// java private static final int CREATEDATE = 3;
const CREATEDATE: i32 = 3;
// java private static final int EDITTIME = 4;
const EDITTIME: i32 = 4;
// java private static final int FILENAME = 5;
const FILENAME: i32 = 5;
// java private static final int FILESIZE = 6;
const FILESIZE: i32 = 6;
// java private static final int KEYWORDS = 7;
const KEYWORDS: i32 = 7;
// java private static final int LASTSAVEDBY = 8;
const LASTSAVEDBY: i32 = 8;
// java private static final int NUMCHARS = 9;
const NUMCHARS: i32 = 9;
// java private static final int NUMPAGES = 10;
const NUMPAGES: i32 = 10;
// java private static final int NUMWORDS = 11;
const NUMWORDS: i32 = 11;
// java private static final int PRINTDATE = 12;
const PRINTDATE: i32 = 12;
// java private static final int REVNUM = 13;
const REVNUM: i32 = 13;
// java private static final int SAVEDATE = 14;
const SAVEDATE: i32 = 14;
// java private static final int SUBJECT = 15;
const SUBJECT: i32 = 15;
// java private static final int TEMPLATE = 16;
const TEMPLATE: i32 = 16;
// java private static final int TITLE = 17;
const TITLE: i32 = 17;
// java private static HashMap<String, Integer> infoMap = new HashMap();
static infoMap: std::sync::LazyLock<HashMap<String, i32>> = std::sync::LazyLock::new(|| {
    let mut m = HashMap::new();
    m.insert("AUTHOR".to_string(), 1);
    m.insert("COMMENTS".to_string(), 2);
    m.insert("CREATEDATE".to_string(), 3);
    m.insert("EDITTIME".to_string(), 4);
    m.insert("FILENAME".to_string(), 5);
    m.insert("FILESIZE".to_string(), 6);
    m.insert("KEYWORDS".to_string(), 7);
    m.insert("LASTSAVEDBY".to_string(), 8);
    m.insert("NUMCHARS".to_string(), 9);
    m.insert("NUMPAGES".to_string(), 10);
    m.insert("NUMWORDS".to_string(), 11);
    m.insert("PRINTDATE".to_string(), 12);
    m.insert("REVNUM".to_string(), 13);
    m.insert("SAVEDATE".to_string(), 14);
    m.insert("SUBJECT".to_string(), 15);
    m.insert("TEMPLATE".to_string(), 16);
    m.insert("TITLE".to_string(), 17);
    m
});

// java public class InfoParse extends BaseParser
#[derive(Default)]
pub struct InfoParse {
    // java BaseParser super_
    pub(crate) super_: BaseParser,
    // java private String resultStr;
    resultStr: Option<String>,
}

// java public class InfoParse extends BaseParser
impl InfoParse {
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
        // java this.commonparse(codeBuffer);
        self.commonparse(&codeBuffer);
        // java if (this.resultStr != null)
        if self.resultStr.is_some() {
          // java this.mResult = new StringBuffer(this.resultStr);
          self.super_.mResult = Some(StringBuffer::new__string(self.resultStr.as_deref().unwrap_or("")));
        }
        // java else
        else {
          // java this.mResult = new StringBuffer("");
          self.super_.mResult = Some(StringBuffer::new__string(""));
        }
      }
    }

    // java public void parseByCharBuffer(char[] codeBuffer, Document subDoc, KFieldType fieldType)
    pub fn parseByCharBuffer__body(&mut self, codeBuffer: &[char], subDoc: &dyn Document, fieldType: KFieldType) {
      // java super.parseByCharBuffer(codeBuffer, subDoc, fieldType);
      self.super_.parseByCharBuffer(codeBuffer, subDoc, fieldType);
      // java this.commonparse(codeBuffer);
      let buf: Vec<char> = codeBuffer.to_vec();
      self.commonparse(&buf);
      // java if (this.resultStr != null)
      if self.resultStr.is_some() {
        // java this.mResult = new StringBuffer(this.resultStr);
        self.super_.mResult = Some(StringBuffer::new__string(self.resultStr.as_deref().unwrap_or("")));
      }
      // java else
      else {
        // java this.mResult = new StringBuffer("");
        self.super_.mResult = Some(StringBuffer::new__string(""));
      }
    }

    // java private void commonparse(char[] codeBuffer)
    pub fn commonparse(&mut self, codeBuffer: &Vec<char>) {
      // java if (this.metaProperty == null)
      if self.super_.metaProperty.is_none() {
        // java this.resultStr = "";
        self.resultStr = Some(String::new());
      }
      // java else
      else {
        // java this.mCodeLex = new FieldCodeLex();
        self.super_.mCodeLex = Some(FieldCodeLex::default());
        // java this.mCodeLex.parse(codeBuffer, codeBuffer.length);
        if let Some(ref mut lex) = self.super_.mCodeLex {
          lex.parse(codeBuffer, codeBuffer.len() as i32);
        }
        // java int paramsCount = this.mCodeLex.getParamCount();
        let mut paramsCount: i32 = self.super_.mCodeLex.as_ref().map(|l| l.getParamCount()).unwrap_or(0);
        // java if (paramsCount == 0)
        if paramsCount == 0 {
          // java this.resultStr = "";
          self.resultStr = Some(String::new());
        }
        // java else
        else {
          // java String infoName = this.mCodeLex.getParam(0).mStr.toUpperCase();
          let mut infoName: String = self.super_.mCodeLex.as_ref()
            .and_then(|l| l.getParam(0))
            .and_then(|p| p.mStr.clone())
            .unwrap_or_default()
            .to_uppercase();
          // java String newValue = null;
          let mut newValue: Option<String> = None;
          // java if (paramsCount == 2)
          if paramsCount == 2 {
            // java newValue = this.mCodeLex.getParam(1).mStr;
            newValue = self.super_.mCodeLex.as_ref()
              .and_then(|l| l.getParam(1))
              .and_then(|p| p.mStr.clone());
          }
          // java int keyMark = (Integer)infoMap.get(infoName);
          let mut keyMark: i32 = infoMap.get(&infoName).copied().unwrap_or(0);
          // java switch (keyMark)
          match keyMark {
            // java case 1:
            1 => {
              // java if (newValue == null)
              if newValue.is_none() {
                // java this.resultStr = this.metaProperty.getAuthor();
                self.resultStr = self.super_.metaProperty.as_ref().map(|m| m.getAuthor());
              }
              // java else
              else {
                // java this.resultStr = newValue;
                self.resultStr = newValue.clone();
                // java this.metaProperty.setAuthor(newValue);
                if let (Some(m), Some(ref v)) = (self.super_.metaProperty.as_mut(), newValue.as_ref()) {
                  m.setAuthor(v);
                }
              }
            }
            // java case 2:
            2 => {
              // java if (newValue == null)
              if newValue.is_none() {
                // java this.resultStr = this.metaProperty.getComments();
                self.resultStr = self.super_.metaProperty.as_ref().map(|m| m.getComments());
              }
              // java else
              else {
                // java this.resultStr = newValue;
                self.resultStr = newValue.clone();
                // java this.metaProperty.setComments(newValue);
                if let (Some(m), Some(ref v)) = (self.super_.metaProperty.as_mut(), newValue.as_ref()) {
                  m.setComments(v);
                }
              }
            }
            // java case 3:
            3 => {
              // java Date createDate = this.metaProperty.getCreateTime();
              let mut createDate: Date = self.super_.metaProperty.as_ref().map(|m| m.getCreateTime()).unwrap_or_default();
              // java if (createDate == null)
              // Soft Date is non-null; still format via SimpleDateFormat
              if false {
                // java this.resultStr = "0/0/0000 0:00:00";
                self.resultStr = Some("0/0/0000 0:00:00".to_string());
              }
              // java else
              else {
                // java SimpleDateFormat sf = new SimpleDateFormat("yyyy/mm/dd HH:mm:ss");
                let mut sf: SimpleDateFormat = SimpleDateFormat::new__string("yyyy/mm/dd HH:mm:ss");
                // java this.resultStr = sf.format(createDate);
                self.resultStr = Some(sf.format(&createDate));
              }
            }
            // java case 4:
            4 => {
              // java Long editTime = this.metaProperty.getEditTime() / 60000L;
              let mut editTime: i64 = self.super_.metaProperty.as_ref().map(|m| m.getEditTime()).unwrap_or(0) / 60000;
              // java this.resultStr = String.valueOf(editTime);
              self.resultStr = Some(editTime.to_string());
            }
            // java case 5:
            5 => {
              // java this.resultStr = this.mSubDocument.getTextDocument().getName();
              self.resultStr = self.super_.subDocument()
          .and_then(|p| p.getTextDocument())
                .map(|td| td.getName());
            }
            // java case 6:
            6 => {
              // java long fileSize = 0L;
              let mut fileSize: i64 = 0;
              // java String absolutePath = this.mSubDocument.getTextDocument().getPath();
              let mut absolutePath: Option<String> = self.super_.subDocument()
          .and_then(|p| p.getTextDocument())
                .map(|td| td.getPath());
              // java File documentFile = null;
              let mut documentFile: Option<File> = None;
              // java if (absolutePath != null)
              if let Some(ref path) = absolutePath {
                if !path.is_empty() {
                  // java documentFile = new File(absolutePath);
                  documentFile = Some(File::new__string(path.clone()));
                  // java if (documentFile.exists())
                  if let Some(ref mut documentFile) = documentFile {
                    if documentFile.exists() {
                      // java fileSize = documentFile.length();
                      fileSize = documentFile.length();
                    }
                  }
                }
              }
              // java this.resultStr = String.valueOf(fileSize);
              self.resultStr = Some(fileSize.to_string());
            }
            // java case 7:
            7 => {
              // java if (newValue == null)
              if newValue.is_none() {
                // java this.resultStr = this.metaProperty.getKeyWords();
                self.resultStr = self.super_.metaProperty.as_ref().map(|m| m.getKeyWords());
              }
              // java else
              else {
                // java this.resultStr = newValue;
                self.resultStr = newValue.clone();
                // java this.metaProperty.setKeyWords(newValue);
                if let (Some(m), Some(ref v)) = (self.super_.metaProperty.as_mut(), newValue.as_ref()) {
                  m.setKeyWords(v);
                }
              }
            }
            // java case 8:
            8 => {
              // java this.resultStr = this.metaProperty.getLastAuthor();
              self.resultStr = self.super_.metaProperty.as_ref().map(|m| m.getLastAuthor());
            }
            // java case 9:
            9 => {
              // java int numChar = this.metaProperty.getCharCount();
              let mut numChar: i32 = self.super_.metaProperty.as_ref().map(|m| m.getCharCount()).unwrap_or(0);
              // java this.resultStr = String.valueOf(numChar);
              self.resultStr = Some(numChar.to_string());
            }
            // java case 10:
            10 => {
              // java int numpages = this.metaProperty.getPageCount();
              let mut numpages: i32 = self.super_.metaProperty.as_ref().map(|m| m.getPageCount()).unwrap_or(0);
              // java this.resultStr = String.valueOf(numpages);
              self.resultStr = Some(numpages.to_string());
            }
            // java case 11:
            11 => {
              // java int numWords = this.metaProperty.getWordCount();
              let mut numWords: i32 = self.super_.metaProperty.as_ref().map(|m| m.getWordCount()).unwrap_or(0);
              // java this.resultStr = String.valueOf(numWords);
              self.resultStr = Some(numWords.to_string());
            }
            // java case 12:
            12 => {
              // java Date printDate = this.metaProperty.getLastPrintTime();
              let mut printDate: Date = self.super_.metaProperty.as_ref().map(|m| m.getLastPrintTime()).unwrap_or_default();
              // java if (printDate == null)
              if false {
                // java this.resultStr = "0/0/0000 0:00:00";
                self.resultStr = Some("0/0/0000 0:00:00".to_string());
              }
              // java else
              else {
                // java SimpleDateFormat sfd = new SimpleDateFormat("dd/mm/yyyy HH:mm:ss");
                let mut sfd: SimpleDateFormat = SimpleDateFormat::new__string("dd/mm/yyyy HH:mm:ss");
                // java this.resultStr = sfd.format(printDate);
                self.resultStr = Some(sfd.format(&printDate));
              }
            }
            // java case 13:
            13 => {
              // java Integer iVal = this.metaProperty.getRevNumber();
              let mut iVal: i32 = self.super_.metaProperty.as_ref().map(|m| m.getRevNumber()).unwrap_or(0);
              // java this.resultStr = null != iVal ? iVal.toString() : null;
              self.resultStr = Some(iVal.to_string());
            }
            // java case 14:
            14 => {
              // java Date saveDate = this.metaProperty.getLastSaveTime();
              let mut saveDate: Date = self.super_.metaProperty.as_ref().map(|m| m.getLastSaveTime()).unwrap_or_default();
              // java if (saveDate == null)
              if false {
                // java this.resultStr = "0/0/0000 0:00:00";
                self.resultStr = Some("0/0/0000 0:00:00".to_string());
              }
              // java else
              else {
                // java SimpleDateFormat sf = new SimpleDateFormat("dd/mm/yyyy HH:mm:ss");
                let mut sf: SimpleDateFormat = SimpleDateFormat::new__string("dd/mm/yyyy HH:mm:ss");
                // java this.resultStr = sf.format(saveDate);
                self.resultStr = Some(sf.format(&saveDate));
              }
            }
            // java case 15:
            15 => {
              // java if (newValue == null)
              if newValue.is_none() {
                // java this.resultStr = this.metaProperty.getSubject();
                self.resultStr = self.super_.metaProperty.as_ref().map(|m| m.getSubject());
              }
              // java else
              else {
                // java this.resultStr = newValue;
                self.resultStr = newValue.clone();
                // java this.metaProperty.setSubject(newValue);
                if let (Some(m), Some(ref v)) = (self.super_.metaProperty.as_mut(), newValue.as_ref()) {
                  m.setSubject(v);
                }
              }
            }
            // java case 16:
            16 => {
              // java this.resultStr = this.metaProperty.getTemplate();
              self.resultStr = self.super_.metaProperty.as_ref().map(|m| m.getTemplate());
            }
            // java case 17:
            17 => {
              // java if (newValue == null)
              if newValue.is_none() {
                // java this.resultStr = this.metaProperty.getTitle();
                self.resultStr = self.super_.metaProperty.as_ref().map(|m| m.getTitle());
              }
              // java else
              else {
                // java this.resultStr = newValue;
                self.resultStr = newValue.clone();
                // java this.metaProperty.setTitle(newValue);
                if let (Some(m), Some(ref v)) = (self.super_.metaProperty.as_mut(), newValue.as_ref()) {
                  m.setTitle(v);
                }
              }
            }
            // java default:
            _ => {
              // java this.resultStr = "";
              self.resultStr = Some(String::new());
            }
          }
          // java if (this.resultStr != null)
          if self.resultStr.is_some() {
            // java this.resultStr = StrFormatUtil.renderByFormat(this.mCodeLex, this.resultStr.toCharArray());
            let chars: Vec<char> = self.resultStr.as_ref().map(|s| s.chars().collect()).unwrap_or_default();
            self.resultStr = StrFormatUtil::renderByFormat(
              self.super_.mCodeLex.as_ref(),
              Some(&chars as &dyn Object),
            ).or_else(|| self.resultStr.clone());
          }
        }
      }
      let _ = (AUTHOR, COMMENTS, CREATEDATE, EDITTIME, FILENAME, FILESIZE, KEYWORDS, LASTSAVEDBY, NUMCHARS, NUMPAGES, NUMWORDS, PRINTDATE, REVNUM, SAVEDATE, SUBJECT, TEMPLATE, TITLE);
    }

}

impl IFieldParser for InfoParse {
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

impl std::ops::Deref for InfoParse {
    type Target = BaseParser;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for InfoParse {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}
