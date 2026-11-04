// Java: import cn.wps.moffice.writer.core.Collection
use crate::cn::wps::moffice::writer::core::Collection::*;
// Java: import cn.wps.moffice.writer.core.Document
use crate::cn::wps::moffice::writer::core::Document::*;
// Java: import cn.wps.moffice.writer.data.IPapxTable
use crate::cn::wps::moffice::writer::data::IPapxTable::*;

// java public interface ListParagraphs
pub trait ListParagraphs {
    // java  Document getDocument()
    fn getDocument(&self) -> &(dyn Document);

}
