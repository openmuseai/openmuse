#![allow(non_snake_case, dead_code, unused_variables, non_upper_case_globals)]

use std::ptr;

// Java: public interface Comparer
pub trait KTreeComparer {
    // Java: abstract boolean greaterThan(T var1, T var2)
    fn greaterThan(&self, a: *mut KTreeNode, b: *mut KTreeNode) -> bool;
}

// Java: public interface INodeAlloc
pub trait KTreeNodeAlloc {
    // Java: abstract T alloc()
    fn alloc(&self) -> *mut KTreeNode;
    // Java: abstract void init(T var1)
    fn init(&self, node: *mut KTreeNode);
}

// Java: public interface Iterator
pub trait KTreeIteratorTrait {
    // Java: abstract IT next()
    fn next(&mut self) -> *mut KTreeNode;
    // Java: abstract IT prev()
    fn prev(&mut self) -> *mut KTreeNode;
    // Java: abstract IT next2()
    fn next2(&mut self) -> *mut KTreeNode;
    // Java: abstract IT prev2()
    fn prev2(&mut self) -> *mut KTreeNode;
    // Java: abstract Iterator<IT> seekBegin()
    fn seekBegin(&mut self);
    // Java: abstract Iterator<IT> seekEnd()
    fn seekEnd(&mut self);
    // Java: abstract Iterator<IT> seek(IT var1)
    fn seek(&mut self, pos: *mut KTreeNode);
    // Java: abstract IT value()
    fn value(&self) -> *mut KTreeNode;
    // Java: abstract boolean isFirst()
    fn isFirst(&self) -> bool;
    // Java: abstract boolean isLast()
    fn isLast(&self) -> bool;
    // Java: abstract Iterator<IT> cloneIterator()
    // Java: abstract boolean isNull()
    fn isNull(&self) -> bool;
    // Java: abstract boolean valid()
    fn valid(&self) -> bool;
    // Java: abstract void recycle()
    fn recycle(&mut self);
}

// Java: public class Node
pub struct KTreeNode {
    // Java field: Node parent
    pub parent: *mut KTreeNode,
    // Java field: Node left
    pub left: *mut KTreeNode,
    // Java field: Node right
    pub right: *mut KTreeNode,
    // Java field: boolean isRed
    pub(crate) isRed: bool,
}

// Java: public class Node
impl KTreeNode {
    // Java: Node()
    pub fn new() -> Self {
        // Java: super();
        Self {
            parent: ptr::null_mut(),
            left: ptr::null_mut(),
            right: ptr::null_mut(),
            isRed: false,
        }
    }

    // Java: void dispose()
    pub fn dispose(&mut self) {
        // Java: this.parent = null;
        self.parent = ptr::null_mut();
        // Java: this.left = null;
        self.left = ptr::null_mut();
        // Java: this.right = null;
        self.right = ptr::null_mut();
        // Java: this.isRed = false;
        self.isRed = false;
    }
}

// Java: public class RecycableNode
pub struct RecycableNode {
    // Java field: CloseGuard guard
    pub base: KTreeNode,
}

// Java: public class RecycableNode
impl RecycableNode {
    // Java: RecycableNode()
    pub fn new() -> Self {
        // Java: this(true);
        Self::new_with_guard(true)
    }

    // Java: RecycableNode(boolean withGuard)
    pub fn new_with_guard(_with_guard: bool) -> Self {
        // Java: super();
        Self { base: KTreeNode::new() }
    }

    // Java: void dispose()
    pub fn dispose(&mut self) {
    }

    // Java: void finalize()
    pub fn finalize(&mut self) {
        // Java: super.finalize();
    }
}

// Java: public class RecycableNodeAlloc
pub struct RecycableNodeAlloc;

// Java: public class RecycableNodeAlloc
impl RecycableNodeAlloc {
    // Java: RecycableNodeAlloc()
    pub fn new() -> Self {
        // Java: super();
        Self
    }

    // Java: void init(NT node)
    pub fn init(&self, _node: *mut RecycableNode) {
    }
}

// Java: public class KTree extends KTree
pub struct KTree {
    // Java field: int mSize
    pub(crate) mSize: i32,
    // Java field: Node mRootNode
    pub(crate) mRootNode: *mut KTreeNode,
    // Java field: Node mEmptyNode
    pub(crate) mEmptyNode: *mut KTreeNode,
    // Java field: Node mBegin
    pub(crate) mBegin: *mut KTreeNode,
    // Java field: Node mEnd
    pub(crate) mEnd: *mut KTreeNode,
}

// Java: public class IteratorImpl
pub struct KTreeIterator {
    // Java field: IteratorImpl<?> next
    // Java field: IteratorImpl<?> sPool
    // Java field: Object sPoolSync
    // Java field: int sPoolSize
    // Java field: int DEFAULT_MAX_POOL_SIZE
    // Java field: int sMaxPoolSize
    // Java field: CloseGuard guard
    // Java field: KTree<IT> mTree
    pub(crate) mTree: *mut KTree,
    // Java field: IT mNode
    pub(crate) mNode: *mut KTreeNode,
}

// Java: public class KTree
impl KTree {
    // Java: KTree()
    pub fn new() -> Self {
        // Java: super();
        let empty = Box::into_raw(Box::new(KTreeNode::new()));
        Self {
            mSize: 0,
            mRootNode: ptr::null_mut(),
            mEmptyNode: empty,
            mBegin: ptr::null_mut(),
            mEnd: ptr::null_mut(),
        }
    }

    // Java: void dispose()
    pub fn dispose(&mut self) {
        // Java: this.mSize = 0;
        self.mSize = 0;
        // Java: if (null != this.mRootNode)
        if !self.mRootNode.is_null()
        {
            // Java: dispose(this.mRootNode);
            Self::dispose_node(self.mRootNode);
            // Java: this.mRootNode = null;
            self.mRootNode = ptr::null_mut();
        }
        // Java: if (null != this.mEmptyNode)
        if !self.mEmptyNode.is_null()
        {
            // Java: this.mEmptyNode.dispose();
            unsafe { (*self.mEmptyNode).dispose(); }
            // Java: this.mEmptyNode = null;
            self.mEmptyNode = ptr::null_mut();
        }
        // Java: this.mBegin = null;
        self.mBegin = ptr::null_mut();
        // Java: this.mEnd = null;
        self.mEnd = ptr::null_mut();
    }

