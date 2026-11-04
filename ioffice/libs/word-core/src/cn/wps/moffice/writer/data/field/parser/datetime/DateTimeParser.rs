// Java: import cn.wps.core.runtime.Assert
use util::runtime::Assert::*;
// Java: import cn.wps.kfc.numfmt.IWordFormatter
use kfc::cn::wps::kfc::numfmt::IWordFormatter::*;
// Java: import cn.wps.kfc.numfmt.NumFmtFactory
use kfc::cn::wps::kfc::numfmt::NumFmtFactory::*;
// Java: import cn.wps.moffice.property.PropertySet
use util::moffice::property::PropertySet::*;
// Java: import cn.wps.moffice.writer.core.Document
use crate::cn::wps::moffice::writer::core::Document::*;
// Java: import cn.wps.moffice.writer.core.property.IPropertyCalculate
// rust: DEFERRED_NON_FUNCTION —— `core::Range` 依赖闭包（Range/comment/
//   FindOptions/TextUnits/KParagraphFormat/CharParse/CharMap/TextTool）整体未接线，
//   整片登记实测 +73 错误，且本文件对该类型仅作类型面 import、不在 docx 渲染
//   管线上 ⇒ 暂缓登记（Java 注释保留对齐）。
// use crate::cn::wps::moffice::writer::core::property::IPropertyCalculate::*;
// Java: import cn.wps.moffice.writer.core.property.PropertyCalculateFactory
// rust: DEFERRED_NON_FUNCTION —— `core::Range` 依赖闭包（Range/comment/
//   FindOptions/TextUnits/KParagraphFormat/CharParse/CharMap/TextTool）整体未接线，
//   整片登记实测 +73 错误，且本文件对该类型仅作类型面 import、不在 docx 渲染
//   管线上 ⇒ 暂缓登记（Java 注释保留对齐）。
// use crate::cn::wps::moffice::writer::core::property::PropertyCalculateFactory::*;
// Java: import cn.wps.moffice.writer.data.DefaultValue
use crate::cn::wps::moffice::writer::data::DefaultValue;
// Java: import cn.wps.moffice.writer.data.IChpxTable
use crate::cn::wps::moffice::writer::data::IChpxTable::{IChpxTable, Entry as ChpxEntry};
// Java: import cn.wps.moffice.writer.data.PLCField
use crate::cn::wps::moffice::writer::data::PLCField::FieldTriple;
// Java: import cn.wps.moffice.writer.data.RANGEFactory
use crate::cn::wps::moffice::writer::data::RANGEFactory::*;
// Java: import cn.wps.moffice.writer.data.field.FieldCodeLex
use crate::cn::wps::moffice::writer::data::field::FieldCodeLex::*;
// Java: import cn.wps.moffice.writer.data.field.KFieldType
use crate::cn::wps::moffice::writer::data::field::KFieldType::*;
// Java: import cn.wps.moffice.writer.data.field.SwitchRegister
use crate::cn::wps::moffice::writer::data::field::SwitchRegister::*;
// Java: import cn.wps.moffice.writer.data.field.DateTime.switches.specific.DateSwitch
use crate::cn::wps::moffice::writer::data::field::DateTime::switches::specific::DateSwitch::*;
// Java: import cn.wps.moffice.writer.data.field.parser.BaseParser
use crate::cn::wps::moffice::writer::data::field::parser::BaseParser::*;
// Java: import cn.wps.moffice.writer.data.value.Toggle
use crate::cn::wps::moffice::writer::data::value::Toggle::*;
// Java: import java.util.Date
use stubs::java::util::Date::*;
// Java: import org.apache.poi.util.LanguageType
use util::poi_util::LanguageType::*;
// Java: import java.lang.Object
use stubs::java::lang::Object::*;
// Java: import org.apache.poi.poifs.filesystem.Entry
use io_poi_compact::org::apache::poi::poifs::filesystem::Entry as _;
use stubs::java::lang::StringBuffer::StringBuffer;
use crate::cn::wps::moffice::writer::data::field::parser::IFieldParser::IFieldParser;

