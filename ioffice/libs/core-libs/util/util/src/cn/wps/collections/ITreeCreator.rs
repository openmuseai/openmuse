#![allow(non_snake_case, dead_code)]

use super::Node::Node;
use super::INodeVisitor::INodeVisitor;
use super::INodeProvider::INodeProvider;
use super::Tree::Tree;

// Java: public interface ITreeCreator extends INodeVisitor
pub trait ITreeCreator: INodeVisitor {
    // Java: abstract void createTree(Tree var1, Node var2, INodeProvider var3)
    fn createTree(&mut self, var1: &mut Tree, var2: *mut Node, var3: &dyn INodeProvider);
}
