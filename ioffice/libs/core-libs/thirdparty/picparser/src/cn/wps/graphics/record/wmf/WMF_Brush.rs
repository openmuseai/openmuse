#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

// Java: import cn.wps.graphics.ReColorMap
// LLM No Rust equivalent: using stub
// Java: import cn.wps.graphics.data.reader.WMFParser
// LLM No Rust equivalent: using stub
// Java: import cn.wps.graphics.gdi.BitGdiPaint
// LLM No Rust equivalent: using stub
// Java: import cn.wps.graphics.gdi.ColorPaint
// LLM No Rust equivalent: using stub
// Java: import cn.wps.graphics.gdi.GdiPainter
// LLM No Rust equivalent: using stub
// Java: import cn.wps.graphics.gdi.IGdiObject
// LLM No Rust equivalent: using stub
// Java: import cn.wps.graphics.gdi.PixelImage
// LLM No Rust equivalent: using stub

use std::fmt;

pub struct ReColorMap;
impl ReColorMap {
    pub fn getReColorFill(oriColor: i32, style: i32) -> i32 { oriColor }
}

pub struct WMFParser { _private: () }
impl WMFParser {
    pub fn readUnsignedShort(&mut self) -> i32 { 0 }
    pub fn readColor(&mut self) -> i32 { 0 }
}

#[derive(Clone)]
pub struct ColorPaint {
    color: i32,
    ori_color: i32,
}
impl ColorPaint {
    pub const NULL: ColorPaint = ColorPaint { color: 0, ori_color: 0 };
    pub fn create(color: i32) -> Self { ColorPaint { color, ori_color: color } }
    pub fn getOriColor(&self) -> i32 { self.ori_color }
    pub fn setColor(&mut self, color: i32) { self.color = color; }
}
impl fmt::Display for ColorPaint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "#{:08X}", self.color)
    }
}

pub struct BitGdiPaint { _private: () }
impl BitGdiPaint {
    pub fn new(image: PixelImage) -> Self { BitGdiPaint { _private: () } }
}

pub struct PixelImage { _private: () }
impl PixelImage {
    pub fn createHatchedImage(color: &ColorPaint, hatch: i32, bkColor: i32) -> Self { PixelImage { _private: () } }
}

pub struct DC { _private: () }
impl DC {
    pub fn setBrushPaint_color(&mut self, _paint: &ColorPaint) {}
    pub fn setBrushPaint_bit(&mut self, _paint: &BitGdiPaint) {}
    pub fn setBrushPaint_null(&mut self, _paint: ColorPaint) {}
}

pub struct Drawer { _private: () }
impl Drawer {
    pub fn getBkColor(&self) -> i32 { 0 }
}

pub struct GdiPainter { _private: () }
impl GdiPainter {
    pub fn getDC(&mut self) -> &mut DC { todo!() }
    pub fn getDrawer(&self) -> &Drawer { todo!() }
}

pub trait IGdiObject {
    fn render(&mut self, renderer: &mut GdiPainter);
}

// Java: public class WMF_Brush implements IGdiObject
pub struct WMF_Brush {
    // Java field: int style
    pub style: i32,
    // Java field: ColorPaint color
    pub color: ColorPaint,
    // Java field: int hatch
    pub hatch: i32,
    // Java field: BitGdiPaint brushPaint
    pub brushPaint: Option<BitGdiPaint>,
}

// Java: public class WMF_Brush implements IGdiObject
impl WMF_Brush {
    // Java: WMF_Brush(int style, ColorPaint color, int hatch)
    pub fn new(style: i32, color: ColorPaint, hatch: i32) -> Self {
        // Java: super();
        // LLM No Rust equivalent: no superclass
        // Java: this.style = style;
        // LLM JavaLine Aggregation: merged into Self struct literal
        // Java: this.color = color;
        // LLM JavaLine Aggregation: merged into Self struct literal
        // Java: this.hatch = hatch;
        // LLM JavaLine Aggregation: merged into Self struct literal
        WMF_Brush { style, color, hatch, brushPaint: None }
    }

    // Java: WMF_Brush(WMFParser wmf)
    pub fn new_from_wmf(wmf: &mut WMFParser) -> Self {
        // Java: super();
        // LLM No Rust equivalent: no superclass
        // Java: this.style = wmf.readUnsignedShort();
        // LLM JavaLine Aggregation: merged into Self struct literal
        let style = wmf.readUnsignedShort();
        // Java: this.color = ColorPaint.create(wmf.readColor());
        // LLM JavaLine Aggregation: merged into Self struct literal
        let color = ColorPaint::create(wmf.readColor());
        // Java: this.hatch = wmf.readUnsignedShort();
        // LLM JavaLine Aggregation: merged into Self struct literal
        let hatch = wmf.readUnsignedShort();
        WMF_Brush { style, color, hatch, brushPaint: None }
    }

    // Java: public String toString()
    pub fn toString(&self) -> String {
        // Java: return "  LogBrush32\n    style: " + this.style + "\n    color: " + this.color + "\n    hatch: " + this.hatch;
        format!("  LogBrush32\n    style: {}\n    color: {}\n    hatch: {}", self.style, self.color, self.hatch)
    }
}

impl IGdiObject for WMF_Brush {
    // Java: public void render(GdiPainter renderer)
    fn render(&mut self, renderer: &mut GdiPainter) {
        // Java: if (this.style == 1)
        if self.style == 1 {
            // Java: renderer.getDC().setBrushPaint(ColorPaint.NULL);
            renderer.getDC().setBrushPaint_null(ColorPaint::NULL);
        // Java: else if (this.style == 2)
        } else if self.style == 2 {
            // Java: int bkColor = renderer.getDrawer().getBkColor();
            let bkColor = renderer.getDrawer().getBkColor();
            // Java: this.brushPaint = new BitGdiPaint(PixelImage.createHatchedImage(this.color, this.hatch, bkColor));
            self.brushPaint = Some(BitGdiPaint::new(PixelImage::createHatchedImage(&self.color, self.hatch, bkColor)));
            // Java: renderer.getDC().setBrushPaint(this.brushPaint);
            if let Some(ref bp) = self.brushPaint {
                renderer.getDC().setBrushPaint_bit(bp);
            }
        // Java: else
        } else {
            // Java: this.color.setColor(ReColorMap.getReColorFill(this.color.getOriColor(), this.style));
            self.color.setColor(ReColorMap::getReColorFill(self.color.getOriColor(), self.style));
            // Java: renderer.getDC().setBrushPaint(this.color);
            renderer.getDC().setBrushPaint_color(&self.color);
        }
    }
}
