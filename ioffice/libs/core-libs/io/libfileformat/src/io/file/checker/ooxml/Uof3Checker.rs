// Java: import cn.wps.io.file.EncryptedType
use io_libfileformat::cn::wps::io::file::EncryptedType;
// Java: import cn.wps.io.file.Tools
use kfs_kfs_app::cn::wps::moffice::kfs::tools::Tools;
// Java: import cn.wps.moffice.kfs.File
use kfs_kfs_app::cn::wps::moffice::kfs::mfs::File;
// Java: import cn.wps.moffice.kfs.RandomAccessFile
use kfs_kfs_app::cn::wps::moffice::kfs::RandomAccessFile;
// Java: import cn.wps.moffice.kfs.zip.ZipInputStream
use kfs_kfs_app::cn::wps::moffice::kfs::zip::ZipInputStream;
// Java: import org.apache.poi.poifs.filesystem.POIFSFileSystem
use io_poi_compact::org::apache::poi::poifs::filesystem::POIFSFileSystem;

// Java: public class Uof3Checker extends OOXMLChecker
pub struct Uof3Checker {
    // Java field: EncryptedType mEncryptedType
    pub(crate) mEncryptedType: Arc<Mutex<EncryptedType>>,
}

// Java: public class Uof3Checker extends OOXMLChecker
impl Uof3Checker {
    // Java: Uof3Checker(File file, RandomAccessFile raf, POIFSFileSystem poifs)
    pub fn Uof3Checker(&self, File file, RandomAccessFile raf, POIFSFileSystem poifs) {
      // Java: super(file, raf, poifs);
      // TODO_RUST_CODE
    }

    // Java: EncryptedType isEncrypted()
    pub fn isEncrypted(&self) -> EncryptedType {
      // Java: if (null != this.mEncryptedType)
        // TODO_RUST_CODE
      {
         // Java: return this.mEncryptedType;
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: ZipInputStream is = Tools.getEncrytionXmlInUof(this.mFile, this.mRaf);
         // TODO_RUST_CODE
         // Java: return null != is ? (this.mEncryptedType = EncryptedType.UOF) : (this.mEncryptedType = EncryptedType.None);
         // TODO_RUST_CODE
      }
    }

}
