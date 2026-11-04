#![allow(non_snake_case, dead_code, unused_variables, non_upper_case_globals)]

use super::KList::{KListNode, KList, INodeAlloc};

// Java: public class KPool extends KList
pub struct KPool {
    // Java field: KList.HeadNode mHeadNode
    mHeadNode: *mut KListNode,
    // Java field: KList.RecycableNodeAlloc mAlloc
    mAlloc: Option<Box<dyn INodeAlloc>>,
}

// Java: public class KPool extends KList
impl KPool {
    // Java: KPool(KList.RecycableNodeAlloc alloc)
    pub fn new(alloc: Box<dyn INodeAlloc>) -> Self {
        // Java: super();
        let head = Box::into_raw(Box::new(KListNode::new_head()));
        // Java: this.mAlloc = alloc;
        Self {
            mHeadNode: head,
            mAlloc: Some(alloc),
        }
    }

    // Java: void dispose()
    pub fn dispose(&mut self) {
        // Java: if (this.mHeadNode != null)
        if !self.mHeadNode.is_null()
        {
            // Java: KList.Node pos = this.mHeadNode.next;
            let mut pos = unsafe { (*self.mHeadNode).next };
            // Java: while(null != pos)
            while !pos.is_null()
            {
                // Java: KList.Node removeNode = pos;
                let remove_node = pos;
                // Java: pos = pos.next;
                pos = unsafe { (*pos).next };
                // Java: removeNode.dispose();
                unsafe { (*remove_node).dispose(); }
            }
            // Java: this.mHeadNode = null;
            self.mHeadNode = std::ptr::null_mut();
            // Java: this.mAlloc = null;
            self.mAlloc = None;
        }
    }

    // Java: T obtain()
    pub fn obtain(&mut self) -> *mut KListNode {
        // Java: T t = (T)(this.mHeadNode.next);
        let t = unsafe { (*self.mHeadNode).next };
        // Java: if (null == t)
        if t.is_null()
        {
            // Java: t = (T)(this.mAlloc.alloc());
            self.mAlloc.as_ref().unwrap().alloc()
        }
        // Java: else
        else
        {
            // Java: this.mHeadNode.next = t.next;
            unsafe { (*self.mHeadNode).next = (*t).next; }
            // Java: t.next = t.prev = null;
            unsafe { (*t).next = std::ptr::null_mut(); (*t).prev = std::ptr::null_mut(); }
            // Java: return t;
            t
        }
    }

    // Java: void recycle(T t)
    pub fn recycle_node(&mut self, t: *mut KListNode) {
        // Java: if (null != t)
        if !t.is_null()
        {
            // Java: this.mAlloc.init(t);
            self.mAlloc.as_ref().unwrap().init(t);
            // Java: t.next = this.mHeadNode.next;
            unsafe { (*t).next = (*self.mHeadNode).next; }
            // Java: this.mHeadNode.next = t;
            unsafe { (*self.mHeadNode).next = t; }
        }
    }

    // Java: void recycle(KList<T> list)
    pub fn recycle_list(&mut self, list: &mut KList) {
        // Java: if (!list.isEmpty())
        if !list.isEmpty()
        {
            // Java: KList.HeadNode head = list.headNode;
            let head = list.headNode;
            // Java: for(KList.Node node = list.headNode.next; node != head; node = node.next)
            let mut node = unsafe { (*list.headNode).next };
            while node != head
            {
                // Java: T t = (T)(node);
                let t = node;
                // Java: this.mAlloc.init(t);
                self.mAlloc.as_ref().unwrap().init(t);
                node = unsafe { (*node).next };
            }
            // Java: list.headNode.prev.next = this.mHeadNode.next;
            unsafe { (*(*list.headNode).prev).next = (*self.mHeadNode).next; }
            // Java: this.mHeadNode.next = list.headNode.next;
            unsafe { (*self.mHeadNode).next = (*list.headNode).next; }
            // Java: list.headNode.next = list.headNode.prev = list.headNode;
            unsafe { (*list.headNode).next = list.headNode; (*list.headNode).prev = list.headNode; }
        }
    }

    // Java: void recycle(KList<T> list, T first, T last)
    pub fn recycle_range(&mut self, list: &mut KList, first: *mut KListNode, last: *mut KListNode) {
        // Java: if (!list.isEmpty() && null != first && null != last)
        if !list.isEmpty() && !first.is_null() && !last.is_null()
        {
            // Java: KList.HeadNode head = list.headNode;
            let head = list.headNode;
            // Java: for(KList.Node node = first; node != head; node = node.next)
            let mut node = first;
            while node != head
            {
                // Java: T t = (T)(node);
                let t = node;
                // Java: this.mAlloc.init(t);
                self.mAlloc.as_ref().unwrap().init(t);
                // Java: if (node == last)
                if node == last
                {
                    // Java: break;
                    break;
                }
                node = unsafe { (*node).next };
            }
            // Java: first.prev.next = last.next;
            unsafe { (*(*first).prev).next = (*last).next; }
            // Java: last.next.prev = first.prev;
            unsafe { (*(*last).next).prev = (*first).prev; }
            // Java: last.next = this.mHeadNode.next;
            unsafe { (*last).next = (*self.mHeadNode).next; }
            // Java: this.mHeadNode.next = first;
            unsafe { (*self.mHeadNode).next = first; }
        }
    }
}
