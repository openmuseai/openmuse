#![allow(non_snake_case, dead_code, unused_variables)]
use crate::cn::wps::moffice::writer::core::Document::Document;

#[derive(Default, Clone)]
pub struct KComments {}

#[derive(Default, Clone)]
pub struct KReplyComments {}

pub struct CommentInfo;

impl CommentInfo {
    pub fn isInvisibaleCP(_subDoc: &dyn Document, _pos: i32) -> bool {
        false
    }
}
