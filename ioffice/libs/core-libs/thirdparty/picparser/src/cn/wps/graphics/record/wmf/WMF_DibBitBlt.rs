#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
// Java: public class WMF_DibBitBlt extends WMFRecord
pub struct WMF_DibBitBlt {
    // Java field: int rop
    rop: i32,
    // Java field: int height
    height: i32,
    // Java field: int width
    width: i32,
    // Java field: int yDest
    yDest: i32,
    // Java field: int xDest
    xDest: i32,
    // Java field: GdiImage image
    image: Option<i32>,
}

impl WMF_DibBitBlt {
    // Java: WMF_DibBitBlt()
    pub fn new() -> Self {
        // Java: super();
        WMF_DibBitBlt {
            rop: 0,
            height: 0,
            width: 0,
            yDest: 0,
            xDest: 0,
            image: None,
        }
    }

    // Java: public WMFRecord read(WMFParser wmf, int len)
    pub fn read(&self, _wmf: i32, _len: i32) -> i32 {
        // Java: WMF_DibBitBlt dibBitBlt = new WMF_DibBitBlt();
        // Java: dibBitBlt.rop = wmf.readUnsignedInt();
        // Java: wmf.skip(4);
        // Java: boolean noBitmap = (len + 6) / 2 == 12;
        // Java: if (noBitmap)
        {
        // Java: wmf.skip(2);
        }
        // Java: dibBitBlt.height = wmf.readShort();
        // Java: dibBitBlt.width = wmf.readShort();
        // Java: dibBitBlt.yDest = wmf.readShort();
        // Java: dibBitBlt.xDest = wmf.readShort();
        // Java: if (!noBitmap)
        {
        // Java: WMF_DibObject dib = new WMF_DibObject(len - 16, wmf);
        // Java: dibBitBlt.image = dib.getImage();
        }
        // Java: return dibBitBlt;
        0
    }

    // Java: public void render(MetaFilePainter renderer)
    pub fn render(&self, _renderer: i32) {
        // Java: renderer.bitBlt(this.xDest, this.yDest, this.width, this.height, this.rop, this.image, (Matrix)null);
    }

}
