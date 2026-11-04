// Java: import cn.wps.collections.ITreeCreator
use util::collections::ITreeCreator::*;
// Java: import cn.wps.collections.ITreeExtension
use util::collections::ITreeExtension::*;
// Java: import cn.wps.collections.Node
use util::collections::Node::*;
// Java: import cn.wps.collections.Tree
use util::collections::Tree::*;
// Java: import cn.wps.moffice.writer.data.internal_v2.ListNode
use crate::cn::wps::moffice::writer::data::internal_v2::ListNode::*;
use util::collections::PositionNode::PositionNode;
use std::ptr;

// java public static final int mask = 63;
pub(crate) const mask: i32 = 63;

// java public class ListTreeExtension implements ITreeExtension
#[derive(Default)]
pub struct ListTreeExtension {
}

fn cast(node: *mut u8) -> *mut ListNode {
    node as *mut ListNode
}

// java public interface ITreeExtension
impl ITreeExtension for ListTreeExtension {
    // java public void onCreateHead(Tree tree, Node head)
    fn onCreateHead(&mut self, tree: *mut u8, head: *mut u8) {
      let _ = tree;
      // java ListNode node = cast(head);
      let node = cast(head);
      if node.is_null() {
        return;
      }
      // java node.next = node.prev = node;
      unsafe {
        (*node).next = node;
        (*node).prev = node;
      }
    }

    // java public void onBinaryTreeInsertedLeft(Tree tree, Node _pNode, Node _pNew, int arg1, Object arg2)
    fn onBinaryTreeInsertedLeft(&mut self, tree: *mut u8, _pNode: *mut u8, _pNew: *mut u8, arg1: i32, arg2: Option<&dyn std::any::Any>) {
      let _ = arg2;
      unsafe {
        let tree = tree as *mut Tree;
        // java ListNode pNew = cast(_pNew);
        let pNew = cast(_pNew);
        // java ListNode pNode = cast(_pNode);
        let pNode = cast(_pNode);
        if tree.is_null() || pNew.is_null() || pNode.is_null() {
          return;
        }
        // java ListNode prev = cast(tree.treePrev(pNew));
        let prev = (*tree).treePrev(pNew as *mut Node) as *mut ListNode;
        // java pNew.next = null;
        (*pNew).next = ptr::null_mut();
        // java pNew.prev = null;
        (*pNew).prev = ptr::null_mut();
        if prev.is_null() {
          return;
        }
        // java if (prev.isHead())
        if (*prev).isHead() {
          // java pNew.prev = prev;
          (*pNew).prev = prev;
          // java prev.next = pNew;
          (*prev).next = pNew;
          // java pNew.offset_prev = arg1;
          (*pNew).offset_prev = arg1;
          // java prev.offset_prev = -arg1;
          (*prev).offset_prev = -arg1;
        } else {
          // java pNew.prev = prev;
          (*pNew).prev = prev;
          // java prev.next = pNew;
          (*prev).next = pNew;
          // java pNew.offset_prev = arg1 - prev.position();
          (*pNew).offset_prev = arg1 - (*prev).position();
        }
        // java if (pNode.isHead())
        if (*pNode).isHead() {
          // java pNew.next = pNode;
          (*pNew).next = pNode;
          // java pNode.prev = pNew;
          (*pNode).prev = pNew;
          // java pNode.offset_prev = -arg1;
          (*pNode).offset_prev = -arg1;
        } else {
          // java pNew.next = pNode;
          (*pNew).next = pNode;
          // java pNode.prev = pNew;
          (*pNode).prev = pNew;
          // java pNode.offset_prev -= pNew.offset_prev;
          (*pNode).offset_prev -= (*pNew).offset_prev;
        }
      }
    }

