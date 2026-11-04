// Java: import cn.wps.moffice.writer.io.reader.xml03.Xml03Context
use crate::cn::wps::moffice::writer::io::reader::xml03::Xml03Context::*;
// Java: import cn.wps.moffice.writer.io.reader.xml07.Xml07Context
use crate::cn::wps::moffice::writer::io::reader::xml07::Xml07Context::*;

// java public abstract class ImporterBase
#[derive(Default, Clone)]
pub struct ImporterBase {
    // java private int mMode;
    mMode: i32,
    // java private Xml07Context mXml07Cxt;
    mXml07Cxt: Option<Xml07Context>,
    // java private Xml03Context mXml03Cxt;
    mXml03Cxt: Option<Xml03Context>,
}

// java public abstract class ImporterBase
impl ImporterBase {
    // java protected  ImporterBase(int mode, Xml07Context xml07Context, Xml03Context xml03Context)
    pub fn ImporterBase(
        mode: i32,
        xml07Context: Option<Xml07Context>,
        xml03Context: Option<Xml03Context>,
    ) -> Self {
      // java this.mMode = mode;
      let mut this = Self {
          mMode: mode,
          mXml07Cxt: None,
          mXml03Cxt: None,
      };
      // java this.mXml07Cxt = xml07Context;
      this.mXml07Cxt = xml07Context;
      // java this.mXml03Cxt = xml03Context;
      this.mXml03Cxt = xml03Context;
      this
    }

    // java public int mode()
    pub fn mode(&self) -> i32 {
      // java return this.mMode;
      return self.mMode;
    }

    // java public Xml07Context xml07Context()
    pub fn xml07Context(&self) -> Option<&Xml07Context> {
      // java return this.mXml07Cxt;
      return self.mXml07Cxt.as_ref();
    }

    // Java xml07Context() returns the live object, which callers then mutate.
    pub fn xml07Context_mut(&mut self) -> Option<&mut Xml07Context> {
      self.mXml07Cxt.as_mut()
    }

    // java public Xml03Context xml03Context()
    pub fn xml03Context(&self) -> Option<&Xml03Context> {
      // java return this.mXml03Cxt;
      return self.mXml03Cxt.as_ref();
    }

    // java protected boolean isXml03()
    pub fn isXml03(&self) -> bool {
      // java return 3 == this.mMode && null != this.mXml03Cxt;
      return 3 == self.mMode && self.mXml03Cxt.is_some();
    }

    // java protected boolean isXml07()
    pub fn isXml07(&self) -> bool {
      // java return 1 == this.mMode && null != this.mXml07Cxt;
      return 1 == self.mMode && self.mXml07Cxt.is_some();
    }

    // java protected boolean isDocx()
    pub fn isDocx(&self) -> bool {
      // java return 0 == this.mMode;
      return 0 == self.mMode;
    }

    // java protected boolean isGVml()
    pub fn isGVml(&self) -> bool {
      // java return 2 == this.mMode;
      return 2 == self.mMode;
    }

}
