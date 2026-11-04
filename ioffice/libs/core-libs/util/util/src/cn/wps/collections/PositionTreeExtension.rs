#![allow(non_snake_case, dead_code, unused_variables)]

use std::ptr;
use super::Node::Node;
use super::Tree::Tree;
use super::ITreeExtension::ITreeExtension;
use super::PositionNode::PositionNode;
use super::VisitorContext::VisitorContext;
use super::INodeProvider::INodeProvider;

// Java field: int mask
pub const MASK: i32 = 63;

// Java: public class PositionTreeExtension implements ITreeExtension
pub struct PositionTreeExtension {
    // Java field: Tree mTree
    pub(crate) mTree: *mut Tree,
    // Java field: Node mHead
    pub(crate) mHead: *mut Node,
}

// Java: public class PositionTreeCreator
pub struct PositionTreeCreator {
}

// Java: public interface IPositionNode
pub trait IPositionNode {
    // Java: abstract int getOffset()
    fn getOffset(&self) -> i32;
    // Java: abstract void setOffset(int var1)
    fn setOffset(&mut self, var1: i32);
}

// Java: public class PositionTreeExtension implements ITreeExtension
impl PositionTreeExtension {
    // Java: PositionTreeExtension()
    pub fn new() -> Self {
        // Java: super();
        Self {
            mTree: ptr::null_mut(),
            mHead: ptr::null_mut(),
        }
    }

    // Java: static IPositionNode cast(Node node)
    fn cast(node: *mut Node) -> *mut PositionNode {
        // Java: return (IPositionNode)node;
        node as *mut PositionNode
    }

    // Java: static int getOffset(Node n)
    pub fn getOffset(n: *mut Node) -> i32 {
        // Java: return cast(n).getOffset();
        unsafe { (*Self::cast(n)).offset }
    }

    // Java: static void setOffset(Node n, int offset)
    pub fn setOffset(n: *mut Node, offset: i32) {
        // Java: cast(n).setOffset(offset);
        unsafe { (*Self::cast(n)).offset = offset; }
    }

    // Java: static void offsetToCP(Node root, int leftRootCP)
    fn offsetToCP(root: *mut Node, left_root_cp: i32) {
        unsafe {
            // Java: int rootCP = leftRootCP + getOffset(root);
            let root_cp = left_root_cp + Self::getOffset(root);
            // Java: setOffset(root, rootCP);
            Self::setOffset(root, root_cp);
            // Java: if (root.left != null)
            if !(*root).left.is_null() {
                // Java: offsetToCP(root.left, leftRootCP);
                Self::offsetToCP((*root).left, left_root_cp);
            }
            // Java: if (root.right != null)
            if !(*root).right.is_null() {
                // Java: offsetToCP(root.right, rootCP);
                Self::offsetToCP((*root).right, root_cp);
            }
        }
    }

    // Java: static void toList(Tree tree, int startCP)
    pub fn toList_with_cp(tree: &mut Tree, startcp: i32) {
        // Java: Node root = tree.getRoot();
        let root = tree.getRoot();
        // Java: if (root != null)
        if !root.is_null() {
            // Java: offsetToCP(root, startCP);
            Self::offsetToCP(root, startcp);
        }
        // Java: Tree.toList(tree);
        Tree::toList(tree);
    }

    // Java: int position(Node node)
    pub fn position(&self, mut node: *mut Node) -> i32 {
        unsafe {
            // Java: Node head = this.mHead;
            let head = self.mHead;
            // Java: int cp = getOffset(node);
            let mut cp = Self::getOffset(node);
            // Java: Node p = node.parent;
            let mut p = (*node).parent;
            // Java: if (p == null)
            if p.is_null() {
                // Java: return cp;
                return cp;
            }
            // Java: else
            else {
                // Java: while(p != head)
                while p != head {
                    // Java: if (node == p.right)
                    if node == (*p).right {
                        // Java: cp += getOffset(p);
                        cp += Self::getOffset(p);
                    }
                    // Java: node = p;
                    node = p;
                    // Java: p = p.parent;
                    p = (*p).parent;
                }
                // Java: return cp;
                return cp;
            }
        }
    }

