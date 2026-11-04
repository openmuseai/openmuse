#![allow(non_snake_case, dead_code, unused_variables, non_upper_case_globals)]

// Java: import java.util.NoSuchElementException

// Java field: String TAG
const TAG: &str = "KList";

// Java: public class KList extends KList
pub struct KList {
    // Java field: HeadNode headNode
    pub headNode: *mut KListNode,
}

// Java: public class Node
pub struct KListNode {
    // Java field: Node next
    pub next: *mut KListNode,
    // Java field: Node prev
    pub prev: *mut KListNode,
    pub is_head: bool,
}

// Java: public class HeadNode

// Java: public class RecycableNode
pub struct RecycableNode {
    // Java field: CloseGuard guard
    pub base: KListNode,
}

// Java: public class RecycableNodeAlloc
pub struct RecycableNodeAlloc {
}

// Java: public class IteratorImpl
pub struct KListIterator {
    // Java field: IteratorImpl<?> next
    // Java field: Object sPoolSync
    // Java field: IteratorImpl<?> sPool
    // Java field: int sPoolSize
    // Java field: int DEFAULT_MAX_POOL_SIZE
    // Java field: int sMaxPoolSize
    // Java field: CloseGuard guard
    // Java field: HeadNode mHead
    pub mHead: *mut KListNode,
    // Java field: Node posNode
    pub posNode: *mut KListNode,
}

pub const DEFAULT_MAX_POOL_SIZE: i32 = 64;

// Java: public class KListEmptyException
pub struct KListEmptyException {
    // Java field: long serialVersionUID
}

// Java: public interface INodeAlloc
pub trait INodeAlloc {
    // Java: abstract T alloc()
    fn alloc(&self) -> *mut KListNode;
    // Java: abstract void init(T var1)
    fn init(&self, node: *mut KListNode);
}

// Java: public interface Iterator
pub trait KListIteratorTrait {
    // Java: abstract IT next()
    fn next(&mut self) -> *mut KListNode;
    // Java: abstract IT prev()
    fn prev(&mut self) -> *mut KListNode;
    // Java: abstract IT next2()
    fn next2(&mut self) -> *mut KListNode;
    // Java: abstract IT prev2()
    fn prev2(&mut self) -> *mut KListNode;
    // Java: abstract void insert(IT var1)
    fn insert(&mut self, node: *mut KListNode);
    // Java: abstract void insertAfter(IT var1)
    fn insertAfter(&mut self, node: *mut KListNode);
    // Java: abstract IT remove()
    fn remove(&mut self) -> *mut KListNode;
    // Java: abstract boolean isEnd()
    fn isEnd(&self) -> bool;
    // Java: abstract boolean hasNext()
    fn hasNext(&self) -> bool;
    // Java: abstract IT value()
    fn value(&self) -> *mut KListNode;
    // Java: abstract Iterator<IT> seekHead()
    fn seekHead(&mut self);
    // Java: abstract Iterator<IT> seekBegin()
    fn seekBegin(&mut self);
    // Java: abstract Iterator<IT> seekEnd()
    fn seekEnd(&mut self);
    // Java: abstract Iterator<IT> seek(IT var1)
    fn seek(&mut self, node: *mut KListNode);
    // Java: abstract Iterator<IT> cloneIterator()
    fn cloneIterator(&self) -> KListIterator;
    // Java: abstract boolean isFirst()
    fn isFirst(&self) -> bool;
    // Java: abstract boolean isLast()
    fn isLast(&self) -> bool;
    // Java: abstract void recycle()
    fn recycle(&mut self);
    // Java: abstract void dispose()
    fn dispose(&mut self);
    // Java: abstract boolean equals(Object var1)
    fn equals(&self, other: &KListIterator) -> bool;
    // Java: abstract boolean valid()
    fn valid(&self) -> bool;
}

