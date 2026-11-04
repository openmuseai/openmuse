// Java: import cn.wps.moffice.writer.data.field.KFieldType
use crate::cn::wps::moffice::writer::data::field::KFieldType::*;
// Java: import cn.wps.moffice.writer.data.field.parser.datetime.DateTimeParser
use crate::cn::wps::moffice::writer::data::field::parser::datetime::DateTimeParser::*;
// Java: import cn.wps.moffice.writer.data.field.parser.formula.FormulaParser
use crate::cn::wps::moffice::writer::data::field::parser::formula::FormulaParser::*;
// Java: import cn.wps.moffice.writer.data.field.parser.IFieldParser
use crate::cn::wps::moffice::writer::data::field::parser::IFieldParser::*;
// Java: import cn.wps.moffice.writer.data.field.parser.PageParser
use crate::cn::wps::moffice::writer::data::field::parser::PageParser::*;
// Java: import cn.wps.moffice.writer.data.field.parser.HyperlinkParser
use crate::cn::wps::moffice::writer::data::field::parser::HyperlinkParser::*;
// Java: import cn.wps.moffice.writer.data.field.parser.NumPagesParser
use crate::cn::wps::moffice::writer::data::field::parser::NumPagesParser::*;
// Java: import cn.wps.moffice.writer.data.field.parser.SectionPagesParser
use crate::cn::wps::moffice::writer::data::field::parser::SectionPagesParser::*;
// Java: import cn.wps.moffice.writer.data.field.parser.MacroBtnParser
use crate::cn::wps::moffice::writer::data::field::parser::MacroBtnParser::*;
// Java: import cn.wps.moffice.writer.data.field.parser.FieldStringResultParser
use crate::cn::wps::moffice::writer::data::field::parser::FieldStringResultParser::*;
// Java: import cn.wps.moffice.writer.data.field.parser.FileNamePaser
use crate::cn::wps::moffice::writer::data::field::parser::FileNamePaser::*;
// Java: import cn.wps.moffice.writer.data.field.parser.FileSizeParse
use crate::cn::wps::moffice::writer::data::field::parser::FileSizeParse::*;
// Java: import cn.wps.moffice.writer.data.field.parser.NumWordsParse
use crate::cn::wps::moffice::writer::data::field::parser::NumWordsParse::*;
// Java: import cn.wps.moffice.writer.data.field.parser.NumCharsParse
use crate::cn::wps::moffice::writer::data::field::parser::NumCharsParse::*;
// Java: import cn.wps.moffice.writer.data.field.parser.EditTimeParser
use crate::cn::wps::moffice::writer::data::field::parser::EditTimeParser::*;
// Java: import cn.wps.moffice.writer.data.field.parser.TemplateParse
use crate::cn::wps::moffice::writer::data::field::parser::TemplateParse::*;
// Java: import cn.wps.moffice.writer.data.field.parser.InfoParse
use crate::cn::wps::moffice::writer::data::field::parser::InfoParse::*;
// Java: import cn.wps.moffice.writer.data.field.parser.RevNumParse
use crate::cn::wps::moffice::writer::data::field::parser::RevNumParse::*;
// Java: import cn.wps.moffice.writer.data.field.parser.AutoNumParse
use crate::cn::wps::moffice::writer::data::field::parser::AutoNumParse::*;
// Java: import cn.wps.moffice.writer.data.field.parser.SectionParse
use crate::cn::wps::moffice::writer::data::field::parser::SectionParse::*;
// Java: import cn.wps.moffice.writer.data.field.parser.BaseParser
use crate::cn::wps::moffice::writer::data::field::parser::BaseParser::*;

// java public class ParserFactory
#[derive(Default)]
pub struct ParserFactory {
}

