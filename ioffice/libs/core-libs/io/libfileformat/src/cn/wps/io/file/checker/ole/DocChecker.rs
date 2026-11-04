// Java: import cn.wps.base.assertion.Assert
// use util::cn::wps::core::runtime::Assert;
// Java: import cn.wps.base.log.IOLog
// use util::cn::wps::base::log::IOLog;
// Java: import cn.wps.io.file.EncryptedType
// use io_libfileformat::cn::wps::io::file::EncryptedType;
// Java: import cn.wps.io.file.Tools
// use io_libfileformat::cn::wps::io::file::Tools;
// Java: import cn.wps.moffice.kfs.File
// use kfs_kfs_app::cn::wps::moffice::kfs::mfs::File;
// Java: import cn.wps.moffice.kfs.RandomAccessFile
// use kfs_kfs_app::cn::wps::moffice::kfs::RandomAccessFile;
// Java: import cn.wps.moffice.util.KSLog
// use util::cn::wps::moffice::util::KSLog;
// Java: import org.apache.poi.hwpf.model.FileInformationBlock
// use io_poi_compact::org::apache::poi::hwpf::model::FileInformationBlock;
// Java: import org.apache.poi.poifs.filesystem.DirectoryNode
// use io_poi_compact::org::apache::poi::poifs::filesystem::DirectoryNode;
// Java: import org.apache.poi.poifs.filesystem.DocumentInputStream
// use io_poi_compact::org::apache::poi::poifs::filesystem::DocumentInputStream;
// Java: import org.apache.poi.poifs.filesystem.POIFSFileSystem
// use io_poi_compact::org::apache::poi::poifs::filesystem::POIFSFileSystem;

// Java field: String TAG
pub(crate) const TAG: &str = "";
// Java field: String STREAM_ENTRY_0TABLE
static STREAM_ENTRY_0TABLE: std::sync::OnceLock<std::sync::Mutex<String>> = std::sync::OnceLock::new();
// Java field: String STREAM_ENTRY_1TABLE
static STREAM_ENTRY_1TABLE: std::sync::OnceLock<std::sync::Mutex<String>> = std::sync::OnceLock::new();
// Java field: String OLE_DIRECTORY_ENTRY_ELENAME_WORDDOCUMENT
pub(crate) const OLE_DIRECTORY_ENTRY_ELENAME_WORDDOCUMENT: &str = "";

pub struct OleFileChecker;

// Java: public class DocChecker extends OleFileChecker
pub struct DocChecker {
    pub(crate) base: OleFileChecker,
    // Java field: FileInformationBlock _fileInformationBlock
    _fileInformationBlock: Option<FileInformationBlock>,
    // Java field: Boolean _isDoc
    pub(crate) _isDoc: Arc<Mutex<Boolean>>,
    // Java field: EncryptedType _encryptedType
    pub(crate) _encryptedType: Arc<Mutex<EncryptedType>>,
}

impl std::ops::Deref for DocChecker {
    type Target = OleFileChecker;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

// Java: public class DocChecker extends OleFileChecker
impl DocChecker {
    // Java: DocChecker(File file, RandomAccessFile raf, POIFSFileSystem poifs)
    pub fn DocChecker(&self, file: File, raf: RandomAccessFile, poifs: POIFSFileSystem) {
      // Java: super(file, raf, poifs);
      // LLM No Rust equivalent: module disabled, type not accessible
    }

