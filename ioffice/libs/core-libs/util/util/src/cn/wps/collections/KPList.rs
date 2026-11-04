#![allow(non_snake_case, dead_code, unused_variables, non_upper_case_globals)]

// Java: public class Node
pub struct KPNode {
    // Java field: Node next
    pub next: *mut KPNode,
    // Java field: Node prev
    pub prev: *mut KPNode,
}

// Java: public class Node
impl KPNode {
    // Java: Node()
    pub fn new() -> Self {
        // Java: super();
        Self {
            next: std::ptr::null_mut(),
            prev: std::ptr::null_mut(),
        }
    }

    // Java: boolean equals(Object node)
    // Java: return this == node;

    // Java: void dispose()
    pub fn dispose(&mut self) {
        // Java: this.next = this.prev = null;
        self.next = std::ptr::null_mut();
        self.prev = std::ptr::null_mut();
    }

    // Java: abstract void init()
    pub fn init(&mut self) {
    }
}

type Node = KPNode;

// Java: public interface INodeAlloc
pub trait KPNodeAlloc {
    // Java: abstract T alloc()
    fn alloc(&mut self) -> *mut Node;
}

// Java: public interface Iterator
pub trait KPIteratorTrait {
    // Java: abstract IT next()
    fn next(&mut self) -> *mut Node;
    // Java: abstract IT prev()
    fn prev(&mut self) -> *mut Node;
    // Java: abstract IT value()
    fn value(&self) -> *mut Node;
    // Java: abstract Iterator<IT> seekBegin()
    fn seekBegin(&mut self);
    // Java: abstract Iterator<IT> seekEnd()
    fn seekEnd(&mut self);
    // Java: abstract Iterator<IT> seek(IT var1)
    fn seek(&mut self, pos: *mut Node);
    // Java: abstract boolean isFirst()
    fn isFirst(&self) -> bool;
    // Java: abstract boolean isLast()
    fn isLast(&self) -> bool;
    // Java: abstract void recycle()
    fn recycle(&mut self);
    // Java: abstract boolean valid()
    fn valid(&self) -> bool;
}

// Java: public class KPList extends KPList
pub struct KPList {
    // Java field: INodeAlloc<T> alloc
    pub alloc_fn: Option<Box<dyn KPNodeAlloc>>,
    // Java field: Node head
    pub head: *mut Node,
    // Java field: Node tail
    pub tail: *mut Node,
}

pub const DEFAULT_MAX_POOL_SIZE: i32 = 64;

// Java: public class IteratorImpl
pub struct KPListIterator {
    // Java field: IteratorImpl<?> next
    // Java field: Object sPoolSync
    // Java field: IteratorImpl<?> sPool
    // Java field: int sPoolSize
    // Java field: int DEFAULT_MAX_POOL_SIZE
    // Java field: int sMaxPoolSize
    // Java field: CloseGuard guard
    // Java field: KPList mList
    pub mList: *mut KPList,
    // Java field: Node posNode
    pub posNode: *mut Node,
}

// Java: public class KPList
impl KPList {
    // Java: KPList(INodeAlloc<T> alloc)
    pub fn new(mut alloc_fn: Box<dyn KPNodeAlloc>) -> Self {
        // Java: super();
        // Java: this.alloc = alloc;
        // Java: this.tail = this.head = alloc.alloc();
        let head = alloc_fn.alloc();
        // Java: this.head.next = this.head.prev = this.head;
        unsafe {
            (*head).next = head;
            (*head).prev = head;
        }
        Self {
            alloc_fn: Some(alloc_fn),
            head,
            tail: head,
        }
    }

