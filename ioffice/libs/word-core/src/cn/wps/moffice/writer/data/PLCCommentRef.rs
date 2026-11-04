#![allow(non_snake_case, dead_code, unused_imports)]
use crate::cn::wps::moffice::writer::core::Document::Document;
use crate::cn::wps::moffice::writer::data::PLC::Node;
use crate::cn::wps::moffice::writer::data::PLC_Char::PLC_Char;

/// Soft usable CommentRefNode until full PLCCommentRef java2rust.
#[derive(Clone, Default)]
pub struct CommentRefNode {
    pub(crate) super_: Node,
    pub(crate) author: Option<String>,
    pub(crate) abbreviation: Option<String>,
}

impl CommentRefNode {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn getCP(&self) -> i32 {
        self.super_.getCP()
    }
}

/// Soft usable PLCCommentRef (Document.getCommentRef) — full semantics deferred.
pub struct PLCCommentRef {
    pub(crate) super_: PLC_Char,
}

impl PLCCommentRef {
    pub fn PLCCommentRef(document: &dyn Document) -> Self {
        Self {
            super_: PLC_Char::PLC_Char(document),
        }
    }
    pub fn size(&self) -> i32 {
        self.super_.size()
    }
    pub fn findCommentRefByCP(&mut self, _cp: i32) -> Option<CommentRefNode> {
        None
    }
    pub fn addCommentRef(&mut self, _cp: i32) -> CommentRefNode {
        CommentRefNode::new()
    }
}

impl std::ops::Deref for PLCCommentRef {
    type Target = PLC_Char;
    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for PLCCommentRef {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}
