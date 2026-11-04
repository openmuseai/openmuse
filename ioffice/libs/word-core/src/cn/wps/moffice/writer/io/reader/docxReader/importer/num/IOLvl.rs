// Java: import cn.wps.base.assertion.IOAssert
use util::base::assertion::IOAssert::*;
// Java: import cn.wps.moffice.property.PropertySet
use util::moffice::property::PropertySet::*;
// Java: import cn.wps.moffice.writer.core.TextDocument
use crate::cn::wps::moffice::writer::core::KStyle::TextDocument as SoftTextDocument;
// Java: import cn.wps.moffice.writer.data.autonum.LSTF
use crate::cn::wps::moffice::writer::data::autonum::LSTF::*;
// Java: import cn.wps.moffice.writer.data.autonum.LVLF
use crate::cn::wps::moffice::writer::data::autonum::LVLF::*;
// Java: import cn.wps.moffice.writer.data.autonum.LvlfData
use crate::cn::wps::moffice::writer::data::autonum::LvlfData::*;
// Java: import cn.wps.moffice.writer.io.reader.ImporterBase
use crate::cn::wps::moffice::writer::io::reader::ImporterBase::*;
// Java: import cn.wps.moffice.writer.io.reader.docxReader.importer.StyleId2Sti
use crate::cn::wps::moffice::writer::io::reader::docxReader::importer::StyleId2Sti::*;
// Java: import cn.wps.moffice.writer.io.reader.docxReader.importer.helpers.ConvertHelper
use crate::cn::wps::moffice::writer::io::reader::docxReader::importer::helpers::ConvertHelper::*;
// Java: import cn.wps.moffice.writer.io.reader.docxReader.importer.helpers.ImporterHelper
use crate::cn::wps::moffice::writer::io::reader::docxReader::importer::helpers::ImporterHelper::*;
// Java: import cn.wps.moffice.writer.io.reader.xml03.Xml03Context
use crate::cn::wps::moffice::writer::io::reader::xml03::Xml03Context::*;
// Java: import cn.wps.moffice.writer.io.reader.xml07.Xml07Context
use crate::cn::wps::moffice::writer::io::reader::xml07::Xml07Context::*;

// java public static final int MAX_LEVEL_COUNT = 9;
pub const MAX_LEVEL_COUNT: i32 = 9;

// java final class IOLvl extends ImporterBase
pub struct IOLvl {
    pub(crate) super_: ImporterBase,
    // java Integer mLvlIlvl = null;
    pub(crate) mLvlIlvl: Option<i32>,
    // java Integer mLvlStart = null;
    pub(crate) mLvlStart: Option<i32>,
    // java Integer mLvlRestart = null;
    pub(crate) mLvlRestart: Option<i32>,
    // java Integer mLvlTemplateCode = null;
    pub(crate) mLvlTemplateCode: Option<i32>,
    // java Integer mLvlJc = null;
    pub(crate) mLvlJc: Option<i32>,
    // java String mPStyle = null;
    pub(crate) mPStyle: Option<String>,
    // java String mLvlSuff = null;
    pub(crate) mLvlSuff: Option<String>,
    // java String mLvlTextVal = null;
    pub(crate) mLvlTextVal: Option<String>,
    // java String mLvlNumFmtVal = null;
    pub(crate) mLvlNumFmtVal: Option<String>,
    // java String mLvlNumberFormat = null;
    pub(crate) mLvlNumberFormat: Option<String>,
    // java Boolean mLvlIsLegal = null;
    pub(crate) mLvlIsLegal: Option<bool>,
    // java Boolean mLvlTentative = null;
    pub(crate) mLvlTentative: Option<bool>,
    // java PropertySet mRPrSet;
    pub(crate) mRPrSet: PropertySet,
    // java PropertySet mPPrSet;
    pub(crate) mPPrSet: PropertySet,
}