    // Java: void dispose()
    pub fn dispose(&mut self) {
        // Java: if (this.head != null)
        if !self.head.is_null()
        {
            // Java: Node pos = this.head;
            let mut pos = self.head;
            // Java: while(null != pos)
            while !pos.is_null()
            {
                // Java: Node removeNode = pos;
                let remove_node = pos;
                // Java: pos = pos.next;
                pos = unsafe { (*pos).next };
                // Java: removeNode.dispose();
                unsafe { (*remove_node).dispose(); }
                if pos == self.head { break; }
            }
            // Java: this.head = null;
            self.head = std::ptr::null_mut();
            // Java: this.tail = null;
            self.tail = std::ptr::null_mut();
            // Java: this.alloc = null;
            self.alloc_fn = None;
        }
    }

    // Java: Iterator<T> iterator()
    pub fn iterator(&mut self) -> KPListIterator {
        // Java: IteratorImpl it = KPList.IteratorImpl.obtain();
        // Java: it.set(this);
        // Java: return it;
        KPListIterator::new(self as *mut KPList)
    }

    // Java: void clean()
    pub fn clean(&mut self) {
        // Java: if (this.head != this.tail)
        if self.head != self.tail
        {
            // Java: for(Node node = this.head; node != this.tail; node = node.next)
            let mut node = self.head;
            while node != self.tail
            {
                // Java: node.init();
                unsafe { (*node).init(); }
                node = unsafe { (*node).next };
            }
            // Java: this.head = this.tail;
            self.head = self.tail;
        }
    }

    // Java: boolean isEmpty()
    pub fn isEmpty(&self) -> bool {
        // Java: return this.head == this.tail;
        self.head == self.tail
    }

    // Java: T obtainAndAddFirst()
    pub fn obtainAndAddFirst(&mut self) -> *mut Node {
        // Java: T node = (T)this.head.prev;
        let mut node = unsafe { (*self.head).prev };
        // Java: if (node == this.tail)
        if node == self.tail
        {
            // Java: node = this.alloc.alloc();
            node = self.alloc_fn.as_mut().unwrap().alloc();
            // Java: this.insertBefore(this.head, node);
            self.insertBefore(self.head, node);
        }
        // Java: this.head = node;
        self.head = node;
        // Java: return node;
        node
    }

    // Java: T obtainAndAddLast()
    pub fn obtainAndAddLast(&mut self) -> *mut Node {
        // Java: T node = (T)this.tail;
        let node = self.tail;
        // Java: this.tail = node.next;
        self.tail = unsafe { (*node).next };
        // Java: if (this.tail == this.head)
        if self.tail == self.head
        {
            // Java: T newTail = this.alloc.alloc();
            let new_tail = self.alloc_fn.as_mut().unwrap().alloc();
            // Java: this.insertAfter(node, newTail);
            self.insertAfter(node, new_tail);
            // Java: this.tail = newTail;
            self.tail = new_tail;
        }
        // Java: return node;
        node
    }

    // Java: T obtainAndInsertAfter(T pos)
    pub fn obtainAndInsertAfter(&mut self, pos: *mut Node) -> *mut Node {
        // Java: if (null != pos && pos != this.tail.prev)
        if !pos.is_null() && pos != unsafe { (*self.tail).prev }
        {
            // Java: T node = null;
            let node;
            // Java: if (this.head.prev != this.tail)
            if unsafe { (*self.head).prev } != self.tail
            {
                // Java: node = (T)this.head.prev;
                node = unsafe { (*self.head).prev };
                // Java: Node prev = node.prev;
                let prev = unsafe { (*node).prev };
                // Java: prev.next = this.head;
                unsafe { (*prev).next = self.head; }
                // Java: this.head.prev = prev;
                unsafe { (*self.head).prev = prev; }
                // Java: node.next = node.prev = null;
                unsafe { (*node).next = std::ptr::null_mut(); (*node).prev = std::ptr::null_mut(); }
                // Java: this.insertAfter(pos, node);
                self.insertAfter(pos, node);
            }
            // Java: else
            else
            {
                // Java: node = this.alloc.alloc();
                node = self.alloc_fn.as_mut().unwrap().alloc();
                // Java: this.insertAfter(pos, node);
                self.insertAfter(pos, node);
            }
            // Java: return node;
            node
        }
        // Java: else
        else
        {
            // Java: return (T)this.obtainAndAddLast();
            self.obtainAndAddLast()
        }
    }