// Java: public class Node
impl KListNode {
    // Java: Node()
    pub fn new() -> Self {
        // Java: super();
        Self {
            next: std::ptr::null_mut(),
            prev: std::ptr::null_mut(),
            is_head: false,
        }
    }

    pub fn new_head() -> Self {
        Self {
            next: std::ptr::null_mut(),
            prev: std::ptr::null_mut(),
            is_head: true,
        }
    }

    // Java: boolean isHead()
    pub fn isHead(&self) -> bool {
        // Java: return false;
        self.is_head
    }

    // Java: boolean equals(Object node)
    // Java: return this == node;

    // Java: void dispose()
    pub fn dispose(&mut self) {
        // Java: this.next = this.prev = null;
        self.next = std::ptr::null_mut();
        self.prev = std::ptr::null_mut();
    }
}

// Java: public class HeadNode
// Java: HeadNode()
// Java: super();
// Java: boolean isHead()
// Java: return true;

// Java: public class RecycableNode
impl RecycableNode {
    // Java: RecycableNode()
    pub fn new() -> Self {
        // Java: this(true);
        Self::new_with_guard(true)
    }

    // Java: RecycableNode(boolean withGuard)
    pub fn new_with_guard(with_guard: bool) -> Self {
        // Java: super();
        Self {
            base: KListNode::new(),
        }
    }

    // Java: void dispose()
    pub fn dispose(&mut self) {
    }

    // Java: void finalize()
    // Java: super.finalize();
}

// Java: public class RecycableNodeAlloc
impl RecycableNodeAlloc {
    // Java: RecycableNodeAlloc()
    pub fn new() -> Self {
        // Java: super();
        Self {}
    }

    // Java: void init(NT node)
    pub fn init(&self, _node: *mut KListNode) {
    }
}

// Java: public class KList extends KList
impl KList {
    // Java: KList()
    pub fn new() -> Self {
        // Java: super();
        let head = Box::into_raw(Box::new(KListNode::new_head()));
        unsafe {
            // Java: this.headNode.next = this.headNode.prev = this.headNode;
            (*head).next = head;
            (*head).prev = head;
        }
        Self { headNode: head }
    }

    // Java: Iterator<T> iterator()
    pub fn iterator(&self) -> KListIterator {
        // Java: IteratorImpl it = KList.IteratorImpl.obtain();
        // Java: it.set(this.headNode);
        // Java: return it;
        KListIterator::new(self.headNode)
    }

    // Java: void dispose()
    pub fn dispose(&mut self) {
        // Java: if (this.headNode != null)
        if !self.headNode.is_null()
        {
            // Java: Node pos = this.headNode.next;
            let mut pos = unsafe { (*self.headNode).next };
            // Java: while(null != pos)
            while !pos.is_null()
            {
                // Java: Node removeNode = pos;
                let remove_node = pos;
                // Java: pos = pos.next;
                pos = unsafe { (*pos).next };
                // Java: removeNode.dispose();
                unsafe { (*remove_node).dispose(); }
                if pos == self.headNode { break; }
            }
            // Java: this.headNode = null;
            self.headNode = std::ptr::null_mut();
        }
    }

    // Java: T firstElement()
    pub fn firstElement(&self) -> *mut KListNode {
        // Java: return (T)(this.headNode.next != this.headNode ? this.headNode.next : null);
        unsafe {
            let next = (*self.headNode).next;
            if next != self.headNode { next } else { std::ptr::null_mut() }
        }
    }

    // Java: T lastElement()
    pub fn lastElement(&self) -> *mut KListNode {
        // Java: return (T)(this.headNode.prev != this.headNode ? this.headNode.prev : null);
        unsafe {
            let prev = (*self.headNode).prev;
            if prev != self.headNode { prev } else { std::ptr::null_mut() }
        }
    }

    // Java: boolean isEmpty()
    pub fn isEmpty(&self) -> bool {
        // Java: return this.headNode.next == this.headNode;
        unsafe { (*self.headNode).next == self.headNode }
    }

