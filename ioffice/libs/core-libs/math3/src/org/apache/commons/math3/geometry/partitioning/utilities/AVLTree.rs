#![allow(unused_imports, dead_code, non_snake_case, unused_variables, unused_mut, unused_assignments)]

// Java: public class implements implements AVL trees.
pub struct implements {
}

// Java: public class is
pub struct is {
}

// Java: public class AVLTree
pub struct AVLTree {
    // Java field: Node top
    pub(crate) top: Option<Box<Node>>,
}

// Java: public enum Skew
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Skew {
    LEFT_HIGH,
    RIGHT_HIGH,
    BALANCED,
}

// Java: public class Node
pub struct Node {
    // Java field: T element
    pub(crate) element: Option<Box<dyn std::any::Any>>,
    // Java field: Node left
    pub(crate) left: Option<Box<Node>>,
    // Java field: Node right
    pub(crate) right: Option<Box<Node>>,
    // Java field: Node parent
    pub(crate) parent: *mut Node,
    // Java field: Skew skew
    pub(crate) skew: Skew,
}

// Java: public interface does
pub trait does {
}

// Java: public class implements implements AVL trees.
impl implements {
}

// Java: public class AVLTree
impl AVLTree {
    // Java: AVLTree()
    pub fn new() -> Self {
        // Java: top = null;
        AVLTree { top: None }
    }

    // Java: void insert(final T element)
    pub fn insert(&mut self, _element: Box<dyn std::any::Any>) {
        // Java: if (element != null)
        // Java: if (top == null)
        // Java: top = new Node(element, null);
        // Java: else
        // Java: top.insert(element);
        todo!()
    }

    // Java: boolean delete(final T element)
    pub fn delete(&mut self, _element: &dyn std::any::Any) -> bool {
        // Java: if (element != null)
        // Java: for (Node node = getNotSmaller(element); node != null; node = node.getNext())
        // Java: // loop over all elements neither smaller nor larger
        // Java: // than the specified one
        // Java: if (node.element == element)
        // Java: node.delete();
        // Java: return true;
        // Java: else if (node.element.compareTo(element) > 0)
        // Java: // all the remaining elements are known to be larger,
        // Java: // the element is not in the tree
        // Java: return false;
        // Java: return false;
        todo!()
    }

    // Java: boolean isEmpty()
    pub fn isEmpty(&self) -> bool {
        // Java: return top == null;
        self.top.is_none()
    }

    // Java: int size()
    pub fn size(&self) -> i32 {
        // Java: return (top == null) ? 0 : top.size();
        match &self.top {
            None => 0,
            Some(node) => node.size(),
        }
    }

    // Java: Node getSmallest()
    pub fn getSmallest(&self) -> Option<&Node> {
        // Java: return (top == null) ? null : top.getSmallest();
        self.top.as_ref().map(|n| n.getSmallest())
    }

    // Java: Node getLargest()
    pub fn getLargest(&self) -> Option<&Node> {
        // Java: return (top == null) ? null : top.getLargest();
        self.top.as_ref().map(|n| n.getLargest())
    }

    // Java: Node getNotSmaller(final T reference)
    pub fn getNotSmaller(&self, _reference: &dyn std::any::Any) -> Option<&Node> {
        // Java: Node candidate = null;
        // Java: for (Node node = top; node != null;)
        // Java: if (node.element.compareTo(reference) < 0)
        // Java: if (node.right == null)
        // Java: return candidate;
        // Java: node = node.right;
        // Java: else
        // Java: candidate = node;
        // Java: if (node.left == null)
        // Java: return candidate;
        // Java: node = node.left;
        // Java: return null;
        todo!()
    }

    // Java: Node getNotLarger(final T reference)
    pub fn getNotLarger(&self, _reference: &dyn std::any::Any) -> Option<&Node> {
        // Java: Node candidate = null;
        // Java: for (Node node = top; node != null;)
        // Java: if (node.element.compareTo(reference) > 0)
        // Java: if (node.left == null)
        // Java: return candidate;
        // Java: node = node.left;
        // Java: else
        // Java: candidate = node;
        // Java: if (node.right == null)
        // Java: return candidate;
        // Java: node = node.right;
        // Java: return null;
        todo!()
    }

}

// Java: public enum Skew
impl Skew {
}

// Java: public class Node
impl Node {
    // Java: T getElement()
    pub fn getElement(&self) -> &dyn std::any::Any {
        // Java: return element;
        self.element.as_ref().unwrap().as_ref()
    }

    // Java: int size()
    pub fn size(&self) -> i32 {
        // Java: return 1 + ((left  == null) ? 0 : left.size()) + ((right == null) ? 0 : right.size());
        1 + self.left.as_ref().map_or(0, |n| n.size()) + self.right.as_ref().map_or(0, |n| n.size())
    }

    // Java: Node getSmallest()
    pub fn getSmallest(&self) -> &Node {
        // Java: Node node = this;
        // Java: while (node.left != null)
        // Java: node = node.left;
        // Java: return node;
        let mut node = self;
        while let Some(ref left) = node.left {
            node = left;
        }
        node
    }

