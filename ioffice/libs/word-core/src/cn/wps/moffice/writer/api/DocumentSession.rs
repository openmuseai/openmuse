//! Heap-stable edit session: hold [`Box<TextDocument>`], mutate, then relayout.
//!
//! Do not move the document after open (SAX `SubDocument.mParent`). FRB should
//! keep this type on the native side; bytes-in `layout_document` remains the
//! one-shot paint path.

use util::moffice::property::PropertySet::empty;

use crate::cn::wps::moffice::writer::core::TextDocument::TextDocument;
use crate::cn::wps::moffice::writer::helper::CoreHelper::CoreHelper;

use super::LayoutDocument::{
    layout_text_document, open_text_document, FontBlob, LayoutError, LayoutFrame, ViewParams,
};
use super::StyleSnapshot::{from_docx, StyleSnapshot};
use super::WordStatistics::{count_words, text_statistics, WordTextStatistics};

/// Opened `.docx` that outlives a single layout call.
pub struct DocumentSession {
    doc: Box<TextDocument>,
    style_snapshot: StyleSnapshot,
}

impl DocumentSession {
    pub fn open(docx: &[u8]) -> Result<Self, LayoutError> {
        Ok(Self {
            doc: open_text_document(docx)?,
            style_snapshot: from_docx(docx),
        })
    }

    pub fn document(&self) -> &TextDocument {
        &self.doc
    }

    pub fn style_snapshot(&self) -> &StyleSnapshot {
        &self.style_snapshot
    }

    pub fn document_mut(&mut self) -> &mut TextDocument {
        &mut self.doc
    }

    pub fn main_length(&self) -> i32 {
        self.doc
            .getMainDocument()
            .map(|m| m.getLength())
            .unwrap_or(0)
    }

    pub fn word_count(&self, start: i32, end: i32) -> i32 {
        count_words(&self.doc, start, end)
    }

    pub fn text_statistics(&self) -> WordTextStatistics {
        text_statistics(&self.doc, 0, self.main_length())
    }

    pub fn layout(
        &self,
        fonts: &[FontBlob],
        view: &ViewParams,
    ) -> Result<LayoutFrame, LayoutError> {
        layout_text_document(&self.doc, fonts, view)
    }

    /// Append a paragraph (text + `\\r`) with empty CHPX/PAPX.
    pub fn append_paragraph(&mut self, text: &str) -> Result<(), LayoutError> {
        let main = self
            .doc
            .getMainDocumentMut()
            .ok_or_else(|| LayoutError::zip("no main subdocument"))?;
        let start = main.getLength();
        let mut s = text.to_string();
        s.push('\r');
        CoreHelper::appendText__string_ps(Some(main), Some(&s), Some(&*empty));
        CoreHelper::insertToPapxTable(Some(main), start, Some(&*empty));
        Ok(())
    }
}
