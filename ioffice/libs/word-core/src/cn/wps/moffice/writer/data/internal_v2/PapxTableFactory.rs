// Java: import cn.wps.moffice.writer.core.Document
use crate::cn::wps::moffice::writer::core::Document::*;
// Java: import cn.wps.moffice.writer.data.IPapxTable
use crate::cn::wps::moffice::writer::data::IPapxTable::*;
// Java: import cn.wps.moffice.writer.data.internal_v2.PapxTable
use crate::cn::wps::moffice::writer::data::internal_v2::PapxTable::*;

// java public class PapxTableFactory
#[derive(Default)]
pub struct PapxTableFactory {
}

// java public class PapxTableFactory
impl PapxTableFactory {
    // java public static IPapxTable createPapxTable(Document document)
    pub fn createPapxTable(document: *mut dyn Document) -> Option<Box<dyn IPapxTable>> {
      // java return document == null ? null : new PapxTable(document);
      if document.is_null() {
        None
      } else {
        Some(Box::new(PapxTable::PapxTable(document)) as Box<dyn IPapxTable>)
      }
    }

}