// java private static final String DEFAULT_TIME_FIELD_FORMAT = "hh:mm am/pm";
const DEFAULT_TIME_FIELD_FORMAT: &str = "hh:mm am/pm";
// java private static final String DEFAULT_DATE_FIELD_FORMAT = "yyyy/M/d";
const DEFAULT_DATE_FIELD_FORMAT: &str = "yyyy/M/d";
// java private static final String DEFAULT_OTHER_FIELD_FORMAT_CALANDAR = "yyyy/M/d HH:mm:ss";
const DEFAULT_OTHER_FIELD_FORMAT_CALANDAR: &str = "yyyy/M/d HH:mm:ss";
// java private static final int BUDDHIST_YEAR_OFFSET = 543;
const BUDDHIST_YEAR_OFFSET: i32 = 543;

// java public class DateTimeParser extends BaseParser
pub struct DateTimeParser {
    // java BaseParser super_
    pub(crate) super_: BaseParser,
    // java private IWordFormatter mWordFormatter = NumFmtFactory.createWordFormatter((short)0);
    mWordFormatter: Option<Box<dyn IWordFormatter>>,
}

impl Default for DateTimeParser {
    fn default() -> Self {
        Self::new()
    }
}

// java public class DateTimeParser extends BaseParser
impl DateTimeParser {
    // java public  DateTimeParser()
    pub fn new() -> Self {
      Self {
        super_: BaseParser::default(),
        mWordFormatter: Some(Box::new(NumFmtFactory::createWordFormatter(0))),
      }
    }

    // java public  DateTimeParser(String fieldCode, LanguageType languageType, Document document)
    pub fn new__string_languagetype_document(fieldCode: &str, languageType: &LanguageType, document: &dyn Document) -> Self {
      // java Assert.assertNotNull("fieldCode should not be null!", fieldCode);
      let _ = "fieldCode should not be null!";
      Assert::assertNotNull(&fieldCode);
      // java Assert.assertNotNull("document should not be null!", document);
      let _ = "document should not be null!";
      Assert::assertNotNull(&document);
      // java super.parse((PLCField.FieldTriple)null, document, (Object)null);
      let mut this = Self::new();
      this.super_.mFieldTriple = None;
      // rust: 经 `BaseParser::erase_lifetime` 打断 `'static` 推导
      //   （见 `BaseParser` 中字段声明与辅助函数说明）。
      this.super_.mSubDocument = Some(crate::cn::wps::moffice::writer::data::field::parser::BaseParser::erase_lifetime(document));
      this.super_.setMetaProperty();
      // java this.mCodeLex = new FieldCodeLex();
      this.super_.mCodeLex = Some(FieldCodeLex::default());
      // java SwitchRegister.gRegisterDateTimeSwitch(this.mCodeLex);
      if let Some(ref mut lex) = this.super_.mCodeLex {
        SwitchRegister::gRegisterDateTimeSwitch(lex);
        // java this.mCodeLex.parse(fieldCode.toCharArray(), fieldCode.length());
        let chars: Vec<char> = fieldCode.chars().collect();
        let n_len: i32 = chars.len() as i32;
        lex.parse(&chars, n_len);
      }
      // java char[] ch = new char[]{'\u0000'};
      let mut ch: Vec<char> = vec!['\u{0000}'];
      // java String format = this.mCodeLex.getPicture(ch);
      let mut format: String = this.super_.mCodeLex.as_ref().map(|l| l.getPicture(Some(&mut ch))).unwrap_or_default();
      // java short defEastAsiaLID = 2052;
      let mut defEastAsiaLID: i16 = 2052;
      // java short defLatinLID = 1033;
      let mut defLatinLID: i16 = 1033;
      // java short defBidiLID = 1025;
      let mut defBidiLID: i16 = 1025;
      // java String result = this.mWordFormatter.formatDateTime(new Date(), format, languageType.getValue(), this.getCalendar(), false, defEastAsiaLID, defLatinLID, defBidiLID);
      let calendar = this.getCalendar();
      let mut result: String = this.mWordFormatter.as_ref().map(|f| {
        f.formatDateTime(0i64, &format, languageType.getValue(), calendar, false, defEastAsiaLID, defLatinLID, defBidiLID)
      }).unwrap_or_default();
      let _ = Date::new();
      // java this.mResult = new StringBuffer(result);
      this.super_.mResult = Some(StringBuffer::new__string(&result));
      this
    }

