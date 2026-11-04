// Java: import cn.wps.moffice.writer.core.Document
use crate::cn::wps::moffice::writer::core::Document::*;
// Java: import cn.wps.moffice.writer.data.IChpxTable
use crate::cn::wps::moffice::writer::data::IChpxTable::*;
// Java: import cn.wps.moffice.writer.data.internal_v2.ChpxTable
use crate::cn::wps::moffice::writer::data::internal_v2::ChpxTable::*;

// java public class ChpxTableFactory
#[derive(Default)]
pub struct ChpxTableFactory {
}

// java public class ChpxTableFactory
impl ChpxTableFactory {
    // java public static IChpxTable createChpxTable(Document document)
    pub fn createChpxTable(document: *mut dyn Document) -> Option<Box<dyn IChpxTable>> {
      // java return document == null ? null : new ChpxTable(document);
      if document.is_null() {
        None
      } else {
        Some(Box::new(ChpxTable::ChpxTable(document)) as Box<dyn IChpxTable>)
      }
    }

}
