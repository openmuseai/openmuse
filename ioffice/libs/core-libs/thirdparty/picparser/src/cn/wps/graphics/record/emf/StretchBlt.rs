#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
// Java: public class StretchBlt extends EMFRecord
pub struct StretchBlt {
    // Java field: Rectangle rcBounds
    rcBounds: Option<i32>,
    // Java field: int xDest
    xDest: i32,
    // Java field: int yDest
    yDest: i32,
    // Java field: int cxDest
    cxDest: i32,
    // Java field: int cyDest
    cyDest: i32,
    // Java field: int rop
    rop: i32,
    // Java field: int xSrc
    xSrc: i32,
    // Java field: int ySrc
    ySrc: i32,
    // Java field: int cxSrc
    cxSrc: i32,
    // Java field: int cySrc
    cySrc: i32,
    // Java field: Matrix matrixSrc
    matrixSrc: Option<i32>,
    // Java field: int bkColorSrc
    bkColorSrc: i32,
    // Java field: int usageSrc
    usageSrc: i32,
    // Java field: PixelImage image
    image: Option<i32>,
}

impl StretchBlt {
    // Java: StretchBlt()
    pub fn new() -> Self {
        // Java: super();
        StretchBlt {
            rcBounds: None,
            xDest: 0,
            yDest: 0,
            cxDest: 0,
            cyDest: 0,
            rop: 0,
            xSrc: 0,
            ySrc: 0,
            cxSrc: 0,
            cySrc: 0,
            matrixSrc: None,
            bkColorSrc: 0,
            usageSrc: 0,
            image: None,
        }
    }

    // Java: public EMFRecord read(EMFParser emf, int size)
    pub fn read(&self, _emf: i32, _size: i32) -> i32 {
        // Java: int finalPos = emf.getCurrentPos() + size;
        // Java: int start = emf.getCurrentPos() - 8;
        // Java: StretchBlt blt = new StretchBlt();
        // Java: blt.rcBounds = emf.readRect();
        // Java: blt.xDest = emf.readInt();
        // Java: blt.yDest = emf.readInt();
        // Java: blt.cxDest = emf.readInt();
        // Java: blt.cyDest = emf.readInt();
        // Java: blt.rop = emf.readUnsignedInt();
        // Java: blt.xSrc = emf.readInt();
        // Java: blt.ySrc = emf.readInt();
        // Java: blt.matrixSrc = emf.readMatrix();
        // Java: blt.bkColorSrc = emf.readUnsignedInt();
        // Java: blt.usageSrc = emf.readUnsignedInt();
        // Java: int offBmi = emf.readInt();
        // Java: int cbBmi = emf.readInt();
        // Java: int offBits = emf.readInt();
        // Java: int cbBits = emf.readInt();
        // Java: blt.cxSrc = emf.readInt();
        // Java: blt.cySrc = emf.readInt();
        // Java: if (size == 100)
        {
        // Java: return blt;
        }
        // Java: else
        {
        // Java: emf.setCurrentPos(start + offBmi);
        // Java: EMFImageInfo info = EMFImageInfo.read(emf);
        // Java: blt.image = EMFImageParser.readImage(emf, info, info.width, info.height, start + offBits, size - (emf.getCurrentPos() - start + 8));
        // Java: if (blt.image != null)
        {
        // Java: blt.image.setRop(blt.rop);
        }
        // Java: emf.setCurrentPos(finalPos);
        // Java: return blt;
        }
        0
    }

    // Java: public void render(MetaFilePainter renderer)
    pub fn render(&self, _renderer: i32) {
        // Java: renderer.bitBlt(this.xDest, this.yDest, this.cxDest, this.cyDest, this.rop, this.image, this.matrixSrc);
    }

}
