// Java: import cn.wps.ole.IOLEStorage
use util_adapter::cn::wps::ole::IOLEStorage;
// Java: import java.io.ByteArrayInputStream
// TODO_RUST_CODE use ???; // java.io.ByteArrayInputStream
// Java: import java.io.InputStream
// TODO_RUST_CODE use ???; // java.io.InputStream
// Java: import org.apache.poi.hwpf.util.StorageUtil
use io_poi_compact::org::apache::poi::hwpf::util::StorageUtil;

// Java: public class ChartOlePacker extends BaseOlePacker
pub struct ChartOlePacker {
    // Java field: String mPath
    mPath: Option<String>,
}

// Java: public class ChartOlePacker extends BaseOlePacker
impl ChartOlePacker {
    // Java: ChartOlePacker(String path)
    pub fn ChartOlePacker(&self, String path) {
      // Java: super(path);
      // TODO_RUST_CODE
      // Java: this.mPath = path;
      // TODO_RUST_CODE
    }

    // Java: String[] getEntryNames()
    pub fn getEntryNames(&self) -> String[] {
      // Java: return new String[]{"\u0003ObjInfo"};
      // TODO_RUST_CODE
    }

    // Java: InputStream getEntryStream(String entryName)
    pub fn getEntryStream(&self, String entryName) -> InputStream {
      // Java: if (null == entryName)
        // TODO_RUST_CODE
      {
         // Java: return null;
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: return "\u0003ObjInfo".equalsIgnoreCase(entryName) ? this.getOleInfoStream() : null;
         // TODO_RUST_CODE
      }
    }

    // Java: InputStream getOleInfoStream()
    pub fn getOleInfoStream(&self) -> InputStream {
      // Java: byte[] oleInfo = new byte[]{0, 2, 3, 0, 13, 0};
      // TODO_RUST_CODE
      // Java: return new ByteArrayInputStream(oleInfo);
      // TODO_RUST_CODE
    }

    // Java: void writeData(IOLEStorage storage)
    pub fn writeData(&self, IOLEStorage storage) {
      // Java: StorageUtil.copyOleFile2Storage(this.mPath, storage);
      // TODO_RUST_CODE
      // Java: super.writeData(storage);
      // TODO_RUST_CODE
    }

    // Java: String packBinFile()
    pub fn packBinFile(&self) -> String {
      // Java: return PackUtil.packBinFile(this.mPath, this, "{00020821-0000-0000-C000-000000000046}");
      // TODO_RUST_CODE
    }

}
