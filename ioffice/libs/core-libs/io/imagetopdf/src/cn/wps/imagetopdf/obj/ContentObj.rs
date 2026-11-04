// Java: import cn.wps.imagetopdf.Image
use crate::cn::wps::imagetopdf::Image::Image;
// Java: import cn.wps.imagetopdf.MyStringBuilder
use crate::cn::wps::imagetopdf::MyStringBuilder::MyStringBuilder;

use super::StreamObj::StreamObj;

// Java: public class ContentObj extends StreamObj
pub struct ContentObj {
    pub(crate) base: StreamObj,
}

// Java: public class ContentObj extends StreamObj
impl ContentObj {
    // Java: ContentObj(Image img)
    pub fn new(img: &Image) -> Self {
        // Java: super("", new byte[0]);
        let mut base = StreamObj::new("", Vec::new());
        // Java: MyStringBuilder sb = new MyStringBuilder();
        let mut sb = MyStringBuilder::new();
        // Java: sb.app("q");
        sb.app_str("q");
        // Java: sb.app("q " + img.getW() + " 0 0 " + img.getH() + " 0 0 cm /I0 Do Q");
        sb.app_str(&format!("q {} 0 0 {} 0 0 cm /I0 Do Q", img.getW(), img.getH()));
        // Java: sb.appn("Q");
        sb.appn("Q");
        // Java: this.stream = sb.getBytes();
        base.stream = sb.getBytes();
        // Java: sb = new MyStringBuilder("<<");
        let sb2 = MyStringBuilder::from_str("<<");
        // Java: this.dict = sb.toString();
        base.base.dict = sb2.toString();
        ContentObj { base }
    }
}
