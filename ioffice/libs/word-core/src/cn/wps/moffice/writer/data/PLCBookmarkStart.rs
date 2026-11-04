#![allow(non_snake_case, dead_code, unused_imports)]
use crate::cn::wps::moffice::writer::core::Document::Document;
use crate::cn::wps::moffice::writer::data::BookmarkNode::BookmarkNode;
use crate::cn::wps::moffice::writer::data::BookmarkNodeType::BookmarkNodeType;
use crate::cn::wps::moffice::writer::data::PLCBookmarkEnd::BookmarkEndNode;
use crate::cn::wps::moffice::writer::data::PLC_CP::PLC_CP;
use crate::cn::wps::moffice::writer::data::RANGEFactory::RANGEFactory;

/// Soft usable BookmarkStartNode.
#[derive(Clone)]
pub struct BookmarkStartNode {
    pub(crate) super_: BookmarkNode,
    pub(crate) name: Option<String>,
    // rust: Java 侧 `end` 与 `BookmarkEndNode.start` 互为引用；Rust 值语义会形成
    //   无限大小类型 ⇒ 本侧用 `Box` 打破环（数据等价，仅所有权形式不同）。
    pub(crate) end: Option<Box<BookmarkEndNode>>,
    // java boolean isInTable;
    pub(crate) isInTable: bool,
    // java boolean isNature;
    pub(crate) isNature: bool,
    // java Integer itcFirst;
    pub(crate) itcFirst: Option<i32>,
    // java Integer itcLim;
    pub(crate) itcLim: Option<i32>,
}

impl Default for BookmarkStartNode {
    fn default() -> Self {
        Self {
            super_: BookmarkNode::new(),
            name: None,
            end: None,
            isInTable: false,
            isNature: false,
            itcFirst: None,
            itcLim: None,
        }
    }
}

impl BookmarkStartNode {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn getName(&self) -> Option<&str> {
        self.name.as_deref()
    }
    pub fn setName(&mut self, name: Option<String>) {
        self.name = name;
    }
    pub fn getType(&self) -> BookmarkNodeType {
        // java return null != this.name ? BookmarkNodeType.BookmarkNodeStart : BookmarkNodeType.CommentNodeStart;
        // rust: ★M-STAGE2 P1-D1 —— 原实现恒返回 `BookmarkNodeStart`，与 Java 的
        //   三元判定不一致（批注起始节点应为 `CommentNodeStart`）。
        if self.name.is_some() {
            BookmarkNodeType::BookmarkNodeStart
        } else {
            BookmarkNodeType::CommentNodeStart
        }
    }
    pub fn getEnd(&self) -> Option<&BookmarkEndNode> {
        // rust: 字段为 `Option<Box<..>>`（打破循环），按引用返回时解 Box。
        self.end.as_deref()
    }

    // java public void setEnd(PLCBookmarkEnd.BookmarkEndNode end)
    pub fn setEnd(&mut self, end: Option<BookmarkEndNode>) {
        // java this.end = end;
        self.end = end.map(Box::new);
    }

    // java public long getRange()
    pub fn getRange(&self) -> i64 {
        // java return null == this.commentRef ? RANGEFactory.getRANGE(this.position(), this.end.position()) : RANGEFactory.getRANGE(this.position(), this.commentRef.position() + 1);
        match self.super_.commentRef.as_ref() {
            None => RANGEFactory::getRANGE(
                self.super_.super_.position(),
                self.end.as_ref().map_or(0, |e| e.super_.super_.position()),
            ),
            Some(cr) => RANGEFactory::getRANGE(
                self.super_.super_.position(),
                cr.super_.position() + 1,
            ),
        }
    }

    // java public boolean getInTable()
    pub fn getInTable(&self) -> bool {
        // java return this.isInTable;
        self.isInTable
    }

    // java public void setInTable(boolean isInTable)
    pub fn setInTable(&mut self, isInTable: bool) {
        // java this.isInTable = isInTable;
        self.isInTable = isInTable;
    }

    // java public Integer getItcFirst()
    pub fn getItcFirst(&self) -> Option<i32> {
        // java return null != this.itcFirst ? this.itcFirst : null;
        self.itcFirst
    }

    // java public void setItcFirst(int itcFirst)
    pub fn setItcFirst(&mut self, itcFirst: i32) {
        // java this.itcFirst = itcFirst;
        self.itcFirst = Some(itcFirst);
    }

    // java public Integer getItcLim()
    pub fn getItcLim(&self) -> Option<i32> {
        // java return this.itcLim;
        self.itcLim
    }

    // java public void setItcLim(int itcLim)
    pub fn setItcLim(&mut self, itcLim: i32) {
        // java this.itcLim = itcLim;
        self.itcLim = Some(itcLim);
    }

    // java public boolean getNature()
    pub fn getNature(&self) -> bool {
        // java return this.isNature;
        self.isNature
    }

    // java public void setNature(boolean isNature)
    pub fn setNature(&mut self, isNature: bool) {
        // java this.isNature = isNature;
        self.isNature = isNature;
    }

    // java public BookmarkStartNode copyReplyCmt(BookmarkStartNode src)
    pub fn copyReplyCmt(&mut self, src: &BookmarkStartNode) -> BookmarkStartNode {
        // java this.commentRef = src.commentRef;
        self.super_.commentRef = src.super_.commentRef.clone();
        // java this.isInTable = src.isInTable;
        self.isInTable = src.isInTable;
        // java this.isNature = src.isNature;
        self.isNature = src.isNature;
        // java this.itcFirst = src.itcFirst;
        self.itcFirst = src.itcFirst;
        // java this.itcLim = src.itcLim;
        self.itcLim = src.itcLim;
        // java this.name = src.name;
        self.name = src.name.clone();
        // java return this;
        return self.clone();
    }
}

/// Soft usable PLCBookmarkStart (Document.getBookmarkStarts / getCommentStarts).
pub struct PLCBookmarkStart {
    pub(crate) super_: PLC_CP,
}

impl PLCBookmarkStart {
    pub fn PLCBookmarkStart(document: &dyn Document) -> Self {
        Self {
            super_: PLC_CP::PLC_CP(document),
        }
    }
    pub fn size(&self) -> i32 {
        self.super_.size()
    }
    pub fn addBookmarkStart(&mut self, _cp: i32) -> BookmarkStartNode {
        BookmarkStartNode::new()
    }
    pub fn removeBookmarkStart(&mut self, _node: &BookmarkStartNode) {}

    // rust: ★M-STAGE2 P1-D1 —— 复用 `PLC::insertAtUpperBound(cp, Node)`（`PLC.rs:231`），
    //   节点以 `PLC.Node` 形式入 PLC（与 `PLCSection` 的既有做法一致）。
    pub fn insertAtUpperBound(&mut self, cp: i32, node: &BookmarkStartNode) {
        // rust: 该调用由 Java 的 `addBookmark*` 发起，`insertAtUpperBound` 本身继承自 `PLC`；
        //   本文件无对应 Java 方法注释，故不构成 §3.8 方法区。
        self.super_.insertAtUpperBound(cp, node.super_.super_.clone());
    }
}

impl std::ops::Deref for PLCBookmarkStart {
    type Target = PLC_CP;
    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for PLCBookmarkStart {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}