    // Java: void remove(Node node)
    pub fn remove_single(&mut self, node: *mut Node) {
        // Java: this.remove(node, node);
        self.remove(node, node);
    }

    // Java: void remove(Node first, Node last)
    pub fn remove(&mut self, first: *mut Node, last: *mut Node) {
        // Java: Node node = first;
        let mut node = first;
        // Java: for(Node end = last.next; node != end; node = node.next)
        let end = unsafe { (*last).next };
        while node != end
        {
            // Java: node.init();
            unsafe { (*node).init(); }
            node = unsafe { (*node).next };
        }
        // Java: if (first == this.head)
        if first == self.head
        {
            // Java: this.head = last.next;
            self.head = unsafe { (*last).next };
        }
        // Java: else if (last == this.tail.prev)
        else if last == unsafe { (*self.tail).prev }
        {
            // Java: this.tail = first;
            self.tail = first;
        }
        // Java: else
        else
        {
            // Java: node = first.prev;
            let node = unsafe { (*first).prev };
            // Java: Node next = last.next;
            let next = unsafe { (*last).next };
            // Java: node.next = next;
            unsafe { (*node).next = next; }
            // Java: next.prev = node;
            unsafe { (*next).prev = node; }
            // Java: Node prevOfHead = this.head.prev;
            let prev_of_head = unsafe { (*self.head).prev };
            // Java: prevOfHead.next = first;
            unsafe { (*prev_of_head).next = first; }
            // Java: first.prev = prevOfHead;
            unsafe { (*first).prev = prev_of_head; }
            // Java: last.next = this.head;
            unsafe { (*last).next = self.head; }
            // Java: this.head.prev = last;
            unsafe { (*self.head).prev = last; }
        }
    }

    // Java: void moveBeforePos(Node node, Node pos)
    pub fn moveBeforePos(&mut self, node: *mut Node, pos: *mut Node) {
        // Java: if (null != node && null != pos)
        if !node.is_null() && !pos.is_null()
        {
            // Java: if (node.next != pos)
            if unsafe { (*node).next } != pos
            {
                // Java: Node prevOfNode = node.prev;
                let prev_of_node = unsafe { (*node).prev };
                // Java: Node nextOfNode = node.next;
                let next_of_node = unsafe { (*node).next };
                // Java: prevOfNode.next = nextOfNode;
                unsafe { (*prev_of_node).next = next_of_node; }
                // Java: nextOfNode.prev = prevOfNode;
                unsafe { (*next_of_node).prev = prev_of_node; }
                // Java: Node prevOfPos = pos.prev;
                let prev_of_pos = unsafe { (*pos).prev };
                // Java: prevOfPos.next = node;
                unsafe { (*prev_of_pos).next = node; }
                // Java: node.next = pos;
                unsafe { (*node).next = pos; }
                // Java: pos.prev = node;
                unsafe { (*pos).prev = node; }
                // Java: node.prev = prevOfPos;
                unsafe { (*node).prev = prev_of_pos; }
                // Java: if (this.head == pos)
                if self.head == pos
                {
                    // Java: this.head = node;
                    self.head = node;
                }
                // Java: else if (this.head == node)
                else if self.head == node
                {
                    // Java: this.head = nextOfNode;
                    self.head = next_of_node;
                }
            }
        }
    }

