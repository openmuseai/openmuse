#[allow(unused_imports)]
// Java: import cn.wps.moffice.kfs.zip.ZipEntry
use kfs_kfs_app::kfs::zip::ZipEntry;
#[allow(unused_imports)]
// Java: import cn.wps.moffice.kfs.zip.ZipFile
use kfs_kfs_app::kfs::zip::ZipFile;
#[allow(unused_imports)]
// Java: import cn.wps.util.manager.FileManager
use util::util::manager::FileManager;
#[allow(unused_imports)]
// Java: import cn.wps.util.zip.model.ZipBaseModel
use util::util::zip::model::ZipBaseModel;
// Java: import com.hp.hpl.inkml.lru.LruCache
#[allow(unused_imports)]
// Java: import com.hp.hpl.inkml.reader.InkmlReader
use crate::com::hp::hpl::inkml::reader::InkmlReader;
// Java: import java.io.File

use crate::com::hp::hpl::inkml::Ink::Ink;

// Java: public class InkModel extends ZipBaseModel
pub struct InkModel {
    // Java field: Ink mInk
    pub(crate) mInk: Option<Ink>,
    // Java field: LruCache<InkModel> mLruCache
    mLruCache: Option<i32>,
}

// Java: public class InkModel extends ZipBaseModel
impl InkModel {
    // Java: InkModel(Ink ink, FileManager fileManager)
    pub fn InkModel(ink: Ink, _file_manager: i32) -> Self {
      // Java: super("ink_temp", fileManager);
      // Java: this.mInk = ink;
      Self { mInk: Some(ink), mLruCache: None }
    }

    // Java: InkModel(ZipFile.RAFStream inputStream, String decompressSuffix, ZipEntry zipEntry, FileManager fileManager)
    pub fn InkModel_from_stream(_input_stream: i32, _decompress_suffix: &str, _zip_entry: i32, _file_manager: i32) -> Self {
      // Java: super(inputStream, decompressSuffix, zipEntry, fileManager);
      Self { mInk: None, mLruCache: None }
    }

    // Java: void setLruCache(LruCache<InkModel> lruCache)
    pub fn setLruCache(&mut self, _lru_cache: i32) {
      // Java: this.mLruCache = lruCache;
      self.mLruCache = Some(_lru_cache);
    }

    // Java: void clearInk()
    pub fn clearInk(&mut self) {
      // Java: this.mInk = null;
      self.mInk = None;
    }

    // Java: Ink getInk()
    pub fn getInk(&mut self) -> Option<&Ink> {
      // Java: if (this.mLruCache != null && this.getUncompressFile() != null)
      if self.mLruCache.is_some()
      {
         // Java: InkModel temp = this.mLruCache.add(this);
         let _temp: Option<i32> = None;
         // Java: if (temp != null)
         if false
         {
            // Java: temp.clearInk();
         }
      }
      // Java: if (this.mInk != null)
      if self.mInk.is_some()
      {
         // Java: return this.mInk;
         return self.mInk.as_ref();
      }
      // Java: else
      else
      {
         // Java: File file = this.getFile(true);
         // Java: InkmlReader reader = new InkmlReader(file.getAbsolutePath());
         // Java: return this.mInk = reader.read();
         return None;
      }
    }

    // Java: Ink getInkByTest()
    pub fn getInkByTest(&self) -> Option<&Ink> {
      // Java: return this.mInk;
      self.mInk.as_ref()
    }

    // Java: boolean isValidModel()
    pub fn isValidModel(&self) -> bool {
      // Java: return this.mInk != null ? true : super.isValidModel();
      self.mInk.is_some()
    }

}
