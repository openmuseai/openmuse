#![allow(non_snake_case, dead_code, unused_imports)]
use crate::cn::wps::moffice::writer::core::Document::Document;
use crate::cn::wps::moffice::writer::data::PLC::Node;
use crate::cn::wps::moffice::writer::data::PLC_Char::PLC_Char;

/// Soft usable CommentTxtNode.
#[derive(Clone, Default)]
pub struct CommentTxtNode {
    pub(crate) super_: Node,
}

impl CommentTxtNode {
    pub fn new() -> Self {
        Self::default()
    }
}

/// Soft usable PLCCommentTxt (Document.getCommentTxt).
pub struct PLCCommentTxt {
    pub(crate) super_: PLC_Char,
}

impl PLCCommentTxt {
    pub fn PLCCommentTxt(document: &dyn Document) -> Self {
        Self {
            super_: PLC_Char::PLC_Char(document),
        }
    }
    pub fn size(&self) -> i32 {
        self.super_.size()
    }
}

impl std::ops::Deref for PLCCommentTxt {
    type Target = PLC_Char;
    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for PLCCommentTxt {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}
