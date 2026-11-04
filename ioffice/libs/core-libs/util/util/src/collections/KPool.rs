// Java: public class KPool extends KList
pub struct KPool {
    // Java field: KList.HeadNode mHeadNode
    mHeadNode: Option<HeadNode>,
    // Java field: KList.RecycableNodeAlloc mAlloc
    mAlloc: Option<RecycableNodeAlloc>,
}

// Java: public class KPool extends KList
impl KPool {
    // Java: KPool(KList.RecycableNodeAlloc alloc)
    pub fn KPool(&self, KList.RecycableNodeAlloc alloc) {
      // Java: super();
      // TODO_RUST_CODE
      // Java: this.mAlloc = alloc;
      // TODO_RUST_CODE
    }

    // Java: void dispose()
    pub fn dispose(&self) {
        // TODO_RUST_CODE
      {
         // Java: KList.Node pos = this.mHeadNode.next;
         // TODO_RUST_CODE
            // TODO_RUST_CODE
         {
            // Java: KList.Node removeNode = pos;
            // TODO_RUST_CODE
            // Java: pos = pos.next;
            // TODO_RUST_CODE
            // Java: removeNode.dispose();
            // TODO_RUST_CODE
         }
         // Java: this.mHeadNode = null;
         // TODO_RUST_CODE
         // Java: this.mAlloc = null;
         // TODO_RUST_CODE
      }
    }

    // Java: T obtain()
    pub fn obtain(&self) -> T {
      // Java: T t = (T)(this.mHeadNode.next);
      // TODO_RUST_CODE
        // TODO_RUST_CODE
      {
         // Java: t = (T)(this.mAlloc.alloc());
         // TODO_RUST_CODE
      }
        // TODO_RUST_CODE
      {
         // Java: this.mHeadNode.next = t.next;
         // TODO_RUST_CODE
         // Java: t.next = t.prev = null;
         // TODO_RUST_CODE
      }
      // Java: return t;
      // TODO_RUST_CODE
    }

    // Java: void recycle(T t)
    pub fn recycle(&self, T t) {
        // TODO_RUST_CODE
      {
         // Java: this.mAlloc.init(t);
         // TODO_RUST_CODE
         // Java: t.next = this.mHeadNode.next;
         // TODO_RUST_CODE
         // Java: this.mHeadNode.next = t;
         // TODO_RUST_CODE
      }
    }

    // Java: void recycle(KList<T> list)
    pub fn recycle(&self, KList<T> list) {
        // TODO_RUST_CODE
      {
         // Java: KList.HeadNode head = list.headNode;
         // TODO_RUST_CODE
            // TODO_RUST_CODE
         {
            // Java: T t = (T)(node);
            // TODO_RUST_CODE
            // Java: this.mAlloc.init(t);
            // TODO_RUST_CODE
         }
         // Java: list.headNode.prev.next = this.mHeadNode.next;
         // TODO_RUST_CODE
         // Java: this.mHeadNode.next = list.headNode.next;
         // TODO_RUST_CODE
         // Java: list.headNode.next = list.headNode.prev = list.headNode;
         // TODO_RUST_CODE
      }
    }

    // Java: void recycle(KList<T> list, T first, T last)
    pub fn recycle(&self, KList<T> list, T first, T last) {
        // TODO_RUST_CODE
      {
         // Java: KList.HeadNode head = list.headNode;
         // TODO_RUST_CODE
            // TODO_RUST_CODE
         {
            // Java: T t = (T)(node);
            // TODO_RUST_CODE
            // Java: this.mAlloc.init(t);
            // TODO_RUST_CODE
                // TODO_RUST_CODE
            {
               // Java: break;
               // TODO_RUST_CODE
            }
         }
         // Java: first.prev.next = last.next;
         // TODO_RUST_CODE
         // Java: last.next.prev = first.prev;
         // TODO_RUST_CODE
         // Java: last.next = this.mHeadNode.next;
         // TODO_RUST_CODE
         // Java: this.mHeadNode.next = first;
         // TODO_RUST_CODE
      }
    }

}
