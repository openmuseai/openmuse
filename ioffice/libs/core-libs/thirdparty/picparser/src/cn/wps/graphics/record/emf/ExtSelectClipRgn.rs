#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
// Java: public class ExtSelectClipRgn extends EMFRecord
pub struct ExtSelectClipRgn {
    // Java field: Rectangle rgn
    rgn: Option<i32>,
    // Java field: int mode
    mode: i32,
}

impl ExtSelectClipRgn {
    // Java: ExtSelectClipRgn()
    pub fn new() -> Self {
        // Java: super();
        ExtSelectClipRgn {
            rgn: None,
            mode: 0,
        }
    }

    // Java: public EMFRecord read(EMFParser emf, int size)
    pub fn read(&self, _emf: i32, _size: i32) -> i32 {
        // Java: int finalPos = emf.getCurrentPos() + size;
        // Java: ExtSelectClipRgn tag = new ExtSelectClipRgn();
        // Java: int length = emf.readUnsignedInt();
        // Java: tag.mode = emf.readUnsignedInt();
        // Java: if (length >= 20)
        {
        // Java: emf.skip(16);
        // Java: tag.rgn = emf.readRect();
        }
        // Java: emf.setCurrentPos(finalPos);
        // Java: return tag;
        0
    }

    // Java: public void render(MetaFilePainter renderer)
    pub fn render(&self, _renderer: i32) {
        // Java: renderer.clipPath(this.rgn, this.mode);
    }

}
