// Java: import cn.wps.moffice.kfs.File
use kfs_kfs_app::cn::wps::moffice::kfs::mfs::File;
// Java: import cn.wps.moffice.kfs.RandomAccessFile
use kfs_kfs_app::cn::wps::moffice::kfs::RandomAccessFile;
// Java: import org.apache.poi.poifs.filesystem.POIFSFileSystem
use io_poi_compact::org::apache::poi::poifs::filesystem::POIFSFileSystem;

// Java field: String OLE_DIRECTORY_ENTRY_ELENAME_PowerPoint_Document
pub(crate) const OLE_DIRECTORY_ENTRY_ELENAME_PowerPoint_Document: &str = "PowerPoint Document";

// Java: public class PPTChecker extends OleFileChecker
pub struct PPTChecker {
    pub(crate) parent: OleFileChecker,
    // Java field: Boolean _isPPT
    pub(crate) _isPPT: Option<bool>,
}

impl std::ops::Deref for PPTChecker {
    type Target = OleFileChecker;
    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

// Java: public class PPTChecker extends OleFileChecker
impl PPTChecker {
    // Java: PPTChecker(File file, RandomAccessFile raf, POIFSFileSystem poifs)
    pub fn PPTChecker(&mut self, file: Option<File>, raf: Option<RandomAccessFile>, poifs: Option<POIFSFileSystem>) {
      // Java: super(file, raf, poifs);
      self.OleFileChecker(file, raf, poifs);
    }

    // Java: boolean isPPT()
    pub fn isPPT(&self) -> bool {
      // Java: if (null != this._isPPT)
      if self._isPPT.is_some() {
         // Java: return this._isPPT;
         return self._isPPT.unwrap();
      }
      // Java: else
      else {
         // Java: this._isPPT = this.hasEntry("PowerPoint Document");
         self._isPPT = Some(self.hasEntry("PowerPoint Document".to_string()));
         // Java: return this._isPPT;
         return self._isPPT.unwrap();
      }
    }

}