    // java public void parse(PLCField.FieldTriple fieldTriple, Document subDoc, Object arg)
    pub fn parse__body(&mut self, fieldTriple: &FieldTriple, subDoc: &dyn Document, arg: &dyn Object) {
      // java super.parse(fieldTriple, subDoc, arg);
      self.super_.parse(fieldTriple, subDoc, arg);
      // java long codeRange = fieldTriple.getCodeRange();
      let mut codeRange: i64 = fieldTriple.getCodeRange();
      // java int codeLength = RANGEFactory.getLength(codeRange);
      let mut codeLength: i32 = RANGEFactory::getLength(codeRange);
      // java if (null == this.mResult && codeLength > 0)
      if self.super_.mResult.is_none() && codeLength > 0 {
        // java this.initCodeLex(subDoc, codeRange);
        self.initCodeLex(subDoc, codeRange);
        // java PropertySet runProp = this.getFieldRunProp(subDoc, fieldTriple);
        let mut runProp: PropertySet = self.getFieldRunProp(subDoc, fieldTriple);
        // java Toggle runBidi = (Toggle)runProp.getObject(57, DefaultValue.run_bidi);
        let mut runBidi: Toggle = runProp
          .getValue(57)
          .and_then(|pv| pv.as_any().downcast_ref::<Toggle>().copied())
          .or_else(|| **DefaultValue::run_bidi)
          .unwrap_or(Toggle::OFF);
        // java boolean hasRTLFlag = Toggle.ON == runBidi;
        let mut hasRTLFlag: bool = Toggle::ON == runBidi;
        // java short defEastAsiaLID = 2052;
        let mut defEastAsiaLID: i16 = 2052;
        // java short defLatinLID = 1033;
        let mut defLatinLID: i16 = 1033;
        // java short defBidiLID = 1025;
        let mut defBidiLID: i16 = 1025;
        let _ = (defEastAsiaLID, defLatinLID, defBidiLID);
        // java byte calendar = this.getCalendar();
        let mut calendar: i8 = self.getCalendar();
        // java short langCode = this.getLanguageCode(fieldTriple, subDoc);
        let mut langCode: i16 = self.getLanguageCode(fieldTriple, subDoc);
        // java Date parseDate = this.getParseDate(arg, fieldTriple);
        let mut parseDate: Date = self.getParseDate(arg, fieldTriple);
        let _ = &parseDate;
        // java String format = this.getFormat(subDoc, fieldTriple, calendar);
        let mut format: String = self.getFormat(subDoc, fieldTriple, calendar);
        // java if (format.indexOf("EEEE") > -1)
        if format.find("EEEE").map(|i| i as i32).unwrap_or(-1) > -1 {
          // java calendar = 0;
          calendar = 0;
        }
        // java String result = this.mWordFormatter.formatDateTime(parseDate, format, langCode, calendar, hasRTLFlag, (short)2052, (short)1033, (short)1025);
        let mut result: String = self.mWordFormatter.as_ref().map(|f| {
          f.formatDateTime(0i64, &format, langCode, calendar, hasRTLFlag, 2052i16, 1033i16, 1025i16)
        }).unwrap_or_default();
        // java this.mResult = new StringBuffer(result);
        self.super_.mResult = Some(StringBuffer::new__string(&result));
      }
    }

