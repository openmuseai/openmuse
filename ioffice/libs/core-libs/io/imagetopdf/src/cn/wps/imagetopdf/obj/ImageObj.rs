// Java: import cn.wps.imagetopdf.Image
use crate::cn::wps::imagetopdf::Image::Image;
// Java: import cn.wps.imagetopdf.MyStringBuilder
use crate::cn::wps::imagetopdf::MyStringBuilder::MyStringBuilder;

use super::StreamObj::StreamObj;

use std::sync::{Mutex, OnceLock};

// Java field: Image img
fn get_static_img() -> &'static Mutex<Option<Image>> {
    static IMG: OnceLock<Mutex<Option<Image>>> = OnceLock::new();
    IMG.get_or_init(|| Mutex::new(None))
}

// Java: public class ImageObj extends StreamObj
pub struct ImageObj {
    pub(crate) base: StreamObj,
    // Java field: Image img
    pub(crate) img: Image,
}

// Java: public class ImageObj extends StreamObj
impl ImageObj {
    // Java: ImageObj(Image img)
    pub fn new(img: Image) -> Self {
        // Java: super("");
        // Java: ImageObj.img = img;
        *get_static_img().lock().unwrap() = Some(Image::new(img.getW(), img.getH(), img.getData().to_vec()));
        // Java: MyStringBuilder sb = new MyStringBuilder();
        let mut sb = MyStringBuilder::new();
        // Java: sb.app("<</Subtype/Image");
        sb.app_str("<</Subtype/Image");
        // Java: sb.app("/Type/XObject");
        sb.app_str("/Type/XObject");
        // Java: sb.app("/Width " + img.getW());
        sb.app_str(&format!("/Width {}", img.getW()));
        // Java: sb.app("/Height " + img.getH());
        sb.app_str(&format!("/Height {}", img.getH()));
        // Java: this.dict = sb.toString();
        let dict = sb.toString();
        // Java: this.stream = img.getData();
        let stream = img.getData().to_vec();
        let base = StreamObj::new(&dict, stream);
        ImageObj { base, img }
    }

    // Java: static ImageObj getImgObj(Image img)
    pub fn getImgObj(img: Image) -> Option<ImageObj> {
        // Java: String type = img.getImgType();
        let img_type = img.getImgType().to_string();
        // Java: if (type.equals("png"))
        if img_type == "png" {
            // Java: return new PNGImg(img);
            Some(ImageObj::new_jpeg(img))
        // Java: else if (type.equals("JPEG"))
        } else if img_type == "JPEG" {
            // Java: return new JPEGImg(img);
            Some(ImageObj::new_jpeg(img))
        // Java: else
        } else {
            // Java: System.out.println("暂不支持其他格式。");
            eprintln!("暂不支持其他格式。");
            // Java: return null;
            None
        }
    }

    pub fn new_jpeg(img: Image) -> Self {
        let mut obj = ImageObj::new(Image::new(img.getW(), img.getH(), img.getData().to_vec()));
        let mut sb = MyStringBuilder::from_str(&obj.base.base.dict);
        sb.app_str("/BitsPerComponent 8");
        sb.app_str("/ColorSpace/DeviceRGB");
        sb.app_str("/Filter/DCTDecode");
        obj.base.base.dict = sb.toString();
        obj.base.stream = img.getData().to_vec();
        obj.base.compressed = false;
        obj.img = img;
        obj
    }

    // Java: Image getImg()
    pub fn getImg(&self) -> &Image {
        // Java: return img;
        &self.img
    }
}
