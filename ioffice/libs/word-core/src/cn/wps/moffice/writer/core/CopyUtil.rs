#![allow(non_snake_case, dead_code, unused_variables)]
use crate::cn::wps::moffice::writer::core::Document::Document;
use crate::cn::wps::moffice::writer::core::SubDocument::SubDocument;

/// Soft compile shell for SubDocument.doCopy until CopyUtil.java is rewritten (0b).
pub struct CopyUtil;

impl CopyUtil {
    pub fn copyComments(_document: &dyn Document, _srcCP: i32, _endCP: i32, _distance: i32) {}

    pub fn copyFootnoteEndnote(_document: &dyn Document, _srcCP: i32, _endCP: i32, _distance: i32) {}

    pub fn copyBookmarks(_document: &dyn Document, _srcCP: i32, _endCP: i32, _distance: i32) {}

    pub fn copyShapes(_document: &mut SubDocument, _srcCP: i32, _endCP: i32, _distance: i32) {}

    pub fn copyMath(_document: &dyn Document, _srcCP: i32, _endCP: i32, _distance: i32) {}
}
