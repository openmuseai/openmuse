// Java: import cn.wps.base.assertion.Assert
use util::cn::wps::core::runtime::Assert;
// Java: import cn.wps.base.log.IOLog
use util::cn::wps::base::log::IOLog;
// Java: import cn.wps.io.file.EncryptedType
use io_libfileformat::cn::wps::io::file::EncryptedType;
// Java: import cn.wps.io.file.Tools
use kfs_kfs_app::cn::wps::moffice::kfs::tools::Tools;
// Java: import cn.wps.moffice.kfs.File
use kfs_kfs_app::cn::wps::moffice::kfs::mfs::File;
// Java: import cn.wps.moffice.kfs.RandomAccessFile
use kfs_kfs_app::cn::wps::moffice::kfs::RandomAccessFile;
// Java: import cn.wps.moffice.util.KSLog
use util::cn::wps::moffice::util::KSLog;
// Java: import org.apache.poi.hwpf.model.FileInformationBlock
use io_poi_compact::org::apache::poi::hwpf::model::FileInformationBlock;
// Java: import org.apache.poi.poifs.filesystem.DirectoryNode
use io_poi_compact::org::apache::poi::poifs::filesystem::DirectoryNode;
// Java: import org.apache.poi.poifs.filesystem.DocumentInputStream
use io_poi_compact::org::apache::poi::poifs::filesystem::DocumentInputStream;
// Java: import org.apache.poi.poifs.filesystem.POIFSFileSystem
use io_poi_compact::org::apache::poi::poifs::filesystem::POIFSFileSystem;

// Java field: String TAG
pub(crate) const TAG: &str = "";
// Java field: String STREAM_ENTRY_0TABLE
static STREAM_ENTRY_0TABLE: std::sync::OnceLock<std::sync::Mutex<String>> = std::sync::OnceLock::new();
// Java field: String STREAM_ENTRY_1TABLE
static STREAM_ENTRY_1TABLE: std::sync::OnceLock<std::sync::Mutex<String>> = std::sync::OnceLock::new();
// Java field: String OLE_DIRECTORY_ENTRY_ELENAME_WORDDOCUMENT
pub(crate) const OLE_DIRECTORY_ENTRY_ELENAME_WORDDOCUMENT: &str = "";

// Java: public class DocChecker extends OleFileChecker
pub struct DocChecker {
    // Java field: FileInformationBlock _fileInformationBlock
    _fileInformationBlock: Option<FileInformationBlock>,
    // Java field: Boolean _isDoc
    pub(crate) _isDoc: Arc<Mutex<Boolean>>,
    // Java field: EncryptedType _encryptedType
    pub(crate) _encryptedType: Arc<Mutex<EncryptedType>>,
}

// Java: public class DocChecker extends OleFileChecker
impl DocChecker {
    // Java: DocChecker(File file, RandomAccessFile raf, POIFSFileSystem poifs)
    pub fn DocChecker(&self, File file, RandomAccessFile raf, POIFSFileSystem poifs) {
      // Java: super(file, raf, poifs);
      // TODO_RUST_CODE
    }

