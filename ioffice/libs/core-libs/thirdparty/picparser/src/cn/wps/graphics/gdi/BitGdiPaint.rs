#![allow(non_snake_case, dead_code, unused_variables)]

use crate::cn::wps::graphics::gdi::IGdiPainter::IGdiPainter;
use crate::cn::wps::graphics::gdi::IGdiObject::{IGdiObject, GdiPainter};
use crate::cn::wps::graphics::gdi::GdiImage::GdiImage;

// Java: public class BitGdiPaint implements IGdiPainter, IGdiObject
pub struct BitGdiPaint {
    // Java field: GdiImage image
    pub(crate) image: Box<dyn GdiImage>,
}

// Java: public class BitGdiPaint implements IGdiPainter, IGdiObject
impl BitGdiPaint {
    // Java: BitGdiPaint(GdiImage image)
    pub fn new(image: Box<dyn GdiImage>) -> Self {
        // Java: super();
        // Java: this.image = image;
        BitGdiPaint { image }
    }

    // Java: int getType()
    pub fn getType_impl(&self) -> i32 {
        // Java: return 1;
        1
    }

    // Java: GdiImage getImage()
    pub fn getImage(&self) -> &dyn GdiImage {
        // Java: return this.image;
        &*self.image
    }

    // Java: void render(GdiPainter renderer)
    pub fn render_impl(&self, renderer: &GdiPainter) {
        // Java: renderer.getDC().setBrushPaint(this);
    }
}

impl IGdiPainter for BitGdiPaint {
    fn getType(&self) -> i32 { self.getType_impl() }
}

impl IGdiObject for BitGdiPaint {
    fn render(&self, var1: &GdiPainter) { self.render_impl(var1); }
}
