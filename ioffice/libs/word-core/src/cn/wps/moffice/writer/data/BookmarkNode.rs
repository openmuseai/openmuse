#![allow(non_snake_case, dead_code, unused_imports)]
// Java: import cn.wps.moffice.writer.data.PLC
use crate::cn::wps::moffice::writer::data::PLC::*;
// Java: import cn.wps.moffice.writer.data.PLCCommentRef
use crate::cn::wps::moffice::writer::data::PLCCommentRef::CommentRefNode;
// Java: import cn.wps.moffice.writer.data.BookmarkNodeType
use crate::cn::wps::moffice::writer::data::BookmarkNodeType::*;

// java public abstract class BookmarkNode extends PLC.Node
#[derive(Clone)]
pub struct BookmarkNode {
    // java PLC.Node super_
    pub(crate) super_: Node,
    // java PLCCommentRef.CommentRefNode commentRef;
    pub(crate) commentRef: Option<CommentRefNode>,
}

// java public abstract class BookmarkNode extends PLC.Node
impl BookmarkNode {
    pub fn new() -> Self {
        Self {
            super_: Node::new(),
            commentRef: None,
        }
    }

    // java public abstract BookmarkNodeType getType()
    pub fn getType(&self) -> BookmarkNodeType {
        BookmarkNodeType::BookmarkNodeStart
    }

    // java public abstract long getRange()
    pub fn getRange(&self) -> i64 {
        0
    }

    // java public void setCommentRef(PLCCommentRef.CommentRefNode commentRef)
    pub fn setCommentRef(&mut self, commentRef: Option<CommentRefNode>) {
      // java this.commentRef = commentRef;
      self.commentRef = commentRef;
    }

    // java public PLCCommentRef.CommentRefNode getCommentRef()
    pub fn getCommentRef(&self) -> Option<CommentRefNode> {
      // java return this.commentRef;
      self.commentRef.clone()
    }
}

impl std::ops::Deref for BookmarkNode {
    type Target = Node;
    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for BookmarkNode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}
