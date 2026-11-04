// Java: import cn.wps.moffice.writer.data.CPieceTable
use crate::cn::wps::moffice::writer::data::CPieceTable::*;
// Java: import cn.wps.moffice.writer.core.Document
use crate::cn::wps::moffice::writer::core::Document::*;
// Java: import cn.wps.moffice.writer.core.TextDocument
use crate::cn::wps::moffice::writer::core::TextDocument::*;

// java public class KRangeBase
#[derive(Default)]
pub struct KRangeBase {
    // java protected Document document;
    pub(crate) document: *mut dyn Document,
    // java protected CPieceTable mCPieceTable;
    pub(crate) mCPieceTable: Option<CPieceTable>,
}

// java public class KRangeBase
impl KRangeBase {
    // java public Document getSubDocument()
    pub fn getSubDocument(&self) -> Box<dyn Document> {
      // java return this.document;
      return self.document;
    }

    // java public final Document getStory()
    pub fn getStory(&self) -> Box<dyn Document> {
      // java return this.document;
      return self.document;
    }

    // java public final int getStoryType()
    pub fn getStoryType(&self) -> i32 {
      // java return this.getStory().getType();
      return self.getStory().getType();
    }

    // java public final int getStoryLength()
    pub fn getStoryLength(&self) -> i32 {
      // java return this.getStory().getLength();
      return self.getStory().getLength();
    }

    // java public final Document getMainDocument()
    pub fn getMainDocument(&self) -> Box<dyn Document> {
      // java return this.document.getMainDocument();
      return self.document.getMainDocument();
    }

    // java public final TextDocument getTextDocument()
    pub fn getTextDocument(&self) -> TextDocument {
      // java return this.document.getTextDocument();
      return self.document.getTextDocument();
    }

    // java public final void setSubDocument(Document document)
    pub fn setSubDocument(&mut self, document: Option<&dyn Document>) {
      // java this.document = document;
      self.document = document;
      // java if (document != null)
      if let Some(document) = document {
        // java this.mCPieceTable = document.getPieceTable();
        self.mCPieceTable = document.getPieceTable();
      }
    }

}
