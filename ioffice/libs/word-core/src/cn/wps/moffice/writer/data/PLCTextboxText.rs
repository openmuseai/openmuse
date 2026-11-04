#![allow(non_snake_case, dead_code, unused_imports)]
use crate::cn::wps::moffice::writer::core::Document::Document;
use crate::cn::wps::moffice::writer::data::PLC::{Node, PLC};
use crate::cn::wps::moffice::writer::data::PLC_Range::PLC_Range;
use crate::cn::wps::moffice::writer::data::RANGEFactory::RANGEFactory;

/// Soft usable PLCTextboxText (Document.getTextboxText).
pub struct PLCTextboxText {
    pub(crate) super_: PLC_Range,
    nodes: Vec<TextboxTextNode>,
}

impl PLCTextboxText {
    // java public PLCTextboxText(Document document)
    pub fn PLCTextboxText(document: &dyn Document) -> Self {
        Self {
            super_: PLC_Range::PLC_Range(document),
            nodes: Vec::new(),
        }
    }
    pub fn size(&self) -> i32 {
        self.super_.size()
    }

    // java public TextboxTextNode findTextboxByCP(int cp)
    pub fn findTextboxByCP(&self, cp: i32) -> Option<TextboxTextNode> {
      // java PLC.Node node = this.findFloor(cp);
      let mut pLowerBound: Node = self.super_.lowerBound__int(cp);
      // java return node != null && node != this.head ? (TextboxTextNode)node : null;
      if self.super_.isHead(pLowerBound.clone()) {
        if self.super_.size() > 0 {
          pLowerBound = pLowerBound.getPrev();
          if self.super_.isHead(pLowerBound.clone()) {
            return None;
          }
          return Some(TextboxTextNode::fromNode(pLowerBound));
        }
        return None;
      }
      if pLowerBound.position() != cp {
        let mut pPrev: Node = pLowerBound.getPrev();
        if self.super_.isHead(pPrev.clone()) {
          return None;
        }
        return Some(TextboxTextNode::fromNode(pPrev));
      }
      Some(TextboxTextNode::fromNode(pLowerBound))
    }

    // java public long getRange(PLC.Node node)
    pub fn getRange(&self, node: &TextboxTextNode) -> i64 {
      self.super_.getRange(node.node.clone())
    }

    // Java: public TextboxTextNode addTextbox(int cp)
    pub fn addTextbox(&mut self, cp: i32) -> TextboxTextNode {
        let node = PLC::alloc_node();
        self.insert__int_node(cp, node);
        let text_node = TextboxTextNode::fromNode(node);
        self.nodes.push(text_node.clone());
        text_node
    }

    // Java: public void removeTextbox(TextboxTextNode node)
    pub fn removeTextbox(&mut self, node: &TextboxTextNode) {
        self.nodes.retain(|stored| stored.node.ptr != node.node.ptr);
        self.remove(node.node);
    }

    pub fn last_textbox(&self) -> Option<TextboxTextNode> {
        let last = self.last();
        if self.isEnd(last) { return None; }
        self.nodes.iter().find(|stored| stored.node.ptr == last.ptr).cloned()
            .or_else(|| Some(TextboxTextNode::fromNode(last)))
    }

    pub fn update_textbox(&mut self, node: &TextboxTextNode) {
        if let Some(stored) = self.nodes.iter_mut().find(|stored| stored.node.ptr == node.node.ptr) {
            *stored = node.clone();
        } else {
            self.nodes.push(node.clone());
        }
    }
}

impl Default for PLCTextboxText {
    fn default() -> Self {
        Self { super_: PLC_Range::default(), nodes: Vec::new() }
    }
}

impl std::ops::Deref for PLCTextboxText {
    type Target = PLC_Range;
    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for PLCTextboxText {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}

// java public class TextboxTextNode extends PLC.Node
#[derive(Clone, Default)]
pub struct TextboxTextNode {
    pub(crate) node: Node,
    // Java field: int mShapeId
    mShapeId: i32,
    mCi: i32,
    mCReusable: i32,
    mFReusable: i16,
}

impl TextboxTextNode {
    pub fn fromNode(node: Node) -> Self {
        Self { node, mShapeId: 0, mCi: 0, mCReusable: 0, mFReusable: 0 }
    }

    // Java: public int getShapeId()
    pub fn getShapeId(&self) -> i32 {
        self.mShapeId
    }

    // Java: public void setShapeId(int nShapeId)
    pub fn setShapeId(&mut self, shape_id: i32) {
        self.mShapeId = shape_id
    }
    pub fn setCi(&mut self, ci: i32) { self.mCi = ci; }
    pub fn setCReusable(&mut self, cReusable: i32) { self.mCReusable = cReusable; }
    pub fn setReusable(&mut self, reusable: i16) { self.mFReusable = reusable; }
    pub fn getCP(&self) -> i32 { self.node.getCP() }
    pub fn new() -> Self {
        Self::default()
    }
}