    // Java: int size()
    pub fn size(&self) -> i32 {
        // Java: return 1 + this.indexOfhelp(this.headNode.prev);
        let last = unsafe { (*self.headNode).prev };
        1 + self.indexOfhelp(last)
    }

    // Java: int indexOf(T node)
    pub fn indexOf(&self, node: *mut KListNode) -> i32 {
        // Java: return node != null ? this.indexOfhelp(node) : -1;
        if !node.is_null() { self.indexOfhelp(node) } else { -1 }
    }

    // Java: boolean contains(T node)
    pub fn contains(&self, node: *mut KListNode) -> bool {
        // Java: return this.indexOf(node) >= 0;
        self.indexOf(node) >= 0
    }

    // Java: void clear()
    pub fn clear(&mut self) {
        // Java: this.headNode.next = this.headNode.prev = this.headNode;
        unsafe {
            (*self.headNode).next = self.headNode;
            (*self.headNode).prev = self.headNode;
        }
    }

    // Java: void add(T node)
    pub fn add(&mut self, node: *mut KListNode) {
        // Java: this.addLast(node);
        self.addLast(node);
    }

    // Java: void add(T pos, T node)
    pub fn add_at(&mut self, pos: *mut KListNode, node: *mut KListNode) {
        // Java: if (node == null)
        if node.is_null()
        {
            // Java: throw new NullPointerException();
            panic!("NullPointerException");
        }
        // Java: else
        {
            // Java: inserthelp(pos, node);
            Self::inserthelp(pos, node);
        }
    }

    // Java: void add(int pos, T node)
    pub fn add_at_index(&mut self, pos: i32, node: *mut KListNode) {
        // Java: if (0 <= pos && pos <= this.size())
        if 0 <= pos && pos <= self.size()
        {
            // Java: if (0 == pos)
            if 0 == pos
            {
                // Java: this.addFirst(node);
                self.addFirst(node);
            }
            // Java: else if (pos == this.size())
            else if pos == self.size()
            {
                // Java: this.addLast(node);
                self.addLast(node);
            }
            // Java: else
            else
            {
                // Java: T t = this.get(pos - 1);
                let t = self.get(pos - 1);
                // Java: this.add(t, node);
                self.add_at(t, node);
            }
        }
    }

    // Java: void addFirst(T node)
    pub fn addFirst(&mut self, node: *mut KListNode) {
        // Java: if (node == null)
        if node.is_null()
        {
            // Java: throw new NullPointerException();
            panic!("NullPointerException");
        }
        // Java: else
        {
            // Java: inserthelp(this.headNode, node);
            Self::inserthelp(self.headNode, node);
        }
    }

    // Java: void addLast(T node)
    pub fn addLast(&mut self, node: *mut KListNode) {
        // Java: if (node == null)
        if node.is_null()
        {
            // Java: throw new NullPointerException();
            panic!("NullPointerException");
        }
        // Java: else
        {
            // Java: inserthelp(this.headNode.prev, node);
            let prev = unsafe { (*self.headNode).prev };
            Self::inserthelp(prev, node);
        }
    }

    // Java: boolean addAll(KList<T> list)
    pub fn addAll(&mut self, list: &mut KList) -> bool {
        // Java: if (list.isEmpty())
        if list.isEmpty()
        {
            // Java: return false;
            return false;
        }
        // Java: else
        {
            // Java: Node dstPos = this.headNode.prev;
            let dst_pos = unsafe { (*self.headNode).prev };
            // Java: Node srcLast = list.headNode.prev;
            let src_last = unsafe { (*list.headNode).prev };
            // Java: Node srcFirst = list.headNode.next;
            let src_first = unsafe { (*list.headNode).next };
            // Java: this.headNode.prev = srcLast;
            unsafe { (*self.headNode).prev = src_last; }
            // Java: srcLast.next = this.headNode;
            unsafe { (*src_last).next = self.headNode; }
            // Java: srcFirst.prev = dstPos;
            unsafe { (*src_first).prev = dst_pos; }
            // Java: dstPos.next = srcFirst;
            unsafe { (*dst_pos).next = src_first; }
            // Java: list.clear();
            list.clear();
            // Java: return true;
            true
        }
    }