    // Java: Node getLargest()
    pub fn getLargest(&self) -> &Node {
        // Java: Node node = this;
        // Java: while (node.right != null)
        // Java: node = node.right;
        // Java: return node;
        let mut node = self;
        while let Some(ref right) = node.right {
            node = right;
        }
        node
    }

    // Java: Node getPrevious()
    pub fn getPrevious(&self) -> Option<&Node> {
        // Java: if (left != null)
        // Java: final Node node = left.getLargest();
        // Java: if (node != null)
        // Java: return node;
        // Java: for (Node node = this; node.parent != null; node = node.parent)
        // Java: if (node != node.parent.left)
        // Java: return node.parent;
        // Java: return null;
        todo!()
    }

    // Java: Node getNext()
    pub fn getNext(&self) -> Option<&Node> {
        // Java: if (right != null)
        // Java: final Node node = right.getSmallest();
        // Java: if (node != null)
        // Java: return node;
        // Java: for (Node node = this; node.parent != null; node = node.parent)
        // Java: if (node != node.parent.right)
        // Java: return node.parent;
        // Java: return null;
        todo!()
    }

    // Java: boolean insert(final T newElement)
    pub fn insert(&mut self, _new_element: Box<dyn std::any::Any>) -> bool {
        // Java: if (newElement.compareTo(this.element) < 0)
        // Java: // the inserted element is smaller than the node
        // Java: if (left == null)
        // Java: left = new Node(newElement, this);
        // Java: return rebalanceLeftGrown();
        // Java: return left.insert(newElement) ? rebalanceLeftGrown() : false;
        // Java: // the inserted element is equal to or greater than the node
        // Java: if (right == null)
        // Java: right = new Node(newElement, this);
        // Java: return rebalanceRightGrown();
        // Java: return right.insert(newElement) ? rebalanceRightGrown() : false;
        todo!()
    }

    // Java: void delete()
    pub fn delete(&mut self) {
        // Java: if ((parent == null) && (left == null) && (right == null))
        // Java: // this was the last node, the tree is now empty
        // Java: element = null;
        // Java: top     = null;
        // Java: else
        // Java: Node node;
        // Java: Node child;
        // Java: boolean leftShrunk;
        // Java: if ((left == null) && (right == null))
        // Java: node       = this;
        // Java: element    = null;
        // Java: leftShrunk = node == node.parent.left;
        // Java: child      = null;
        // Java: else
        // Java: node       = (left != null) ? left.getLargest() : right.getSmallest();
        // Java: element    = node.element;
        // Java: leftShrunk = node == node.parent.left;
        // Java: child      = (node.left != null) ? node.left : node.right;
        // Java: node = node.parent;
        // Java: if (leftShrunk)
        // Java: node.left = child;
        // Java: else
        // Java: node.right = child;
        // Java: if (child != null)
        // Java: child.parent = node;
        // Java: while (leftShrunk ? node.rebalanceLeftShrunk() : node.rebalanceRightShrunk())
        // Java: if (node.parent == null)
        // Java: return;
        // Java: leftShrunk = node == node.parent.left;
        // Java: node = node.parent;
        todo!()
    }

    // Java: boolean rebalanceLeftGrown()
    pub fn rebalanceLeftGrown(&mut self) -> bool {
        // Java: switch (skew)
        // Java: case LEFT_HIGH:
        // Java: if (left.skew == Skew.LEFT_HIGH)
        // Java: rotateCW();
        // Java: skew       = Skew.BALANCED;
        // Java: right.skew = Skew.BALANCED;
        // Java: else
        // Java: final Skew s = left.right.skew;
        // Java: left.rotateCCW();
        // Java: rotateCW();
        // Java: switch(s)
        // Java: case LEFT_HIGH:
        // Java: left.skew  = Skew.BALANCED;
        // Java: right.skew = Skew.RIGHT_HIGH;
        // Java: break;
        // Java: case RIGHT_HIGH:
        // Java: left.skew  = Skew.LEFT_HIGH;
        // Java: right.skew = Skew.BALANCED;
        // Java: break;
        // Java: default:
        // Java: left.skew  = Skew.BALANCED;
        // Java: right.skew = Skew.BALANCED;
        // Java: skew = Skew.BALANCED;
        // Java: return false;
        // Java: case RIGHT_HIGH:
        // Java: skew = Skew.BALANCED;
        // Java: return false;
        // Java: default:
        // Java: skew = Skew.LEFT_HIGH;
        // Java: return true;
        todo!()
    }

