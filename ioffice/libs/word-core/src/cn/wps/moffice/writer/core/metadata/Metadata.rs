// Java: import cn.wps.io.opc.CustomXmlInfo
use io_wps_io::io::opc::CustomXmlInfo::*;
// Java: import java.util.ArrayList
// Java: import cn.wps.moffice.writer.core.metadata.CustomMetadata
use crate::cn::wps::moffice::writer::core::metadata::CustomMetadata::*;
// Java: import cn.wps.moffice.writer.core.metadata.PIDSI
use crate::cn::wps::moffice::writer::core::metadata::PIDSI::*;
// Java: import cn.wps.moffice.writer.core.metadata.PIDDSI
use crate::cn::wps::moffice::writer::core::metadata::PIDDSI::*;

// java public class Metadata
#[derive(Default)]
pub struct Metadata {
    // java private CustomMetadata mCustomMetadata = null;
    mCustomMetadata: Option<CustomMetadata>,
    // java private PIDSI mPidsi = null;
    mPidsi: Option<PIDSI>,
    // java private PIDDSI mPiddsi = null;
    mPiddsi: Option<PIDDSI>,
    // java private ArrayList<CustomXmlInfo> mCustomXmlInfos;
    mCustomXmlInfos: Option<Vec<CustomXmlInfo>>,
}

// java public class Metadata
impl Metadata {
    // java public void dispose()
    pub fn dispose(&mut self) {
      // java if (this.mCustomMetadata != null)
      // rust: Java `x.dispose(); x = null;` ⇒ 先 `take()` 取出再 dispose。
      if let Some(mut mCustomMetadata) = self.mCustomMetadata.take() {
        // java this.mCustomMetadata.dispose();
        mCustomMetadata.dispose();
      }
      // java if (this.mPidsi != null)
      if let Some(mut mPidsi) = self.mPidsi.take() {
        // java this.mPidsi.dispose();
        mPidsi.dispose();
      }
      // java if (this.mPiddsi != null)
      if let Some(mut mPiddsi) = self.mPiddsi.take() {
        // java this.mPiddsi.dispose();
        mPiddsi.dispose();
      }
    }

    // java public boolean hasData()
    pub fn hasData(&self) -> bool {
      // java return this.mPiddsi != null && this.mPiddsi.hasData() || this.mPidsi != null && this.mPidsi.hasData() || this.mCustomMetadata != null && this.mCustomMetadata.hasData();
      return self.mPiddsi.as_ref().map_or(false, |m| m.hasData())
          || self.mPidsi.as_ref().map_or(false, |m| m.hasData())
          || self.mCustomMetadata.as_ref().map_or(false, |m| m.hasData());
    }

    // java public void setCustomXmlInfos(ArrayList<CustomXmlInfo> infos)
    pub fn setCustomXmlInfos(&mut self, infos: Vec<CustomXmlInfo>) {
      // java this.mCustomXmlInfos = infos;
      self.mCustomXmlInfos = Some(infos);
    }

    // java public ArrayList<CustomXmlInfo> getCustomXmlInfos()
    pub fn getCustomXmlInfos(&self) -> &[CustomXmlInfo] {
      // java return this.mCustomXmlInfos;
      // rust: `CustomXmlInfo` 未派生 `Clone` ⇒ 以切片借用返回
      //   （Java 返回 List 引用，语义一致）。
      return self.mCustomXmlInfos.as_deref().unwrap_or(&[]);
    }

    // java public CustomMetadata getCustomMetadata()
    pub fn getCustomMetadata(&mut self) -> CustomMetadata {
      // java if (this.mCustomMetadata == null)
      if self.mCustomMetadata.is_none() {
        // java this.mCustomMetadata = new CustomMetadata();
        self.mCustomMetadata = Some(CustomMetadata::default());
      }
      // java return this.mCustomMetadata;
      return self.mCustomMetadata.clone().unwrap_or_default();
    }

    // java public PIDSI getPIDSI()
    pub fn getPIDSI(&mut self) -> PIDSI {
      // java if (this.mPidsi == null)
      if self.mPidsi.is_none() {
        // java this.mPidsi = new PIDSI();
        self.mPidsi = Some(PIDSI::default());
      }
      // java return this.mPidsi;
      return self.mPidsi.clone().unwrap_or_default();
    }

    // java public PIDDSI getPIDDSI()
    pub fn getPIDDSI(&mut self) -> PIDDSI {
      // java if (this.mPiddsi == null)
      if self.mPiddsi.is_none() {
        // java this.mPiddsi = new PIDDSI();
        self.mPiddsi = Some(PIDDSI::default());
      }
      // java return this.mPiddsi;
      return self.mPiddsi.clone().unwrap_or_default();
    }

}
