use crate::cn::wps::moffice::writer::layout::base::env::LayoutState::LayoutState;
use crate::cn::wps::moffice::writer::layout::base::graph::Graphs::Graphs;
use crate::cn::wps::moffice::writer::layout::base::line::SubLineEnv::SubLineEnv;

// Java: public interface IPunctSpaceHandler
pub trait IPunctSpaceHandler {
    // Java: void setEnv(LayoutState var1);
    fn setEnv(&mut self, env: *mut LayoutState);
    // Java: void setSubLineEnv(SubLineEnv var1);
    fn setSubLineEnv(&mut self, env: *mut SubLineEnv);
    // Java: void onAddRun(Graphs var1, Graphs var2);
    fn onAddRun(&mut self, prevRun: *mut Graphs, run: *mut Graphs);
    // Java: float getCompressWidth(Graphs var1, Graphs var2);
    fn getCompressWidth(&mut self, prevRun: *mut Graphs, run: *mut Graphs) -> f32;
    // Java: void dispose();
    fn dispose(&mut self);
}
