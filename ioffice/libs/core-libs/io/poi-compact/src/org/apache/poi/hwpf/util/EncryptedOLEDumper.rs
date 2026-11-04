// Java: import cn.wps.moffice.kfs.FileOutputStream
use kfs_kfs_app::cn::wps::moffice::kfs::FileOutputStream::*;
// Java: import cn.wps.moffice.util.StringUtil
use util::moffice::util::StringUtil::*;
// Java: import cn.wps.moffice.util.zlib.ZInputStream
use util_adapter::cn::wps::moffice::util::zlib::ZInputStream::*;
// Java: import java.io.IOException
use stubs::java::io::IOException::*;
// Java: import org.apache.poi.hwpf.model.FOBJH
use crate::org::apache::poi::hwpf::model::FOBJH::*;
// Java: import org.apache.poi.poifs.filesystem.DocumentInputStream
use crate::org::apache::poi::poifs::filesystem::DocumentInputStream::*;
// Java: import org.apache.poi.hwpf.util.OleParseInterruptException
use crate::org::apache::poi::hwpf::util::OleParseInterruptException::*;

// java public class EncryptedOLEDumper
#[derive(Default)]
pub struct EncryptedOLEDumper {
}

// java public class EncryptedOLEDumper
impl EncryptedOLEDumper {
    // java public  EncryptedOLEDumper()
    pub fn EncryptedOLEDumper() -> Self {
      // java super();
      Self {}
    }

    // java public static String dump(DocumentInputStream dataStream, int offset, String path)
    pub fn dump(dataStream: &mut DocumentInputStream, offset: i32, path: &str) -> Option<String> {
      // java if (StringUtil.isEmpty(path))
      if StringUtil::isEmpty(Some(&mut path)) {
        // java return null;
        return None;
      }
      // java else
      else {
        // java long originOffset = dataStream.tell();
        let mut originOffset: i64 = dataStream.tell();
        // java dataStream.seek((long)offset);
        dataStream.seek(((offset) as i64));
        // java byte[] buff = new byte[8];
        let mut buff: Vec<u8> = vec![Default::default(); 8usize];
        // java dataStream.read(buff);
        dataStream.read__bytearr(&buff);
        // java FOBJH fobjh = new FOBJH(buff);
        let mut fobjh: FOBJH = FOBJH::new(buff);
        // java int originSize = dataStream.readInt();
        let mut originSize: i32 = dataStream.readInt();
        // java int sizeOleFollowed = fobjh.getCbObj();
        let mut sizeOleFollowed: i32 = fobjh.getCbObj();
        // java if (fobjh.isCompressed())
        if fobjh.isCompressed() {
          // java dumpDecrypted(path, dataStream, originSize);
          Self::dumpDecrypted(&path, &dataStream, originSize);
        }
        // java else
        else {
          // java dumpDirectly(path, dataStream, sizeOleFollowed);
          Self::dumpDirectly(&path, &mut dataStream, sizeOleFollowed);
        }
        // java dataStream.seek(originOffset);
        dataStream.seek(originOffset);
        // java return path;
        return path;
      }
    }

    // java private static void dumpDirectly(String path, DocumentInputStream dataStream, int size)
    pub fn dumpDirectly(path: &str, dataStream: &mut DocumentInputStream, size: i32) {
      // java byte[] buff = new byte[4096];
      let mut buff: Vec<u8> = vec![Default::default(); 4096usize];
      // java FileOutputStream fos = new FileOutputStream(path);
      let mut fos: Box<dyn std::any::Any + Send + Sync> = FileOutputStream::new(path);
      // java while (StorageUtil.checkInterrupted())
      while StorageUtil::checkInterrupted() {
        // java int n = dataStream.read(buff);
        let mut n: i32 = dataStream.read__bytearr(&buff);
        // java if (n <= 0)
        if n <= 0 {
          // java break;
          break;
        }
        // java int nWrite = n > size ? size : n;
        let mut nWrite: i32 = (if n > size { size } else { n });
        // java fos.write(buff, 0, nWrite);
        fos.write(buff, 0, nWrite);
        // java size -= nWrite;
        size = size - nWrite;
        // java if (0 == size)
        if 0 == size {
          // java break;
          break;
        }
      }
      // java fos.close();
      fos.close();
    }

    // java private static void dumpDecrypted(String path, DocumentInputStream dataStream, int size)
    pub fn dumpDecrypted(path: &str, dataStream: &DocumentInputStream, size: i32) {
      // java ZInputStream zis = new ZInputStream(dataStream);
      let mut zis: ZInputStream = ZInputStream::new(dataStream);
      // java FileOutputStream fos = new FileOutputStream(path);
      let mut fos: Box<dyn std::any::Any + Send + Sync> = FileOutputStream::new(path);
      // java byte[] buff = new byte[4096];
      let mut buff: Vec<u8> = vec![Default::default(); 4096usize];
      // java while (StorageUtil.checkInterrupted())
      while StorageUtil::checkInterrupted() {
        // java int n = zis.read(buff);
        let mut n: i32 = zis.read(buff);
        // java if (n <= 0)
        if n <= 0 {
          // java break;
          break;
        }
        // java int nWrite = n > size ? size : n;
        let mut nWrite: i32 = (if n > size { size } else { n });
        // java fos.write(buff, 0, nWrite);
        fos.write(buff, 0, nWrite);
        // java size -= nWrite;
        size = size - nWrite;
        // java if (0 == size)
        if 0 == size {
          // java break;
          break;
        }
      }
      // java fos.close();
      fos.close();
    }

}
