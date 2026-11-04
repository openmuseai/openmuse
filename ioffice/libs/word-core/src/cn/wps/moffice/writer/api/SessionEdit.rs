//! Apply insert/remove on an open [`DocumentSession`], then full relayout.
//!
//! Mutate `TextDocument` storage first; never rewrite `DrawText.text` on a Frame.
//! Incremental `CoreThread` cache is out of scope (wr-20).

use util::moffice::property::PropertySet::empty;

use crate::cn::wps::collections::textrope::ITextRope::ITextRope;
use crate::cn::wps::moffice::writer::core::Document::Document;
use crate::cn::wps::moffice::writer::core::DocumentEvent::DocumentEvent;
use crate::cn::wps::moffice::writer::data::IChpxTable::IChpxTable;

use super::DocumentSession::DocumentSession;
use super::LayoutDocument::{LayoutError, LayoutFrame, FontBlob, ViewParams};

pub struct SessionEdit;

impl SessionEdit {
    /// Insert UTF-8 text at `cp` (Unicode scalars). Empty text is a no-op INSERT of length 0.
    pub fn insert(
        session: &mut DocumentSession,
        cp: i32,
        text: &str,
    ) -> Result<DocumentEvent, LayoutError> {
        let main = session
            .document_mut()
            .getMainDocumentMut()
            .ok_or_else(|| LayoutError::zip("no main subdocument"))?;
        let len = main.getLength();
        if cp < 0 || cp > len {
            return Err(LayoutError::zip("insert cp out of range"));
        }
        let n = text.chars().count() as i32;
        if n == 0 {
            return Ok(DocumentEvent::insert_at(cp, 0));
        }
        let textRope = main.getTextRope();
        if let Some(r) = textRope {
            if !r.is_null() {
                unsafe { (*r).insert__int_string_int_int(cp, text, 0, n) };
            }
        }
        if let Some(t) = Document::getChpxTable(main) {
            if !t.is_null() {
                unsafe {
                    IChpxTable::insert(&mut *t, cp, Some(&*empty));
                }
            }
        }
        Ok(DocumentEvent::insert_at(cp, n))
    }

    /// Erase half-open `[start, end)` on the main rope. Empty interval is a no-op REMOVE of length 0.
    pub fn remove(
        session: &mut DocumentSession,
        start: i32,
        end: i32,
    ) -> Result<DocumentEvent, LayoutError> {
        let main = session
            .document_mut()
            .getMainDocumentMut()
            .ok_or_else(|| LayoutError::zip("no main subdocument"))?;
        let len = main.getLength();
        if start < 0 || end < 0 || start > len || end > len {
            return Err(LayoutError::zip("remove range out of bounds"));
        }
        if start >= end {
            return Ok(DocumentEvent::remove_at(start, 0));
        }
        let textRope = main.getTextRope();
        if let Some(r) = textRope {
            if !r.is_null() {
                unsafe { (*r).erase(start, end) };
            }
        }
        Ok(DocumentEvent::remove_at(start, end - start))
    }

    /// Full layout after storage mutation (not incremental cache).
    pub fn relayout(
        session: &DocumentSession,
        fonts: &[FontBlob],
        view: &ViewParams,
    ) -> Result<LayoutFrame, LayoutError> {
        session.layout(fonts, view)
    }
}
