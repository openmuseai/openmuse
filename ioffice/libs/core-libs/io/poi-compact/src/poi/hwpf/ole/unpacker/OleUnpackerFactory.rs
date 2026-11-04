// Java: import cn.wps.base.log.IOLog
use util::cn::wps::base::log::IOLog;
// Java: import cn.wps.moffice.kfs.RandomAccessFile
use kfs_kfs_app::cn::wps::moffice::kfs::RandomAccessFile;
// Java: import java.io.IOException
// TODO_RUST_CODE use ???; // java.io.IOException
// Java: import java.util.Iterator
// TODO_RUST_CODE use ???; // java.util.Iterator
// Java: import org.apache.poi.hwpf.HWPFDocumentCore
use io_poi_compact::org::apache::poi::hwpf::HWPFDocumentCore;
// Java: import org.apache.poi.poifs.filesystem.DirectoryEntry
use io_poi_compact::org::apache::poi::poifs::filesystem::DirectoryEntry;
// Java: import org.apache.poi.poifs.filesystem.DirectoryNode
use io_poi_compact::org::apache::poi::poifs::filesystem::DirectoryNode;
// Java: import org.apache.poi.poifs.filesystem.Entry
use io_poi_compact::org::apache::poi::poifs::filesystem::Entry;
// Java: import org.apache.poi.poifs.filesystem.POIFSFileSystem
use io_poi_compact::org::apache::poi::poifs::filesystem::POIFSFileSystem;

// Java: public class OleUnpackerFactory
pub struct OleUnpackerFactory {
}

// Java: public class OleUnpackerFactory
impl OleUnpackerFactory {
    // Java: OleUnpackerFactory()
    pub fn OleUnpackerFactory(&self) {
      // Java: super();
      // TODO_RUST_CODE
    }

    // Java: static IOleUnpacker getParser(Entry baseEntry)
    pub fn getParser(&self, Entry baseEntry) -> IOleUnpacker {
      // Java: if (null == baseEntry)
        // TODO_RUST_CODE
      {
         // Java: return null;
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: if (baseEntry instanceof DirectoryEntry)
            // TODO_RUST_CODE
         {
            // Java: DirectoryEntry directoryEntry = (DirectoryEntry)baseEntry;
            // TODO_RUST_CODE
            // Java: Iterator<Entry> iterator = directoryEntry.getEntries();
            // TODO_RUST_CODE
            // Java: if (iterator != null)
                // TODO_RUST_CODE
            {
               // Java: while(iterator.hasNext())
                    // TODO_RUST_CODE
               {
                  // Java: Entry entry = (Entry)iterator.next();
                  // TODO_RUST_CODE
                  // Java: String entryName = entry != null ? entry.getName() : null;
                  // TODO_RUST_CODE
                  // Java: if (entryName != null && "package".equalsIgnoreCase(entryName))
                        // TODO_RUST_CODE
                  {
                     // Java: return new CommonOleUnpacker(baseEntry);
                     // TODO_RUST_CODE
                  }
                  // Java: if (entryName != null && "contents".equalsIgnoreCase(entryName))
                        // TODO_RUST_CODE
                  {
                     // Java: return new ContentsOleUnpacker(baseEntry);
                     // TODO_RUST_CODE
                  }
                  // Java: if (entryName != null && "\u0001Ole10Native".equalsIgnoreCase(entryName))
                        // TODO_RUST_CODE
                  {
                     // Java: return new Native10OleUnpacker(baseEntry);
                     // TODO_RUST_CODE
                  }
               }
            }
         }
         // Java: return new CompoundOleUnpacker(baseEntry);
         // TODO_RUST_CODE
      }
    }

    // Java: static IOleUnpacker getParser(String binPath)
    pub fn getParser(&self, String binPath) -> IOleUnpacker {
      // Java: POIFSFileSystem poifsFileSystem = null;
      // TODO_RUST_CODE
      // Java: RandomAccessFile randomAccessFile = null;
      // TODO_RUST_CODE
      // Java: try
        // TODO_RUST_CODE
      {
         // Java: randomAccessFile = new RandomAccessFile(binPath, "r");
         // TODO_RUST_CODE
         // Java: poifsFileSystem = HWPFDocumentCore.verifyAndBuildPOIFS(randomAccessFile);
         // TODO_RUST_CODE
      }
      // Java: catch (Exception var6)
        // TODO_RUST_CODE
      {
         // Java: IOLog.e("OleEntryParser", "getOleEntryByFile failed");
         // TODO_RUST_CODE
         // Java: if (null != poifsFileSystem)
            // TODO_RUST_CODE
         {
            // Java: poifsFileSystem.dispose();
            // TODO_RUST_CODE
         }
         // Java: if (null != randomAccessFile)
            // TODO_RUST_CODE
         {
            // Java: try
                // TODO_RUST_CODE
            {
               // Java: randomAccessFile.close();
               // TODO_RUST_CODE
            }
            // Java: catch (IOException var5)
                // TODO_RUST_CODE
            {
               // Java: IOLog.e("OleEntryParser", "close file failed");
               // TODO_RUST_CODE
            }
         }
         // Java: return null;
         // TODO_RUST_CODE
      }
      // Java: DirectoryNode node = poifsFileSystem.getRoot();
      // TODO_RUST_CODE
      // Java: return null == node ? null : getParser((Entry)node);
      // TODO_RUST_CODE
    }

}
