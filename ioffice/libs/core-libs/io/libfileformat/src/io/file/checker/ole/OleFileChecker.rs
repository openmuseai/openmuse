// Java: import cn.wps.base.log.Log
use kfs_kfs_app::cn::wps::moffice::kfs::log::Log;
// Java: import cn.wps.io.file.Tools
use kfs_kfs_app::cn::wps::moffice::kfs::tools::Tools;
// Java: import cn.wps.moffice.kfs.File
use kfs_kfs_app::cn::wps::moffice::kfs::mfs::File;
// Java: import cn.wps.moffice.kfs.RandomAccessFile
use kfs_kfs_app::cn::wps::moffice::kfs::RandomAccessFile;
// Java: import java.io.IOException
// TODO_RUST_CODE use ???; // java.io.IOException
// Java: import org.apache.poi.poifs.filesystem.DirectoryNode
use io_poi_compact::org::apache::poi::poifs::filesystem::DirectoryNode;
// Java: import org.apache.poi.poifs.filesystem.DocumentEntry
use io_poi_compact::org::apache::poi::poifs::filesystem::DocumentEntry;
// Java: import org.apache.poi.poifs.filesystem.POIFSFileSystem
use io_poi_compact::org::apache::poi::poifs::filesystem::POIFSFileSystem;

// Java field: String TAG
pub(crate) const TAG: &str = "";

// Java: public class OleFileChecker
pub struct OleFileChecker {
    // Java field: Boolean _isOle
    pub(crate) _isOle: Arc<Mutex<Boolean>>,
    // Java field: File _file
    pub(crate) _file: Arc<Mutex<File>>,
    // Java field: RandomAccessFile mRaf
    pub(crate) mRaf: Arc<Mutex<RandomAccessFile>>,
    // Java field: POIFSFileSystem _Poifs
    pub(crate) _Poifs: Arc<Mutex<POIFSFileSystem>>,
}

// Java: public class OleFileChecker
impl OleFileChecker {
    // Java: OleFileChecker(File file, RandomAccessFile raf, POIFSFileSystem poifs)
    pub fn OleFileChecker(&self, File file, RandomAccessFile raf, POIFSFileSystem poifs) {
      // Java: super();
      // TODO_RUST_CODE
      // Java: this._file = file;
      // TODO_RUST_CODE
      // Java: this.mRaf = raf;
      // TODO_RUST_CODE
      // Java: this._Poifs = poifs;
      // TODO_RUST_CODE
    }

    // Java: boolean isOle()
    pub fn isOle(&self) -> bool {
      // Java: RandomAccessFile file = null;
      // TODO_RUST_CODE
      // Java: try
        // TODO_RUST_CODE
      {
         // Java: file = null != this.mRaf ? this.mRaf : new RandomAccessFile(this._file, "r");
         // TODO_RUST_CODE
         // Java: this._isOle = POIFSFileSystem.hasPOIFSHeader(file);
         // TODO_RUST_CODE
      }
      // Java: catch (IOException e)
        // TODO_RUST_CODE
      {
         // Java: Log.e(TAG, "IOException", e);
         // TODO_RUST_CODE
      }
      // Java: if (null != file)
        // TODO_RUST_CODE
      {
         // Java: try
            // TODO_RUST_CODE
         {
            // Java: file.close();
            // TODO_RUST_CODE
         }
         // Java: catch (IOException e)
            // TODO_RUST_CODE
         {
            // Java: Log.e(TAG, "IOException", e);
            // TODO_RUST_CODE
         }
      }
      // Java: return this._isOle != null ? this._isOle : false;
      // TODO_RUST_CODE
    }

    // Java: boolean hasEntry(String nameEntry)
    pub fn hasEntry(&self, String nameEntry) -> bool {
      // Java: if (!this.isOle())
        // TODO_RUST_CODE
      {
         // Java: return false;
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: if (null == this._Poifs)
            // TODO_RUST_CODE
         {
            // Java: long length = 0L;
            // TODO_RUST_CODE
            // Java: try
                // TODO_RUST_CODE
            {
               // Java: length = null != this.mRaf ? this.mRaf.length() : this._file.length();
               // TODO_RUST_CODE
               // Java: if (length <= 1048576L)
                    // TODO_RUST_CODE
               {
                  // Java: if (null != this.mRaf)
                        // TODO_RUST_CODE
                  {
                     // Java: this._Poifs = Tools.createPOIFSFileSystem(this.mRaf);
                     // TODO_RUST_CODE
                  }
                  // Java: else
                        // TODO_RUST_CODE
                  {
                     // Java: this._Poifs = Tools.createPOIFSFileSystem(Tools.createFileInputStream(this._file));
                     // TODO_RUST_CODE
                  }
               }
               // Java: else if (null != this.mRaf)
                    // TODO_RUST_CODE
               {
                  // Java: this._Poifs = Tools.createPOIFSFileSystem(this.mRaf);
                  // TODO_RUST_CODE
               }
               // Java: else
                    // TODO_RUST_CODE
               {
                  // Java: this._Poifs = Tools.createPOIFSFileSystem(this._file);
                  // TODO_RUST_CODE
               }
            }
            // Java: catch (IOException e)
                // TODO_RUST_CODE
            {
               // Java: Log.e(TAG, "IOException", e);
               // TODO_RUST_CODE
            }
         }
         // Java: if (null == this._Poifs)
            // TODO_RUST_CODE
         {
            // Java: return false;
            // TODO_RUST_CODE
         }
         // Java: else
            // TODO_RUST_CODE
         {
            // Java: DirectoryNode directory = this._Poifs.getRoot();
            // TODO_RUST_CODE
            // Java: if (null == directory)
                // TODO_RUST_CODE
            {
               // Java: return false;
               // TODO_RUST_CODE
            }
            // Java: else
                // TODO_RUST_CODE
            {
               // Java: DocumentEntry documentProps = Tools.createDocumentEntry(nameEntry, directory);
               // TODO_RUST_CODE
               // Java: return null != documentProps;
               // TODO_RUST_CODE
            }
         }
      }
    }

    // Java: POIFSFileSystem getPOIFSFileSystem()
    pub fn getPOIFSFileSystem(&self) -> POIFSFileSystem {
      // Java: return this._Poifs;
      // TODO_RUST_CODE
    }

}
