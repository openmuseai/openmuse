// Java: import cn.wps.moffice.writer.status.IStatus
use crate::cn::wps::moffice::writer::status::IStatus::*;
// Java: import cn.wps.moffice.writer.core.Document
use crate::cn::wps::moffice::writer::core::Document::*;

// java public interface IWriterModelStatus
pub trait IWriterModelStatus {
    // java  Document getDocument()
    fn getDocument(&self) -> &(dyn Document);

}