    // Java: static int getNodePosition(Node node)
    pub fn getNodePosition(mut node: *mut Node) -> i32 {
        unsafe {
            // Java: int cp = getOffset(node);
            let mut cp = Self::getOffset(node);
            // Java: Node p = node.parent;
            let mut p = (*node).parent;
            // Java: if (p == null)
            if p.is_null() {
                // Java: return cp;
                return cp;
            }
            // Java: else
            else {
                // Java: while(!p.isHead())
                while !(*p).isHead() {
                    // Java: if (node == p.right)
                    if node == (*p).right {
                        // Java: cp += getOffset(p);
                        cp += Self::getOffset(p);
                    }
                    // Java: node = p;
                    node = p;
                    // Java: p = p.parent;
                    p = (*p).parent;
                }
                // Java: return cp;
                return cp;
            }
        }
    }

    // Java: void moveRight(Node node, int steps)
    pub fn moveRight(&self, node: *mut Node, steps: i32) {
        unsafe {
            // Java: setOffset(node, getOffset(node) + steps);
            Self::setOffset(node, Self::getOffset(node) + steps);
            // Java: for(Node p = node.right; p != null; p = p.left)
            let mut p = (*node).right;
            while !p.is_null() {
                // Java: setOffset(p, getOffset(p) - steps);
                Self::setOffset(p, Self::getOffset(p) - steps);
                p = (*p).left;
            }
        }
    }

    // Java: void moveLeft(Node node, int steps)
    pub fn moveLeft(&self, node: *mut Node, steps: i32) {
        // Java: this.moveRight(node, -steps);
        self.moveRight(node, -steps);
    }

    // Java: Node lowerBound(int cp)
    pub fn lowerBound(&self, cp: i32) -> *mut Node {
        // Java: return this.lowerBound(cp, (int[])null);
        self.lowerBound_with_offset(cp, None)
    }

    // Java: Node upperBound(int cp)
    pub fn upperBound(&self, cp: i32) -> *mut Node {
        // Java: return this.upperBound(cp, (int[])null);
        self.upperBound_with_offset(cp, None)
    }

    // Java: Node lowerBound(int cp, _offset: &[i32])
    pub fn lowerBound_with_offset(&self, mut cp: i32, offset: Option<&mut [i32]>) -> *mut Node {
        unsafe {
            // Java: Node y = this.mHead;
            let mut y = self.mHead;
            // Java: Node x = this.mHead.parent;
            let mut x = (*self.mHead).parent;
            // Java: while(x != null)
            while !x.is_null() {
                // Java: if (cp <= getOffset(x))
                if cp <= Self::getOffset(x) {
                    // Java: y = x;
                    y = x;
                    // Java: x = x.left;
                    x = (*x).left;
                }
                // Java: else
                else {
                    // Java: cp -= getOffset(x);
                    cp -= Self::getOffset(x);
                    // Java: x = x.right;
                    x = (*x).right;
                }
            }
            // Java: if (offset != null)
            if let Some(off) = offset {
                // Java: offset[0] = cp;
                off[0] = cp;
            }
            // Java: return y;
            y
        }
    }

    // Java: Node upperBound(int cp, _offset: &[i32])
    pub fn upperBound_with_offset(&self, mut cp: i32, offset: Option<&mut [i32]>) -> *mut Node {
        unsafe {
            // Java: Node y = this.mHead;
            let mut y = self.mHead;
            // Java: Node x = this.mHead.parent;
            let mut x = (*self.mHead).parent;
            // Java: while(x != null)
            while !x.is_null() {
                // Java: if (cp >= getOffset(x))
                if cp >= Self::getOffset(x) {
                    // Java: cp -= getOffset(x);
                    cp -= Self::getOffset(x);
                    // Java: x = x.right;
                    x = (*x).right;
                }
                // Java: else
                else {
                    // Java: y = x;
                    y = x;
                    // Java: x = x.left;
                    x = (*x).left;
                }
            }
            // Java: if (offset != null)
            if let Some(off) = offset {
                // Java: offset[0] = cp;
                off[0] = cp;
            }
            // Java: return y;
            y
        }
    }

