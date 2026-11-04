// Rust stub for KerningHandler (auto-generated)
// Original: cn/wps/moffice/writer/layout/base/line/charSpace/KerningHandler.java
use crate::*;

pub struct KerningHandler {
    // TODO: fields
}

impl KerningHandler {
    pub fn new() -> Self { todo!() }
    pub fn reuseClean(&self) -> bool { todo!() }
    pub fn reuseInit(&self) { todo!() }
    pub fn dispose(&self) { todo!() }
    pub fn setEnv(&self, env: LayoutState) { todo!() }
    pub fn handleKerning(&self, prevRun: Graphs, run: Graphs) { todo!() }
    pub fn handleKerningNoEffectToCache(&self, prevRun: Graphs, run: Graphs) -> f32 { todo!() }
    pub fn needProcessPrevRun(&self, fontStyle0: TypoFontStyle, fontStyle1: TypoFontStyle) -> bool { todo!() }
    pub fn processRun(&self, run: Graphs) { todo!() }
    pub fn processPrevRun(&self, prevRun: Graphs, run: Graphs) { todo!() }
    pub fn round(&self, val: f32) -> i32 { todo!() }
}
