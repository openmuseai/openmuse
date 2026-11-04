#![allow(non_snake_case, dead_code, unused_variables)]

use std::ptr;
use super::Node::Node;
use super::PositionTree::PositionTree;
use super::PositionNode::PositionNode;

// Java: public class PositionTreeCommands
pub struct PositionTreeCommands;

// Java: public class CmdInsertLeft
pub struct CmdInsertLeft {
    // Java field: PositionTree mTree
    pub(crate) mTree: *mut PositionTree,
    // Java field: PositionNode mNew
    pub(crate) mNew: *mut PositionNode,
    // Java field: PositionNode mNext
    pub(crate) mNext: *mut PositionNode,
    // Java field: int mOffset
    pub(crate) mOffset: i32,
}

// Java: public class CmdInsertRight
pub struct CmdInsertRight {
    // Java field: PositionTree mTree
    pub(crate) mTree: *mut PositionTree,
    // Java field: PositionNode mNew
    pub(crate) mNew: *mut PositionNode,
    // Java field: PositionNode mPrev
    pub(crate) mPrev: *mut PositionNode,
    // Java field: int mOffset
    pub(crate) mOffset: i32,
}

// Java: public class CmdRemove
pub struct CmdRemove {
    // Java field: PositionTree mTree
    pub(crate) mTree: *mut PositionTree,
    // Java field: PositionNode mPos
    pub(crate) mPos: *mut PositionNode,
    // Java field: int mCp
    pub(crate) mCp: i32,
    // Java field: PositionNode mPositionNode
    pub(crate) mPositionNode: *mut PositionNode,
}

// Java: public class CmdInsertSpace
pub struct CmdInsertSpace {
    // Java field: PositionTree tree
    pub(crate) tree: *mut PositionTree,
    // Java field: PositionNode leaf
    pub(crate) leaf: *mut PositionNode,
    // Java field: int length
    pub(crate) length: i32,
}

// Java: public class CmdInsertSpaceLower
pub struct CmdInsertSpaceLower {
    // Java field: PositionTree tree
    pub(crate) tree: *mut PositionTree,
    // Java field: int cp
    pub(crate) cp: i32,
    // Java field: int length
    pub(crate) length: i32,
}

// Java: public class CmdMoveRight
pub struct CmdMoveRight {
    // Java field: PositionNode node
    pub(crate) node: *mut PositionNode,
    // Java field: int steps
    pub(crate) steps: i32,
}

// Java: public class PositionTreeCommands
impl PositionTreeCommands {
    // Java: PositionTreeCommands()
    pub fn new() -> Self {
        // Java: super();
        Self
    }
}

// Java: public class CmdInsertLeft
impl CmdInsertLeft {
    // Java: CmdInsertLeft(PositionTree tree, PositionNode pNew, PositionNode next, int offset)
    pub fn new(tree: *mut PositionTree, p_new: *mut PositionNode, next: *mut PositionNode, offset: i32) -> Self {
        // Java: super();
        // Java: this.mTree = tree;
        // Java: this.mNext = next;
        // Java: this.mOffset = offset;
        // Java: this.mNew = pNew;
        Self {
            mTree: tree,
            mNew: p_new,
            mNext: next,
            mOffset: offset,
        }
    }

    // Java: void execute()
    pub fn execute(&mut self) {
        // Java: this.mNew.offset = this.mOffset;
        unsafe { (*self.mNew).offset = self.mOffset; }
        // Java: this.mNew.left = null;
        unsafe { (*self.mNew).base.left = ptr::null_mut(); }
        // Java: this.mNew.right = null;
        unsafe { (*self.mNew).base.right = ptr::null_mut(); }
        // Java: this.mTree.insertLeft(this.mNext, this.mNew);
        unsafe { (*self.mTree).base.insertLeft_simple(self.mNext as *mut Node, self.mNew as *mut Node); }
    }

    // Java: void unexecute()
    pub fn unexecute(&mut self) {
        // Java: this.mTree.erase(this.mNew);
        unsafe { (*self.mTree).base.erase(self.mNew as *mut Node); }
    }
}

// Java: public class CmdInsertRight
impl CmdInsertRight {
    // Java: CmdInsertRight(PositionTree tree, PositionNode prev, PositionNode pNew, int offset)
    pub fn new(tree: *mut PositionTree, prev: *mut PositionNode, p_new: *mut PositionNode, offset: i32) -> Self {
        // Java: super();
        // Java: this.mTree = tree;
        // Java: this.mPrev = prev;
        // Java: this.mOffset = offset;
        // Java: this.mNew = pNew;
        Self {
            mTree: tree,
            mNew: p_new,
            mPrev: prev,
            mOffset: offset,
        }
    }

