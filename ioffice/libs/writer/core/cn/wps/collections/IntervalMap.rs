// Java: import java.util.ArrayList
// Java: import java.lang.StringBuilder
use stubs::java::lang::StringBuilder::*;
// Java: import cn.wps.collections.IntervalTree
use crate::cn::wps::collections::IntervalTree::*;
// Java: import cn.wps.io.dom.Node
use stubs::cn::wps::io::dom::Node::*;
// Java: import cn.wps.collections.IntervalMapNode
use crate::cn::wps::collections::IntervalMapNode::*;
// Java: import java.lang.RuntimeException
use stubs::java::lang::RuntimeException::*;
// Java: import 
// Java: import java.lang.InsertSpace
use stubs::java::lang::InsertSpace::*;

// Java: public class IntervalMap 
#[derive(Default)]
pub struct IntervalMap {
    // Java field:    IntervalMapTree tree1 = new IntervalMapTree(this.createNode1());
    pub tree1: IntervalMapTree,
    // Java field:    IntervalMapTree tree2 = new IntervalMapTree(this.createNode2());
    pub tree2: IntervalMapTree,
}

// Java: public static class IntervalMapNode1 extends IntervalMapNode 
#[derive(Default)]
pub struct IntervalMapNode1 {
    // Java field:       private int mLength;
    mLength: i32,
}

// Java: public static class IntervalMapNode2 extends IntervalMapNode 
#[derive(Default)]
pub struct IntervalMapNode2 {
}

// Java: public static class IntervalMapTree extends IntervalTree 
#[derive(Default)]
pub struct IntervalMapTree {
    // Java field:       IntervalMapTree peer;
    pub peer: IntervalMapTree,
}

// Java: public static class Segment 
#[derive(Default)]
pub struct Segment {
    // Java field:       public int position1;
    pub position1: i32,
    // Java field:       public int position2;
    pub position2: i32,
    // Java field:       public int length;
    pub length: i32,
}

// Java: public class IntervalMap 
impl IntervalMap {
    // Java: public IntervalMap() 
    pub fn new() -> IntervalMap {
      Self { tree1: Default::default(), tree2: Default::default() }
    }

    // Java: public final IntervalTree getTree1() 
    pub fn getTree1(&mut self) -> &IntervalTree {
      // Java: return this.tree1;
      return self.tree1;
    }

    // Java: public final IntervalTree getTree2() 
    pub fn getTree2(&mut self) -> &IntervalTree {
      // Java: return this.tree2;
      return self.tree2;
    }

    // Java: protected IntervalMapNode1 createNode1() 
    pub fn createNode1(&mut self) -> &IntervalMapNode1 {
      // Java: return new IntervalMapNode1();
      return IntervalMapNode1::new();
    }

    // Java: protected IntervalMapNode2 createNode2() 
    pub fn createNode2(&mut self) -> &IntervalMapNode2 {
      // Java: return new IntervalMapNode2();
      return IntervalMapNode2::new();
    }

    // Java: private static final IntervalMapNode cast(Node node) 
    pub fn cast(node: Box<dyn Node>) -> IntervalMapNode {
      // Java: return (IntervalMapNode)node;
      return (node).downcast_ref::<IntervalMapNode>().unwrap();
    }

    // Java: public int forwardMapping(int start1, int def) 
    pub fn forwardMapping(&mut self, start1: i32, def: i32) -> i32 {
      // Java: return mapping(this.tree1, start1, def);
      return Self::mapping(self.tree1, start1, def);
    }

    // Java: public int backwardMapping(int start2, int def) 
    pub fn backwardMapping(&mut self, start2: i32, def: i32) -> i32 {
      // Java: return mapping(this.tree2, start2, def);
      return Self::mapping(self.tree2, start2, def);
    }

