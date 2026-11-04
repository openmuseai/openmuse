// Java: import cn.wps.base.assertion.IOAssert
use util::base::assertion::IOAssert::*;
// Java: import cn.wps.moffice.property.MutablePropertySet
use util::moffice::property::MutablePropertySet::*;
// Java: import cn.wps.moffice.writer.core.TextDocument
use crate::cn::wps::moffice::writer::core::TextDocument::TextDocument;
// Java: import cn.wps.moffice.writer.io.reader.ImporterBase
use crate::cn::wps::moffice::writer::io::reader::ImporterBase::*;
// Java: import cn.wps.moffice.writer.io.reader.docxReader.importer.NumIdRedirection
use crate::cn::wps::moffice::writer::io::reader::docxReader::importer::NumIdRedirection::*;
// Java: import cn.wps.moffice.writer.io.reader.docxReader.importer.StyleId2Sti
use crate::cn::wps::moffice::writer::io::reader::docxReader::importer::StyleId2Sti::*;
// Java: import cn.wps.moffice.writer.io.reader.docxReader.importer.helpers.ConvertHelper
use crate::cn::wps::moffice::writer::io::reader::docxReader::importer::helpers::ConvertHelper::ConvertHelper;
use crate::cn::wps::moffice::writer::io::reader::docxReader::importer::helpers::ConvertHelper::Attributes;
// Java: import cn.wps.moffice.writer.io.reader.docxReader.importer.helpers.ParaPropsConverterSimple
use crate::cn::wps::moffice::writer::io::reader::docxReader::importer::helpers::ParaPropsConverterSimple::*;
// Java: import cn.wps.moffice.writer.io.reader.docxReader.importer.helpers.ShareConvertHelper
use crate::cn::wps::moffice::writer::io::reader::docxReader::importer::helpers::ShareConvertHelper::*;
// Java: import cn.wps.moffice.writer.io.reader.xml03.Xml03Context
use crate::cn::wps::moffice::writer::io::reader::xml03::Xml03Context::*;
// Java: import cn.wps.moffice.writer.io.reader.xml03.Xml03Utils
use crate::cn::wps::moffice::writer::io::reader::xml03::Xml03Utils::*;
// Java: import cn.wps.moffice.writer.io.reader.xml07.Xml07Context
use crate::cn::wps::moffice::writer::io::reader::xml07::Xml07Context::*;
use super::AbstractNumIdRedirection::AbstractNumIdRedirection;
use super::IOLvl::IOLvl;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

// java abstract class NumberingLevelImporter extends ImporterBase
pub struct NumberingLevelImporter {
    pub(crate) super_: ImporterBase,
    // java protected TextDocument mDocument;
    pub(crate) mDocument: *mut TextDocument,
    // java protected StyleId2Sti mStyleId2Sti;
    pub(crate) mStyleId2Sti: Rc<RefCell<StyleId2Sti>>,
    // java protected NumIdRedirection mNumIdRedirection;
    pub(crate) mNumIdRedirection: Rc<RefCell<NumIdRedirection>>,
    // java protected AbstractNumIdRedirection mAbstractNumIdRedirect;
    pub(crate) mAbstractNumIdRedirect: Rc<RefCell<AbstractNumIdRedirection>>,
    // java private Map<Integer, String> nfcToNumFmtMap = null;
    nfcToNumFmtMap: Option<HashMap<i32, String>>,
}

// java abstract class NumberingLevelImporter extends ImporterBase
impl NumberingLevelImporter {
    // java NumberingLevelImporter(...)
    pub fn NumberingLevelImporter(
        document: *mut TextDocument,
        styleId2Sti: Rc<RefCell<StyleId2Sti>>,
        numIdRedirection: Rc<RefCell<NumIdRedirection>>,
        abstractNumIdRedirect: Rc<RefCell<AbstractNumIdRedirection>>,
        mode: i32,
        xml07Context: Option<Xml07Context>,
        xml03Context: Option<Xml03Context>,
    ) -> Self {
      // java super(mode, xml07Context, xml03Context);
      let doc_opt = if document.is_null() { None } else { Some(()) };
      IOAssert::assertNotNull_msg("document should not be null", &doc_opt);
      // java this.mDocument = document;
      Self {
        super_: ImporterBase::ImporterBase(mode, xml07Context, xml03Context),
        mDocument: document,
        mStyleId2Sti: styleId2Sti,
        mNumIdRedirection: numIdRedirection,
        mAbstractNumIdRedirect: abstractNumIdRedirect,
        nfcToNumFmtMap: None,
      }
    }

