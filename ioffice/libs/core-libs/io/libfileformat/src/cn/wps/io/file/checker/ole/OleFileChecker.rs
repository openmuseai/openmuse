// Java: import cn.wps.base.log.Log
use kfs_kfs_app::cn::wps::moffice::kfs::log::Log;
// Java: import cn.wps.io.file.Tools
use io_libfileformat::cn::wps::io::file::Tools;
// Java: import cn.wps.moffice.kfs.File
use kfs_kfs_app::cn::wps::moffice::kfs::mfs::File;
// Java: import cn.wps.moffice.kfs.RandomAccessFile
use kfs_kfs_app::cn::wps::moffice::kfs::RandomAccessFile;
// Java: import java.io.IOException
// use placeholder; // java.io.IOException
// Java: import org.apache.poi.poifs.filesystem.DirectoryNode
use io_poi_compact::org::apache::poi::poifs::filesystem::DirectoryNode;
// Java: import org.apache.poi.poifs.filesystem.DocumentEntry
use io_poi_compact::org::apache::poi::poifs::filesystem::DocumentEntry;
// Java: import org.apache.poi.poifs.filesystem.POIFSFileSystem
use io_poi_compact::org::apache::poi::poifs::filesystem::POIFSFileSystem;

// Java field: String TAG
pub(crate) const TAG: &str = "OleFileChecker";

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
    pub fn OleFileChecker(&mut self, file: Option<File>, raf: Option<RandomAccessFile>, poifs: Option<POIFSFileSystem>) {
      // Java: super();
      let _ = ();
      // Java: this._file = file;
      self._file = file;
      // Java: this.mRaf = raf;
      self.mRaf = raf;
      // Java: this._Poifs = poifs;
      self._Poifs = poifs;
    }

    // Java: boolean isOle()
    pub fn isOle(&self) -> bool {
      // Java: RandomAccessFile file = null;
      let file = todo!();
      // Java: try
      match (|| -> Result<(), Box<dyn std::error::Error>> {
         // Java: file = null != this.mRaf ? this.mRaf : new RandomAccessFile(this._file, "r");
         let file = todo!();
         // Java: this._isOle = POIFSFileSystem.hasPOIFSHeader(file);
         self._isOle = todo!();
         Ok(())
      })() {
      // Java: catch (IOException e)
      Err(_e) => {
         // Java: Log.e(TAG, "IOException", e);
         Log::e(TAG, "IOException");
      }, Ok(_) => {} }
      // Java: if (null != file)
      if false {
         // Java: try
         match (|| -> Result<(), Box<dyn std::error::Error>> {
            // Java: file.close();
            file.close();
            Ok(())
         })() {
         // Java: catch (IOException e)
         Err(_e) => {
            // Java: Log.e(TAG, "IOException", e);
            Log::e(TAG, "IOException");
         }, Ok(_) => {} }
      }
      // Java: return this._isOle != null ? this._isOle : false;
      return self._isOle.unwrap_or(false);
    }

    // Java: boolean hasEntry(String nameEntry)
    pub fn hasEntry(&self, nameEntry: String) -> bool {
      // Java: if (!this.isOle())
      if !self.isOle() {
         // Java: return false;
         return false;
      }
      // Java: else
      else {
         // Java: if (null == this._Poifs)
         if self._Poifs.is_none() {
            // Java: long length = 0L;
            let length: i64 = 0;
            // Java: try
            match (|| -> Result<(), Box<dyn std::error::Error>> {
               // Java: length = null != this.mRaf ? this.mRaf.length() : this._file.length();
               let length = todo!();
               // Java: if (length <= 1048576L)
               if length <= 1048576i64 {
                  // Java: if (null != this.mRaf)
                  if self.mRaf.is_some() {
                     // Java: this._Poifs = Tools.createPOIFSFileSystem(this.mRaf);
                     self._Poifs = Tools::createPOIFSFileSystem(todo!());
                  }
                  // Java: else
                  else {
                     // Java: this._Poifs = Tools.createPOIFSFileSystem(Tools.createFileInputStream(this._file));
                     self._Poifs = Tools::createPOIFSFileSystem(Tools::createFileInputStream(todo!()));
                  }
               }
               // Java: else if (null != this.mRaf)
               else if self.mRaf.is_some() {
                  // Java: this._Poifs = Tools.createPOIFSFileSystem(this.mRaf);
                  self._Poifs = Tools::createPOIFSFileSystem(todo!());
               }
               // Java: else
               else {
                  // Java: this._Poifs = Tools.createPOIFSFileSystem(this._file);
                  self._Poifs = Tools::createPOIFSFileSystem(todo!());
               }
               Ok(())
            })() {
            // Java: catch (IOException e)
            Err(_e) => {
               // Java: Log.e(TAG, "IOException", e);
               Log::e(TAG, "IOException");
            }, Ok(_) => {} }
         }
         // Java: if (null == this._Poifs)
         if self._Poifs.is_none() {
            // Java: return false;
            return false;
         }
         // Java: else
         else {
            // Java: DirectoryNode directory = this._Poifs.getRoot();
            let directory = todo!();
            // Java: if (null == directory)
            if false {
               // Java: return false;
               return false;
            }
            // Java: else
            else {
               // Java: DocumentEntry documentProps = Tools.createDocumentEntry(nameEntry, directory);
               let documentProps = Tools::createDocumentEntry(todo!(), todo!());
               // Java: return null != documentProps;
               return documentProps.is_some();
            }
         }
      }
    }

    // Java: POIFSFileSystem getPOIFSFileSystem()
    pub fn getPOIFSFileSystem(&self) -> POIFSFileSystem {
      // Java: return this._Poifs;
      return todo!();
    }

}
