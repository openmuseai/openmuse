// Java: cn.wps.moffice.writer.layout.base.line.charSpace.IWhiteSpaceHandler
// rust: ★wr-29 05 —— 真实移植（Java 28 行）
#![allow(dead_code, non_snake_case)]

use crate::cn::wps::moffice::writer::layout::base::env::LayoutState::LayoutState;
use crate::cn::wps::moffice::writer::layout::base::graph::Graphs::Graphs;
use crate::cn::wps::moffice::writer::layout::base::line::SubLineEnv::SubLineEnv;

// java public interface IWhiteSpaceHandler
pub trait IWhiteSpaceHandler {
    // java void setEnv(LayoutState var1);
    fn setEnv(&mut self, var1: &LayoutState);

    // java void setSubLineEnv(SubLineEnv var1);
    fn setSubLineEnv(&mut self, var1: &mut SubLineEnv);

    // java boolean handleSpace(Graphs var1, Graphs var2);
    fn handleSpace(&mut self, var1: &mut Graphs, var2: &mut Graphs) -> bool;

    // java void endLine();
    fn endLine(&mut self);
}
