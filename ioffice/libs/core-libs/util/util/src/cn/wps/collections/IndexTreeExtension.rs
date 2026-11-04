#![allow(non_snake_case, dead_code, unused_variables, unused_mut, unused_unsafe)]

use super::Node::Node;
use super::ITreeExtension::ITreeExtension;
use super::INodeProvider::INodeProvider;
use super::VisitorContext::VisitorContext;

// Java: public class IndexTreeExtension implements ITreeExtension
pub struct IndexTreeExtension {
    // Java field: int mask
    // (mask is a class-level static const, value = 59)
    // Java field: Node mHead
    pub mHead: *mut Node,
}

pub const MASK_INDEX: i32 = 59;

// Java: public class IndexTreeExtension implements ITreeExtension
impl IndexTreeExtension {
    // Java: IndexTreeExtension()
    pub fn new() -> Self {
        // Java: super();
        Self {
            mHead: std::ptr::null_mut(),
        }
    }

    // Java: Node getRoot()
    pub fn getRoot(&self) -> *mut Node {
        // Java: return this.mHead.parent;
        unsafe { (*self.mHead).parent }
    }

    // Java: static IIndexTreeNode cast(Node node)
    fn cast(node: *mut Node) -> &'static mut dyn IIndexTreeNode {
        // Java: return (IIndexTreeNode)node;
        unsafe { &mut *(node as *mut IndexTreeNodeImpl) }
    }

    // Java: static int getLeftCount(Node node)
    fn getLeftCount_static(node: *mut Node) -> i32 {
        // Java: return cast(node).getLeftCount();
        unsafe { (*(node as *mut IndexTreeNodeImpl)).getLeftCount() }
    }

    // Java: static void setLeftCount(Node node, int nLeftCount)
    pub fn setLeftCount_static(node: *mut Node, nleftcount: i32) {
        // Java: cast(node).setLeftCount(nLeftCount);
        unsafe { (*(node as *mut IndexTreeNodeImpl)).setLeftCount(nleftcount); }
    }

    // Java: int size()
    pub fn size(&self) -> i32 {
        // Java: return cast(this.mHead).getLeftCount();
        Self::getLeftCount_static(self.mHead)
    }

    // Java: Node get(int index)
    pub fn get(&self, mut index: i32) -> *mut Node {
        // Java: Node x = this.getRoot();
        let mut x = self.getRoot();
        // Java: while(x != null)
        while !x.is_null() {
            unsafe {
                // Java: if (index < cast(x).getLeftCount())
                let left_count = Self::getLeftCount_static(x);
                if index < left_count {
                    // Java: x = x.left;
                    x = (*x).left;
                // Java: else
                } else {
                    // Java: if (index < cast(x).getLeftCount() + 1)
                    if index < left_count + 1 {
                        // Java: return x;
                        return x;
                    }
                    // Java: index -= cast(x).getLeftCount() + 1;
                    index -= left_count + 1;
                    // Java: x = x.right;
                    x = (*x).right;
                }
            }
        }
        // Java: return x;
        x
    }

    // Java: int index(Node node)
    pub fn index(&self, mut node: *mut Node) -> i32 {
        // Java: int n = cast(node).getLeftCount();
        let mut n = Self::getLeftCount_static(node);
        // Java: Node p;
        // Java: for(Node pRoot = this.getRoot(); node != pRoot; node = p)
        let p_root = self.getRoot();
        unsafe {
            while node != p_root {
                // Java: p = node.parent;
                let p = (*node).parent;
                // Java: if (node == p.right)
                if node == (*p).right {
                    // Java: n += cast(p).getLeftCount() + 1;
                    n += Self::getLeftCount_static(p) + 1;
                }
                node = p;
            }
        }
        // Java: return n;
        n
    }

    // Java: static int getNodeIndex(Node node)
    pub fn getNodeIndex(mut node: *mut Node) -> i32 {
        // Java: int n;
        // Java: Node p;
        // Java: for(n = getLeftCount(node); !node.isRoot(); node = p)
        let mut n = Self::getLeftCount_static(node);
        unsafe {
            while !(*node).isRoot() {
                // Java: p = node.parent;
                let p = (*node).parent;
                // Java: if (node == p.right)
                if node == (*p).right {
                    // Java: n += getLeftCount(p) + 1;
                    n += Self::getLeftCount_static(p) + 1;
                }
                node = p;
            }
        }
        // Java: return n;
        n
    }

    // Java: void onCreateHead(Tree tree, Node head)
    pub fn onCreateHead_impl(&mut self, tree: *mut u8, head: *mut Node) {
        // Java: this.mHead = head;
        self.mHead = head;
        // Java: setLeftCount(this.mHead, 0);
        Self::setLeftCount_static(self.mHead, 0);
    }

    // Java: static void inc(Node n)
    fn inc(n: *mut Node) {
        // Java: IIndexTreeNode p = cast(n);
        // Java: p.setLeftCount(p.getLeftCount() + 1);
        let count = Self::getLeftCount_static(n);
        Self::setLeftCount_static(n, count + 1);
    }

    // Java: static void dec(Node n)
    fn dec(n: *mut Node) {
        // Java: IIndexTreeNode p = cast(n);
        // Java: p.setLeftCount(p.getLeftCount() - 1);
        let count = Self::getLeftCount_static(n);
        Self::setLeftCount_static(n, count - 1);
    }

    // Java: void afterInsert(Node pNew)
    fn afterInsert(&mut self, p_new: *mut Node) {
        // Java: setLeftCount(pNew, 0);
        Self::setLeftCount_static(p_new, 0);
        // Java: Node n = pNew;
        let mut n = p_new;
        // Java: Node p;
        // Java: for(Node root = this.mHead.parent; n != root; n = p)
        unsafe {
            let root = (*self.mHead).parent;
            while n != root {
                // Java: p = n.parent;
                let p = (*n).parent;
                // Java: if (p.left == n)
                if (*p).left == n {
                    // Java: inc(p);
                    Self::inc(p);
                }
                n = p;
            }
        }
        // Java: inc(this.mHead);
        Self::inc(self.mHead);
    }

    // Java: void onBinaryTreeInsertedLeft(Tree tree, Node pNode, Node pNew, int arg1, Object arg2)
    pub fn onBinaryTreeInsertedLeft_impl(&mut self, tree: *mut u8, pnode: *mut Node, pnew: *mut Node, arg1: i32, _arg2: Option<&dyn std::any::Any>) {
        // Java: this.afterInsert(pNew);
        self.afterInsert(pnew);
    }

    // Java: void onBinaryTreeInsertedRight(Tree tree, Node pNode, Node pNew, int arg1, Object arg2)
    pub fn onBinaryTreeInsertedRight_impl(&mut self, tree: *mut u8, pnode: *mut Node, pnew: *mut Node, arg1: i32, _arg2: Option<&dyn std::any::Any>) {
        // Java: this.afterInsert(pNew);
        self.afterInsert(pnew);
    }

    // Java: void beforeSwap(Tree tree, Node z, Node y)
    pub fn beforeSwap_impl(&mut self, tree: *mut u8, z: *mut Node, y: *mut Node) {
        // Java: IIndexTreeNode rz = cast(z);
        // Java: IIndexTreeNode ry = cast(y);
        // Java: int nLeftCount = rz.getLeftCount();
        let n_left_count = Self::getLeftCount_static(z);
        // Java: rz.setLeftCount(ry.getLeftCount());
        Self::setLeftCount_static(z, Self::getLeftCount_static(y));
        // Java: ry.setLeftCount(nLeftCount);
        Self::setLeftCount_static(y, n_left_count);
    }

    // Java: void beforeErase(Tree tree, Node z, Node swp)
    pub fn beforeErase_impl(&mut self, tree: *mut u8, mut z: *mut Node, swp: *mut Node) {
        // Java: Node p;
        // Java: for(p = z.parent; p != this.mHead; p = p.parent)
        unsafe {
            let mut p = (*z).parent;
            while p != self.mHead {
                // Java: if (z == p.left)
                if z == (*p).left {
                    // Java: dec(p);
                    Self::dec(p);
                }
                // Java: z = p;
                z = p;
                p = (*p).parent;
            }
            // Java: dec(p);
            Self::dec(p);
        }
    }

    // Java: void onBinaryTreeErased(Tree tree, Node z, Node x, Node x_parent)
    pub fn onBinaryTreeErased_impl(&mut self, tree: *mut u8, z: *mut u8, x: *mut u8, xparent: *mut u8) {
    }

    // Java: void beforeRotateLeft(Tree tree, Node x, Node y)
    pub fn beforeRotateLeft_impl(&mut self, tree: *mut u8, x: *mut Node, y: *mut Node) {
        // Java: cast(y).setLeftCount(cast(y).getLeftCount() + cast(x).getLeftCount() + 1);
        let new_count = Self::getLeftCount_static(y) + Self::getLeftCount_static(x) + 1;
        Self::setLeftCount_static(y, new_count);
    }

    // Java: void beforeRotateRight(Tree tree, Node y, Node x)
    pub fn beforeRotateRight_impl(&mut self, tree: *mut u8, y: *mut Node, x: *mut Node) {
        // Java: cast(y).setLeftCount(cast(y).getLeftCount() - cast(x).getLeftCount() - 1);
        let new_count = Self::getLeftCount_static(y) - Self::getLeftCount_static(x) - 1;
        Self::setLeftCount_static(y, new_count);
    }

    // Java: ITreeCreator getTreeCreator()
    pub fn getTreeCreator_impl(&self) -> IndexTreeCreator {
        // Java: return new IndexTreeCreator();
        IndexTreeCreator::new()
    }
}