// java final class IOLvl extends ImporterBase
impl IOLvl {
    // java public IOLvl(int mode, Xml07Context xml07Context, Xml03Context xml03Context)
    pub fn IOLvl(mode: i32, xml07Context: Option<Xml07Context>, xml03Context: Option<Xml03Context>) -> Self {
      // java super(mode, xml07Context, xml03Context);
      // java this.mRPrSet = PropertySet.empty;
      // java this.mPPrSet = PropertySet.empty;
      Self {
        super_: ImporterBase::ImporterBase(mode, xml07Context, xml03Context),
        mLvlIlvl: None,
        mLvlStart: None,
        mLvlRestart: None,
        mLvlTemplateCode: None,
        mLvlJc: None,
        mPStyle: None,
        mLvlSuff: None,
        mLvlTextVal: None,
        mLvlNumFmtVal: None,
        mLvlNumberFormat: None,
        mLvlIsLegal: None,
        mLvlTentative: None,
        mRPrSet: PropertySet::empty(),
        mPPrSet: PropertySet::empty(),
      }
    }

    // java void dispose()
    pub fn dispose(&mut self) {
      // java this.mLvlIlvl = null;
      self.mLvlIlvl = None;
      self.mLvlStart = None;
      self.mLvlRestart = None;
      self.mLvlTemplateCode = None;
      self.mLvlJc = None;
      self.mPStyle = None;
      self.mLvlSuff = None;
      self.mLvlTextVal = None;
      self.mLvlNumFmtVal = None;
      self.mLvlNumberFormat = None;
      self.mLvlIsLegal = None;
      self.mLvlTentative = None;
      // java this.mRPrSet = PropertySet.empty;
      self.mRPrSet = PropertySet::empty();
      // java this.mPPrSet = PropertySet.empty;
      self.mPPrSet = PropertySet::empty();
    }

    // java boolean isValid()
    pub fn isValid(&self) -> bool {
      // java return null != this.mLvlIlvl && this.mLvlIlvl >= 0 && this.mLvlIlvl <= 8;
      self.mLvlIlvl.is_some_and(|v| (0..=8).contains(&v))
    }

    // java void setRgIstd(StyleId2Sti styleId2Sti, int index, LSTF lstf)
    pub fn setRgIstd(&self, styleId2Sti: &StyleId2Sti, index: i32, lstf: &mut LSTF) {
      // java if (null != this.mPStyle && this.mPStyle.length() > 0)
      if self.mPStyle.as_ref().is_some_and(|s| !s.is_empty()) {
        // java Integer sti = styleId2Sti.getSti(this.mPStyle);
        let sti = styleId2Sti.getSti(self.mPStyle.as_deref().unwrap_or(""));
        // java IOAssert.assertNotNull("sti should not be null!", sti);
        IOAssert::assertNotNull_msg("sti should not be null!", &sti);
        // java lstf.setRgistd(index, null != sti ? sti.shortValue() : 4095);
        lstf.setRgistd__int_short(index, sti.unwrap_or(4095) as i16);
      }
    }

    // java void setRgTplc(int index, LSTF lstf)
    pub fn setRgTplc(&self, index: i32, lstf: &mut LSTF) {
      // java if (null != this.mLvlTemplateCode)
      if let Some(code) = self.mLvlTemplateCode {
        // java lstf.setfHybrid(true);
        lstf.setfHybrid(true);
        // java lstf.setfSimpleList(false);
        lstf.setfSimpleList(false);
        // java int[] rgtplc = lstf.getRgtplc();
        let rgtplc = lstf.getRgtplc();
        // java if (null == rgtplc || rgtplc.length == 0)
        if rgtplc.is_empty() {
          // java lstf.initRgTplc();
          lstf.initRgTplc();
        }
        // java lstf.setRgtplc(index, this.mLvlTemplateCode);
        lstf.setRgtplc__int_int(index, code);
      }
    }