    // Java: boolean rebalanceRightGrown()
    pub fn rebalanceRightGrown(&mut self) -> bool {
        // Java: switch (skew)
        // Java: case LEFT_HIGH:
        // Java: skew = Skew.BALANCED;
        // Java: return false;
        // Java: case RIGHT_HIGH:
        // Java: if (right.skew == Skew.RIGHT_HIGH)
        // Java: rotateCCW();
        // Java: skew      = Skew.BALANCED;
        // Java: left.skew = Skew.BALANCED;
        // Java: else
        // Java: final Skew s = right.left.skew;
        // Java: right.rotateCW();
        // Java: rotateCCW();
        // Java: switch (s)
        // Java: case LEFT_HIGH:
        // Java: left.skew  = Skew.BALANCED;
        // Java: right.skew = Skew.RIGHT_HIGH;
        // Java: break;
        // Java: case RIGHT_HIGH:
        // Java: left.skew  = Skew.LEFT_HIGH;
        // Java: right.skew = Skew.BALANCED;
        // Java: break;
        // Java: default:
        // Java: left.skew  = Skew.BALANCED;
        // Java: right.skew = Skew.BALANCED;
        // Java: skew = Skew.BALANCED;
        // Java: return false;
        // Java: default:
        // Java: skew = Skew.RIGHT_HIGH;
        // Java: return true;
        todo!()
    }

    // Java: boolean rebalanceLeftShrunk()
    pub fn rebalanceLeftShrunk(&mut self) -> bool {
        // Java: switch (skew)
        // Java: case LEFT_HIGH:
        // Java: skew = Skew.BALANCED;
        // Java: return true;
        // Java: case RIGHT_HIGH:
        // Java: if (right.skew == Skew.RIGHT_HIGH)
        // Java: rotateCCW();
        // Java: skew      = Skew.BALANCED;
        // Java: left.skew = Skew.BALANCED;
        // Java: return true;
        // Java: else if (right.skew == Skew.BALANCED)
        // Java: rotateCCW();
        // Java: skew      = Skew.LEFT_HIGH;
        // Java: left.skew = Skew.RIGHT_HIGH;
        // Java: return false;
        // Java: else
        // Java: final Skew s = right.left.skew;
        // Java: right.rotateCW();
        // Java: rotateCCW();
        // Java: switch (s)
        // Java: case LEFT_HIGH:
        // Java: left.skew  = Skew.BALANCED;
        // Java: right.skew = Skew.RIGHT_HIGH;
        // Java: break;
        // Java: case RIGHT_HIGH:
        // Java: left.skew  = Skew.LEFT_HIGH;
        // Java: right.skew = Skew.BALANCED;
        // Java: break;
        // Java: default:
        // Java: left.skew  = Skew.BALANCED;
        // Java: right.skew = Skew.BALANCED;
        // Java: skew = Skew.BALANCED;
        // Java: return true;
        // Java: default:
        // Java: skew = Skew.RIGHT_HIGH;
        // Java: return false;
        todo!()
    }

    // Java: boolean rebalanceRightShrunk()
    pub fn rebalanceRightShrunk(&mut self) -> bool {
        // Java: switch (skew)
        // Java: case RIGHT_HIGH:
        // Java: skew = Skew.BALANCED;
        // Java: return true;
        // Java: case LEFT_HIGH:
        // Java: if (left.skew == Skew.LEFT_HIGH)
        // Java: rotateCW();
        // Java: skew       = Skew.BALANCED;
        // Java: right.skew = Skew.BALANCED;
        // Java: return true;
        // Java: else if (left.skew == Skew.BALANCED)
        // Java: rotateCW();
        // Java: skew       = Skew.RIGHT_HIGH;
        // Java: right.skew = Skew.LEFT_HIGH;
        // Java: return false;
        // Java: else
        // Java: final Skew s = left.right.skew;
        // Java: left.rotateCCW();
        // Java: rotateCW();
        // Java: switch (s)
        // Java: case LEFT_HIGH:
        // Java: left.skew  = Skew.BALANCED;
        // Java: right.skew = Skew.RIGHT_HIGH;
        // Java: break;
        // Java: case RIGHT_HIGH:
        // Java: left.skew  = Skew.LEFT_HIGH;
        // Java: right.skew = Skew.BALANCED;
        // Java: break;
        // Java: default:
        // Java: left.skew  = Skew.BALANCED;
        // Java: right.skew = Skew.BALANCED;
        // Java: skew = Skew.BALANCED;
        // Java: return true;
        // Java: default:
        // Java: skew = Skew.LEFT_HIGH;
        // Java: return false;
        todo!()
    }

    // Java: void rotateCW()
    pub fn rotateCW(&mut self) {
        // Java: final T tmpElt       = element;
        // Java: element              = left.element;
        // Java: left.element         = tmpElt;
        // Java: final Node tmpNode   = left;
        // Java: left                 = tmpNode.left;
        // Java: tmpNode.left         = tmpNode.right;
        // Java: tmpNode.right        = right;
        // Java: right                = tmpNode;
        // Java: if (left != null)
        // Java: left.parent = this;
        // Java: if (right.right != null)
        // Java: right.right.parent = right;
        todo!()
    }

    // Java: void rotateCCW()
    pub fn rotateCCW(&mut self) {
        // Java: final T tmpElt        = element;
        // Java: element               = right.element;
        // Java: right.element         = tmpElt;
        // Java: final Node tmpNode    = right;
        // Java: right                 = tmpNode.right;
        // Java: tmpNode.right         = tmpNode.left;
        // Java: tmpNode.left          = left;
        // Java: left                  = tmpNode;
        // Java: if (right != null)
        // Java: right.parent = this;
        // Java: if (left.left != null)
        // Java: left.left.parent = left;
        todo!()
    }

}
