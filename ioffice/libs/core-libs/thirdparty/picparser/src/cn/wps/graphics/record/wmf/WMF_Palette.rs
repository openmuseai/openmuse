#![allow(non_snake_case, non_camel_case_types, dead_code, unused_variables, unused_imports)]

// Java: import cn.wps.graphics.gdi.GdiPainter
// Java: import cn.wps.graphics.gdi.IGdiObject

use crate::cn::wps::graphics::gdi::IGdiObject::{IGdiObject, GdiPainter};

// Java: public class WMF_Palette implements IGdiObject
pub struct WMF_Palette {
}

// Java: public class WMF_Palette implements IGdiObject
impl WMF_Palette {
    // Java: WMF_Palette()
    pub fn new() -> Self {
        // Java: super();
        WMF_Palette {}
    }

    // Java: void render(GdiPainter renderer)
    pub fn render_impl(&self, renderer: &GdiPainter) {
    }
}

impl IGdiObject for WMF_Palette {
    fn render(&self, _var1: &GdiPainter) {}
}