    // Java: Node insertSpaceFromLowerBound(int cp, int length)
    pub fn insertSpaceFromLowerBound(&self, mut cp: i32, length: i32) -> *mut Node {
        unsafe {
            // Java: Node y = this.mHead;
            let mut y = self.mHead;
            // Java: Node x = this.mHead.parent;
            let mut x = (*self.mHead).parent;
            // Java: while(x != null)
            while !x.is_null() {
                // Java: if (cp <= getOffset(x))
                if cp <= Self::getOffset(x) {
                    // Java: setOffset(x, getOffset(x) + length);
                    Self::setOffset(x, Self::getOffset(x) + length);
                    // Java: y = x;
                    y = x;
                    // Java: x = x.left;
                    x = (*x).left;
                }
                // Java: else
                else {
                    // Java: cp -= getOffset(x);
                    cp -= Self::getOffset(x);
                    // Java: x = x.right;
                    x = (*x).right;
                }
            }
            // Java: return y;
            y
        }
    }

    // Java: Node insertSpaceFromUpperBound(int cp, int length)
    pub fn insertSpaceFromUpperBound(&self, mut cp: i32, length: i32) -> *mut Node {
        unsafe {
            // Java: Node y = this.mHead;
            let mut y = self.mHead;
            // Java: Node x = this.mHead.parent;
            let mut x = (*self.mHead).parent;
            // Java: while(x != null)
            while !x.is_null() {
                // Java: if (cp >= getOffset(x))
                if cp >= Self::getOffset(x) {
                    // Java: cp -= getOffset(x);
                    cp -= Self::getOffset(x);
                    // Java: x = x.right;
                    x = (*x).right;
                }
                // Java: else
                else {
                    // Java: setOffset(x, getOffset(x) + length);
                    Self::setOffset(x, Self::getOffset(x) + length);
                    // Java: y = x;
                    y = x;
                    // Java: x = x.left;
                    x = (*x).left;
                }
            }
            // Java: return y;
            y
        }
    }

    // Java: Node removeSpaceFromLowerBound(int cp, int length)
    pub fn removeSpaceFromLowerBound(&self, cp: i32, length: i32) -> *mut Node {
        // Java: return this.insertSpaceFromLowerBound(cp, -length);
        self.insertSpaceFromLowerBound(cp, -length)
    }

    // Java: Node removeSpaceFromUpperBound(int cp, int length)
    pub fn removeSpaceFromUpperBound(&self, cp: i32, length: i32) -> *mut Node {
        // Java: return this.insertSpaceFromUpperBound(cp, -length);
        self.insertSpaceFromUpperBound(cp, -length)
    }

    // Java: void insertSpaceBefore(Node pNode, int nLength)
    pub fn insertSpaceBefore(&self, pnode: *mut Node, nlength: i32) {
        unsafe {
            // Java: Node head = this.mHead;
            let head = self.mHead;
            // Java: if (pNode != this.mHead)
            if pnode != self.mHead {
                // Java: setOffset(pNode, getOffset(pNode) + nLength);
                Self::setOffset(pnode, Self::getOffset(pnode) + nlength);
                // Java: Node x = pNode;
                let mut x = pnode;
                // Java: for(Node y = pNode.parent; y != head; y = y.parent)
                let mut y = (*pnode).parent;
                while y != head {
                    // Java: if (y.left == x)
                    if (*y).left == x {
                        // Java: setOffset(y, getOffset(y) + nLength);
                        Self::setOffset(y, Self::getOffset(y) + nlength);
                    }
                    // Java: x = y;
                    x = y;
                    y = (*y).parent;
                }
            }
        }
    }

    // Java: void eraseSpaceBefore(Node pNode, int nLength)
    pub fn eraseSpaceBefore(&self, pnode: *mut Node, nlength: i32) {
        // Java: this.insertSpaceBefore(pNode, -nLength);
        self.insertSpaceBefore(pnode, -nlength);
    }
}

