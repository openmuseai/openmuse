// Java: import org.apache.poi.poifs.filesystem.Entry
use crate::org::apache::poi::poifs::filesystem::Entry::*;
// Java: import org.apache.poi.hwpf.ole.unpacker.BaseOleUnpacker
use crate::org::apache::poi::hwpf::ole::unpacker::BaseOleUnpacker::*;

// java public class ContentsOleUnpacker extends BaseOleUnpacker
#[derive(Default)]
pub struct ContentsOleUnpacker {
    // java BaseOleUnpacker super_
    pub(crate) super_: BaseOleUnpacker,
}

// java public class ContentsOleUnpacker extends BaseOleUnpacker
impl ContentsOleUnpacker {
    // java public  ContentsOleUnpacker(Entry baseEntry)
    pub fn ContentsOleUnpacker(baseEntry: &dyn Entry) -> Self {
      // java super(baseEntry);
      Self { super_: BaseOleUnpacker::new(&baseEntry) }
    }

    // java public Entry parseEntry()
    pub fn parseEntry(&self) -> Option<Box<dyn Entry>> {
      // java Entry packageEntry = this.getEntry(this.mBaseEntry, "contents");
      let mut packageEntry: Box<dyn Entry> = self.super_.getEntry(&self.mBaseEntry, Some(&"contents"));
      // java return packageEntry != null ? packageEntry : this.mBaseEntry;
      return (if packageEntry.is_some() { Some(packageEntry) } else { Some(self.mBaseEntry) });
    }

}

impl std::ops::Deref for ContentsOleUnpacker {
    type Target = BaseOleUnpacker;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for ContentsOleUnpacker {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}
