#![allow(non_snake_case, dead_code, unused_variables)]

use super::Node::Node;
use super::Tree::Tree;
use super::ITreeExtension::ITreeExtension;
use super::INodeProvider::INodeProvider;
use super::INodeVisitor::INodeVisitor;
use super::ITreeCreator::ITreeCreator;
use super::VisitorContext::VisitorContext;

// Java field: int mask
pub const MASK: i32 = 6;

// Java: public class TreeNodeCounter extends TreeExtensionAdapter
pub struct TreeNodeCounter {
    // Java field: int mNodeCount
    pub(crate) mNodeCount: i32,
}

// Java: public class TreeNodeCounterCreator
pub struct TreeNodeCounterCreator {
    counter: *mut TreeNodeCounter,
}

// Java: public class TreeNodeCounter extends TreeExtensionAdapter
impl TreeNodeCounter {
    // Java: TreeNodeCounter()
    pub fn new() -> Self {
        // Java: this(0);
        Self::new_with_count(0)
    }

    // Java: TreeNodeCounter(int nInitialCount)
    pub fn new_with_count(n_initial_count: i32) -> Self {
        // Java: super();
        // Java: this.mNodeCount = nInitialCount;
        Self { mNodeCount: n_initial_count }
    }

    // Java: ITreeCreator getTreeCreator()
    pub fn getTreeCreator_impl(&self) -> Option<Box<dyn std::any::Any>> {
        // Java: return new TreeNodeCounterCreator();
        None
    }

    // Java: void onBinaryTreeErased(Tree tree, Node z, Node x, Node x_parent)
    pub fn onBinaryTreeErased_impl(&mut self, _tree: *mut u8, _z: *mut u8, _x: *mut u8, _x_parent: *mut u8) {
        // Java: --this.mNodeCount;
        self.mNodeCount -= 1;
    }

    // Java: void onBinaryTreeInsertedLeft(Tree tree, Node pNew, Node pNode, int arg1, Object arg2)
    pub fn onBinaryTreeInsertedLeft_impl(&mut self, _tree: *mut u8, _p_new: *mut u8, _p_node: *mut u8, _arg1: i32, _arg2: Option<&dyn std::any::Any>) {
        // Java: ++this.mNodeCount;
        self.mNodeCount += 1;
    }

    // Java: void onBinaryTreeInsertedRight(Tree tree, Node pNew, Node pNode, int arg1, Object arg2)
    pub fn onBinaryTreeInsertedRight_impl(&mut self, _tree: *mut u8, _p_new: *mut u8, _p_node: *mut u8, _arg1: i32, _arg2: Option<&dyn std::any::Any>) {
        // Java: ++this.mNodeCount;
        self.mNodeCount += 1;
    }
}

impl ITreeExtension for TreeNodeCounter {
    fn onCreateHead(&mut self, _tree: *mut u8, _head: *mut u8) {}
    fn onBinaryTreeInsertedLeft(&mut self, tree: *mut u8, p_new: *mut u8, p_node: *mut u8, arg1: i32, arg2: Option<&dyn std::any::Any>) {
        self.onBinaryTreeInsertedLeft_impl(tree, p_new, p_node, arg1, arg2);
    }
    fn onBinaryTreeInsertedRight(&mut self, tree: *mut u8, p_new: *mut u8, p_node: *mut u8, arg1: i32, arg2: Option<&dyn std::any::Any>) {
        self.onBinaryTreeInsertedRight_impl(tree, p_new, p_node, arg1, arg2);
    }
    fn beforeSwap(&mut self, _tree: *mut u8, _z: *mut u8, _y: *mut u8) {}
    fn beforeErase(&mut self, _tree: *mut u8, _z: *mut u8, _swp: *mut u8) {}
    fn onBinaryTreeErased(&mut self, tree: *mut u8, z: *mut u8, x: *mut u8, x_parent: *mut u8) {
        self.onBinaryTreeErased_impl(tree, z, x, x_parent);
    }
    fn beforeRotateLeft(&mut self, _tree: *mut u8, _x: *mut u8, _y: *mut u8) {}
    fn beforeRotateRight(&mut self, _tree: *mut u8, _y: *mut u8, _x: *mut u8) {}
    fn getTreeCreator(&self) -> Option<Box<dyn std::any::Any>> {
        self.getTreeCreator_impl()
    }
}

// Java: public class TreeNodeCounterCreator
impl TreeNodeCounterCreator {
    // Java: TreeNodeCounterCreator()
    pub fn new(counter: *mut TreeNodeCounter) -> Self {
        // Java: super();
        Self { counter }
    }
}

impl INodeVisitor for TreeNodeCounterCreator {
    fn init(&mut self, n_tree_size: i32) {
        unsafe { (*self.counter).mNodeCount = n_tree_size; }
    }
    fn visit(&mut self, _node: *mut Node, _ctx: &mut VisitorContext) {}
    fn finish(&mut self, _root: *mut Node) {}
}

impl ITreeCreator for TreeNodeCounterCreator {
    // Java: void createTree(Tree tree, Node root, INodeProvider list)
    fn createTree(&mut self, _tree: &mut Tree, _root: *mut Node, _list: &dyn INodeProvider) {
    }
}

// Java: void finish(Node root)
// Java: void init(int nTreeSize)
// Java: TreeNodeCounter.this.mNodeCount = nTreeSize;
// Java: void visit(Node node, VisitorContext ctx)
