// Java: import cn.wps.collections.PositionNode
use util::collections::PositionNode::*;
// Java: import cn.wps.collections.PositionTree
use util::collections::PositionTree::*;
// Java: import cn.wps.moffice.writer.data.internal_v2.ListTreeExtension
use crate::cn::wps::moffice::writer::data::internal_v2::ListTreeExtension::*;
// Java: import cn.wps.moffice.writer.data.internal_v2.ListNode
use crate::cn::wps::moffice::writer::data::internal_v2::ListNode::*;
use std::ptr;

// java public class ListPositionTree extends PositionTree
pub struct ListPositionTree {
    // java PositionTree super_
    pub(crate) super_: PositionTree,
    // java ListTreeExtension lists = new ListTreeExtension();
    pub(crate) lists: ListTreeExtension,
}

impl Default for ListPositionTree {
    fn default() -> Self {
        Self {
            super_: PositionTree::new(ptr::null_mut()),
            lists: ListTreeExtension::default(),
        }
    }
}

// java public static class CPIterator
pub struct CPIterator {
    // java PositionTree tree;
    pub(crate) tree: *mut PositionTree,
    // java ListNode curr;
    pub(crate) curr: *mut ListNode,
    // java int start;
    pub(crate) start: i32,
}

impl Default for CPIterator {
    fn default() -> Self {
        Self {
            tree: ptr::null_mut(),
            curr: ptr::null_mut(),
            start: 0,
        }
    }
}

// java public class ListPositionTree extends PositionTree
impl ListPositionTree {
    // java public  ListPositionTree(ListNode head)
    pub fn ListPositionTree(head: *mut ListNode) -> Self {
      // java super(head);
      let mut this = Self {
        super_: PositionTree::new(head as *mut PositionNode),
        lists: ListTreeExtension::default(),
      };
      // java this.addExtension(this.lists, 63);
      this.super_.base.addExtension(Box::new(ListTreeExtension::default()), 63);
      this
    }

    // java public final int position(ListNode node)
    pub fn position(&self, node: *mut ListNode) -> i32 {
      // java return this.positions.position(node);
      self.super_.position(node as *mut PositionNode)
    }

    // java public void insertSpaceBefore(PositionNode pNode, int nLength)
    pub fn insertSpaceBefore(&mut self, pNode: *mut PositionNode, nLength: i32) {
      // java super.insertSpaceBefore(pNode, nLength);
      self.super_.insertSpaceBefore(pNode, nLength);
      // java this.lists.insertSpaceBefore(pNode, nLength);
      self.lists.insertSpaceBefore(pNode, nLength);
    }

    // java public void moveRight(PositionNode node, int nLength)
    pub fn moveRight(&mut self, node: *mut PositionNode, nLength: i32) {
      // java super.moveRight(node, nLength);
      self.super_.moveRight(node, nLength);
      // java this.lists.moveRight(node, nLength);
      self.lists.moveRight(node, nLength);
    }

    // java public void dispose()
    pub fn dispose(&mut self) {
      // java super.dispose();
      // java this.positions = null;
    }

}

impl std::ops::Deref for ListPositionTree {
    type Target = PositionTree;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for ListPositionTree {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}

// java public static class CPIterator
impl CPIterator {
    pub fn new() -> Self {
      Self::default()
    }

    pub fn new__positiontree_listnode(tree: *mut PositionTree, node: *mut ListNode) -> Self {
      let mut this = Self::new();
      this.initPositon(tree, node);
      this
    }

    pub fn new__positiontree(tree: *mut PositionTree) -> Self {
      let mut this = Self::new();
      if !tree.is_null() {
        unsafe {
          this.initPositon(tree, (*tree).base.mHead as *mut ListNode);
        }
      }
      this
    }

    pub fn initPositon(&mut self, tree: *mut PositionTree, node: *mut ListNode) {
      self.tree = tree;
      self.curr = node;
      self.start = if node.is_null() {
        0
      } else {
        unsafe { if (*node).isHead() { 0 } else { (*node).position() } }
      };
    }

    pub fn clean(&mut self) {
      self.tree = ptr::null_mut();
      self.curr = ptr::null_mut();
      self.start = 0;
    }

    pub fn getTree(&self) -> *mut PositionTree {
      self.tree
    }

    pub fn hasNext(&self) -> bool {
      if self.curr.is_null() || self.tree.is_null() {
        return false;
      }
      unsafe { (*self.curr).next != ((*self.tree).base.mHead as *mut ListNode) }
    }

    pub fn hasPrev(&self) -> bool {
      if self.curr.is_null() || self.tree.is_null() {
        return false;
      }
      unsafe { (*self.curr).prev != ((*self.tree).base.mHead as *mut ListNode) }
    }

    pub fn next(&mut self) -> *mut ListNode {
      if !self.curr.is_null() {
        unsafe {
          self.curr = (*self.curr).next;
          if !self.curr.is_null() {
            self.start = self.start + (*self.curr).offset_prev;
          }
        }
      }
      self.curr
    }

    pub fn prev(&mut self) -> *mut ListNode {
      if !self.curr.is_null() {
        unsafe {
          self.start = self.start - (*self.curr).offset_prev;
          self.curr = (*self.curr).prev;
        }
      }
      self.curr
    }

    pub fn getStart(&self) -> i32 {
      return self.start;
    }

    pub fn getEnd(&self) -> i32 {
      if self.curr.is_null() {
        return self.start;
      }
      unsafe {
        let next = (*self.curr).next;
        if next.is_null() {
          return self.start;
        }
        if (*next).isHead() {
          (*next).position()
        } else {
          self.start + (*next).offset_prev
        }
      }
    }

    pub fn moveRight(&mut self, steps: i32) {
      self.start = self.start + steps;
      if !self.tree.is_null() && !self.curr.is_null() {
        unsafe {
          (*self.tree).moveRight(self.curr as *mut PositionNode, steps);
        }
      }
    }

    pub fn getNode(&self) -> *mut ListNode {
      self.curr
    }

    pub fn isEnd(&self) -> bool {
      if self.tree.is_null() {
        return self.curr.is_null();
      }
      unsafe { self.curr == ((*self.tree).base.mHead as *mut ListNode) }
    }

}
