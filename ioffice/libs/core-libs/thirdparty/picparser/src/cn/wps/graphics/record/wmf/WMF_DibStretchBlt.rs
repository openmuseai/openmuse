#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
// Java: public class WMF_DibStretchBlt extends WMFRecord
pub struct WMF_DibStretchBlt {
    // Java field: int rop
    rop: i32,
    // Java field: int destHeight
    destHeight: i32,
    // Java field: int destWidth
    destWidth: i32,
    // Java field: int yDest
    yDest: i32,
    // Java field: int xDest
    xDest: i32,
    // Java field: GdiImage image
    image: Option<i32>,
}

impl WMF_DibStretchBlt {
    // Java: WMF_DibStretchBlt()
    pub fn new() -> Self {
        // Java: super();
        WMF_DibStretchBlt {
            rop: 0,
            destHeight: 0,
            destWidth: 0,
            yDest: 0,
            xDest: 0,
            image: None,
        }
    }

    // Java: public WMFRecord read(WMFParser wmf, int len)
    pub fn read(&self, _wmf: i32, _len: i32) -> i32 {
        // Java: WMF_DibStretchBlt tag = new WMF_DibStretchBlt();
        // Java: tag.rop = wmf.readUnsignedInt();
        // Java: wmf.skip(8);
        // Java: boolean isSpecified = len != 14;
        // Java: if (!isSpecified)
        {
        // Java: wmf.skip(2);
        }
        // Java: tag.destHeight = wmf.readShort();
        // Java: tag.destWidth = wmf.readShort();
        // Java: tag.yDest = wmf.readShort();
        // Java: tag.xDest = wmf.readShort();
        // Java: if (isSpecified)
        {
        // Java: try
        {
        // Java: WMF_DibObject dibObject = new WMF_DibObject(len - 20, wmf);
        // Java: tag.image = dibObject.getImage();
        // Java: if (tag.image != null)
        {
        // Java: tag.image.setRop(tag.rop);
        }
        }
        // Java: catch (ArrayIndexOutOfBoundsException e)
        {
        // Java: e.printStackTrace();
        }
        }
        // Java: return tag;
        0
    }

    // Java: public void render(MetaFilePainter renderer)
    pub fn render(&self, _renderer: i32) {
        // Java: renderer.getAffectCharsetSet().add(WMF_DibStretchBlt.class.getSimpleName());
        // Java: renderer.getDrawer().drawImage(this.image, this.xDest, this.yDest, this.destWidth, this.destHeight, true);
    }

}
