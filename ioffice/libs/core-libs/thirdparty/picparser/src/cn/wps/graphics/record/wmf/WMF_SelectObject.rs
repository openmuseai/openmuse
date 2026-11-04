#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
// Java: public class WMF_SelectObject extends WMFRecord
pub struct WMF_SelectObject {
    // Java field: int index
    index: i32,
}

impl WMF_SelectObject {
    // Java: WMF_SelectObject()
    pub fn new() -> Self {
        // Java: super();
        WMF_SelectObject {
            index: 0,
        }
    }

    // Java: public WMFRecord read(WMFParser wmf, int len)
    pub fn read(&self, _wmf: i32, _len: i32) -> i32 {
        // Java: WMF_SelectObject tag = new WMF_SelectObject();
        // Java: tag.index = wmf.readUnsignedShort();
        // Java: return tag;
        0
    }

    // Java: public void render(MetaFilePainter renderer)
    pub fn render(&self, _renderer: i32) {
        // Java: renderer.selectObject(this.index);
    }

    // Java: public String toString()
    pub fn toString(&self) -> String {
        // Java: return "SelectObject";
        String::new()
    }

}
