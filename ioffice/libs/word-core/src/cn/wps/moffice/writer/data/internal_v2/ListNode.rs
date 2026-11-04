// Java: import cn.wps.collections.PositionNode
use util::collections::PositionNode::*;
use std::ptr;

// java public class ListNode extends PositionNode
#[repr(C)]
pub struct ListNode {
    // java PositionNode super_
    pub(crate) super_: PositionNode,
    // java ListNode prev;
    pub(crate) prev: *mut Self,
    // java ListNode next;
    pub(crate) next: *mut Self,
    // java int offset_prev;
    pub(crate) offset_prev: i32,
}

impl Default for ListNode {
    fn default() -> Self {
        Self {
            super_: PositionNode::new(),
            prev: ptr::null_mut(),
            next: ptr::null_mut(),
            offset_prev: 0,
        }
    }
}

// java public class ListNode extends PositionNode
impl ListNode {
    // java public  ListNode(int offset)
    pub fn ListNode(offset: i32) -> Self {
      // java super(offset);
      Self {
        super_: PositionNode::new_with_offset(offset),
        prev: ptr::null_mut(),
        next: ptr::null_mut(),
        offset_prev: 0,
      }
    }

    // java public ListNode getNext()
    pub fn getNext(&self) -> *mut ListNode {
      // java return this.next;
      self.next
    }

    // java public ListNode getPrev()
    pub fn getPrev(&self) -> *mut ListNode {
      // java return this.prev;
      self.prev
    }

    // java public void moveRight(int steps)
    pub fn moveRight(&mut self, steps: i32) {
      // java super.moveRight(steps);
      self.super_.moveRight(steps);
      // java this.offset_prev += steps;
      self.offset_prev = self.offset_prev + steps;
      // java if (null != this.next)
      if !self.next.is_null() {
        // java ListNode var10000 = this.next;
        // java var10000.offset_prev -= steps;
        unsafe { (*self.next).offset_prev = (*self.next).offset_prev - steps; }
      }
    }

    // java public int getOffsetPrev()
    pub fn getOffsetPrev(&self) -> i32 {
      // java return this.offset_prev;
      return self.offset_prev;
    }

    // java public void setOffsetPrev(int offset_prev)
    pub fn setOffsetPrev(&mut self, offset_prev: i32) {
      // java this.offset_prev = offset_prev;
      self.offset_prev = offset_prev;
    }

    pub fn isHead(&self) -> bool {
      self.super_.base.isHead()
    }

    pub fn getHead(&self) -> *mut ListNode {
      self.super_.base.getHead() as *mut ListNode
    }

    pub fn as_node_ptr(&self) -> *mut util::collections::Node::Node {
      self as *const ListNode as *mut util::collections::Node::Node
    }
}

impl std::ops::Deref for ListNode {
    type Target = PositionNode;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for ListNode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}