    // java void importLevelStart(Attributes attributes)
    pub fn importLevelStart(cur: &mut IOLvl, attributes: &dyn Attributes) {
      // java IOAssert.assertNotNull("attributes should not be null", attributes);
      let marker = Some(());
      IOAssert::assertNotNull_msg("attributes should not be null", &marker);
      // java this.mCurIOLvl.mLvlIlvl = ShareConvertHelper.getInteger4STDecimalNumber(attributes, "ilvl");
      cur.mLvlIlvl = ShareConvertHelper::getInteger4STDecimalNumber__attributes_string(attributes, "ilvl");
      // java this.mCurIOLvl.mLvlTemplateCode = ShareConvertHelper.convertSTLongHexNumber(attributes, "tplc");
      cur.mLvlTemplateCode = ShareConvertHelper::convertSTLongHexNumber__attributes_string(attributes, "tplc");
      // java this.mCurIOLvl.mLvlTentative = ConvertHelper.getBoolean(attributes, "tentative");
      cur.mLvlTentative = ConvertHelper::getBoolean(attributes, "tentative");
    }

    // java void importLevelEnd(MutablePropertySet rPrBaseSet, MutablePropertySet pPrSet)
    pub fn importLevelEnd(cur: &mut IOLvl, rPrBaseSet: &MutablePropertySet, pPrSet: &MutablePropertySet) {
      // java if (null != pPrSet)
      let mut p = pPrSet.clone_mps();
      // java ParaPropsConverterSimple.correctPPrBase(pPrSet);
      ParaPropsConverterSimple::correctPPrBase(Some(&mut p));
      // java this.mCurIOLvl.mPPrSet = pPrSet.freeze();
      cur.mPPrSet = p.freeze();
      // java if (null != rPrBaseSet)
      let mut r = rPrBaseSet.clone_mps();
      // java this.mCurIOLvl.mRPrSet = rPrBaseSet.freeze();
      cur.mRPrSet = r.freeze();
    }

    // java void importLvlChild(int elementId, Attributes attributes)
    pub fn importLvlChild(&mut self, cur: &mut IOLvl, elementId: i32, attributes: &dyn Attributes) {
      // java IOAssert.assertNotNull("attributes should not be null", attributes);
      let marker = Some(());
      IOAssert::assertNotNull_msg("attributes should not be null", &marker);
      match elementId {
        // java case -1106578487: case 1017748453: break;
        -1106578487 | 1017748453 => {}
        // java case -1034390745:
        -1034390745 => {
          // java this.mCurIOLvl.mLvlNumberFormat = ConvertHelper.getString(attributes);
          cur.mLvlNumberFormat = ConvertHelper::getString(attributes);
          // java this.mCurIOLvl.mLvlNumFmtVal = ConvertHelper.getString(attributes);
          cur.mLvlNumFmtVal = ConvertHelper::getString(attributes);
        }
        // java case -1008274655:
        -1008274655 => {
          cur.mPStyle = ConvertHelper::getString(attributes);
        }
        // java case -567736435:
        -567736435 => {
          cur.mLvlRestart = ShareConvertHelper::getInteger4STDecimalNumber(attributes);
        }
        // java case 108971:
        108971 => {
          // java Integer anInt = ConvertHelper.getInt(attributes, "val");
          let anInt = ConvertHelper::getInt(attributes, "val");
          if let Some(anInt) = anInt {
            // java String temp = this.nfcToNumFmt(anInt);
            if let Some(temp) = self.nfcToNumFmt(anInt) {
              cur.mLvlNumberFormat = Some(temp.clone());
              cur.mLvlNumFmtVal = Some(temp);
            }
          }
        }
        // java case 3541666:
        3541666 => {
          // java String content = ConvertHelper.getString(attributes);
          let content = ConvertHelper::getString(attributes);
          // java content = this.isXml03() ? Xml03Utils.firstCharToLowerCase(content) : content;
          let content = if self.super_.isXml03() {
            Xml03Utils::firstCharToLowerCase(content.as_deref())
          } else {
            content
          };
          cur.mLvlSuff = content;
        }
        // java case 100472007:
        100472007 => {
          cur.mLvlIsLegal = ConvertHelper::toBoolean4NotNull(attributes);
        }
        // java case 103361787:
        103361787 => {
          cur.mLvlJc = Some(ConvertHelper::getStringHashCode(attributes));
        }
        // java case 109757538:
        109757538 => {
          cur.mLvlStart = ShareConvertHelper::getInteger4STDecimalNumber(attributes);
        }
        // java case 546733167:
        546733167 => {
          cur.mLvlTextVal = ConvertHelper::getString(attributes);
        }
        _ => {
          // java IOAssert.fail("it should not reach here");
          IOAssert::fail__string("it should not reach here");
        }
      }
    }