    // Java: boolean isDoc()
    pub fn isDoc(&self) -> bool {
      // Java: if (null != this._isDoc)
        // TODO_RUST_CODE
      {
         // Java: return this._isDoc;
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: KSLog.d("IOPerformance", "DocChecker.isDoc() begin");
         // TODO_RUST_CODE
         // Java: if (!this.isOle())
            // TODO_RUST_CODE
         {
            // Java: return this._isDoc = false;
            // TODO_RUST_CODE
         }
         // Java: else
            // TODO_RUST_CODE
         {
            // Java: if (null == this._Poifs)
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
                  // Java: this._Poifs = Tools.createPOIFSFileSystem(this._file);
                  // TODO_RUST_CODE
               }
            }
            // Java: if (null == this._Poifs)
                // TODO_RUST_CODE
            {
               // Java: return this._isDoc = false;
               // TODO_RUST_CODE
            }
            // Java: else
                // TODO_RUST_CODE
            {
               // Java: DirectoryNode directory = this._Poifs.getRoot();
               // TODO_RUST_CODE
               // Java: if (directory == null)
                    // TODO_RUST_CODE
               {
                  // Java: return this._isDoc = false;
                  // TODO_RUST_CODE
               }
               // Java: else
                    // TODO_RUST_CODE
               {
                  // Java: byte[] fibBase = createFibBaseStream("WordDocument", directory);
                  // TODO_RUST_CODE
                  // Java: if (null == fibBase)
                        // TODO_RUST_CODE
                  {
                     // Java: return this._isDoc = false;
                     // TODO_RUST_CODE
                  }
                  // Java: else
                        // TODO_RUST_CODE
                  {
                     // Java: this._fileInformationBlock = new FileInformationBlock(fibBase);
                     // TODO_RUST_CODE
                     // Java: if (null != this._fileInformationBlock && this._fileInformationBlock.getNFib() > 104)
                            // TODO_RUST_CODE
                     {
                        // Java: KSLog.d("IOPerformance", "DocChecker.isDoc() end");
                        // TODO_RUST_CODE
                        // Java: return this._isDoc = true;
                        // TODO_RUST_CODE
                     }
                     // Java: else
                            // TODO_RUST_CODE
                     {
                        // Java: return this._isDoc = false;
                        // TODO_RUST_CODE
                     }
                  }
               }
            }
         }
      }
    }

    // Java: EncryptedType isEncrypted()
    pub fn isEncrypted(&self) -> EncryptedType {
      // Java: if (null != this._encryptedType)
        // TODO_RUST_CODE
      {
         // Java: return this._encryptedType;
         // TODO_RUST_CODE
      }
      // Java: else if ((null == this._isDoc || this._isDoc) && this.isDoc())
        // TODO_RUST_CODE
      {
         // Java: if (null != this._fileInformationBlock && this._fileInformationBlock.isFEncrypted())
            // TODO_RUST_CODE
         {
            // Java: if ((!this._fileInformationBlock.isFWhichTblStm() || null != this._Poifs.getRoot().getEntry2(STREAM_ENTRY_1TABLE)) && (this._fileInformationBlock.isFWhichTblStm() || null != this._Poifs.getRoot().getEntry2(STREAM_ENTRY_0TABLE)))
                // TODO_RUST_CODE
            {
               // Java: this._encryptedType = EncryptedType.Doc;
               // TODO_RUST_CODE
            }
            // Java: else
                // TODO_RUST_CODE
            {
               // Java: this._encryptedType = EncryptedType.None;
               // TODO_RUST_CODE
            }
         }
         // Java: else
            // TODO_RUST_CODE
         {
            // Java: this._encryptedType = EncryptedType.None;
            // TODO_RUST_CODE
         }
         // Java: return this._encryptedType;
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: return null;
         // TODO_RUST_CODE
      }
    }

    // Java: boolean isWriteReservation()
    pub fn isWriteReservation(&self) -> bool {
      // Java: if ((null == this._isDoc || this._isDoc) && this.isDoc())
        // TODO_RUST_CODE
      {
         // Java: return null != this._fileInformationBlock ? this._fileInformationBlock.isFWriteReservation() : false;
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: return false;
         // TODO_RUST_CODE
      }
    }

    // Java: static byte[] createFibBaseStream(String name, DirectoryNode directory)
    pub fn createFibBaseStream(&self, String name, DirectoryNode directory) -> byte[] {
      // Java: Assert.assertNotNull("name should not be null!", name);
      // TODO_RUST_CODE
      // Java: Assert.assertNotNull("directory should not be null!", directory);
      // TODO_RUST_CODE
      // Java: DocumentInputStream is = null;
      // TODO_RUST_CODE
      // Java: try
        // TODO_RUST_CODE
      {
         // Java: is = directory.createDocumentInputStream(name);
         // TODO_RUST_CODE
      }
      // Java: catch (Throwable e)
        // TODO_RUST_CODE
      {
         // Java: IOLog.e(TAG, "Throwable", e);
         // TODO_RUST_CODE
      }
      // Java: if (null == is)
        // TODO_RUST_CODE
      {
         // Java: return null;
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: try
            // TODO_RUST_CODE
         {
            // Java: byte[] fibbase = new byte[32];
            // TODO_RUST_CODE
            // Java: is.read(fibbase);
            // TODO_RUST_CODE
            // Java: byte[] var4 = fibbase;
            // TODO_RUST_CODE
            // Java: return var4;
            // TODO_RUST_CODE
         }
         // Java: catch (Throwable e)
            // TODO_RUST_CODE
         {
            // Java: IOLog.e(TAG, "Throwable", e);
            // TODO_RUST_CODE
         }
         // Java: finally
            // TODO_RUST_CODE
         {
            // Java: is.close();
            // TODO_RUST_CODE
         }
         // Java: return null;
         // TODO_RUST_CODE
      }
    }

}
