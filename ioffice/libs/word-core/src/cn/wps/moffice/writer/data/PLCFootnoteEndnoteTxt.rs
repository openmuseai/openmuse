#![allow(non_snake_case, dead_code, unused_imports)]
// Java: import cn.wps.moffice.writer.core.Document
use crate::cn::wps::moffice::writer::core::Document::Document;
// Java: import cn.wps.moffice.writer.data.PLC
use crate::cn::wps::moffice::writer::data::PLC::Node;
// Java: import cn.wps.moffice.writer.data.PLC_Range
use crate::cn::wps::moffice::writer::data::PLC_Range::PLC_Range;
// Java: import cn.wps.moffice.writer.data.PLCFootnoteEndnoteRef
use crate::cn::wps::moffice::writer::data::PLCFootnoteEndnoteRef::FootnoteEndnoteRefNode;
// Java: import cn.wps.moffice.writer.data.RANGEFactory
use crate::cn::wps::moffice::writer::data::RANGEFactory::RANGEFactory;

// java public class PLCFootnoteEndnoteTxt extends PLC_Range
pub struct PLCFootnoteEndnoteTxt {
    // java PLC_Range super_
    pub(crate) super_: PLC_Range,
    // rust: ★M-STAGE2 P1-D1 —— 本字段为 **Rust 侧既有扩展**（Java 无对应）：
    //   `SubDocument::footnoteInvalidCount` / `FinishReadingWork` 依赖它。
    //   保留以免破坏既有行为。
    pub invalid_nodes: i32,
}

// rust: ★M-STAGE2 P1-D1 —— Rust 侧 `SubDocument` 字段初始化 / `FinishReadingWork`
//   需要 `Default`；以 `PLC_Range::default()` 构造。
impl Default for PLCFootnoteEndnoteTxt {
    fn default() -> Self {
        Self { super_: PLC_Range::default(), invalid_nodes: 0 }
    }
}

// java public class PLCFootnoteEndnoteTxt extends PLC_Range
impl PLCFootnoteEndnoteTxt {
    // java public  PLCFootnoteEndnoteTxt(Document document)
    pub fn PLCFootnoteEndnoteTxt(document: &dyn Document) -> Self {
      // java super(document);
      Self { super_: PLC_Range::PLC_Range(document), invalid_nodes: 0 }
    }

    // java public int size()
    pub fn size(&self) -> i32 {
      // java return super.size();
      self.super_.size()
    }

    // rust: ★M-STAGE2 P1-D1 —— Rust 侧既有扩展（Java 无），保留。
    pub fn removeInvalids(&mut self) {
        self.invalid_nodes = 0;
    }

    // java private PLC.Node _findFloor(int cp)
    fn _findFloor(&self, cp: i32) -> Option<Node> {
      // java PLC.Node pLowerBound = this.lowerBound(cp);
      let mut pLowerBound: Node = self.super_.lowerBound__int(cp);
      // java if (this.isHead(pLowerBound))
      if self.super_.isHead(pLowerBound.clone()) {
        // java if (this.size() > 0)
        if self.super_.size() > 0 {
          // java pLowerBound = pLowerBound.getPrev();
          pLowerBound = pLowerBound.getPrev();
          // java return pLowerBound;
          return Some(pLowerBound);
        }
        // java else
        else {
          // java return null;
          return None;
        }
      }
      // java else if (pLowerBound.position() != cp)
      else if pLowerBound.position() != cp {
        // java PLC.Node pPrev = pLowerBound.getPrev();
        let mut pPrev: Node = pLowerBound.getPrev();
        // java return this.isHead(pPrev) ? null : pPrev;
        return if self.super_.isHead(pPrev.clone()) { None } else { Some(pPrev) };
      }
      // java else
      else {
        // java return pLowerBound;
        return Some(pLowerBound);
      }
    }

    // java public FootnoteEndnoteTxtNode findFootnoteEndnoteTxtByCP(int cp)
    pub fn findFootnoteEndnoteTxtByCP(&self, cp: i32) -> Option<FootnoteEndnoteTxtNode> {
      // java PLC.Node node = this.findUpper(cp);
      let mut node: Node = self.super_.findUpper(cp);
      // java return node != this.head ? (FootnoteEndnoteTxtNode)node : null;
      if !self.super_.isHead(node.clone()) {
        Some(FootnoteEndnoteTxtNode::fromNode(node))
      } else {
        None
      }
    }

    // java public FootnoteEndnoteTxtNode findFootnoteEndnoteTxtContainsCP(int cp)
    pub fn findFootnoteEndnoteTxtContainsCP(&self, cp: i32) -> Option<FootnoteEndnoteTxtNode> {
      // java PLC.Node node = this._findFloor(cp);
      let mut node: Option<Node> = self._findFloor(cp);
      // java return node != null && node != this.head ? (FootnoteEndnoteTxtNode)node : null;
      match node {
        Some(n) if !self.super_.isHead(n.clone()) => Some(FootnoteEndnoteTxtNode::fromNode(n)),
        _ => None,
      }
    }

