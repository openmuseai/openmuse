// Java: import org.apache.poi.poifs.filesystem.Entry
use crate::org::apache::poi::poifs::filesystem::Entry::*;
// Java: import org.apache.poi.hwpf.ole.unpacker.BaseOleUnpacker
use crate::org::apache::poi::hwpf::ole::unpacker::BaseOleUnpacker::*;

// java public class CommonOleUnpacker extends BaseOleUnpacker
#[derive(Default)]
pub struct CommonOleUnpacker {
    // java BaseOleUnpacker super_
    pub(crate) super_: BaseOleUnpacker,
}

// java public class CommonOleUnpacker extends BaseOleUnpacker
impl CommonOleUnpacker {
    // java public  CommonOleUnpacker(Entry baseEntry)
    pub fn CommonOleUnpacker(baseEntry: &dyn Entry) -> Self {
      // java super(baseEntry);
      Self { super_: BaseOleUnpacker::new(&baseEntry) }
    }

    // java public Entry parseEntry()
    pub fn parseEntry(&self) -> Option<Box<dyn Entry>> {
      // java Entry packageEntry = this.getEntry(this.mBaseEntry, "package");
      let mut packageEntry: Box<dyn Entry> = self.super_.getEntry(&self.mBaseEntry, Some(&"package"));
      // java return packageEntry != null ? packageEntry : this.mBaseEntry;
      return (if packageEntry.is_some() { Some(packageEntry) } else { Some(self.mBaseEntry) });
    }

}

impl std::ops::Deref for CommonOleUnpacker {
    type Target = BaseOleUnpacker;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for CommonOleUnpacker {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}