    // Java: T remove(T node)
    pub fn remove_node(&mut self, node: *mut KListNode) -> *mut KListNode {
        // Java: if (this.isEmpty())
        if self.isEmpty()
        {
            // Java: return null;
            return std::ptr::null_mut();
        }
        // Java: else
        {
            // Java: this.ensureElementExist(node);
            self.ensureElementExist(node);
            // Java: removehelp(node);
            Self::removehelp(node);
            // Java: return node;
            node
        }
    }

    // Java: T remove(int index)
    pub fn remove_at(&mut self, index: i32) -> *mut KListNode {
        // Java: if (index >= 0 && index < this.size())
        if index >= 0 && index < self.size()
        {
            // Java: T t = this.get(index);
            let t = self.get(index);
            // Java: return (T)this.remove(t);
            self.remove_node(t)
        }
        // Java: else
        else
        {
            // Java: return null;
            std::ptr::null_mut()
        }
    }

    // Java: T removeLast()
    pub fn removeLast(&mut self) -> *mut KListNode {
        // Java: if (!this.isEmpty())
        if !self.isEmpty()
        {
            // Java: T node = (T)this.headNode.prev;
            let node = unsafe { (*self.headNode).prev };
            // Java: removehelp(node);
            Self::removehelp(node);
            // Java: return node;
            node
        }
        // Java: else
        else
        {
            // Java: throw new NoSuchElementException();
            panic!("NoSuchElementException");
        }
    }

    // Java: T removeFirst()
    pub fn removeFirst(&mut self) -> *mut KListNode {
        // Java: if (!this.isEmpty())
        if !self.isEmpty()
        {
            // Java: T node = (T)this.headNode.next;
            let node = unsafe { (*self.headNode).next };
            // Java: removehelp(node);
            Self::removehelp(node);
            // Java: return node;
            node
        }
        // Java: else
        else
        {
            // Java: throw new NoSuchElementException();
            panic!("NoSuchElementException");
        }
    }

    // Java: T get(int index)
    pub fn get(&self, index: i32) -> *mut KListNode {
        // Java: if (index < 0)
        if index < 0
        {
            // Java: return null;
            return std::ptr::null_mut();
        }
        // Java: else
        {
            // Java: Node head = this.headNode;
            let head = self.headNode;
            // Java: Node node = this.headNode.next;
            let mut node = unsafe { (*self.headNode).next };
            // Java: if (node == head)
            if node == head
            {
                // Java: return null;
                return std::ptr::null_mut();
            }
            // Java: else
            {
                // Java: for(int pos = 0; pos < index; ++pos)
                for _pos in 0..index
                {
                    // Java: node = node.next;
                    node = unsafe { (*node).next };
                    // Java: if (node == head)
                    if node == head
                    {
                        // Java: break;
                        break;
                    }
                }
                // Java: return (T)(node == head ? null : node);
                if node == head { std::ptr::null_mut() } else { node }
            }
        }
    }

    // Java: T getNext(T node)
    pub fn getNext(&self, node: *mut KListNode) -> *mut KListNode {
        // Java: Node next = node.next;
        let next = unsafe { (*node).next };
        // Java: return (T)(next.isHead() ? null : next);
        if unsafe { (*next).isHead() } { std::ptr::null_mut() } else { next }
    }

    // Java: T getPrev(T node)
    pub fn getPrev(&self, node: *mut KListNode) -> *mut KListNode {
        // Java: Node prev = node.prev;
        let prev = unsafe { (*node).prev };
        // Java: return (T)(prev.isHead() ? null : prev);
        if unsafe { (*prev).isHead() } { std::ptr::null_mut() } else { prev }
    }