// Java: public class IndexTreeCreator
pub struct IndexTreeCreator;

// Java: public class IndexTreeCreator
impl IndexTreeCreator {
    // Java: IndexTreeCreator()
    pub fn new() -> Self {
        // Java: super();
        Self
    }

    // Java: void init(int nTreeSize)
    pub fn init(&self, ntreesize: i32) {
    }

    // Java: void visit(Node node, VisitorContext ctx)
    pub fn visit(&self, node: *mut Node, ctx: &VisitorContext) {
        // Java: TreeCreatorContext c = (TreeCreatorContext)ctx;
        // Java: IndexTreeExtension.setLeftCount(node, c.getLeftSize());
    }

    // Java: void finish(Node root)
    pub fn finish(&self, root: *mut Node) {
    }

    // Java: void createTree(Tree tree, Node root, INodeProvider list)
    pub fn createTree(&self, tree: *mut u8, root: *mut Node, list: &dyn INodeProvider) {
        // Java: Node head = tree.getHead();
        // Java: IndexTreeExtension.setLeftCount(head, list.getNodeCount());
    }
}

// Java: public interface IIndexTreeNode
pub trait IIndexTreeNode {
    // Java: abstract int getLeftCount()
    fn getLeftCount(&self) -> i32;

    // Java: abstract void setLeftCount(int var1)
    fn setLeftCount(&mut self, var1: i32);
}

