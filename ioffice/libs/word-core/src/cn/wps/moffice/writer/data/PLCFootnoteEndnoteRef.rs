#![allow(non_snake_case, dead_code, unused_imports)]
// Java: import cn.wps.moffice.writer.core.Document
use crate::cn::wps::moffice::writer::core::Document::Document;
// Java: import cn.wps.moffice.writer.data.PLC
use crate::cn::wps::moffice::writer::data::PLC::Node;
// Java: import cn.wps.moffice.writer.data.PLC_Char
use crate::cn::wps::moffice::writer::data::PLC_Char::PLC_Char;
// Java: import cn.wps.moffice.writer.data.PLCFootnoteEndnoteTxt
use crate::cn::wps::moffice::writer::data::PLCFootnoteEndnoteTxt::FootnoteEndnoteTxtNode;

// java public class PLCFootnoteEndnoteRef extends PLC_Char
pub struct PLCFootnoteEndnoteRef {
    // java PLC_Char super_
    pub(crate) super_: PLC_Char,
}

impl Default for PLCFootnoteEndnoteRef {
    fn default() -> Self {
        Self { super_: PLC_Char::default() }
    }
}

// java public class PLCFootnoteEndnoteRef extends PLC_Char
impl PLCFootnoteEndnoteRef {
    // java public  PLCFootnoteEndnoteRef(Document document)
    pub fn PLCFootnoteEndnoteRef(document: &dyn Document) -> Self {
      // java super(document);
      Self { super_: PLC_Char::PLC_Char(document) }
    }

    // java public int size()
    pub fn size(&self) -> i32 {
      // java return super.size();
      self.super_.size()
    }

    // java public FootnoteEndnoteRefNode addFootnoteEndnoteRef(int cp)
    pub fn addFootnoteEndnoteRef(&mut self, cp: i32) -> FootnoteEndnoteRefNode {
      // java FootnoteEndnoteRefNode node = new FootnoteEndnoteRefNode();
      let node = FootnoteEndnoteRefNode {
        node: crate::cn::wps::moffice::writer::data::PLC::PLC::alloc_node(),
        numStart: 0,
        footnoteEndnoteTxt: None,
      };
      // java this.insert(cp, node);
      self.super_.insert__int_node(cp, node.node);
      // java return node;
      node
    }

    // java public void doOnRemoveText(int start, int end)
    pub fn doOnRemoveText(&mut self, start: i32, end: i32) {
      // java super.doOnRemoveText(start, end);
      self.super_.doOnRemoveText(start, end);
    }

    // java public void remove(PLC.Node node)
    pub fn remove(&mut self, node: Node) {
      // java this.beforeRemoveNode((FootnoteEndnoteRefNode)node);
      Self::beforeRemoveNode(&mut self.super_, &FootnoteEndnoteRefNode::fromNode(node.clone()));
      // java super.remove(node);
      self.super_.remove(node);
    }