    // Java: static void dispose(Node node)
    fn dispose_node(node: *mut KTreeNode) {
        // Java: if (null != node.left)
        if !unsafe { (*node).left }.is_null()
        {
            // Java: dispose(node.left);
            Self::dispose_node(unsafe { (*node).left });
        }
        // Java: if (null != node.right)
        if !unsafe { (*node).right }.is_null()
        {
            // Java: dispose(node.right);
            Self::dispose_node(unsafe { (*node).right });
        }
        // Java: node.dispose();
        unsafe { (*node).dispose(); }
    }

    // Java: Iterator<T> iterator()
    pub fn iterator(&mut self) -> KTreeIterator {
        // Java: IteratorImpl it = KTree.IteratorImpl.obtain();
        // Java: it.setTree(this);
        // Java: return it;
        let mut it = KTreeIterator::new();
        it.setTree(self as *mut KTree);
        it
    }

    // Java: boolean isEmpty()
    pub fn isEmpty(&self) -> bool {
        // Java: return this.mRootNode == null;
        self.mRootNode.is_null()
    }

    // Java: T removeRoot()
    pub fn removeRoot(&mut self) -> *mut KTreeNode {
        // Java: this.mSize = 0;
        self.mSize = 0;
        // Java: Node root = this.mRootNode;
        let root = self.mRootNode;
        // Java: this.mRootNode = null;
        self.mRootNode = ptr::null_mut();
        // Java: this.mBegin = null;
        self.mBegin = ptr::null_mut();
        // Java: this.mEnd = null;
        self.mEnd = ptr::null_mut();
        // Java: return (T)root;
        root
    }

    // Java: Node getBegin()
    pub fn getBegin(&self) -> *mut KTreeNode {
        // Java: return this.mBegin;
        self.mBegin
    }

    // Java: Node getEnd()
    pub fn getEnd(&self) -> *mut KTreeNode {
        // Java: return this.mEnd;
        self.mEnd
    }

    // Java: void updateBegin()
    pub fn updateBegin(&mut self) {
        // Java: this.mBegin = this.mRootNode.left == null ? this.mRootNode : getLeftBottom(this.mRootNode);
        self.mBegin = if unsafe { (*self.mRootNode).left }.is_null() {
            self.mRootNode
        } else {
            Self::getLeftBottom(self.mRootNode)
        };
    }

    // Java: void updateEnd()
    pub fn updateEnd(&mut self) {
        // Java: this.mEnd = this.mRootNode.right == null ? this.mRootNode : getRightBottom(this.mRootNode);
        self.mEnd = if unsafe { (*self.mRootNode).right }.is_null() {
            self.mRootNode
        } else {
            Self::getRightBottom(self.mRootNode)
        };
    }

    // Java: void insertNode(T node, Comparer compare)
    pub fn insertNode(&mut self, node: *mut KTreeNode, compare: &dyn KTreeComparer) {
        // Java: if (null != node && null != compare)
        if !node.is_null()
        {
            // Java: ++this.mSize;
            self.mSize += 1;
            // Java: if (null == this.mRootNode)
            if self.mRootNode.is_null()
            {
                // Java: this.mRootNode = node;
                self.mRootNode = node;
                // Java: this.mRootNode.isRed = false;
                unsafe { (*self.mRootNode).isRed = false; }
                // Java: this.mBegin = node;
                self.mBegin = node;
                // Java: this.mEnd = node;
                self.mEnd = node;
            }
            // Java: else
            else
            {
                // Java: Node parentNode = null;
                let mut parent_node: *mut KTreeNode = ptr::null_mut();
                // Java: Node insertPos = this.mRootNode;
                let mut insert_pos = self.mRootNode;
                // Java: node.isRed = true;
                unsafe { (*node).isRed = true; }
                // Java: do
                while !insert_pos.is_null()
                {
                    // Java: parentNode = insertPos;
                    parent_node = insert_pos;
                    // Java: if (compare.greaterThan(node, insertPos))
                    if compare.greaterThan(node, insert_pos)
                    {
                        // Java: insertPos = insertPos.right;
                        insert_pos = unsafe { (*insert_pos).right };
                    }
                    // Java: else
                    else
                    {
                        // Java: insertPos = insertPos.left;
                        insert_pos = unsafe { (*insert_pos).left };
                    }
                }
                // Java: if (compare.greaterThan(node, parentNode))
                if compare.greaterThan(node, parent_node)
                {
                    // Java: setRightChild(parentNode, node);
                    Self::setRightChild(parent_node, node);
                }
                // Java: else
                else
                {
                    // Java: setLeftChild(parentNode, node);
                    Self::setLeftChild(parent_node, node);
                }
                // Java: for(Node balanceNode = node; null != balanceNode; balanceNode = this.balanceTreeInsert(balanceNode))
                let mut balance_node = node;
                while !balance_node.is_null()
                {
                    balance_node = self.balanceTreeInsert(balance_node);
                }
                // Java: this.updateBegin();
                self.updateBegin();
                // Java: this.updateEnd();
                self.updateEnd();
            }
        }
    }