    // java LvlfData getLvlfData(TextDocument document)
    pub fn getLvlfData(&self, document: &SoftTextDocument) -> LvlfData {
      // java LvlfData lvlfData = new LvlfData(document);
      let mut lvlfData = LvlfData::LvlfData(document);
      // java lvlfData.setLVLF(this.getLvlf());
      lvlfData.setLVLF(Some(self.getLvlf()));
      // java lvlfData.setParaProp(this.mPPrSet);
      lvlfData.setParaProp(Some(self.mPPrSet.clone()));
      // java lvlfData.setRunProp(this.mRPrSet);
      lvlfData.setRunProp(Some(self.mRPrSet.clone()));
      // java lvlfData.setXstStr(this.getXst());
      lvlfData.setXstStr(Some(self.getXst()));
      // java return lvlfData;
      lvlfData
    }

    // java private LVLF getLvlf()
    fn getLvlf(&self) -> LVLF {
      // java LVLF lvlf = new LVLF();
      let mut lvlf = LVLF::default();
      // java lvlf.iStartAt = null == this.mLvlStart ? 1 : this.mLvlStart;
      lvlf.iStartAt = self.mLvlStart.unwrap_or(1);
      // java lvlf.jc = (byte)(null == this.mLvlJc ? 0 : ImporterHelper.convertAlignment(this.mLvlJc));
      lvlf.jc = self.mLvlJc.map(ImporterHelper::convertAlignment).unwrap_or(0) as u8;
      // java lvlf.fLegal = null == this.mLvlIsLegal ? false : this.mLvlIsLegal;
      lvlf.fLegal = self.mLvlIsLegal.unwrap_or(false);
      // java lvlf.fTentative = null == this.mLvlTentative ? false : this.mLvlTentative;
      lvlf.fTentative = self.mLvlTentative.unwrap_or(false);
      // java lvlf.ixchFollow = null == this.mLvlSuff ? 0 : (byte)ImporterHelper.convertIxchFollow(this.mLvlSuff);
      lvlf.ixchFollow = self.mLvlSuff.as_deref().map(ImporterHelper::convertIxchFollow).unwrap_or(0) as u8;
      // java lvlf.rgbxchNums = getRgbxchNums(this.mLvlTextVal);
      lvlf.rgbxchNums = Some(Self::getRgbxchNums(self.mLvlTextVal.as_deref()));
      // java lvlf.iRestartAfterLevel = this.mLvlIlvl.byteValue();
      lvlf.iRestartAfterLevel = self.mLvlIlvl.unwrap_or(0) as u8;
      // java this.setNfc(lvlf);
      self.setNfc(&mut lvlf);
      // java this.setRestart(lvlf);
      self.setRestart(&mut lvlf);
      lvlf
    }

    // java private void setRestart(LVLF lvlf)
    fn setRestart(&self, lvlf: &mut LVLF) {
      // java if (null != this.mLvlRestart && this.mLvlRestart >= 0 && this.mLvlRestart < this.mLvlIlvl)
      if let (Some(restart), Some(ilvl)) = (self.mLvlRestart, self.mLvlIlvl) {
        if restart >= 0 && restart < ilvl {
          // java lvlf.fNoRestart = true;
          lvlf.fNoRestart = true;
          // java lvlf.iRestartAfterLevel = this.mLvlRestart.byteValue();
          lvlf.iRestartAfterLevel = restart as u8;
        }
      }
    }

    // java private void setNfc(LVLF lvlf)
    fn setNfc(&self, lvlf: &mut LVLF) {
      // java if (null != this.mLvlNumberFormat)
      if let Some(fmt) = self.mLvlNumberFormat.as_deref() {
        // java Integer nfc = ConvertHelper.xmlNfc2MsoNfc(this.mLvlNumberFormat, this.isXml03());
        let nfc = ConvertHelper::xmlNfc2MsoNfc(fmt, self.super_.isXml03());
        // java lvlf.nfc = (byte)(null == nfc ? 0 : nfc);
        lvlf.nfc = nfc.unwrap_or(0) as u8;
      }
    }

    // java private static byte[] getRgbxchNums(String lvlTextVal)
    fn getRgbxchNums(lvlTextVal: Option<&str>) -> Vec<u8> {
      // java byte[] rgbxchNums = new byte[9];
      let mut rgbxchNums = vec![0u8; 9];
      // java if (null != lvlTextVal)
      if let Some(text) = lvlTextVal {
        // java getRgbxchNums(lvlTextVal, rgbxchNums);
        Self::fillRgbxchNums(text, &mut rgbxchNums);
      }
      rgbxchNums
    }

