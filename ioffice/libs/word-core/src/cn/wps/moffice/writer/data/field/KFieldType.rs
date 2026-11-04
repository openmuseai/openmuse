// Java: import cn.wps.base.assertion.Assert
use util::base::assertion::Assert::*;
// Java: import java.util.HashMap
use std::collections::HashMap;
use std::sync::OnceLock;

// java public enum KFieldType
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum KFieldType {
    FieldEmpty,
    FieldRefOmitted,
    FieldRef,
    FieldSet,
    FieldIf,
    FieldIndex,
    FieldStyleRef,
    FieldSequence,
    FieldTOC,
    FieldInfo,
    FieldTitle,
    FieldSubject,
    FieldAuthor,
    FieldKeyWord,
    FieldComments,
    FieldLastSavedBy,
    FieldCreateDate,
    FieldSaveDate,
    FieldPrintDate,
    FieldRevisionNum,
    FieldEditTime,
    FieldNumPages,
    FieldNumWords,
    FieldNumChars,
    FieldFileName,
    FieldTemplate,
    FieldDate,
    FieldTime,
    FieldPage,
    FieldExpression,
    FieldQuote,
    FieldPageRef,
    FieldAsk,
    FieldFillIn,
    FieldNext,
    FieldNextIf,
    FieldSkipIf,
    FieldMergeRec,
    FieldPrint,
    FieldEQ,
    FieldGoToButton,
    FieldMacroButton,
    FieldAutoNumOutline,
    FieldAutoNumLegal,
    FieldAutoNum,
    FieldAutoNumOut,
    FieldLink,
    FieldSymbol,
    FieldEmbed,
    FieldMergeField,
    FieldUserName,
    FieldUserInitials,
    FieldUserAddress,
    FieldBarCode,
    FieldDocVariable,
    FieldSectionPages,
    FieldSection,
    FieldIncludePicture,
    FieldIncludeText,
    FieldFileSize,
    FieldNoteRef,
    FieldTOA,
    FieldMergeSeq,
    FieldDatabase,
    FieldAutoText,
    FieldCompare,
    FieldAdvance,
    FieldDocProperty,
    FieldHyperlink,
    FieldAutoTextList,
    FieldListNum,
    FieldBidiOutline,
    FieldAddressBlock,
    FieldGreetingLine,
    FieldShape,
    FieldCitation,
    FieldBibliography,
    FieldControl,
}

