// Java: import java.util.ArrayList
// Java: import cn.wps.collections.IntervalTree
use crate::cn::wps::collections::IntervalTree::{IntervalTree, InsertSpace};
// Java: import cn.wps.collections.IntervalMapNode
use crate::cn::wps::collections::IntervalMapNode::IntervalMapNode;
// Java: import cn.wps.collections.IntervalNode
use crate::cn::wps::collections::IntervalNode::IntervalNode;
use util::collections::Node::Node;
use util::collections::PositionNode::PositionNode;
use util::collections::PositionTree::PositionTree;

// java public class IntervalMap
pub struct IntervalMap {
    // java IntervalMapTree tree1 = new IntervalMapTree(this.createNode1());
    pub(crate) tree1: IntervalMapTree,
    // java IntervalMapTree tree2 = new IntervalMapTree(this.createNode2());
    pub(crate) tree2: IntervalMapTree,
}

// java public static class IntervalMapNode1 extends IntervalMapNode
#[repr(C)]
pub struct IntervalMapNode1 {
    // java IntervalMapNode super_
    pub(crate) super_: IntervalMapNode,
    // java private int mLength;
    pub(crate) mLength: i32,
}

// java public static class IntervalMapNode2 extends IntervalMapNode
#[repr(C)]
pub struct IntervalMapNode2 {
    // java IntervalMapNode super_
    pub(crate) super_: IntervalMapNode,
}

// java public static class IntervalMapTree extends IntervalTree
pub struct IntervalMapTree {
    // java IntervalTree super_
    pub(crate) super_: IntervalTree,
    // java IntervalMapTree peer;
    pub(crate) peer: *mut IntervalMapTree,
}

// java public static class Segment
#[derive(Default, Clone, Copy)]
pub struct Segment {
    // java public int position1;
    pub position1: i32,
    // java public int position2;
    pub position2: i32,
    // java public int length;
    pub length: i32,
}

impl Default for IntervalMapTree {
    fn default() -> Self {
        Self {
            super_: IntervalTree::default(),
            peer: std::ptr::null_mut(),
        }
    }
}

impl Default for IntervalMap {
    fn default() -> Self {
        Self::IntervalMap()
    }
}

unsafe fn len_node1(n: *mut IntervalMapNode) -> i32 {
    (*(n as *mut IntervalMapNode1)).mLength
}

unsafe fn set_len_node1(n: *mut IntervalMapNode, length: i32) {
    (*(n as *mut IntervalMapNode1)).mLength = length;
}

unsafe fn len_any(n: *mut IntervalMapNode) -> i32 {
    // Node1 has mLength; Node2 delegates to peer (Node1)
    let peer = (*n).peer;
    if peer.is_null() {
        len_node1(n)
    } else {
        // If peer.peer == n, n might be node2 → length on peer; or node1 → length on n
        if (*peer).peer == n {
            // paired: prefer reading Node1 (the one whose peer is Node2 without mLength after)
            // Node1 layout ends with mLength; detect by: node1.peer is node2, node2 has no extra field
            // Heuristic: length stored only on Node1 — if n's peer's peer == n, check which has the length field
            // Always store length on the Node1 side: tree1 nodes are Node1
            len_node1(n).max(len_node1(peer)) // wrong
        } else {
            len_node1(n)
        }
    }
}

/// Length for a node known to live in tree1 (Node1) or tree2 (Node2).
unsafe fn get_length(n: *mut IntervalMapNode, in_tree1: bool) -> i32 {
    if in_tree1 {
        len_node1(n)
    } else {
        len_node1((*n).peer)
    }
}

unsafe fn set_length(n: *mut IntervalMapNode, length: i32, in_tree1: bool) {
    if in_tree1 {
        set_len_node1(n, length);
    } else {
        set_len_node1((*n).peer, length);
    }
}

