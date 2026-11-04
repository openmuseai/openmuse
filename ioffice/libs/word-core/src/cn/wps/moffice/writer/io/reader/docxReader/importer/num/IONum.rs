// Java: import cn.wps.base.assertion.IOAssert
use util::base::assertion::IOAssert::*;
// Java: import cn.wps.moffice.writer.core.TextDocument
use crate::cn::wps::moffice::writer::core::KStyle::TextDocument as SoftTextDocument;
// Java: import cn.wps.moffice.writer.data.autonum.LFO
use crate::cn::wps::moffice::writer::data::autonum::LFO::*;
// Java: import cn.wps.moffice.writer.data.autonum.LFOLVL
use crate::cn::wps::moffice::writer::data::autonum::LFOLVL::*;
// Java: import cn.wps.moffice.writer.data.autonum.LfoData
use crate::cn::wps::moffice::writer::data::autonum::LfoData::*;
// Java: import cn.wps.moffice.writer.data.autonum.LfoLvlData
use crate::cn::wps::moffice::writer::data::autonum::LfoLvlData::*;
// Java: import cn.wps.moffice.writer.io.reader.ImporterBase
use crate::cn::wps::moffice::writer::io::reader::ImporterBase::*;
// Java: import cn.wps.moffice.writer.io.reader.xml03.Xml03Context
use crate::cn::wps::moffice::writer::io::reader::xml03::Xml03Context::*;
// Java: import cn.wps.moffice.writer.io.reader.xml07.Xml07Context
use crate::cn::wps::moffice::writer::io::reader::xml07::Xml07Context::*;
use super::AbstractNumIdRedirection::AbstractNumIdRedirection;
use super::IOLvl::IOLvl;
use super::IOLvlOverride::IOLvlOverride;

// java final class IONum extends ImporterBase
pub struct IONum {
    pub(crate) super_: ImporterBase,
    // java Integer mNumId = null;
    pub(crate) mNumId: Option<i32>,
    // java Integer mAbstractNumId = null;
    pub(crate) mAbstractNumId: Option<i32>,
    // java int mLvlOverrideIndex = -1;
    mLvlOverrideIndex: i32,
    // java IOLvlOverride[] mIOLvlArr = new IOLvlOverride[9];
    mIOLvlArr: [Option<IOLvlOverride>; 9],
}

// java final class IONum extends ImporterBase
impl IONum {
    // java IONum(int mode, Xml07Context xml07Context, Xml03Context xml03Context)
    pub fn IONum(mode: i32, xml07Context: Option<Xml07Context>, xml03Context: Option<Xml03Context>) -> Self {
      // java super(mode, xml07Context, xml03Context);
      Self {
        super_: ImporterBase::ImporterBase(mode, xml07Context, xml03Context),
        mNumId: None,
        mAbstractNumId: None,
        mLvlOverrideIndex: -1,
        mIOLvlArr: std::array::from_fn(|_| None),
      }
    }

    // java boolean isValid()
    pub fn isValid(&self) -> bool {
      // java return null != this.mNumId && null != this.mAbstractNumId;
      self.mNumId.is_some() && self.mAbstractNumId.is_some()
    }

    // java private int getLvlOverrideConut()
    fn getLvlOverrideConut(&self) -> i32 {
      // java return this.mLvlOverrideIndex + 1;
      self.mLvlOverrideIndex + 1
    }

    // java IOLvlOverride obtainIOLvlOverride()
    pub fn obtainIOLvlOverride(&mut self) -> &mut IOLvlOverride {
      // java if (this.mLvlOverrideIndex >= 8)
      if self.mLvlOverrideIndex >= 8 {
        // java --this.mLvlOverrideIndex;
        self.mLvlOverrideIndex -= 1;
      }
      // java ++this.mLvlOverrideIndex;
      self.mLvlOverrideIndex += 1;
      let idx = self.mLvlOverrideIndex as usize;
      // java if (null == this.mIOLvlArr[this.mLvlOverrideIndex])
      if self.mIOLvlArr[idx].is_none() {
        let mode = self.super_.mode();
        let xml07 = self.super_.xml07Context().cloned();
        let xml03 = self.super_.xml03Context().cloned();
        // java this.mIOLvlArr[this.mLvlOverrideIndex] = new IOLvlOverride(...);
        self.mIOLvlArr[idx] = Some(IOLvlOverride::IOLvlOverride(mode, xml07, xml03));
      }
      // java return this.mIOLvlArr[this.mLvlOverrideIndex];
      self.mIOLvlArr[idx].as_mut().expect("lvl override")
    }

    // java 当前已 obtain 的 override（不前进下标）
    pub fn currentOverride(&mut self) -> Option<&mut IOLvlOverride> {
      if self.mLvlOverrideIndex < 0 {
        return None;
      }
      self.mIOLvlArr.get_mut(self.mLvlOverrideIndex as usize).and_then(|s| s.as_mut())
    }

    // java void reset()
    pub fn reset(&mut self) {
      // java this.mNumId = null;
      self.mNumId = None;
      // java this.mAbstractNumId = null;
      self.mAbstractNumId = None;
      // java for(int i = 0; i <= this.mLvlOverrideIndex; ++i)
      let last = self.mLvlOverrideIndex;
      let mut i = 0;
      while i <= last {
        // java this.mIOLvlArr[i].dispose();
        if let Some(slot) = self.mIOLvlArr.get_mut(i as usize).and_then(|s| s.as_mut()) {
          slot.dispose();
        }
        i += 1;
      }
      // java this.mLvlOverrideIndex = -1;
      self.mLvlOverrideIndex = -1;
    }

