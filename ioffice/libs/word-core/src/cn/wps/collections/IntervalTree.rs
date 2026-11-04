// Java: import cn.wps.collections.PositionTree
use util::collections::PositionTree::PositionTree;
use util::collections::PositionNode::PositionNode;
use util::collections::Node::Node;
use util::collections::Tree::Tree;
// Java: import cn.wps.collections.IntervalNode
use crate::cn::wps::collections::IntervalNode::IntervalNode;
use crate::cn::wps::collections::IntervalMapNode::IntervalMapNode;

// java public class IntervalTree extends PositionTree
pub struct IntervalTree {
    // java PositionTree super_
    pub(crate) super_: PositionTree,
}

// java public static class InsertSpace
#[derive(Clone, Copy)]
pub struct InsertSpace {
    // java IntervalNode node;
    pub(crate) node: *mut IntervalNode,
    // java boolean left;
    pub(crate) left: bool,
    // java int pos;
    pub(crate) pos: i32,
}

impl Default for InsertSpace {
    fn default() -> Self {
        Self {
            node: std::ptr::null_mut(),
            left: false,
            pos: 0,
        }
    }
}

impl Default for IntervalTree {
    fn default() -> Self {
        // Placeholder — prefer IntervalTree::new(head)
        Self {
            super_: PositionTree::new(std::ptr::null_mut()),
        }
    }
}

// java public class IntervalTree extends PositionTree
impl IntervalTree {
    // java public  IntervalTree(IntervalNode head)
    pub fn IntervalTree(head: *mut IntervalNode) -> Self {
      // java super(head);
      let head_pn = head as *mut PositionNode;
      Self {
        super_: PositionTree::new(head_pn),
      }
    }

    pub fn new(head: *mut IntervalNode) -> Self {
        Self::IntervalTree(head)
    }

    // java  int getEndPosition()
    pub fn getEndPosition(&self) -> i32 {
      // java if (this.isEmpty())
      if self.isEmpty() {
        // java throw new RuntimeException("No interval in the tree");
        panic!("No interval in the tree");
      }
      // java else
      else {
        // java IntervalNode node = (IntervalNode)this.rightmost();
        let node = self.rightmost() as *mut IntervalNode;
        unsafe {
          // java return node.position() + node.getLength();
          return (*node).position() + IntervalMapNode::virtual_get_length(node);
        }
      }
    }

    // java public InsertSpace findForInsertSpace(int cp)
    pub fn findForInsertSpace(&self, cp: i32) -> InsertSpace {
      // java IntervalNode p = (IntervalNode)this.lowerBound(cp);
      let p = self.super_.lowerBound(cp) as *mut IntervalNode;
      // java if (p == this.mHead)
      if p as *mut Node == self.mHead() {
        // java IntervalNode pprev = (IntervalNode)p.getPrev();
        let pprev = unsafe { (*p).super_.base.getPrev() as *mut IntervalNode };
        // java if (pprev == this.mHead)
        if pprev as *mut Node == self.mHead() {
          // java return new InsertSpace(p, true, 0);
          return InsertSpace::InsertSpace(p, true, 0);
        }
        // java else
        else {
          // java int pprev_pos = this.position(pprev);
          let pprev_pos: i32 = self.super_.position(pprev as *mut PositionNode);
          // java return cp < pprev_pos + pprev.getLength() ? new InsertSpace(pprev, false, cp - pprev_pos) : new InsertSpace(p, true, 0);
          return if cp < pprev_pos + unsafe { IntervalMapNode::virtual_get_length(pprev) } {
            InsertSpace::InsertSpace(pprev, false, cp - pprev_pos)
          } else {
            InsertSpace::InsertSpace(p, true, 0)
          };
        }
      }
      // java else
      else {
        // java int p_pos = this.position(p);
        let p_pos: i32 = self.super_.position(p as *mut PositionNode);
        // java if (p_pos == cp)
        if p_pos == cp {
          // java return new InsertSpace(p, true, 0);
          return InsertSpace::InsertSpace(p, true, 0);
        }
        // java else
        else {
          // java IntervalNode pprev = (IntervalNode)p.getPrev();
          let pprev = unsafe { (*p).super_.base.getPrev() as *mut IntervalNode };
          // java if (pprev == this.mHead)
          if pprev as *mut Node == self.mHead() {
            // java return new InsertSpace(p, true, 0);
            return InsertSpace::InsertSpace(p, true, 0);
          }
          // java else
          else {
            // java int pprev_pos = this.position(pprev);
            let pprev_pos: i32 = self.super_.position(pprev as *mut PositionNode);
            // java return cp >= pprev_pos + pprev.getLength() ? new InsertSpace(p, true, 0) : new InsertSpace(pprev, false, cp - pprev_pos);
            return if cp >= pprev_pos + unsafe { IntervalMapNode::virtual_get_length(pprev) } {
              InsertSpace::InsertSpace(p, true, 0)
            } else {
              InsertSpace::InsertSpace(pprev, false, cp - pprev_pos)
            };
          }
        }
      }
    }

