// Java: import cn.wps.moffice.kfs.File
use kfs_kfs_app::cn::wps::moffice::kfs::File::*;
// Java: import cn.wps.moffice.writer.core.Document
use crate::cn::wps::moffice::writer::core::Document::*;
// Java: import cn.wps.moffice.writer.data.PLCField
use crate::cn::wps::moffice::writer::data::PLCField::FieldTriple;
// Java: import cn.wps.moffice.writer.data.field.KFieldType
use crate::cn::wps::moffice::writer::data::field::KFieldType::*;
// Java: import cn.wps.moffice.writer.data.field.parser.NumFieldParse
use crate::cn::wps::moffice::writer::data::field::parser::NumFieldParse::*;
// Java: import java.lang.Object
use stubs::java::lang::Object::*;
use crate::cn::wps::moffice::writer::data::field::parser::IFieldParser::IFieldParser;
use crate::cn::wps::moffice::writer::data::field::FieldCodeLex::FieldCodeLex;

// java private static final char SWITCH_K = 'k';
const SWITCH_K: char = 'k';
// java private static final char SWITCH_M = 'm';
const SWITCH_M: char = 'm';

// java public class FileSizeParse extends NumFieldParse
#[derive(Default)]
pub struct FileSizeParse {
    // java NumFieldParse super_
    pub(crate) super_: NumFieldParse,
}

// java public class FileSizeParse extends NumFieldParse
impl FileSizeParse {
    // java public void parse(PLCField.FieldTriple fieldTriple, Document subDoc, Object arg)
    pub fn parse__body(&mut self, fieldTriple: &FieldTriple, subDoc: &dyn Document, arg: &dyn Object) {
      // java super.parse(fieldTriple, subDoc, arg);
      self.super_.parse(fieldTriple, subDoc, arg);
      // java if (arg instanceof String && "parse".equals(arg))
      if arg.as_any().downcast_ref::<String>().map(|s| s.as_str() == "parse").unwrap_or(false) {
        // java char[] codeBuffer = this.getCodeBuffer();
        let mut codeBuffer: Vec<char> = self.super_.getCodeBuffer();
        // java this.numparse(codeBuffer);
        self.super_.mCodeLex = Some(FieldCodeLex::default());
        self.registerSwitch();
        if let Some(ref mut lex) = self.super_.mCodeLex {
          lex.parse(&codeBuffer, codeBuffer.len() as i32);
        }
        let field_num = self.setFieldValue();
        self.super_.format_field_num(field_num);
      }
    }

    // java public void parseByCharBuffer(char[] codeBuffer, Document subDoc, KFieldType fieldType)
    pub fn parseByCharBuffer__body(&mut self, codeBuffer: &[char], subDoc: &dyn Document, fieldType: KFieldType) {
      // java super.parseByCharBuffer(codeBuffer, subDoc, fieldType);
      self.super_.parseByCharBuffer(codeBuffer, subDoc, fieldType);
      // java this.numparse(codeBuffer);
      let buf: Vec<char> = codeBuffer.to_vec();
      self.super_.mCodeLex = Some(FieldCodeLex::default());
      self.registerSwitch();
      if let Some(ref mut lex) = self.super_.mCodeLex {
        lex.parse(&buf, buf.len() as i32);
      }
      let field_num = self.setFieldValue();
      self.super_.format_field_num(field_num);
    }

    // java public int setFieldValue()
    pub fn setFieldValue(&mut self) -> i32 {
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
      // java if (this.mCodeLex.findSwitch('k', (StringBuffer)null))
      if self.super_.mCodeLex.as_ref().map(|l| l.findSwitch(SWITCH_K, None)).unwrap_or(false) {
        // java fileSize = Math.round((double)fileSize / (double)1000.0F);
        fileSize = ((fileSize as f64) / 1000.0_f64).round() as i64;
      }
      // java else if (this.mCodeLex.findSwitch('m', (StringBuffer)null))
      else if self.super_.mCodeLex.as_ref().map(|l| l.findSwitch(SWITCH_M, None)).unwrap_or(false) {
        // java fileSize = Math.round((double)fileSize / (double)1000000.0F);
        fileSize = ((fileSize as f64) / 1_000_000.0_f64).round() as i64;
      }
      // java return (int)fileSize;
      return fileSize as i32;
    }

    // java public void registerSwitch()
    pub fn registerSwitch(&mut self) {
      // java this.mCodeLex.registerPrivateSwitch('k');
      if let Some(ref mut lex) = self.super_.mCodeLex {
        lex.registerPrivateSwitch__char(SWITCH_K);
        // java this.mCodeLex.registerPrivateSwitch('m');
        lex.registerPrivateSwitch__char(SWITCH_M);
      }
    }

}

impl IFieldParser for FileSizeParse {
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

impl std::ops::Deref for FileSizeParse {
    type Target = NumFieldParse;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for FileSizeParse {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}