    // java LfoData convert2LfoData(...)
    pub fn convert2LfoData(&self, document: &SoftTextDocument, abstractNumIdRedirect: &mut AbstractNumIdRedirection, data: &mut LfoData) {
      // java IOAssert.assertNotNull("data should not be null", data);
      let data_opt = Some(());
      IOAssert::assertNotNull_msg("data should not be null", &data_opt);
      // java data.setLFO(this.getLfo(abstractNumIdRedirect));
      data.setLFO(Some(self.getLfo(abstractNumIdRedirect)));
      // java int count = this.getLvlOverrideConut();
      let count = self.getLvlOverrideConut();
      let mut i = 0;
      while i < count {
        // java data.append(this.getLfoLvlData(this.mIOLvlArr[i], document));
        if let Some(ov) = self.mIOLvlArr.get(i as usize).and_then(|s| s.as_ref()) {
          let lvl = self.getLfoLvlData(ov, document);
          data.append(&lvl);
        }
        i += 1;
      }
    }

    // java private LfoLvlData getLfoLvlData(IOLvlOverride ioLeLvlOverride, TextDocument document)
    fn getLfoLvlData(&self, ioLeLvlOverride: &IOLvlOverride, document: &SoftTextDocument) -> LfoLvlData {
      // java LfoLvlData lfoLvlData = new LfoLvlData();
      let mut lfoLvlData = LfoLvlData::default();
      // java lfoLvlData.setLFOLVL(getLfoLvl(ioLeLvlOverride));
      lfoLvlData.setLFOLVL(Some(Self::getLfoLvl(ioLeLvlOverride)));
      // java this.setLvlfData(ioLeLvlOverride.mIOLvl, document, lfoLvlData);
      self.setLvlfData(ioLeLvlOverride.mIOLvl.as_ref(), document, &mut lfoLvlData);
      lfoLvlData
    }

    // java private void setLvlfData(IOLvl ioLvl, TextDocument document, LfoLvlData lfoLvlData)
    fn setLvlfData(&self, ioLvl: Option<&IOLvl>, document: &SoftTextDocument, lfoLvlData: &mut LfoLvlData) {
      // java if (null != ioLvl && ioLvl.isValid())
      if let Some(ioLvl) = ioLvl {
        if ioLvl.isValid() {
          // java lfoLvlData.setLvlfData(ioLvl.getLvlfData(document));
          lfoLvlData.setLvlfData(Some(ioLvl.getLvlfData(document)));
        }
      }
    }

    // java private static LFOLVL getLfoLvl(IOLvlOverride ioLeLvlOverride)
    fn getLfoLvl(ioLeLvlOverride: &IOLvlOverride) -> LFOLVL {
      // java LFOLVL lfoLvl = new LFOLVL();
      let mut lfoLvl = LFOLVL::default();
      // java Integer startAt = ioLeLvlOverride.mStartOverride;
      let startAt = ioLeLvlOverride.mStartOverride;
      // java lfoLvl.iStartAt = null != startAt ? startAt : 1;
      lfoLvl.iStartAt = startAt.unwrap_or(1);
      // java lfoLvl.fStartAt = null != startAt;
      lfoLvl.fStartAt = startAt.is_some();
      // java Integer ilvl = ioLeLvlOverride.mIlvl;
      // java lfoLvl.ilvl = ilvl != null ? ilvl.byteValue() : 0;
      lfoLvl.ilvl = ioLeLvlOverride.mIlvl.unwrap_or(0) as u8;
      // java IOLvl ioLvl = ioLeLvlOverride.mIOLvl;
      // java lfoLvl.fFormatting = null == ioLvl ? false : ioLvl.isValid();
      lfoLvl.fFormatting = ioLeLvlOverride.mIOLvl.as_ref().is_some_and(|l| l.isValid());
      lfoLvl
    }

    // java private LFO getLfo(AbstractNumIdRedirection abstractNumIdRedirect)
    fn getLfo(&self, abstractNumIdRedirect: &mut AbstractNumIdRedirection) -> LFO {
      // java LFO lfo = new LFO();
      let mut lfo = LFO::default();
      // java lfo.lsid = this.getRedirectedAbstractNumId(abstractNumIdRedirect);
      lfo.lsid = self.getRedirectedAbstractNumId(abstractNumIdRedirect);
      // java lfo.clfolvl = (byte)this.getLvlOverrideConut();
      lfo.clfolvl = self.getLvlOverrideConut() as u8;
      lfo
    }

    // java private int getRedirectedAbstractNumId(...)
    fn getRedirectedAbstractNumId(&self, abstractNumIdRedirect: &mut AbstractNumIdRedirection) -> i32 {
      // java Integer abstractNumId = this.mAbstractNumId;
      let abstractNumId = self.mAbstractNumId;
      // java IOAssert.assertNotNull("abstractNumId should not be null", abstractNumId);
      IOAssert::assertNotNull_msg("abstractNumId should not be null", &abstractNumId);
      // java Integer redirectedAbstractNumId = abstractNumIdRedirect.getId(abstractNumId);
      let mut redirected = abstractNumIdRedirect.getId(abstractNumId);
      // java if (null == redirectedAbstractNumId)
      if redirected.is_none() {
        // java redirectedAbstractNumId = abstractNumIdRedirect.put(abstractNumId);
        redirected = Some(abstractNumIdRedirect.put(abstractNumId));
      }
      redirected.unwrap_or(0)
    }
}