    // Java: void insertBefore(T node, T pos)
    pub fn insertBefore(&mut self, node: *mut KTreeNode, pos: *mut KTreeNode) {
        // Java: if (null != node)
        if !node.is_null()
        {
            // Java: ++this.mSize;
            self.mSize += 1;
            // Java: if (null == pos)
            if pos.is_null()
            {
                // Java: node.isRed = false;
                unsafe { (*node).isRed = false; }
                // Java: this.mRootNode = node;
                self.mRootNode = node;
                // Java: this.mBegin = node;
                self.mBegin = node;
                // Java: this.mEnd = node;
                self.mEnd = node;
            }
            // Java: else
            else
            {
                // Java: node.isRed = true;
                unsafe { (*node).isRed = true; }
                // Java: if (null == pos.left)
                if unsafe { (*pos).left }.is_null()
                {
                    // Java: setLeftChild(pos, node);
                    Self::setLeftChild(pos, node);
                }
                // Java: else
                else
                {
                    // Java: Node prevNode = this.findPrevNodeForReplace(pos);
                    let prev_node = self.findPrevNodeForReplace(pos);
                    // Java: setRightChild(prevNode, node);
                    Self::setRightChild(prev_node, node);
                }
                // Java: for(Node balanceNode = node; null != balanceNode; balanceNode = this.balanceTreeInsert(balanceNode))
                let mut balance_node = node;
                while !balance_node.is_null()
                {
                    balance_node = self.balanceTreeInsert(balance_node);
                }
                // Java: if (pos == this.mBegin)
                if pos == self.mBegin
                {
                    // Java: this.mBegin = node;
                    self.mBegin = node;
                }
            }
        }
    }

    // Java: void addLast(T node)
    pub fn addLast(&mut self, node: *mut KTreeNode) {
        // Java: if (null != node)
        if !node.is_null()
        {
            // Java: ++this.mSize;
            self.mSize += 1;
            // Java: if (null == this.mRootNode)
            if self.mRootNode.is_null()
            {
                // Java: this.mRootNode = node;
                self.mRootNode = node;
                // Java: this.mRootNode.isRed = false;
                unsafe { (*self.mRootNode).isRed = false; }
                // Java: this.mBegin = node;
                self.mBegin = node;
                // Java: this.mEnd = node;
                self.mEnd = node;
            }
            // Java: else
            else
            {
                // Java: node.isRed = true;
                unsafe { (*node).isRed = true; }
                // Java: setRightChild(this.mEnd, node);
                Self::setRightChild(self.mEnd, node);
                // Java: for(Node balanceNode = node; null != balanceNode; balanceNode = this.balanceTreeInsert(balanceNode))
                let mut balance_node = node;
                while !balance_node.is_null()
                {
                    balance_node = self.balanceTreeInsert(balance_node);
                }
                // Java: this.mEnd = node;
                self.mEnd = node;
            }
        }
    }

    // Java: T removeNode(T node)
    pub fn removeNode(&mut self, node: *mut KTreeNode) -> *mut KTreeNode {
        // Java: if (null == node)
        if node.is_null()
        {
            // Java: return null;
            return ptr::null_mut();
        }
        // Java: else
        {
            // Java: --this.mSize;
            self.mSize -= 1;
            // Java: Node leftChild = node.left;
            let left_child = unsafe { (*node).left };
            // Java: Node rightChild = node.right;
            let right_child = unsafe { (*node).right };
            // Java: if (node == this.mRootNode && null == leftChild && null == rightChild)
            if node == self.mRootNode && left_child.is_null() && right_child.is_null()
            {
                // Java: this.mRootNode = null;
                self.mRootNode = ptr::null_mut();
                // Java: this.mBegin = null;
                self.mBegin = ptr::null_mut();
                // Java: this.mEnd = null;
                self.mEnd = ptr::null_mut();
                // Java: return node;
                return node;
            }
            // Java: else
            {
                // Java: Node replaceNode;
                let replace_node: *mut KTreeNode;
                // Java: if (null == leftChild && null == rightChild)
                if left_child.is_null() && right_child.is_null()
                {
                    // Java: replaceNode = node;
                    replace_node = node;
                }
                // Java: else if (null != leftChild && leftChild.isRed && null == rightChild)
                else if !left_child.is_null() && unsafe { (*left_child).isRed } && right_child.is_null()
                {
                    // Java: replaceNode = leftChild;
                    replace_node = left_child;
                }
                // Java: else if (null != rightChild && rightChild.isRed && null == leftChild)
                else if !right_child.is_null() && unsafe { (*right_child).isRed } && left_child.is_null()
                {
                    // Java: replaceNode = rightChild;
                    replace_node = right_child;
                }
                // Java: else
                else
                {
                    // Java: replaceNode = this.findPrevNodeForReplace(node);
                    replace_node = self.findPrevNodeForReplace(node);
                }

                // Java: Node newNode = this.replaceNode(node, replaceNode);
                let new_node = self.replaceNode(node, replace_node);
                // Java: boolean replaceNodeIsRed = replaceNode.isRed;
                let replace_node_is_red = unsafe { (*replace_node).isRed };
                // Java: if (replaceNode != node)
                if replace_node != node
                {
                    // Java: replaceNode.isRed = node.isRed;
                    unsafe { (*replace_node).isRed = (*node).isRed; }
                }
                // Java: if (!replaceNodeIsRed)
                if !replace_node_is_red
                {
                    // Java: if (newNode.isRed)
                    if unsafe { (*new_node).isRed }
                    {
                        // Java: newNode.isRed = false;
                        unsafe { (*new_node).isRed = false; }
                    }
                    // Java: else
                    else
                    {
                        // Java: for(Node balanceNode = newNode; null != balanceNode; balanceNode = this.balanceTreeRemove(balanceNode))
                        let mut balance_node = new_node;
                        while !balance_node.is_null()
                        {
                            balance_node = self.balanceTreeRemove(balance_node);
                        }
                    }
                }
                // Java: this.removeEmptyNode();
                self.removeEmptyNode();
                // Java: node.parent = null;
                unsafe { (*node).parent = ptr::null_mut(); }
                // Java: node.left = null;
                unsafe { (*node).left = ptr::null_mut(); }
                // Java: node.right = null;
                unsafe { (*node).right = ptr::null_mut(); }
                // Java: this.updateBegin();
                self.updateBegin();
                // Java: this.updateEnd();
                self.updateEnd();
                // Java: return node;
                node
            }
        }
    }

    // Java: int size()
    pub fn size(&self) -> i32 {
        // Java: return this.mSize;
        self.mSize
    }

