use crate::cn::wps::moffice::writer::layout::base::env::LayoutState::LayoutState;
use crate::cn::wps::moffice::writer::layout::base::graph::Graphs::Graphs;
use crate::cn::wps::moffice::writer::layout::base::line::SubLineEnv::SubLineEnv;

// Java: public interface IAutoSpaceHandler extends AutoSpaceType
pub trait IAutoSpaceHandler {
    // Java: void setEnv(LayoutState var1);
    fn setEnv(&mut self, env: *mut LayoutState);
    // Java: void setSubLineEnv(SubLineEnv var1);
    fn setSubLineEnv(&mut self, env: *mut SubLineEnv);
    // Java: void dealAutoSpace(Graphs var1, Graphs var2);
    fn dealAutoSpace(&mut self, prev: *mut Graphs, cur: *mut Graphs);
}
