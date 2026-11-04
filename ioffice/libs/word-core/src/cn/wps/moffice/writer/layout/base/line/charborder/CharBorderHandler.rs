// Rust stub for CharBorderHandler (auto-generated)
// Original: cn/wps/moffice/writer/layout/base/line/charborder/CharBorderHandler.java
use crate::*;

pub struct CharBorderHandler {
    // TODO: fields
}

impl CharBorderHandler {
    pub fn new() -> Self { todo!() }
    pub fn reuseClean(&self) -> bool { todo!() }
    pub fn reuseInit(&self) { todo!() }
    pub fn setEnv(&self, env: LayoutState) { todo!() }
    pub fn setSubLineEnv(&self, subLineEnv: SubLineEnv) { todo!() }
    pub fn getBdrEnv(&self) -> BdrEnv { todo!() }
    pub fn handleRun(&self, run: Graphs) { todo!() }
    pub fn endLine(&self, normalBreakLine: bool) { todo!() }
    pub fn adjustBdrForSpace(&self, normalBreakLine: bool) { todo!() }
    pub fn extendBdrForTailSpaces(&self, after: Graphs) { todo!() }
    pub fn rollbackBdrForTailSapces(&self, beginGraph: Graphs) { todo!() }
}