    // Java: private static int mapping(IntervalMapTree tree, int start, int def) 
    pub fn mapping(tree: IntervalMapTree, start: i32, def: i32) -> i32 {
      // Java: if (tree.size() == 1)
      if tree.size() == 1 {
         // Java: IntervalMapNode root = (IntervalMapNode)tree.getRoot();
         let root: IntervalMapNode = (tree.getRoot()).downcast_ref::<IntervalMapNode>().unwrap();
         // Java: int len = root.getLength();
         let len: i32 = root.getLength();
         // Java: int pos1 = root.offset;
         let pos1: i32 = root.offset;
         // Java: int pos2 = root.getPeer().offset;
         let pos2: i32 = root.getPeer().offset;
         // Java: return start >= pos1 && start < pos1 + len ? start - pos1 + pos2 : def;
         return (if start >= pos1 && start < pos1 + len { start - pos1 + pos2 } else { def });
      }
      // Java: else
      else {
         // Java: int pos = start;
         let pos: i32 = start;
         // Java: int off = 0;
         let off: i32 = 0;
         // Java: IntervalMapNode p1 = null;
         let mut p1: IntervalMapNode = IntervalMapNode::default();
         // Java: IntervalMapNode node = (IntervalMapNode)tree.getRoot();
         let node: IntervalMapNode = (tree.getRoot()).downcast_ref::<IntervalMapNode>().unwrap();
         // Java: while(node != null)
         while node.is_some() {
            // Java: if (pos < node.offset)
            if pos < node.offset {
               // Java: node = (IntervalMapNode)node.left;
               node = (node.left).downcast_ref::<IntervalMapNode>().unwrap();
            }
            // Java: else
            else {
               // Java: if (pos < node.offset + node.getLength())
               if pos < node.offset + node.getLength() {
                  // Java: off = pos - node.offset;
                  off = pos - node.offset;
                  // Java: p1 = node;
                  p1 = node;
                  // Java: break;
                  break;
               }
               // Java: pos -= node.offset;
               pos = pos - node.offset;
               // Java: node = (IntervalMapNode)node.right;
               node = (node.right).downcast_ref::<IntervalMapNode>().unwrap();
            }
         }
         // Java: return p1 != null ? tree.peer.position(p1.peer) + off : def;
         return (if p1.is_some() { tree.peer.position(p1.peer) + off } else { def });
      }
    }

