#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
// Java: public class Wmf_SetMapMode extends WMFRecord
pub struct Wmf_SetMapMode {
    // Java field: int mapMode
    mapMode: i32,
}

impl Wmf_SetMapMode {
    // Java: Wmf_SetMapMode()
    pub fn new() -> Self {
        // Java: super();
        Wmf_SetMapMode {
            mapMode: 0,
        }
    }

    // Java: public WMFRecord read(WMFParser wmf, int len)
    pub fn read(&self, _wmf: i32, _len: i32) -> i32 {
        // Java: Wmf_SetMapMode tag = new Wmf_SetMapMode();
        // Java: tag.mapMode = wmf.readUnsignedShort();
        // Java: return tag;
        0
    }

    // Java: public void render(MetaFilePainter renderer)
    pub fn render(&self, _renderer: i32) {
        // Java: renderer.getContext().setMapMode(this.mapMode);
    }

    // Java: public int getMapMode()
    pub fn getMapMode(&self) -> i32 {
        // Java: return this.mapMode;
        0
    }

}
