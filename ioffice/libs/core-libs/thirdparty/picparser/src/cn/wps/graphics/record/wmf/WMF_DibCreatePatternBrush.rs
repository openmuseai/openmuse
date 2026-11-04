#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
// Java: public class WMF_DibCreatePatternBrush extends WMFRecord
pub struct WMF_DibCreatePatternBrush {
    // Java field: int style
    style: i32,
    // Java field: int usage
    usage: i32,
    // Java field: int type
    r#type: i32,
    // Java field: int width
    width: i32,
    // Java field: int height
    height: i32,
    // Java field: int widthBytes
    widthBytes: i32,
    // Java field: int planes
    planes: i32,
    // Java field: int bitsPixel
    bitsPixel: i32,
    // Java field: BitGdiPaint brushPaint
    brushPaint: Option<i32>,
}

impl WMF_DibCreatePatternBrush {
    // Java: WMF_DibCreatePatternBrush()
    pub fn new() -> Self {
        // Java: super();
        WMF_DibCreatePatternBrush {
            style: 0,
            usage: 0,
            r#type: 0,
            width: 0,
            height: 0,
            widthBytes: 0,
            planes: 0,
            bitsPixel: 0,
            brushPaint: None,
        }
    }

    // Java: public WMFRecord read(WMFParser wmf, int len)
    pub fn read(&self, _wmf: i32, _len: i32) -> i32 {
        // Java: WMF_DibCreatePatternBrush tag = new WMF_DibCreatePatternBrush();
        // Java: tag.style = wmf.readUnsignedShort();
        // Java: tag.usage = wmf.readUnsignedShort();
        // Java: WMF_DibObject dibObject = new WMF_DibObject(len - 4, wmf);
        // Java: GdiImage image = dibObject.getImage();
        // Java: tag.brushPaint = new BitGdiPaint(image);
        // Java: return tag;
        0
    }

    // Java: public void render(MetaFilePainter renderer)
    pub fn render(&self, _renderer: i32) {
        // Java: renderer.getDC().setBrushPaint(this.brushPaint);
        // Java: renderer.getGdiObjects().setObject(-1, this.brushPaint);
    }

}