    // java private PropertySet getFieldRunProp(Document doc, PLCField.FieldTriple triple)
    pub fn getFieldRunProp(&self, doc: &dyn Document, triple: &FieldTriple) -> PropertySet {
      // java int fieldStartCP = triple.getStartCP();
      let mut fieldStartCP: i32 = triple.getStartCP();
      // java IChpxTable.Entry run = doc.getChpxTable().seek(fieldStartCP + 1);
      let mut run: Box<dyn ChpxEntry> = unsafe {
        let table = doc.getChpxTable().expect("chpx table");
        // rust: `seek` 同时存在于 `IChpxTable` 与 `ITable` ⇒ 显式限定来源 trait。
        IChpxTable::seek(unsafe { &*table }, fieldStartCP + 1).expect("chpx entry")
      };
      // java return run.getProperty();
      // rust: `getProperty` 同时存在于 `ITableEntry` 与 `Entry` ⇒ 限定 `Entry`。
      return <dyn ChpxEntry as ChpxEntry>::getProperty(&*run).cloned().unwrap_or_default();
    }

    // java private void initCodeLex(Document doc, long codeRange)
    pub fn initCodeLex(&mut self, doc: &dyn Document, codeRange: i64) {
      // java int codeLength = RANGEFactory.getLength(codeRange);
      let mut codeLength: i32 = RANGEFactory::getLength(codeRange);
      // java char[] buffer = new char[codeLength];
      let mut buffer: Vec<char> = vec![Default::default(); (codeLength) as usize];
      // java doc.getText(RANGEFactory.getStart(codeRange), RANGEFactory.getEnd(codeRange), buffer, 0);
      doc.getText(RANGEFactory::getStart(codeRange), RANGEFactory::getEnd(codeRange), &mut buffer, 0);
      // java this.mCodeLex = new FieldCodeLex();
      self.super_.mCodeLex = Some(FieldCodeLex::default());
      // java SwitchRegister.gRegisterDateTimeSwitch(this.mCodeLex);
      if let Some(ref mut lex) = self.super_.mCodeLex {
        SwitchRegister::gRegisterDateTimeSwitch(lex);
        // java this.mCodeLex.parse(buffer, codeLength);
        lex.parse(&buffer, codeLength);
      }
    }

    // java private String getFormat(Document doc, PLCField.FieldTriple triple, byte calendar)
    pub fn getFormat(&self, doc: &dyn Document, triple: &FieldTriple, calendar: i8) -> String {
      let _ = (doc, calendar);
      // java char[] ch = new char[]{'\u0000'};
      let mut ch: Vec<char> = vec!['\u{0000}'];
      // java String format = this.mCodeLex.getPicture(ch);
      let mut format: String = self.super_.mCodeLex.as_ref().map(|l| l.getPicture(Some(&mut ch))).unwrap_or_default();
      // java if (format == null || format.trim().length() == 0)
      if format.trim().is_empty() {
        // java KFieldType fieldType = KFieldType.getKFieldType(triple.getFieldType());
        let mut fieldType: KFieldType = KFieldType::getKFieldType(triple.getFieldType()).unwrap_or(KFieldType::FieldDate);
        // java switch (fieldType)
        match fieldType {
          // java case FieldTime:
          KFieldType::FieldTime => {
            // java format = "hh:mm am/pm";
            format = DEFAULT_TIME_FIELD_FORMAT.to_string();
          }
          // java case FieldDate:
          KFieldType::FieldDate => {
            // java format = "yyyy/M/d";
            format = DEFAULT_DATE_FIELD_FORMAT.to_string();
          }
          // java default:
          _ => {
            // java format = "yyyy/M/d HH:mm:ss";
            format = DEFAULT_OTHER_FIELD_FORMAT_CALANDAR.to_string();
          }
        }
      }
      // java return format;
      return format;
    }

