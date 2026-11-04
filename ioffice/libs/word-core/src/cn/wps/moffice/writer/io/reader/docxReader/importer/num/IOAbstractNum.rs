// Java: import cn.wps.base.assertion.IOAssert
use util::base::assertion::IOAssert::*;
// Java: import cn.wps.moffice.writer.data.autonum.LSTF
use crate::cn::wps::moffice::writer::data::autonum::LSTF::*;
// Java: import cn.wps.moffice.writer.data.autonum.LstData
use crate::cn::wps::moffice::writer::data::autonum::LstData::*;
// Java: import cn.wps.moffice.writer.data.autonum.LvlfData
use crate::cn::wps::moffice::writer::data::autonum::LvlfData::*;
// Java: import cn.wps.moffice.writer.io.reader.ImporterBase
use crate::cn::wps::moffice::writer::io::reader::ImporterBase::*;
// Java: import cn.wps.moffice.writer.io.reader.docxReader.importer.StyleId2Sti
use crate::cn::wps::moffice::writer::io::reader::docxReader::importer::StyleId2Sti::*;
// Java: import cn.wps.moffice.writer.io.reader.xml03.Xml03Context
use crate::cn::wps::moffice::writer::io::reader::xml03::Xml03Context::*;
// Java: import cn.wps.moffice.writer.io.reader.xml07.Xml07Context
use crate::cn::wps::moffice::writer::io::reader::xml07::Xml07Context::*;
use crate::cn::wps::moffice::writer::core::KStyle::TextDocument as SoftTextDocument;
use super::AbstractNumIdRedirection::AbstractNumIdRedirection;
use super::IOLvl::IOLvl;

// java final class IOAbstractNum extends ImporterBase
pub struct IOAbstractNum {
    pub(crate) super_: ImporterBase,
    // java String mName = null;
    pub(crate) mName: Option<String>,
    // java String mStyleLink = null;
    pub(crate) mStyleLink: Option<String>,
    // java String mNumStyleLink = null;
    pub(crate) mNumStyleLink: Option<String>,
    // java String mMultiLevelType = null;
    pub(crate) mMultiLevelType: Option<String>,
    // java Integer mTemplateCode = null;
    pub(crate) mTemplateCode: Option<i32>,
    // java Integer mAbstractNumId = null;
    pub(crate) mAbstractNumId: Option<i32>,
    // java int mLvlIndex = -1;
    pub(crate) mLvlIndex: i32,
    // java IOLvl[] mIOLvlArr = null;
    pub(crate) mIOLvlArr: [Option<IOLvl>; 9],
}

// java final class IOAbstractNum extends ImporterBase
impl IOAbstractNum {
    // java IOAbstractNum(int mode, Xml07Context xml07Context, Xml03Context xml03Context)
    pub fn IOAbstractNum(mode: i32, xml07Context: Option<Xml07Context>, xml03Context: Option<Xml03Context>) -> Self {
      // java super(mode, xml07Context, xml03Context);
      // java this.mIOLvlArr = new IOLvl[9];
      Self {
        super_: ImporterBase::ImporterBase(mode, xml07Context, xml03Context),
        mName: None,
        mStyleLink: None,
        mNumStyleLink: None,
        mMultiLevelType: None,
        mTemplateCode: None,
        mAbstractNumId: None,
        mLvlIndex: -1,
        mIOLvlArr: std::array::from_fn(|_| None),
      }
    }

    // java void dispose()
    pub fn dispose(&mut self) {
      // java this.mName = null;
      self.mName = None;
      self.mStyleLink = None;
      self.mNumStyleLink = None;
      self.mMultiLevelType = None;
      self.mTemplateCode = None;
      self.mAbstractNumId = None;
      // java for(int i = 0; i <= this.mLvlIndex; ++i)
      let last = self.mLvlIndex;
      let mut i = 0;
      while i <= last {
        if let Some(lvl) = self.mIOLvlArr.get_mut(i as usize).and_then(|s| s.as_mut()) {
          // java this.mIOLvlArr[i].dispose();
          lvl.dispose();
        }
        i += 1;
      }
      // java this.mLvlIndex = -1;
      self.mLvlIndex = -1;
    }

    // java boolean isStyleLinkExistence()
    pub fn isStyleLinkExistence(&self) -> bool {
      // java return null != this.mStyleLink && this.mStyleLink.length() > 0;
      self.mStyleLink.as_ref().is_some_and(|s| !s.is_empty())
    }

    // java boolean isNumStyleLinkExistence()
    pub fn isNumStyleLinkExistence(&self) -> bool {
      // java return null != this.mNumStyleLink && this.mNumStyleLink.length() > 0;
      self.mNumStyleLink.as_ref().is_some_and(|s| !s.is_empty())
    }

