// Java: import cn.wps.ole.IOLEStorage
use util_adapter::cn::wps::ole::IOLEStorage::*;
// Java: import java.io.ByteArrayInputStream
use stubs::java::io::ByteArrayInputStream::*;
// Java: import java.io.InputStream
use stubs::java::io::InputStream::*;
// Java: import org.apache.poi.hwpf.util.StorageUtil
use crate::org::apache::poi::hwpf::util::StorageUtil::*;
// Java: import org.apache.poi.hwpf.ole.packer.BaseOlePacker
use crate::org::apache::poi::hwpf::ole::packer::BaseOlePacker::*;

// java public class ChartOlePacker extends BaseOlePacker
#[derive(Default)]
pub struct ChartOlePacker {
    // java BaseOlePacker super_
    pub(crate) super_: BaseOlePacker,
    // java private String mPath;
    mPath: Option<String>,
}

// java public class ChartOlePacker extends BaseOlePacker
impl ChartOlePacker {
    // java public  ChartOlePacker(String path)
    pub fn ChartOlePacker(path: &str) -> Self {
      // java super(path);
      // java this.mPath = path;
      Self { super_: BaseOlePacker::new(&path), mPath: path }
    }

    // java public String[] getEntryNames()
    pub fn getEntryNames(&self) -> Vec<String> {
      // java return new String[]{"\u0003ObjInfo"};
      return vec!["\u{0003}ObjInfo"];
    }

    // java public InputStream getEntryStream(String entryName)
    pub fn getEntryStream(&self, entryName: Option<&str>) -> Option<Box<dyn std::io::Read>> {
      // java if (null == entryName)
      if entryName.is_none() {
        // java return null;
        return None;
      }
      // java else
      else {
        // java return "\u0003ObjInfo".equalsIgnoreCase(entryName) ? this.getOleInfoStream() : null;
        return (if "\u{0003}ObjInfo".equalsIgnoreCase(entryName) { Some(self.getOleInfoStream()) } else { None });
      }
    }

    // java protected InputStream getOleInfoStream()
    pub fn getOleInfoStream(&self) -> Box<dyn std::io::Read> {
      // java byte[] oleInfo = new byte[]{0, 2, 3, 0, 13, 0};
      let mut oleInfo: Vec<u8> = vec![0, 2, 3, 0, 13, 0];
      // java return new ByteArrayInputStream(oleInfo);
      return ByteArrayInputStream::new(oleInfo);
    }

    // java public void writeData(IOLEStorage storage)
    pub fn writeData(&self, storage: &dyn IOLEStorage) {
      // java StorageUtil.copyOleFile2Storage(this.mPath, storage);
      StorageUtil::copyOleFile2Storage(Some(&self.mPath), Some(&mut storage));
      // java super.writeData(storage);
      self.super_.writeData__iolestorage(&mut storage);
    }

    // java public String packBinFile()
    pub fn packBinFile(&self) -> String {
      // java return PackUtil.packBinFile(this.mPath, this, "{00020821-0000-0000-C000-000000000046}");
      return PackUtil::packBinFile(&self.mPath, &mut self, &"{00020821-0000-0000-C000-000000000046}");
    }

}

impl std::ops::Deref for ChartOlePacker {
    type Target = BaseOlePacker;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for ChartOlePacker {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}