    // Java: void removeEmptyNode()
    fn removeEmptyNode(&mut self) {
        // Java: if (null != this.mEmptyNode.parent)
        if !unsafe { (*self.mEmptyNode).parent }.is_null()
        {
            // Java: Node parentNodeForEmptyNode = this.mEmptyNode.parent;
            let parent = unsafe { (*self.mEmptyNode).parent };
            // Java: if (parentNodeForEmptyNode.left == this.mEmptyNode)
            if unsafe { (*parent).left } == self.mEmptyNode
            {
                // Java: setLeftChild(parentNodeForEmptyNode, (Node)null);
                Self::setLeftChild_null(parent);
            }
            // Java: else
            else
            {
                // Java: setRightChild(parentNodeForEmptyNode, (Node)null);
                Self::setRightChild_null(parent);
            }
            // Java: this.mEmptyNode.parent = null;
            unsafe { (*self.mEmptyNode).parent = ptr::null_mut(); }
        }
    }

    // Java: Node balanceTreeInsert(Node node)
    fn balanceTreeInsert(&mut self, node: *mut KTreeNode) -> *mut KTreeNode {
        // Java: Node parentNode = node.parent;
        let parent_node = unsafe { (*node).parent };
        // Java: if (!parentNode.isRed)
        if !unsafe { (*parent_node).isRed }
        {
            // Java: return null;
            return ptr::null_mut();
        }
        // Java: else
        {
            // Java: Node grandFatherNode = parentNode.parent;
            let gf_node = unsafe { (*parent_node).parent };
            // Java: Node uncleNode = null;
            let mut uncle_node: *mut KTreeNode;
            // Java: boolean rightUncle = false;
            let right_uncle;
            // Java: if (grandFatherNode.left == parentNode)
            if unsafe { (*gf_node).left } == parent_node
            {
                // Java: uncleNode = grandFatherNode.right;
                uncle_node = unsafe { (*gf_node).right };
                // Java: rightUncle = true;
                right_uncle = true;
            }
            // Java: else
            else
            {
                // Java: uncleNode = grandFatherNode.left;
                uncle_node = unsafe { (*gf_node).left };
                // Java: rightUncle = false;
                right_uncle = false;
            }
            // Java: if (null != uncleNode && uncleNode.isRed)
            if !uncle_node.is_null() && unsafe { (*uncle_node).isRed }
            {
                // Java: parentNode.isRed = false;
                unsafe { (*parent_node).isRed = false; }
                // Java: uncleNode.isRed = false;
                unsafe { (*uncle_node).isRed = false; }
                // Java: if (this.mRootNode == grandFatherNode)
                if self.mRootNode == gf_node
                {
                    // Java: return null;
                    return ptr::null_mut();
                }
                // Java: else
                else
                {
                    // Java: grandFatherNode.isRed = true;
                    unsafe { (*gf_node).isRed = true; }
                    // Java: return grandFatherNode;
                    return gf_node;
                }
            }
            // Java: else
            else
            {
                // Java: if (rightUncle)
                if right_uncle
                {
                    // Java: if (parentNode.left == node)
                    if unsafe { (*parent_node).left } == node
                    {
                        // Java: parentNode.isRed = false;
                        unsafe { (*parent_node).isRed = false; }
                        // Java: grandFatherNode.isRed = true;
                        unsafe { (*gf_node).isRed = true; }
                        // Java: this.turnLL(parentNode);
                        self.turnLL(parent_node);
                    }
                    // Java: else
                    else
                    {
                        // Java: grandFatherNode.isRed = true;
                        unsafe { (*gf_node).isRed = true; }
                        // Java: node.isRed = false;
                        unsafe { (*node).isRed = false; }
                        // Java: this.turnLR(parentNode);
                        self.turnLR(parent_node);
                    }
                }
                // Java: else if (parentNode.left == node)
                else if unsafe { (*parent_node).left } == node
                {
                    // Java: node.isRed = false;
                    unsafe { (*node).isRed = false; }
                    // Java: grandFatherNode.isRed = true;
                    unsafe { (*gf_node).isRed = true; }
                    // Java: this.turnRL(parentNode);
                    self.turnRL(parent_node);
                }
                // Java: else
                else
                {
                    // Java: parentNode.isRed = false;
                    unsafe { (*parent_node).isRed = false; }
                    // Java: grandFatherNode.isRed = true;
                    unsafe { (*gf_node).isRed = true; }
                    // Java: this.turnRR(parentNode);
                    self.turnRR(parent_node);
                }
                // Java: return null;
                ptr::null_mut()
            }
        }
    }

