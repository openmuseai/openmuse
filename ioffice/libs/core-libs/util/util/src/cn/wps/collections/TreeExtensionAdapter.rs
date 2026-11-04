#![allow(non_snake_case, dead_code, unused_variables)]

use super::Node::Node;
use super::Tree::Tree;
use super::ITreeExtension::ITreeExtension;
use super::INodeProvider::INodeProvider;
use super::INodeVisitor::INodeVisitor;
use super::ITreeCreator::ITreeCreator;
use super::VisitorContext::VisitorContext;

// Java: public class TreeExtensionAdapter implements ITreeExtension
pub struct TreeExtensionAdapter;

struct NoopTreeCreator;

impl INodeVisitor for NoopTreeCreator {
    fn init(&mut self, _var1: i32) {}
    fn visit(&mut self, _var1: *mut Node, _var2: &mut VisitorContext) {}
    fn finish(&mut self, _var1: *mut Node) {}
}

impl ITreeCreator for NoopTreeCreator {
    fn createTree(&mut self, _var1: &mut Tree, _var2: *mut Node, _var3: &dyn INodeProvider) {}
}

// Java: public class TreeExtensionAdapter implements ITreeExtension
impl TreeExtensionAdapter {
    // Java: TreeExtensionAdapter()
    pub fn new() -> Self {
        // Java: super();
        Self
    }
}

impl ITreeExtension for TreeExtensionAdapter {
    // Java: void beforeErase(Tree tree, Node z, Node swp)
    fn beforeErase(&mut self, _tree: *mut u8, _z: *mut u8, _swp: *mut u8) {
    }

    // Java: void beforeRotateLeft(Tree tree, Node x, Node y)
    fn beforeRotateLeft(&mut self, _tree: *mut u8, _x: *mut u8, _y: *mut u8) {
    }

    // Java: void beforeRotateRight(Tree tree, Node y, Node x)
    fn beforeRotateRight(&mut self, _tree: *mut u8, _y: *mut u8, _x: *mut u8) {
    }

    // Java: void beforeSwap(Tree tree, Node z, Node y)
    fn beforeSwap(&mut self, _tree: *mut u8, _z: *mut u8, _y: *mut u8) {
    }

    // Java: ITreeCreator getTreeCreator()
    fn getTreeCreator(&self) -> Option<Box<dyn std::any::Any>> {
        // Java: return new ITreeCreator()
        // Java: public void createTree(Tree tree, Node root, INodeProvider list)
        // Java: public void finish(Node root)
        // Java: public void init(int nTreeSize)
        // Java: public void visit(Node node, VisitorContext ctx)
        Some(Box::new(NoopTreeCreator))
    }

    // Java: void onBinaryTreeErased(Tree tree, Node z, Node x, Node x_parent)
    fn onBinaryTreeErased(&mut self, _tree: *mut u8, _z: *mut u8, _x: *mut u8, _x_parent: *mut u8) {
    }

    // Java: void onBinaryTreeInsertedLeft(Tree tree, Node pNode, Node pNew, int arg1, Object arg2)
    fn onBinaryTreeInsertedLeft(&mut self, _tree: *mut u8, _p_node: *mut u8, _p_new: *mut u8, _arg1: i32, _arg2: Option<&dyn std::any::Any>) {
    }

    // Java: void onBinaryTreeInsertedRight(Tree tree, Node pNode, Node pNew, int arg1, Object arg2)
    fn onBinaryTreeInsertedRight(&mut self, _tree: *mut u8, _p_node: *mut u8, _p_new: *mut u8, _arg1: i32, _arg2: Option<&dyn std::any::Any>) {
    }

    // Java: void onCreateHead(Tree tree, Node head)
    fn onCreateHead(&mut self, _tree: *mut u8, _head: *mut u8) {
    }
}
