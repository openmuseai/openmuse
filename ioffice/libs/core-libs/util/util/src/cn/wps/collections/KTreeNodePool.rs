#![allow(non_snake_case, dead_code, unused_variables)]

use std::ptr;
use super::KTree::{KTreeNode, KTreeNodeAlloc, KTree};

// Java: public class KTreeNodePool extends KTree
pub struct KTreeNodePool {
    // Java field: T mHead
    mHead: *mut KTreeNode,
    // Java field: KTree.INodeAlloc<T> mAlloc
    pub(crate) mAlloc: Option<Box<dyn KTreeNodeAlloc>>,
}

// Java: public class KTreeNodePool extends KTree
impl KTreeNodePool {
    // Java: KTreeNodePool(KTree.INodeAlloc<T> alloc)
    pub fn new(alloc: Box<dyn KTreeNodeAlloc>) -> Self {
        // Java: super();
        // Java: this.mAlloc = alloc;
        Self {
            mHead: ptr::null_mut(),
            mAlloc: Some(alloc),
        }
    }

    // Java: void dispose()
    pub fn dispose(&mut self) {
        // Java: while(null != this.mHead)
        while !self.mHead.is_null()
        {
            // Java: T del = this.mHead;
            let del = self.mHead;
            // Java: this.mHead = (T)(del.right);
            self.mHead = unsafe { (*del).right };
            // Java: del.dispose();
            unsafe { (*del).dispose(); }
        }
        // Java: this.mAlloc = null;
        self.mAlloc = None;
    }

    // Java: T obtain()
    pub fn obtain(&mut self) -> *mut KTreeNode {
        // Java: if (this.mHead == null)
        if self.mHead.is_null()
        {
            // Java: return this.mAlloc.alloc();
            return self.mAlloc.as_ref().unwrap().alloc();
        }
        // Java: else
        else
        {
            // Java: T t = this.mHead;
            let t = self.mHead;
            // Java: this.mHead = (T)(this.mHead.right);
            self.mHead = unsafe { (*self.mHead).right };
            // Java: t.left = t.right = t.parent = null;
            unsafe {
                (*t).left = ptr::null_mut();
                (*t).right = ptr::null_mut();
                (*t).parent = ptr::null_mut();
            }
            // Java: this.mAlloc.init(t);
            self.mAlloc.as_ref().unwrap().init(t);
            // Java: return t;
            return t;
        }
    }

    // Java: void recycle(T t)
    pub fn recycle_node(&mut self, t: *mut KTreeNode) {
        // Java: if (null != t)
        if !t.is_null()
        {
            // Java: this.doRecycle(t);
            self.doRecycle(t);
        }
    }

    // Java: void recycle(KTree<T> tree)
    pub fn recycle_tree(&mut self, tree: &mut KTree) {
        // Java: if (!tree.isEmpty())
        if !tree.isEmpty()
        {
            // Java: this.doRecycle(tree.removeRoot());
            let root = tree.removeRoot();
            self.doRecycle(root);
        }
    }

    // Java: void doRecycle(T t)
    fn doRecycle(&mut self, t: *mut KTreeNode) {
        // Java: if (t.left != null)
        if !unsafe { (*t).left }.is_null()
        {
            // Java: this.doRecycle((T)t.left);
            self.doRecycle(unsafe { (*t).left });
        }
        // Java: if (t.right != null)
        if !unsafe { (*t).right }.is_null()
        {
            // Java: this.doRecycle((T)t.right);
            self.doRecycle(unsafe { (*t).right });
        }
        // Java: t.left = null;
        unsafe { (*t).left = ptr::null_mut(); }
        // Java: t.parent = null;
        unsafe { (*t).parent = ptr::null_mut(); }
        // Java: t.right = this.mHead;
        unsafe { (*t).right = self.mHead; }
        // Java: this.mHead = t;
        self.mHead = t;
    }
}
