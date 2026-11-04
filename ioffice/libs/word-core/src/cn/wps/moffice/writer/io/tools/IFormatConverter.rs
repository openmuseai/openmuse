#![allow(non_snake_case, dead_code, unused_variables, unused_imports)]
//! Soft IFormatConverter — TextDocument is a trait in Document.rs until full struct is mod-wired.

use crate::cn::wps::moffice::writer::core::TextDocument::TextDocument;

// java public interface IFormatConverter
pub trait IFormatConverter {
    // java  void convertToDocX(TextDocument var1)
    fn convertToDocX(&mut self, var1: &dyn SoftTextDocument);

    // java  void convertToDoc(TextDocument var1)
    fn convertToDoc(&mut self, var1: &dyn SoftTextDocument);
}
