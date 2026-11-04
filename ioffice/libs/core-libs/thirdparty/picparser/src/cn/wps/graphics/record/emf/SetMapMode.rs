#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
// Java: public class SetMapMode extends EMFRecord
pub struct SetMapMode {
    // Java field: int mapMode
    mapMode: i32,
}

impl SetMapMode {
    // Java: SetMapMode()
    pub fn new() -> Self {
        // Java: super();
        SetMapMode {
            mapMode: 0,
        }
    }

    // Java: public EMFRecord read(EMFParser emf, int size)
    pub fn read(&self, _emf: i32, _size: i32) -> i32 {
        // Java: SetMapMode tag = new SetMapMode();
        // Java: tag.mapMode = emf.readUnsignedInt();
        // Java: return tag;
        0
    }

    // Java: public void render(MetaFilePainter renderer)
    pub fn render(&self, _renderer: i32) {
        // Java: renderer.getContext().setMapMode(this.mapMode);
    }

}
