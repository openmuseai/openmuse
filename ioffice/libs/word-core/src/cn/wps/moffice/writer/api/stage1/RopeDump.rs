//! Stage1 main-story text dump.
//!
//! There is no Java type `RopeDump`. The I/O is Java
//! `SubDocument.getText(0, getLength(), buf, 0)` via the existing
//! [`CpText::getText`] adapter (rope, not zip `<w:t>`).

use crate::cn::wps::moffice::writer::core::TextDocument::TextDocument;
use crate::cn::wps::moffice::writer::layout::base::text::CpText::CpText;

pub struct RopeDump {}

impl RopeDump {
    /// Java `getText(0, length)` on the main subdocument. Missing main → `""`.
    pub fn dump(doc: &TextDocument) -> String {
        let Some(main) = doc.getMainDocument() else {
            return String::new();
        };
        CpText::getText(main, 0, main.getLength())
    }
}