    // java IOLvl obtainIOLvl()
    pub fn obtainIOLvl(&mut self) -> &mut IOLvl {
      // java if (this.mLvlIndex >= 8)
      if self.mLvlIndex >= 8 {
        self.mLvlIndex -= 1;
      }
      // java ++this.mLvlIndex;
      self.mLvlIndex += 1;
      let idx = self.mLvlIndex as usize;
      // java if (null == this.mIOLvlArr[this.mLvlIndex])
      if self.mIOLvlArr[idx].is_none() {
        let mode = self.super_.mode();
        let xml07 = self.super_.xml07Context().cloned();
        let xml03 = self.super_.xml03Context().cloned();
        self.mIOLvlArr[idx] = Some(IOLvl::IOLvl(mode, xml07, xml03));
      }
      self.mIOLvlArr[idx].as_mut().expect("io lvl")
    }

    // java void convert2LstData(...)
    pub fn convert2LstData(&mut self, document: &SoftTextDocument, styleId2Sti: &StyleId2Sti, abstIdRedirect: &mut AbstractNumIdRedirection, lstData: &mut LstData) {
      // java IOAssert.assertNotNull("lstData should not be null", lstData);
      let marker = Some(());
      IOAssert::assertNotNull_msg("lstData should not be null", &marker);
      // java this.setLSTF(styleId2Sti, abstIdRedirect, lstData);
      self.setLSTF(styleId2Sti, abstIdRedirect, lstData);
      // java this.setLvlfDatas(document, lstData);
      self.setLvlfDatas(document, lstData);
    }

    // java private void setLSTF(...)
    fn setLSTF(&mut self, styleId2Sti: &StyleId2Sti, abstIdRedirect: &mut AbstractNumIdRedirection, lstData: &mut LstData) {
      // java LSTF lstf = lstData.getLSTF();
      let mut lstf = lstData.getLSTF().unwrap_or_default();
      // java this.getLstf(styleId2Sti, abstIdRedirect, lstf);
      self.getLstf(styleId2Sti, abstIdRedirect, &mut lstf);
      // java lstData.setLETF(lstf);
      lstData.setLETF(Some(lstf));
    }

    // java private LSTF getLstf(...)
    fn getLstf(&mut self, styleId2Sti: &StyleId2Sti, abstIdRedirect: &mut AbstractNumIdRedirection, lstf: &mut LSTF) {
      // java lstf.setName(this.mName);
      if let Some(name) = self.mName.as_deref() {
        lstf.setName(name);
      }
      // java this.setTplc(lstf);
      self.setTplc(lstf);
      // java this.setLsid(abstIdRedirect, lstf);
      self.setLsid(abstIdRedirect, lstf);
      // java this.setRgIstd(styleId2Sti, lstf);
      self.setRgIstd(styleId2Sti, lstf);
      // java this.convertMultiLevelType(lstf);
      self.convertMultiLevelType(lstf);
    }

    // java private void setTplc(LSTF lstf)
    fn setTplc(&self, lstf: &mut LSTF) {
      // java int tplc = null == this.mTemplateCode ? -1 : this.mTemplateCode;
      let tplc = self.mTemplateCode.unwrap_or(-1);
      // java lstf.setTplc(tplc);
      lstf.setTplc(tplc);
    }

    // java private void convertMultiLevelType(LSTF lstf)
    fn convertMultiLevelType(&mut self, lstf: &mut LSTF) {
      // java if (null != this.mMultiLevelType)
      if let Some(kind) = self.mMultiLevelType.clone() {
        // java lstf.setfSimpleList("singleLevel".equals(this.mMultiLevelType));
        lstf.setfSimpleList(kind == "singleLevel");
        // java boolean isFHybrid = "hybridMultilevel".equals(this.mMultiLevelType);
        let isFHybrid = kind == "hybridMultilevel";
        // java lstf.setfHybrid(isFHybrid);
        lstf.setfHybrid(isFHybrid);
        // java if (isFHybrid) lstf.initRgTplc();
        if isFHybrid {
          lstf.initRgTplc();
        }
        // java this.setRgTplc(lstf);
        self.setRgTplc(lstf);
      }
    }

    // java private void setRgTplc(LSTF lstf)
    fn setRgTplc(&self, lstf: &mut LSTF) {
      // java for(int i = 0; i <= this.mLvlIndex; ++i)
      let mut i = 0;
      while i <= self.mLvlIndex {
        if let Some(ioLvl) = self.mIOLvlArr.get(i as usize).and_then(|s| s.as_ref()) {
          // java ioLvl.setRgTplc(i, lstf);
          ioLvl.setRgTplc(i, lstf);
        }
        i += 1;
      }
    }

    // java private void setLsid(...)
    fn setLsid(&self, abstIdRedirect: &mut AbstractNumIdRedirection, lstf: &mut LSTF) {
      // java Integer redirectedAbstractNumId = abstIdRedirect.getId(this.mAbstractNumId);
      let redirected = abstIdRedirect.getId(self.mAbstractNumId);
      // java if (null == redirectedAbstractNumId)
      if redirected.is_none() {
        // java lstf.setLsid(abstIdRedirect.put(this.mAbstractNumId));
        lstf.setLsid(abstIdRedirect.put(self.mAbstractNumId));
      } else {
        // java lstf.setLsid(redirectedAbstractNumId);
        lstf.setLsid(redirected.unwrap_or(0));
      }
    }

