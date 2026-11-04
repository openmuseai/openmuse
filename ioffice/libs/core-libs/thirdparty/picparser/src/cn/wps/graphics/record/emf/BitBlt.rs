#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
// Java: public class BitBlt extends EMFRecord
pub struct BitBlt {
    // Java field: int x
    x: i32,
    // Java field: int y
    y: i32,
    // Java field: int width
    width: i32,
    // Java field: int height
    height: i32,
    // Java field: Matrix transform
    transform: Option<i32>,
    // Java field: GdiImage image
    image: Option<i32>,
    // Java field: int rop
    rop: i32,
}

impl BitBlt {
    // Java: BitBlt()
    pub fn new() -> Self {
        // Java: super();
        BitBlt {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
            transform: None,
            image: None,
            rop: 0,
        }
    }

    // Java: public EMFRecord read(EMFParser emf, int size)
    pub fn read(&self, _emf: i32, _size: i32) -> i32 {
        // Java: BitBlt tag = new BitBlt();
        // Java: emf.skip(16);
        // Java: tag.x = emf.readInt();
        // Java: tag.y = emf.readInt();
        // Java: tag.width = emf.readInt();
        // Java: tag.height = emf.readInt();
        // Java: tag.rop = emf.readUnsignedInt();
        // Java: emf.skip(8);
        // Java: tag.transform = emf.readMatrix();
        // Java: emf.skip(12);
        // Java: int bmiSize = emf.readUnsignedInt();
        // Java: emf.skip(4);
        // Java: int bitmapSize = emf.readUnsignedInt();
        // Java: EMFImageInfo info = bmiSize > 0 ? EMFImageInfo.read(emf) : null;
        // Java: if (info != null)
        {
        // Java: tag.image = EMFImageParser.readImage(emf, info, tag.width, tag.height, bitmapSize);
        }
        // Java: return tag;
        0
    }

    // Java: public void render(MetaFilePainter renderer)
    pub fn render(&self, _renderer: i32) {
        // Java: renderer.bitBlt(this.x, this.y, this.width, this.height, this.rop, this.image, this.transform);
    }

}
