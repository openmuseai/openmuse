#![allow(non_snake_case, dead_code, unused_variables)]

use super::PositionTree::PositionTree;
use super::PositionNode::PositionNode;
use super::IndexTreeExtension::IndexTreeExtension;
use super::ITreeExtension::ITreeExtension;
use super::Node::Node;

// Java: public class IndexedPositionTree extends PositionTree
pub struct IndexedPositionTree {
    pub base: PositionTree,
    // Java field: IndexTreeExtension indexes
    pub(crate) indexes: IndexTreeExtension,
}

// Java: public class IndexedPositionTree extends PositionTree
impl IndexedPositionTree {
    // Java: IndexedPositionTree(PositionNode head)
    pub fn new(head: *mut PositionNode) -> Self {
        // Java: super(head);
        let mut base = PositionTree::new(head);
        // Java: this.addExtension(this.indexes, 59);
        let indexes = IndexTreeExtension::new();
        base.base.addExtension(Box::new(IndexTreeExtension::new()), 59);
        Self {
            base,
            indexes,
        }
    }

    // Java: PositionNode get(int index)
    pub fn get(&self, index: i32) -> *mut PositionNode {
        // Java: return (PositionNode)this.indexes.get(index);
        self.indexes.get(index) as *mut PositionNode
    }

    // Java: int index(PositionNode node)
    pub fn index(&self, node: *mut PositionNode) -> i32 {
        // Java: return this.indexes.index(node);
        self.indexes.index(node as *mut Node)
    }
}