    // Java: Node balanceTreeRemove(Node node)
    fn balanceTreeRemove(&mut self, node: *mut KTreeNode) -> *mut KTreeNode {
        // Java: if (this.mRootNode == node)
        if self.mRootNode == node
        {
            // Java: return null;
            return ptr::null_mut();
        }
        // Java: else
        {
            // Java: Node parentNode = node.parent;
            let parent_node = unsafe { (*node).parent };
            // Java: boolean isLeft = parentNode.left == node;
            let is_left = unsafe { (*parent_node).left } == node;
            // Java: Node bortherNode = null;
            let borther_node: *mut KTreeNode;
            // Java: if (isLeft)
            if is_left
            {
                // Java: bortherNode = parentNode.right;
                borther_node = unsafe { (*parent_node).right };
            }
            // Java: else
            else
            {
                // Java: bortherNode = parentNode.left;
                borther_node = unsafe { (*parent_node).left };
            }
            // Java: if (null == bortherNode)
            if borther_node.is_null()
            {
                // Java: return null;
                return ptr::null_mut();
            }
            // Java: else if (bortherNode.isRed)
            else if unsafe { (*borther_node).isRed }
            {
                // Java: bortherNode.isRed = false;
                unsafe { (*borther_node).isRed = false; }
                // Java: parentNode.isRed = true;
                unsafe { (*parent_node).isRed = true; }
                // Java: if (isLeft)
                if is_left
                {
                    // Java: this.turnRR(bortherNode);
                    self.turnRR(borther_node);
                }
                // Java: else
                else
                {
                    // Java: this.turnLL(bortherNode);
                    self.turnLL(borther_node);
                }
                // Java: return node;
                return node;
            }
            // Java: else
            else
            {
                // Java: Node node1 = bortherNode.left;
                let node1 = unsafe { (*borther_node).left };
                // Java: Node node2 = bortherNode.right;
                let node2 = unsafe { (*borther_node).right };
                // Java: boolean node1IsRed = node1 != null && node1.isRed;
                let node1_is_red = !node1.is_null() && unsafe { (*node1).isRed };
                // Java: boolean node2IsRed = node2 != null && node2.isRed;
                let node2_is_red = !node2.is_null() && unsafe { (*node2).isRed };
                // Java: if (!node1IsRed && !node2IsRed)
                if !node1_is_red && !node2_is_red
                {
                    // Java: if (parentNode.isRed)
                    if unsafe { (*parent_node).isRed }
                    {
                        // Java: bortherNode.isRed = true;
                        unsafe { (*borther_node).isRed = true; }
                        // Java: parentNode.isRed = false;
                        unsafe { (*parent_node).isRed = false; }
                        // Java: return null;
                        return ptr::null_mut();
                    }
                    // Java: else
                    else
                    {
                        // Java: bortherNode.isRed = true;
                        unsafe { (*borther_node).isRed = true; }
                        // Java: return parentNode;
                        return parent_node;
                    }
                }
                // Java: else
                else
                {
                    // Java: if (isLeft)
                    if is_left
                    {
                        // Java: if (node2IsRed)
                        if node2_is_red
                        {
                            // Java: bortherNode.isRed = parentNode.isRed;
                            unsafe { (*borther_node).isRed = (*parent_node).isRed; }
                            // Java: node2.isRed = false;
                            unsafe { (*node2).isRed = false; }
                            // Java: this.turnRR(bortherNode);
                            self.turnRR(borther_node);
                        }
                        // Java: else
                        else
                        {
                            // Java: node1.isRed = parentNode.isRed;
                            unsafe { (*node1).isRed = (*parent_node).isRed; }
                            // Java: this.turnRL(bortherNode);
                            self.turnRL(borther_node);
                        }
                        // Java: parentNode.isRed = false;
                        unsafe { (*parent_node).isRed = false; }
                    }
                    // Java: else
                    else
                    {
                        // Java: if (node1IsRed)
                        if node1_is_red
                        {
                            // Java: bortherNode.isRed = parentNode.isRed;
                            unsafe { (*borther_node).isRed = (*parent_node).isRed; }
                            // Java: node1.isRed = false;
                            unsafe { (*node1).isRed = false; }
                            // Java: this.turnLL(bortherNode);
                            self.turnLL(borther_node);
                        }
                        // Java: else
                        else
                        {
                            // Java: node2.isRed = parentNode.isRed;
                            unsafe { (*node2).isRed = (*parent_node).isRed; }
                            // Java: this.turnLR(bortherNode);
                            self.turnLR(borther_node);
                        }
                        // Java: parentNode.isRed = false;
                        unsafe { (*parent_node).isRed = false; }
                    }
                    // Java: return null;
                    return ptr::null_mut();
                }
            }
        }
    }

    // Java: Node findPrevNodeForReplace(Node node)
    fn findPrevNodeForReplace(&self, node: *mut KTreeNode) -> *mut KTreeNode {
        // Java: Node prevNode = node.left;
        let mut prev_node = unsafe { (*node).left };
        // Java: for(Node rightChild = prevNode.right; null != rightChild; rightChild = rightChild.right)
        let mut right_child = unsafe { (*prev_node).right };
        while !right_child.is_null()
        {
            // Java: prevNode = rightChild;
            prev_node = right_child;
            right_child = unsafe { (*right_child).right };
        }
        // Java: return prevNode;
        prev_node
    }

    // Java: Node replaceNode(Node node, Node replaceNode)
    fn replaceNode(&mut self, node: *mut KTreeNode, replace_node: *mut KTreeNode) -> *mut KTreeNode {
        // Java: this.onReplaceNode(node, replaceNode);
        self.onReplaceNode(node, replace_node);
        // Java: Node newNode = replaceNode.left;
        let mut new_node = unsafe { (*replace_node).left };
        // Java: if (null == newNode)
        if new_node.is_null()
        {
            // Java: newNode = replaceNode.right;
            new_node = unsafe { (*replace_node).right };
        }
        // Java: if (null == newNode)
        if new_node.is_null()
        {
            // Java: this.removeEmptyNode();
            self.removeEmptyNode();
            // Java: newNode = this.mEmptyNode;
            new_node = self.mEmptyNode;
            // Java: setLeftChild(replaceNode, newNode);
            Self::setLeftChild(replace_node, new_node);
        }
        // Java: if (node.left != replaceNode)
        if unsafe { (*node).left } != replace_node
        {
            // Java: if (this.mRootNode == replaceNode)
            if self.mRootNode == replace_node
            {
                // Java: this.mRootNode = newNode;
                self.mRootNode = new_node;
                // Java: newNode.parent = null;
                unsafe { (*new_node).parent = ptr::null_mut(); }
            }
            // Java: else
            else
            {
                // Java: Node parentNodeOfReplaceNode = replaceNode.parent;
                let parent_of_replace = unsafe { (*replace_node).parent };
                // Java: if (parentNodeOfReplaceNode.left == replaceNode)
                if unsafe { (*parent_of_replace).left } == replace_node
                {
                    // Java: setLeftChild(parentNodeOfReplaceNode, newNode);
                    Self::setLeftChild(parent_of_replace, new_node);
                }
                // Java: else
                else
                {
                    // Java: setRightChild(parentNodeOfReplaceNode, newNode);
                    Self::setRightChild(parent_of_replace, new_node);
                }
            }
        }
        // Java: if (node != replaceNode)
        if node != replace_node
        {
            // Java: if (node == this.mRootNode)
            if node == self.mRootNode
            {
                // Java: this.mRootNode = replaceNode;
                self.mRootNode = replace_node;
                // Java: replaceNode.parent = null;
                unsafe { (*replace_node).parent = ptr::null_mut(); }
            }
            // Java: else
            else
            {
                // Java: Node parentNode = node.parent;
                let parent_node = unsafe { (*node).parent };
                // Java: if (parentNode.left == node)
                if unsafe { (*parent_node).left } == node
                {
                    // Java: setLeftChild(parentNode, replaceNode);
                    Self::setLeftChild(parent_node, replace_node);
                }
                // Java: else
                else
                {
                    // Java: setRightChild(parentNode, replaceNode);
                    Self::setRightChild(parent_node, replace_node);
                }
            }
            // Java: if (replaceNode != node.left)
            if replace_node != unsafe { (*node).left }
            {
                // Java: setLeftChild(replaceNode, node.left);
                Self::setLeftChild(replace_node, unsafe { (*node).left });
            }
            // Java: setRightChild(replaceNode, node.right);
            Self::setRightChild(replace_node, unsafe { (*node).right });
        }
        // Java: return newNode;
        new_node
    }

