#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
// Java: public class CreateDIBPatternBrushPt extends EMFRecord
pub struct CreateDIBPatternBrushPt {
    // Java field: EMFImageInfo info
    info: Option<i32>,
    // Java field: GdiImage image
    image: Option<i32>,
    // Java field: int index
    index: i32,
}

impl CreateDIBPatternBrushPt {
    // Java: CreateDIBPatternBrushPt()
    pub fn new() -> Self {
        // Java: super();
        CreateDIBPatternBrushPt {
            info: None,
            image: None,
            index: 0,
        }
    }

    // Java: public EMFRecord read(EMFParser emf, int size)
    pub fn read(&self, _emf: i32, _size: i32) -> i32 {
        // Java: CreateDIBPatternBrushPt tag = new CreateDIBPatternBrushPt();
        // Java: int start = emf.getCurrentPos() - 8;
        // Java: tag.index = emf.readUnsignedInt();
        // Java: int usage = emf.readInt();
        // Java: int offBmi = emf.readInt();
        // Java: int cbBmi = emf.readInt();
        // Java: int offBits = emf.readInt();
        // Java: int cbBits = emf.readInt();
        // Java: emf.setCurrentPos(start + offBmi);
        // Java: tag.info = EMFImageInfo.read(emf);
        // Java: tag.image = EMFImageParser.readImage(emf, tag.info, tag.info.width, tag.info.height, start + offBits, size - (emf.getCurrentPos() - start + 8));
        // Java: return tag;
        0
    }

    // Java: public void render(MetaFilePainter renderer)
    pub fn render(&self, _renderer: i32) {
        // Java: BitGdiPaint paint = new BitGdiPaint(this.image);
        // Java: renderer.getGdiObjects().setObject(this.index, paint);
    }

}