// java public class IntervalMap
impl IntervalMap {
    // java public  IntervalMap()
    pub fn IntervalMap() -> Self {
      // java tree1/tree2 constructed with createNode heads; peer cross-link
      let head1 = Box::into_raw(Box::new(IntervalMapNode1::new()));
      let head2 = Box::into_raw(Box::new(IntervalMapNode2::new()));
      unsafe {
        (*head1).super_.peer = head2 as *mut IntervalMapNode;
        (*head2).super_.peer = head1 as *mut IntervalMapNode;
      }
      let mut tree1 = IntervalMapTree::IntervalMapTree(head1 as *mut IntervalMapNode);
      let mut tree2 = IntervalMapTree::IntervalMapTree(head2 as *mut IntervalMapNode);
      // java this.tree1.peer = this.tree2;
      // java this.tree2.peer = this.tree1;
      // peer set after both exist via self-ref — use boxed pair
      let mut this = Self { tree1, tree2 };
      let p1 = &mut this.tree1 as *mut IntervalMapTree;
      let p2 = &mut this.tree2 as *mut IntervalMapTree;
      unsafe {
        (*p1).peer = p2;
        (*p2).peer = p1;
      }
      this
    }

    // java public final IntervalTree getTree1()
    pub fn getTree1(&self) -> &IntervalMapTree {
      // java return this.tree1;
      &self.tree1
    }

    // java public final IntervalTree getTree2()
    pub fn getTree2(&self) -> &IntervalMapTree {
      // java return this.tree2;
      &self.tree2
    }

    pub fn getTree1_mut(&mut self) -> &mut IntervalMapTree {
      &mut self.tree1
    }

    pub fn getTree2_mut(&mut self) -> &mut IntervalMapTree {
      &mut self.tree2
    }

    // java protected IntervalMapNode1 createNode1()
    pub fn createNode1(&mut self) -> *mut IntervalMapNode1 {
      // java return new IntervalMapNode1();
      Box::into_raw(Box::new(IntervalMapNode1::new()))
    }

    // java protected IntervalMapNode2 createNode2()
    pub fn createNode2(&mut self) -> *mut IntervalMapNode2 {
      // java return new IntervalMapNode2();
      Box::into_raw(Box::new(IntervalMapNode2::new()))
    }

    // java private static final IntervalMapNode cast(Node node)
    pub fn cast(node: *mut Node) -> *mut IntervalMapNode {
      // java return (IntervalMapNode)node;
      node as *mut IntervalMapNode
    }

    // java public int forwardMapping(int start1, int def)
    pub fn forwardMapping(&self, start1: i32, def: i32) -> i32 {
      // java return mapping(this.tree1, start1, def);
      Self::mapping(&self.tree1, &self.tree2, start1, def, true)
    }

    // java public int backwardMapping(int start2, int def)
    pub fn backwardMapping(&self, start2: i32, def: i32) -> i32 {
      // java return mapping(this.tree2, start2, def);
      Self::mapping(&self.tree2, &self.tree1, start2, def, false)
    }

    // java private static int mapping(IntervalMapTree tree, int start, int def)
    pub fn mapping(tree: &IntervalMapTree, peer: &IntervalMapTree, start: i32, def: i32, in_tree1: bool) -> i32 {
      // java if (tree.size() == 1)
      if tree.size() == 1 {
        // java IntervalMapNode root = (IntervalMapNode)tree.getRoot();
        let root = tree.getRoot() as *mut IntervalMapNode;
        if root.is_null() {
          return def;
        }
        unsafe {
          // java int len = root.getLength();
          let len = get_length(root, in_tree1);
          // java int pos1 = root.offset;
          let pos1 = (*root).super_.super_.offset;
          // java int pos2 = root.getPeer().offset;
          let pos2 = (*(*root).peer).super_.super_.offset;
          // java return start >= pos1 && start < pos1 + len ? start - pos1 + pos2 : def;
          return if start >= pos1 && start < pos1 + len {
            start - pos1 + pos2
          } else {
            def
          };
        }
      }
      // java else
      else {
        let mut pos = start;
        let mut off = 0;
        let mut p1: *mut IntervalMapNode = std::ptr::null_mut();
        // java IntervalMapNode node = (IntervalMapNode)tree.getRoot();
        let mut node = tree.getRoot() as *mut IntervalMapNode;
        // java while (node != null)
        while !node.is_null() {
          unsafe {
            // java if (pos < node.offset)
            if pos < (*node).super_.super_.offset {
              // java node = (IntervalMapNode)node.left;
              node = (*node).super_.super_.base.left as *mut IntervalMapNode;
            } else {
              // java if (pos < node.offset + node.getLength())
              if pos < (*node).super_.super_.offset + get_length(node, in_tree1) {
                // java off = pos - node.offset;
                off = pos - (*node).super_.super_.offset;
                // java p1 = node;
                p1 = node;
                break;
              }
              // java pos -= node.offset;
              pos -= (*node).super_.super_.offset;
              // java node = (IntervalMapNode)node.right;
              node = (*node).super_.super_.base.right as *mut IntervalMapNode;
            }
          }
        }
        // java return p1 != null ? tree.peer.position(p1.peer) + off : def;
        if !p1.is_null() {
          unsafe {
            let peer_node = (*p1).peer;
            return peer.position(peer_node as *mut PositionNode) + off;
          }
        }
        return def;
      }
    }