    // Java: void onReplaceNode(Node node, Node replaceNode)
    pub fn onReplaceNode(&self, _node: *mut KTreeNode, _replace_node: *mut KTreeNode) {
    }

    // Java: void changeRoot(Node oldRoot, Node newRoot)
    fn changeRoot(&mut self, old_root: *mut KTreeNode, new_root: *mut KTreeNode) {
        // Java: if (this.mRootNode == oldRoot)
        if self.mRootNode == old_root
        {
            // Java: this.mRootNode = newRoot;
            self.mRootNode = new_root;
            // Java: newRoot.parent = null;
            unsafe { (*new_root).parent = ptr::null_mut(); }
        }
        // Java: else
        else
        {
            // Java: Node grandGrandFatherNode = oldRoot.parent;
            let ggf = unsafe { (*old_root).parent };
            // Java: if (grandGrandFatherNode.left == oldRoot)
            if unsafe { (*ggf).left } == old_root
            {
                // Java: setLeftChild(grandGrandFatherNode, newRoot);
                Self::setLeftChild(ggf, new_root);
            }
            // Java: else
            else
            {
                // Java: setRightChild(grandGrandFatherNode, newRoot);
                Self::setRightChild(ggf, new_root);
            }
        }
    }

    // Java: void turnLL(Node parentNode)
    fn turnLL(&mut self, parent_node: *mut KTreeNode) {
        // Java: Node rightNode = parentNode.right;
        let right_node = unsafe { (*parent_node).right };
        // Java: Node grandFatherNode = parentNode.parent;
        let gf = unsafe { (*parent_node).parent };
        // Java: this.changeRoot(grandFatherNode, parentNode);
        self.changeRoot(gf, parent_node);
        // Java: setLeftChild(grandFatherNode, rightNode);
        Self::setLeftChild(gf, right_node);
        // Java: setRightChild(parentNode, grandFatherNode);
        Self::setRightChild(parent_node, gf);
        // Java: this.onTurnLL(grandFatherNode, parentNode, rightNode);
        self.onTurnLL(gf, parent_node, right_node);
    }

    // Java: void onTurnLL(Node grandFatherNode, Node parentNode, Node rightNode)
    pub fn onTurnLL(&self, _gf: *mut KTreeNode, _pn: *mut KTreeNode, _rn: *mut KTreeNode) {
    }

    // Java: void turnLR(Node parentNode)
    fn turnLR(&mut self, parent_node: *mut KTreeNode) {
        // Java: Node right = parentNode.right;
        let right = unsafe { (*parent_node).right };
        // Java: Node node2 = right.left;
        let node2 = unsafe { (*right).left };
        // Java: Node node3 = right.right;
        let node3 = unsafe { (*right).right };
        // Java: Node grandFatherNode = parentNode.parent;
        let gf = unsafe { (*parent_node).parent };
        // Java: this.changeRoot(grandFatherNode, right);
        self.changeRoot(gf, right);
        // Java: setLeftChild(right, parentNode);
        Self::setLeftChild(right, parent_node);
        // Java: setRightChild(right, grandFatherNode);
        Self::setRightChild(right, gf);
        // Java: setRightChild(parentNode, node2);
        Self::setRightChild(parent_node, node2);
        // Java: setLeftChild(grandFatherNode, node3);
        Self::setLeftChild(gf, node3);
        // Java: this.onTurnLR(grandFatherNode, parentNode, right, node2, node3);
        self.onTurnLR(gf, parent_node, right, node2, node3);
    }

    // Java: void onTurnLR(Node grandFatherNode, Node parentNode, Node right, Node node2, Node node3)
    pub fn onTurnLR(&self, _gf: *mut KTreeNode, _pn: *mut KTreeNode, _r: *mut KTreeNode, _n2: *mut KTreeNode, _n3: *mut KTreeNode) {
    }

    // Java: void turnRR(Node parentNode)
    fn turnRR(&mut self, parent_node: *mut KTreeNode) {
        // Java: Node left = parentNode.left;
        let left = unsafe { (*parent_node).left };
        // Java: Node grandFatherNode = parentNode.parent;
        let gf = unsafe { (*parent_node).parent };
        // Java: this.changeRoot(grandFatherNode, parentNode);
        self.changeRoot(gf, parent_node);
        // Java: setLeftChild(parentNode, grandFatherNode);
        Self::setLeftChild(parent_node, gf);
        // Java: setRightChild(grandFatherNode, left);
        Self::setRightChild(gf, left);
        // Java: this.onTurnRR(grandFatherNode, parentNode, left);
        self.onTurnRR(gf, parent_node, left);
    }

    // Java: void onTurnRR(Node grandFatherNode, Node parentNode, Node left)
    pub fn onTurnRR(&self, _gf: *mut KTreeNode, _pn: *mut KTreeNode, _l: *mut KTreeNode) {
    }

    // Java: void turnRL(Node parentNode)
    fn turnRL(&mut self, parent_node: *mut KTreeNode) {
        // Java: Node left = parentNode.left;
        let left = unsafe { (*parent_node).left };
        // Java: Node grandFatherNode = parentNode.parent;
        let gf = unsafe { (*parent_node).parent };
        // Java: Node node1 = left.left;
        let node1 = unsafe { (*left).left };
        // Java: Node node2 = left.right;
        let node2 = unsafe { (*left).right };
        // Java: this.changeRoot(grandFatherNode, left);
        self.changeRoot(gf, left);
        // Java: setLeftChild(left, grandFatherNode);
        Self::setLeftChild(left, gf);
        // Java: setRightChild(left, parentNode);
        Self::setRightChild(left, parent_node);
        // Java: setRightChild(grandFatherNode, node1);
        Self::setRightChild(gf, node1);
        // Java: setLeftChild(parentNode, node2);
        Self::setLeftChild(parent_node, node2);
        // Java: this.onTurnRL(grandFatherNode, parentNode, left, node1, node2);
        self.onTurnRL(gf, parent_node, left, node1, node2);
    }