    // Java: void execute()
    pub fn execute(&mut self) {
        // Java: this.mNew.offset = this.mOffset;
        unsafe { (*self.mNew).offset = self.mOffset; }
        // Java: this.mNew.left = null;
        unsafe { (*self.mNew).base.left = ptr::null_mut(); }
        // Java: this.mNew.right = null;
        unsafe { (*self.mNew).base.right = ptr::null_mut(); }
        // Java: this.mTree.insertRight(this.mPrev, this.mNew);
        unsafe { (*self.mTree).base.insertRight_simple(self.mPrev as *mut Node, self.mNew as *mut Node); }
    }

    // Java: void unexecute()
    pub fn unexecute(&mut self) {
        // Java: this.mTree.erase(this.mNew);
        unsafe { (*self.mTree).base.erase(self.mNew as *mut Node); }
    }
}

// Java: public class CmdRemove
impl CmdRemove {
    // Java: CmdRemove(PositionTree tree, PositionNode node)
    pub fn new(tree: *mut PositionTree, node: *mut PositionNode) -> Self {
        // Java: super();
        // Java: this.mTree = tree;
        // Java: this.mPositionNode = node;
        // Java: this.mPos = (PositionNode)node.getNext();
        let pos = unsafe { (*node).base.getNext() } as *mut PositionNode;
        // Java: this.mCp = node.position();
        let cp = unsafe { (*node).position() };
        Self {
            mTree: tree,
            mPositionNode: node,
            mPos: pos,
            mCp: cp,
        }
    }

    // Java: void execute()
    pub fn execute(&mut self) {
        // Java: this.mTree.erase(this.mPositionNode);
        unsafe { (*self.mTree).base.erase(self.mPositionNode as *mut Node); }
    }

    // Java: void unexecute()
    pub fn unexecute(&mut self) {
        // Java: this.mTree.insertLeft(this.mPos, this.mPositionNode, this.mCp);
        unsafe { (*self.mTree).insertLeft_pos(self.mPos, self.mPositionNode, self.mCp); }
    }
}

// Java: public class CmdInsertSpace
impl CmdInsertSpace {
    // Java: CmdInsertSpace(PositionTree tree, PositionNode leaf, int length)
    pub fn new(tree: *mut PositionTree, leaf: *mut PositionNode, length: i32) -> Self {
        // Java: super();
        // Java: this.tree = tree;
        // Java: this.leaf = leaf;
        // Java: this.length = length;
        Self { tree, leaf, length }
    }

    // Java: void execute()
    pub fn execute(&self) {
        // Java: this.tree.insertSpaceBefore(this.leaf, this.length);
        unsafe { (*self.tree).insertSpaceBefore(self.leaf, self.length); }
    }

    // Java: void unexecute()
    pub fn unexecute(&self) {
        // Java: this.tree.eraseSpaceBefore(this.leaf, this.length);
        unsafe { (*self.tree).eraseSpaceBefore(self.leaf, self.length); }
    }
}

// Java: public class CmdInsertSpaceLower
impl CmdInsertSpaceLower {
    // Java: CmdInsertSpaceLower(PositionTree tree, int cp, int length)
    pub fn new(tree: *mut PositionTree, cp: i32, length: i32) -> Self {
        // Java: super();
        // Java: this.tree = tree;
        // Java: this.cp = cp;
        // Java: this.length = length;
        Self { tree, cp, length }
    }

    // Java: void execute()
    pub fn execute(&self) {
        // Java: this.tree.insertSpaceFromLowerBound(this.cp, this.length);
        unsafe { (*self.tree).insertSpaceFromLowerBound(self.cp, self.length); }
    }

    // Java: void unexecute()
    pub fn unexecute(&self) {
        // Java: this.tree.insertSpaceFromLowerBound(this.cp, -this.length);
        unsafe { (*self.tree).insertSpaceFromLowerBound(self.cp, -self.length); }
    }
}

// Java: public class CmdMoveRight
impl CmdMoveRight {
    // Java: CmdMoveRight(PositionNode node, int steps)
    pub fn new(node: *mut PositionNode, steps: i32) -> Self {
        // Java: super();
        // Java: this.node = node;
        // Java: this.steps = steps;
        Self { node, steps }
    }

    // Java: void execute()
    pub fn execute(&mut self) {
        // Java: this.node.moveRight(this.steps);
        unsafe { (*self.node).moveRight(self.steps); }
    }

    // Java: void unexecute()
    pub fn unexecute(&mut self) {
        // Java: this.node.moveLeft(this.steps);
        unsafe { (*self.node).moveLeft(self.steps); }
    }
}