    // Java: public IntervalMapNode addMapping1(int start1, int start2, int length) 
    pub fn addMapping1(&mut self, start1: i32, start2: i32, length: i32) -> &IntervalMapNode {
      // Java: if (!this.tree1.isEmpty())
      if !self.tree1.isEmpty() {
         // Java: if (this.tree2.isEmpty())
         if self.tree2.isEmpty() {
            // Java: throw new RuntimeException("The IntervalMap is in an insane state!");
            panic!("{}", RuntimeException::new("The IntervalMap is in an insane state!"));
         }
         // Java: IntervalMapNode1 node1 = (IntervalMapNode1)this.tree1.rightmost();
         let node1: IntervalMapNode1 = (self.tree1.rightmost()).downcast_ref::<IntervalMapNode1>().unwrap();
         // Java: IntervalMapNode2 node2 = (IntervalMapNode2)this.tree2.rightmost();
         let node2: IntervalMapNode2 = (self.tree2.rightmost()).downcast_ref::<IntervalMapNode2>().unwrap();
         // Java: if (node1.peer == node2)
         if node1.peer == node2 {
            // Java: int node1_pos = node1.position();
            let node1_pos: i32 = node1.position();
            // Java: int node2_pos = node2.position();
            let node2_pos: i32 = node2.position();
            // Java: int node_len = node1.mLength;
            let node_len: i32 = node1.mLength;
            // Java: if (node1_pos + node_len == start1 && node2_pos + node_len == start2)
            if node1_pos + node_len == start1 && node2_pos + node_len == start2 {
               // Java: node1.mLength += length;
               node1.mLength = node1.mLength + length;
               // Java: return node1;
               return node1;
            }
         }
      }
      // Java: IntervalMapNode p1 = (IntervalMapNode)this.tree1.lowerBound(start1);
      let p1: IntervalMapNode = (self.tree1.lowerBound(start1)).downcast_ref::<IntervalMapNode>().unwrap();
      // Java: IntervalMapNode p2 = (IntervalMapNode)this.tree2.lowerBound(start2);
      let p2: IntervalMapNode = (self.tree2.lowerBound(start2)).downcast_ref::<IntervalMapNode>().unwrap();
      // Java: if (p1 != this.tree1.mHead && p2 != this.tree2.mHead && p1.peer == p2 && start1 + length == this.tree1.position(p1) && start2 + length == this.tree2.position(p2))
      if p1 != self.tree1.mHead && p2 != self.tree2.mHead && p1.peer == p2 && start1 + length == self.tree1.position(p1) && start2 + length == self.tree2.position(p2) {
         // Java: p1.moveLeft(length);
         p1.moveLeft(length);
         // Java: p1.setLength(p1.getLength() + length);
         p1.setLength(p1.getLength() + length);
         // Java: p2.moveLeft(length);
         p2.moveLeft(length);
         // Java: return p1;
         return p1;
      }
      // Java: else
      else {
         // Java: IntervalMapNode p1_prev = (IntervalMapNode)p1.getPrev();
         let p1_prev: IntervalMapNode = (p1.getPrev()).downcast_ref::<IntervalMapNode>().unwrap();
         // Java: IntervalMapNode p2_prev = (IntervalMapNode)p2.getPrev();
         let p2_prev: IntervalMapNode = (p2.getPrev()).downcast_ref::<IntervalMapNode>().unwrap();
         // Java: int p1_prev_pos = this.tree1.position(p1_prev);
         let p1_prev_pos: i32 = self.tree1.position(p1_prev);
         // Java: int p2_prev_pos = this.tree2.position(p2_prev);
         let p2_prev_pos: i32 = self.tree2.position(p2_prev);
         // Java: if (p1_prev != this.tree1.mHead && p2_prev != this.tree2.mHead && p1_prev.peer == p2_prev && p1_prev_pos + p1_prev.getLength() == start1 && p2_prev_pos + p2_prev.getLength() == start2)
         if p1_prev != self.tree1.mHead && p2_prev != self.tree2.mHead && p1_prev.peer == p2_prev && p1_prev_pos + p1_prev.getLength() == start1 && p2_prev_pos + p2_prev.getLength() == start2 {
            // Java: p1_prev.setLength(p1_prev.getLength() + length);
            p1_prev.setLength(p1_prev.getLength() + length);
            // Java: return p1_prev;
            return p1_prev;
         }
         // Java: else
         else {
            // Java: IntervalMapNode n1 = this.createNode1();
            let n1: IntervalMapNode = self.createNode1();
            // Java: IntervalMapNode n2 = this.createNode2();
            let n2: IntervalMapNode = self.createNode2();
            // Java: n1.peer = n2;
            n1.peer = n2;
            // Java: n2.peer = n1;
            n2.peer = n1;
            // Java: n1.setLength(length);
            n1.setLength(length);
            // Java: n1.offset = start1 - p1_prev_pos;
            n1.offset = start1 - p1_prev_pos;
            // Java: n2.offset = start2 - p2_prev_pos;
            n2.offset = start2 - p2_prev_pos;
            // Java: this.tree1.insertRight(p1_prev, n1);
            self.tree1.insertRight(p1_prev, n1);
            // Java: this.tree2.insertRight(p2_prev, n2);
            self.tree2.insertRight(p2_prev, n2);
            // Java: return n1;
            return n1;
         }
      }
    }

