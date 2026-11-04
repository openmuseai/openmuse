#![allow(non_snake_case, non_camel_case_types, dead_code, unused_variables, unused_imports)]

// Java: import cn.wps.graphics.gdi.GdiPainter
// Java: import cn.wps.graphics.gdi.IGdiObject

use crate::cn::wps::graphics::gdi::IGdiObject::{IGdiObject, GdiPainter};

// Java: public class WMF_PatternBrush implements IGdiObject
pub struct WMF_PatternBrush {
}

// Java: public class WMF_PatternBrush implements IGdiObject
impl WMF_PatternBrush {
    // Java: WMF_PatternBrush()
    pub fn new() -> Self {
        // Java: super();
        WMF_PatternBrush {}
    }

    // Java: void render(GdiPainter renderer)
    pub fn render_impl(&self, renderer: &GdiPainter) {
    }
}

impl IGdiObject for WMF_PatternBrush {
    fn render(&self, _var1: &GdiPainter) {}
}
