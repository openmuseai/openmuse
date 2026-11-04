// Java: import cn.wps.moffice.writer.core.Document
use crate::cn::wps::moffice::writer::core::Document::*;
// Java: import cn.wps.moffice.writer.data.ITextStream
use crate::cn::wps::moffice::writer::data::ITextStream::*;
// Java: import cn.wps.moffice.writer.data.KTextStream
use crate::cn::wps::moffice::writer::data::KTextStream::*;

// java public class TextData
#[derive(Default)]
pub struct TextData {
}

// java public class TextData
impl TextData {
    // java public static ITextStream createTextStream(Document document)
    pub fn createTextStream(document: &dyn Document) -> Box<dyn ITextStream> {
      // java return new KTextStream(document);
      Box::new(KTextStream::KTextStream(document))
    }

}
