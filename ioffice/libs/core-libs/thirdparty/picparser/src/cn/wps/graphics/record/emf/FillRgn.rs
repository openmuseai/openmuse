#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
// Java: public class FillRgn extends EMFRecord
pub struct FillRgn {
    // Java field: Rectangle bounds
    bounds: Option<i32>,
    // Java field: int rgnDataSize
    rgnDataSize: i32,
    // Java field: int ihBrush
    ihBrush: i32,
    // Java field: Vector<RgnData> rgnData
    rgnData: Option<i32>,
}

impl FillRgn {
    // Java: FillRgn()
    pub fn new() -> Self {
        // Java: super();
        FillRgn {
            bounds: None,
            rgnDataSize: 0,
            ihBrush: 0,
            rgnData: None,
        }
    }

    // Java: public EMFRecord read(EMFParser emf, int size)
    pub fn read(&self, _emf: i32, _size: i32) -> i32 {
        // Java: FillRgn tag = new FillRgn();
        // Java: tag.bounds = emf.readRect();
        // Java: tag.rgnDataSize = emf.readUnsignedInt();
        // Java: tag.ihBrush = emf.readUnsignedInt();
        // Java: tag.rgnData = new Vector();
        // Java: tag.rgnData.add(readRgnData(emf));
        // Java: return tag;
        0
    }

    // Java: public void render(MetaFilePainter renderer)
    pub fn render(&self, _renderer: i32) {
        // Java: for(RgnData data : this.rgnData)
        {
        // Java: renderer.fillRects(data.rects);
        }
    }

    // Java: private static RgnData readRgnData(EMFParser emf)
    pub fn readRgnData(_emf: i32) -> i32 {
        // Java: int size = emf.readUnsignedInt();
        // Java: int type = emf.readUnsignedInt();
        // Java: int countRect = emf.readUnsignedInt();
        // Java: int rgnSize = emf.readUnsignedInt();
        // Java: Rectangle bounds = emf.readRect();
        // Java: Vector<Rectangle> rects = new Vector();
        // Java: for(int i = 0; i < countRect; ++i)
        {
        // Java: rects.add(emf.readRect());
        }
        // Java: return new RgnData(bounds, rects);
        0
    }

}
