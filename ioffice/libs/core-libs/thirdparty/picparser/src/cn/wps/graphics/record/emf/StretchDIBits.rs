#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
// Java: public class StretchDIBits extends EMFRecord
pub struct StretchDIBits {
    // Java field: Rectangle bounds
    bounds: Option<i32>,
    // Java field: int xDest
    xDest: i32,
    // Java field: int yDest
    yDest: i32,
    // Java field: int widthDest
    widthDest: i32,
    // Java field: int heightDest
    heightDest: i32,
    // Java field: int xSrc
    xSrc: i32,
    // Java field: int ySrc
    ySrc: i32,
    // Java field: int widthSrc
    widthSrc: i32,
    // Java field: int heightSrc
    heightSrc: i32,
    // Java field: int offBmiSrc
    offBmiSrc: i32,
    // Java field: int cbBmiSrc
    cbBmiSrc: i32,
    // Java field: int offBitsSrc
    offBitsSrc: i32,
    // Java field: int cbBitsSrc
    cbBitsSrc: i32,
    // Java field: int usageSrc
    usageSrc: i32,
    // Java field: int rop
    rop: i32,
    // Java field: EMFImageInfo info
    info: Option<i32>,
    // Java field: GdiImage image
    image: Option<i32>,
}

impl StretchDIBits {
    // Java: StretchDIBits()
    pub fn new() -> Self {
        // Java: super();
        StretchDIBits {
            bounds: None,
            xDest: 0,
            yDest: 0,
            widthDest: 0,
            heightDest: 0,
            xSrc: 0,
            ySrc: 0,
            widthSrc: 0,
            heightSrc: 0,
            offBmiSrc: 0,
            cbBmiSrc: 0,
            offBitsSrc: 0,
            cbBitsSrc: 0,
            usageSrc: 0,
            rop: 0,
            info: None,
            image: None,
        }
    }

    // Java: public EMFRecord read(EMFParser emf, int size)
    pub fn read(&self, _emf: i32, _size: i32) -> i32 {
        // Java: int start = emf.getCurrentPos() - 8;
        // Java: StretchDIBits tag = new StretchDIBits();
        // Java: tag.bounds = emf.readRect();
        // Java: tag.xDest = emf.readInt();
        // Java: tag.yDest = emf.readInt();
        // Java: tag.xSrc = emf.readInt();
        // Java: tag.ySrc = emf.readInt();
        // Java: tag.widthDest = emf.readInt();
        // Java: tag.heightDest = emf.readInt();
        // Java: tag.offBmiSrc = emf.readUnsignedInt();
        // Java: tag.cbBmiSrc = emf.readUnsignedInt();
        // Java: tag.offBitsSrc = emf.readUnsignedInt();
        // Java: tag.cbBitsSrc = emf.readUnsignedInt();
        // Java: tag.usageSrc = emf.readUnsignedInt();
        // Java: tag.rop = emf.readUnsignedInt();
        // Java: tag.widthSrc = emf.readInt();
        // Java: tag.heightSrc = emf.readInt();
        // Java: emf.setCurrentPos(start + tag.offBmiSrc);
        // Java: tag.info = EMFImageInfo.read(emf);
        // Java: tag.image = EMFImageParser.readImage(emf, tag.info, tag.widthDest, tag.heightDest, start + tag.offBitsSrc, size - (emf.getCurrentPos() - start - 8));
        // Java: if (tag.image != null)
        {
        // Java: tag.image.setRop(tag.rop);
        }
        // Java: return tag;
        0
    }

    // Java: public void render(MetaFilePainter renderer)
    pub fn render(&self, _renderer: i32) {
        // Java: if (this.image != null && this.image.getRop() == 15597702)
        {
        // Java: MetaFileRecord next = renderer.getNextRecord(this);
        // Java: if (next != null && next instanceof StretchDIBits)
        {
        // Java: StretchDIBits stretchDIBits = (StretchDIBits)next;
        // Java: if (stretchDIBits.image.getRop() == 8913094)
        {
        // Java: return;
        }
        }
        }
        // Java: renderer.getDrawer().drawImage(this.image, this.xDest, this.yDest, this.widthSrc, this.heightSrc);
    }

}