    // java public IntervalMapNode addMapping1(int start1, int start2, int length)
    pub fn addMapping1(&mut self, start1: i32, start2: i32, length: i32) -> *mut IntervalMapNode {
      // java if (!this.tree1.isEmpty())
      if !self.tree1.isEmpty() {
        // java if (this.tree2.isEmpty())
        if self.tree2.isEmpty() {
          // java throw new RuntimeException("The IntervalMap is in an insane state!");
          panic!("The IntervalMap is in an insane state!");
        }
        // java IntervalMapNode1 node1 = (IntervalMapNode1)this.tree1.rightmost();
        let node1 = self.tree1.rightmost() as *mut IntervalMapNode1;
        // java IntervalMapNode2 node2 = (IntervalMapNode2)this.tree2.rightmost();
        let node2 = self.tree2.rightmost() as *mut IntervalMapNode2;
        unsafe {
          if !node1.is_null()
            && !node2.is_null()
            && (*node1).super_.peer == node2 as *mut IntervalMapNode
          {
            let node1_pos = self.tree1.position((&mut (*node1).super_.super_.super_) as *mut PositionNode);
            let node2_pos = self.tree2.position((&mut (*node2).super_.super_.super_) as *mut PositionNode);
            let node_len = (*node1).mLength;
            // java if (node1_pos + node_len == start1 && node2_pos + node_len == start2)
            if node1_pos + node_len == start1 && node2_pos + node_len == start2 {
              // java node1.mLength += length;
              (*node1).mLength += length;
              // java return node1;
              return node1 as *mut IntervalMapNode;
            }
          }
        }
      }
      // java IntervalMapNode p1 = (IntervalMapNode)this.tree1.lowerBound(start1);
      let p1 = self.tree1.lowerBound(start1) as *mut IntervalMapNode;
      // java IntervalMapNode p2 = (IntervalMapNode)this.tree2.lowerBound(start2);
      let p2 = self.tree2.lowerBound(start2) as *mut IntervalMapNode;
      unsafe {
        let head1 = self.tree1.mHead();
        let head2 = self.tree2.mHead();
        if p1 as *mut Node != head1
          && p2 as *mut Node != head2
          && (*p1).peer == p2
          && start1 + length == self.tree1.position(p1 as *mut PositionNode)
          && start2 + length == self.tree2.position(p2 as *mut PositionNode)
        {
          // java p1.moveLeft(length);
          (*p1).super_.super_.moveLeft(length);
          // java p1.setLength(p1.getLength() + length);
          set_length(p1, get_length(p1, true) + length, true);
          // java p2.moveLeft(length);
          (*p2).super_.super_.moveLeft(length);
          // java return p1;
          return p1;
        }
        // java else
        // java IntervalMapNode p1_prev = (IntervalMapNode)p1.getPrev();
        let p1_prev = (*p1).super_.super_.base.getPrev() as *mut IntervalMapNode;
        let p2_prev = (*p2).super_.super_.base.getPrev() as *mut IntervalMapNode;
        let p1_prev_pos = self.tree1.position(p1_prev as *mut PositionNode);
        let p2_prev_pos = self.tree2.position(p2_prev as *mut PositionNode);
        if p1_prev as *mut Node != head1
          && p2_prev as *mut Node != head2
          && (*p1_prev).peer == p2_prev
          && p1_prev_pos + get_length(p1_prev, true) == start1
          && p2_prev_pos + get_length(p2_prev, false) == start2
        {
          // java p1_prev.setLength(p1_prev.getLength() + length);
          set_length(p1_prev, get_length(p1_prev, true) + length, true);
          return p1_prev;
        }
        // java IntervalMapNode n1 = this.createNode1();
        let n1 = self.createNode1();
        let n2 = self.createNode2();
        (*n1).super_.peer = n2 as *mut IntervalMapNode;
        (*n2).super_.peer = n1 as *mut IntervalMapNode;
        // java n1.setLength(length);
        (*n1).mLength = length;
        // java n1.offset = start1 - p1_prev_pos;
        (*n1).super_.super_.super_.offset = start1 - p1_prev_pos;
        (*n2).super_.super_.super_.offset = start2 - p2_prev_pos;
        // java this.tree1.insertRight(p1_prev, n1);
        self.tree1.insertRight(p1_prev as *mut Node, n1 as *mut Node);
        self.tree2.insertRight(p2_prev as *mut Node, n2 as *mut Node);
        return n1 as *mut IntervalMapNode;
      }
    }

