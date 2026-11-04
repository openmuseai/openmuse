// Java: import cn.wps.collections.PositionNode
use util::collections::PositionNode::*;
// Java: import cn.wps.collections.PositionTree
use util::collections::PositionTree::*;
// Java: import cn.wps.moffice.writer.data.internal_v2.ListIndexNode
use crate::cn::wps::moffice::writer::data::internal_v2::ListIndexNode::*;
// Java: import cn.wps.moffice.writer.data.internal_v2.ListNode
use crate::cn::wps::moffice::writer::data::internal_v2::ListNode::*;
// Java: import cn.wps.moffice.writer.data.internal_v2.ListPositionTree
use crate::cn::wps::moffice::writer::data::internal_v2::ListPositionTree::*;

// java public class ListIndexPositionTree extends IndexedPositionTree
// Position + list links are what PLCSection queries need. IndexedPositionTree's
// IndexTreeExtension.cast overlays PositionNode.offset (not ListIndexNode.mLeftCount).
pub struct ListIndexPositionTree {
    pub(crate) super_: ListPositionTree,
}

impl Default for ListIndexPositionTree {
    fn default() -> Self {
        Self {
            super_: ListPositionTree::default(),
        }
    }
}

pub struct CPIterator {
    pub(crate) super_: super::ListPositionTree::CPIterator,
    pub(crate) index: i32,
}

impl Default for CPIterator {
    fn default() -> Self {
        Self {
            super_: super::ListPositionTree::CPIterator::default(),
            index: -1,
        }
    }
}

impl ListIndexPositionTree {
    pub fn ListIndexPositionTree(head: *mut ListIndexNode) -> Self {
      Self {
        super_: ListPositionTree::ListPositionTree(head as *mut ListNode),
      }
    }

    pub fn new(head: *mut ListIndexNode) -> Self {
        Self::ListIndexPositionTree(head)
    }

    pub fn insertSpaceBefore(&mut self, pNode: *mut PositionNode, nLength: i32) {
      self.super_.insertSpaceBefore(pNode, nLength);
    }

    pub fn moveRight(&mut self, node: *mut PositionNode, nLength: i32) {
      self.super_.moveRight(node, nLength);
    }

    pub fn dispose(&mut self) {
      self.super_.dispose();
    }

    pub fn lowerBound(&self, cp: i32) -> *mut PositionNode {
        self.super_.lowerBound(cp)
    }
    pub fn lowerBound_with_offset(&self, cp: i32, offset: Option<&mut [i32]>) -> *mut PositionNode {
        self.super_.lowerBound_with_offset(cp, offset)
    }
    pub fn upperBound(&self, cp: i32) -> *mut PositionNode {
        self.super_.upperBound(cp)
    }
    pub fn upperBound_with_offset(&self, cp: i32, offset: Option<&mut [i32]>) -> *mut PositionNode {
        self.super_.upperBound_with_offset(cp, offset)
    }
    pub fn size(&self) -> i32 {
        self.super_.super_.base.size()
    }
    pub fn isEmpty(&self) -> bool {
        self.super_.super_.base.isEmpty()
    }
    pub fn resetHead(&mut self, head: *mut PositionNode) {
        self.super_.super_.base.resetHead_with(head as *mut util::collections::Node::Node);
    }
    pub fn insertLeft(
        &mut self,
        pos: *mut PositionNode,
        p_new: *mut PositionNode,
        cp: i32,
        arg2: Option<&dyn std::any::Any>,
    ) {
        self.super_.super_.base.insertLeft(
            pos as *mut util::collections::Node::Node,
            p_new as *mut util::collections::Node::Node,
            cp,
            arg2,
        );
    }
    pub fn erase(&mut self, z: *mut PositionNode) -> *mut PositionNode {
        self.super_.super_.base.erase(z as *mut util::collections::Node::Node) as *mut PositionNode
    }
    pub fn position(&self, node: *mut PositionNode) -> i32 {
        self.super_.position(node as *mut ListNode)
    }
    pub fn as_position_tree(&self) -> *mut PositionTree {
        &self.super_.super_ as *const PositionTree as *mut PositionTree
    }
}

impl std::ops::Deref for ListIndexPositionTree {
    type Target = ListPositionTree;
    fn deref(&self) -> &Self::Target { &self.super_ }
}
impl std::ops::DerefMut for ListIndexPositionTree {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.super_ }
}

impl CPIterator {
    pub fn new() -> Self { Self::default() }

    pub fn new__positiontree_listindexnode(tree: *mut PositionTree, node: *mut ListIndexNode) -> Self {
      let mut this = Self::new();
      this.initListIndex(tree, node);
      this
    }

    pub fn initListIndex(&mut self, tree: *mut PositionTree, node: *mut ListIndexNode) {
      self.super_.initPositon(tree, node as *mut ListNode);
      self.index = if node.is_null() {
        -1
      } else {
        unsafe { if (*node).isHead() { -1 } else { (*node).index() } }
      };
    }

    pub fn clean(&mut self) {
      self.super_.clean();
      self.index = -1;
    }

    pub fn next(&mut self) -> *mut ListIndexNode {
      self.index += 1;
      self.super_.next() as *mut ListIndexNode
    }

    pub fn prev(&mut self) -> *mut ListIndexNode {
      self.index -= 1;
      self.super_.prev() as *mut ListIndexNode
    }

    pub fn getIndex(&self) -> i32 { self.index }
    pub fn getStart(&self) -> i32 { self.super_.getStart() }
    pub fn hasNext(&self) -> bool { self.super_.hasNext() }
    pub fn getNode(&self) -> *mut ListIndexNode { self.super_.getNode() as *mut ListIndexNode }
}

impl std::ops::Deref for CPIterator {
    type Target = super::ListPositionTree::CPIterator;
    fn deref(&self) -> &Self::Target { &self.super_ }
}
impl std::ops::DerefMut for CPIterator {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.super_ }
}
