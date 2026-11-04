#![allow(non_snake_case, non_upper_case_globals, dead_code, unused_variables, unused_imports, unused_mut)]

// Java: import cn.wps.graphics.ReColorMap
// Java: import cn.wps.graphics.data.reader.BufferReader

use crate::cn::wps::graphics::gdi::IGdiObject::{IGdiObject, GdiPainter};
use crate::cn::wps::graphics::gdi::ColorPaint::ColorPaint;
use crate::cn::wps::graphics::data::reader::BufferReader::BufferReader;

// Java: public class GdiPen implements IGdiObject
pub struct GdiPen {
    // Java field: int style
    style: i32,
    // Java field: int width
    width: i32,
    // Java field: ColorPaint color
    color: Option<ColorPaint>,
    // Java field: float[] dash
    dash: Option<Vec<f32>>,
}

// Java: public class GdiPen implements IGdiObject
impl GdiPen {
    // Java: GdiPen()
    pub fn new() -> Self {
        // Java: super();
        GdiPen { style: 0, width: 0, color: None, dash: None }
    }

    // Java: GdiPen(int style, int width, ColorPaint color)
    pub fn new_with_args(style: i32, width: i32, color: ColorPaint) -> Self {
        // Java: super();
        // Java: this.style = style;
        // Java: this.width = width;
        // Java: this.color = color;
        GdiPen { style, width, color: Some(color), dash: None }
    }

    // Java: static GdiPen readExtLogPen(BufferReader reader)
    pub fn readExtLogPen(reader: &mut BufferReader) -> GdiPen {
        // Java: GdiPen pen = new GdiPen();
        let mut pen = GdiPen::new();
        // Java: pen.style = reader.readUnsignedInt();
        pen.style = reader.readUnsignedInt_one();
        // Java: pen.width = reader.readUnsignedInt();
        pen.width = reader.readUnsignedInt_one();
        // Java: reader.skip(4);
        reader.skip(4);
        // Java: pen.color = ColorPaint.create(reader.readColor());
        pen.color = Some(ColorPaint::create(reader.readColor()));
        // Java: reader.skip(4);
        reader.skip(4);
        // Java: int nStyle = reader.readUnsignedInt();
        let nStyle = reader.readUnsignedInt_one();
        // Java: if (nStyle == 0)
        if nStyle == 0 {
            // Java: reader.skip(4);
            reader.skip(4);
        }
        // Java: int[] dash = reader.readUnsignedInt(nStyle);
        let dash_ints = if nStyle > 0 {
            let mut d = vec![0i32; nStyle as usize];
            for j in 0..nStyle as usize { d[j] = reader.readUnsignedInt_one(); }
            Some(d)
        } else { None };
        // Java: if (dash != null)
        if let Some(ref d) = dash_ints {
            // Java: pen.dash = new float[dash.length];
            let mut dash_f = vec![0.0f32; d.len()];
            // Java: for(int i = 0; i < dash.length; ++i)
            for i in 0..d.len() {
                // Java: pen.dash[i] = (float)dash[i];
                dash_f[i] = d[i] as f32;
            }
            pen.dash = Some(dash_f);
        }
        // Java: return pen;
        pen
    }

    // Java: static GdiPen readLogPen(BufferReader reader)
    pub fn readLogPen(reader: &mut BufferReader) -> GdiPen {
        // Java: GdiPen pen = new GdiPen();
        let mut pen = GdiPen::new();
        // Java: pen.style = reader.readUnsignedInt();
        pen.style = reader.readUnsignedInt_one();
        // Java: pen.width = reader.readUnsignedInt();
        pen.width = reader.readUnsignedInt_one();
        // Java: reader.skip(4);
        reader.skip(4);
        // Java: pen.color = ColorPaint.create(reader.readColor());
        pen.color = Some(ColorPaint::create(reader.readColor()));
        // Java: return pen;
        pen
    }

    // Java: void render(GdiPainter renderer)
    pub fn render_impl(&mut self, renderer: &GdiPainter) {
        // Java: if ((this.style & 255) == 5)
        if (self.style & 255) == 5 {
            // Java: this.color = ColorPaint.NULL;
            self.color = Some(ColorPaint::new(0));
        }
        // Java: int cap = this.style & 3840;
        let cap = self.style & 3840;
        // Java: int join = this.style & '\uf000';
        let join = self.style & 0xF000;
        // Java: float[] dstDash = GdiTool.getDash(this.style & 255, this.dash);
        let _dstDash = self.dash.clone();
        // Java: int meterLimit = renderer.getDC().getMeterLimit();
        let meterLimit = 0;
        // Java: GdiStroke stroke = new GdiStroke((float)this.width, cap, join, (float)meterLimit, dstDash, 0.0F);
        // Java: this.color.setColor(ReColorMap.getReColor(this.color.getOriColor()));
        // Java: renderer.getDC().setLinePaint(this.color);
        // Java: renderer.getDC().setStroke(stroke);
    }
}

impl IGdiObject for GdiPen {
    fn render(&self, _var1: &GdiPainter) {}
}
