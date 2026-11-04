// Java: import cn.wps.collections.IntervalMap
use crate::cn::wps::collections::IntervalMap::{
    IntervalMap, IntervalMapNode1, IntervalMapNode2, IntervalMapTree,
};
// Java: import cn.wps.collections.textrope.ITextPiece
use crate::cn::wps::collections::textrope::ITextPiece::ITextPiece;
use util::collections::Node::Node;
use util::collections::PositionNode::PositionNode;

// java  class KTextPieceTable extends IntervalMap
pub struct KTextPieceTable {
    // java IntervalMap super_
    pub(crate) super_: IntervalMap,
}

// java static class CPNode extends IntervalMap.IntervalMapNode1 implements ITextPiece
#[repr(C)]
pub struct CPNode {
    // java IntervalMap.IntervalMapNode1 super_
    pub(crate) super_: IntervalMapNode1,
}

// java static class FCNode extends IntervalMap.IntervalMapNode2
#[repr(C)]
pub struct FCNode {
    // java IntervalMap.IntervalMapNode2 super_
    pub(crate) super_: IntervalMapNode2,
}

impl Default for KTextPieceTable {
    fn default() -> Self {
        Self::new()
    }
}

// java  class KTextPieceTable extends IntervalMap
impl KTextPieceTable {
    pub fn new() -> Self {
        Self {
            super_: IntervalMap::IntervalMap(),
        }
    }

    // java protected CPNode createNode1()
    pub fn createNode1(&mut self) -> *mut CPNode {
      // java return new CPNode();
      Box::into_raw(Box::new(CPNode::new()))
    }

    // java protected FCNode createNode2()
    pub fn createNode2(&mut self) -> *mut FCNode {
      // java return new FCNode();
      Box::into_raw(Box::new(FCNode::new()))
    }

    pub fn forwardMapping(&self, start1: i32, def: i32) -> i32 {
        self.super_.forwardMapping(start1, def)
    }
    pub fn backwardMapping(&self, start2: i32, def: i32) -> i32 {
        self.super_.backwardMapping(start2, def)
    }
    pub fn getTree1(&self) -> &IntervalMapTree {
        self.super_.getTree1()
    }
    pub fn getTree2(&self) -> &IntervalMapTree {
        self.super_.getTree2()
    }
    pub fn addMapping1(&mut self, start1: i32, start2: i32, length: i32) -> *mut crate::cn::wps::collections::IntervalMapNode::IntervalMapNode {
        self.super_.addMapping1(start1, start2, length)
    }
    pub fn insertSpace1(&mut self, start1: i32, length: i32) {
        self.super_.insertSpace1(start1, length)
    }
    pub fn eraseMapping1(&mut self, start1: i32, length: i32, segments: Option<&mut Vec<crate::cn::wps::collections::IntervalMap::Segment>>) {
        self.super_.eraseMapping1(start1, length, segments)
    }
    pub fn eraseSpace1(&mut self, start1: i32, length: i32) {
        self.super_.eraseSpace1(start1, length)
    }
}

impl std::ops::Deref for KTextPieceTable {
    type Target = IntervalMap;
    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}
impl std::ops::DerefMut for KTextPieceTable {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}

impl CPNode {
    pub fn new() -> Self {
        Self {
            super_: IntervalMapNode1::new(),
        }
    }
    // java public int getCp()
    pub fn getCp(&self) -> i32 {
      // java return this.position();
      self.super_.super_.super_.super_.position()
    }
    // java public int getFc()
    pub fn getFc(&self) -> i32 {
      // java return super.getPeer().position();
      unsafe {
        let peer = self.super_.super_.peer;
        if peer.is_null() {
          0
        } else {
          (*peer).super_.super_.position()
        }
      }
    }
    // java public CPNode getNext()
    pub fn getNext(&self) -> *mut CPNode {
      // java return (CPNode)super.getNext();
      self.super_.super_.super_.super_.base.getNext() as *mut CPNode
    }
    // java public CPNode getPrev()
    pub fn getPrev(&self) -> *mut CPNode {
      // java return (CPNode)super.getPrev();
      self.super_.super_.super_.super_.base.getPrev() as *mut CPNode
    }
    // java public boolean isEnd()
    pub fn isEnd(&self) -> bool {
      // java return this.isHead();
      self.super_.super_.super_.super_.base.isHead()
    }
    // java public String toString()
    pub fn toString(&self) -> String {
      let mut sb = String::from("CPNode");
      if self.isEnd() {
        sb.push_str("(head)");
      }
      if !self.isEnd() {
        sb.push_str(&format!(
          "\ncp = {}\nfc = {}\nlength = {}",
          self.getCp(),
          self.getFc(),
          self.super_.mLength
        ));
      }
      sb.push('\n');
      sb
    }
}

impl ITextPiece for CPNode {
    fn getCp(&self) -> i32 {
        CPNode::getCp(self)
    }
    fn getFc(&self) -> i32 {
        CPNode::getFc(self)
    }
    fn getLength(&self) -> i32 {
        self.super_.mLength
    }
    fn getNext(&self) -> Option<Box<dyn ITextPiece>> {
        None /* soft: raw tree next; callers use pointer APIs */
    }
    fn getPrev(&self) -> Option<Box<dyn ITextPiece>> {
        None
    }
    fn isEnd(&self) -> bool {
        CPNode::isEnd(self)
    }
}

impl FCNode {
    pub fn new() -> Self {
        Self {
            super_: IntervalMapNode2::new(),
        }
    }
}

impl std::ops::Deref for CPNode {
    type Target = IntervalMapNode1;
    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}
impl std::ops::DerefMut for CPNode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}
