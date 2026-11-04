#![allow(non_snake_case, dead_code, unused_variables, non_upper_case_globals)]

use std::sync::Mutex;
use super::KList::{KListNode, KList, INodeAlloc};
use super::KPool::KPool;

// Java: public class KSyncPool extends KList
pub struct KSyncPool {
    pool: Mutex<KPool>,
}

// Java: public class KSyncPool extends KList
impl KSyncPool {
    // Java: KSyncPool(KList.RecycableNodeAlloc alloc)
    pub fn new(alloc: Box<dyn INodeAlloc>) -> Self {
        // Java: super(alloc);
        Self {
            pool: Mutex::new(KPool::new(alloc)),
        }
    }

    // Java: T obtain()
    pub fn obtain(&self) -> *mut KListNode {
        // Java: return (T)super.obtain();
        self.pool.lock().unwrap().obtain()
    }

    // Java: void recycle(T t)
    pub fn recycle_node(&self, t: *mut KListNode) {
        // Java: super.recycle(t);
        self.pool.lock().unwrap().recycle_node(t);
    }

    // Java: void recycle(KList<T> list)
    pub fn recycle_list(&self, list: &mut KList) {
        // Java: super.recycle(list);
        self.pool.lock().unwrap().recycle_list(list);
    }

    // Java: void recycle(KList<T> list, T first, T last)
    pub fn recycle_range(&self, list: &mut KList, first: *mut KListNode, last: *mut KListNode) {
        // Java: super.recycle(list, first, last);
        self.pool.lock().unwrap().recycle_range(list, first, last);
    }
}
