#![allow(non_snake_case, dead_code, unused_variables, unused_imports)]

// Java: import cn.wps.graphics.PictureInfo

use crate::cn::wps::graphics::data::emf::EMFHeader::EMFHeader;

pub struct PictureInfo {
    pub r#type: i32,
    pub width: i32,
    pub height: i32,
    pub path: String,
    pub byteLen: i32,
}
impl PictureInfo {
    pub fn new(path: String, pic_type: i32, width: i32, height: i32, byteLen: i32) -> Self {
        PictureInfo { r#type: pic_type, width, height, path, byteLen }
    }
}

// Java: public class EMFPictureInfo extends PictureInfo
pub struct EMFPictureInfo {
    // Java field: EMFHeader mHeader
    pub(crate) mHeader: Option<EMFHeader>,
    pub base: PictureInfo,
}

// Java: public class EMFPictureInfo extends PictureInfo
impl EMFPictureInfo {
    // Java: EMFPictureInfo(String path, int type, EMFHeader header, int byteLen)
    pub fn new(path: String, pic_type: i32, header: EMFHeader, byteLen: i32) -> Self {
        // Java: super(path, type, (int)header.getWidth(), (int)header.getHeight(), byteLen);
        let mut base = PictureInfo::new(path, pic_type, header.getWidth() as i32, header.getHeight() as i32, byteLen);
        // Java: float rw = (float)header.getRcFrameWidth();
        let rw = header.getRcFrameWidth() as f32;
        // Java: float rh = (float)header.getRcFrameHeight();
        let rh = header.getRcFrameHeight() as f32;
        // Java: if (rw > 0.0F && rh > 0.0F)
        if rw > 0.0 && rh > 0.0 {
            // Java: if (header.isEmfPlus())
            if header.isEmfPlus() {
                // Java: float dpiX = (float)header.getDpiX();
                let dpiX = header.getDpiX() as f32;
                // Java: float dpiY = (float)header.getDpiY();
                let dpiY = header.getDpiY() as f32;
                // Java: if (dpiX > 0.0F && dpiY > 0.0F)
                if dpiX > 0.0 && dpiY > 0.0 {
                    // Java: this.width = (int)Math.ceil((double)(dpiX * rw * 0.01F / 25.4F));
                    base.width = ((dpiX * rw * 0.01 / 25.4) as f64).ceil() as i32;
                    // Java: this.height = (int)Math.ceil((double)(dpiY * rh * 0.01F / 25.4F));
                    base.height = ((dpiY * rh * 0.01 / 25.4) as f64).ceil() as i32;
                }
            }
            // Java: else
            else {
                // Java: float xdpmm = header.getxdpmm();
                let xdpmm = header.getxdpmm();
                // Java: float ydpmm = header.getydpmm();
                let ydpmm = header.getydpmm();
                // Java: if (xdpmm > 0.0F && ydpmm > 0.0F)
                if xdpmm > 0.0 && ydpmm > 0.0 {
                    // Java: this.width = (int)Math.ceil((double)(rw * 0.01F * xdpmm));
                    base.width = ((rw * 0.01 * xdpmm) as f64).ceil() as i32;
                    // Java: this.height = (int)Math.ceil((double)(rh * 0.01F * ydpmm));
                    base.height = ((rh * 0.01 * ydpmm) as f64).ceil() as i32;
                }
            }
        }
        // Java: this.mHeader = header;
        EMFPictureInfo { mHeader: Some(header), base }
    }
}