// java public enum KFieldType
impl KFieldType {
    /// enum constants in declaration order (ctor registers FLT/NAME maps).
    fn values() -> &'static [KFieldType] {
        &[
            KFieldType::FieldEmpty,
            KFieldType::FieldRefOmitted,
            KFieldType::FieldRef,
            KFieldType::FieldSet,
            KFieldType::FieldIf,
            KFieldType::FieldIndex,
            KFieldType::FieldStyleRef,
            KFieldType::FieldSequence,
            KFieldType::FieldTOC,
            KFieldType::FieldInfo,
            KFieldType::FieldTitle,
            KFieldType::FieldSubject,
            KFieldType::FieldAuthor,
            KFieldType::FieldKeyWord,
            KFieldType::FieldComments,
            KFieldType::FieldLastSavedBy,
            KFieldType::FieldCreateDate,
            KFieldType::FieldSaveDate,
            KFieldType::FieldPrintDate,
            KFieldType::FieldRevisionNum,
            KFieldType::FieldEditTime,
            KFieldType::FieldNumPages,
            KFieldType::FieldNumWords,
            KFieldType::FieldNumChars,
            KFieldType::FieldFileName,
            KFieldType::FieldTemplate,
            KFieldType::FieldDate,
            KFieldType::FieldTime,
            KFieldType::FieldPage,
            KFieldType::FieldExpression,
            KFieldType::FieldQuote,
            KFieldType::FieldPageRef,
            KFieldType::FieldAsk,
            KFieldType::FieldFillIn,
            KFieldType::FieldNext,
            KFieldType::FieldNextIf,
            KFieldType::FieldSkipIf,
            KFieldType::FieldMergeRec,
            KFieldType::FieldPrint,
            KFieldType::FieldEQ,
            KFieldType::FieldGoToButton,
            KFieldType::FieldMacroButton,
            KFieldType::FieldAutoNumOutline,
            KFieldType::FieldAutoNumLegal,
            KFieldType::FieldAutoNum,
            KFieldType::FieldAutoNumOut,
            KFieldType::FieldLink,
            KFieldType::FieldSymbol,
            KFieldType::FieldEmbed,
            KFieldType::FieldMergeField,
            KFieldType::FieldUserName,
            KFieldType::FieldUserInitials,
            KFieldType::FieldUserAddress,
            KFieldType::FieldBarCode,
            KFieldType::FieldDocVariable,
            KFieldType::FieldSectionPages,
            KFieldType::FieldSection,
            KFieldType::FieldIncludePicture,
            KFieldType::FieldIncludeText,
            KFieldType::FieldFileSize,
            KFieldType::FieldNoteRef,
            KFieldType::FieldTOA,
            KFieldType::FieldMergeSeq,
            KFieldType::FieldDatabase,
            KFieldType::FieldAutoText,
            KFieldType::FieldCompare,
            KFieldType::FieldAdvance,
            KFieldType::FieldDocProperty,
            KFieldType::FieldHyperlink,
            KFieldType::FieldAutoTextList,
            KFieldType::FieldListNum,
            KFieldType::FieldBidiOutline,
            KFieldType::FieldAddressBlock,
            KFieldType::FieldGreetingLine,
            KFieldType::FieldShape,
            KFieldType::FieldCitation,
            KFieldType::FieldBibliography,
            KFieldType::FieldControl,
        ]
    }

    /// private fields mType / mFlt from enum constant ctor args.
    fn mType(&self) -> &'static str {
        match self {
            KFieldType::FieldEmpty => "EMPTY",
            KFieldType::FieldRefOmitted => "REF_OMITTED",
            KFieldType::FieldRef => "REF",
            KFieldType::FieldSet => "SET",
            KFieldType::FieldIf => "IF",
            KFieldType::FieldIndex => "INDEX",
            KFieldType::FieldStyleRef => "STYLEREF",
            KFieldType::FieldSequence => "SEQ",
            KFieldType::FieldTOC => "TOC",
            KFieldType::FieldInfo => "INFO",
            KFieldType::FieldTitle => "TITLE",
            KFieldType::FieldSubject => "SUBJECT",
            KFieldType::FieldAuthor => "AUTHOR",
            KFieldType::FieldKeyWord => "KEYWORDS",
            KFieldType::FieldComments => "COMMENTS",
            KFieldType::FieldLastSavedBy => "LASTSAVEDBY",
            KFieldType::FieldCreateDate => "CREATEDATE",
            KFieldType::FieldSaveDate => "SAVEDATE",
            KFieldType::FieldPrintDate => "PRINTDATE",
            KFieldType::FieldRevisionNum => "REVNUM",
            KFieldType::FieldEditTime => "EDITTIME",
            KFieldType::FieldNumPages => "NUMPAGES",
            KFieldType::FieldNumWords => "NUMWORDS",
            KFieldType::FieldNumChars => "NUMCHARS",
            KFieldType::FieldFileName => "FILENAME",
            KFieldType::FieldTemplate => "TEMPLATE",
            KFieldType::FieldDate => "DATE",
            KFieldType::FieldTime => "TIME",
            KFieldType::FieldPage => "PAGE",
            KFieldType::FieldExpression => "=",
            KFieldType::FieldQuote => "QUOTE",
            KFieldType::FieldPageRef => "PAGEREF",
            KFieldType::FieldAsk => "ASK",
            KFieldType::FieldFillIn => "FILLIN",
            KFieldType::FieldNext => "NEXT",
            KFieldType::FieldNextIf => "NEXTIF",
            KFieldType::FieldSkipIf => "SKIPIF",
            KFieldType::FieldMergeRec => "MERGEREC",
            KFieldType::FieldPrint => "PRINT",
            KFieldType::FieldEQ => "EQ",
            KFieldType::FieldGoToButton => "GOTOBUTTON",
            KFieldType::FieldMacroButton => "MACROBUTTON",
            KFieldType::FieldAutoNumOutline => "AUTONUMOUT",
            KFieldType::FieldAutoNumLegal => "AUTONUMLGL",
            KFieldType::FieldAutoNum => "AUTONUM",
            KFieldType::FieldAutoNumOut => "AUTONUMOUT",
            KFieldType::FieldLink => "LINK",
            KFieldType::FieldSymbol => "SYMBOL",
            KFieldType::FieldEmbed => "EMBED",
            KFieldType::FieldMergeField => "MERGEFIELD",
            KFieldType::FieldUserName => "USERNAME",
            KFieldType::FieldUserInitials => "USERINITIALS",
            KFieldType::FieldUserAddress => "USERADDRESS",
            KFieldType::FieldBarCode => "BARCODE",
            KFieldType::FieldDocVariable => "DOCVARIABLE",
            KFieldType::FieldSectionPages => "SECTIONPAGES",
            KFieldType::FieldSection => "SECTION",
            KFieldType::FieldIncludePicture => "INCLUDEPICTURE",
            KFieldType::FieldIncludeText => "INCLUDETEXT",
            KFieldType::FieldFileSize => "FILESIZE",
            KFieldType::FieldNoteRef => "NOTEREF",
            KFieldType::FieldTOA => "TOA",
            KFieldType::FieldMergeSeq => "MERGESEQ",
            KFieldType::FieldDatabase => "DATABASE",
            KFieldType::FieldAutoText => "AUTOTEXT",
            KFieldType::FieldCompare => "COMPARE",
            KFieldType::FieldAdvance => "ADVANCE",
            KFieldType::FieldDocProperty => "DOCPROPERTY",
            KFieldType::FieldHyperlink => "HYPERLINK",
            KFieldType::FieldAutoTextList => "AUTOTEXTLIST",
            KFieldType::FieldListNum => "LISTNUM",
            KFieldType::FieldBidiOutline => "BIDIOUTLINE",
            KFieldType::FieldAddressBlock => "ADDRESSBLOCK",
            KFieldType::FieldGreetingLine => "GREETINGLINE",
            KFieldType::FieldShape => "SHAPE",
            KFieldType::FieldCitation => "CITATION",
            KFieldType::FieldBibliography => "BIBLIOGRAPHY",
            KFieldType::FieldControl => "CONTROL",
        }
    }

    fn mFlt(&self) -> i32 {
        match self {
            KFieldType::FieldEmpty => 1,
            KFieldType::FieldRefOmitted => 2,
            KFieldType::FieldRef => 3,
            KFieldType::FieldSet => 6,
            KFieldType::FieldIf => 7,
            KFieldType::FieldIndex => 8,
            KFieldType::FieldStyleRef => 10,
            KFieldType::FieldSequence => 12,
            KFieldType::FieldTOC => 13,
            KFieldType::FieldInfo => 14,
            KFieldType::FieldTitle => 15,
            KFieldType::FieldSubject => 16,
            KFieldType::FieldAuthor => 17,
            KFieldType::FieldKeyWord => 18,
            KFieldType::FieldComments => 19,
            KFieldType::FieldLastSavedBy => 20,
            KFieldType::FieldCreateDate => 21,
            KFieldType::FieldSaveDate => 22,
            KFieldType::FieldPrintDate => 23,
            KFieldType::FieldRevisionNum => 24,
            KFieldType::FieldEditTime => 25,
            KFieldType::FieldNumPages => 26,
            KFieldType::FieldNumWords => 27,
            KFieldType::FieldNumChars => 28,
            KFieldType::FieldFileName => 29,
            KFieldType::FieldTemplate => 30,
            KFieldType::FieldDate => 31,
            KFieldType::FieldTime => 32,
            KFieldType::FieldPage => 33,
            KFieldType::FieldExpression => 34,
            KFieldType::FieldQuote => 35,
            KFieldType::FieldPageRef => 37,
            KFieldType::FieldAsk => 38,
            KFieldType::FieldFillIn => 39,
            KFieldType::FieldNext => 41,
            KFieldType::FieldNextIf => 42,
            KFieldType::FieldSkipIf => 43,
            KFieldType::FieldMergeRec => 44,
            KFieldType::FieldPrint => 48,
            KFieldType::FieldEQ => 49,
            KFieldType::FieldGoToButton => 50,
            KFieldType::FieldMacroButton => 51,
            KFieldType::FieldAutoNumOutline => 52,
            KFieldType::FieldAutoNumLegal => 53,
            KFieldType::FieldAutoNum => 54,
            KFieldType::FieldAutoNumOut => 52,
            KFieldType::FieldLink => 56,
            KFieldType::FieldSymbol => 57,
            KFieldType::FieldEmbed => 58,
            KFieldType::FieldMergeField => 59,
            KFieldType::FieldUserName => 60,
            KFieldType::FieldUserInitials => 61,
            KFieldType::FieldUserAddress => 62,
            KFieldType::FieldBarCode => 63,
            KFieldType::FieldDocVariable => 64,
            KFieldType::FieldSectionPages => 66,
            KFieldType::FieldSection => 65,
            KFieldType::FieldIncludePicture => 67,
            KFieldType::FieldIncludeText => 68,
            KFieldType::FieldFileSize => 69,
            KFieldType::FieldNoteRef => 72,
            KFieldType::FieldTOA => 73,
            KFieldType::FieldMergeSeq => 75,
            KFieldType::FieldDatabase => 78,
            KFieldType::FieldAutoText => 79,
            KFieldType::FieldCompare => 80,
            KFieldType::FieldAdvance => 84,
            KFieldType::FieldDocProperty => 85,
            KFieldType::FieldHyperlink => 88,
            KFieldType::FieldAutoTextList => 89,
            KFieldType::FieldListNum => 90,
            KFieldType::FieldBidiOutline => 92,
            KFieldType::FieldAddressBlock => 93,
            KFieldType::FieldGreetingLine => 94,
            KFieldType::FieldShape => 95,
            KFieldType::FieldCitation => 96,
            KFieldType::FieldBibliography => 97,
            KFieldType::FieldControl => 87,
        }
    }

    pub fn toString(&self) -> String {
        let mType_opt: Option<&str> = Some(self.mType());
        Assert::assertNotNull_msg("mType should not be null!", &mType_opt);
        self.mType().to_string()
    }

    pub fn getFlt(&self) -> i32 {
        self.mFlt()
    }

    pub fn getKFieldType(flt: i32) -> Option<KFieldType> {
        let map = FLT::sFlt2KFieldType();
        let map_opt: Option<&HashMap<i32, KFieldType>> = Some(map);
        Assert::assertNotNull_msg(
            "FLT.sFlt2KFieldType should not be null!",
            &map_opt,
        );
        map.get(&flt).copied()
    }

    pub fn getKFieldType__String(name: Option<&str>) -> Option<KFieldType> {
        let map = NAME::sName2KFieldType();
        let map_opt: Option<&HashMap<String, KFieldType>> = Some(map);
        Assert::assertNotNull_msg(
            "NAME.sName2KFieldType should not be null!",
            &map_opt,
        );
        let Some(name) = name else {
            return None;
        };
        map.get(name).copied()
    }
}

