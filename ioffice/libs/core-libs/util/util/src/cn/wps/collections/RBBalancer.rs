#![allow(non_snake_case, dead_code, unused_variables, unused_mut, unused_unsafe)]

use super::Node::Node;
use super::Tree::Tree;
use super::ITreeBalancer::ITreeBalancer;
use super::INodeProvider::INodeProvider;
use super::VisitorContext::VisitorContext;

// Java: public class RBBalancer implements ITreeBalancer
pub struct RBBalancer;

// Java field: int COLOR_BLACK
const COLOR_BLACK: i32 = 1;
// Java field: int COLOR_RED
const COLOR_RED: i32 = 2;
// Java field: int RED_MASK
const RED_MASK: i32 = 1;

// Java: public class RBBalancer implements ITreeBalancer
impl RBBalancer {
    // Java: RBBalancer()
    pub fn new() -> Self {
        // Java: super();
        Self
    }

    // Java: static boolean isRed(Node n)
    pub fn isRed(n: *mut Node) -> bool {
        // Java: return (n.flags & 1) != 0;
        unsafe { ((*n).flags & 1) != 0 }
    }

    // Java: static boolean isBlack(Node n)
    pub fn isBlack(n: *mut Node) -> bool {
        // Java: return (n.flags & 1) == 0;
        unsafe { ((*n).flags & 1) == 0 }
    }

    // Java: static int getColor(Node n)
    fn getColor(n: *mut Node) -> i32 {
        // Java: return isRed(n) ? 2 : 1;
        if Self::isRed(n) { 2 } else { 1 }
    }

    // Java: static void setColor(Node n, int nColor)
    fn setColor(n: *mut Node, n_color: i32) {
        // Java: if (nColor == 2)
        if n_color == 2 {
            // Java: n.flags |= 1;
            unsafe { (*n).flags |= 1; }
        // Java: else
        } else {
            // Java: n.flags &= -2;
            unsafe { (*n).flags &= -2; }
        }
    }

    // Java: static void setRed(Node n)
    pub fn setRed(n: *mut Node) {
        // Java: n.flags |= 1;
        unsafe { (*n).flags |= 1; }
    }

    // Java: static void setBlack(Node n)
    pub fn setBlack(n: *mut Node) {
        // Java: n.flags &= -2;
        unsafe { (*n).flags &= -2; }
    }

    // Java: static void swapColor(Node a, Node b)
    fn swapColor(a: *mut Node, b: *mut Node) {
        // Java: int ac = getColor(a);
        let ac = Self::getColor(a);
        // Java: int bc = getColor(b);
        let bc = Self::getColor(b);
        // Java: setColor(a, bc);
        Self::setColor(a, bc);
        // Java: setColor(b, ac);
        Self::setColor(b, ac);
    }

    // Java: void onCreateHead(Tree tree, Node head)
    pub fn onCreateHead_impl(&mut self, _tree: &mut Tree, head: *mut Node) {
        // Java: setRed(head);
        Self::setRed(head);
    }

    // Java: void onBinaryTreeInserted(Tree tree, Node pNew)
    pub fn onBinaryTreeInserted_impl(&mut self, tree: &mut Tree, p_new: *mut Node) {
        // Java: this.insertFixup(tree, pNew);
        self.insertFixup(tree, p_new);
    }

    // Java: void beforeSwap(Tree tree, Node z, Node y)
    pub fn beforeSwap_impl(&mut self, _tree: &mut Tree, z: *mut Node, y: *mut Node) {
        // Java: swapColor(z, y);
        Self::swapColor(z, y);
    }

    // Java: void onBinaryTreeErased(Tree tree, Node z, Node x, Node x_parent)
    pub fn onBinaryTreeErased_impl(&mut self, tree: &mut Tree, z: *mut Node, x: *mut Node, x_parent: *mut Node) {
        // Java: this.eraseFixup(tree, z, x, x_parent);
        self.eraseFixup(tree, z, x, x_parent);
    }