    // Java: public void eraseMapping1(int start1, int length, ArrayList<Segment> segments) 
    pub fn eraseMapping1(&mut self, start1: i32, length: i32, segments: Vec<Segment>) {
      // Java: int end1 = start1 + length;
      let end1: i32 = start1 + length;
      // Java: IntervalTree.InsertSpace pHit = this.tree1.findForInsertSpace(start1);
      let pHit: InsertSpace = self.tree1.findForInsertSpace(start1);
      // Java: if (pHit.node != this.tree1.mHead)
      if pHit.node != self.tree1.mHead {
         // Java: IntervalMapNode s;
         let s: IntervalMapNode = IntervalMapNode::default();
         // Java: if (pHit.left)
         if pHit.left {
            // Java: s = (IntervalMapNode)pHit.node;
            s = (pHit.node).downcast_ref::<IntervalMapNode>().unwrap();
         }
         // Java: else
         else {
            // Java: if (pHit.pos + length < pHit.node.getLength())
            if pHit.pos + length < pHit.node.getLength() {
               // Java: if (segments != null)
               if segments.is_some() {
                  // Java: segments.add(new Segment(start1, this.tree2.position(cast(pHit.node).peer) + pHit.pos, length));
                  segments.add(Segment::new(start1, self.tree2.position(Self::cast(pHit.node).peer) + pHit.pos, length));
               }
               // Java: this.split1((IntervalMapNode)pHit.node, pHit.pos + length);
               self.split1((pHit.node).downcast_ref::<IntervalMapNode>().unwrap(), pHit.pos + length);
               // Java: pHit.node.setLength(pHit.pos);
               pHit.node.setLength(pHit.pos);
               // Java: return;
               return;
            }
            // Java: if (segments != null)
            if segments.is_some() {
               // Java: Segment segment = new Segment(start1, this.tree2.position(cast(pHit.node).peer) + pHit.pos, pHit.node.getLength() - pHit.pos);
               let segment: Segment = Segment::new(start1, self.tree2.position(Self::cast(pHit.node).peer) + pHit.pos, pHit.node.getLength() - pHit.pos);
               // Java: segments.add(segment);
               segments.add(segment);
            }
            // Java: pHit.node.setLength(pHit.pos);
            pHit.node.setLength(pHit.pos);
            // Java: s = (IntervalMapNode)pHit.node.getNext();
            s = (pHit.node.getNext()).downcast_ref::<IntervalMapNode>().unwrap();
         }
         // Java: Segment pLastSegment = null;
         let mut pLastSegment: Segment = Segment::default();
         // Java: pHit = this.tree1.findForInsertSpace(end1);
         pHit = self.tree1.findForInsertSpace(end1);
         // Java: IntervalMapNode e = (IntervalMapNode)pHit.node;
         let e: IntervalMapNode = (pHit.node).downcast_ref::<IntervalMapNode>().unwrap();
         // Java: if (e != this.tree1.mHead && !pHit.left)
         if e != self.tree1.mHead && !pHit.left {
            // Java: if (segments != null)
            if segments.is_some() {
               // Java: pLastSegment = new Segment(end1 - pHit.pos, this.tree2.position(cast(pHit.node).peer), pHit.pos);
               pLastSegment = Segment::new(end1 - pHit.pos, self.tree2.position(Self::cast(pHit.node).peer), pHit.pos);
            }
            // Java: e.moveRight(pHit.pos);
            e.moveRight(pHit.pos);
            // Java: e.setLength(e.getLength() - pHit.pos);
            e.setLength(e.getLength() - pHit.pos);
            // Java: e.peer.moveRight(pHit.pos);
            e.peer.moveRight(pHit.pos);
         }
         // Java: while(s != e)
         while s != e {
            // Java: if (segments != null)
            if segments.is_some() {
               // Java: segments.add(new Segment(this.tree1.position(s), this.tree2.position(s.peer), s.getLength()));
               segments.add(Segment::new(self.tree1.position(s), self.tree2.position(s.peer), s.getLength()));
            }
            // Java: IntervalMapNode s_next = (IntervalMapNode)s.getNext();
            let s_next: IntervalMapNode = (s.getNext()).downcast_ref::<IntervalMapNode>().unwrap();
            // Java: this.tree1.erase(s);
            self.tree1.erase(s);
            // Java: this.tree2.erase(s.peer);
            self.tree2.erase(s.peer);
            // Java: s = s_next;
            s = s_next;
         }
         // Java: if (segments != null && pLastSegment != null)
         if segments.is_some() && pLastSegment.is_some() {
            // Java: segments.add(pLastSegment);
            segments.add(pLastSegment);
         }
      }
    }

    // Java: public void insertSpace1(int start1, int length) 
    pub fn insertSpace1(&mut self, start1: i32, length: i32) {
      // Java: this.insertSpace(this.tree1, start1, length);
      self.insertSpace(self.tree1, start1, length);
    }

    // Java: public void insertSpace2(int start2, int length) 
    pub fn insertSpace2(&mut self, start2: i32, length: i32) {
      // Java: this.insertSpace(this.tree2, start2, length);
      self.insertSpace(self.tree2, start2, length);
    }