    // java public FootnoteEndnoteTxtNode addFootnoteEndnoteTxt(int cp)
    pub fn addFootnoteEndnoteTxt(&mut self, cp: i32) -> FootnoteEndnoteTxtNode {
      // java FootnoteEndnoteTxtNode node = new FootnoteEndnoteTxtNode();
      let mut node: FootnoteEndnoteTxtNode = FootnoteEndnoteTxtNode::new();
      // java this.insert(cp, node);
      self.super_.insert__int_node(cp, node.node.clone());
      // java return node;
      return node;
    }

    // java public void removeFootnoteEndnoteTxt(FootnoteEndnoteTxtNode node)
    pub fn removeFootnoteEndnoteTxt(&mut self, node: &FootnoteEndnoteTxtNode) {
      // java this.remove(node);
      self.super_.remove(node.node.clone());
    }

    // java public long getFootnoteEndnoteRange(FootnoteEndnoteTxtNode node)
    pub fn getFootnoteEndnoteRange(&self, node: &FootnoteEndnoteTxtNode) -> i64 {
      // java int start = node.position();
      let mut start: i32 = node.node.position();
      // java PLC.Node next = node.getNext();
      let mut next: Node = node.node.getNext();
      // java int end;
      let mut end: i32;
      // java if (this.isHead(next))
      if self.super_.isHead(next.clone()) {
        // java end = this.getDocument().getLength();
        end = self.super_.getDocument().map_or(0, |d| d.getLength());
      }
      // java else
      else {
        // java end = next.position();
        end = next.position();
      }
      // java return RANGEFactory.getRANGE(start, end);
      return RANGEFactory::getRANGE(start, end);
    }

    // java public long getRange(PLC.Node node)
    pub fn getRange(&self, node: &FootnoteEndnoteTxtNode) -> i64 {
      self.getFootnoteEndnoteRange(node)
    }
}

// java public class FootnoteEndnoteTxtNode extends PLC.Node
#[derive(Clone)]
pub struct FootnoteEndnoteTxtNode {
    // java PLC.Node super_
    pub(crate) node: Node,
    // java @AtomMember PLCFootnoteEndnoteRef.FootnoteEndnoteRefNode footnoteEndnoteRef;
    // rust: Java 侧本字段与 `FootnoteEndnoteRefNode.footnoteEndnoteTxt` 互为引用；
    //   Rust 值语义会构成无限大小类型 ⇒ 本侧用 `Box` 打破环。
    pub(crate) footnoteEndnoteRef: Option<Box<FootnoteEndnoteRefNode>>,
}

impl Default for FootnoteEndnoteTxtNode {
    fn default() -> Self {
        Self { node: Node::new(), footnoteEndnoteRef: None }
    }
}

// java public class FootnoteEndnoteTxtNode extends PLC.Node
impl FootnoteEndnoteTxtNode {
    // java public  FootnoteEndnoteTxtNode()
    pub fn new() -> Self {
        Self::default()
    }

    /// rust: 从既有 `PLC.Node` 重建节点视图（Java 侧是同一对象强转，Rust 值语义需重建）。
    pub fn fromNode(node: Node) -> Self {
        Self { node, footnoteEndnoteRef: None }
    }

    // java public PLCFootnoteEndnoteRef.FootnoteEndnoteRefNode getFootnoteEndnoteRef()
    pub fn getFootnoteEndnoteRef(&self) -> Option<&FootnoteEndnoteRefNode> {
      // java return this.footnoteEndnoteRef;
      self.footnoteEndnoteRef.as_deref()
    }

    // java public void setFootnoteEndnoteRef(PLCFootnoteEndnoteRef.FootnoteEndnoteRefNode footnoteEndnoteRef)
    pub fn setFootnoteEndnoteRef(&mut self, footnoteEndnoteRef: Option<FootnoteEndnoteRefNode>) {
      // java PLCFootnoteEndnoteRef.FootnoteEndnoteRefNode pOldData = this.footnoteEndnoteRef;
      let mut pOldData: Option<Box<FootnoteEndnoteRefNode>> = self.footnoteEndnoteRef.take();
      // java this.footnoteEndnoteRef = footnoteEndnoteRef;
      self.footnoteEndnoteRef = footnoteEndnoteRef.map(Box::new);
      // java Transaction t = PLCFootnoteEndnoteTxt.this.getTransaction();
      // java if (t != null && t.getState() == 1)
      // java t.submitCommand(new setFootnoteEndnoteRef(pOldData));
      // rust: no-stmt —— 事务命令回放属**非管线核心**逻辑，Java 侧仅在事务
      //   state==1 时触发；字段赋值（本方法的真实效果）已在上方完成。
      let _ = pOldData;
    }
}

impl std::ops::Deref for PLCFootnoteEndnoteTxt {
    type Target = PLC_Range;
    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for PLCFootnoteEndnoteTxt {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}
