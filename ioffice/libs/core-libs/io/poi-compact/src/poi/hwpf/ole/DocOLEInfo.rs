// Java: import cn.wps.base.assertion.IOAssert
use util::cn::wps::base::assertion::IOAssert;

// Java: public class DocOLEInfo extends OleInfo
pub struct DocOLEInfo {
    // Java field: int mPicLocation
    pub(crate) mPicLocation: i32,
    // Java field: String mBinPath
    pub(crate) mBinPath: Arc<Mutex<String>>,
    // Java field: boolean mEncrypt
    pub(crate) mEncrypt: bool,
}

// Java: public class DocOLEInfo extends OleInfo
impl DocOLEInfo {
    // Java: DocOLEInfo(int picLocation, String binPath, boolean encrypt)
    pub fn DocOLEInfo(&self, int picLocation, String binPath, boolean encrypt) {
      // Java: super();
      // TODO_RUST_CODE
      // Java: IOAssert.assertNotNull("binName should not be null!", binPath);
      // TODO_RUST_CODE
      // Java: this.mPicLocation = picLocation;
      // TODO_RUST_CODE
      // Java: this.mBinPath = binPath;
      // TODO_RUST_CODE
      // Java: this.mEncrypt = encrypt;
      // TODO_RUST_CODE
    }

    // Java: String getBinPath()
    pub fn getBinPath(&self) -> String {
      // Java: return this.mBinPath;
      // TODO_RUST_CODE
    }

    // Java: int getLocation()
    pub fn getLocation(&self) -> i32 {
      // Java: return this.mPicLocation;
      // TODO_RUST_CODE
    }

    // Java: boolean isEncrypt()
    pub fn isEncrypt(&self) -> bool {
      // Java: return this.mEncrypt;
      // TODO_RUST_CODE
    }

}
