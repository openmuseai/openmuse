#![allow(non_snake_case, dead_code, unused_variables)]

// Java: import cn.wps.graphics.PictureInfo

pub struct PictureInfo {
    pub pic_type: i32,
    pub width: i32,
    pub height: i32,
    pub path: String,
    pub byteLen: i32,
}

impl PictureInfo {
    pub fn new(path: String, pic_type: i32, width: i32, height: i32, length: i32) -> Self {
        PictureInfo { pic_type, width, height, path, byteLen: length }
    }
}

// Java: public class WMFPictureInfo extends PictureInfo
pub struct WMFPictureInfo {
    pub base: PictureInfo,
    // Java field: PictureInfo mEmbedInfo
    pub(crate) mEmbedInfo: Option<PictureInfo>,
}

// Java: public class WMFPictureInfo extends PictureInfo
impl WMFPictureInfo {
    // Java: WMFPictureInfo(String path, int type, int originWidth, int originHeight, int length)
    pub fn new(path: String, pic_type: i32, originWidth: i32, originHeight: i32, length: i32) -> Self {
        // Java: super(path, type, originWidth, originHeight, length);
        WMFPictureInfo {
            base: PictureInfo::new(path, pic_type, originWidth, originHeight, length),
            mEmbedInfo: None,
        }
    }

    // Java: PictureInfo getEmbedInfo()
    pub fn getEmbedInfo(&self) -> &Option<PictureInfo> {
        // Java: return this.mEmbedInfo;
        &self.mEmbedInfo
    }

    // Java: void setEmbedInfo(PictureInfo info)
    pub fn setEmbedInfo(&mut self, info: Option<PictureInfo>) {
        // Java: this.mEmbedInfo = info;
        self.mEmbedInfo = info;
    }
}
