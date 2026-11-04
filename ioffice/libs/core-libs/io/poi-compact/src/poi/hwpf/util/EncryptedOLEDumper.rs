// Java: import cn.wps.moffice.kfs.FileOutputStream
use kfs_kfs_app::cn::wps::moffice::kfs::mfs::FileOutputStream;
// Java: import cn.wps.moffice.util.StringUtil
use util::org::apache::poi::util::StringUtil;
// Java: import cn.wps.moffice.util.zlib.ZInputStream
use util_adapter::cn::wps::moffice::util::zlib::ZInputStream;
// Java: import java.io.IOException
// TODO_RUST_CODE use ???; // java.io.IOException
// Java: import org.apache.poi.hwpf.model.FOBJH
use io_poi_compact::poi::hwpf::model::FOBJH;
// Java: import org.apache.poi.poifs.filesystem.DocumentInputStream
use io_poi_compact::org::apache::poi::poifs::filesystem::DocumentInputStream;

// Java: public class EncryptedOLEDumper
pub struct EncryptedOLEDumper {
}

// Java: public class EncryptedOLEDumper
impl EncryptedOLEDumper {
    // Java: EncryptedOLEDumper()
    pub fn EncryptedOLEDumper(&self) {
      // Java: super();
      // TODO_RUST_CODE
    }

    // Java: static String dump(DocumentInputStream dataStream, int offset, String path)
    pub fn dump(&self, DocumentInputStream dataStream, int offset, String path) -> String {
      // Java: if (StringUtil.isEmpty(path))
        // TODO_RUST_CODE
      {
         // Java: return null;
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: long originOffset = dataStream.tell();
         // TODO_RUST_CODE
         // Java: dataStream.seek((long)offset);
         // TODO_RUST_CODE
         // Java: byte[] buff = new byte[8];
         // TODO_RUST_CODE
         // Java: dataStream.read(buff);
         // TODO_RUST_CODE
         // Java: FOBJH fobjh = new FOBJH(buff);
         // TODO_RUST_CODE
         // Java: int originSize = dataStream.readInt();
         // TODO_RUST_CODE
         // Java: int sizeOleFollowed = fobjh.getCbObj();
         // TODO_RUST_CODE
         // Java: if (fobjh.isCompressed())
            // TODO_RUST_CODE
         {
            // Java: dumpDecrypted(path, dataStream, originSize);
            // TODO_RUST_CODE
         }
         // Java: else
            // TODO_RUST_CODE
         {
            // Java: dumpDirectly(path, dataStream, sizeOleFollowed);
            // TODO_RUST_CODE
         }
         // Java: dataStream.seek(originOffset);
         // TODO_RUST_CODE
         // Java: return path;
         // TODO_RUST_CODE
      }
    }

    // Java: static void dumpDirectly(String path, DocumentInputStream dataStream, int size)
    pub fn dumpDirectly(&self, String path, DocumentInputStream dataStream, int size) {
      // Java: byte[] buff = new byte[4096];
      // TODO_RUST_CODE
      // Java: FileOutputStream fos = new FileOutputStream(path);
      // TODO_RUST_CODE
      // Java: while(StorageUtil.checkInterrupted())
        // TODO_RUST_CODE
      {
         // Java: int n = dataStream.read(buff);
         // TODO_RUST_CODE
         // Java: if (n <= 0)
            // TODO_RUST_CODE
         {
            // Java: break;
            // TODO_RUST_CODE
         }
         // Java: int nWrite = n > size ? size : n;
         // TODO_RUST_CODE
         // Java: fos.write(buff, 0, nWrite);
         // TODO_RUST_CODE
         // Java: size -= nWrite;
         // TODO_RUST_CODE
         // Java: if (0 == size)
            // TODO_RUST_CODE
         {
            // Java: break;
            // TODO_RUST_CODE
         }
      }
      // Java: fos.close();
      // TODO_RUST_CODE
    }

    // Java: static void dumpDecrypted(String path, DocumentInputStream dataStream, int size)
    pub fn dumpDecrypted(&self, String path, DocumentInputStream dataStream, int size) {
      // Java: ZInputStream zis = new ZInputStream(dataStream);
      // TODO_RUST_CODE
      // Java: FileOutputStream fos = new FileOutputStream(path);
      // TODO_RUST_CODE
      // Java: byte[] buff = new byte[4096];
      // TODO_RUST_CODE
      // Java: while(StorageUtil.checkInterrupted())
        // TODO_RUST_CODE
      {
         // Java: int n = zis.read(buff);
         // TODO_RUST_CODE
         // Java: if (n <= 0)
            // TODO_RUST_CODE
         {
            // Java: break;
            // TODO_RUST_CODE
         }
         // Java: int nWrite = n > size ? size : n;
         // TODO_RUST_CODE
         // Java: fos.write(buff, 0, nWrite);
         // TODO_RUST_CODE
         // Java: size -= nWrite;
         // TODO_RUST_CODE
         // Java: if (0 == size)
            // TODO_RUST_CODE
         {
            // Java: break;
            // TODO_RUST_CODE
         }
      }
      // Java: fos.close();
      // TODO_RUST_CODE
    }

}
