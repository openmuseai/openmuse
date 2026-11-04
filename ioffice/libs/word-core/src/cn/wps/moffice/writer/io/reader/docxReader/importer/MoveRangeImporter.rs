// Java: package cn.wps.moffice.writer.io.reader.docxReader.importer;
// Java: import cn.wps.moffice.writer.core.TextDocument;
use crate::cn::wps::moffice::writer::core::TextDocument::TextDocument;
// Java: import cn.wps.moffice.writer.data.value.Revision;
use crate::cn::wps::moffice::writer::data::value::Revision::Revision;
// Java: import cn.wps.moffice.writer.io.reader.docxReader.importer.helpers.ConvertHelper;
use crate::cn::wps::moffice::writer::io::reader::docxReader::importer::helpers::ConvertHelper::ConvertHelper;
// Java: import org.xml.sax.Attributes;
use crate::cn::wps::moffice::writer::io::reader::docxReader::importer::helpers::ConvertHelper::Attributes;

// Java: public class MoveRangeImporter
#[derive(Default)]
pub struct MoveRangeImporter {
    // java TextDocument mDocument;
    // rust: ★Java 引用语义 ⇒ 裸指针（与 `SubDocument.mParent` 等既有约定一致）。
    pub mDocument: Option<*mut TextDocument>,
}

impl MoveRangeImporter {
    // java public MoveRangeImporter(TextDocument textDocument)
    pub fn MoveRangeImporter(textDocument: Option<*mut TextDocument>) -> Self {
      // java this.mDocument = textDocument;
      Self { mDocument: textDocument }
    }

    // java public Revision importMoveFromRangeStart(Attributes attributes)
    pub fn importMoveFromRangeStart(&self, attributes: &dyn Attributes) -> Revision {
      // java Revision delReiviosn = ConvertHelper.convertDelRevision(attributes);
      let delReiviosn = ConvertHelper::convertDelRevision(attributes);
      // java return delReiviosn;
      return delReiviosn;
    }

    // java public Revision importMoveToRangeStart(Attributes attributes)
    pub fn importMoveToRangeStart(&self, attributes: &dyn Attributes) -> Revision {
      // java Revision InsRevision = ConvertHelper.convertInsRevision(attributes);
      let InsRevision = ConvertHelper::convertInsRevision(attributes);
      // java return InsRevision;
      return InsRevision;
    }
}
