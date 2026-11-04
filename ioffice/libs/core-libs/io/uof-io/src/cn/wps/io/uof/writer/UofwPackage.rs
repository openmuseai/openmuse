// Java: import cn.wps.moffice.kfs.zip.ZipEntry
use core_libs_kfs::kfs::zip::ZipEntry::ZipEntry;
// Java: import cn.wps.moffice.kfs.zip.ZipOutputStream
use core_libs_kfs::kfs::zip::ZipOutputStream::ZipOutputStream;
// Java: import cn.wps.moffice.util.FileUtil
// Java: import java.io.FileNotFoundException
// Java: import java.io.FileOutputStream
// Java: import java.io.IOException
// Java: import java.io.OutputStream
use std::io::Write;

// Java: public class UofwPackage
pub struct UofwPackage {
    // Java field: String mPath
    mPath: Option<String>,
    // Java field: ZipOutputStream mOs
    pub(crate) mOs: Option<ZipOutputStream>,
}

// Java: public class UofwPackage
impl UofwPackage {
    // Java: UofwPackage(String path)
    pub fn UofwPackage(path: String) -> std::io::Result<Self> {
      // Java: super();
      let mut pkg = UofwPackage {
          mPath: None,
          mOs: None,
      };
      // Java: this.mPath = path;
      pkg.mPath = Some(path.clone());
      // Java: this.mOs = new ZipOutputStream(new FileOutputStream(this.mPath));
      let fos = std::fs::File::create(&path)?;
      pkg.mOs = Some(ZipOutputStream::new(Box::new(fos)));
      Ok(pkg)
    }

    // Java: OutputStream createOutputStream(String name)
    pub fn createOutputStream(&mut self, name: String) -> Option<&mut ZipOutputStream> {
      // Java: ZipEntry e = new ZipEntry(name);
      let e = ZipEntry::new__string(name);
      // Java: try
      {
         // Java: this.mOs.putNextEntry(e);
         if let Some(ref mut os) = self.mOs {
             if os.putNextEntry_simple(e).is_ok() {
         // Java: return this.mOs;
                 return self.mOs.as_mut();
             }
         }
      }
      // Java: catch (IOException var4)
      {
         // Java: return null;
         None
      }
    }

    // Java: OutputStream createOutputStream(String name, int method, long compressedSize, long size, long crc)
    pub fn createOutputStream_full(&mut self, name: String, method: i32, compressedSize: i64, size: i64, crc: i64) -> Option<&mut ZipOutputStream> {
      // Java: ZipEntry zipEntry = new ZipEntry(name);
      let mut zipEntry = ZipEntry::new__string(name);
      // Java: zipEntry.setMethod(method);
      zipEntry.setMethod(method);
      // Java: zipEntry.setCompressedSize(compressedSize);
      zipEntry.setCompressedSize(compressedSize);
      // Java: zipEntry.setSize(size);
      zipEntry.setSize(size);
      // Java: zipEntry.setCrc(crc);
      zipEntry.setCrc(crc);
      // Java: try
      {
         // Java: this.mOs.putNextEntry(zipEntry);
         if let Some(ref mut os) = self.mOs {
             if os.putNextEntry_simple(zipEntry).is_ok() {
         // Java: return this.mOs;
                 return self.mOs.as_mut();
             }
         }
      }
      // Java: catch (IOException var11)
      {
         // Java: return null;
         None
      }
    }

    // Java: void close()
    pub fn close(&mut self) {
      // Java: FileUtil.closeQuietly(this.mOs);
      if let Some(mut os) = self.mOs.take() {
          let _ = os.close();
      }
    }

}
