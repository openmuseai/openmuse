#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
// Java: public class ExtTextOutA extends EMFRecord
pub struct ExtTextOutA {
    // Java field: String text
    text: Option<String>,
    // Java field: PointF pos
    pos: Option<i32>,
    // Java field: float[] widths
    widths: Option<Vec<f32>>,
}

impl ExtTextOutA {
    // Java: ExtTextOutA()
    pub fn new() -> Self {
        // Java: super();
        ExtTextOutA {
            text: None,
            pos: None,
            widths: None,
        }
    }

    // Java: public EMFRecord read(EMFParser emf, int size)
    pub fn read(&self, _emf: i32, _size: i32) -> i32 {
        // Java: ExtTextOutA tag = new ExtTextOutA();
        // Java: emf.skip(28);
        // Java: tag.pos = emf.readIntPoint();
        // Java: int tLength = emf.readUnsignedInt();
        // Java: emf.skip(28);
        // Java: String charset = null;
        // Java: byte[] b = emf.readByte(tLength);
        // Java: if (b.length == 1 && b[0] < 0)
        {
        // Java: charset = "ISO-8859-1";
        }
        // Java: else
        {
        // Java: charset = "GBK";
        }
        // Java: try
        {
        // Java: tag.text = new String(b, charset);
        }
        // Java: catch (UnsupportedEncodingException e)
        {
        // Java: e.printStackTrace();
        }
        // Java: if (tLength % 4 != 0)
        {
        // Java: emf.skip(4 - tLength % 4);
        }
        // Java: float[] widths = null;
        // Java: if (tag.text.length() == tLength)
        {
        // Java: widths = new float[tLength];
        // Java: for(int i = 0; i < tLength; ++i)
        {
        // Java: widths[i] = (float)emf.readUnsignedInt();
        }
        }
        // Java: else if (tag.text.length() * 2 == tLength)
        {
        // Java: int len = tLength / 2;
        // Java: widths = new float[len];
        // Java: for(int i = 0; i < len; ++i)
        {
        // Java: widths[i] = (float)(emf.readUnsignedInt() + emf.readUnsignedInt());
        }
        }
        // Java: else
        {
        // Java: emf.skip(4 * tLength);
        }
        // Java: tag.widths = widths;
        // Java: return tag;
        0
    }

    // Java: public void render(MetaFilePainter renderer)
    pub fn render(&self, _renderer: i32) {
        // Java: renderer.drawTextA(this.text, (int)this.pos.x, (int)this.pos.y, this.widths);
    }

}