    // Java: boolean isDoc()
    pub fn isDoc(&self) -> bool {
      // Java: if (null != this._isDoc)
        // LLM No Rust equivalent: module disabled, type not accessible
      {
         // Java: return this._isDoc;
         // LLM No Rust equivalent: module disabled, type not accessible
      }
      // Java: else
        // LLM No Rust equivalent: module disabled, type not accessible
      {
         // Java: KSLog.d("IOPerformance", "DocChecker.isDoc() begin");
         // LLM No Rust equivalent: module disabled, type not accessible
         // Java: if (!this.isOle())
            // LLM No Rust equivalent: module disabled, type not accessible
         {
            // Java: return this._isDoc = false;
            // LLM No Rust equivalent: module disabled, type not accessible
         }
         // Java: else
            // LLM No Rust equivalent: module disabled, type not accessible
         {
            // Java: if (null == this._Poifs)
                // LLM No Rust equivalent: module disabled, type not accessible
            {
               // Java: if (null != this.mRaf)
                    // LLM No Rust equivalent: module disabled, type not accessible
               {
                  // Java: this._Poifs = Tools.createPOIFSFileSystem(this.mRaf);
                  // LLM No Rust equivalent: module disabled, type not accessible
               }
               // Java: else
                    // LLM No Rust equivalent: module disabled, type not accessible
               {
                  // Java: this._Poifs = Tools.createPOIFSFileSystem(this._file);
                  // LLM No Rust equivalent: module disabled, type not accessible
               }
            }
            // Java: if (null == this._Poifs)
                // LLM No Rust equivalent: module disabled, type not accessible
            {
               // Java: return this._isDoc = false;
               // LLM No Rust equivalent: module disabled, type not accessible
            }
            // Java: else
                // LLM No Rust equivalent: module disabled, type not accessible
            {
               // Java: DirectoryNode directory = this._Poifs.getRoot();
               // LLM No Rust equivalent: module disabled, type not accessible
               // Java: if (directory == null)
                    // LLM No Rust equivalent: module disabled, type not accessible
               {
                  // Java: return this._isDoc = false;
                  // LLM No Rust equivalent: module disabled, type not accessible
               }
               // Java: else
                    // LLM No Rust equivalent: module disabled, type not accessible
               {
                  // Java: byte[] fibBase = createFibBaseStream("WordDocument", directory);
                  // LLM No Rust equivalent: module disabled, type not accessible
                  // Java: if (null == fibBase)
                        // LLM No Rust equivalent: module disabled, type not accessible
                  {
                     // Java: return this._isDoc = false;
                     // LLM No Rust equivalent: module disabled, type not accessible
                  }
                  // Java: else
                        // LLM No Rust equivalent: module disabled, type not accessible
                  {
                     // Java: this._fileInformationBlock = new FileInformationBlock(fibBase);
                     // LLM No Rust equivalent: module disabled, type not accessible
                     // Java: if (null != this._fileInformationBlock && this._fileInformationBlock.getNFib() > 104)
                            // LLM No Rust equivalent: module disabled, type not accessible
                     {
                        // Java: KSLog.d("IOPerformance", "DocChecker.isDoc() end");
                        // LLM No Rust equivalent: module disabled, type not accessible
                        // Java: return this._isDoc = true;
                        // LLM No Rust equivalent: module disabled, type not accessible
                     }
                     // Java: else
                            // LLM No Rust equivalent: module disabled, type not accessible
                     {
                        // Java: return this._isDoc = false;
                        // LLM No Rust equivalent: module disabled, type not accessible
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
        // LLM No Rust equivalent: module disabled, type not accessible
      {
         // Java: return this._encryptedType;
         // LLM No Rust equivalent: module disabled, type not accessible
      }
      // Java: else if ((null == this._isDoc || this._isDoc) && this.isDoc())
        // LLM No Rust equivalent: module disabled, type not accessible
      {
         // Java: if (null != this._fileInformationBlock && this._fileInformationBlock.isFEncrypted())
            // LLM No Rust equivalent: module disabled, type not accessible
         {
            // Java: if ((!this._fileInformationBlock.isFWhichTblStm() || null != this._Poifs.getRoot().getEntry2(STREAM_ENTRY_1TABLE)) && (this._fileInformationBlock.isFWhichTblStm() || null != this._Poifs.getRoot().getEntry2(STREAM_ENTRY_0TABLE)))
                // LLM No Rust equivalent: module disabled, type not accessible
            {
               // Java: this._encryptedType = EncryptedType.Doc;
               // LLM No Rust equivalent: module disabled, type not accessible
            }
            // Java: else
                // LLM No Rust equivalent: module disabled, type not accessible
            {
               // Java: this._encryptedType = EncryptedType.None;
               // LLM No Rust equivalent: module disabled, type not accessible
            }
         }
         // Java: else
            // LLM No Rust equivalent: module disabled, type not accessible
         {
            // Java: this._encryptedType = EncryptedType.None;
            // LLM No Rust equivalent: module disabled, type not accessible
         }
         // Java: return this._encryptedType;
         // LLM No Rust equivalent: module disabled, type not accessible
      }
      // Java: else
        // LLM No Rust equivalent: module disabled, type not accessible
      {
         // Java: return null;
         // LLM No Rust equivalent: module disabled, type not accessible
      }
    }

    // Java: boolean isWriteReservation()
    pub fn isWriteReservation(&self) -> bool {
      // Java: if ((null == this._isDoc || this._isDoc) && this.isDoc())
        // LLM No Rust equivalent: module disabled, type not accessible
      {
         // Java: return null != this._fileInformationBlock ? this._fileInformationBlock.isFWriteReservation() : false;
         // LLM No Rust equivalent: module disabled, type not accessible
      }
      // Java: else
        // LLM No Rust equivalent: module disabled, type not accessible
      {
         // Java: return false;
         // LLM No Rust equivalent: module disabled, type not accessible
      }
    }

    // Java: static byte[] createFibBaseStream(String name, DirectoryNode directory)
    pub fn createFibBaseStream(&self, name: String, directory: DirectoryNode) -> byte[] {
      // Java: Assert.assertNotNull("name should not be null!", name);
      // LLM No Rust equivalent: module disabled, type not accessible
      // Java: Assert.assertNotNull("directory should not be null!", directory);
      // LLM No Rust equivalent: module disabled, type not accessible
      // Java: DocumentInputStream is = null;
      // LLM No Rust equivalent: module disabled, type not accessible
      // Java: try
        // LLM No Rust equivalent: module disabled, type not accessible
      {
         // Java: is = directory.createDocumentInputStream(name);
         // LLM No Rust equivalent: module disabled, type not accessible
      }
      // Java: catch (Throwable e)
        // LLM No Rust equivalent: module disabled, type not accessible
      {
         // Java: IOLog.e(TAG, "Throwable", e);
         // LLM No Rust equivalent: module disabled, type not accessible
      }
      // Java: if (null == is)
        // LLM No Rust equivalent: module disabled, type not accessible
      {
         // Java: return null;
         // LLM No Rust equivalent: module disabled, type not accessible
      }
      // Java: else
        // LLM No Rust equivalent: module disabled, type not accessible
      {
         // Java: try
            // LLM No Rust equivalent: module disabled, type not accessible
         {
            // Java: byte[] fibbase = new byte[32];
            // LLM No Rust equivalent: module disabled, type not accessible
            // Java: is.read(fibbase);
            // LLM No Rust equivalent: module disabled, type not accessible
            // Java: byte[] var4 = fibbase;
            // LLM No Rust equivalent: module disabled, type not accessible
            // Java: return var4;
            // LLM No Rust equivalent: module disabled, type not accessible
         }
         // Java: catch (Throwable e)
            // LLM No Rust equivalent: module disabled, type not accessible
         {
            // Java: IOLog.e(TAG, "Throwable", e);
            // LLM No Rust equivalent: module disabled, type not accessible
         }
         // Java: finally
            // LLM No Rust equivalent: module disabled, type not accessible
         {
            // Java: is.close();
            // LLM No Rust equivalent: module disabled, type not accessible
         }
         // Java: return null;
         // LLM No Rust equivalent: module disabled, type not accessible
      }
    }

}
