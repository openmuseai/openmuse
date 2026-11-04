#![allow(non_snake_case, dead_code)]

// Java: import cn.wps.moffice.drawing.media.IMediaLib
use super::media::IMediaLib::IMediaLib;
// Java: import cn.wps.moffice.drawing.media.MediaTypeEnum
use super::media::MediaTypeEnum::MediaTypeEnum;

use super::PropBase::PropBase;
use std::sync::Arc;
use std::any::Any;

type Element = Option<Arc<dyn Any + Send + Sync>>;

// Java field: long serialVersionUID
pub(crate) const serialVersionUID: i64 = 1;

// Java: public class OLE extends PropBase
pub struct OLE {
    pub base: PropBase,
    // Java field: IMediaLib mMediaLib
    pub(crate) mMediaLib: Option<Arc<dyn IMediaLib + Send + Sync>>,
}

// Java: public class OLE extends PropBase
impl OLE {
    // Java: OLE clone()
    pub fn clone(&mut self) -> OLE {
      // Java: return (OLE)super.clone();
      OLE { base: self.base.clone(), mMediaLib: self.mMediaLib.clone() }
    }

    // Java: OLE(int mediaid, IMediaLib mediaLib)
    pub fn new_with_media(mediaid: i32, mediaLib: Arc<dyn IMediaLib + Send + Sync>) -> Self {
      // Java: super(2);
      let mut ole = OLE { base: PropBase::new_with_capacity(2), mMediaLib: Some(mediaLib) };
      // Java: this.setMediaID(mediaid);
      ole.setMediaID(mediaid);
      // Java: this.mMediaLib = mediaLib;
      ole
    }

    // Java: OLE(int mediaID)
    pub fn new_with_id(mediaID: i32) -> Self {
      // Java: super(2);
      let mut ole = OLE { base: PropBase::new_with_capacity(2), mMediaLib: None };
      // Java: this.setMediaID(mediaID);
      ole.setMediaID(mediaID);
      ole
    }

    // Java: void setMediaID(int id)
    pub fn setMediaID(&mut self, id: i32) {
      // Java: this.mProperty.setInt(471, id);
      self.base.mProperty.as_mut().unwrap().setInt(471, id);
    }

    // Java: int getMediaID()
    pub fn getMediaID(&self) -> i32 {
      // Java: return this.mProperty.getInt(471, -1);
      self.base.mProperty.as_ref().unwrap().base.getInt_default(471, -1)
    }

    // Java: String getProgId()
    pub fn getProgId(&self) -> String {
      // Java: return this.mProperty.getString(867, (String)null);
      self.base.mProperty.as_ref().unwrap().base.getString_default(867, "")
    }

    // Java: void setProgId(String progId)
    pub fn setProgId(&mut self, progId: String) {
      // Java: this.mProperty.setString(867, progId);
      self.base.mProperty.as_mut().unwrap().setString(867, progId);
    }

    // Java: void setVbaCLSID(String clsid)
    pub fn setVbaCLSID(&mut self, clsid: String) {
      // Java: this.mProperty.setString(864, clsid);
      self.base.mProperty.as_mut().unwrap().setString(864, clsid);
    }

    // Java: String getVbaCLSID()
    pub fn getVbaCLSID(&self) -> String {
      // Java: return this.mProperty.getString(864, (String)null);
      self.base.mProperty.as_ref().unwrap().base.getString_default(864, "")
    }

    // Java: String getVbaPersistence()
    pub fn getVbaPersistence(&self) -> String {
      // Java: return this.mProperty.getString(865, (String)null);
      self.base.mProperty.as_ref().unwrap().base.getString_default(865, "")
    }

    // Java: void setVbaPersistence(String persistence)
    pub fn setVbaPersistence(&mut self, persistence: String) {
      // Java: this.mProperty.setString(865, persistence);
      self.base.mProperty.as_mut().unwrap().setString(865, persistence);
    }

    // Java: boolean isVba()
    pub fn isVba(&self) -> bool {
      // Java: return this.mProperty.getBoolean(866, false);
      self.base.mProperty.as_ref().unwrap().base.getBoolean_default(866, false)
    }

    // Java: void setIsVba(boolean isVba)
    pub fn setIsVba(&mut self, isVba: bool) {
      // Java: this.mProperty.setBoolean(866, isVba);
      self.base.mProperty.as_mut().unwrap().setBoolean(866, isVba);
    }

    // Java: boolean getIsIcon()
    pub fn getIsIcon(&self) -> bool {
      // Java: return this.mProperty.getBoolean(868, false);
      self.base.mProperty.as_ref().unwrap().base.getBoolean_default(868, false)
    }

    // Java: void setIsIcon(boolean isIcon)
    pub fn setIsIcon(&mut self, isIcon: bool) {
      // Java: this.mProperty.setBoolean(868, isIcon);
      self.base.mProperty.as_mut().unwrap().setBoolean(868, isIcon);
    }

    // Java: void setCLSID(String clsid)
    pub fn setCLSID(&mut self, _clsid: String) {
    }

    // Java: String getCLSID()
    pub fn getCLSID(&self) -> String {
      // Java: return null;
      String::new()
    }

    // Java: String getStoragePath()
    pub fn getStoragePath(&self) -> String {
      // Java: if (null == this.mMediaLib)
      if self.mMediaLib.is_none()
      {
         // Java: return null;
         return String::new();
      }
      // Java: else
      else
      {
         // Java: int mediaId = this.getMediaID();
         let mediaId = self.getMediaID();
         // Java: return this.mMediaLib.getMediaByID(mediaId, MediaTypeEnum.OLE);
         self.mMediaLib.as_ref().unwrap().getMediaByID(mediaId, MediaTypeEnum::OLE)
      }
    }

}