    // Java: private void insertSpace(IntervalMapTree tree, int start, int length) 
    pub fn insertSpace(&mut self, tree: IntervalMapTree, start: i32, length: i32) {
      // Java: if (!tree.isEmpty() && start < tree.getEndPosition())
      if !tree.isEmpty() && start < tree.getEndPosition() {
         // Java: IntervalTree.InsertSpace pInsertSpace = tree.findForInsertSpace(start);
         let pInsertSpace: InsertSpace = tree.findForInsertSpace(start);
         // Java: if (pInsertSpace.left)
         if pInsertSpace.left {
            // Java: tree.insertSpaceBefore(pInsertSpace.node, length);
            tree.insertSpaceBefore(pInsertSpace.node, length);
         }
         // Java: else
         else {
            // Java: IntervalMapNode pNew = this.split(tree, (IntervalMapNode)pInsertSpace.node, pInsertSpace.pos);
            let pNew: IntervalMapNode = self.split(tree, (pInsertSpace.node).downcast_ref::<IntervalMapNode>().unwrap(), pInsertSpace.pos);
            // Java: tree.insertSpaceBefore(pNew, length);
            tree.insertSpaceBefore(pNew, length);
         }
      }
    }

    // Java: private IntervalMapNode split1(IntervalMapNode node1, int length) 
    pub fn split1(&mut self, node1: IntervalMapNode, length: i32) -> &IntervalMapNode {
      // Java: return this.split(this.tree1, node1, length);
      return self.split(self.tree1, node1, length);
    }

    // Java: private IntervalMapNode split(IntervalMapTree tree, IntervalMapNode node, int length) 
    pub fn split(&mut self, tree: IntervalMapTree, node: IntervalMapNode, length: i32) -> &IntervalMapNode {
      // Java: IntervalMapNode node_peer = node.peer;
      let node_peer: IntervalMapNode = node.peer;
      // Java: int nNewNodeLength = node.getLength() - length;
      let nNewNodeLength: i32 = node.getLength() - length;
      // Java: IntervalMapNode new1 = this.createNode1();
      let new1: IntervalMapNode = self.createNode1();
      // Java: IntervalMapNode new2 = this.createNode2();
      let new2: IntervalMapNode = self.createNode2();
      // Java: new1.peer = new2;
      new1.peer = new2;
      // Java: new2.peer = new1;
      new2.peer = new1;
      // Java: new1.setLength(nNewNodeLength);
      new1.setLength(nNewNodeLength);
      // Java: node.setLength(length);
      node.setLength(length);
      // Java: tree.insertRight(node, new1, tree.position(node) + length);
      tree.insertRight(node, new1, tree.position(node) + length);
      // Java: tree.peer.insertRight(node_peer, new2, tree.peer.position(node_peer) + length);
      tree.peer.insertRight(node_peer, new2, tree.peer.position(node_peer) + length);
      // Java: return new1;
      return new1;
    }

    // Java: public final void eraseSpace1(int start1, int length) 
    pub fn eraseSpace1(&mut self, start1: i32, length: i32) {
      // Java: eraseSpace(this.tree1, start1, length);
      Self::eraseSpace(self.tree1, start1, length);
    }

    // Java: public final void eraseSpace2(int start2, int length) 
    pub fn eraseSpace2(&mut self, start2: i32, length: i32) {
      // Java: eraseSpace(this.tree2, start2, length);
      Self::eraseSpace(self.tree2, start2, length);
    }