    // java private String nfcToNumFmt(Integer nfc)
    fn nfcToNumFmt(&mut self, nfc: i32) -> Option<String> {
      // java if (null == this.nfcToNumFmtMap)
      if self.nfcToNumFmtMap.is_none() {
        let pairs = [
          (0, "decimal"), (1, "upperRoman"), (2, "lowerRoman"), (3, "upperLetter"), (4, "lowerLetter"),
          (5, "ordinal"), (6, "cardinalText"), (7, "ordinalText"), (8, "hex"), (9, "chicago"),
          (10, "ideographDigital"), (11, "japaneseCounting"), (12, "aiueo"), (13, "iroha"),
          (14, "decimalFullWidth"), (15, "decimalHalfWidth"), (16, "japaneseLegal"),
          (17, "japaneseDigitalTenThousand"), (18, "decimalEnclosedCircle"), (19, "decimalFullWidth2"),
          (20, "aiueoFullWidth"), (21, "irohaFullWidth"), (22, "decimalZero"), (23, "bullet"),
          (24, "ganada"), (25, "chosung"), (26, "decimalEnclosedFullstop"), (27, "decimalEnclosedParen"),
          (28, "decimalEnclosedCircleChinese"), (29, "ideographEnclosedCircle"), (30, "ideographTraditional"),
          (31, "ideographZodiac"), (32, "ideographZodiacTraditional"), (33, "taiwaneseCounting"),
          (34, "ideographLegalTraditional"), (35, "taiwaneseCountingThousand"), (36, "taiwaneseDigital"),
          (37, "chineseCounting"), (38, "chineseLegalSimplified"), (39, "chineseCountingThousand"),
          (41, "koreanDigital"), (42, "koreanCounting"), (43, "koreanLegal"), (44, "koreanDigital2"),
          (45, "hebrew1"), (46, "arabicAlpha"), (47, "hebrew2"), (48, "arabicAbjad"),
          (49, "hindiVowels"), (50, "hindiConsonants"), (51, "hindiNumbers"), (52, "hindiCounting"),
          (53, "thaiLetters"), (54, "thaiNumbers"), (55, "thaiCounting"), (56, "vietnameseCounting"),
          (57, "numberInDash"), (58, "russianLower"), (59, "russianUpper"), (255, "none"),
        ];
        // java this.nfcToNumFmtMap = new HashMap(); this.nfcToNumFmtMap.put(...)
        let mut map = HashMap::new();
        for (k, v) in pairs {
          map.insert(k, v.to_string());
        }
        self.nfcToNumFmtMap = Some(map);
      }
      // java return (String)this.nfcToNumFmtMap.get(nfc);
      self.nfcToNumFmtMap.as_ref().and_then(|m| m.get(&nfc).cloned())
    }
}