    // Java: void moveAfterPos(Node node, Node pos)
    pub fn moveAfterPos(&mut self, node: *mut Node, pos: *mut Node) {
        // Java: if (null != node && null != pos)
        if !node.is_null() && !pos.is_null()
        {
            // Java: if (pos.next != node)
            if unsafe { (*pos).next } != node
            {
                // Java: Node prevOfNode = node.prev;
                let prev_of_node = unsafe { (*node).prev };
                // Java: Node nextOfNode = node.next;
                let next_of_node = unsafe { (*node).next };
                // Java: prevOfNode.next = nextOfNode;
                unsafe { (*prev_of_node).next = next_of_node; }
                // Java: nextOfNode.prev = prevOfNode;
                unsafe { (*next_of_node).prev = prev_of_node; }
                // Java: Node nextOfPos = pos.next;
                let next_of_pos = unsafe { (*pos).next };
                // Java: nextOfNode.prev = node;
                unsafe { (*next_of_pos).prev = node; }
                // Java: node.prev = pos;
                unsafe { (*node).prev = pos; }
                // Java: pos.next = node;
                unsafe { (*pos).next = node; }
                // Java: node.next = nextOfPos;
                unsafe { (*node).next = next_of_pos; }
                // Java: if (this.head == node)
                if self.head == node
                {
                    // Java: this.head = nextOfNode;
                    self.head = next_of_node;
                }
            }
        }
    }

    // Java: int size()
    pub fn size(&self) -> i32 {
        // Java: Node node = this.head;
        let mut node = self.head;
        // Java: int size;
        // Java: for(size = 0; this.tail != node; ++size)
        let mut size = 0;
        while self.tail != node
        {
            // Java: node = node.next;
            node = unsafe { (*node).next };
            size += 1;
        }
        // Java: return size;
        size
    }

    // Java: T getNext(T node)
    pub fn getNext(&self, node: *mut Node) -> *mut Node {
        // Java: if (null == node)
        if node.is_null()
        {
            // Java: return null;
            return std::ptr::null_mut();
        }
        // Java: else
        {
            // Java: Node next = node.next;
            let next = unsafe { (*node).next };
            // Java: return (T)(next == this.tail ? null : next);
            if next == self.tail { std::ptr::null_mut() } else { next }
        }
    }

    // Java: T getPrev(T node)
    pub fn getPrev(&self, node: *mut Node) -> *mut Node {
        // Java: if (null == node)
        if node.is_null()
        {
            // Java: return null;
            return std::ptr::null_mut();
        }
        // Java: else
        {
            // Java: return (T)(node == this.head ? null : node.prev);
            if node == self.head { std::ptr::null_mut() } else { unsafe { (*node).prev } }
        }
    }

    // Java: boolean addAll(KPList<T> list)
    pub fn addAll(&mut self, list: &mut KPList) -> bool {
        // Java: if (list.isEmpty())
        if list.isEmpty()
        {
            // Java: return false;
            return false;
        }
        // Java: else
        {
            // Java: Node first = list.head;
            let first = list.head;
            // Java: Node prevOfFirst = first.prev;
            let prev_of_first = unsafe { (*first).prev };
            // Java: Node nextOfLast = list.tail;
            let next_of_last = list.tail;
            // Java: Node last = nextOfLast.prev;
            let last = unsafe { (*next_of_last).prev };
            // Java: prevOfFirst.next = nextOfLast;
            unsafe { (*prev_of_first).next = next_of_last; }
            // Java: nextOfLast.prev = prevOfFirst;
            unsafe { (*next_of_last).prev = prev_of_first; }
            // Java: list.head = list.tail;
            list.head = list.tail;
            // Java: if (this.head == this.tail)
            if self.head == self.tail
            {
                // Java: this.head = first;
                self.head = first;
            }
            // Java: Node pos = this.tail.prev;
            let pos = unsafe { (*self.tail).prev };
            // Java: pos.next = first;
            unsafe { (*pos).next = first; }
            // Java: first.prev = pos;
            unsafe { (*first).prev = pos; }
            // Java: last.next = this.tail;
            unsafe { (*last).next = self.tail; }
            // Java: this.tail.prev = last;
            unsafe { (*self.tail).prev = last; }
            // Java: return true;
            true
        }
    }