    // java private static void getRgbxchNums(String lvlTextVal, byte[] rgbxchNums)
    fn fillRgbxchNums(lvlTextVal: &str, rgbxchNums: &mut [u8]) {
      // java int rgbxchIndex = 0;
      let mut rgbxchIndex: usize = 0;
      let chars: Vec<char> = lvlTextVal.chars().collect();
      // java int len = lvlTextVal.length();
      let len = chars.len();
      // java int i = 0;
      let mut i = 0usize;
      let mut k = 0i32;
      // java for(... i < len && i + 1 != len && rgbxchIndex < 9 ...)
      while i < len && i + 1 != len && rgbxchIndex < 9 {
        // java if ('%' == lvlTextVal.charAt(i))
        if chars[i] == '%' {
          i += 1;
          // java if (isValidLevelNum(lvlTextVal.charAt(i)))
          if Self::isValidLevelNum(chars[i]) {
            // java rgbxchNums[rgbxchIndex++] = (byte)(k + 1);
            rgbxchNums[rgbxchIndex] = (k + 1) as u8;
            rgbxchIndex += 1;
          }
        }
        i += 1;
        k += 1;
      }
    }

    // java private static boolean isValidLevelNum(char codePoint)
    fn isValidLevelNum(codePoint: char) -> bool {
      // java return '1' <= codePoint && codePoint <= '9';
      ('1'..='9').contains(&codePoint)
    }

    // java private String getXst()
    fn getXst(&self) -> String {
      // java if (null == this.mLvlTextVal)
      let Some(text) = self.mLvlTextVal.as_deref() else {
        // java return "";
        return String::new();
      };
      // java int len = this.mLvlTextVal.length();
      if text.is_empty() {
        // java return "";
        return String::new();
      }
      // java return "bullet".equals(this.mLvlNumFmtVal) ? getXst4BulletFmt(this.mLvlTextVal) : this.convertLvlTextVal2Xst(len);
      if self.mLvlNumFmtVal.as_deref() == Some("bullet") {
        return Self::getXst4BulletFmt(text);
      }
      self.convertLvlTextVal2Xst(text)
    }

    // java private static String getXst4BulletFmt(String lvlTextVal)
    fn getXst4BulletFmt(lvlTextVal: &str) -> String {
      // java return String.valueOf(lvlTextVal.charAt(0));
      lvlTextVal.chars().next().map(|c| c.to_string()).unwrap_or_default()
    }

    // java private String convertLvlTextVal2Xst(int len)
    fn convertLvlTextVal2Xst(&self, text: &str) -> String {
      let chars: Vec<char> = text.chars().collect();
      let len = chars.len();
      // java char[] data = new char[len];
      let mut data: Vec<char> = Vec::with_capacity(len);
      // java int i = 0;
      let mut i = 0usize;
      let ilvl = self.mLvlIlvl.unwrap_or(0);
      while i < len {
        // java char cur = this.mLvlTextVal.charAt(i);
        let cur = chars[i];
        // java if ('%' == cur)
        if cur == '%' {
          i += 1;
          // java if (i == len) break;
          if i == len {
            break;
          }
          // java char next = this.mLvlTextVal.charAt(i);
          let next = chars[i];
          // java int zeroBased = next - 49;
          let zeroBased = (next as i32) - 49;
          // java if (zeroBased >= 0 && zeroBased <= this.mLvlIlvl)
          if zeroBased >= 0 && zeroBased <= ilvl {
            // java data[dataIndex++] = (char)zeroBased;
            data.push(char::from_u32(zeroBased as u32).unwrap_or('\0'));
          } else if !next.is_ascii_digit() {
            // java data[dataIndex++] = '%'; data[dataIndex++] = next;
            data.push('%');
            data.push(next);
          }
        } else {
          // java data[dataIndex++] = cur;
          data.push(cur);
        }
        i += 1;
      }
      // java return new String(data, 0, dataIndex);
      data.into_iter().collect()
    }
}
