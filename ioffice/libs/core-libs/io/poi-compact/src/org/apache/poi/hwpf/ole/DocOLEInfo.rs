// Java: import cn.wps.base.assertion.IOAssert
use util::base::assertion::IOAssert::*;
// Java: import org.apache.poi.hwpf.ole.OleInfo
use crate::org::apache::poi::hwpf::ole::OleInfo::*;

// java public class DocOLEInfo extends OleInfo
#[derive(Default)]
pub struct DocOLEInfo {
    // java OleInfo super_
    pub(crate) super_: OleInfo,
    // java private int mPicLocation = -1;
    mPicLocation: i32,
    // java private String mBinPath = null;
    mBinPath: Option<String>,
    // java private boolean mEncrypt;
    mEncrypt: bool,
}

// java public class DocOLEInfo extends OleInfo
impl DocOLEInfo {
    // java public  DocOLEInfo(int picLocation, String binPath, boolean encrypt)
    pub fn DocOLEInfo(picLocation: i32, binPath: Option<String>, encrypt: bool) -> Self {
      // java super();
      // java IOAssert.assertNotNull("binName should not be null!", binPath);
      IOAssert::assertNotNull__string_object(&"binName should not be null!", Some(&binPath));
      // java this.mPicLocation = picLocation;
      let mut this = Self { super_: OleInfo::new(), mPicLocation: picLocation, mBinPath: binPath, mEncrypt: encrypt };
      // java this.mBinPath = binPath;
      // java this.mEncrypt = encrypt;
      this
    }

    // java public final String getBinPath()
    pub fn getBinPath(&self) -> Option<String> {
      // java return this.mBinPath;
      return self.mBinPath.clone();
    }

    // java public final int getLocation()
    pub fn getLocation(&self) -> i32 {
      // java return this.mPicLocation;
      return self.mPicLocation;
    }

    // java public final boolean isEncrypt()
    pub fn isEncrypt(&self) -> bool {
      // java return this.mEncrypt;
      return self.mEncrypt;
    }

}

impl std::ops::Deref for DocOLEInfo {
    type Target = OleInfo;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for DocOLEInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}