    // Java: void ensureElementExist(Node node)
    fn ensureElementExist(&self, _node: *mut KListNode) {
    }

    // Java: int indexOfhelp(Node node)
    fn indexOfhelp(&self, node: *mut KListNode) -> i32 {
        // Java: int index = 0;
        let mut index: i32 = 0;
        // Java: for(Node pos = this.headNode.next; pos != this.headNode; ++index)
        let mut pos = unsafe { (*self.headNode).next };
        while pos != self.headNode
        {
            // Java: if (node == pos)
            if node == pos
            {
                // Java: return index;
                return index;
            }
            // Java: pos = pos.next;
            pos = unsafe { (*pos).next };
            index += 1;
        }
        // Java: return -1;
        -1
    }

    // Java: static void inserthelp(Node pos, Node node)
    pub fn inserthelp(pos: *mut KListNode, node: *mut KListNode) {
        unsafe {
            // Java: if (null != node.next && null != node.prev)
            if !(*node).next.is_null() && !(*node).prev.is_null()
            {
                // Java: node.prev.next = node.next;
                (*(*node).prev).next = (*node).next;
                // Java: node.next.prev = node.prev;
                (*(*node).next).prev = (*node).prev;
            }
            // Java: Node next = pos.next;
            let next = (*pos).next;
            // Java: pos.next = node;
            (*pos).next = node;
            // Java: next.prev = node;
            (*next).prev = node;
            // Java: node.prev = pos;
            (*node).prev = pos;
            // Java: node.next = next;
            (*node).next = next;
        }
    }

    // Java: static void removehelp(Node node)
    pub fn removehelp(node: *mut KListNode) {
        unsafe {
            // Java: Node next = node.next;
            let next = (*node).next;
            // Java: Node prev = node.prev;
            let prev = (*node).prev;
            // Java: if (null != next)
            if !next.is_null()
            {
                // Java: next.prev = prev;
                (*next).prev = prev;
            }
            // Java: if (null != prev)
            if !prev.is_null()
            {
                // Java: prev.next = next;
                (*prev).next = next;
            }
            // Java: node.next = node.prev = null;
            (*node).next = std::ptr::null_mut();
            (*node).prev = std::ptr::null_mut();
        }
    }

    // Java: String toString()
    pub fn to_string(&self) -> String {
        // Java: StringBuffer buffer = new StringBuffer();
        let mut buffer = String::new();
        // Java: buffer.append("size = ");
        // Java: buffer.append(this.size());
        // Java: buffer.append('\n');
        buffer.push_str(&format!("size = {}\n", self.size()));
        // Java: Iterator<T> iter = this.iterator();
        // Java: int index = 0;
        // Java: while(!iter.isEnd())
        // Java: buffer.append("\n{ index = ");
        // Java: buffer.append(index++);
        // Java: buffer.append("\n");
        // Java: buffer.append(iter.next().toString());
        // Java: buffer.append("\n}");
        // Java: iter.recycle();
        // Java: return buffer.toString();
        buffer
    }
}

// Java: public class IteratorImpl
impl KListIterator {
    // Java: IteratorImpl(HeadNode head)
    pub fn new(head: *mut KListNode) -> Self {
        // Java: super();
        // Java: this.set(head);
        let pos = unsafe { (*head).next };
        Self {
            mHead: head,
            posNode: pos,
        }
    }

    // Java: IteratorImpl()
    pub fn new_empty() -> Self {
        // Java: super();
        Self {
            mHead: std::ptr::null_mut(),
            posNode: std::ptr::null_mut(),
        }
    }

    // Java: void set(HeadNode head)
    pub fn set(&mut self, head: *mut KListNode) {
        // Java: this.mHead = head;
        self.mHead = head;
        // Java: this.posNode = head.next;
        self.posNode = unsafe { (*head).next };
    }

