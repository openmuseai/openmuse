// Java: import cn.wps.moffice.writer.layout.base.env.LayoutState
use crate::cn::wps::moffice::writer::layout::base::env::LayoutState::LayoutState;
// Java: import cn.wps.moffice.writer.layout.base.graph.FilterArgs
use crate::cn::wps::moffice::writer::layout::base::graph::FilterArgs::FilterArgs;

// java public interface IFilter
pub trait IFilter {
    // java  boolean doFilter(FilterArgs var1, LayoutState var2)
    fn doFilter(&mut self, var1: &mut FilterArgs, var2: &mut LayoutState) -> bool;

}