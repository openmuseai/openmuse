#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
// Java: public class AlphaBlend extends EMFRecord
pub struct AlphaBlend {
    // Java field: int x
    x: i32,
    // Java field: int y
    y: i32,
    // Java field: int width
    width: i32,
    // Java field: int height
    height: i32,
    // Java field: GdiImage image
    image: Option<i32>,
}

impl AlphaBlend {
    // Java: AlphaBlend()
    pub fn new() -> Self {
        // Java: super();
        AlphaBlend {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
            image: None,
        }
    }

    // Java: public EMFRecord read(EMFParser emf, int size)
    pub fn read(&self, _emf: i32, _size: i32) -> i32 {
        // Java: int finalPos = emf.getCurrentPos() + size;
        // Java: AlphaBlend tag = new AlphaBlend();
        // Java: EMFImageInfo info = new EMFImageInfo();
        // Java: emf.skip(16);
        // Java: tag.x = emf.readInt();
        // Java: tag.y = emf.readInt();
        // Java: tag.width = emf.readInt();
        // Java: tag.height = emf.readInt();
        // Java: info.readBlend(emf);
        // Java: emf.skip(44);
        // Java: int bmiSize = emf.readUnsignedInt();
        // Java: emf.skip(4);
        // Java: int bitmapSize = emf.readUnsignedInt();
        // Java: emf.skip(8);
        // Java: if (bmiSize > 0)
        {
        // Java: info.readInfo(emf);
        }
        // Java: tag.image = EMFImageParser.readImage(emf, info, info.width, info.height, bitmapSize);
        // Java: emf.setCurrentPos(finalPos);
        // Java: return tag;
        0
    }

    // Java: public void render(MetaFilePainter renderer)
    pub fn render(&self, _renderer: i32) {
        // Java: renderer.getDrawer().drawImage(this.image, this.x, this.y, this.width, this.height);
    }

}