#[repr(C)]
pub struct IndexTreeNodeImpl {
    pub base: Node,
    pub left_count: i32,
}

impl IIndexTreeNode for IndexTreeNodeImpl {
    fn getLeftCount(&self) -> i32 { self.left_count }
    fn setLeftCount(&mut self, var1: i32) { self.left_count = var1; }
}

impl ITreeExtension for IndexTreeExtension {
    fn onCreateHead(&mut self, tree: *mut u8, node: *mut u8) {
        self.onCreateHead_impl(tree, node as *mut Node);
    }
    fn onBinaryTreeInsertedLeft(&mut self, tree: *mut u8, var2: *mut u8, var3: *mut u8, var4: i32, var5: Option<&dyn std::any::Any>) {
        self.onBinaryTreeInsertedLeft_impl(tree, var2 as *mut Node, var3 as *mut Node, var4, var5);
    }
    fn onBinaryTreeInsertedRight(&mut self, tree: *mut u8, var2: *mut u8, var3: *mut u8, var4: i32, var5: Option<&dyn std::any::Any>) {
        self.onBinaryTreeInsertedRight_impl(tree, var2 as *mut Node, var3 as *mut Node, var4, var5);
    }
    fn beforeSwap(&mut self, tree: *mut u8, var2: *mut u8, var3: *mut u8) {
        self.beforeSwap_impl(tree, var2 as *mut Node, var3 as *mut Node);
    }
    fn beforeErase(&mut self, tree: *mut u8, var2: *mut u8, var3: *mut u8) {
        self.beforeErase_impl(tree, var2 as *mut Node, var3 as *mut Node);
    }
    fn onBinaryTreeErased(&mut self, tree: *mut u8, var2: *mut u8, var3: *mut u8, var4: *mut u8) {
        self.onBinaryTreeErased_impl(tree, var2, var3, var4);
    }
    fn beforeRotateLeft(&mut self, tree: *mut u8, var2: *mut u8, var3: *mut u8) {
        self.beforeRotateLeft_impl(tree, var2 as *mut Node, var3 as *mut Node);
    }
    fn beforeRotateRight(&mut self, tree: *mut u8, var2: *mut u8, var3: *mut u8) {
        self.beforeRotateRight_impl(tree, var2 as *mut Node, var3 as *mut Node);
    }
    fn getTreeCreator(&self) -> Option<Box<dyn std::any::Any>> {
        Some(Box::new(self.getTreeCreator_impl()))
    }
}
