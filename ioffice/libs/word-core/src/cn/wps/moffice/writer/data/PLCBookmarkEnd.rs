#![allow(non_snake_case, dead_code, unused_imports)]
use crate::cn::wps::moffice::writer::core::Document::Document;
use crate::cn::wps::moffice::writer::data::BookmarkNode::BookmarkNode;
use crate::cn::wps::moffice::writer::data::BookmarkNodeType::BookmarkNodeType;
use crate::cn::wps::moffice::writer::data::PLC_CP::PLC_CP;
use crate::cn::wps::moffice::writer::data::PLCBookmarkStart::BookmarkStartNode;
use crate::cn::wps::moffice::writer::data::RANGEFactory::RANGEFactory;

/// Soft usable BookmarkEndNode.
#[derive(Clone)]
pub struct BookmarkEndNode {
    pub(crate) super_: BookmarkNode,
    // java PLCBookmarkStart.BookmarkStartNode start;
    pub(crate) start: Option<BookmarkStartNode>,
}

impl Default for BookmarkEndNode {
    fn default() -> Self {
        Self {
            super_: BookmarkNode::new(),
            start: None,
        }
    }
}

impl BookmarkEndNode {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn getType(&self) -> BookmarkNodeType {
        // java return null == this.commentRef ? BookmarkNodeType.BookmarkNodeEnd : BookmarkNodeType.CommentNodeEnd;
        // rust: ★M-STAGE2 P1-D1 —— 原实现恒返回 `BookmarkNodeEnd`，与 Java 的三元判定不一致。
        if self.super_.commentRef.is_none() {
            BookmarkNodeType::BookmarkNodeEnd
        } else {
            BookmarkNodeType::CommentNodeEnd
        }
    }
    pub fn getCP(&self) -> i32 {
        self.super_.getCP()
    }

    // java public PLCBookmarkStart.BookmarkStartNode getStart()
    pub fn getStart(&self) -> Option<&BookmarkStartNode> {
        // java return this.start;
        self.start.as_ref()
    }

    // java public void setStart(PLCBookmarkStart.BookmarkStartNode start)
    pub fn setStart(&mut self, start: Option<BookmarkStartNode>) {
        // java this.start = start;
        self.start = start;
    }

    // java public long getRange()
    pub fn getRange(&self) -> i64 {
        // java return null == this.commentRef ? RANGEFactory.getRANGE(this.start.position(), this.position()) : RANGEFactory.getRANGE(this.start.position(), this.commentRef.position() + 1);
        let startPos = self.start.as_ref().map_or(0, |n| n.super_.super_.position());
        match self.super_.commentRef.as_ref() {
            None => RANGEFactory::getRANGE(startPos, self.super_.super_.position()),
            Some(cr) => RANGEFactory::getRANGE(startPos, cr.super_.position() + 1),
        }
    }

    // java public void copyReplyCmt(BookmarkEndNode node)
    pub fn copyReplyCmt(&mut self, node: &BookmarkEndNode) {
        // java int cp = node.position();
        let cp: i32 = node.super_.super_.position();
        // java this.moveRight(cp - this.position());
        // rust: `BookmarkEndNode extends PLC.Node`，故 `this.moveRight(int)` 解析到
        //   `ListIndexNode.moveRight(int steps)`（Java 原文为空实现），
        //   而非外层 `PLC.moveRight(Node, int)`。
        let delta = cp - self.super_.super_.position();
        self.super_.super_.moveRight(delta);
    }
}

/// Soft usable PLCBookmarkEnd (Document.getBookmarkEnds / getCommentEnds).
pub struct PLCBookmarkEnd {
    pub(crate) super_: PLC_CP,
}

impl PLCBookmarkEnd {
    pub fn PLCBookmarkEnd(document: &dyn Document) -> Self {
        Self {
            super_: PLC_CP::PLC_CP(document),
        }
    }
    pub fn size(&self) -> i32 {
        self.super_.size()
    }
    pub fn removeBookmarkEnd(&mut self, _node: &BookmarkEndNode) {}

    // rust: ★M-STAGE2 P1-D1 —— 复用 `PLC::insertAtUpperBound(cp, Node)`（`PLC.rs:231`）。
    pub fn insertAtUpperBound(&mut self, cp: i32, node: &BookmarkEndNode) {
        // rust: 该调用由 Java 的 `addBookmark*` 发起，`insertAtUpperBound` 本身继承自 `PLC`；
        //   本文件无对应 Java 方法注释，故不构成 §3.8 方法区。
        self.super_.insertAtUpperBound(cp, node.super_.super_.clone());
    }
}

impl std::ops::Deref for PLCBookmarkEnd {
    type Target = PLC_CP;
    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for PLCBookmarkEnd {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}