impl Default for KFieldType {
    fn default() -> Self {
        KFieldType::FieldEmpty
    }
}

/// nested `public static class FLT`
pub struct FLT {}

impl FLT {
    pub const EMPTY: i32 = 1;
    pub const REF_OMITTED: i32 = 2;
    pub const REF: i32 = 3;
    pub const SET: i32 = 6;
    pub const IF: i32 = 7;
    pub const INDEX: i32 = 8;
    pub const STYLEREF: i32 = 10;
    pub const SEQ: i32 = 12;
    pub const TOC: i32 = 13;
    pub const INFO: i32 = 14;
    pub const TITLE: i32 = 15;
    pub const SUBJECT: i32 = 16;
    pub const AUTHOR: i32 = 17;
    pub const KEYWORDS: i32 = 18;
    pub const COMMENTS: i32 = 19;
    pub const LASTSAVEDBY: i32 = 20;
    pub const CREATEDATE: i32 = 21;
    pub const SAVEDATE: i32 = 22;
    pub const PRINTDATE: i32 = 23;
    pub const REVNUM: i32 = 24;
    pub const EDITTIME: i32 = 25;
    pub const NUMPAGES: i32 = 26;
    pub const NUMWORDS: i32 = 27;
    pub const NUMCHARS: i32 = 28;
    pub const FILENAME: i32 = 29;
    pub const TEMPLATE: i32 = 30;
    pub const DATE: i32 = 31;
    pub const TIME: i32 = 32;
    pub const PAGE: i32 = 33;
    pub const EXPRESSION: i32 = 34;
    pub const QUOTE: i32 = 35;
    pub const PAGEREF: i32 = 37;
    pub const ASK: i32 = 38;
    pub const FILLIN: i32 = 39;
    pub const NEXT: i32 = 41;
    pub const NEXTIF: i32 = 42;
    pub const SKIPIF: i32 = 43;
    pub const MERGEREC: i32 = 44;
    pub const PRINT: i32 = 48;
    pub const EQ: i32 = 49;
    pub const GOTOBUTTON: i32 = 50;
    pub const MACROBUTTON: i32 = 51;
    pub const AUTONUMOUT: i32 = 52;
    pub const AUTONUMLGL: i32 = 53;
    pub const AUTONUM: i32 = 54;
    pub const LINK: i32 = 56;
    pub const SYMBOL: i32 = 57;
    pub const EMBED: i32 = 58;
    pub const MERGEFIELD: i32 = 59;
    pub const USERNAME: i32 = 60;
    pub const USERINITIALS: i32 = 61;
    pub const USERADDRESS: i32 = 62;
    pub const BARCODE: i32 = 63;
    pub const DOCVARIABLE: i32 = 64;
    pub const SECTION: i32 = 65;
    pub const SECTIONPAGES: i32 = 66;
    pub const INCLUDEPICTURE: i32 = 67;
    pub const INCLUDETEXT: i32 = 68;
    pub const FILESIZE: i32 = 69;
    pub const NOTEREF: i32 = 72;
    pub const TOA: i32 = 73;
    pub const MERGESEQ: i32 = 75;
    pub const DATABASE: i32 = 78;
    pub const AUTOTEXT: i32 = 79;
    pub const COMPARE: i32 = 80;
    pub const ADVANCE: i32 = 84;
    pub const DOCPROPERTY: i32 = 85;
    pub const HYPERLINK: i32 = 88;
    pub const AUTOTEXTLIST: i32 = 89;
    pub const LISTNUM: i32 = 90;
    pub const BIDIOUTLINE: i32 = 92;
    pub const ADDRESSBLOCK: i32 = 93;
    pub const GREETINGLINE: i32 = 94;
    pub const SHAPE: i32 = 95;
    pub const CITATION: i32 = 96;
    pub const BIBLIOGRAPHY: i32 = 97;
    pub const RD: i32 = 62;
    pub const TA: i32 = 61;
    pub const TC: i32 = 60;
    pub const XE: i32 = 20;
    pub const OCX: i32 = 87;

