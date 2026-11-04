// Java: import cn.wps.moffice.writer.io.reader.ImporterBase
use crate::cn::wps::moffice::writer::io::reader::ImporterBase::*;
// Java: import cn.wps.moffice.writer.io.reader.xml03.Xml03Context
use crate::cn::wps::moffice::writer::io::reader::xml03::Xml03Context::*;
// Java: import cn.wps.moffice.writer.io.reader.xml07.Xml07Context
use crate::cn::wps::moffice::writer::io::reader::xml07::Xml07Context::*;
use super::IOLvl::IOLvl;

// java final class IOLvlOverride extends ImporterBase
pub struct IOLvlOverride {
    pub(crate) super_: ImporterBase,
    // java Integer mIlvl = null;
    pub(crate) mIlvl: Option<i32>,
    // java Integer mStartOverride = null;
    pub(crate) mStartOverride: Option<i32>,
    // java IOLvl mIOLvl = null;
    pub(crate) mIOLvl: Option<IOLvl>,
}

// java final class IOLvlOverride extends ImporterBase
impl IOLvlOverride {
    // java IOLvlOverride(int mode, Xml07Context xml07Context, Xml03Context xml03Context)
    pub fn IOLvlOverride(mode: i32, xml07Context: Option<Xml07Context>, xml03Context: Option<Xml03Context>) -> Self {
      // java super(mode, xml07Context, xml03Context);
      Self {
        super_: ImporterBase::ImporterBase(mode, xml07Context, xml03Context),
        mIlvl: None,
        mStartOverride: None,
        mIOLvl: None,
      }
    }

    // java IOLvl getIOLvl()
    pub fn getIOLvl(&mut self) -> &mut IOLvl {
      // java if (null == this.mIOLvl)
      if self.mIOLvl.is_none() {
        // java this.mIOLvl = new IOLvl(this.mode(), this.xml07Context(), this.xml03Context());
        let mode = self.super_.mode();
        let xml07 = self.super_.xml07Context().cloned();
        let xml03 = self.super_.xml03Context().cloned();
        self.mIOLvl = Some(IOLvl::IOLvl(mode, xml07, xml03));
      }
      // java return this.mIOLvl;
      self.mIOLvl.as_mut().expect("mIOLvl")
    }

    // java void dispose()
    pub fn dispose(&mut self) {
      // java this.mIlvl = null;
      self.mIlvl = None;
      // java this.mStartOverride = null;
      self.mStartOverride = None;
      // java if (null != this.mIOLvl)
      if let Some(lvl) = self.mIOLvl.as_mut() {
        // java this.mIOLvl.dispose();
        lvl.dispose();
        // java this.mIOLvl = null;
        self.mIOLvl = None;
      }
    }
}
