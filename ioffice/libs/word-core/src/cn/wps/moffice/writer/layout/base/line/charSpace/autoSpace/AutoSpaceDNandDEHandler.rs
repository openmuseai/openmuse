#![allow(non_snake_case)]
use crate::cn::wps::moffice::writer::layout::base::env::DocumentEnv::DocumentEnv;
use crate::cn::wps::moffice::writer::layout::base::env::LayoutState::LayoutState;
use crate::cn::wps::moffice::writer::layout::base::graph::Graphs::Graphs;
use crate::cn::wps::moffice::writer::layout::base::line::SubLineEnv::SubLineEnv;
use super::AutoSpaceBaseHandler::AutoSpaceBaseHandler;
use super::IAutoSpaceHandler::IAutoSpaceHandler;

// Java: public class AutoSpaceDNandDEHandler extends AutoSpaceBaseHandler
pub struct AutoSpaceDNandDEHandler { pub super_: AutoSpaceBaseHandler }
impl AutoSpaceDNandDEHandler {
    // Java: public AutoSpaceDNandDEHandler(DocumentEnv docEnv)
    pub fn AutoSpaceDNandDEHandler(docEnv: *const DocumentEnv) -> Self { Self { super_: AutoSpaceBaseHandler::AutoSpaceBaseHandler(docEnv) } }
    // Java: public void dealAutoSpace(Graphs prev, Graphs cur)
    pub fn dealAutoSpace(&mut self, prev: *mut Graphs, cur: *mut Graphs) {
        // Java: if (null != prev)
        if prev.is_null() || cur.is_null() { return; }
        let (prev, cur) = unsafe { (&mut *prev, &*cur) };
        // Java: needToAddSpace for 1↔2 and 4↔2.
        let needToAddSpace = matches!((prev.autoSpaceType, cur.autoSpaceType), (1,2)|(2,1)|(4,2)|(2,4));
        // Java: if (needToAddSpace) this.extendSpace(prev);
        if needToAddSpace { self.super_.extendSpace(prev); }
    }
}
impl IAutoSpaceHandler for AutoSpaceDNandDEHandler {
    fn setEnv(&mut self, env: *mut LayoutState) { self.super_.setEnv(env); }
    fn setSubLineEnv(&mut self, env: *mut SubLineEnv) { self.super_.setSubLineEnv(env); }
    fn dealAutoSpace(&mut self, prev: *mut Graphs, cur: *mut Graphs) { Self::dealAutoSpace(self, prev, cur); }
}