    // Java: private static void eraseSpace(IntervalMapTree tree, int start, int length) 
    pub fn eraseSpace(tree: IntervalMapTree, start: i32, length: i32) {
      // Java: IntervalMapNode n1 = (IntervalMapNode)tree.lowerBound(start);
      let n1: IntervalMapNode = (tree.lowerBound(start)).downcast_ref::<IntervalMapNode>().unwrap();
      // Java: tree.eraseSpaceBefore(n1, length);
      tree.eraseSpaceBefore(n1, length);
      // Java: if (n1 != tree.mHead)
      if n1 != tree.mHead {
         // Java: IntervalMapNode n1_prev = (IntervalMapNode)n1.getPrev();
         let n1_prev: IntervalMapNode = (n1.getPrev()).downcast_ref::<IntervalMapNode>().unwrap();
         // Java: if (n1_prev != tree.mHead)
         if n1_prev != tree.mHead {
            // Java: IntervalMapNode n2 = n1.peer;
            let n2: IntervalMapNode = n1.peer;
            // Java: IntervalMapNode n2_prev = (IntervalMapNode)n2.getPrev();
            let n2_prev: IntervalMapNode = (n2.getPrev()).downcast_ref::<IntervalMapNode>().unwrap();
            // Java: if (n1_prev.peer == n2_prev && tree.position(n1_prev) + n1_prev.getLength() == tree.position(n1) && tree.peer.position(n2_prev) + n2_prev.getLength() == tree.peer.position(n2))
            if n1_prev.peer == n2_prev && tree.position(n1_prev) + n1_prev.getLength() == tree.position(n1) && tree.peer.position(n2_prev) + n2_prev.getLength() == tree.peer.position(n2) {
               // Java: n1_prev.setLength(n1_prev.getLength() + n1.getLength());
               n1_prev.setLength(n1_prev.getLength() + n1.getLength());
               // Java: tree.erase(n1);
               tree.erase(n1);
               // Java: tree.peer.erase(n2);
               tree.peer.erase(n2);
            }
         }
      }
    }

    // Java: public String toString() 
    pub fn toString(&mut self) -> String {
      // Java: StringBuilder sb = new StringBuilder();
      let sb: String = StringBuilder::new();
      // Java: sb.append(this.getClass().getSimpleName());
      sb.append(self.getClass().getSimpleName());
      // Java: sb.append("\ncontent = {\n");
      sb.append("\ncontent = {\n");
      let mut node: IntervalMapNode = (self.tree1.leftmost()).downcast_ref::<IntervalMapNode>().unwrap();
      // Java: for(IntervalMapNode node = (IntervalMapNode)this.tree1.leftmost(); !node.isHead(); node = (IntervalMapNode)node.getNext())
      while !node.isHead() {
         // Java: int cp = node.position();
         let cp: i32 = node.position();
         // Java: int fc = node.peer.position();
         let fc: i32 = node.peer.position();
         // Java: int len = node.getLength();
         let len: i32 = node.getLength();
         // Java: sb.append(String.format("[%d, %d)\t<=>\t[%d, %d),\n", cp, cp + len, fc, fc + len));
         sb.append(String::format("[%d, %d)\t<=>\t[%d, %d),\n", cp, cp + len, fc, fc + len));
         node = (node.getNext()).downcast_ref::<IntervalMapNode>().unwrap();
      }
      // Java: sb.append("}\n");
      sb.append("}\n");
      // Java: return sb.toString();
      return sb.toString();
    }

}

// Java: public static class IntervalMapNode1 extends IntervalMapNode 
impl IntervalMapNode1 {
    // Java: public int getLength() 
    pub fn getLength(&mut self) -> i32 {
         // Java: return this.mLength;
         return self.mLength;
    }

    // Java: public void setLength(int length) 
    pub fn setLength(&mut self, length: i32) {
         // Java: this.mLength = length;
         self.mLength = length;
    }

}

// Java: public static class IntervalMapNode2 extends IntervalMapNode 
impl IntervalMapNode2 {
    // Java: public int getLength() 
    pub fn getLength(&mut self) -> i32 {
         // Java: return this.peer.getLength();
         return self.peer.getLength();
    }

    // Java: public void setLength(int length) 
    pub fn setLength(&mut self, length: i32) {
         // Java: this.peer.setLength(length);
         self.peer.setLength(length);
    }

}

// Java: public static class IntervalMapTree extends IntervalTree 
impl IntervalMapTree {
    // Java: public IntervalMapTree(IntervalMapNode head) 
    pub fn new(head: IntervalMapNode) -> IntervalMapTree {
      Self { peer: Default::default() }
    }

}

// Java: public static class Segment 
impl Segment {
    // Java: public Segment(int position1, int position2, int length) 
    pub fn new(position1: i32, position2: i32, length: i32) -> Segment {
      Self { position1: position1, position2: position2, length: length }
    }

    // Java: public String toString() 
    pub fn toString(&mut self) -> String {
         // Java: return "Segment: [" + this.position1 + " <=> " + this.position2 + ", " + this.length + "]";
         return "Segment: [" + self.position1 + " <=> " + self.position2 + ", " + self.len() + "]";
    }

}