    // Java: void onTurnRL(Node grandFatherNode, Node parentNode, Node left, Node node1, Node node2)
    pub fn onTurnRL(&self, _gf: *mut KTreeNode, _pn: *mut KTreeNode, _l: *mut KTreeNode, _n1: *mut KTreeNode, _n2: *mut KTreeNode) {
    }

    // Java: static void setLeftChild(Node node, Node leftChild)
    pub fn setLeftChild(node: *mut KTreeNode, left_child: *mut KTreeNode) {
        // Java: node.left = leftChild;
        unsafe { (*node).left = left_child; }
        // Java: if (null != leftChild)
        if !left_child.is_null()
        {
            // Java: leftChild.parent = node;
            unsafe { (*left_child).parent = node; }
        }
    }

    fn setLeftChild_null(node: *mut KTreeNode) {
        unsafe { (*node).left = ptr::null_mut(); }
    }

    // Java: static void setRightChild(Node node, Node rightChild)
    pub fn setRightChild(node: *mut KTreeNode, right_child: *mut KTreeNode) {
        // Java: node.right = rightChild;
        unsafe { (*node).right = right_child; }
        // Java: if (null != rightChild)
        if !right_child.is_null()
        {
            // Java: rightChild.parent = node;
            unsafe { (*right_child).parent = node; }
        }
    }

    fn setRightChild_null(node: *mut KTreeNode) {
        unsafe { (*node).right = ptr::null_mut(); }
    }

    // Java: static Node getNext(Node node)
    pub fn getNext(node: *mut KTreeNode) -> *mut KTreeNode {
        // Java: return node.right != null ? getLeftBottom(node.right) : getLeftTop(node).parent;
        if !unsafe { (*node).right }.is_null() {
            Self::getLeftBottom(unsafe { (*node).right })
        } else {
            let top = Self::getLeftTop(node);
            unsafe { (*top).parent }
        }
    }

    // Java: static Node getPrev(Node node)
    pub fn getPrev(node: *mut KTreeNode) -> *mut KTreeNode {
        // Java: return node.left != null ? getRightBottom(node.left) : getRightTop(node).parent;
        if !unsafe { (*node).left }.is_null() {
            Self::getRightBottom(unsafe { (*node).left })
        } else {
            let top = Self::getRightTop(node);
            unsafe { (*top).parent }
        }
    }

    // Java: static Node getRoot(Node node)
    pub fn getRoot(node: *mut KTreeNode) -> *mut KTreeNode {
        // Java: if (null == node)
        if node.is_null()
        {
            // Java: return null;
            return ptr::null_mut();
        }
        // Java: else
        // Java: Node root;
        let mut root = node;
        // Java: for(root = node; null != root.parent; root = root.parent)
        while !unsafe { (*root).parent }.is_null()
        {
            root = unsafe { (*root).parent };
        }
        // Java: return root;
        root
    }

    // Java: static Node getLeftBottom(Node node)
    pub fn getLeftBottom(mut node: *mut KTreeNode) -> *mut KTreeNode {
        // Java: for(Node leftChild = node.left; leftChild != null; leftChild = leftChild.left)
        let mut left_child = unsafe { (*node).left };
        while !left_child.is_null()
        {
            // Java: node = leftChild;
            node = left_child;
            left_child = unsafe { (*left_child).left };
        }
        // Java: return node;
        node
    }

    // Java: static Node getRightBottom(Node node)
    pub fn getRightBottom(mut node: *mut KTreeNode) -> *mut KTreeNode {
        // Java: for(Node rightChild = node.right; rightChild != null; rightChild = rightChild.right)
        let mut right_child = unsafe { (*node).right };
        while !right_child.is_null()
        {
            // Java: node = rightChild;
            node = right_child;
            right_child = unsafe { (*right_child).right };
        }
        // Java: return node;
        node
    }

    // Java: static Node getLeftTop(Node node)
    pub fn getLeftTop(mut node: *mut KTreeNode) -> *mut KTreeNode {
        // Java: for(Node parent = node.parent; null != parent && parent.right == node; parent = parent.parent)
        let mut parent = unsafe { (*node).parent };
        while !parent.is_null() && unsafe { (*parent).right } == node
        {
            // Java: node = parent;
            node = parent;
            parent = unsafe { (*parent).parent };
        }
        // Java: return node;
        node
    }

    // Java: static Node getRightTop(Node node)
    pub fn getRightTop(mut node: *mut KTreeNode) -> *mut KTreeNode {
        // Java: for(Node parent = node.parent; null != parent && parent.left == node; parent = parent.parent)
        let mut parent = unsafe { (*node).parent };
        while !parent.is_null() && unsafe { (*parent).left } == node
        {
            // Java: node = parent;
            node = parent;
            parent = unsafe { (*parent).parent };
        }
        // Java: return node;
        node
    }

    // Java: static void setRed(Node node, boolean value)
    pub fn setRed(node: *mut KTreeNode, value: bool) {
        // Java: node.isRed = value;
        unsafe { (*node).isRed = value; }
    }

    // Java: String toString()
    pub fn to_string(&mut self) -> String {
        // Java: StringBuffer buffer = new StringBuffer();
        // Java: buffer.append("size = ");
        // Java: buffer.append(this.size());
        // Java: buffer.append('\n');
        // Java: Iterator<T> iter = this.iterator();
        // Java: int index = 0;
        // Java: T t = null;
        // Java: while(null != (t = iter.next()))
        // Java: buffer.append("\n{ index = ");
        // Java: buffer.append(index++);
        // Java: buffer.append("\n");
        // Java: buffer.append(t.toString());
        // Java: buffer.append("\n}");
        // Java: iter.recycle();
        // Java: return buffer.toString();
        format!("size = {}\n", self.mSize)
    }
}

