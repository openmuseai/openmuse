// Java: import cn.wps.moffice.kfs.File
use kfs_kfs_app::cn::wps::moffice::kfs::mfs::File;
// Java: import cn.wps.moffice.kfs.RandomAccessFile
use kfs_kfs_app::cn::wps::moffice::kfs::RandomAccessFile;
// Java: import org.apache.poi.poifs.filesystem.POIFSFileSystem
use io_poi_compact::org::apache::poi::poifs::filesystem::POIFSFileSystem;

// Java field: String OLE_DIRECTORY_ENTRY_ELENAME_WORKBOOK
pub(crate) const OLE_DIRECTORY_ENTRY_ELENAME_WORKBOOK: &str = "Workbook";

// Java: public class XlsChecker extends OleFileChecker
pub struct XlsChecker {
    pub(crate) parent: OleFileChecker,
    // Java field: Boolean _isXls
    pub(crate) _isXls: Option<bool>,
}

impl std::ops::Deref for XlsChecker {
    type Target = OleFileChecker;
    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

// Java: public class XlsChecker extends OleFileChecker
impl XlsChecker {
    // Java: XlsChecker(File file, RandomAccessFile raf, POIFSFileSystem poifs)
    pub fn XlsChecker(&mut self, file: Option<File>, raf: Option<RandomAccessFile>, poifs: Option<POIFSFileSystem>) {
      // Java: super(file, raf, poifs);
      self.parent.OleFileChecker(file, raf, poifs);
    }

    // Java: boolean isXls()
    pub fn isXls(&self) -> bool {
      // Java: if (null != this._isXls)
      if self._isXls.is_some() {
         // Java: return this._isXls;
         return self._isXls.unwrap();
      }
      // Java: else
      else {
         // Java: this._isXls = this.hasEntry("Workbook");
         self._isXls = Some(self.hasEntry("Workbook".to_string()));
         // Java: return this._isXls;
         return self._isXls.unwrap();
      }
    }

}