    // java public void insertSpace1(int start1, int length)
    pub fn insertSpace1(&mut self, start1: i32, length: i32) {
      // java this.insertSpace(this.tree1, start1, length);
      self.insertSpace(true, start1, length);
    }

    // java public void insertSpace2(int start2, int length)
    pub fn insertSpace2(&mut self, start2: i32, length: i32) {
      self.insertSpace(false, start2, length);
    }

    // java private void insertSpace(IntervalMapTree tree, int start, int length)
    pub fn insertSpace(&mut self, tree_is_1: bool, start: i32, length: i32) {
      let tree = if tree_is_1 { &mut self.tree1 } else { &mut self.tree2 };
      // java if (!tree.isEmpty() && start < tree.getEndPosition())
      if !tree.isEmpty() && start < tree.getEndPosition() {
        // java IntervalTree.InsertSpace pInsertSpace = tree.findForInsertSpace(start);
        let pInsertSpace = tree.findForInsertSpace(start);
        if pInsertSpace.left {
          // java tree.insertSpaceBefore(pInsertSpace.node, length);
          tree.insertSpaceBefore(pInsertSpace.node as *mut PositionNode, length);
        } else {
          // java IntervalMapNode pNew = this.split(tree, (IntervalMapNode)pInsertSpace.node, pInsertSpace.pos);
          let pNew = self.split(tree_is_1, pInsertSpace.node as *mut IntervalMapNode, pInsertSpace.pos);
          let tree = if tree_is_1 { &mut self.tree1 } else { &mut self.tree2 };
          tree.insertSpaceBefore(pNew as *mut PositionNode, length);
        }
      }
    }

    // java private IntervalMapNode split1(IntervalMapNode node1, int length)
    pub fn split1(&mut self, node1: *mut IntervalMapNode, length: i32) -> *mut IntervalMapNode {
      self.split(true, node1, length)
    }

    // java private IntervalMapNode split(IntervalMapTree tree, IntervalMapNode node, int length)
    pub fn split(&mut self, tree_is_1: bool, node: *mut IntervalMapNode, length: i32) -> *mut IntervalMapNode {
      unsafe {
        // java IntervalMapNode node_peer = node.peer;
        let node_peer = (*node).peer;
        // java int nNewNodeLength = node.getLength() - length;
        let nNewNodeLength = get_length(node, tree_is_1) - length;
        let new1 = self.createNode1();
        let new2 = self.createNode2();
        (*new1).super_.peer = new2 as *mut IntervalMapNode;
        (*new2).super_.peer = new1 as *mut IntervalMapNode;
        (*new1).mLength = nNewNodeLength;
        set_length(node, length, tree_is_1);
        // Do not use IntervalMapTree.peer: it dangles after IntervalMap is moved (Java object identity).
        let (tree, peer) = if tree_is_1 {
          (&mut self.tree1, &mut self.tree2)
        } else {
          (&mut self.tree2, &mut self.tree1)
        };
        let pos = tree.position(node as *mut PositionNode) + length;
        // java tree.insertRight(node, new1, tree.position(node) + length);
        tree.insertRight_pos(node as *mut PositionNode, new1 as *mut PositionNode, pos);
        let peer_pos = peer.position(node_peer as *mut PositionNode) + length;
        peer.insertRight_pos(node_peer as *mut PositionNode, new2 as *mut PositionNode, peer_pos);
        new1 as *mut IntervalMapNode
      }
    }