    pub fn sFlt2KFieldType() -> &'static HashMap<i32, KFieldType> {
        static MAP: OnceLock<HashMap<i32, KFieldType>> = OnceLock::new();
        MAP.get_or_init(|| {
            let mut m = HashMap::new();
            // enum ctor order: put(mFlt, this) — later same flt overwrites
            for v in KFieldType::values() {
                m.insert(v.mFlt(), *v);
            }
            m
        })
    }
}

/// nested `private static class NAME`
pub struct NAME {}

impl NAME {
    pub const EMPTY: &'static str = "EMPTY";
    pub const REF_OMITTED: &'static str = "REF_OMITTED";
    pub const REF: &'static str = "REF";
    pub const SET: &'static str = "SET";
    pub const IF: &'static str = "IF";
    pub const INDEX: &'static str = "INDEX";
    pub const STYLEREF: &'static str = "STYLEREF";
    pub const SEQ: &'static str = "SEQ";
    pub const TOC: &'static str = "TOC";
    pub const INFO: &'static str = "INFO";
    pub const TITLE: &'static str = "TITLE";
    pub const SUBJECT: &'static str = "SUBJECT";
    pub const AUTHOR: &'static str = "AUTHOR";
    pub const KEYWORDS: &'static str = "KEYWORDS";
    pub const COMMENTS: &'static str = "COMMENTS";
    pub const LASTSAVEDBY: &'static str = "LASTSAVEDBY";
    pub const CREATEDATE: &'static str = "CREATEDATE";
    pub const SAVEDATE: &'static str = "SAVEDATE";
    pub const PRINTDATE: &'static str = "PRINTDATE";
    pub const REVNUM: &'static str = "REVNUM";
    pub const EDITTIME: &'static str = "EDITTIME";
    pub const NUMPAGES: &'static str = "NUMPAGES";
    pub const NUMWORDS: &'static str = "NUMWORDS";
    pub const NUMCHARS: &'static str = "NUMCHARS";
    pub const FILENAME: &'static str = "FILENAME";
    pub const TEMPLATE: &'static str = "TEMPLATE";
    pub const DATE: &'static str = "DATE";
    pub const TIME: &'static str = "TIME";
    pub const PAGE: &'static str = "PAGE";
    pub const EXPRESSION: &'static str = "=";
    pub const QUOTE: &'static str = "QUOTE";
    pub const PAGEREF: &'static str = "PAGEREF";
    pub const ASK: &'static str = "ASK";
    pub const FILLIN: &'static str = "FILLIN";
    pub const NEXT: &'static str = "NEXT";
    pub const NEXTIF: &'static str = "NEXTIF";
    pub const SKIPIF: &'static str = "SKIPIF";
    pub const MERGEREC: &'static str = "MERGEREC";
    pub const PRINT: &'static str = "PRINT";
    pub const EQ: &'static str = "EQ";
    pub const GOTOBUTTON: &'static str = "GOTOBUTTON";
    pub const MACROBUTTON: &'static str = "MACROBUTTON";
    pub const AUTONUMOUT: &'static str = "AUTONUMOUT";
    pub const AUTONUMLGL: &'static str = "AUTONUMLGL";
    pub const AUTONUM: &'static str = "AUTONUM";
    pub const LINK: &'static str = "LINK";
    pub const SYMBOL: &'static str = "SYMBOL";
    pub const EMBED: &'static str = "EMBED";
    pub const MERGEFIELD: &'static str = "MERGEFIELD";
    pub const USERNAME: &'static str = "USERNAME";
    pub const USERINITIALS: &'static str = "USERINITIALS";
    pub const USERADDRESS: &'static str = "USERADDRESS";
    pub const BARCODE: &'static str = "BARCODE";
    pub const DOCVARIABLE: &'static str = "DOCVARIABLE";
    pub const SECTION: &'static str = "SECTION";
    pub const SECTIONPAGES: &'static str = "SECTIONPAGES";
    pub const INCLUDETEXT: &'static str = "INCLUDETEXT";
    pub const INCLUDEPICTURE: &'static str = "INCLUDEPICTURE";
    pub const FILESIZE: &'static str = "FILESIZE";
    pub const NOTEREF: &'static str = "NOTEREF";
    pub const TOA: &'static str = "TOA";
    pub const MERGESEQ: &'static str = "MERGESEQ";
    pub const DATABASE: &'static str = "DATABASE";
    pub const AUTOTEXT: &'static str = "AUTOTEXT";
    pub const COMPARE: &'static str = "COMPARE";
    pub const ADVANCE: &'static str = "ADVANCE";
    pub const DOCPROPERTY: &'static str = "DOCPROPERTY";
    pub const HYPERLINK: &'static str = "HYPERLINK";
    pub const AUTOTEXTLIST: &'static str = "AUTOTEXTLIST";
    pub const LISTNUM: &'static str = "LISTNUM";
    pub const BIDIOUTLINE: &'static str = "BIDIOUTLINE";
    pub const ADDRESSBLOCK: &'static str = "ADDRESSBLOCK";
    pub const GREETINGLINE: &'static str = "GREETINGLINE";
    pub const SHAPE: &'static str = "SHAPE";
    pub const CITATION: &'static str = "CITATION";
    pub const BIBLIOGRAPHY: &'static str = "BIBLIOGRAPHY";
    pub const CONTROL: &'static str = "CONTROL";
    pub const RD: &'static str = "RD";
    pub const TA: &'static str = "TA";
    pub const TC: &'static str = "TC";
    pub const XE: &'static str = "XE";

    pub fn sName2KFieldType() -> &'static HashMap<String, KFieldType> {
        static MAP: OnceLock<HashMap<String, KFieldType>> = OnceLock::new();
        MAP.get_or_init(|| {
            let mut m = HashMap::new();
            for v in KFieldType::values() {
                m.insert(v.mType().to_string(), *v);
            }
            m
        })
    }
}