    // java public IntervalNode find(int pos, int[] off)
    pub fn find(&self, mut pos: i32, off: Option<&mut [i32]>) -> *mut IntervalNode {
      // java IntervalNode node = (IntervalNode)this.getRoot();
      let mut node = self.getRoot() as *mut IntervalNode;
      // java while (node != null)
      while !node.is_null() {
        unsafe {
          // java if (pos < node.offset)
          if pos < (*node).super_.offset {
            // java node = (IntervalNode)node.left;
            node = (*node).super_.base.left as *mut IntervalNode;
          }
          // java else
          else {
            // java if (pos < node.offset + node.getLength())
            if pos < (*node).super_.offset + IntervalMapNode::virtual_get_length(node) {
              // java if (off != null)
              if let Some(off) = off {
                // java off[0] = pos - node.offset;
                off[0] = pos - (*node).super_.offset;
              }
              // java return node;
              return node;
            }
            // java pos -= node.offset;
            pos = pos - (*node).super_.offset;
            // java node = (IntervalNode)node.right;
            node = (*node).super_.base.right as *mut IntervalNode;
          }
        }
      }
      // java return null;
      return std::ptr::null_mut();
    }

    pub fn isEmpty(&self) -> bool {
        self.super_.base.isEmpty()
    }

    pub fn rightmost(&self) -> *mut Node {
        self.super_.base.rightmost()
    }

    pub fn leftmost(&self) -> *mut Node {
        unsafe { (*self.super_.base.mHead).left }
    }

    pub fn getRoot(&self) -> *mut Node {
        self.super_.base.getRoot()
    }

    pub fn mHead(&self) -> *mut Node {
        self.super_.base.mHead
    }

    pub fn getHead(&self) -> *mut Node {
        self.super_.base.getHead()
    }

    pub fn size(&self) -> i32 {
        self.super_.base.size()
    }

    pub fn position(&self, node: *mut PositionNode) -> i32 {
        self.super_.position(node)
    }

    pub fn lowerBound(&self, cp: i32) -> *mut PositionNode {
        self.super_.lowerBound(cp)
    }

    pub fn insertRight(&mut self, pos: *mut Node, p_new: *mut Node) {
        self.super_.base.insertRight_simple(pos, p_new);
    }

    pub fn base_tree_mut(&mut self) -> &mut Tree {
        &mut self.super_.base
    }
}

impl std::ops::Deref for IntervalTree {
    type Target = PositionTree;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for IntervalTree {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}

// java public static class InsertSpace
impl InsertSpace {
    // java public  InsertSpace(IntervalNode node, boolean left, int pos)
    pub fn InsertSpace(node: *mut IntervalNode, left: bool, pos: i32) -> Self {
      // java this.node = node;
      Self { node: node, left: left, pos: pos }
      // java this.left = left;
      // java this.pos = pos;
    }

}
