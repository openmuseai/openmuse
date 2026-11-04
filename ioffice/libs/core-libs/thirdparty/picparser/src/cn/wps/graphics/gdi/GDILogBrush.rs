#![allow(non_snake_case, non_upper_case_globals, dead_code, unused_variables, unused_imports, unused_mut)]

// Java: import cn.wps.graphics.ReColorMap
// Java: import cn.wps.graphics.data.reader.BufferReader

use crate::cn::wps::graphics::gdi::IGdiObject::{IGdiObject, GdiPainter};
use crate::cn::wps::graphics::gdi::ColorPaint::ColorPaint;
use crate::cn::wps::graphics::data::reader::BufferReader::BufferReader;

// Java: public class GDILogBrush implements IGdiObject
pub struct GDILogBrush {
    // Java field: int STYLE_SOLID
    // Java field: int STYLE_NULL
    // Java field: int STYLE_HATCH
    // Java field: int style
    style: i32,
    // Java field: int hatch
    hatch: i32,
    // Java field: ColorPaint color
    color: Option<ColorPaint>,
    // Java field: BitGdiPaint brushPaint
    brushPaint: bool,
}

pub(crate) const STYLE_SOLID: i32 = 0;
pub(crate) const STYLE_NULL: i32 = 1;
pub(crate) const STYLE_HATCH: i32 = 2;

// Java: public class GDILogBrush implements IGdiObject
impl GDILogBrush {
    // Java: GDILogBrush(int style, ColorPaint color)
    pub fn new_with_args(style: i32, color: Option<ColorPaint>) -> Self {
        // Java: super();
        // Java: this.style = style;
        // Java: this.color = color;
        GDILogBrush { style, hatch: 0, color, brushPaint: false }
    }

    // Java: GDILogBrush()
    pub fn new() -> Self {
        // Java: super();
        GDILogBrush { style: 0, hatch: 0, color: None, brushPaint: false }
    }

    // Java: static GDILogBrush readLogBrush(BufferReader reader)
    pub fn readLogBrush(reader: &mut BufferReader) -> GDILogBrush {
        // Java: GDILogBrush brush = new GDILogBrush();
        let mut brush = GDILogBrush::new();
        // Java: brush.style = reader.readUnsignedInt();
        brush.style = reader.readUnsignedInt_one();
        // Java: brush.color = ColorPaint.create(reader.readColor());
        brush.color = Some(ColorPaint::create(reader.readColor()));
        // Java: brush.hatch = reader.readUnsignedInt();
        brush.hatch = reader.readUnsignedInt_one();
        // Java: return brush;
        brush
    }

    // Java: void render(GdiPainter renderer)
    pub fn render_impl(&mut self, renderer: &GdiPainter) {
        // Java: if (this.style == 1)
        if self.style == 1 {
            // Java: renderer.getDC().setBrushPaint(ColorPaint.NULL);
        }
        // Java: else if (this.style == 2)
        else if self.style == 2 {
            // Java: int bkColor = renderer.getDrawer().getBkColor();
            let bkColor = 0;
            // Java: this.brushPaint = new BitGdiPaint(PixelImage.createHatchedImage(this.color, this.hatch, bkColor));
            // Java: renderer.getDC().setBrushPaint(this.brushPaint);
        }
        // Java: else
        else {
            // Java: this.color.setColor(ReColorMap.getReColorFill(this.color.getOriColor(), this.style));
            // Java: renderer.getDC().setBrushPaint(this.color);
        }
    }
}

impl IGdiObject for GDILogBrush {
    fn render(&self, _var1: &GdiPainter) {}
}