    // Java: void insertFixup(Tree tree, Node x)
    fn insertFixup(&mut self, tree: &mut Tree, mut x: *mut Node) {
        // Java: Node head = tree.mHead;
        let head = tree.mHead;
        // Java: setRed(x);
        Self::setRed(x);
        unsafe {
            // Java: while(x != head.parent && isRed(x.parent))
            while x != (*head).parent && Self::isRed((*x).parent) {
                // Java: if (x.parent == x.parent.parent.left)
                if (*x).parent == (*(*(*x).parent).parent).left {
                    // Java: Node y = x.parent.parent.right;
                    let mut y = (*(*(*x).parent).parent).right;
                    // Java: if (y != null && isRed(y))
                    if !y.is_null() && Self::isRed(y) {
                        // Java: setBlack(x.parent);
                        Self::setBlack((*x).parent);
                        // Java: setBlack(y);
                        Self::setBlack(y);
                        // Java: setRed(x.parent.parent);
                        Self::setRed((*(*x).parent).parent);
                        // Java: x = x.parent.parent;
                        x = (*(*x).parent).parent;
                    // Java: else
                    } else {
                        // Java: if (x == x.parent.right)
                        if x == (*(*x).parent).right {
                            // Java: x = x.parent;
                            x = (*x).parent;
                            // Java: tree.rotateLeft(x, head);
                            tree.rotateLeft(x, head);
                        }
                        // Java: setBlack(x.parent);
                        Self::setBlack((*x).parent);
                        // Java: setRed(x.parent.parent);
                        Self::setRed((*(*x).parent).parent);
                        // Java: tree.rotateRight(x.parent.parent, head);
                        tree.rotateRight((*(*x).parent).parent, head);
                    }
                // Java: else
                } else {
                    // Java: Node y = x.parent.parent.left;
                    let mut y = (*(*(*x).parent).parent).left;
                    // Java: if (y != null && isRed(y))
                    if !y.is_null() && Self::isRed(y) {
                        // Java: setBlack(x.parent);
                        Self::setBlack((*x).parent);
                        // Java: setBlack(y);
                        Self::setBlack(y);
                        // Java: setRed(x.parent.parent);
                        Self::setRed((*(*x).parent).parent);
                        // Java: x = x.parent.parent;
                        x = (*(*x).parent).parent;
                    // Java: else
                    } else {
                        // Java: if (x == x.parent.left)
                        if x == (*(*x).parent).left {
                            // Java: x = x.parent;
                            x = (*x).parent;
                            // Java: tree.rotateRight(x, head);
                            tree.rotateRight(x, head);
                        }
                        // Java: setBlack(x.parent);
                        Self::setBlack((*x).parent);
                        // Java: setRed(x.parent.parent);
                        Self::setRed((*(*x).parent).parent);
                        // Java: tree.rotateLeft(x.parent.parent, head);
                        tree.rotateLeft((*(*x).parent).parent, head);
                    }
                }
            }
        }
        // Java: setBlack(head.parent);
        unsafe { Self::setBlack((*head).parent); }
    }

