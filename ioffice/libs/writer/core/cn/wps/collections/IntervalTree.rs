// Java: import cn.wps.collections.PositionTree
use util::collections::PositionTree::*;
// Java: import cn.wps.collections.IntervalNode
use crate::cn::wps::collections::IntervalNode::*;
// Java: import java.lang.RuntimeException
use stubs::java::lang::RuntimeException::*;

// Java: public class IntervalTree extends PositionTree 
#[derive(Default)]
pub struct IntervalTree {
    // Java field: PositionTree super_;
    pub super_: PositionTree,
}

// Java: public static class InsertSpace 
#[derive(Default)]
pub struct InsertSpace {
    // Java field:       IntervalNode node;
    pub node: IntervalNode,
    // Java field:       boolean left;
    pub left: bool,
    // Java field:       int pos;
    pub pos: i32,
}

// Java: public class IntervalTree extends PositionTree 
impl IntervalTree {
    // Java: public IntervalTree(IntervalNode head) 
    pub fn new(head: IntervalNode) -> IntervalTree {
      Self { super_: PositionTree::new__intervalnode(head) }
    }

    // Java: int getEndPosition() 
    pub fn getEndPosition(&mut self) -> i32 {
      // Java: if (this.isEmpty())
      if self.isEmpty() {
         // Java: throw new RuntimeException("No interval in the tree");
         panic!("{}", RuntimeException::new("No interval in the tree"));
      }
      // Java: else
      else {
         // Java: IntervalNode node = (IntervalNode)this.rightmost();
         let node: IntervalNode = (self.rightmost()).downcast_ref::<IntervalNode>().unwrap();
         // Java: return node.position() + node.getLength();
         return node.position() + node.getLength();
      }
    }

    // Java: public InsertSpace findForInsertSpace(int cp) 
    pub fn findForInsertSpace(&mut self, cp: i32) -> &InsertSpace {
      // Java: IntervalNode p = (IntervalNode)this.lowerBound(cp);
      let p: IntervalNode = (self.lowerBound(cp)).downcast_ref::<IntervalNode>().unwrap();
      // Java: if (p == this.mHead)
      if p == self.mHead {
         // Java: IntervalNode pprev = (IntervalNode)p.getPrev();
         let pprev: IntervalNode = (p.getPrev()).downcast_ref::<IntervalNode>().unwrap();
         // Java: if (pprev == this.mHead)
         if pprev == self.mHead {
            // Java: return new InsertSpace(p, true, 0);
            return InsertSpace::new(p, true, 0);
         }
         // Java: else
         else {
            // Java: int pprev_pos = this.position(pprev);
            let pprev_pos: i32 = self.position(pprev);
            // Java: return cp < pprev_pos + pprev.getLength() ? new InsertSpace(pprev, false, cp - pprev_pos) : new InsertSpace(p, true, 0);
            return (if cp < pprev_pos + pprev.getLength() { InsertSpace::new(pprev, false, cp - pprev_pos) } else { InsertSpace::new(p, true, 0) });
         }
      }
      // Java: else
      else {
         // Java: int p_pos = this.position(p);
         let p_pos: i32 = self.super_.position(p);
         // Java: if (p_pos == cp)
         if p_pos == cp {
            // Java: return new InsertSpace(p, true, 0);
            return InsertSpace::new(p, true, 0);
         }
         // Java: else
         else {
            // Java: IntervalNode pprev = (IntervalNode)p.getPrev();
            let pprev: IntervalNode = (p.getPrev()).downcast_ref::<IntervalNode>().unwrap();
            // Java: if (pprev == this.mHead)
            if pprev == self.mHead {
               // Java: return new InsertSpace(p, true, 0);
               return InsertSpace::new(p, true, 0);
            }
            // Java: else
            else {
               // Java: int pprev_pos = this.position(pprev);
               let pprev_pos: i32 = self.super_.position(pprev);
               // Java: return cp >= pprev_pos + pprev.getLength() ? new InsertSpace(p, true, 0) : new InsertSpace(pprev, false, cp - pprev_pos);
               return (if cp >= pprev_pos + pprev.getLength() { InsertSpace::new(p, true, 0) } else { InsertSpace::new(pprev, false, cp - pprev_pos) });
            }
         }
      }
    }

    // Java: public IntervalNode find(int pos, int[] off) 
    pub fn find(&mut self, pos: i32, off: Vec<i32>) -> Option<IntervalNode> {
      // Java: IntervalNode node = (IntervalNode)this.getRoot();
      let node: IntervalNode = (self.getRoot()).downcast_ref::<IntervalNode>().unwrap();
      // Java: while(node != null)
      while node.is_some() {
         // Java: if (pos < node.offset)
         if pos < node.offset {
            // Java: node = (IntervalNode)node.left;
            node = (node.left).downcast_ref::<IntervalNode>().unwrap();
         }
         // Java: else
         else {
            // Java: if (pos < node.offset + node.getLength())
            if pos < node.offset + node.getLength() {
               // Java: if (off != null)
               if off.is_some() {
                  // Java: off[0] = pos - node.offset;
                  off[0] = pos - node.offset;
               }
               // Java: return node;
               return node;
            }
            // Java: pos -= node.offset;
            pos = pos - node.offset;
            // Java: node = (IntervalNode)node.right;
            node = (node.right).downcast_ref::<IntervalNode>().unwrap();
         }
      }
      // Java: return null;
      return None;
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

// Java: public static class InsertSpace 
impl InsertSpace {
    // Java: public InsertSpace(IntervalNode node, boolean left, int pos) 
    pub fn new(node: IntervalNode, left: bool, pos: i32) -> InsertSpace {
      Self { node: node, left: left, pos: pos }
    }

}
