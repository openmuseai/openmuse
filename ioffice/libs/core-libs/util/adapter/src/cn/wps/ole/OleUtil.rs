// Java: import cn.wps.moffice.kfs.File
// use kfs_kfs_app::cn::wps::moffice::kfs::mfs::File;
// Java: import cn.wps.moffice.kfs.FileInputStream
// use kfs_kfs_app::cn::wps::moffice::kfs::mfs::FileInputStream;
// Java: import java.io.FileOutputStream
// Java: import java.io.IOException
// Java: import java.io.InputStream
// Java: import java.io.OutputStream

// Java: public class OleUtil
pub struct OleUtil {
}

// Java: public class OleUtil
impl OleUtil {
    // Java: OleUtil()
    pub fn new() -> Self {
      // Java: super();
      OleUtil {}
    }

    // Java: static void write2OLEStream(IOLEStream out, File in)
    pub fn write2OLEStream(_out: &mut dyn std::any::Any, _in_file: &dyn std::any::Any) {
      // Java: FileInputStream fis = new FileInputStream(in);
      // Java: write2OLEStream(out, (InputStream)fis);
      todo!()
    }

    // Java: static void write2OLEStream(IOLEStream out, InputStream in)
    pub fn write2OLEStream_from_stream(_out: &mut dyn std::any::Any, _in_stream: &mut dyn std::any::Any) {
      // Java: byte[] buff = new byte[4096];
      // Java: while(true)
      {
         // Java: int n = in.read(buff);
         // Java: if (n <= 0)
         {
            // Java: in.close();
            // Java: out.close();
            // Java: return;
         }
         // Java: out.write(buff, 0, n);
      }
      todo!()
    }

    // Java: static void readFromOLEStream(String out, IOLEStream in)
    pub fn readFromOLEStream(_out: &str, _in_stream: &mut dyn std::any::Any) {
      // Java: OutputStream fos = new FileOutputStream(out);
      // Java: readFromOLEStream(fos, in);
      todo!()
    }

    // Java: static void readFromOLEStream(OutputStream out, IOLEStream in)
    pub fn readFromOLEStream_from_output(_out: &mut dyn std::any::Any, _in_stream: &mut dyn std::any::Any) {
      // Java: byte[] buff = new byte[4096];
      // Java: while(true)
      {
         // Java: int n = in.read(buff, 0, buff.length);
         // Java: if (n <= 0)
         {
            // Java: out.close();
            // Java: in.close();
            // Java: return;
         }
         // Java: out.write(buff, 0, n);
      }
      todo!()
    }

}
