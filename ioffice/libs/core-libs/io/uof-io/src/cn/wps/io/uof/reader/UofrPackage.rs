// Java: import cn.wps.io.uof.UotZipEntrySource
use crate::cn::wps::io::uof::UotZipEntrySource::UotZipEntrySource;
// Java: import cn.wps.moffice.kfs.File
use core_libs_kfs::kfs::mfs::File::File;
// Java: import cn.wps.moffice.kfs.RandomAccessFile
use core_libs_kfs::kfs::mfs::RandomAccessFile::RandomAccessFile;
// Java: import cn.wps.moffice.kfs.zip.ZipEntry
use core_libs_kfs::kfs::zip::ZipEntry::ZipEntry;
// Java: import cn.wps.moffice.kfs.zip.ZipFile
use core_libs_kfs::kfs::zip::ZipFile::ZipFile;
// Java: import java.io.IOException
// Java: import java.io.InputStream
// Java: import java.util.Enumeration

// Java: public class UofrPackage implements UotZipEntrySource
pub struct UofrPackage {
    // Java field: ZipFile mZipFile
    pub(crate) mZipFile: Option<ZipFile>,
}

// Java: public class UofrPackage implements UotZipEntrySource
impl UofrPackage {
    // Java: UofrPackage(String path)
    pub fn UofrPackage(path: &str) -> std::io::Result<Self> {
      // Java: super();
      // Java: this.mZipFile = new ZipFile(path);
      Ok(UofrPackage {
          mZipFile: Some(ZipFile::new_from_file(path)?),
      })
    }

    // Java: UofrPackage(RandomAccessFile raf)
    pub fn UofrPackage_raf(raf: std::fs::File) -> std::io::Result<Self> {
      // Java: super();
      // Java: this.mZipFile = new ZipFile(raf);
      // std::fs::File has no stable path; wrap via kfs RandomAccessFile when available.
      // For now accept the handle by buffering to a temp file path ZipFile can open.
      use std::io::{Read, Write};
      let mut tmp = std::env::temp_dir();
      tmp.push(format!("uof_raf_{}.bin", std::process::id()));
      let mut out = std::fs::File::create(&tmp)?;
      let mut raf = raf;
      std::io::copy(&mut raf, &mut out)?;
      out.flush()?;
      let path = tmp.to_string_lossy().to_string();
      Ok(UofrPackage {
          mZipFile: Some(ZipFile::new_from_file(&path)?),
      })
    }

    // Java: UofrPackage(File file)
    pub fn UofrPackage_file(file: &File) -> std::io::Result<Self> {
      // Java: super();
      // Java: this.mZipFile = new ZipFile(file);
      Ok(UofrPackage {
          mZipFile: Some(ZipFile::new_from_file(&file.getAbsolutePath())?),
      })
    }

    // Java: InputStream getInputStream(String name)
    pub fn getInputStream_by_name(&self, name: &str) -> Option<Vec<u8>> {
      // Java: ZipEntry entry = new ZipEntry(name);
      let entry = ZipEntry::new__string(name.to_string());
      self.getInputStream_entry(&entry)
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
      // Java: return "UofPackage";
      "UofPackage".to_string()
    }

    // Java: ZipFile getZipFile()
    pub fn getZipFile(&self) -> Option<&ZipFile> {
      // Java: return this.mZipFile;
      self.mZipFile.as_ref()
    }

    // Java: public Enumeration<? extends ZipEntry> getEntries()
    pub fn getEntries_impl(&self) -> Vec<ZipEntry> {
      // Java: return this.mZipFile.entries();
      if let Some(zf) = &self.mZipFile {
          return zf.entries();
      }
      Vec::new()
    }

    // Java: int size()
    pub fn size_impl(&self) -> i32 {
      // Java: return this.mZipFile.size();
      if let Some(zf) = &self.mZipFile {
          return zf.size() as i32;
      }
      0
    }

    // Java: InputStream getInputStream(ZipEntry entry)
    pub fn getInputStream_entry(&self, entry: &ZipEntry) -> Option<Vec<u8>> {
      // Java: try
      {
         // Java: return this.mZipFile.getInputStream(entry);
         if let Some(zf) = &self.mZipFile {
             let mut zf_mut = ZipFile::new_from_file(&zf.filename).ok()?;
             return zf_mut.getInputStream(entry).map(|mut r| {
                 let mut buf = Vec::new();
                 let _ = std::io::Read::read_to_end(&mut *r, &mut buf);
                 buf
             });
         }
      }
      // Java: catch (IOException var3)
      {
         // Java: return null;
         None
      }
    }

    // Java: void close()
    pub fn close_impl(&mut self) {
      // Java: if (null != this.mZipFile)
      if self.mZipFile.is_some() {
         // Java: this.mZipFile.close();
         if let Some(ref mut zf) = self.mZipFile {
             let _ = zf.close();
         }
         // Java: this.mZipFile = null;
         self.mZipFile = None;
      }
    }

}

impl UotZipEntrySource for UofrPackage {
    fn getEntries(&self) -> Vec<ZipEntry> {
        self.getEntries_impl()
    }
    fn size(&self) -> i32 {
        self.size_impl()
    }
    fn getInputStream(&self, var1: &ZipEntry) -> std::io::Result<Box<dyn std::io::Read>> {
        match self.getInputStream_entry(var1) {
            Some(data) => Ok(Box::new(std::io::Cursor::new(data))),
            None => Err(std::io::Error::new(std::io::ErrorKind::NotFound, "entry not found")),
        }
    }
    fn close(&mut self) {
        self.close_impl();
    }
}
