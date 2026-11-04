// Java: import cn.wps.base.log.IOLog
use util::base::log::IOLog::*;
// Java: import cn.wps.moffice.util.FileUtil
use util::moffice::util::FileUtil::*;
// Java: import java.io.FileNotFoundException
use stubs::java::io::FileNotFoundException::*;
// Java: import java.io.IOException
use stubs::java::io::IOException::*;
// Java: import java.util.Iterator
// Java: import org.apache.poi.poifs.filesystem.DirectoryEntry
use crate::org::apache::poi::poifs::filesystem::DirectoryEntry::*;
// Java: import org.apache.poi.poifs.filesystem.DocumentEntry
use crate::org::apache::poi::poifs::filesystem::DocumentEntry::*;
// Java: import org.apache.poi.poifs.filesystem.DocumentInputStream
use crate::org::apache::poi::poifs::filesystem::DocumentInputStream::*;
// Java: import org.apache.poi.poifs.filesystem.Entry
use crate::org::apache::poi::poifs::filesystem::Entry::*;
// Java: import org.apache.poi.hwpf.ole.unpacker.IOleUnpacker
use crate::org::apache::poi::hwpf::ole::unpacker::IOleUnpacker::*;

// java public abstract class BaseOleUnpacker implements IOleUnpacker
#[derive(Default)]
pub struct BaseOleUnpacker {
    // java protected Entry mBaseEntry;
    pub(crate) mBaseEntry: Option<Box<dyn Entry>>,
}

// java public interface IOleUnpacker
impl IOleUnpacker for BaseOleUnpacker {
    // java public boolean parseFile(String desPath)
    fn parseFile(&mut self, desPath: &str) -> Option<bool> {
      // java Entry packageEntry = this.parseEntry();
      let mut packageEntry: Option<&(dyn Entry)> = self.parseEntry();
      // java return null != packageEntry ? this.writeFile(packageEntry, desPath) : false;
      return (if packageEntry.is_some() { Some(self.writeFile(&packageEntry, &desPath)) } else { Some(false) });
    }

    // java  Entry parseEntry()
    fn parseEntry(&mut self) -> Option<&(dyn Entry)> {
    }

}

// java public abstract class BaseOleUnpacker implements IOleUnpacker
impl BaseOleUnpacker {
    // java public  BaseOleUnpacker(Entry baseEntry)
    pub fn BaseOleUnpacker(baseEntry: &dyn Entry) -> Self {
      // java super();
      // java this.mBaseEntry = baseEntry;
      Self { mBaseEntry: baseEntry }
    }

    // java protected Entry getEntry(Entry baseEntry, String targetEntryName)
    pub fn getEntry(&self, baseEntry: &dyn Entry, targetEntryName: Option<&str>) -> Option<Box<dyn Entry>> {
          // java if (targetEntryName != null && baseEntry instanceof DirectoryEntry)
          if let Some(targetEntryName) = targetEntryName {
            if baseEntry.downcast_ref::<Box<dyn DirectoryEntry>>().is_some() {
              // java DirectoryEntry directoryEntry = (DirectoryEntry)baseEntry;
              let mut directoryEntry: Box<dyn DirectoryEntry> = todo!("cast → Box<dyn DirectoryEntry>");
              // java Iterator<Entry> iterator = directoryEntry.getEntries();
              let mut iterator: Vec<Entry> = directoryEntry.getEntries();
              // java while (iterator.hasNext())
              while iterator.hasNext() {
                // java Entry entry = (Entry)iterator.next();
                let mut entry: Box<dyn Entry> = todo!("cast → Box<dyn Entry>");
                // java String entryName = entry != null ? entry.getName() : null;
                let mut entryName: String = (if entry.is_some() { entry.getName() } else { String::new() });
                // java if (entryName != null && (targetEntryName.equals(entryName.toLowerCase()) || targetEntryName.equals(entryName)))
                if entryName.is_some() && targetEntryName.equals(entryName.toLowerCase()) || targetEntryName.equals(entryName) {
                  // java return directoryEntry.getEntry(entryName);
                  return directoryEntry.getEntry(entryName);
                }
              }
            }
          }
          // java FileNotFoundException e
          todo!();
            // java IOLog.e("BaseOleUnpacker", "getEntry failed", e);
            IOLog::e__string_string_throwable(&"BaseOleUnpacker", &"getEntry failed", &e);
      // java return baseEntry;
      return baseEntry;
    }

    // java protected boolean writeFile(Entry entry, String desPath)
    pub fn writeFile(&mut self, entry: &dyn Entry, desPath: &str) -> bool {
      // java if (entry instanceof DocumentEntry)
      if entry.downcast_ref::<Box<dyn DocumentEntry>>().is_some() {
            // java return FileUtil.writeToFile(desPath, new DocumentInputStream((DocumentEntry)entry));
            return FileUtil::writeToFile__string_inputstream(&desPath, &DocumentInputStream::new(todo!("cast → Box<dyn DocumentEntry>")));
            // java IOException var4
            todo!();
              // java IOLog.e("BaseOleUnpacker", "write file failed");
              IOLog::e__string_string(&"BaseOleUnpacker", &"write file failed");
      }
      // java return false;
      return false;
    }

}