    // Java: IT next()
    pub fn next(&mut self) -> *mut KListNode {
        // Java: Node pos = this.posNode;
        let pos = self.posNode;
        // Java: if (pos != this.mHead)
        if pos != self.mHead
        {
            // Java: this.posNode = pos.next;
            self.posNode = unsafe { (*pos).next };
            // Java: return (IT)pos;
            pos
        }
        // Java: else
        else
        {
            // Java: return null;
            std::ptr::null_mut()
        }
    }

    // Java: IT prev()
    pub fn prev(&mut self) -> *mut KListNode {
        // Java: Node pos = this.posNode;
        let pos = self.posNode;
        // Java: if (pos != this.mHead)
        if pos != self.mHead
        {
            // Java: this.posNode = pos.prev;
            self.posNode = unsafe { (*pos).prev };
            // Java: return (IT)pos;
            pos
        }
        // Java: else
        else
        {
            // Java: return null;
            std::ptr::null_mut()
        }
    }

    // Java: IT next2()
    pub fn next2(&mut self) -> *mut KListNode {
        // Java: return (IT)(this.mHead != (this.posNode = this.posNode.next) ? this.posNode : null);
        self.posNode = unsafe { (*self.posNode).next };
        if self.mHead != self.posNode { self.posNode } else { std::ptr::null_mut() }
    }

    // Java: IT prev2()
    pub fn prev2(&mut self) -> *mut KListNode {
        // Java: return (IT)(this.mHead != (this.posNode = this.posNode.prev) ? this.posNode : null);
        self.posNode = unsafe { (*self.posNode).prev };
        if self.mHead != self.posNode { self.posNode } else { std::ptr::null_mut() }
    }

    // Java: Iterator<IT> seekHead()
    pub fn seekHead(&mut self) {
        // Java: this.posNode = this.mHead;
        self.posNode = self.mHead;
        // Java: return this;
    }

    // Java: Iterator<IT> seekBegin()
    pub fn seekBegin(&mut self) {
        // Java: this.posNode = this.mHead.next;
        self.posNode = unsafe { (*self.mHead).next };
        // Java: return this;
    }

    // Java: Iterator<IT> seekEnd()
    pub fn seekEnd(&mut self) {
        // Java: this.posNode = this.mHead.prev;
        self.posNode = unsafe { (*self.mHead).prev };
        // Java: return this;
    }

    // Java: Iterator<IT> seek(IT node)
    pub fn seek(&mut self, node: *mut KListNode) {
        // Java: if (node == null)
        if node.is_null()
        {
            // Java: throw new NullPointerException();
            panic!("NullPointerException");
        }
        // Java: else
        {
            // Java: this.posNode = node;
            self.posNode = node;
            // Java: return this;
        }
    }

    // Java: void reset(Node pos)
    pub fn reset(&mut self, pos: *mut KListNode) {
        // Java: this.posNode = pos;
        self.posNode = pos;
    }

    // Java: void insert(IT node)
    pub fn insert(&mut self, node: *mut KListNode) {
        // Java: KList.inserthelp(this.posNode.prev, node);
        let prev = unsafe { (*self.posNode).prev };
        KList::inserthelp(prev, node);
        // Java: this.posNode = node;
        self.posNode = node;
    }

    // Java: void insertAfter(IT node)
    pub fn insertAfter(&mut self, node: *mut KListNode) {
        // Java: KList.inserthelp(this.posNode, node);
        KList::inserthelp(self.posNode, node);
        // Java: this.posNode = node;
        self.posNode = node;
    }