    // Java: void eraseFixup(Tree tree, Node z, Node x, Node x_parent)
    fn eraseFixup(&mut self, tree: &mut Tree, z: *mut Node, mut x: *mut Node, mut x_parent: *mut Node) {
        // Java: Node head = tree.mHead;
        let head = tree.mHead;
        // Java: if (!isRed(z))
        if !Self::isRed(z) {
            unsafe {
                // Java: while(x != head.parent && (x == null || isBlack(x)))
                while x != (*head).parent && (x.is_null() || Self::isBlack(x)) {
                    // Java: if (x == x_parent.left)
                    if x == (*x_parent).left {
                        // Java: Node w = x_parent.right;
                        let mut w = (*x_parent).right;
                        // Java: if (isRed(w))
                        if Self::isRed(w) {
                            // Java: setBlack(w);
                            Self::setBlack(w);
                            // Java: setRed(x_parent);
                            Self::setRed(x_parent);
                            // Java: tree.rotateLeft(x_parent, head);
                            tree.rotateLeft(x_parent, head);
                            // Java: w = x_parent.right;
                            w = (*x_parent).right;
                        }
                        // Java: if (w.left != null && !isBlack(w.left) || w.right != null && !isBlack(w.right))
                        if (!(*w).left.is_null() && !Self::isBlack((*w).left)) || (!(*w).right.is_null() && !Self::isBlack((*w).right)) {
                            // Java: if (w.right == null || isBlack(w.right))
                            if (*w).right.is_null() || Self::isBlack((*w).right) {
                                // Java: if (w.left != null)
                                if !(*w).left.is_null() {
                                    // Java: setBlack(w.left);
                                    Self::setBlack((*w).left);
                                }
                                // Java: setRed(w);
                                Self::setRed(w);
                                // Java: tree.rotateRight(w, head);
                                tree.rotateRight(w, head);
                                // Java: w = x_parent.right;
                                w = (*x_parent).right;
                            }
                            // Java: setColor(w, getColor(x_parent));
                            Self::setColor(w, Self::getColor(x_parent));
                            // Java: setBlack(x_parent);
                            Self::setBlack(x_parent);
                            // Java: if (w.right != null)
                            if !(*w).right.is_null() {
                                // Java: setBlack(w.right);
                                Self::setBlack((*w).right);
                            }
                            // Java: tree.rotateLeft(x_parent, head);
                            tree.rotateLeft(x_parent, head);
                            // Java: break;
                            break;
                        }
                        // Java: setRed(w);
                        Self::setRed(w);
                        // Java: x = x_parent;
                        x = x_parent;
                        // Java: x_parent = x_parent.parent;
                        x_parent = (*x_parent).parent;
                    // Java: else
                    } else {
                        // Java: Node w = x_parent.left;
                        let mut w = (*x_parent).left;
                        // Java: if (isRed(w))
                        if Self::isRed(w) {
                            // Java: setBlack(w);
                            Self::setBlack(w);
                            // Java: setRed(x_parent);
                            Self::setRed(x_parent);
                            // Java: tree.rotateRight(x_parent, head);
                            tree.rotateRight(x_parent, head);
                            // Java: w = x_parent.left;
                            w = (*x_parent).left;
                        }
                        // Java: if (w.right != null && !isBlack(w.right) || w.left != null && !isBlack(w.left))
                        if (!(*w).right.is_null() && !Self::isBlack((*w).right)) || (!(*w).left.is_null() && !Self::isBlack((*w).left)) {
                            // Java: if (w.left == null || isBlack(w.left))
                            if (*w).left.is_null() || Self::isBlack((*w).left) {
                                // Java: if (w.right != null)
                                if !(*w).right.is_null() {
                                    // Java: setBlack(w.right);
                                    Self::setBlack((*w).right);
                                }
                                // Java: setRed(w);
                                Self::setRed(w);
                                // Java: tree.rotateLeft(w, head);
                                tree.rotateLeft(w, head);
                                // Java: w = x_parent.left;
                                w = (*x_parent).left;
                            }
                            // Java: setColor(w, getColor(x_parent));
                            Self::setColor(w, Self::getColor(x_parent));
                            // Java: setBlack(x_parent);
                            Self::setBlack(x_parent);
                            // Java: if (w.left != null)
                            if !(*w).left.is_null() {
                                // Java: setBlack(w.left);
                                Self::setBlack((*w).left);
                            }
                            // Java: tree.rotateRight(x_parent, head);
                            tree.rotateRight(x_parent, head);
                            // Java: break;
                            break;
                        }
                        // Java: setRed(w);
                        Self::setRed(w);
                        // Java: x = x_parent;
                        x = x_parent;
                        // Java: x_parent = x_parent.parent;
                        x_parent = (*x_parent).parent;
                    }
                }
                // Java: if (x != null)
                if !x.is_null() {
                    // Java: setBlack(x);
                    Self::setBlack(x);
                }
            }
        }
    }