    // java private void beforeRemoveNode(FootnoteEndnoteRefNode pRefNode)
    fn beforeRemoveNode(_self: &mut PLC_Char, _pRefNode: &FootnoteEndnoteRefNode) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java PLCFootnoteEndnoteTxt pTxt = null;
      let mut pTxt: Option<PLCFootnoteEndnoteTxt> = None;
      // java Document pFootnoteEndnoteDoc = this.getDocument();
      let mut pFootnoteEndnoteDoc: Option<&dyn Document> = _self.getDocument();
      // java if (pFootnoteEndnoteDoc.getType() == 1)
      if pFootnoteEndnoteDoc.map_or(false, |d| d.getType() == 1) {
        // java pTxt = pFootnoteEndnoteDoc.getFootnoteTxt();
      }
      // java else
      else {
        // java pTxt = pFootnoteEndnoteDoc.getEndnoteTxt();
      }
      // java PLCFootnoteEndnoteTxt.FootnoteEndnoteTxtNode pTxtNode = pRefNode.getFootnoteEndnoteTxtNode();
      let mut pTxtNode: Option<FootnoteEndnoteTxtNode> = _pRefNode.getFootnoteEndnoteTxtNode().cloned();
      // java int textStartCp = pTxtNode.getCP();
      let mut textStartCp: i32 = pTxtNode.as_ref().map_or(0, |n| n.node.getCP());
      // java if (0 <= textStartCp && textStartCp < pFootnoteEndnoteDoc.getLength())
      if 0 <= textStartCp && textStartCp < pFootnoteEndnoteDoc.map_or(0, |d| d.getLength()) {
        // java long range = pTxt.getFootnoteEndnoteRange(pTxtNode);
        // java pFootnoteEndnoteDoc.getRange(RANGEFactory.getStart(range), RANGEFactory.getEnd(range)).removeText();
      }
      // java else
      else {
        // java pTxt.remove(pTxtNode);
      }
      // rust: ★M-STAGE2 P1-D1 —— 删除引用节点时的联动清理，依赖
      //   `Document.getRange(...).removeText()`（选区子系统）与
      //   `PLCFootnoteEndnoteTxt::remove` 的节点视图一致性，
      //   属**非管线核心**（语料 `<w:footnoteReference>`=0）。按用户指令合法延迟。
      */
    }
}

impl std::ops::Deref for PLCFootnoteEndnoteRef {
    type Target = PLC_Char;
    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for PLCFootnoteEndnoteRef {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}

// java public static class FootnoteEndnoteRefNode extends PLC.Node
#[derive(Clone)]
pub struct FootnoteEndnoteRefNode {
    // java PLC.Node super_
    pub(crate) node: Node,
    // java @AtomMember int numStart;
    pub(crate) numStart: i32,
    // java @AtomMember PLCFootnoteEndnoteTxt.FootnoteEndnoteTxtNode footnoteEndnoteTxt;
    pub(crate) footnoteEndnoteTxt: Option<FootnoteEndnoteTxtNode>,
}

impl Default for FootnoteEndnoteRefNode {
    fn default() -> Self {
        Self { node: Node::new(), numStart: 0, footnoteEndnoteTxt: None }
    }
}

// java public static class FootnoteEndnoteRefNode extends PLC.Node
impl FootnoteEndnoteRefNode {
    // java public  FootnoteEndnoteRefNode()
    pub fn new() -> Self {
        Self::default()
    }

    /// rust: 从既有 `PLC.Node` 重建节点视图（Java 侧是同一对象强转，Rust 值语义需重建）。
    pub fn fromNode(node: Node) -> Self {
        let num_start = if node.ptr.is_null() { 0 } else { unsafe { (*node.ptr).numStart } };
        Self { node, numStart: num_start, footnoteEndnoteTxt: None }
    }

    // java public int getNumStart()
    pub fn getNumStart(&self) -> i32 {
      if self.node.ptr.is_null() {
        return self.numStart;
      }
      unsafe { (*self.node.ptr).numStart }
    }

    // java public void setNumberStart(int numberStart)
    pub fn setNumberStart(&mut self, numberStart: i32) {
      self.numStart = numberStart;
      if !self.node.ptr.is_null() {
        unsafe { (*self.node.ptr).numStart = numberStart; }
      }
    }

    // java public PLCFootnoteEndnoteTxt.FootnoteEndnoteTxtNode getFootnoteEndnoteTxtNode()
    pub fn getFootnoteEndnoteTxtNode(&self) -> Option<&FootnoteEndnoteTxtNode> {
      // java return this.footnoteEndnoteTxt;
      self.footnoteEndnoteTxt.as_ref()
    }

    // java public void setFootnoteEndnoteTxtNode(PLCFootnoteEndnoteTxt.FootnoteEndnoteTxtNode txtNode)
    pub fn setFootnoteEndnoteTxtNode(&mut self, txtNode: Option<FootnoteEndnoteTxtNode>) {
      // java this.footnoteEndnoteTxt = txtNode;
      self.footnoteEndnoteTxt = txtNode;
    }
}