// Java: public class IteratorImpl
impl KTreeIterator {
    // Java: IteratorImpl()
    pub fn new() -> Self {
        // Java: super();
        Self {
            mTree: ptr::null_mut(),
            mNode: ptr::null_mut(),
        }
    }

    // Java: static IteratorImpl obtain()
    pub fn obtain() -> KTreeIterator {
        // Java: synchronized(sPoolSync)
        // Java: if (sPool != null)
        // Java: IteratorImpl m = sPool;
        // Java: sPool = m.next;
        // Java: m.next = null;
        // Java: --sPoolSize;
        // Java: return m;
        // Java: IteratorImpl it = new IteratorImpl();
        // Java: return it;
        KTreeIterator::new()
    }

    // Java: void recycle()
    pub fn recycle(&mut self) {
        // Java: synchronized(sPoolSync)
        // Java: if (sPoolSize < sMaxPoolSize)
        // Java: this.next = sPool;
        // Java: sPool = this;
        // Java: ++sPoolSize;
        self.mTree = ptr::null_mut();
        self.mNode = ptr::null_mut();
    }

    // Java: void setTree(KTree<IT> tree)
    pub fn setTree(&mut self, tree: *mut KTree) {
        // Java: this.mTree = tree;
        self.mTree = tree;
        // Java: this.seekBegin();
        self.seekBegin();
    }

    // Java: IT next()
    pub fn next(&mut self) -> *mut KTreeNode {
        // Java: IT curNode = this.mNode;
        let cur_node = self.mNode;
        // Java: if (null != curNode)
        if !cur_node.is_null()
        {
            // Java: this.mNode = (IT)KTree.getNext(curNode);
            self.mNode = KTree::getNext(cur_node);
        }
        // Java: return curNode;
        cur_node
    }

    // Java: IT prev()
    pub fn prev(&mut self) -> *mut KTreeNode {
        // Java: IT curNode = this.mNode;
        let cur_node = self.mNode;
        // Java: if (null != curNode)
        if !cur_node.is_null()
        {
            // Java: this.mNode = (IT)KTree.getPrev(curNode);
            self.mNode = KTree::getPrev(cur_node);
        }
        // Java: return curNode;
        cur_node
    }

    // Java: Iterator<IT> seekBegin()
    pub fn seekBegin(&mut self) {
        // Java: this.mNode = (IT)(null != this.mTree.mRootNode ? this.mTree.mBegin : null);
        let tree = unsafe { &*self.mTree };
        self.mNode = if !tree.mRootNode.is_null() { tree.mBegin } else { ptr::null_mut() };
        // Java: return this;
    }

    // Java: Iterator<IT> seekEnd()
    pub fn seekEnd(&mut self) {
        // Java: this.mNode = (IT)(null != this.mTree.mRootNode ? this.mTree.mEnd : null);
        let tree = unsafe { &*self.mTree };
        self.mNode = if !tree.mRootNode.is_null() { tree.mEnd } else { ptr::null_mut() };
        // Java: return this;
    }

    // Java: Iterator<IT> seek(IT pos)
    pub fn seek(&mut self, pos: *mut KTreeNode) {
        // Java: this.mNode = pos;
        self.mNode = pos;
        // Java: return this;
    }

    // Java: IT value()
    pub fn value(&self) -> *mut KTreeNode {
        // Java: return this.mNode;
        self.mNode
    }

    // Java: boolean isFirst()
    pub fn isFirst(&self) -> bool {
        // Java: return this.mNode != null ? this.mTree.mBegin == this.mNode : false;
        if !self.mNode.is_null() {
            unsafe { (*self.mTree).mBegin == self.mNode }
        } else {
            false
        }
    }

    // Java: boolean isLast()
    pub fn isLast(&self) -> bool {
        // Java: return this.mNode != null ? this.mTree.mEnd == this.mNode : false;
        if !self.mNode.is_null() {
            unsafe { (*self.mTree).mEnd == self.mNode }
        } else {
            false
        }
    }

    // Java: IT next2()
    pub fn next2(&mut self) -> *mut KTreeNode {
        // Java: return this.mNode != null ? (this.mNode = (IT)KTree.getNext(this.mNode)) : null;
        if !self.mNode.is_null() {
            self.mNode = KTree::getNext(self.mNode);
            self.mNode
        } else {
            ptr::null_mut()
        }
    }

    // Java: IT prev2()
    pub fn prev2(&mut self) -> *mut KTreeNode {
        // Java: return this.mNode != null ? (this.mNode = (IT)KTree.getPrev(this.mNode)) : null;
        if !self.mNode.is_null() {
            self.mNode = KTree::getPrev(self.mNode);
            self.mNode
        } else {
            ptr::null_mut()
        }
    }

    // Java: Iterator<IT> cloneIterator()
    pub fn cloneIterator(&self) -> KTreeIterator {
        // Java: IteratorImpl<IT> it = obtain();
        let mut it = Self::obtain();
        // Java: it.setTree(this.mTree);
        it.mTree = self.mTree;
        // Java: it.seek(this.mNode);
        it.mNode = self.mNode;
        // Java: return it;
        it
    }

    // Java: boolean isNull()
    pub fn isNull(&self) -> bool {
        // Java: return this.mNode == null;
        self.mNode.is_null()
    }

    // Java: boolean valid()
    pub fn valid(&self) -> bool {
        // Java: if (this.mNode == null)
        if self.mNode.is_null()
        {
            // Java: return false;
            return false;
        }
        // Java: else
        {
            // Java: for(Node curNode = this.mNode; null != curNode; curNode = curNode.parent)
            let mut cur_node = self.mNode;
            while !cur_node.is_null()
            {
                // Java: if (curNode == this.mTree.mRootNode)
                if cur_node == unsafe { (*self.mTree).mRootNode }
                {
                    // Java: return true;
                    return true;
                }
                cur_node = unsafe { (*cur_node).parent };
            }
            // Java: return false;
            false
        }
    }
}
