#![allow(non_snake_case, dead_code, unused_variables)]

use std::ptr;
use super::Node::Node;
use super::Tree::Tree;
use super::ITreeBalancer::ITreeBalancer;
use super::ITreeExtension::ITreeExtension;
use super::PositionNode::PositionNode;
use super::PositionTreeExtension::PositionTreeExtension;

struct RBBalancer {}

impl ITreeBalancer for RBBalancer {
    fn onCreateHead(&mut self, _tree: *mut u8, node: *mut u8) {
        unsafe {
            let node = node as *mut Node;
            (*node).flags &= !1;
        }
    }
    fn onBinaryTreeInserted(&mut self, _tree: *mut u8, _node: *mut u8) {}
    fn beforeSwap(&mut self, _tree: *mut u8, _var2: *mut u8, _var3: *mut u8) {}
    fn onBinaryTreeErased(&mut self, _tree: *mut u8, _var2: *mut u8, _var3: *mut u8, _var4: *mut u8) {}
    fn getTreeCreator(&self) -> Option<Box<dyn std::any::Any>> { None }
}

// Java: public class PositionTree extends Tree
pub struct PositionTree {
    // Java field: (base class Tree)
    pub base: Tree,
    // Java field: PositionTreeExtension positions
    pub(crate) positions: PositionTreeExtension,
}

// Java: public class PositionTree extends Tree
impl PositionTree {
    // Java: PositionTree(PositionNode head)
    pub fn new(head: *mut PositionNode) -> Self {
        // Java: super(head, new RBBalancer());
        let head_node = head as *mut Node;
        let mut tree = Tree::new(head_node, Box::new(RBBalancer {}));
        let mut positions = PositionTreeExtension::new();
        // Java: this.addExtension(this.positions, 63);
        // Same object in Java (field + extension list). Rust cannot Box the field,
        // so onCreateHead the field used by lowerBound/insertSpaceBefore, and a
        // second extension receives Tree insert/erase callbacks.
        let tree_ptr = &mut tree as *mut Tree as *mut u8;
        ITreeExtension::onCreateHead(&mut positions, tree_ptr, head_node as *mut u8);
        tree.addExtension(Box::new(PositionTreeExtension::new()), 63);
        Self {
            base: tree,
            positions,
        }
    }

    // Java: int position(PositionNode node)
    pub fn position(&self, node: *mut PositionNode) -> i32 {
        // Java: return this.positions.position(node);
        self.positions.position(node as *mut Node)
    }

    // Java: PositionNode lowerBound(int cp)
    pub fn lowerBound(&self, cp: i32) -> *mut PositionNode {
        // Java: return this.lowerBound(cp, (int[])null);
        self.lowerBound_with_offset(cp, None)
    }

    // Java: PositionNode upperBound(int cp)
    pub fn upperBound(&self, cp: i32) -> *mut PositionNode {
        // Java: return this.upperBound(cp, (int[])null);
        self.upperBound_with_offset(cp, None)
    }

    // Java: PositionNode lowerBound(int cp, _offset: &[i32])
    pub fn lowerBound_with_offset(&self, cp: i32, offset: Option<&mut [i32]>) -> *mut PositionNode {
        // Java: return (PositionNode)this.positions.lowerBound(cp, offset);
        self.positions.lowerBound_with_offset(cp, offset) as *mut PositionNode
    }

    // Java: PositionNode upperBound(int cp, _offset: &[i32])
    pub fn upperBound_with_offset(&self, cp: i32, offset: Option<&mut [i32]>) -> *mut PositionNode {
        // Java: return (PositionNode)this.positions.upperBound(cp, offset);
        self.positions.upperBound_with_offset(cp, offset) as *mut PositionNode
    }

    // Java: PositionNode insertSpaceFromLowerBound(int cp, int length)
    pub fn insertSpaceFromLowerBound(&self, cp: i32, length: i32) -> *mut PositionNode {
        // Java: PositionNode pNode = this.lowerBound(cp);
        let p_node = self.lowerBound(cp);
        // Java: this.insertSpaceBefore(pNode, length);
        self.insertSpaceBefore(p_node, length);
        // Java: return pNode;
        p_node
    }

    // Java: int insertLeft(PositionNode pos, PositionNode pNew, int cp)
    pub fn insertLeft_pos(&mut self, pos: *mut PositionNode, p_new: *mut PositionNode, cp: i32) -> i32 {
        // Java: _offset: &[i32] = new int[]{0};
        let offset = std::cell::RefCell::new([0i32]);
        // Java: this.insertLeft(pos, pNew, cp, offset);
        self.base.insertLeft(pos as *mut Node, p_new as *mut Node, cp, Some(&offset as &dyn std::any::Any));
        // Java: return offset[0];
        let result = offset.borrow()[0];
        result
    }

    // Java: int insertRight(PositionNode pos, PositionNode pNew, int cp)
    pub fn insertRight_pos(&mut self, pos: *mut PositionNode, p_new: *mut PositionNode, cp: i32) -> i32 {
        // Java: _offset: &[i32] = new int[]{0};
        let offset = std::cell::RefCell::new([0i32]);
        // Java: this.insertRight(pos, pNew, cp, offset);
        self.base.insertRight(pos as *mut Node, p_new as *mut Node, cp, Some(&offset as &dyn std::any::Any));
        // Java: return offset[0];
        let result = offset.borrow()[0];
        result
    }

    // Java: void moveRight(PositionNode node, int nLength)
    pub fn moveRight(&self, node: *mut PositionNode, n_length: i32) {
        // Java: this.positions.moveRight(node, nLength);
        self.positions.moveRight(node as *mut Node, n_length);
    }

    // Java: PositionNode insertSpaceFromUpperBound(int cp, int length)
    pub fn insertSpaceFromUpperBound(&self, cp: i32, length: i32) -> *mut PositionNode {
        // Java: PositionNode pNode = this.upperBound(cp);
        let p_node = self.upperBound(cp);
        // Java: this.insertSpaceBefore(pNode, length);
        self.insertSpaceBefore(p_node, length);
        // Java: return pNode;
        p_node
    }

    // Java: PositionNode removeSpaceFromLowerBound(int cp, int length)
    pub fn removeSpaceFromLowerBound(&self, cp: i32, length: i32) -> *mut PositionNode {
        // Java: return this.insertSpaceFromLowerBound(cp, -length);
        self.insertSpaceFromLowerBound(cp, -length)
    }

    // Java: PositionNode removeSpaceFromUpperBound(int cp, int length)
    pub fn removeSpaceFromUpperBound(&self, cp: i32, length: i32) -> *mut PositionNode {
        // Java: return this.insertSpaceFromUpperBound(cp, -length);
        self.insertSpaceFromUpperBound(cp, -length)
    }

    // Java: void insertSpaceBefore(PositionNode pNode, int nLength)
    pub fn insertSpaceBefore(&self, p_node: *mut PositionNode, n_length: i32) {
        // Java: this.positions.insertSpaceBefore(pNode, nLength);
        self.positions.insertSpaceBefore(p_node as *mut Node, n_length);
    }

    // Java: void eraseSpaceBefore(PositionNode pNode, int nLength)
    pub fn eraseSpaceBefore(&self, p_node: *mut PositionNode, n_length: i32) {
        // Java: this.insertSpaceBefore(pNode, -nLength);
        self.insertSpaceBefore(p_node, -n_length);
    }

    // Java: String toString()
    pub fn to_string(&self) -> String {
        // Java: StringWriter sb = new StringWriter();
        // Java: return sb.toString();
        String::new()
    }
}
