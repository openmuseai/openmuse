#![allow(non_snake_case)]
use crate::cn::wps::moffice::writer::layout::base::env::DocumentEnv::DocumentEnv;
use crate::cn::wps::moffice::writer::layout::base::env::LayoutState::LayoutState;
use crate::cn::wps::moffice::writer::layout::base::graph::Graphs::Graphs;
use crate::cn::wps::moffice::writer::layout::base::line::SubLineEnv::SubLineEnv;
use super::AutoSpaceBaseHandler::AutoSpaceBaseHandler;
use super::IAutoSpaceHandler::IAutoSpaceHandler;

// Java: public class AutoSpaceDEHandler extends AutoSpaceBaseHandler
pub struct AutoSpaceDEHandler { pub super_: AutoSpaceBaseHandler }
impl AutoSpaceDEHandler {
    // Java: public AutoSpaceDEHandler(DocumentEnv docEnv)
    pub fn AutoSpaceDEHandler(docEnv: *const DocumentEnv) -> Self { Self { super_: AutoSpaceBaseHandler::AutoSpaceBaseHandler(docEnv) } }
    // Java: public void dealAutoSpace(Graphs prev, Graphs cur)
    pub fn dealAutoSpace(&mut self, prev: *mut Graphs, cur: *mut Graphs) {
        if prev.is_null() || cur.is_null() { return; }
        let (prev, cur) = unsafe { (&mut *prev, &*cur) };
        // Java: 1 == prevType && 2 == curType || 2 == prevType && 1 == curType.
        if matches!((prev.autoSpaceType, cur.autoSpaceType), (1,2)|(2,1)) { self.super_.extendSpace(prev); }
    }
}
impl IAutoSpaceHandler for AutoSpaceDEHandler {
    fn setEnv(&mut self, env: *mut LayoutState) { self.super_.setEnv(env); }
    fn setSubLineEnv(&mut self, env: *mut SubLineEnv) { self.super_.setSubLineEnv(env); }
    fn dealAutoSpace(&mut self, prev: *mut Graphs, cur: *mut Graphs) { Self::dealAutoSpace(self, prev, cur); }
}
