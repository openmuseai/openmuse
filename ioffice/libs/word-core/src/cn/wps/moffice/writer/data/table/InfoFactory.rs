// Java: import cn.wps.moffice.writer.core.Document
use crate::cn::wps::moffice::writer::core::Document::*;
// Java: import cn.wps.moffice.writer.data.table.TableInfos
use crate::cn::wps::moffice::writer::data::table::TableInfos::*;
// Java: import cn.wps.moffice.writer.data.table.TableInfosImpl
use crate::cn::wps::moffice::writer::data::table::TableInfosImpl::*;

// java public class InfoFactory
#[derive(Default)]
pub struct InfoFactory {
}

// java public class InfoFactory
impl InfoFactory {
    // java public static TableInfos createTableInfos(Document document)
    pub fn createTableInfos(document: &dyn Document) -> Box<dyn TableInfos> {
      // java return new TableInfosImpl(document);
      Box::new(TableInfosImpl::TableInfosImpl(document))
    }

}