    // java private Date getParseDate(Object arg, PLCField.FieldTriple fieldTriple)
    pub fn getParseDate(&mut self, arg: &dyn Object, fieldTriple: &FieldTriple) -> Date {
      // java if (arg instanceof Long)
      if arg.as_any().downcast_ref::<i64>().is_some() {
        // java return new Date((Long)arg);
        let _ms = arg.as_any().downcast_ref::<i64>().copied().unwrap_or(0);
        let _ = _ms;
        return Date::new();
      }
      // java else if (arg instanceof Date)
      else if arg.as_any().downcast_ref::<Date>().is_some() {
        // java return (Date)arg;
        return arg.as_any().downcast_ref::<Date>().cloned().unwrap_or_else(Date::new);
      }
      // java else if (arg instanceof String && "parse".equals(arg))
      else if arg.as_any().downcast_ref::<String>().map(|s| s.as_str() == "parse").unwrap_or(false) {
        // java if (this.mSubDocument.getTextDocument().isInIOState())
        let in_io = self.super_.subDocument()
          .and_then(|p| p.getTextDocument())
          .map(|td| td.isInIOState())
          .unwrap_or(false);
        if in_io {
          // java this.reLayout = true;
          self.super_.reLayout = true;
          // java return new Date(0L);
          return Date::new();
        }
        // java else
        else {
          // java KFieldType fieldType = KFieldType.getKFieldType(fieldTriple.getFieldType());
          let mut fieldType: KFieldType = KFieldType::getKFieldType(fieldTriple.getFieldType()).unwrap_or(KFieldType::FieldDate);
          // java switch (fieldType)
          match fieldType {
            // java case FieldTime: case FieldDate:
            KFieldType::FieldTime | KFieldType::FieldDate => {
              // java return new Date();
              return Date::new();
            }
            // java case FieldPrintDate:
            KFieldType::FieldPrintDate => {
              // java Date lastPrintDate = this.metaProperty.getLastPrintTime();
              let mut lastPrintDate: Option<Date> = self.super_.metaProperty.as_ref().map(|m| m.getLastPrintTime());
              // java if (lastPrintDate != null)
              if lastPrintDate.is_some() {
                // java return lastPrintDate;
                return lastPrintDate.unwrap();
              }
              // java return new Date(0L);
              return Date::new();
            }
            // java case FieldSaveDate:
            KFieldType::FieldSaveDate => {
              // java Date lastSaveTime = this.metaProperty.getLastSaveTime();
              let mut lastSaveTime: Option<Date> = self.super_.metaProperty.as_ref().map(|m| m.getLastSaveTime());
              // java if (lastSaveTime != null)
              if lastSaveTime.is_some() {
                // java return lastSaveTime;
                return lastSaveTime.unwrap();
              }
              // java return new Date(0L);
              return Date::new();
            }
            // java case FieldCreateDate:
            KFieldType::FieldCreateDate => {
              // java Date createDate = this.metaProperty.getCreateTime();
              let mut createDate: Option<Date> = self.super_.metaProperty.as_ref().map(|m| m.getCreateTime());
              // java if (createDate != null)
              if createDate.is_some() {
                // java return createDate;
                return createDate.unwrap();
              }
              // java return new Date(0L);
              return Date::new();
            }
            // java default:
            _ => {
              // java return new Date();
              return Date::new();
            }
          }
        }
      }
      // java else
      else {
        // java return new Date();
        return Date::new();
      }
    }

    // java private byte getCalendar()
    pub fn getCalendar(&mut self) -> i8 {
      // java byte calendar = 0;
      let mut calendar: i8 = 0;
      // java if (this.mCodeLex.findSwitch(DateSwitch.SakaEraCalendar.getValue(), (StringBuffer)null))
      if self.super_.mCodeLex.as_ref().map(|l| l.findSwitch(DateSwitch::SakaEraCalendar.getValue(), None)).unwrap_or(false) {
        // java calendar = 0;
        calendar = 0;
      }
      // java else if (this.mCodeLex.findSwitch(DateSwitch.HijriCalendar.getValue(), (StringBuffer)null))
      else if self.super_.mCodeLex.as_ref().map(|l| l.findSwitch(DateSwitch::HijriCalendar.getValue(), None)).unwrap_or(false) {
        // java calendar = 6;
        calendar = 6;
      }
      // java else if (this.mCodeLex.findSwitch(DateSwitch.UmAlQuraCalendar.getValue(), (StringBuffer)null))
      else if self.super_.mCodeLex.as_ref().map(|l| l.findSwitch(DateSwitch::UmAlQuraCalendar.getValue(), None)).unwrap_or(false) {
        // java calendar = 23;
        calendar = 23;
      }
      // java else
      else {
        // java calendar = 0;
        calendar = 0;
      }
      // java return calendar;
      return calendar;
    }