    // java private void setRgIstd(...)
    fn setRgIstd(&self, styleId2Sti: &StyleId2Sti, lstf: &mut LSTF) {
      // java lstf.initRgIstd();
      lstf.initRgIstd();
      // java if (null != styleId2Sti)
      // java this.convertPStyle2RgIstd(styleId2Sti, lstf);
      self.convertPStyle2RgIstd(styleId2Sti, lstf);
    }

    // java private void convertPStyle2RgIstd(...)
    fn convertPStyle2RgIstd(&self, styleId2Sti: &StyleId2Sti, lstf: &mut LSTF) {
      let mut i = 0;
      while i <= self.mLvlIndex {
        if let Some(ioLvl) = self.mIOLvlArr.get(i as usize).and_then(|s| s.as_ref()) {
          // java ioLvl.setRgIstd(styleId2Sti, i, lstf);
          ioLvl.setRgIstd(styleId2Sti, i, lstf);
        }
        i += 1;
      }
    }

    // java private void setLvlfDatas(TextDocument document, LstData lstData)
    fn setLvlfDatas(&mut self, document: &SoftTextDocument, lstData: &mut LstData) {
      // java if (null == lstData.getLvlfDataArray())
      if lstData.getLvlfDataArray().map(|v| v.is_empty()).unwrap_or(true) {
        // java int size = this.mLvlIndex + 1;
        let size = self.mLvlIndex + 1;
        if size <= 0 {
          // java this.setDefaultLvlfDatas(document, lstData);
          self.setDefaultLvlfDatas(document, lstData);
        } else {
          // java this.setLvlfDatas(document, size, lstData);
          self.fillLvlfDatas(document, size, lstData);
        }
      }
    }

    // java private void setDefaultLvlfDatas(...)
    fn setDefaultLvlfDatas(&self, document: &SoftTextDocument, lstData: &mut LstData) {
      let mode = self.super_.mode();
      let xml07 = self.super_.xml07Context().cloned();
      let xml03 = self.super_.xml03Context().cloned();
      // java if ("singleLevel".equals(this.mMultiLevelType))
      let datas = if self.mMultiLevelType.as_deref() == Some("singleLevel") {
        // java lvlfDatas = new LvlfData[1];
        let mut ioLvl = IOLvl::IOLvl(mode, xml07, xml03);
        // java ioLvl.mLvlIlvl = 0;
        ioLvl.mLvlIlvl = Some(0);
        vec![ioLvl.getLvlfData(document)]
      } else {
        // java int size = 9;
        let mut datas = Vec::with_capacity(9);
        let mut i = 0;
        while i < 9 {
          let mut ioLvl = IOLvl::IOLvl(mode, xml07.clone(), xml03.clone());
          // java ioLvl.mLvlIlvl = i;
          ioLvl.mLvlIlvl = Some(i);
          datas.push(ioLvl.getLvlfData(document));
          i += 1;
        }
        datas
      };
      // java lstData.setLvlfDataArray(lvlfDatas);
      lstData.setLvlfDataArray(&datas);
    }

    // java private void setLvlfDatas(TextDocument document, int size, LstData lstData)
    fn fillLvlfDatas(&mut self, document: &SoftTextDocument, size: i32, lstData: &mut LstData) {
      // java if (size == 1 && !this.mIOLvlArr[0].isValid())
      if size == 1 && self.mIOLvlArr.first().and_then(|s| s.as_ref()).is_some_and(|l| !l.isValid()) {
        if let Some(lvl) = self.mIOLvlArr[0].as_mut() {
          // java this.mIOLvlArr[0].mLvlIlvl = 0;
          lvl.mLvlIlvl = Some(0);
          let data = lvl.getLvlfData(document);
          // java lstData.setLvlfDataArray(lvlfDatas);
          lstData.setLvlfDataArray(&vec![data]);
        }
        return;
      }
      let mut datas: Vec<Option<LvlfData>> = (0..size).map(|_| None).collect();
      let mut i = 0;
      while i < size {
        if let Some(ioLvl) = self.mIOLvlArr.get(i as usize).and_then(|s| s.as_ref()) {
          // java if (ioLvl.isValid())
          if ioLvl.isValid() {
            let ilvl = ioLvl.mLvlIlvl.unwrap_or(0);
            if let Some(slot) = datas.get_mut(ilvl as usize) {
              // java lvlfDatas[ioLvl.mLvlIlvl] = ioLvl.getLvlfData(document);
              *slot = Some(ioLvl.getLvlfData(document));
            }
          }
        }
        i += 1;
      }
      let flat: Vec<LvlfData> = datas.into_iter().flatten().collect();
      lstData.setLvlfDataArray(&flat);
    }
}
