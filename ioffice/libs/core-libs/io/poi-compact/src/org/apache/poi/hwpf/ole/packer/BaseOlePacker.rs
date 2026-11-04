// Java: import cn.wps.base.log.IOLog
use util::base::log::IOLog::*;
// Java: import cn.wps.moffice.kfs.File
use kfs_kfs_app::cn::wps::moffice::kfs::File::*;
// Java: import cn.wps.moffice.kfs.FileInputStream
use kfs_kfs_app::cn::wps::moffice::kfs::FileInputStream::*;
// Java: import cn.wps.moffice.util.FileUtil
use util::moffice::util::FileUtil::*;
// Java: import cn.wps.ole.IOLEStorage
use util_adapter::cn::wps::ole::IOLEStorage::*;
// Java: import cn.wps.ole.IOLEStream
use util_adapter::cn::wps::ole::IOLEStream::*;
// Java: import java.io.ByteArrayInputStream
use stubs::java::io::ByteArrayInputStream::*;
// Java: import java.io.FileNotFoundException
use stubs::java::io::FileNotFoundException::*;
// Java: import java.io.IOException
use stubs::java::io::IOException::*;
// Java: import java.io.InputStream
use stubs::java::io::InputStream::*;
// Java: import org.apache.poi.poifs.filesystem.DirectoryEntry
use crate::org::apache::poi::poifs::filesystem::DirectoryEntry::*;
// Java: import org.apache.poi.hwpf.ole.packer.IOlePacker
use crate::org::apache::poi::hwpf::ole::packer::IOlePacker::*;

// java private static final String TAG = null;
const TAG: &str = "";
// java protected static final String ENTITY_NAME_PKG = "Package";
pub const ENTITY_NAME_PKG: &str = "Package";
// java protected static final String ENTITY_NAME_OBJINFO = "\u0003ObjInfo";
pub const ENTITY_NAME_OBJINFO: &str = "\u{0003}ObjInfo";

// java public abstract class BaseOlePacker implements IOlePacker
#[derive(Default)]
pub struct BaseOlePacker {
    // java protected String mPath;
    pub mPath: Option<String>,
}

// java public interface IOlePacker
impl IOlePacker for BaseOlePacker {
    // java public String[] getEntryNames()
    fn getEntryNames(&self) -> Vec<String> {
      // java return new String[]{"Package", "\u0003ObjInfo"};
      return vec!["Package", "\u{0003}ObjInfo"];
    }

    // java public InputStream getEntryStream(String entryName)
    fn getEntryStream(&self, entryName: Option<&str>) -> Option<Box<dyn std::io::Read>> {
      // java if (null == entryName)
      if entryName.is_none() {
        // java return null;
        return None;
      }
      // java else if ("Package".equals(entryName))
      else if "Package".equals(entryName) {
        // java return this.getPackageStream();
        return self.getPackageStream();
      }
      // java else
      else {
        // java return "\u0003ObjInfo".equalsIgnoreCase(entryName) ? this.getOleInfoStream() : null;
        return (if "\u{0003}ObjInfo".equalsIgnoreCase(entryName) { Some(self.getOleInfoStream()) } else { None });
      }
    }

    // java public void writeData(IOLEStorage storage)
    fn writeData__iolestorage(&mut self, storage: &mut dyn IOLEStorage) {
      // java String[] names = this.getEntryNames();
      let mut names: Vec<String> = self.getEntryNames();
      // java if (null != names)
      if names.is_some() {
        let mut index: i32 = 0;
        // java for (int index = 0; index < names.length ++index)
        while index < (names.len() as i32) {
          // java String name = names[index];
          let mut name: String = names[(index) as usize];
          // java InputStream inputStream = this.getEntryStream(name);
          let mut inputStream: Box<dyn std::io::Read> = self.getEntryStream(Some(&name));
          // java if (null != inputStream)
          if inputStream.is_some() {
                // java IOLEStream destStm = storage.createStream(name);
                let mut destStm: Box<dyn IOLEStream> = storage.createStream(&name);
                // java this.writeEntry(name, destStm, inputStream);
                self.writeEntry(&name, Some(&mut destStm), &mut inputStream);
                // java IOException var10
                todo!();
                  // java IOLog.e(TAG, "createStream failed: " + name);
                  IOLog::e__string_string(&TAG, &"createStream failed: " + name);
                  // java FileUtil.closeQuietly(inputStream);
                  FileUtil::closeQuietly__fileoutputstream(inputStream);
          }
        }
      }
    }

