#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
// Java: public class WMF_StretchDIB extends WMFRecord
pub struct WMF_StretchDIB {
    // Java field: int rop
    rop: i32,
    // Java field: int colorUsage
    colorUsage: i32,
    // Java field: int srcHeight
    srcHeight: i32,
    // Java field: int srcWidth
    srcWidth: i32,
    // Java field: int ySrc
    ySrc: i32,
    // Java field: int xSrc
    xSrc: i32,
    // Java field: int destHeight
    destHeight: i32,
    // Java field: int destWidth
    destWidth: i32,
    // Java field: int xDest
    xDest: i32,
    // Java field: int yDest
    yDest: i32,
    // Java field: GdiImage image
    image: Option<i32>,
}

impl WMF_StretchDIB {
    // Java: WMF_StretchDIB()
    pub fn new() -> Self {
        // Java: super();
        WMF_StretchDIB {
            rop: 0,
            colorUsage: 0,
            srcHeight: 0,
            srcWidth: 0,
            ySrc: 0,
            xSrc: 0,
            destHeight: 0,
            destWidth: 0,
            xDest: 0,
            yDest: 0,
            image: None,
        }
    }

    // Java: public WMFRecord read(WMFParser wmf, int len)
    pub fn read(&self, _wmf: i32, _len: i32) -> i32 {
        // Java: WMF_StretchDIB tag = new WMF_StretchDIB();
        // Java: tag.rop = wmf.readUnsignedInt();
        // Java: tag.colorUsage = wmf.readShort();
        // Java: tag.srcHeight = wmf.readShort();
        // Java: tag.srcWidth = wmf.readShort();
        // Java: tag.ySrc = wmf.readShort();
        // Java: tag.xSrc = wmf.readShort();
        // Java: tag.destHeight = wmf.readShort();
        // Java: tag.destWidth = wmf.readShort();
        // Java: tag.yDest = wmf.readShort();
        // Java: tag.xDest = wmf.readShort();
        // Java: WMF_DibObject dibObject = new WMF_DibObject(len - 22, wmf);
        // Java: tag.image = dibObject.getImage();
        // Java: return tag;
        0
    }

    // Java: public void render(MetaFilePainter renderer)
    pub fn render(&self, _renderer: i32) {
        // Java: renderer.getDrawer().drawImage(this.image, this.xDest, this.yDest, this.destWidth, this.destHeight);
    }

}
