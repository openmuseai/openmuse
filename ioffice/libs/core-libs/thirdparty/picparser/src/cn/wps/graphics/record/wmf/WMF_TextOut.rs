#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
// Java: public class WMF_TextOut extends WMFRecord
pub struct WMF_TextOut {
    // Java field: int x
    x: i32,
    // Java field: int y
    y: i32,
    // Java field: byte[] bytes
    bytes: Option<Vec<u8>>,
    // Java field: String text
    text: Option<String>,
}

impl WMF_TextOut {
    // Java: WMF_TextOut()
    pub fn new() -> Self {
        // Java: super();
        WMF_TextOut {
            x: 0,
            y: 0,
            bytes: None,
            text: None,
        }
    }

    // Java: public WMFRecord read(WMFParser wmf, int len)
    pub fn read(&self, _wmf: i32, _len: i32) -> i32 {
        // Java: WMF_TextOut tag = new WMF_TextOut();
        // Java: int strlenght = wmf.readShort();
        // Java: tag.bytes = wmf.readByte(strlenght);
        // Java: if ((strlenght & 1) == 1)
        {
        // Java: wmf.skip(1);
        }
        // Java: tag.y = wmf.readShort();
        // Java: tag.x = wmf.readShort();
        // Java: return tag;
        0
    }

    // Java: public void render(MetaFilePainter renderer)
    pub fn render(&self, _renderer: i32) {
        // Java: if (null == this.text)
        {
        // Java: this.text = WMF_ExtTextOut.readWmfString(this.bytes, renderer.getDC().getFont(), renderer.getAffectCharsetSet());
        }
        // Java: renderer.drawTextW(this.text, this.x, this.y);
    }

    // Java: public String toString()
    pub fn toString(&self) -> String {
        // Java: return super.toString() + "(" + this.x + ", " + this.y + ", " + this.text + ")";
        String::new()
    }

}