    // java public void writeData(DirectoryEntry entry)
    fn writeData__directoryentry(&self, entry: &mut dyn DirectoryEntry) {
      // java String[] names = this.getEntryNames();
      let mut names: Vec<String> = self.getEntryNames();
      // java if (null != names)
      if names.is_some() {
        let mut index: i32 = 0;
        // java for (int index = 0; index < names.length ++index)
        while index < (names.len() as i32) {
          // java String name = names[index];
          let mut name: String = names[(index) as usize];
          // java InputStream inputStream = this.getEntryStream(name);
          let mut inputStream: Box<dyn std::io::Read> = self.getEntryStream(Some(&name));
          // java if (null != inputStream)
          if inputStream.is_some() {
                // java entry.createDocument(name, inputStream);
                entry.createDocument__string_inputstream(&name, &inputStream);
                // java IOException var10
                todo!();
                  // java IOLog.e(TAG, "createStream failed: " + name);
                  IOLog::e__string_string(&TAG, &"createStream failed: " + name);
                  // java FileUtil.closeQuietly(inputStream);
                  FileUtil::closeQuietly__fileoutputstream(inputStream);
          }
        }
      }
    }

    // java public String packBinFile()
    fn packBinFile(&self) -> String {
      // java return this.mPath;
      return self.mPath;
    }

}

// java public abstract class BaseOlePacker implements IOlePacker
impl BaseOlePacker {
    // java public  BaseOlePacker(String path)
    pub fn BaseOlePacker(path: &str) -> Self {
      // java super();
      // java this.mPath = path;
      Self { mPath: path }
    }

    // java protected InputStream getPackageStream()
    pub fn getPackageStream(&self) -> Option<Box<dyn std::io::Read>> {
      // java if (null != this.mPath)
      if let Some(mPath) = self.mPath.as_ref() {
            // java return new FileInputStream(new File(this.mPath));
            return FileInputStream::new(File::new(mPath));
            // java FileNotFoundException var2
            todo!();
              // java IOLog.e(TAG, "file not found ：" + this.mPath);
              IOLog::e__string_string(&TAG, &"file not found ：" + mPath);
      }
      // java return null;
      return None;
    }

    // java protected InputStream getOleInfoStream()
    pub fn getOleInfoStream(&self) -> Box<dyn std::io::Read> {
      // java byte[] oleInfo = new byte[]{64, 0, 3, 0, 13, 0};
      let mut oleInfo: Vec<u8> = vec![64, 0, 3, 0, 13, 0];
      // java return new ByteArrayInputStream(oleInfo);
      return ByteArrayInputStream::new(oleInfo);
    }

    // java protected void writeEntry(String entryName, IOLEStream destStm, InputStream inputStream)
    pub fn writeEntry(&mut self, entryName: &str, destStm: Option<&mut dyn IOLEStream>, inputStream: &mut dyn std::io::Read) {
      // java byte[] buffer = new byte[4096];
      let mut buffer: Vec<u8> = vec![Default::default(); 4096usize];
      // java int len = 0;
      let mut len: i32 = 0;
          // java while ((len = inputStream.read(buffer)) > 0)
          while { len = inputStream.read(buffer); len } > 0 {
            // java destStm.write(buffer, 0, len);
            destStm.write(buffer, 0, len);
          }
          // java if (destStm != null)
          if let Some(destStm) = destStm {
            // java destStm.close();
            destStm.close();
          }
          // java IOException e
          todo!();
            // java IOLog.e(TAG, "writeData failed: " + entryName, e);
            IOLog::e__string_string_throwable(&TAG, &"writeData failed: " + entryName, &e);
            // java FileUtil.closeQuietly(inputStream);
            FileUtil::closeQuietly__fileoutputstream(inputStream);
    }

}
