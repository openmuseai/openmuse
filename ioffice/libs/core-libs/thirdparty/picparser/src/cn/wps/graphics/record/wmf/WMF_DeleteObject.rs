#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
// Java: public class WMF_DeleteObject extends WMFRecord
pub struct WMF_DeleteObject {
    // Java field: int index
    index: i32,
}

impl WMF_DeleteObject {
    // Java: WMF_DeleteObject()
    pub fn new() -> Self {
        // Java: super();
        WMF_DeleteObject {
            index: 0,
        }
    }

    // Java: public WMFRecord read(WMFParser wmf, int len)
    pub fn read(&self, _wmf: i32, _len: i32) -> i32 {
        // Java: WMF_DeleteObject deleteObject = new WMF_DeleteObject();
        // Java: deleteObject.index = wmf.readUnsignedShort();
        // Java: return deleteObject;
        0
    }

    // Java: public void render(MetaFilePainter renderer)
    pub fn render(&self, _renderer: i32) {
        // Java: renderer.getGdiObjects().setObject(this.index, (IGdiObject)null);
    }

    // Java: public String toString()
    pub fn toString(&self) -> String {
        // Java: return "DeleteObject";
        String::new()
    }

}
