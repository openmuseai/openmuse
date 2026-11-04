#![allow(non_snake_case)]
use crate::cn::wps::moffice::writer::layout::base::env::DocumentEnv::DocumentEnv;
use crate::cn::wps::moffice::writer::layout::base::env::LayoutState::LayoutState;
use crate::cn::wps::moffice::writer::layout::base::graph::Graphs::Graphs;
use crate::cn::wps::moffice::writer::layout::base::line::SubLineEnv::SubLineEnv;
use super::AutoSpaceBaseHandler::AutoSpaceBaseHandler;
use super::IAutoSpaceHandler::IAutoSpaceHandler;

// Java: public class AutoSpaceDNHandler extends AutoSpaceBaseHandler
pub struct AutoSpaceDNHandler { pub super_: AutoSpaceBaseHandler }
impl AutoSpaceDNHandler {
    // Java: public AutoSpaceDNHandler(DocumentEnv docEnv)
    pub fn AutoSpaceDNHandler(docEnv: *const DocumentEnv) -> Self { Self { super_: AutoSpaceBaseHandler::AutoSpaceBaseHandler(docEnv) } }
    // Java: public void dealAutoSpace(Graphs prev, Graphs cur)
    pub fn dealAutoSpace(&mut self, prev: *mut Graphs, cur: *mut Graphs) {
        if prev.is_null() || cur.is_null() { return; }
        let (prev, cur) = unsafe { (&mut *prev, &*cur) };
        // Java: 4 == prevType && 2 == curType || 2 == prevType && 4 == curType.
        if matches!((prev.autoSpaceType, cur.autoSpaceType), (4,2)|(2,4)) { self.super_.extendSpace(prev); }
    }
}
impl IAutoSpaceHandler for AutoSpaceDNHandler {
    fn setEnv(&mut self, env: *mut LayoutState) { self.super_.setEnv(env); }
    fn setSubLineEnv(&mut self, env: *mut SubLineEnv) { self.super_.setSubLineEnv(env); }
    fn dealAutoSpace(&mut self, prev: *mut Graphs, cur: *mut Graphs) { Self::dealAutoSpace(self, prev, cur); }
}
