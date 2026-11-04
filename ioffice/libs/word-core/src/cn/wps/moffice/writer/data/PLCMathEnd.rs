// Java: import cn.wps.moffice.transaction.AtomMember
use util::moffice::transaction::AtomMember::*;
// Java: import cn.wps.moffice.writer.core.Document
use crate::cn::wps::moffice::writer::core::Document::*;
// Java: import cn.wps.moffice.writer.data.PLC_CP
use crate::cn::wps::moffice::writer::data::PLC_CP::*;
// Java: import cn.wps.moffice.writer.data.PLC
use crate::cn::wps::moffice::writer::data::PLC::*;
// Java: import cn.wps.io.dom.Node
// (generator mis-import; use PLC::Node)
// Java: import cn.wps.moffice.writer.data.PLCMathStart
use crate::cn::wps::moffice::writer::data::PLCMathStart::{alloc_omath, heap_omath, load_end, OMathStartNode, PLCMathStart};

// java public class PLCMathEnd extends PLC_CP
pub struct PLCMathEnd {
    // java PLC_CP super_
    pub(crate) super_: PLC_CP,
    // java boolean encloseInsertedChar = false;
    pub(crate) encloseInsertedChar: bool,
}

impl Default for PLCMathEnd {
    fn default() -> Self {
        // Document-less default for derive compatibility; real ctor takes Document
        Self {
            super_: PLC_CP {
                super_: PLC::new(None),
            },
            encloseInsertedChar: false,
        }
    }
}

// java public static class OMathEndNode extends PLC.Node
#[derive(Default, Clone)]
pub struct OMathEndNode {
    // java PLC.Node super_
    pub(crate) super_: Node,
    // java @AtomMember PLCMathStart.OMathStartNode start;
    // rust: Java 侧 `OMathEndNode.start` ↔ `OMathStartNode.end` 互为引用；
    //   Rust 值语义构成无限大小类型 ⇒ 本侧用 `Box` 打破环。
    pub(crate) start: Option<Box<OMathStartNode>>,
}

// java public class PLCMathEnd extends PLC_CP
impl PLCMathEnd {
    // java public  PLCMathEnd(Document document)
    pub fn PLCMathEnd(document: &dyn Document) -> Self {
      // java super(document);
      Self { super_: PLC_CP::PLC_CP(document), encloseInsertedChar: false }
    }

    // java public OMathEndNode findEndByCP(int cp)
    pub fn findEndByCP(&mut self, cp: i32) -> Option<OMathEndNode> {
      // java PLC.Node node = this.find(cp);
      let node: Node = self.super_.find(cp);
      // java return node == this.head ? null : (OMathEndNode)node;
      if self.super_.isHead(node) {
        return None;
      }
      return Some(load_end(node));
    }

    // java public OMathEndNode addOMathEnd(int cp)
    pub fn addOMathEnd(&mut self, cp: i32) -> OMathEndNode {
      // java OMathEndNode node = new OMathEndNode(); this.insert(cp, node);
      let node = alloc_omath(2);
      self.super_.insert__int_node(cp, node);
      load_end(node)
    }

    // java public void removeOMathEnd(OMathEndNode node)
    pub fn removeOMathEnd(&mut self, node: &mut OMathEndNode) {
      // java this.remove(node);
      self.super_.remove(node.super_);
    }

    // java public boolean isEncloseInsertedChar()
    pub fn isEncloseInsertedChar(&self) -> bool {
      // java return this.encloseInsertedChar;
      return self.encloseInsertedChar;
    }

    // java public void setEncloseInsertedChar(boolean encloseInsertedChar)
    pub fn setEncloseInsertedChar(&mut self, encloseInsertedChar: bool) {
      // java this.encloseInsertedChar = encloseInsertedChar;
      self.encloseInsertedChar = encloseInsertedChar;
    }

    // java public void doOnInsertText(int start, int end)
    pub fn doOnInsertText(&mut self, start: i32, end: i32) {
      // java if (this.encloseInsertedChar)
      if self.encloseInsertedChar {
        // java PLC.Node iter = this.lowerBound(start);
        let iter: Node = self.super_.lowerBound__int(start);
        // java this.insertSpaceBefore(iter, end - start);
        self.super_.insertSpaceBefore(iter, end - start);
      }
      // java else
      else {
        // java super.doOnInsertText(start, end);
        self.super_.doOnInsertText(start, end);
      }
    }

}

impl std::ops::Deref for PLCMathEnd {
    type Target = PLC_CP;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for PLCMathEnd {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}

// java public static class OMathEndNode extends PLC.Node
impl OMathEndNode {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn getCP(&self) -> i32 {
        self.super_.getCP()
    }

    // java public PLCMathStart.OMathStartNode getStart()
    pub fn getStart(&self) -> Option<&OMathStartNode> {
      // java return this.start;
      return self.start.as_deref();
    }

    // java public void setStart(PLCMathStart.OMathStartNode start)
    pub fn setStart(&mut self, start: Option<OMathStartNode>) {
      // java this.start = start;
      if let Some(heap) = heap_omath(self.super_) {
        let other = start.as_ref().map(|n| n.super_).unwrap_or(Node::default());
        unsafe { (*heap).other = other; }
        if let Some(start_heap) = heap_omath(other) {
          unsafe { (*start_heap).other = self.super_; }
        }
      }
      self.start = start.map(Box::new);
    }

}

impl std::ops::Deref for OMathEndNode {
    type Target = Node;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for OMathEndNode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}