    // Java: void insertAfter(Node pos, Node node)
    fn insertAfter(&self, pos: *mut Node, node: *mut Node) {
        unsafe {
            // Java: Node next = pos.next;
            let next = (*pos).next;
            // Java: pos.next = node;
            (*pos).next = node;
            // Java: node.prev = pos;
            (*node).prev = pos;
            // Java: node.next = next;
            (*node).next = next;
            // Java: next.prev = node;
            (*next).prev = node;
        }
    }

    // Java: void insertBefore(Node pos, Node node)
    fn insertBefore(&self, pos: *mut Node, node: *mut Node) {
        unsafe {
            // Java: Node prev = pos.prev;
            let prev = (*pos).prev;
            // Java: prev.next = node;
            (*prev).next = node;
            // Java: node.prev = prev;
            (*node).prev = prev;
            // Java: node.next = pos;
            (*node).next = pos;
            // Java: pos.prev = node;
            (*pos).prev = node;
        }
    }

    // Java: T firstElement()
    pub fn firstElement(&self) -> *mut Node {
        // Java: return (T)(this.head == this.tail ? null : this.head);
        if self.head == self.tail { std::ptr::null_mut() } else { self.head }
    }

    // Java: T lastElement()
    pub fn lastElement(&self) -> *mut Node {
        // Java: return (T)(this.head == this.tail ? null : this.tail.prev);
        if self.head == self.tail { std::ptr::null_mut() } else { unsafe { (*self.tail).prev } }
    }

    // Java: String toString()
    pub fn to_string(&self) -> String {
        // Java: StringBuffer buffer = new StringBuffer();
        let mut buffer = String::new();
        // Java: buffer.append("size = ");
        // Java: buffer.append(this.size());
        // Java: buffer.append('\n');
        buffer.push_str(&format!("size = {}\n", self.size()));
        // Java: if (!this.isEmpty())
        // Java: T t = (T)this.head;
        // Java: for(int index = 0; null != t; t = this.getNext(t))
        // Java: buffer.append("\n{ index = ");
        // Java: buffer.append(index++);
        // Java: buffer.append("\n");
        // Java: buffer.append(t.toString());
        // Java: buffer.append("\n}");
        // Java: return buffer.toString();
        buffer
    }
}

// Java: public class Node
// (Node is defined in Node.rs)

// Java: public class IteratorImpl
impl KPListIterator {
    // Java: IteratorImpl(KPList list)
    pub fn new(list: *mut KPList) -> Self {
        // Java: super();
        // Java: this.set(list);
        Self {
            mList: list,
            posNode: std::ptr::null_mut(),
        }
    }

    // Java: IteratorImpl()
    pub fn new_empty() -> Self {
        // Java: super();
        Self {
            mList: std::ptr::null_mut(),
            posNode: std::ptr::null_mut(),
        }
    }

    // Java: void set(KPList list)
    pub fn set(&mut self, list: *mut KPList) {
        // Java: this.mList = list;
        self.mList = list;
    }

    // Java: static IteratorImpl obtain()
    pub fn obtain() -> KPListIterator {
        // Java: synchronized(sPoolSync)
        {
            // Java: if (sPool != null)
            {
                // Java: IteratorImpl m = sPool;
                // Java: sPool = m.next;
                // Java: m.next = null;
                // Java: --sPoolSize;
                // Java: return m;
            }
        }
        // Java: IteratorImpl it = new IteratorImpl();
        // Java: return it;
        KPListIterator::new_empty()
    }

    // Java: void recycle()
    pub fn recycle(&mut self) {
        // Java: this.mList = null;
        self.mList = std::ptr::null_mut();
        // Java: this.posNode = null;
        self.posNode = std::ptr::null_mut();
        // Java: synchronized(sPoolSync)
        {
            // Java: if (sPoolSize < sMaxPoolSize)
            {
                // Java: this.next = sPool;
                // Java: sPool = this;
                // Java: ++sPoolSize;
            }
        }
    }

