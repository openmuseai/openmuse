// Java: import cn.wps.imagetopdf.Image
use crate::cn::wps::imagetopdf::Image::Image;
// Java: import cn.wps.imagetopdf.MyStringBuilder
use crate::cn::wps::imagetopdf::MyStringBuilder::MyStringBuilder;

use super::ImageObj::ImageObj;

// Java: public class JPEGImg extends ImageObj
pub struct JPEGImg {
    pub(crate) base: ImageObj,
}

// Java: public class JPEGImg extends ImageObj
impl JPEGImg {
    // Java: JPEGImg(Image img)
    pub fn new(img: Image) -> Self {
        // Java: super(img);
        let mut base = ImageObj::new(Image::new(img.getW(), img.getH(), img.getData().to_vec()));
        // Java: MyStringBuilder sb = new MyStringBuilder(this.dict);
        let mut sb = MyStringBuilder::from_str(&base.base.base.dict);
        // Java: sb.app("/BitsPerComponent 8");
        sb.app_str("/BitsPerComponent 8");
        // Java: sb.app("/ColorSpace/DeviceRGB");
        sb.app_str("/ColorSpace/DeviceRGB");
        // Java: sb.app("/Filter/DCTDecode");
        sb.app_str("/Filter/DCTDecode");
        // Java: this.dict = sb.toString();
        base.base.base.dict = sb.toString();
        // Java: this.stream = img.getData();
        base.base.stream = img.getData().to_vec();
        // Java: this.compressed = false;
        base.base.compressed = false;
        JPEGImg { base }
    }
}