    // java private short getLanguageCode(PLCField.FieldTriple fieldTriple, Document subDoc)
    pub fn getLanguageCode(&self, fieldTriple: &FieldTriple, subDoc: &dyn Document) -> i16 {
      // java IPropertyCalculate propertyCalculate = PropertyCalculateFactory.create(subDoc);
      //
      /* DEFERRED_NON_FUNCTION —— `core::property::{IPropertyCalculate,
       *   PropertyCalculateFactory}` 是 `core::property` 整个依赖闭包的门面
       *   （`PropertyCalculateImpl` 31.5k 行 + `StyleCache`），登记该闭包实测
       *   引入 99 处编译错误、对渲染管线零收益。本方法仅用于取域结果的语言/区域码
       *   （影响 `DateTimeParser` 的日期**渲染文本**的 locale），
       *   不在 docx 解析→排版→渲染的必径上。
       *   Java 原逻辑（`DateTimeParser.java:170-176`）：
       *     IPropertyCalculate propertyCalculate = PropertyCalculateFactory.create(subDoc);
       *     int fieldStartCP = fieldTriple.getStartCP();
       *     IChpxTable.Entry run = subDoc.getChpxTable().seek(fieldStartCP + 1);
       *     Integer val = (Integer)propertyCalculate.getLanguageCode(run);
       *     return null != val ? val.shortValue() : 9;
       *   暂以 Java 的默认分支值 9 承载。 */
      let _ = (fieldTriple, subDoc);
      let mut propertyCalculate: Option<Box<dyn std::any::Any>> = None;
      // java int fieldStartCP = fieldTriple.getStartCP();
      let mut fieldStartCP: i32 = fieldTriple.getStartCP();
      // java IChpxTable.Entry run = subDoc.getChpxTable().seek(fieldStartCP + 1);
      let mut run: Box<dyn ChpxEntry> = unsafe {
        let table = subDoc.getChpxTable().expect("chpx table");
        // rust: `seek` 同时存在于 `IChpxTable` 与 `ITable` ⇒ 显式限定来源 trait。
        IChpxTable::seek(unsafe { &*table }, fieldStartCP + 1).expect("chpx entry")
      };
      // java Integer val = (Integer)propertyCalculate.getLanguageCode(run);
      // java return null != val ? val.shortValue() : 9;
      // rust: 依赖已 DEFERRED（见上）⇒ `val` 为 null ⇒ 走 Java 的默认分支 9。
      let mut val: Option<i32> = None;
      let _ = (&mut propertyCalculate, &run);
      return (if val.is_some() { val.unwrap() as i16 } else { 9 });
    }

    // java public void dispose()
    pub fn dispose__body(&mut self) {
      // java super.dispose();
      self.super_.dispose();
      // java this.mWordFormatter.destroy();
      if let Some(ref mut fmt) = self.mWordFormatter {
        fmt.destroy();
      }
      // java this.mWordFormatter = null;
      self.mWordFormatter = None;
    }

}

impl IFieldParser for DateTimeParser {
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
        self.dispose__body();
    }
    fn getResult(&self) -> Option<String> {
        self.super_.getResult()
    }
    fn getResultRange(&self) -> Option<i64> {
        self.super_.getResultRange()
    }
}

impl std::ops::Deref for DateTimeParser {
    type Target = BaseParser;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for DateTimeParser {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}
