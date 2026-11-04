// Java: import org.apache.poi.hwpf.ole.stream.NativeStreamParser
use crate::org::apache::poi::hwpf::ole::stream::NativeStreamParser::*;
// Java: import org.apache.poi.poifs.filesystem.Entry
use crate::org::apache::poi::poifs::filesystem::Entry::*;
// Java: import org.apache.poi.hwpf.ole.unpacker.CompoundOleUnpacker
use crate::org::apache::poi::hwpf::ole::unpacker::CompoundOleUnpacker::*;

// java public class Native10OleUnpacker extends CompoundOleUnpacker
#[derive(Default)]
pub struct Native10OleUnpacker {
    // java CompoundOleUnpacker super_
    pub(crate) super_: CompoundOleUnpacker,
}

// java public class Native10OleUnpacker extends CompoundOleUnpacker
impl Native10OleUnpacker {
    // java public  Native10OleUnpacker(Entry baseEntry)
    pub fn Native10OleUnpacker(baseEntry: &dyn Entry) -> Self {
      // java super(baseEntry);
      Self { super_: CompoundOleUnpacker::new(&baseEntry) }
    }

    // java public Entry parseEntry()
    pub fn parseEntry(&self) -> Option<Box<dyn Entry>> {
      // java Entry packageEntry = this.getEntry(this.mBaseEntry, "\u0001Ole10Native");
      let mut packageEntry: Box<dyn Entry> = self.super_.getEntry(&self.mBaseEntry, Some(&"\u{0001}Ole10Native"));
      // java return packageEntry != null ? packageEntry : this.mBaseEntry;
      return (if packageEntry.is_some() { Some(packageEntry) } else { Some(self.mBaseEntry) });
    }

    // java public boolean parseFile(String desPath)
    pub fn parseFile(&mut self, desPath: &str) -> bool {
      // java boolean result = super.parseFile(desPath);
      let mut result: bool = self.super_.parseFile(&desPath);
      // java if (result)
      if result {
        // java String name = NativeStreamParser.getFileName(desPath);
        let mut name: String = NativeStreamParser::getFileName(&desPath);
        // java if (null != name)
        if name.is_some() {
          // java name = name.toLowerCase();
          name = name.toLowerCase();
          // java if (name.endsWith("penkit") || name.endsWith("3dobj"))
          if name.endsWith("penkit") || name.endsWith("3dobj") {
            // java return true;
            return true;
          }
        }
      }
      // java Entry packageEntry = this.parseEntry();
      let mut packageEntry: Box<dyn Entry> = self.parseEntry();
      // java if (null != packageEntry)
      if packageEntry.is_some() {
        // java return this.writeFile(packageEntry, desPath);
        return self.super_.writeFile(&packageEntry, &desPath);
      }
      // java else
      else {
        // java return false;
        return false;
      }
    }

}

impl std::ops::Deref for Native10OleUnpacker {
    type Target = CompoundOleUnpacker;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for Native10OleUnpacker {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}
