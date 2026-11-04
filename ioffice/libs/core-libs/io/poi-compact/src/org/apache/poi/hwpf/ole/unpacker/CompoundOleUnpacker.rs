// Java: import cn.wps.base.log.IOLog
use util::base::log::IOLog::*;
// Java: import cn.wps.ole.IOLEDocument
use util_adapter::cn::wps::ole::IOLEDocument::*;
// Java: import cn.wps.ole.IOLEStorage
use util_adapter::cn::wps::ole::IOLEStorage::*;
// Java: import cn.wps.ole.OLEDocumentFactory
use util_adapter::cn::wps::ole::OLEDocumentFactory::*;
// Java: import java.io.IOException
use stubs::java::io::IOException::*;
// Java: import org.apache.poi.hpsf.ClassID
use crate::org::apache::poi::hpsf::ClassID::*;
// Java: import org.apache.poi.hwpf.util.OleParseInterruptException
use crate::org::apache::poi::hwpf::util::OleParseInterruptException::*;
// Java: import org.apache.poi.hwpf.util.StorageUtil
use crate::org::apache::poi::hwpf::util::StorageUtil::*;
// Java: import org.apache.poi.poifs.filesystem.DirectoryNode
use crate::org::apache::poi::poifs::filesystem::DirectoryNode::*;
// Java: import org.apache.poi.poifs.filesystem.Entry
use crate::org::apache::poi::poifs::filesystem::Entry::*;
// Java: import org.apache.poi.hwpf.ole.unpacker.BaseOleUnpacker
use crate::org::apache::poi::hwpf::ole::unpacker::BaseOleUnpacker::*;

// java public class CompoundOleUnpacker extends BaseOleUnpacker
#[derive(Default)]
pub struct CompoundOleUnpacker {
    // java BaseOleUnpacker super_
    pub(crate) super_: BaseOleUnpacker,
}

// java public class CompoundOleUnpacker extends BaseOleUnpacker
impl CompoundOleUnpacker {
    // java public  CompoundOleUnpacker(Entry baseEntry)
    pub fn CompoundOleUnpacker(baseEntry: &dyn Entry) -> Self {
      // java super(baseEntry);
      Self { super_: BaseOleUnpacker::new(&baseEntry) }
    }

    // java public boolean parseFile(String desPath)
    pub fn parseFile(&self, desPath: &str) -> bool {
      // java if (this.mBaseEntry != null && this.mBaseEntry instanceof DirectoryNode)
      if self.mBaseEntry.is_some() && self.mBaseEntry.downcast_ref::<DirectoryNode>().is_some() {
        // java IOLEDocument document = null;
        let mut document: Box<dyn IOLEDocument> = Box::new(()) as Box<dyn IOLEDocument>;
        // java IOLEStorage storage = null;
        let mut storage: Box<dyn IOLEStorage> = Box::new(()) as Box<dyn IOLEStorage>;
            // java document = OLEDocumentFactory.create(desPath, 2);
            document = OLEDocumentFactory::create__string_int(&desPath, 2);
            // java storage = document.getRootStorage();
            storage = document.getRootStorage();
            // java ClassID classId = ((DirectoryNode)this.mBaseEntry).getStorageClsid();
            let mut classId: ClassID = (self.mBaseEntry).downcast_ref::<DirectoryNode>().unwrap().getStorageClsid();
            // java storage.setStorageClsid(classId.getBytesLittleEndian());
            storage.setStorageClsid(classId.getBytesLittleEndian());
            // java StorageUtil.writeStorage(storage, this.mBaseEntry);
            StorageUtil::writeStorage(&mut storage, &self.mBaseEntry);
            // java boolean var5 = true;
            let mut var5: bool = true;
            // java return var5;
            return var5;
            // java IOException var10
            todo!();
              // java IOLog.e("CompoundOleUnpacker", "parseFile entry failed");
              IOLog::e(&"CompoundOleUnpacker", &"parseFile entry failed");
            // java OleParseInterruptException var11
            todo!();
              // java IOLog.e("CompoundOleUnpacker", "parseFile entry failed, OleParseInterruptException");
              IOLog::e(&"CompoundOleUnpacker", &"parseFile entry failed, OleParseInterruptException");
              // java StorageUtil.closeStorage(storage);
              StorageUtil::closeStorage(Some(&mut storage));
              // java StorageUtil.closeDocument(document);
              StorageUtil::closeDocument(Some(&mut document));
        // java return false;
        return false;
      }
      // java else
      else {
        // java IOLog.d("CompoundOleUnpacker", "baseEntry is not DirectoryNode");
        IOLog::d(&"CompoundOleUnpacker", &"baseEntry is not DirectoryNode");
        // java return false;
        return false;
      }
    }

    // java public Entry parseEntry()
    pub fn parseEntry(&self) -> Box<dyn Entry> {
      // java return this.mBaseEntry;
      return self.mBaseEntry;
    }

}

impl std::ops::Deref for CompoundOleUnpacker {
    type Target = BaseOleUnpacker;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for CompoundOleUnpacker {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}