    // Java: ITreeCreator getTreeCreator()
    pub fn getTreeCreator_impl(&self) -> RBTreeCreator {
        // Java: return new RBTreeCreator();
        RBTreeCreator::new()
    }
}

// Java: public class RBTreeCreator
pub struct RBTreeCreator {
    // Java field: int mMaxLevel
    pub mMaxLevel: i32,
}

// Java: public class RBTreeCreator
impl RBTreeCreator {
    // Java: RBTreeCreator()
    pub fn new() -> Self {
        // Java: super();
        Self { mMaxLevel: 0 }
    }

    // Java: int getMaxLevel()
    pub fn getMaxLevel(&self) -> i32 {
        // Java: return this.mMaxLevel;
        self.mMaxLevel
    }

    // Java: void init(int nTreeSize)
    pub fn init(&mut self, n_tree_size: i32) {
        // Java: this.mMaxLevel = TreeCreator.maxLevel(nTreeSize);
        self.mMaxLevel = max_level(n_tree_size);
    }

    // Java: void visit(Node node, VisitorContext ctx)
    pub fn visit(&self, node: *mut Node, ctx: &VisitorContext) {
        // Java: if (ctx.getLevel() == this.mMaxLevel)
        if ctx.getLevel() == self.mMaxLevel {
            // Java: RBBalancer.setRed(node);
            RBBalancer::setRed(node);
        // Java: else
        } else {
            // Java: RBBalancer.setBlack(node);
            RBBalancer::setBlack(node);
        }
    }

    // Java: void finish(Node root)
    pub fn finish(&self, root: *mut Node) {
        // Java: if (root != null)
        if !root.is_null() {
            // Java: RBBalancer.setBlack(root);
            RBBalancer::setBlack(root);
        }
    }

    // Java: void createTree(Tree tree, Node root, INodeProvider list)
    pub fn createTree(&self, tree: &mut Tree, _root: *mut Node, _list: &dyn INodeProvider) {
        // Java: Node head = tree.getHead();
        let head = tree.mHead;
        // Java: RBBalancer.setRed(head);
        RBBalancer::setRed(head);
    }
}

fn max_level(n: i32) -> i32 {
    if n <= 0 { return 0; }
    let mut level = 0;
    let mut count = 1;
    while count <= n {
        level += 1;
        count = count * 2 + 1;
    }
    level
}

impl ITreeBalancer for RBBalancer {
    fn onCreateHead(&mut self, tree: *mut u8, node: *mut u8) {
        unsafe { self.onCreateHead_impl(&mut *(tree as *mut Tree), node as *mut Node); }
    }
    fn onBinaryTreeInserted(&mut self, tree: *mut u8, node: *mut u8) {
        unsafe { self.onBinaryTreeInserted_impl(&mut *(tree as *mut Tree), node as *mut Node); }
    }
    fn beforeSwap(&mut self, tree: *mut u8, var2: *mut u8, var3: *mut u8) {
        unsafe { self.beforeSwap_impl(&mut *(tree as *mut Tree), var2 as *mut Node, var3 as *mut Node); }
    }
    fn onBinaryTreeErased(&mut self, tree: *mut u8, var2: *mut u8, var3: *mut u8, var4: *mut u8) {
        unsafe { self.onBinaryTreeErased_impl(&mut *(tree as *mut Tree), var2 as *mut Node, var3 as *mut Node, var4 as *mut Node); }
    }
    fn getTreeCreator(&self) -> Option<Box<dyn std::any::Any>> {
        Some(Box::new(self.getTreeCreator_impl()))
    }
}