    // Java: IT next()
    pub fn next(&mut self) -> *mut Node {
        // Java: Node pos = this.posNode;
        let pos = self.posNode;
        // Java: if (null != this.mList)
        if !self.mList.is_null()
        {
            // Java: this.posNode = this.mList.getNext(this.posNode);
            self.posNode = unsafe { (*self.mList).getNext(self.posNode) };
        }
        // Java: return (IT)pos;
        pos
    }

    // Java: IT prev()
    pub fn prev(&mut self) -> *mut Node {
        // Java: Node pos = this.posNode;
        let pos = self.posNode;
        // Java: if (null != this.mList)
        if !self.mList.is_null()
        {
            // Java: this.posNode = this.mList.getPrev(this.posNode);
            self.posNode = unsafe { (*self.mList).getPrev(self.posNode) };
        }
        // Java: return (IT)pos;
        pos
    }

    // Java: IT value()
    pub fn value(&self) -> *mut Node {
        // Java: return (IT)this.posNode;
        self.posNode
    }

    // Java: Iterator<IT> seekBegin()
    pub fn seekBegin(&mut self) {
        // Java: this.posNode = null;
        self.posNode = std::ptr::null_mut();
        // Java: KPList list = this.mList;
        // Java: if (null != list && list.head != list.tail)
        if !self.mList.is_null() {
            let list = unsafe { &*self.mList };
            if list.head != list.tail
            {
                // Java: this.posNode = list.head;
                self.posNode = list.head;
            }
        }
        // Java: return this;
    }

    // Java: Iterator<IT> seekEnd()
    pub fn seekEnd(&mut self) {
        // Java: this.posNode = null;
        self.posNode = std::ptr::null_mut();
        // Java: KPList list = this.mList;
        // Java: if (null != list && list.head != list.tail)
        if !self.mList.is_null() {
            let list = unsafe { &*self.mList };
            if list.head != list.tail
            {
                // Java: this.posNode = list.tail.prev;
                self.posNode = unsafe { (*list.tail).prev };
            }
        }
        // Java: return this;
    }

    // Java: Iterator<IT> seek(IT pos)
    pub fn seek(&mut self, pos: *mut Node) {
        // Java: this.posNode = pos;
        self.posNode = pos;
        // Java: return this;
    }

    // Java: boolean isFirst()
    pub fn isFirst(&self) -> bool {
        // Java: return null != this.mList && null != this.mList.head ? this.posNode == this.mList.head : false;
        if !self.mList.is_null() {
            let list = unsafe { &*self.mList };
            if !list.head.is_null() {
                return self.posNode == list.head;
            }
        }
        false
    }

    // Java: boolean isLast()
    pub fn isLast(&self) -> bool {
        // Java: return null != this.mList && null != this.mList.tail ? this.posNode == this.mList.tail.prev : false;
        if !self.mList.is_null() {
            let list = unsafe { &*self.mList };
            if !list.tail.is_null() {
                return self.posNode == unsafe { (*list.tail).prev };
            }
        }
        false
    }

    // Java: boolean valid()
    pub fn valid(&self) -> bool {
        // Java: if (null != this.posNode && null != this.mList)
        if !self.posNode.is_null() && !self.mList.is_null()
        {
            // Java: Node find = this.posNode;
            let find = self.posNode;
            let list = unsafe { &*self.mList };
            // Java: Node head = this.mList.head;
            // Java: Node tail = this.mList.tail;
            // Java: for(Node curPos = head; curPos != tail; curPos = curPos.next)
            let mut cur_pos = list.head;
            while cur_pos != list.tail
            {
                // Java: if (curPos == find)
                if cur_pos == find
                {
                    // Java: return true;
                    return true;
                }
                cur_pos = unsafe { (*cur_pos).next };
            }
            // Java: return false;
            false
        }
        // Java: else
        else
        {
            // Java: return false;
            false
        }
    }
}
