// Java: import cn.wps.kfc.impl.arrayList.WLongArrayList
use util::kfc::r#impl::arrayList::WLongArrayList::*;
// Java: import cn.wps.moffice.writer.core.Document
use crate::cn::wps::moffice::writer::core::Document::*;

// java public interface IFilter
pub trait IFilter {
    // java  WLongArrayList getFilterRanges(Document var1, long var2)
    fn getFilterRanges(&self, var1: &dyn Document, var2: i64) -> &WLongArrayList;

}