    // Java: IT remove()
    pub fn remove(&mut self) -> *mut KListNode {
        // Java: if (this.posNode != this.mHead)
        if self.posNode != self.mHead
        {
            // Java: Node next = this.posNode.next;
            let next = unsafe { (*self.posNode).next };
            // Java: Node rmNode = this.posNode;
            let rm_node = self.posNode;
            // Java: KList.removehelp(this.posNode);
            KList::removehelp(self.posNode);
            // Java: this.posNode = next;
            self.posNode = next;
            // Java: return (IT)rmNode;
            rm_node
        }
        // Java: else
        else
        {
            // Java: throw new IllegalStateException();
            panic!("IllegalStateException");
        }
    }

    // Java: boolean isEnd()
    pub fn isEnd(&self) -> bool {
        // Java: return this.mHead == this.posNode;
        self.mHead == self.posNode
    }

    // Java: boolean hasNext()
    pub fn hasNext(&self) -> bool {
        // Java: return this.mHead != this.posNode.next;
        unsafe { self.mHead != (*self.posNode).next }
    }

    // Java: IT value()
    pub fn value(&self) -> *mut KListNode {
        // Java: if (this.posNode != null && this.posNode != this.mHead)
        if !self.posNode.is_null() && self.posNode != self.mHead
        {
            // Java: return (IT)this.posNode;
            self.posNode
        }
        // Java: else
        else
        {
            // Java: throw new IllegalStateException();
            panic!("IllegalStateException");
        }
    }

    // Java: Iterator<IT> cloneIterator()
    pub fn cloneIterator(&self) -> KListIterator {
        // Java: IteratorImpl<IT> it = obtain();
        // Java: it.mHead = this.mHead;
        // Java: it.reset(this.posNode);
        // Java: return it;
        KListIterator {
            mHead: self.mHead,
            posNode: self.posNode,
        }
    }

    // Java: boolean isFirst()
    pub fn isFirst(&self) -> bool {
        // Java: return this.mHead.next == this.posNode;
        unsafe { (*self.mHead).next == self.posNode }
    }

    // Java: boolean isLast()
    pub fn isLast(&self) -> bool {
        // Java: return this.mHead.prev == this.posNode;
        unsafe { (*self.mHead).prev == self.posNode }
    }

    // Java: static IteratorImpl obtain()
    pub fn obtain() -> KListIterator {
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
        KListIterator::new_empty()
    }

    // Java: void recycle()
    pub fn recycle(&mut self) {
        // Java: this.mHead = null;
        self.mHead = std::ptr::null_mut();
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

    // Java: void dispose()
    pub fn dispose(&mut self) {
        // Java: this.mHead = null;
        self.mHead = std::ptr::null_mut();
        // Java: this.posNode = null;
        self.posNode = std::ptr::null_mut();
        // Java: this.next = null;
    }

    // Java: boolean equals(Object obj)
    // Java: if (this == obj)
    // Java: return true;
    // Java: else if (!(obj instanceof IteratorImpl))
    // Java: return false;
    // Java: else
    // Java: IteratorImpl<IT> rhs = (IteratorImpl)obj;
    // Java: return this.posNode == rhs.posNode;

    // Java: boolean valid()
    pub fn valid(&self) -> bool {
        // Java: if (null == this.posNode)
        if self.posNode.is_null()
        {
            // Java: return false;
            return false;
        }
        // Java: else
        {
            // Java: Node node;
            // Java: for(node = this.posNode.next; null != node && !node.isHead(); node = node.next)
            let mut node = unsafe { (*self.posNode).next };
            while !node.is_null() && unsafe { !(*node).isHead() }
            {
                node = unsafe { (*node).next };
            }
            // Java: return node == this.mHead;
            node == self.mHead
        }
    }
}

// Java: public class KListEmptyException
impl KListEmptyException {
    // Java: KListEmptyException()
    pub fn new() -> Self {
        // Java: super();
        Self {}
    }

    // Java: KListEmptyException(String detailMessage)
    pub fn new_with_message(_detail_message: &str) -> Self {
        // Java: super(detailMessage);
        Self {}
    }
}