impl PositionTreeExtension {
    /// Tree is passed into every callback; refresh after Rust moves the outer struct.
    fn rebind(&mut self, tree: *mut u8) {
        if tree.is_null() {
            return;
        }
        self.mTree = tree as *mut Tree;
        unsafe {
            if !self.mTree.is_null() {
                self.mHead = (*self.mTree).mHead;
            }
        }
    }
}

impl ITreeExtension for PositionTreeExtension {
    // Java: void onCreateHead(Tree tree, Node head)
    fn onCreateHead(&mut self, tree: *mut u8, head: *mut u8) {
        // Java: this.mTree = tree;
        self.mTree = tree as *mut Tree;
        // Java: this.mHead = head;
        self.mHead = head as *mut Node;
    }

    // Java: void onBinaryTreeInsertedLeft(Tree tree, Node pNode, Node pNew, int arg1, Object arg2)
    fn onBinaryTreeInsertedLeft(&mut self, tree: *mut u8, pnode: *mut u8, p_new: *mut u8, arg1: i32, arg2: Option<&dyn std::any::Any>) {
        self.rebind(tree);
        unsafe {
            let p_new = p_new as *mut Node;
            // Java: if (null != arg2 && arg2 instanceof int[])
            if let Some(any_ref) = arg2 {
                if let Some(offset_ref) = any_ref.downcast_ref::<std::cell::RefCell<[i32; 1]>>() {
                    // Java: _offset: &[i32] = (int[])arg2;
                    let mut offset = offset_ref.borrow_mut();
                    // Java: Node pLeftRoot = this.mTree.getLeftRoot(pNew);
                    let p_left_root = (*self.mTree).getLeftRoot(p_new);
                    // Java: if (pLeftRoot.isHead())
                    if (*p_left_root).isHead() {
                        // Java: offset[0] = arg1;
                        offset[0] = arg1;
                    }
                    // Java: else
                    else {
                        // Java: offset[0] = arg1 - this.position(pLeftRoot);
                        offset[0] = arg1 - self.position(p_left_root);
                    }
                    // Java: setOffset(pNew, offset[0]);
                    Self::setOffset(p_new, offset[0]);
                }
            }
        }
    }

    // Java: void onBinaryTreeInsertedRight(Tree tree, Node pNode, Node pNew, int arg1, Object arg2)
    fn onBinaryTreeInsertedRight(&mut self, tree: *mut u8, pnode: *mut u8, p_new: *mut u8, arg1: i32, arg2: Option<&dyn std::any::Any>) {
        self.rebind(tree);
        unsafe {
            let p_new = p_new as *mut Node;
            // Java: if (null != arg2 && arg2 instanceof int[])
            if let Some(any_ref) = arg2 {
                if let Some(offset_ref) = any_ref.downcast_ref::<std::cell::RefCell<[i32; 1]>>() {
                    // Java: _offset: &[i32] = (int[])arg2;
                    let mut offset = offset_ref.borrow_mut();
                    // Java: Node pLeftRoot = this.mTree.getLeftRoot(pNew);
                    let p_left_root = (*self.mTree).getLeftRoot(p_new);
                    // Java: if (pLeftRoot.isHead())
                    if (*p_left_root).isHead() {
                        // Java: offset[0] = arg1;
                        offset[0] = arg1;
                    }
                    // Java: else
                    else {
                        // Java: offset[0] = arg1 - this.position(pLeftRoot);
                        offset[0] = arg1 - self.position(p_left_root);
                    }
                    // Java: setOffset(pNew, offset[0]);
                    Self::setOffset(p_new, offset[0]);
                }
            }
        }
    }

    // Java: void beforeSwap(Tree tree, Node z, Node y)
    fn beforeSwap(&mut self, tree: *mut u8, z: *mut u8, y: *mut u8) {
        unsafe {
            let z = z as *mut Node;
            let y = y as *mut Node;
            // Java: int off = getOffset(y);
            let off = Self::getOffset(y);
            // Java: setOffset(y, getOffset(y) + getOffset(z));
            Self::setOffset(y, Self::getOffset(y) + Self::getOffset(z));
            // Java: for(Node p = y.parent; p != z; p = p.parent)
            let mut p = (*y).parent;
            while p != z {
                // Java: setOffset(p, getOffset(p) - off);
                Self::setOffset(p, Self::getOffset(p) - off);
                p = (*p).parent;
            }
            // Java: setOffset(z, getOffset(z) - getOffset(y));
            Self::setOffset(z, Self::getOffset(z) - Self::getOffset(y));
        }
    }