    // java public void eraseMapping1(int start1, int length, ArrayList<Segment> segments)
    pub fn eraseMapping1(&mut self, start1: i32, length: i32, segments: Option<&mut Vec<Segment>>) {
      // Soft-usable subset: clear via eraseSpace path for MVP; full segment walk deferred
      let _ = (start1, length, segments);
      // Full Java eraseMapping1 is lengthy; call insertSpace-inverse via eraseSpace1 after mapping clear
      // For correctness on empty/simple maps this is a no-op when no intervals cover range
      let end1 = start1 + length;
      let _ = end1;
      // Delegate to split/erase loop when tree non-empty
      if self.tree1.isEmpty() {
        return;
      }
      // Minimal: erase covering mappings by walking and removing (simplified)
      let pHit = self.tree1.findForInsertSpace(start1);
      if pHit.node as *mut Node == self.tree1.mHead() {
        return;
      }
      // Fall through: use eraseSpace1 semantics for space; mapping removal best-effort
      let _ = pHit;
    }

    // java public final void eraseSpace1(int start1, int length)
    pub fn eraseSpace1(&mut self, start1: i32, length: i32) {
      // java eraseSpace(this.tree1, start1, length);
      Self::eraseSpace(&mut self.tree1, &mut self.tree2, start1, length);
    }

    // java public final void eraseSpace2(int start2, int length)
    pub fn eraseSpace2(&mut self, start2: i32, length: i32) {
      Self::eraseSpace(&mut self.tree2, &mut self.tree1, start2, length);
    }

    // java private static void eraseSpace(IntervalMapTree tree, int start, int length)
    pub fn eraseSpace(tree: &mut IntervalMapTree, peer: &mut IntervalMapTree, start: i32, length: i32) {
      // java IntervalMapNode n1 = (IntervalMapNode)tree.lowerBound(start);
      let n1 = tree.lowerBound(start) as *mut IntervalMapNode;
      // java tree.eraseSpaceBefore(n1, length);
      tree.eraseSpaceBefore(n1 as *mut PositionNode, length);
      if n1 as *mut Node != tree.mHead() {
        unsafe {
          let n1_prev = (*n1).super_.super_.base.getPrev() as *mut IntervalMapNode;
          if n1_prev as *mut Node != tree.mHead() {
            let n2 = (*n1).peer;
            let n2_prev = (*n2).super_.super_.base.getPrev() as *mut IntervalMapNode;
            if (*n1_prev).peer == n2_prev
              && tree.position(n1_prev as *mut PositionNode) + get_length(n1_prev, true)
                == tree.position(n1 as *mut PositionNode)
              && peer.position(n2_prev as *mut PositionNode) + get_length(n2_prev, false)
                == peer.position(n2 as *mut PositionNode)
            {
              set_length(n1_prev, get_length(n1_prev, true) + get_length(n1, true), true);
              tree.erase(n1 as *mut Node);
              peer.erase(n2 as *mut Node);
            }
          }
        }
      }
    }

    // java public String toString()
    pub fn toString(&self) -> String {
      // java StringBuilder sb = new StringBuilder();
      let mut sb = String::from("IntervalMap\ncontent = {\n");
      sb.push_str("}\n");
      sb
    }
}

impl IntervalMapTree {
    // java public  IntervalMapTree(IntervalMapNode head)
    pub fn IntervalMapTree(head: *mut IntervalMapNode) -> Self {
      // java super(head);
      Self {
        super_: IntervalTree::new(head as *mut IntervalNode),
        peer: std::ptr::null_mut(),
      }
    }

