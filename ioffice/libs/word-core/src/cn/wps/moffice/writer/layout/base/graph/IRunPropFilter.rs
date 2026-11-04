// Java: import cn.wps.moffice.property.PropertySet
use util::moffice::property::PropertySet::PropertySet;
// Java: import cn.wps.moffice.writer.data.IChpxTable
use crate::cn::wps::moffice::writer::data::IChpxTable::{IChpxTable, Entry as ChpxEntry};
// Java: import cn.wps.moffice.writer.layout.base.env.LayoutState
use crate::cn::wps::moffice::writer::layout::base::env::LayoutState::LayoutState;

// java public interface IRunPropFilter
pub trait IRunPropFilter {
    // java  PropertySet doFilter(PropertySet var1, IChpxTable.Entry var2, boolean var3, LayoutState var4)
    fn doFilter(&self, var1: &PropertySet, var2: &dyn ChpxEntry, var3: bool, var4: &mut LayoutState) -> PropertySet;

    // rust: Java `filter instanceof IgnoreHpsAndVAlignRunFilter` / `IgnoreCapsAndSmallCapsFilter` 判别
    //       默认 false，具体过滤器覆写。
    fn isIgnoreHpsAndVAlignRunFilter(&self) -> bool {
        false
    }
    fn isIgnoreCapsAndSmallCapsFilter(&self) -> bool {
        false
    }
}