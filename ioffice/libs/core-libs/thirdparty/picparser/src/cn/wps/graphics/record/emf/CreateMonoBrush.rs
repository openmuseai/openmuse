#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
// Java: public class CreateMonoBrush extends EMFRecord
pub struct CreateMonoBrush {
    // Java field: int index
    index: i32,
    // Java field: int usage
    usage: i32,
    // Java field: EMFImageInfo info
    info: Option<i32>,
    // Java field: int offBmi
    offBmi: i32,
    // Java field: int cbBmi
    cbBmi: i32,
    // Java field: int offBits
    offBits: i32,
    // Java field: int cbBits
    cbBits: i32,
    // Java field: BitGdiPaint brushPaint
    brushPaint: Option<i32>,
}

impl CreateMonoBrush {
    // Java: CreateMonoBrush()
    pub fn new() -> Self {
        // Java: super();
        CreateMonoBrush {
            index: 0,
            usage: 0,
            info: None,
            offBmi: 0,
            cbBmi: 0,
            offBits: 0,
            cbBits: 0,
            brushPaint: None,
        }
    }

    // Java: public EMFRecord read(EMFParser emf, int size)
    pub fn read(&self, _emf: i32, _size: i32) -> i32 {
        // Java: int pos = emf.getCurrentPos();
        // Java: CreateMonoBrush cmb = new CreateMonoBrush();
        // Java: cmb.index = emf.readUnsignedInt();
        // Java: cmb.usage = emf.readUnsignedInt();
        // Java: cmb.offBmi = emf.readUnsignedInt();
        // Java: cmb.cbBmi = emf.readUnsignedInt();
        // Java: cmb.offBits = emf.readUnsignedInt();
        // Java: cmb.cbBits = emf.readUnsignedInt();
        // Java: emf.setCurrentPos(pos + cmb.offBmi - 8);
        // Java: cmb.info = EMFImageInfo.read(emf);
        // Java: cmb.info.special = true;
        // Java: int imageSize = cmb.offBits - cmb.offBmi;
        // Java: if (cmb.info.clrUsed == 0)
        {
        // Java: imageSize = (int)((double)imageSize - Math.pow((double)2.0F, (double)cmb.info.bitCount) * (double)4.0F);
        }
        // Java: GdiImage image = EMFImageParser.readImage(emf, cmb.info, cmb.info.width, cmb.info.height, imageSize);
        // Java: cmb.brushPaint = new BitGdiPaint(image);
        // Java: return cmb;
        0
    }

    // Java: public void render(MetaFilePainter renderer)
    pub fn render(&self, _renderer: i32) {
        // Java: renderer.getDC().setBrushPaint(this.brushPaint);
        // Java: renderer.getGdiObjects().setObject(this.index, this.brushPaint);
    }

}
