// Java: import cn.wps.moffice.writer.io.reader.xml07.Xml07Context
use crate::cn::wps::moffice::writer::io::reader::xml07::Xml07Context::*;
// Java: import cn.wps.moffice.writer.io.reader.xml07.pi.IProcInfo
use crate::cn::wps::moffice::writer::io::reader::xml07::pi::IProcInfo::*;
// Java: import cn.wps.moffice.writer.io.reader.xml07.co.ICoreObject
use crate::cn::wps::moffice::writer::io::reader::xml07::co::ICoreObject::*;

// java public class CoreObject implements ICoreObject
#[derive(Default)]
pub struct CoreObject {
    // java private int mCnt = 0;
    mCnt: i32,
    // java protected String mOwnerPart;
    pub(crate) mOwnerPart: Option<String>,
    // java private ICoreObject mNext;
    mNext: Option<Box<dyn ICoreObject>>,
}

// java public interface ICoreObject
impl ICoreObject for CoreObject {
    // java public boolean isReady()
    fn isReady(&self) -> bool {
      // java return 0 == this.mCnt;
      return 0 == self.mCnt;
    }

    // java public void start()
    fn start(&mut self) {
      // java ++this.mCnt;
      { let _tmp = self.mCnt; self.mCnt += 1; _tmp };
    }

    // java public void end()
    fn end(&mut self) {
      // java --this.mCnt;
      { let _tmp = self.mCnt; self.mCnt -= 1; _tmp };
    }

    // java public ICoreObject getNext()
    fn getNext(&self) -> Box<dyn ICoreObject> {
      // java return this.mNext;
      return self.mNext;
    }

    // java public void setNext(ICoreObject co)
    fn setNext(&mut self, co: &dyn ICoreObject) {
      // java this.mNext = co;
      self.mNext = co;
    }

    // java public int getType()
    fn getType(&self) -> i32 {
      // java return 0;
      return 0;
    }

    // java public void setOwnerPart(String owerPart)
    fn setOwnerPart(&mut self, owerPart: &str) {
      // java this.mOwnerPart = owerPart;
      self.mOwnerPart = owerPart;
    }

    // java public String getOwnerPart()
    fn getOwnerPart(&self) -> String {
      // java return this.mOwnerPart;
      return self.mOwnerPart;
    }

    // java public void process(Xml07Context ctxt, IProcInfo info)
    fn process(&mut self, ctxt: &Xml07Context, info: &dyn IProcInfo) {
    }

}

// java public class CoreObject implements ICoreObject
impl CoreObject {
}