// java public class ParserFactory
impl ParserFactory {
    // java public static IFieldParser getFieldParser(KFieldType fieldType)
    pub fn getFieldParser(fieldType: KFieldType) -> Box<dyn IFieldParser> {
      // java switch (fieldType)
      match fieldType {
        // java case FieldPage:
        KFieldType::FieldPage => {
          // java return new PageParser();
          return Box::new(PageParser::default());
        }
        // java case FieldTime: case FieldDate: case FieldPrintDate: case FieldSaveDate: case FieldCreateDate:
        KFieldType::FieldTime | KFieldType::FieldDate | KFieldType::FieldPrintDate | KFieldType::FieldSaveDate | KFieldType::FieldCreateDate => {
          // java return new DateTimeParser();
          return Box::new(DateTimeParser::default());
        }
        // java case FieldHyperlink:
        KFieldType::FieldHyperlink => {
          // java return new HyperlinkParser();
          return Box::new(HyperlinkParser::default());
        }
        // java case FieldNumPages:
        KFieldType::FieldNumPages => {
          // java return new NumPagesParser();
          return Box::new(NumPagesParser::default());
        }
        // java case FieldSectionPages:
        KFieldType::FieldSectionPages => {
          // java return new SectionPagesParser();
          return Box::new(SectionPagesParser::default());
        }
        // java case FieldMacroButton:
        KFieldType::FieldMacroButton => {
          // java return new MacroBtnParser();
          return Box::new(MacroBtnParser::default());
        }
        // java case FieldAuthor: case FieldComments: case FieldLastSavedBy: case FieldSubject: case FieldTitle: case FieldKeyWord: case FieldUserAddress: case FieldUserInitials: case FieldUserName:
        KFieldType::FieldAuthor | KFieldType::FieldComments | KFieldType::FieldLastSavedBy | KFieldType::FieldSubject | KFieldType::FieldTitle | KFieldType::FieldKeyWord | KFieldType::FieldUserAddress | KFieldType::FieldUserInitials | KFieldType::FieldUserName => {
          // java return new FieldStringResultParser();
          return Box::new(FieldStringResultParser::default());
        }
        // java case FieldFileName:
        KFieldType::FieldFileName => {
          // java return new FileNamePaser();
          return Box::new(FileNamePaser::default());
        }
        // java case FieldFileSize:
        KFieldType::FieldFileSize => {
          // java return new FileSizeParse();
          return Box::new(FileSizeParse::default());
        }
        // java case FieldNumWords:
        KFieldType::FieldNumWords => {
          // java return new NumWordsParse();
          return Box::new(NumWordsParse::default());
        }
        // java case FieldNumChars:
        KFieldType::FieldNumChars => {
          // java return new NumCharsParse();
          return Box::new(NumCharsParse::default());
        }
        // java case FieldExpression:
        KFieldType::FieldExpression => {
          // java return new FormulaParser();
          return Box::new(FormulaParser::default());
        }
        // java case FieldEditTime:
        KFieldType::FieldEditTime => {
          // java return new EditTimeParser();
          return Box::new(EditTimeParser::default());
        }
        // java case FieldTemplate:
        KFieldType::FieldTemplate => {
          // java return new TemplateParse();
          return Box::new(TemplateParse::default());
        }
        // java case FieldInfo:
        KFieldType::FieldInfo => {
          // java return new InfoParse();
          return Box::new(InfoParse::default());
        }
        // java case FieldRevisionNum:
        KFieldType::FieldRevisionNum => {
          // java return new RevNumParse();
          return Box::new(RevNumParse::default());
        }
        // java case FieldAutoNumLegal: case FieldAutoNum:
        KFieldType::FieldAutoNumLegal | KFieldType::FieldAutoNum => {
          // java return new AutoNumParse();
          return Box::new(AutoNumParse::default());
        }
        // java case FieldSection:
        KFieldType::FieldSection => {
          // java return new SectionParse();
          return Box::new(SectionParse::default());
        }
        // java default:
        _ => {
          // java return new BaseParser();
          return Box::new(BaseParser::default());
        }
      }
    }

    pub fn getParser(fieldType: KFieldType) -> Box<dyn IFieldParser> {
        Self::getFieldParser(fieldType)
    }

}