    // java public void onBinaryTreeInsertedRight(Tree tree, Node _pNode, Node _pNew, int arg1, Object arg2)
    fn onBinaryTreeInsertedRight(&mut self, tree: *mut u8, _pNode: *mut u8, _pNew: *mut u8, arg1: i32, arg2: Option<&dyn std::any::Any>) {
      let _ = arg2;
      unsafe {
        let tree = tree as *mut Tree;
        // java ListNode pNew = cast(_pNew);
        let pNew = cast(_pNew);
        // java ListNode pNode = cast(_pNode);
        let pNode = cast(_pNode);
        if tree.is_null() || pNew.is_null() || pNode.is_null() {
          return;
        }
        // java ListNode next = cast(tree.treeNext(pNew));
        let next = (*tree).treeNext(pNew as *mut Node) as *mut ListNode;
        // java pNew.next = null;
        (*pNew).next = ptr::null_mut();
        // java pNew.prev = null;
        (*pNew).prev = ptr::null_mut();
        // java if (pNode.isHead())
        if (*pNode).isHead() {
          // java pNew.prev = pNode;
          (*pNew).prev = pNode;
          // java pNode.next = pNew;
          (*pNode).next = pNew;
          // java pNew.offset_prev = arg1;
          (*pNew).offset_prev = arg1;
          // java pNode.offset_prev = -arg1;
          (*pNode).offset_prev = -arg1;
        } else {
          // java pNew.prev = pNode;
          (*pNew).prev = pNode;
          // java pNode.next = pNew;
          (*pNode).next = pNew;
          // java pNew.offset_prev = arg1 - pNode.position();
          (*pNew).offset_prev = arg1 - (*pNode).position();
        }
        if next.is_null() {
          return;
        }
        // java if (next.isHead())
        if (*next).isHead() {
          // java pNew.next = next;
          (*pNew).next = next;
          // java next.prev = pNew;
          (*next).prev = pNew;
          // java next.offset_prev = -arg1;
          (*next).offset_prev = -arg1;
        } else {
          // java pNew.next = next;
          (*pNew).next = next;
          // java next.prev = pNew;
          (*next).prev = pNew;
          // java next.offset_prev -= pNew.offset_prev;
          (*next).offset_prev -= (*pNew).offset_prev;
        }
      }
    }

    // java public void beforeSwap(Tree tree, Node _z, Node _swp)
    fn beforeSwap(&mut self, tree: *mut u8, _z: *mut u8, _swp: *mut u8) {
      let _ = (tree, _z, _swp);
    }

    // java public void beforeErase(Tree tree, Node z, Node swp)
    fn beforeErase(&mut self, tree: *mut u8, z: *mut u8, swp: *mut u8) {
      let _ = (tree, swp);
      // java ListNode node = (ListNode)z;
      let node = cast(z);
      if node.is_null() {
        return;
      }
      unsafe {
        // java ListNode next = node.next;
        let next = (*node).next;
        if next.is_null() {
          return;
        }
        // java next.offset_prev += node.offset_prev;
        (*next).offset_prev += (*node).offset_prev;
      }
    }

    // java public void onBinaryTreeErased(Tree tree, Node z, Node x, Node x_parent)
    fn onBinaryTreeErased(&mut self, tree: *mut u8, z: *mut u8, x: *mut u8, x_parent: *mut u8) {
      let _ = (tree, x, x_parent);
      // java ListNode node = (ListNode)z;
      let node = cast(z);
      if node.is_null() {
        return;
      }
      unsafe {
        // java ListNode prev = node.prev;
        let prev = (*node).prev;
        // java ListNode next = node.next;
        let next = (*node).next;
        if prev.is_null() || next.is_null() {
          return;
        }
        // java prev.next = next;
        (*prev).next = next;
        // java next.prev = prev;
        (*next).prev = prev;
      }
    }

    // java public void beforeRotateLeft(Tree tree, Node x, Node y)
    fn beforeRotateLeft(&mut self, tree: *mut u8, x: *mut u8, y: *mut u8) {
      let _ = (tree, x, y);
    }

    // java public void beforeRotateRight(Tree tree, Node y, Node x)
    fn beforeRotateRight(&mut self, tree: *mut u8, y: *mut u8, x: *mut u8) {
      let _ = (tree, y, x);
    }

    // java public ITreeCreator getTreeCreator()
    fn getTreeCreator(&self) -> Option<Box<dyn std::any::Any>> {
      // java return null;
      return None;
    }

}

// java public class ListTreeExtension implements ITreeExtension
impl ListTreeExtension {
    // java public void insertSpaceBefore(Node pNode, int nLength)
    pub fn insertSpaceBefore(&mut self, pNode: *mut PositionNode, nLength: i32) {
      // java ListNode node = (ListNode)pNode;
      let node = pNode as *mut ListNode;
      if node.is_null() {
        return;
      }
      unsafe {
        // java if (!node.isHead())
        if !(*node).isHead() {
          // java node.offset_prev += nLength;
          (*node).offset_prev += nLength;
        }
        // java ListNode var10000 = cast(node.getHead());
        let head = (*node).getHead();
        if !head.is_null() {
          // java var10000.offset_prev -= nLength;
          (*head).offset_prev -= nLength;
        }
      }
    }

    // java public void moveRight(Node pNode, int steps)
    pub fn moveRight(&mut self, pNode: *mut PositionNode, steps: i32) {
      // java ListNode node = (ListNode)pNode;
      let node = pNode as *mut ListNode;
      if node.is_null() {
        return;
      }
      unsafe {
        // java if (!node.isHead())
        if !(*node).isHead() {
          // java node.offset_prev += steps;
          (*node).offset_prev += steps;
        }
        // java if (null != node.next)
        if !(*node).next.is_null() {
          // java ListNode var10000 = node.next;
          // java var10000.offset_prev -= steps;
          (*(*node).next).offset_prev -= steps;
        }
      }
    }

}
