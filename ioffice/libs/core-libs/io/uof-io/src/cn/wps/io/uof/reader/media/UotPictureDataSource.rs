// Java: import cn.wps.base.assertion.IOAssert
// Java: import cn.wps.io.uof.UotZipEntrySource
use crate::cn::wps::io::uof::UotZipEntrySource::UotZipEntrySource;
// Java: import cn.wps.moffice.kfs.zip.ZipEntry
use core_libs_kfs::kfs::zip::ZipEntry::ZipEntry;

// Java: public class UotPictureDataSource
pub struct UotPictureDataSource {
    // Java field: String mFilename
    pub(crate) mFilename: String,
    // Java field: ZipEntry mEntry
    pub(crate) mEntry: ZipEntry,
    // Java field: UotZipEntrySource mSource
    pub(crate) mSource: Box<dyn UotZipEntrySource>,
}

// Java: public class UotPictureDataSource
impl UotPictureDataSource {
    // Java: UotPictureDataSource(String filename, ZipEntry entry, UotZipEntrySource source)
    pub fn UotPictureDataSource(filename: String, entry: ZipEntry, source: Box<dyn UotZipEntrySource>) -> Self {
      // Java: super();
      // Java: IOAssert.assertNotNull("filename should not be null!", filename);
      assert!(!filename.is_empty(), "filename should not be null!");
      // Java: IOAssert.assertNotNull("entry should not be null!", entry);
      // Java: IOAssert.assertNotNull("source should not be null!", source);
      // Java: this.mFilename = filename;
      // Java: this.mEntry = entry;
      // Java: this.mSource = source;
      UotPictureDataSource {
          mFilename: filename,
          mEntry: entry,
          mSource: source,
      }
    }

    // Java: String getFilename()
    pub fn getFilename(&self) -> &str {
      // Java: return this.mFilename;
      &self.mFilename
    }

    // Java: ZipEntry getEntry()
    pub fn getEntry(&self) -> &ZipEntry {
      // Java: return this.mEntry;
      &self.mEntry
    }

    // Java: UotZipEntrySource getSource()
    pub fn getSource(&self) -> &dyn UotZipEntrySource {
      // Java: return this.mSource;
      &*self.mSource
    }

}
