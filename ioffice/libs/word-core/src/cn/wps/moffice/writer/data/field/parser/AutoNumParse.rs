// Java: import cn.wps.moffice.writer.core.Document
use crate::cn::wps::moffice::writer::core::Document::*;
// Java: import cn.wps.moffice.writer.data.PLCField
use crate::cn::wps::moffice::writer::data::PLCField::FieldTriple;
// Java: import cn.wps.moffice.writer.data.field.AutoNumFieldCache
use crate::cn::wps::moffice::writer::data::field::AutoNumFieldCache::*;
// Java: import cn.wps.moffice.writer.data.field.FieldCodeLex
use crate::cn::wps::moffice::writer::data::field::FieldCodeLex::*;
// Java: import cn.wps.moffice.writer.data.field.StrFormatUtil
use crate::cn::wps::moffice::writer::data::field::StrFormatUtil::*;
// Java: import java.util.HashMap
// Java: import java.util.Iterator
// Java: import cn.wps.moffice.writer.data.field.parser.BaseParser
use crate::cn::wps::moffice::writer::data::field::parser::BaseParser::*;
// Java: import java.lang.Object
use stubs::java::lang::Object::*;
use stubs::java::lang::StringBuffer::StringBuffer;
use crate::cn::wps::moffice::writer::data::field::parser::IFieldParser::IFieldParser;
use crate::cn::wps::moffice::writer::data::field::KFieldType::KFieldType;
use std::collections::HashMap;

// java public class AutoNumParse extends BaseParser
pub struct AutoNumParse {
    // java BaseParser super_
    pub(crate) super_: BaseParser,
    // java private String format = "Arabic";
    format: Option<String>,
    // java private String resultStr = "";
    resultStr: Option<String>,
}

impl Default for AutoNumParse {
    fn default() -> Self {
        Self {
            super_: BaseParser::default(),
            format: Some("Arabic".to_string()),
            resultStr: Some(String::new()),
        }
    }
}

// java public class AutoNumParse extends BaseParser
impl AutoNumParse {
    // java public void parse(PLCField.FieldTriple fieldTriple, Document subDoc, Object arg)
    pub fn parse__body(&mut self, fieldTriple: &FieldTriple, subDoc: &dyn Document, arg: &dyn Object) {
      // java super.parse(fieldTriple, subDoc, arg);
      self.super_.parse(fieldTriple, subDoc, arg);
      // java if (arg instanceof String && "parse".equals(arg))
      if arg.as_any().downcast_ref::<String>().map(|s| s.as_str() == "parse").unwrap_or(false) {
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
          // java HashMap<String, Boolean> formatMap = this.mCodeLex.getFormats();
          let mut formatMap: HashMap<String, bool> = self.super_.mCodeLex.as_ref().map(|l| l.getFormats().clone()).unwrap_or_default();
          // java Iterator<String> iter = formatMap.keySet().iterator();
          let mut iter = formatMap.keys();
          // java this.format = (String)iter.next();
          self.format = iter.next().cloned();
        }
        // java AutoNumFieldCache autoNumFieldCache = subDoc.getAutoNumField();
        let mut num: i64 = 0;
        if let Some(autoNumFieldCache) = subDoc.getAutoNumField() {
          // java long num = (long)autoNumFieldCache.getAutoNum(fieldTriple.getCodeRange());
          // rust: `FieldTriple.getCodeRange()` 门面返回 `i64`（`PLCField.rs:1343`，
          //   非 `Option`）；`getAutoNumField()` 门面返回裸 `*mut ()`
          //   （`Document.rs:199`，指向 `AutoNumFieldCache`）⇒ 还原为指针解引用调用。
          let range: i64 = fieldTriple.getCodeRange();
          num = unsafe {
            (*(autoNumFieldCache
              as *mut crate::cn::wps::moffice::writer::data::field::AutoNumFieldCache::AutoNumFieldCache))
              .getAutoNum(range)
          } as i64;
        }
        // java this.resultStr = StrFormatUtil.renderByFormat(this.mCodeLex, num);
        self.resultStr = StrFormatUtil::renderByFormat(
          self.super_.mCodeLex.as_ref(),
          Some(&num as &dyn Object),
        );
        // java this.mResult = new StringBuffer(this.resultStr);
        self.super_.mResult = Some(StringBuffer::new__string(self.resultStr.as_deref().unwrap_or("")));
      }
    }

}

impl IFieldParser for AutoNumParse {
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

impl std::ops::Deref for AutoNumParse {
    type Target = BaseParser;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for AutoNumParse {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}
