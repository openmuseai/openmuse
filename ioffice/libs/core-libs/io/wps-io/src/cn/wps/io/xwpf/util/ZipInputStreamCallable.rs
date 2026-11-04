// Java: import cn.wps.moffice.kfs.zip.ZipInputStream
// use kfs_kfs_app::cn::wps::moffice::kfs::zip::ZipInputStream;
// Java: import java.util.concurrent.Callable

use std::io;

// Java: public class ZipInputStreamCallable implements Callable
pub struct ZipInputStreamCallable {
    // Java field: ZipInputStream mInp
    mInp: Option<Box<dyn std::any::Any + Send + Sync>>,
}

// Java: public class ZipInputStreamCallable implements Callable
impl ZipInputStreamCallable {
    // Java: ZipInputStreamCallable(ZipInputStream inp)
    pub fn new(inp: Box<dyn std::any::Any + Send + Sync>) -> Self {
      // Java: super();
      // Java: this.mInp = inp;
      ZipInputStreamCallable { mInp: Some(inp) }
    }

    // Java: Void call()
    pub fn call(&mut self) -> io::Result<()> {
      // Java: this.mInp.close();
      self.mInp = None;
      // Java: return null;
      Ok(())
    }

}