    // Java: void beforeErase(Tree tree, Node z, Node swp)
    fn beforeErase(&mut self, tree: *mut u8, z: *mut u8, swp: *mut u8) {
        unsafe {
            let z = z as *mut Node;
            let swp = swp as *mut Node;
            // Java: if (z == swp && z.right != null)
            if z == swp && !(*z).right.is_null() {
                // Java: int off = getOffset(z);
                let off = Self::getOffset(z);
                // Java: for(Node n = z.right; n != null; n = n.left)
                let mut n = (*z).right;
                while !n.is_null() {
                    // Java: setOffset(n, getOffset(n) + off);
                    Self::setOffset(n, Self::getOffset(n) + off);
                    n = (*n).left;
                }
            }
            // Java: setOffset(z, this.position(z));
            Self::setOffset(z, self.position(z));
        }
    }

    // Java: void onBinaryTreeErased(Tree tree, Node z, Node x, Node x_parent)
    fn onBinaryTreeErased(&mut self, tree: *mut u8, z: *mut u8, x: *mut u8, x_parent: *mut u8) {
        unsafe {
            let z = z as *mut Node;
            // Java: z.parent = null;
            (*z).parent = ptr::null_mut();
        }
    }

    // Java: void beforeRotateLeft(Tree tree, Node x, Node y)
    fn beforeRotateLeft(&mut self, tree: *mut u8, x: *mut u8, y: *mut u8) {
        unsafe {
            let x = x as *mut Node;
            let y = y as *mut Node;
            // Java: setOffset(y, getOffset(y) + getOffset(x));
            Self::setOffset(y, Self::getOffset(y) + Self::getOffset(x));
        }
    }

    // Java: void beforeRotateRight(Tree tree, Node y, Node x)
    fn beforeRotateRight(&mut self, tree: *mut u8, y: *mut u8, x: *mut u8) {
        unsafe {
            let y = y as *mut Node;
            let x = x as *mut Node;
            // Java: setOffset(y, getOffset(y) - getOffset(x));
            Self::setOffset(y, Self::getOffset(y) - Self::getOffset(x));
        }
    }

    // Java: ITreeCreator getTreeCreator()
    fn getTreeCreator(&self) -> Option<Box<dyn std::any::Any>> {
        // Java: return new PositionTreeCreator();
        None
    }
}

// Java: public class PositionTreeCreator
impl PositionTreeCreator {
    // Java: PositionTreeCreator()
    pub fn new() -> Self {
        // Java: super();
        Self {}
    }

    // Java: void init(int nTreeSize)
    pub fn init(&self, ntreesize: i32) {
    }

    // Java: void visit(Node node, VisitorContext ctx)
    pub fn visit(&self, node: *mut Node, ctx: &VisitorContext) {
        // Java: if (ctx.getNodeType() == 2)
        {
            unsafe {
                // Java: Node pLeftRoot = node.parent;
                let p_left_root = (*node).parent;
                // Java: int nLeftRootOffset = PositionTreeExtension.getOffset(pLeftRoot);
                let n_left_root_offset = PositionTreeExtension::getOffset(p_left_root);
                // Java: for(Node x = node; x != null; x = x.left)
                let mut x = node;
                while !x.is_null() {
                    // Java: int offset = PositionTreeExtension.getOffset(x);
                    let offset = PositionTreeExtension::getOffset(x);
                    // Java: PositionTreeExtension.setOffset(x, offset - nLeftRootOffset);
                    PositionTreeExtension::setOffset(x, offset - n_left_root_offset);
                    x = (*x).left;
                }
            }
        }
    }

    // Java: void finish(Node root)
    pub fn finish(&self, root: *mut Node) {
    }

    // Java: void createTree(Tree tree, Node root, INodeProvider list)
    pub fn createTree(&self, tree: *mut Tree, root: *mut Node, list: *mut u8) {
    }
}