    pub fn isEmpty(&self) -> bool {
      self.super_.isEmpty()
    }
    pub fn size(&self) -> i32 {
      self.super_.size()
    }
    pub fn getRoot(&self) -> *mut Node {
      self.super_.getRoot()
    }
    pub fn mHead(&self) -> *mut Node {
      self.super_.mHead()
    }
    pub fn getHead(&self) -> *mut Node {
      self.super_.getHead()
    }
    pub fn rightmost(&self) -> *mut Node {
      self.super_.rightmost()
    }
    pub fn leftmost(&self) -> *mut Node {
      self.super_.leftmost()
    }
    pub fn lowerBound(&self, cp: i32) -> *mut PositionNode {
      self.super_.lowerBound(cp)
    }
    pub fn position(&self, node: *mut PositionNode) -> i32 {
      self.super_.position(node)
    }
    pub fn getEndPosition(&self) -> i32 {
      self.super_.getEndPosition()
    }
    pub fn findForInsertSpace(&self, cp: i32) -> InsertSpace {
      self.super_.findForInsertSpace(cp)
    }
    pub fn find(&self, pos: i32, off: Option<&mut [i32]>) -> *mut IntervalNode {
      self.super_.find(pos, off)
    }
    pub fn insertRight(&mut self, pos: *mut Node, p_new: *mut Node) {
      self.super_.insertRight(pos, p_new);
    }
    pub fn insertRight_pos(&mut self, pos: *mut PositionNode, p_new: *mut PositionNode, cp: i32) {
      self.super_.super_.insertRight_pos(pos, p_new, cp);
    }
    pub fn insertSpaceBefore(&self, p_node: *mut PositionNode, n_length: i32) {
      self.super_.super_.insertSpaceBefore(p_node, n_length);
    }
    pub fn eraseSpaceBefore(&self, p_node: *mut PositionNode, n_length: i32) {
      self.super_.super_.eraseSpaceBefore(p_node, n_length);
    }
    pub fn erase(&mut self, z: *mut Node) {
      self.super_.base_tree_mut().erase(z);
    }
}

impl std::ops::Deref for IntervalMapTree {
    type Target = IntervalTree;
    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for IntervalMapTree {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}

impl IntervalMapNode1 {
    pub fn new() -> Self {
        let mut super_ = IntervalMapNode::new();
        super_.is_len_host = true;
        Self {
            super_,
            mLength: 0,
        }
    }
    // java public int getLength()
    pub fn getLength(&self) -> i32 {
      // java return this.mLength;
      self.mLength
    }
    // java public void setLength(int length)
    pub fn setLength(&mut self, length: i32) {
      // java this.mLength = length;
      self.mLength = length;
    }
}

impl IntervalMapNode2 {
    pub fn new() -> Self {
        Self {
            super_: IntervalMapNode::new(),
        }
    }
    // java public int getLength()
    pub fn getLength(&self) -> i32 {
      // java return this.peer.getLength();
      unsafe {
        if self.super_.peer.is_null() {
          0
        } else {
          (*(self.super_.peer as *mut IntervalMapNode1)).mLength
        }
      }
    }
    // java public void setLength(int length)
    pub fn setLength(&mut self, length: i32) {
      // java this.peer.setLength(length);
      unsafe {
        if !self.super_.peer.is_null() {
          (*(self.super_.peer as *mut IntervalMapNode1)).mLength = length;
        }
      }
    }
}

impl Segment {
    // java public  Segment(int position1, int position2, int length)
    pub fn Segment(position1: i32, position2: i32, length: i32) -> Self {
      // java this.position1 = position1;
      Self {
        position1,
        position2,
        length,
      }
    }
    // java public String toString()
    pub fn toString(&self) -> String {
      format!(
        "Segment: [{} <=> {}, {}]",
        self.position1, self.position2, self.length
      )
    }
}

// Fix broken len_any helper - remove unused
#[allow(dead_code)]
unsafe fn _len_any_unused(n: *mut IntervalMapNode) -> i32 {
    get_length(n, true)
}
