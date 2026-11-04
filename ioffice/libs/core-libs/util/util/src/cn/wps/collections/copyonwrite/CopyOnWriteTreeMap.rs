#![allow(dead_code, non_snake_case, unused_variables, unused_assignments, unused_mut)]

// Java: import java.util.ArrayList
// Java: import java.util.Comparator

use std::sync::{Arc, Mutex};
use std::any::Any;

type K = i32;
type Val = Option<Arc<dyn Any + Send + Sync>>;

// Java field: boolean DEBUG
pub(crate) const DEBUG: bool = false;
// Java field: ArrayList<RB_Node<?, ?>> sSharePool
struct SendNodePtr(*mut TRBNode);
unsafe impl Send for SendNodePtr {}
unsafe impl Sync for SendNodePtr {}
static S_SHARE_POOL_T: Mutex<Vec<SendNodePtr>> = Mutex::new(Vec::new());
// Java field: int LOCAL_CACHE_MAX
pub(crate) const LOCAL_CACHE_MAX: i32 = 128;
// Java field: int SHARE_CACHE_MAX
pub(crate) const SHARE_CACHE_MAX: i32 = 4096;

// Java: public class RB_Node
pub struct TRBNode {
    // Java field: K key
    pub(crate) key: K,
    // Java field: V value
    pub(crate) value: Val,
    // Java field: RB_Node<K, V> left
    pub(crate) left: *mut TRBNode,
    // Java field: RB_Node<K, V> right
    pub(crate) right: *mut TRBNode,
    // Java field: boolean isBlack
    pub(crate) isBlack: bool,
    // Java field: int refCount
    pub(crate) refCount: i32,
}

// Java: public class CopyOnWriteTreeMap
pub struct CopyOnWriteTreeMap {
    // Java field: Comparator<? super K> comparator
    comparator: Box<dyn Fn(K, K) -> i32 + Send + Sync>,
    // Java field: RB_Node<K, V> NIL
    pub(crate) NIL: *mut TRBNode,
    // Java field: Object _rootLock
    _rootLock: Mutex<()>,
    // Java field: RB_Node<K, V> _root
    _root: *mut TRBNode,
    // Java field: int _size
    pub(crate) _size: i32,
    // Java field: Alloc<K, V> _alloc
    pub(crate) _alloc: TAlloc,
}

// Java: public class RecycleAlloc
pub struct TRecycleAlloc {
    // Java field: ArrayList<RB_Node<K, V>> localPool
    pub(crate) localPool: Vec<*mut TRBNode>,
    // Java field: int LOCAL_CACHE_MAX
    // Java field: int SHARE_CACHE_MAX
}

// Java: public class Alloc
pub struct TAlloc {
    // Java field: ThreadLocal<RecycleAlloc<K, V>> alloc
    pub(crate) alloc: Mutex<Option<TRecycleAlloc>>,
}

// Java: public class Iterator
pub struct CowTreeMapIterator {
    // Java field: RB_Node<K, V> root
    pub(crate) root: *mut TRBNode,
    // Java field: RB_Node<K, V> curr
    pub(crate) curr: *mut TRBNode,
    pub(crate) nil: *mut TRBNode,
}

unsafe impl Send for TRBNode {}
unsafe impl Sync for TRBNode {}
unsafe impl Send for CopyOnWriteTreeMap {}
unsafe impl Sync for CopyOnWriteTreeMap {}
unsafe impl Send for CowTreeMapIterator {}
unsafe impl Sync for CowTreeMapIterator {}

// Java: public class RB_Node
impl TRBNode {
    // Java: RB_Node()
    pub fn new() -> Self {
        // Java: super();
        TRBNode { key: 0, value: None, left: std::ptr::null_mut(), right: std::ptr::null_mut(), isBlack: false, refCount: 0 }
    }

    // Java: K key()
    pub fn key(&self) -> K {
        // Java: return this.key;
        self.key
    }

    // Java: V value()
    pub fn value(&self) -> &Val {
        // Java: return this.value;
        &self.value
    }

    // Java: RB_Node<K, V> left()
    pub fn left(&self) -> *mut TRBNode {
        // Java: return this.left;
        self.left
    }

    // Java: RB_Node<K, V> right()
    pub fn right(&self) -> *mut TRBNode {
        // Java: return this.right;
        self.right
    }

    // Java: boolean isBlack()
    pub fn isBlack(&self) -> bool {
        // Java: return this.isBlack;
        self.isBlack
    }

    // Java: void setValue(V value)
    pub fn setValue(&mut self, value: Val) {
        // Java: this.value = value;
        self.value = value;
    }

    // Java: void setData(K key, V value, boolean isBlack)
    pub fn setData(&mut self, key: K, value: Val, isBlack: bool) {
        // Java: this.key = key;
        self.key = key;
        // Java: this.value = value;
        self.value = value;
        // Java: this.isBlack = isBlack;
        self.isBlack = isBlack;
    }

    // Java: void setBlack(boolean isBlack)
    pub fn setBlack(&mut self, isBlack: bool) {
        // Java: this.isBlack = isBlack;
        self.isBlack = isBlack;
    }

    // Java: int refCount()
    pub fn refCount(&self) -> i32 {
        // Java: return this.refCount;
        self.refCount
    }
}

// Java: public class RecycleAlloc
impl TRecycleAlloc {
    // Java: RecycleAlloc()
    pub fn new() -> Self {
        // Java: super();
        // Java: this.localPool = new ArrayList();
        TRecycleAlloc { localPool: Vec::new() }
    }

    // Java: RB_Node<K, V> alloc()
    pub fn alloc(&mut self) -> *mut TRBNode {
        // Java: if (this.localPool.isEmpty() && sSharePool.size() > 64)
        if self.localPool.is_empty() {
            let share_big = S_SHARE_POOL_T.lock().map(|s| s.len() > 64).unwrap_or(false);
            if share_big {
                // Java: synchronized(sSharePool)
                {
                    let mut share = S_SHARE_POOL_T.lock().unwrap();
                    // Java: for(int count = Math.min(sSharePool.size(), 64); count > 0; --count)
                    let count = share.len().min(64);
                    for _ in 0..count
                    {
                        // Java: this.localPool.add((RB_Node)sSharePool.remove(sSharePool.size() - 1));
                        if let Some(SendNodePtr(n)) = share.pop() { self.localPool.push(n); }
                    }
                }
            }
        }
        // Java: RB_Node<K, V> node = null;
        let node: *mut TRBNode;
        // Java: if (this.localPool.size() > 0)
        if !self.localPool.is_empty()
        {
            // Java: node = (RB_Node)this.localPool.remove(this.localPool.size() - 1);
            node = self.localPool.pop().unwrap();
        }
        // Java: else
        else
        {
            // Java: node = new RB_Node<K, V>();
            node = Box::into_raw(Box::new(TRBNode::new()));
        }
        // Java: return node;
        node
    }

    // Java: void free(RB_Node<K, V> node)
    pub fn free(&mut self, node: *mut TRBNode) {
        // Java: if (this.localPool.size() >= 128 && sSharePool.size() < 4096)
        {
            let share_small = S_SHARE_POOL_T.lock().map(|s| s.len() < SHARE_CACHE_MAX as usize).unwrap_or(false);
            if self.localPool.len() >= LOCAL_CACHE_MAX as usize && share_small {
                // Java: synchronized(sSharePool)
                {
                    let mut share = S_SHARE_POOL_T.lock().unwrap();
                    // Java: for(int count = 64; count > 0; --count)
                    for _ in 0..64
                    {
                        // Java: sSharePool.add((RB_Node)this.localPool.remove(this.localPool.size() - 1));
                        if let Some(n) = self.localPool.pop() { share.push(SendNodePtr(n)); }
                    }
                }
            }
        }
        // Java: if (this.localPool.size() < 128)
        if self.localPool.len() < LOCAL_CACHE_MAX as usize
        {
            // Java: this.localPool.add(node);
            self.localPool.push(node);
        }
    }
}

// Java: public class Alloc
impl TAlloc {
    // Java: Alloc()
    pub fn new() -> Self {
        // Java: super();
        // Java: this.alloc = new ThreadLocal();
        TAlloc { alloc: Mutex::new(None) }
    }

    // Java: RB_Node<K, V> alloc()
    pub fn alloc(&self) -> *mut TRBNode {
        // Java: RecycleAlloc<K, V> _alloc = (RecycleAlloc)this.alloc.get();
        let mut guard = self.alloc.lock().unwrap();
        // Java: if (null == _alloc)
        if guard.is_none()
        {
            // Java: _alloc = new RecycleAlloc<K, V>();
            // Java: this.alloc.set(_alloc);
            *guard = Some(TRecycleAlloc::new());
        }
        // Java: return _alloc.alloc();
        guard.as_mut().unwrap().alloc()
    }

    // Java: void free(RB_Node<K, V> node)
    pub fn free(&self, node: *mut TRBNode) {
        // Java: RecycleAlloc<K, V> _alloc = (RecycleAlloc)this.alloc.get();
        let mut guard = self.alloc.lock().unwrap();
        // Java: if (null != _alloc)
        if let Some(ref mut _alloc) = *guard
        {
            // Java: _alloc.free(node);
            _alloc.free(node);
        }
    }
}

// Java: public class CopyOnWriteTreeMap
impl CopyOnWriteTreeMap {
    // Java: CopyOnWriteTreeMap()
    pub fn new() -> Self {
        // Java: super();
        // Java: this.comparator = (Comparator<? super K>)(Comparator<?>)NATURAL_ORDER;
        let comparator: Box<dyn Fn(K, K) -> i32 + Send + Sync> = Box::new(|a: K, b: K| a.cmp(&b) as i32);
        // Java: this.NIL = new RB_Node<K, V>();
        let nil = Box::into_raw(Box::new(TRBNode::new()));
        unsafe {
            // Java: this._root = this.NIL;
            // Java: this._alloc = new Alloc<K, V>();
            // Java: this.NIL.left = this.NIL;
            (*nil).left = nil;
            // Java: this.NIL.right = this.NIL;
            (*nil).right = nil;
            // Java: this.NIL.isBlack = true;
            (*nil).isBlack = true;
        }
        CopyOnWriteTreeMap { comparator, NIL: nil, _rootLock: Mutex::new(()), _root: nil, _size: 0, _alloc: TAlloc::new() }
    }

    // Java: CopyOnWriteTreeMap(CopyOnWriteTreeMap<K, V> clone)
    pub fn new_from(clone: &mut CopyOnWriteTreeMap) -> Self {
        // Java: super();
        // Java: this.comparator = clone.comparator;
        let comparator: Box<dyn Fn(K, K) -> i32 + Send + Sync> = Box::new(|a: K, b: K| a.cmp(&b) as i32);
        // Java: this.NIL = clone.NIL;
        let nil = clone.NIL;
        let mut _root;
        let mut _size;
        // Java: synchronized(clone._rootLock)
        {
            let _lock = clone._rootLock.lock().unwrap();
            // Java: this._root = clone._root;
            _root = clone._root;
            // Java: if (this._root != this.NIL)
            if _root != nil
            {
                // Java: this.increaseRef(this._root);
                unsafe { (*_root).refCount += 1; }
            }
            // Java: this._size = clone._size;
            _size = clone._size;
        }
        // Java: this._alloc = clone._alloc;
        CopyOnWriteTreeMap { comparator, NIL: nil, _rootLock: Mutex::new(()), _root, _size, _alloc: TAlloc::new() }
    }

    // Java: void updateTree(RB_Node<K, V> root, int size)
    fn updateTree(&mut self, root: *mut TRBNode, size: i32) {
        // Java: RB_Node<K, V> oldRoot = null;
        let mut oldRoot: *mut TRBNode = std::ptr::null_mut();
        // Java: synchronized(this._rootLock)
        {
            let _lock = self._rootLock.lock().unwrap();
            // Java: if (this._root != root)
            if self._root != root
            {
                // Java: oldRoot = this._root;
                oldRoot = self._root;
                // Java: this._root = root;
                self._root = root;
            }
            // Java: this._size = size;
            self._size = size;
        }
        // Java: if (null != oldRoot)
        if !oldRoot.is_null() && oldRoot != self.NIL
        {
            // Java: this.decreaseRef(oldRoot);
            self.decreaseRef(oldRoot);
        }
    }

    // Java: private RB_Node<K, V> root()
    fn root(&self) -> *mut TRBNode {
        // Java: return this._root;
        self._root
    }

    // Java: private RB_Node<K, V> increaseRefRoot()
    fn increaseRefRoot(&self) -> *mut TRBNode {
        // Java: synchronized(this._rootLock)
        {
            let _lock = self._rootLock.lock().unwrap();
            // Java: if (this._root != this.NIL)
            if self._root != self.NIL
            {
                // Java: this.increaseRef(this._root);
                unsafe { (*self._root).refCount += 1; }
            }
            // Java: return this._root;
            self._root
        }
    }

    // Java: int size()
    pub fn size(&self) -> i32 {
        // Java: return this._size;
        self._size
    }

    // Java: private RB_Node<K, V> alloc()
    fn alloc(&self) -> *mut TRBNode {
        // Java: RB_Node<K, V> node = this._alloc.alloc();
        let node = self._alloc.alloc();
        unsafe {
            // Java: node.left = this.NIL;
            (*node).left = self.NIL;
            // Java: node.right = this.NIL;
            (*node).right = self.NIL;
            // Java: node.refCount = 0;
            (*node).refCount = 0;
            // Java: node.isBlack = false;
            (*node).isBlack = false;
        }
        // Java: return node;
        node
    }

    // Java: void _free(RB_Node<K, V> node)
    fn _free(&self, node: *mut TRBNode) {
        unsafe {
            // Java: node.left = null;
            (*node).left = std::ptr::null_mut();
            // Java: node.right = null;
            (*node).right = std::ptr::null_mut();
            // Java: node.refCount = 0;
            (*node).refCount = 0;
            // Java: node.isBlack = false;
            (*node).isBlack = false;
        }
        // Java: this._alloc.free(node);
        self._alloc.free(node);
    }

    // Java: void setLeft(RB_Node<K, V> parent, RB_Node<K, V> left)
    fn setLeft(&self, parent: *mut TRBNode, left: *mut TRBNode) {
        unsafe {
            // Java: RB_Node<K, V> oldLeft = parent.left;
            let oldLeft = (*parent).left;
            // Java: parent.left = left;
            (*parent).left = left;
            // Java: if (left != this.NIL)
            if left != self.NIL
            {
                // Java: this.increaseRef(left);
                self.increaseRef(left);
            }
            // Java: if (oldLeft != this.NIL)
            if oldLeft != self.NIL
            {
                // Java: this.decreaseRef(oldLeft);
                self.decreaseRef(oldLeft);
            }
        }
    }

    // Java: void setRight(RB_Node<K, V> parent, RB_Node<K, V> right)
    fn setRight(&self, parent: *mut TRBNode, right: *mut TRBNode) {
        unsafe {
            // Java: RB_Node<K, V> oldRight = parent.right;
            let oldRight = (*parent).right;
            // Java: parent.right = right;
            (*parent).right = right;
            // Java: if (right != this.NIL)
            if right != self.NIL
            {
                // Java: this.increaseRef(right);
                self.increaseRef(right);
            }
            // Java: if (oldRight != this.NIL)
            if oldRight != self.NIL
            {
                // Java: this.decreaseRef(oldRight);
                self.decreaseRef(oldRight);
            }
        }
    }

    // Java: void increaseRef(RB_Node<K, V> node)
    fn increaseRef(&self, node: *mut TRBNode) {
        // Java: synchronized(node)
        unsafe {
            // Java: ++node.refCount;
            (*node).refCount += 1;
        }
    }

    // Java: void decreaseRef(RB_Node<K, V> node)
    fn decreaseRef(&self, node: *mut TRBNode) {
        // Java: int ref;
        let r: i32;
        // Java: synchronized(node)
        unsafe {
            // Java: ref = --node.refCount;
            (*node).refCount -= 1;
            r = (*node).refCount;
        }
        // Java: if (0 == ref)
        if 0 == r
        {
            unsafe {
                // Java: if (node.left != this.NIL)
                if (*node).left != self.NIL
                {
                    // Java: this.decreaseRef(node.left);
                    self.decreaseRef((*node).left);
                }
                // Java: if (node.right != this.NIL)
                if (*node).right != self.NIL
                {
                    // Java: this.decreaseRef(node.right);
                    self.decreaseRef((*node).right);
                }
            }
            // Java: this._free(node);
            self._free(node);
        }
    }

    // Java: void rollLeft(RB_Node<K, V> parent, RB_Node<K, V> node)
    fn rollLeft_parent(&self, parent: *mut TRBNode, node: *mut TRBNode) {
        // Java: RB_Node<K, V> c = this.rollLeft(node);
        let c = self.rollLeft(node);
        unsafe {
            // Java: if (parent.left() == node)
            if (*parent).left == node
            {
                // Java: parent.left = c;
                (*parent).left = c;
            }
            // Java: else
            else
            {
                // Java: parent.right = c;
                (*parent).right = c;
            }
        }
    }

    // Java: private RB_Node<K, V> rollLeft(RB_Node<K, V> node)
    fn rollLeft(&self, node: *mut TRBNode) -> *mut TRBNode {
        unsafe {
            // Java: RB_Node<K, V> right = node.right();
            let right = (*node).right;
            // Java: node.right = right.left;
            (*node).right = (*right).left;
            // Java: right.left = node;
            (*right).left = node;
            // Java: return right;
            right
        }
    }

    // Java: void rollRight(RB_Node<K, V> parent, RB_Node<K, V> node)
    fn rollRight_parent(&self, parent: *mut TRBNode, node: *mut TRBNode) {
        // Java: RB_Node<K, V> c = this.rollRight(node);
        let c = self.rollRight(node);
        unsafe {
            // Java: if (parent.left() == node)
            if (*parent).left == node
            {
                // Java: parent.left = c;
                (*parent).left = c;
            }
            // Java: else
            else
            {
                // Java: parent.right = c;
                (*parent).right = c;
            }
        }
    }

    // Java: private RB_Node<K, V> rollRight(RB_Node<K, V> node)
    fn rollRight(&self, node: *mut TRBNode) -> *mut TRBNode {
        unsafe {
            // Java: RB_Node<K, V> left = node.left();
            let left = (*node).left;
            // Java: node.left = left.right;
            (*node).left = (*left).right;
            // Java: left.right = node;
            (*left).right = node;
            // Java: return left;
            left
        }
    }

    // Java: private RB_Node<K, V> first(RB_Node<K, V> root)
    fn first(&self, root: *mut TRBNode) -> *mut TRBNode {
        // Java: RB_Node<K, V> node = null;
        let mut node: *mut TRBNode = std::ptr::null_mut();
        // Java: for(RB_Node<K, V> curr = root; curr != this.NIL; curr = curr.left())
        let mut curr = root;
        while curr != self.NIL
        {
            // Java: node = curr;
            node = curr;
            unsafe { curr = (*curr).left; }
        }
        // Java: return node;
        node
    }

    // Java: private RB_Node<K, V> last(RB_Node<K, V> root)
    fn last(&self, root: *mut TRBNode) -> *mut TRBNode {
        // Java: RB_Node<K, V> node = null;
        let mut node: *mut TRBNode = std::ptr::null_mut();
        // Java: for(RB_Node<K, V> curr = root; curr != this.NIL; curr = curr.right())
        let mut curr = root;
        while curr != self.NIL
        {
            // Java: node = curr;
            node = curr;
            unsafe { curr = (*curr).right; }
        }
        // Java: return node;
        node
    }

    // Java: private RB_Node<K, V> next(RB_Node<K, V> root, RB_Node<K, V> node)
    fn next(&self, root: *mut TRBNode, node: *mut TRBNode) -> *mut TRBNode {
        unsafe {
            // Java: if (node.right() != this.NIL)
            if (*node).right != self.NIL
            {
                // Java: RB_Node<K, V> next = node.right();
                let mut next = (*node).right;
                // Java: for(RB_Node<K, V> curr = next; curr != this.NIL; curr = curr.left())
                let mut curr = next;
                while curr != self.NIL
                {
                    // Java: next = curr;
                    next = curr;
                    curr = (*curr).left;
                }
                // Java: return next;
                return next;
            }
            // Java: else
            {
                // Java: RB_Node<K, V> next = null;
                let mut next: *mut TRBNode = std::ptr::null_mut();
                // Java: RB_Node<K, V> curr = root;
                let mut curr = root;
                // Java: while(curr != this.NIL)
                while curr != self.NIL
                {
                    // Java: int v = this.comparator.compare(curr.key(), node.key());
                    let v = (self.comparator)((*curr).key, (*node).key);
                    // Java: if (0 == v)
                    if 0 == v
                    {
                        // Java: break;
                        break;
                    }
                    // Java: if (v < 0)
                    if v < 0
                    {
                        // Java: curr = curr.right();
                        curr = (*curr).right;
                    }
                    // Java: else
                    else
                    {
                        // Java: next = curr;
                        next = curr;
                        // Java: curr = curr.left();
                        curr = (*curr).left;
                    }
                }
                // Java: return next;
                return next;
            }
        }
    }

    // Java: private RB_Node<K, V> prev(RB_Node<K, V> root, RB_Node<K, V> node)
    fn prev(&self, root: *mut TRBNode, node: *mut TRBNode) -> *mut TRBNode {
        unsafe {
            // Java: if (node.left() != this.NIL)
            if (*node).left != self.NIL
            {
                // Java: RB_Node<K, V> prev = node.left();
                let mut prev = (*node).left;
                // Java: for(RB_Node<K, V> curr = prev; curr != this.NIL; curr = curr.right())
                let mut curr = prev;
                while curr != self.NIL
                {
                    // Java: prev = curr;
                    prev = curr;
                    curr = (*curr).right;
                }
                // Java: return prev;
                return prev;
            }
            // Java: else
            {
                // Java: RB_Node<K, V> prev = null;
                let mut prev: *mut TRBNode = std::ptr::null_mut();
                // Java: RB_Node<K, V> curr = root;
                let mut curr = root;
                // Java: while(curr != this.NIL)
                while curr != self.NIL
                {
                    // Java: int v = this.comparator.compare(curr.key(), node.key());
                    let v = (self.comparator)((*curr).key, (*node).key);
                    // Java: if (0 == v)
                    if 0 == v
                    {
                        // Java: break;
                        break;
                    }
                    // Java: if (v < 0)
                    if v < 0
                    {
                        // Java: prev = curr;
                        prev = curr;
                        // Java: curr = curr.right();
                        curr = (*curr).right;
                    }
                    // Java: else
                    else
                    {
                        // Java: curr = curr.left();
                        curr = (*curr).left;
                    }
                }
                // Java: return prev;
                return prev;
            }
        }
    }

    // Java: private RB_Node<K, V> parent(RB_Node<K, V> root, K key)
    fn parent(&self, root: *mut TRBNode, key: K) -> *mut TRBNode {
        // Java: RB_Node<K, V> parent = this.NIL;
        let mut parent = self.NIL;
        // Java: int v;
        // Java: for(RB_Node<K, V> curr = root; curr != this.NIL; curr = v < 0 ? curr.right() : curr.left())
        let mut curr = root;
        unsafe {
            while curr != self.NIL
            {
                // Java: v = this.comparator.compare(curr.key(), key);
                let v = (self.comparator)((*curr).key, key);
                // Java: if (0 == v)
                if 0 == v
                {
                    // Java: break;
                    break;
                }
                // Java: parent = curr;
                parent = curr;
                curr = if v < 0 { (*curr).right } else { (*curr).left };
            }
        }
        // Java: return parent;
        parent
    }

    // Java: V get(K key)
    pub fn get(&self, key: K) -> Val {
        // Java: RB_Node<K, V> root = this.increaseRefRoot();
        let root = self.increaseRefRoot();
        // Java: RB_Node<K, V> node = this.get(root, key);
        let node = self.get_node(root, key);
        // Java: V value = node.value();
        let value = if node.is_null() || node == self.NIL { None } else { unsafe { (*node).value.clone() } };
        // Java: if (root != this.NIL)
        if root != self.NIL
        {
            // Java: this.decreaseRef(root);
            self.decreaseRef(root);
        }
        // Java: return value;
        value
    }

    // Java: private RB_Node<K, V> get(RB_Node<K, V> root, K key)
    fn get_node(&self, root: *mut TRBNode, key: K) -> *mut TRBNode {
        // Java: RB_Node<K, V> curr;
        // Java: int v;
        // Java: for(curr = root; curr != this.NIL; curr = v < 0 ? curr.right() : curr.left())
        let mut curr = root;
        unsafe {
            while curr != self.NIL
            {
                // Java: v = this.comparator.compare(curr.key(), key);
                let v = (self.comparator)((*curr).key, key);
                // Java: if (0 == v)
                if 0 == v
                {
                    // Java: break;
                    break;
                }
                curr = if v < 0 { (*curr).right } else { (*curr).left };
            }
        }
        // Java: return curr;
        curr
    }

    // Java: private RB_Node<K, V> lowerBound(RB_Node<K, V> root, K key)
    fn lowerBound_node(&self, root: *mut TRBNode, key: K) -> *mut TRBNode {
        // Java: RB_Node<K, V> parent = this.NIL;
        let mut parent = self.NIL;
        // Java: RB_Node<K, V> curr;
        // Java: int v;
        let mut curr = root;
        // Java: for(curr = root; curr != this.NIL; curr = v < 0 ? curr.right() : curr.left())
        unsafe {
            while curr != self.NIL
            {
                // Java: v = this.comparator.compare(curr.key(), key);
                let v = (self.comparator)((*curr).key, key);
                // Java: if (0 == v)
                if 0 == v
                {
                    // Java: break;
                    break;
                }
                // Java: parent = curr;
                parent = curr;
                curr = if v < 0 { (*curr).right } else { (*curr).left };
            }
        }
        // Java: if (this.NIL != curr)
        if self.NIL != curr
        {
            // Java: return curr;
            return curr;
        }
        // Java: else if (this.NIL == parent)
        else if self.NIL == parent
        {
            // Java: return null;
            return std::ptr::null_mut();
        }
        // Java: else
        {
            // Java: return this.comparator.compare(parent.key(), key) > 0 ? parent : this.next(root, parent);
            unsafe { if (self.comparator)((*parent).key, key) > 0 { parent } else { self.next(root, parent) } }
        }
    }

    // Java: private RB_Node<K, V> upperBound(RB_Node<K, V> root, K key)
    fn upperBound_node(&self, root: *mut TRBNode, key: K) -> *mut TRBNode {
        // Java: RB_Node<K, V> parent = this.NIL;
        let mut parent = self.NIL;
        // Java: RB_Node<K, V> curr;
        // Java: int v;
        let mut curr = root;
        // Java: for(curr = root; curr != this.NIL; curr = v < 0 ? curr.right() : curr.left())
        unsafe {
            while curr != self.NIL
            {
                // Java: v = this.comparator.compare(curr.key(), key);
                let v = (self.comparator)((*curr).key, key);
                // Java: if (0 == v)
                if 0 == v
                {
                    // Java: break;
                    break;
                }
                // Java: parent = curr;
                parent = curr;
                curr = if v < 0 { (*curr).right } else { (*curr).left };
            }
        }
        // Java: if (this.NIL != curr)
        if self.NIL != curr
        {
            // Java: return this.next(root, curr);
            return self.next(root, curr);
        }
        // Java: else if (this.NIL == parent)
        else if self.NIL == parent
        {
            // Java: return null;
            return std::ptr::null_mut();
        }
        // Java: else
        {
            // Java: return this.comparator.compare(parent.key(), key) > 0 ? parent : this.next(root, parent);
            unsafe { if (self.comparator)((*parent).key, key) > 0 { parent } else { self.next(root, parent) } }
        }
    }

    // Java: private RB_Node<K, V> copyNode(RB_Node<K, V> copyNode)
    fn copyNode_single(&self, cn: *mut TRBNode) -> *mut TRBNode {
        // Java: RB_Node<K, V> _node = this.alloc();
        let _node = self.alloc();
        unsafe {
            // Java: _node.setData(copyNode.key(), copyNode.value(), copyNode.isBlack());
            (*_node).setData((*cn).key, (*cn).value.clone(), (*cn).isBlack);
        }
        // Java: this.setLeft(_node, copyNode.left());
        self.setLeft(_node, unsafe { (*cn).left });
        // Java: this.setRight(_node, copyNode.right());
        self.setRight(_node, unsafe { (*cn).right });
        // Java: return _node;
        _node
    }

    // Java: private RB_Node<K, V> copyNode(RB_Node<K, V> parent, RB_Node<K, V> copyNode)
    fn copyNode_with_parent(&self, parent: *mut TRBNode, cn: *mut TRBNode) -> *mut TRBNode {
        // Java: RB_Node<K, V> _node = this.alloc();
        let _node = self.alloc();
        unsafe {
            // Java: _node.setData(copyNode.key(), copyNode.value(), copyNode.isBlack());
            (*_node).setData((*cn).key, (*cn).value.clone(), (*cn).isBlack);
        }
        // Java: this.setLeft(_node, copyNode.left());
        self.setLeft(_node, unsafe { (*cn).left });
        // Java: this.setRight(_node, copyNode.right());
        self.setRight(_node, unsafe { (*cn).right });
        unsafe {
            // Java: if (copyNode == parent.left())
            if cn == (*parent).left
            {
                // Java: this.setLeft(parent, _node);
                self.setLeft(parent, _node);
            }
            // Java: else
            else
            {
                // Java: this.setRight(parent, _node);
                self.setRight(parent, _node);
            }
        }
        // Java: return _node;
        _node
    }

    // Java: public CopyOnWriteTreeMap<K, V> clone()
    pub fn clone(&mut self) -> CopyOnWriteTreeMap {
        // Java: return new CopyOnWriteTreeMap<K, V>(this);
        CopyOnWriteTreeMap::new_from(self)
    }

    // Java: V insert(K key, V value)
    pub fn insert(&mut self, key: K, value: Val) -> Val {
        let nil = self.NIL;
        // Java: RB_Node<K, V> oldRoot = this.root();
        let oldRoot = self.root();
        // Java: RB_Node<K, V> root = this.NIL;
        let mut root = nil;
        // Java: RB_Node<K, V> parent = this.NIL;
        let mut parent = nil;
        // Java: int v;
        // Java: for(RB_Node<K, V> curr = oldRoot; curr != this.NIL; curr = v < 0 ? curr.right() : curr.left())
        let mut curr = oldRoot;
        unsafe {
            while curr != nil
            {
                // Java: RB_Node<K, V> node;
                let node: *mut TRBNode;
                // Java: if (this.NIL == parent)
                if nil == parent
                {
                    // Java: node = this.copyNode(curr);
                    node = self.copyNode_single(curr);
                    // Java: root = node;
                    root = node;
                    // Java: this.increaseRef(node);
                    self.increaseRef(node);
                }
                // Java: else
                else
                {
                    // Java: node = this.copyNode(parent, curr);
                    node = self.copyNode_with_parent(parent, curr);
                }
                // Java: v = this.comparator.compare(curr.key(), key);
                let v = (self.comparator)((*curr).key, key);
                // Java: if (0 == v)
                if 0 == v
                {
                    // Java: break;
                    break;
                }
                // Java: parent = node;
                parent = node;
                curr = if v < 0 { (*curr).right } else { (*curr).left };
            }
        }
        // Java: V oldValue = null;
        let mut oldValue: Val = None;
        // Java: RB_Node<K, V> node = null;
        let mut node: *mut TRBNode = std::ptr::null_mut();
        unsafe {
            // Java: if (this.NIL == parent)
            if nil == parent
            {
                // Java: if (this.NIL == root)
                if nil == root
                {
                    // Java: node = this.alloc();
                    node = self.alloc();
                    // Java: node.setData(key, value, true);
                    (*node).setData(key, value.clone(), true);
                    // Java: this.increaseRef(node);
                    self.increaseRef(node);
                    // Java: root = node;
                    root = node;
                }
                // Java: else
                else
                {
                    // Java: oldValue = root.value();
                    oldValue = (*root).value.clone();
                    // Java: root.setValue(value);
                    (*root).setValue(value.clone());
                }
            }
            // Java: else if (this.comparator.compare(parent.key(), key) < 0)
            else if (self.comparator)((*parent).key, key) < 0
            {
                // Java: if (this.NIL == parent.right())
                if nil == (*parent).right
                {
                    // Java: node = this.alloc();
                    node = self.alloc();
                    // Java: node.setData(key, value, false);
                    (*node).setData(key, value.clone(), false);
                    // Java: this.setRight(parent, node);
                    self.setRight(parent, node);
                }
                // Java: else
                else
                {
                    // Java: oldValue = parent.right().value();
                    oldValue = (*(*parent).right).value.clone();
                    // Java: parent.right().setValue(value);
                    (*(*parent).right).setValue(value.clone());
                }
            }
            // Java: else if (this.NIL == parent.left())
            else if nil == (*parent).left
            {
                // Java: node = this.alloc();
                node = self.alloc();
                // Java: node.setData(key, value, false);
                (*node).setData(key, value.clone(), false);
                // Java: this.setLeft(parent, node);
                self.setLeft(parent, node);
            }
            // Java: else
            else
            {
                // Java: oldValue = parent.left().value();
                oldValue = (*(*parent).left).value.clone();
                // Java: parent.left().setValue(value);
                (*(*parent).left).setValue(value.clone());
            }
        }
        // Java: if (null != node && this.NIL != parent)
        if !node.is_null() && nil != parent
        {
            // Java: root = this.insertFixup(root, parent, node);
            root = self.insertFixup(root, parent, node);
            // Java: this.updateTree(root, this.size() + 1);
            let s = self.size();
            self.updateTree(root, s + 1);
            // Java: return oldValue;
            return oldValue;
        }
        // Java: else
        else
        {
            // Java: this.updateTree(root, this.size() + 1);
            let s = self.size();
            self.updateTree(root, s + 1);
            // Java: return oldValue;
            return oldValue;
        }
    }

    // Java: private RB_Node<K, V> insertFixup(RB_Node<K, V> root, RB_Node<K, V> parent, RB_Node<K, V> node)
    fn insertFixup(&self, mut root: *mut TRBNode, mut parent: *mut TRBNode, mut node: *mut TRBNode) -> *mut TRBNode {
        let nil = self.NIL;
        // Java: while(true)
        loop
        {
            unsafe {
                // Java: if (!parent.isBlack())
                if !(*parent).isBlack
                {
                    // Java: RB_Node<K, V> pp = this.parent(root, parent.key());
                    let mut pp = self.parent(root, (*parent).key);
                    // Java: if (this.NIL == pp)
                    if nil == pp
                    {
                        // Java: parent.setBlack(true);
                        (*parent).isBlack = true;
                    }
                    // Java: else if (parent == pp.left())
                    else if parent == (*pp).left
                    {
                        // Java: RB_Node<K, V> ppright = pp.right();
                        let mut ppright = (*pp).right;
                        // Java: if (!ppright.isBlack())
                        if !(*ppright).isBlack
                        {
                            // Java: ppright = this.copyNode(pp, ppright);
                            ppright = self.copyNode_with_parent(pp, ppright);
                            // Java: ppright.setBlack(true);
                            (*ppright).isBlack = true;
                            // Java: parent.setBlack(true);
                            (*parent).isBlack = true;
                            // Java: pp.setBlack(false);
                            (*pp).isBlack = false;
                            // Java: node = pp;
                            node = pp;
                            // Java: parent = this.parent(root, pp.key());
                            parent = self.parent(root, (*pp).key);
                            // Java: if (this.NIL != parent)
                            if nil != parent
                            {
                                // Java: continue;
                                continue;
                            }
                            // Java: pp.setBlack(true);
                            (*pp).isBlack = true;
                            // Java: root = pp;
                            root = pp;
                        }
                        // Java: else
                        else
                        {
                            // Java: if (node == parent.right())
                            if node == (*parent).right
                            {
                                // Java: this.rollLeft(pp, parent);
                                self.rollLeft_parent(pp, parent);
                                // Java: parent = node;
                                parent = node;
                            }
                            // Java: parent.setBlack(true);
                            (*parent).isBlack = true;
                            // Java: pp.setBlack(false);
                            (*pp).isBlack = false;
                            // Java: RB_Node<K, V> ppp = this.parent(root, pp.key());
                            let ppp = self.parent(root, (*pp).key);
                            // Java: if (this.NIL == ppp)
                            if nil == ppp
                            {
                                // Java: this.rollRight(pp);
                                self.rollRight(pp);
                                // Java: root = parent;
                                root = parent;
                            }
                            // Java: else
                            else
                            {
                                // Java: this.rollRight(ppp, pp);
                                self.rollRight_parent(ppp, pp);
                            }
                        }
                    }
                    // Java: else
                    else
                    {
                        // Java: RB_Node<K, V> ppleft = pp.left();
                        let mut ppleft = (*pp).left;
                        // Java: if (!ppleft.isBlack())
                        if !(*ppleft).isBlack
                        {
                            // Java: ppleft = this.copyNode(pp, ppleft);
                            ppleft = self.copyNode_with_parent(pp, ppleft);
                            // Java: ppleft.setBlack(true);
                            (*ppleft).isBlack = true;
                            // Java: parent.setBlack(true);
                            (*parent).isBlack = true;
                            // Java: pp.setBlack(false);
                            (*pp).isBlack = false;
                            // Java: node = pp;
                            node = pp;
                            // Java: parent = this.parent(root, pp.key());
                            parent = self.parent(root, (*pp).key);
                            // Java: if (this.NIL != parent)
                            if nil != parent
                            {
                                // Java: continue;
                                continue;
                            }
                            // Java: pp.setBlack(true);
                            (*pp).isBlack = true;
                            // Java: root = pp;
                            root = pp;
                        }
                        // Java: else
                        else
                        {
                            // Java: if (node == parent.left())
                            if node == (*parent).left
                            {
                                // Java: this.rollRight(pp, parent);
                                self.rollRight_parent(pp, parent);
                                // Java: parent = node;
                                parent = node;
                            }
                            // Java: parent.setBlack(true);
                            (*parent).isBlack = true;
                            // Java: pp.setBlack(false);
                            (*pp).isBlack = false;
                            // Java: RB_Node<K, V> ppp = this.parent(root, pp.key());
                            let ppp = self.parent(root, (*pp).key);
                            // Java: if (this.NIL == ppp)
                            if nil == ppp
                            {
                                // Java: this.rollLeft(pp);
                                self.rollLeft(pp);
                                // Java: root = parent;
                                root = parent;
                            }
                            // Java: else
                            else
                            {
                                // Java: this.rollLeft(ppp, pp);
                                self.rollLeft_parent(ppp, pp);
                            }
                        }
                    }
                }
                // Java: return root;
                return root;
            }
        }
    }

    // Java: V delete(K key)
    pub fn delete(&mut self, key: K) -> Val {
        let nil = self.NIL;
        // Java: RB_Node<K, V> root = this.root();
        let mut root = self.root();
        // Java: RB_Node<K, V> curr = this.get(root, key);
        let mut curr = self.get_node(root, key);
        // Java: if (this.NIL == curr)
        if nil == curr
        {
            // Java: return null;
            return None;
        }
        // Java: else
        unsafe {
            // Java: RB_Node<K, V> y;
            let mut y: *mut TRBNode;
            // Java: if (curr.left() != this.NIL && curr.right() != this.NIL)
            if (*curr).left != nil && (*curr).right != nil
            {
                // Java: y = this.next(root, curr);
                y = self.next(root, curr);
            }
            // Java: else
            else
            {
                // Java: y = curr;
                y = curr;
            }
            // Java: RB_Node<K, V> x;
            let x: *mut TRBNode;
            // Java: if (this.NIL != y.left())
            if nil != (*y).left
            {
                // Java: x = y.left();
                x = (*y).left;
            }
            // Java: else
            else
            {
                // Java: x = y.right();
                x = (*y).right;
            }
            // Java: RB_Node<K, V> py = this.parent(root, y.key());
            let mut py = self.parent(root, (*y).key);
            // Java: if (this.NIL == py)
            if nil == py
            {
                // Java: RB_Node<K, V> node = this.alloc();
                let node = self.alloc();
                // Java: node.setData(x.key(), x.value(), true);
                (*node).setData((*x).key, (*x).value.clone(), true);
                // Java: this.increaseRef(node);
                self.increaseRef(node);
                // Java: root = node;
                root = node;
            }
            // Java: else
            else
            {
                // Java: RB_Node<K, V> _curr = root;
                let mut _curr = root;
                // Java: root = this.NIL;
                root = nil;
                // Java: int v;
                // Java: for(RB_Node<K, V> parent = this.NIL; _curr != this.NIL; _curr = v < 0 ? _curr.right() : _curr.left())
                let mut parent_d = nil;
                while _curr != nil
                {
                    // Java: RB_Node<K, V> node;
                    let node: *mut TRBNode;
                    // Java: if (this.NIL == parent)
                    if nil == parent_d
                    {
                        // Java: node = this.copyNode(_curr);
                        node = self.copyNode_single(_curr);
                        // Java: root = node;
                        root = node;
                        // Java: this.increaseRef(node);
                        self.increaseRef(node);
                    }
                    // Java: else
                    else
                    {
                        // Java: node = this.copyNode(parent, _curr);
                        node = self.copyNode_with_parent(parent_d, _curr);
                    }
                    // Java: v = this.comparator.compare(_curr.key(), py.key());
                    let v = (self.comparator)((*_curr).key, (*py).key);
                    // Java: if (0 == v)
                    if 0 == v
                    {
                        // Java: py = node;
                        py = node;
                        // Java: break;
                        break;
                    }
                    // Java: parent = node;
                    parent_d = node;
                    _curr = if v < 0 { (*_curr).right } else { (*_curr).left };
                }
                // Java: if (y != curr)
                if y != curr
                {
                    // Java: curr = this.get(root, key);
                    curr = self.get_node(root, key);
                }
                // Java: if (y == py.left())
                if y == (*py).left
                {
                    // Java: this.setLeft(py, x);
                    self.setLeft(py, x);
                }
                // Java: else
                else
                {
                    // Java: this.setRight(py, x);
                    self.setRight(py, x);
                }
            }
            // Java: if (y != curr)
            if y != curr
            {
                // Java: curr.setData(y.key(), y.value(), curr.isBlack());
                (*curr).setData((*y).key, (*y).value.clone(), (*curr).isBlack);
            }
            // Java: if (y.isBlack() && this.NIL != py)
            if (*y).isBlack && nil != py
            {
                // Java: root = this.deleteFixup(root, py, x);
                root = self.deleteFixup(root, py, x);
            }
            // Java: V oldValue = y.value();
            let oldValue = (*y).value.clone();
            // Java: this.updateTree(root, this.size() - 1);
            let s = self.size();
            self.updateTree(root, s - 1);
            // Java: return oldValue;
            return oldValue;
        }
    }

    // Java: boolean _isParent(RB_Node<K, V> parent, RB_Node<K, V> node)
    fn _isParent(&self, parent: *mut TRBNode, node: *mut TRBNode) -> bool {
        // Java: return parent.left() == node || parent.right() == node;
        unsafe { (*parent).left == node || (*parent).right == node }
    }

    // Java: private RB_Node<K, V> deleteFixup(RB_Node<K, V> root, RB_Node<K, V> parent, RB_Node<K, V> node)
    fn deleteFixup(&self, mut root: *mut TRBNode, mut parent: *mut TRBNode, mut node: *mut TRBNode) -> *mut TRBNode {
        let nil = self.NIL;
        // Java: while(true)
        loop
        {
            unsafe {
                // Java: if (node != root && node.isBlack())
                if node != root && (*node).isBlack
                {
                    // Java: if (node == parent.left())
                    if node == (*parent).left
                    {
                        // Java: RB_Node<K, V> pright = parent.right();
                        let mut pright = (*parent).right;
                        // Java: RB_Node<K, V> pp = this.parent(root, parent.key());
                        let mut pp = self.parent(root, (*parent).key);
                        // Java: if (!pright.isBlack())
                        if !(*pright).isBlack
                        {
                            // Java: pright = this.copyNode(parent, pright);
                            pright = self.copyNode_with_parent(parent, pright);
                            // Java: pright.setBlack(true);
                            (*pright).isBlack = true;
                            // Java: parent.setBlack(false);
                            (*parent).isBlack = false;
                            // Java: if (pp == this.NIL)
                            if pp == nil
                            {
                                // Java: root = this.rollLeft(parent);
                                root = self.rollLeft(parent);
                            }
                            // Java: else
                            else
                            {
                                // Java: this.rollLeft(pp, parent);
                                self.rollLeft_parent(pp, parent);
                            }
                            // Java: pp = pright;
                            pp = pright;
                            // Java: pright = parent.right();
                            pright = (*parent).right;
                        }
                        // Java: if (pright.left().isBlack() && pright.right().isBlack())
                        if (*(*pright).left).isBlack && (*(*pright).right).isBlack
                        {
                            // Java: pright = this.copyNode(parent, pright);
                            pright = self.copyNode_with_parent(parent, pright);
                            // Java: pright.setBlack(false);
                            (*pright).isBlack = false;
                            // Java: node = parent;
                            node = parent;
                            // Java: parent = pp;
                            parent = pp;
                            // Java: continue;
                            continue;
                        }
                        // Java: if (pright.right().isBlack())
                        if (*(*pright).right).isBlack
                        {
                            // Java: pright = this.copyNode(parent, pright);
                            pright = self.copyNode_with_parent(parent, pright);
                            // Java: this.copyNode(pright, pright.left());
                            self.copyNode_with_parent(pright, (*pright).left);
                            // Java: pright.left().setBlack(true);
                            (*(*pright).left).isBlack = true;
                            // Java: pright.setBlack(false);
                            (*pright).isBlack = false;
                            // Java: this.rollRight(parent, pright);
                            self.rollRight_parent(parent, pright);
                            // Java: pright = parent.right();
                            pright = (*parent).right;
                        }
                        // Java: if (pright.refCount() > 1)
                        if (*pright).refCount > 1
                        {
                            // Java: pright = this.copyNode(parent, pright);
                            pright = self.copyNode_with_parent(parent, pright);
                        }
                        // Java: pright.setBlack(parent.isBlack());
                        (*pright).isBlack = (*parent).isBlack;
                        // Java: parent.setBlack(true);
                        (*parent).isBlack = true;
                        // Java: if (pright.right().refCount() > 1)
                        if (*(*pright).right).refCount > 1
                        {
                            // Java: this.copyNode(pright, pright.right());
                            self.copyNode_with_parent(pright, (*pright).right);
                        }
                        // Java: pright.right().setBlack(true);
                        (*(*pright).right).isBlack = true;
                        // Java: if (pp == this.NIL)
                        if pp == nil
                        {
                            // Java: root = this.rollLeft(parent);
                            root = self.rollLeft(parent);
                        }
                        // Java: else
                        else
                        {
                            // Java: this.rollLeft(pp, parent);
                            self.rollLeft_parent(pp, parent);
                        }
                        // Java: parent = this.NIL;
                        parent = nil;
                        // Java: node = root;
                        node = root;
                    }
                    // Java: else
                    else
                    {
                        // Java: RB_Node<K, V> pleft = parent.left();
                        let mut pleft = (*parent).left;
                        // Java: RB_Node<K, V> pp = this.parent(root, parent.key());
                        let mut pp = self.parent(root, (*parent).key);
                        // Java: if (!pleft.isBlack())
                        if !(*pleft).isBlack
                        {
                            // Java: pleft = this.copyNode(parent, pleft);
                            pleft = self.copyNode_with_parent(parent, pleft);
                            // Java: pleft.setBlack(true);
                            (*pleft).isBlack = true;
                            // Java: parent.setBlack(false);
                            (*parent).isBlack = false;
                            // Java: if (pp == this.NIL)
                            if pp == nil
                            {
                                // Java: root = this.rollRight(parent);
                                root = self.rollRight(parent);
                            }
                            // Java: else
                            else
                            {
                                // Java: this.rollRight(pp, parent);
                                self.rollRight_parent(pp, parent);
                            }
                            // Java: pp = pleft;
                            pp = pleft;
                            // Java: pleft = parent.left();
                            pleft = (*parent).left;
                        }
                        // Java: if (pleft.left().isBlack() && pleft.right().isBlack())
                        if (*(*pleft).left).isBlack && (*(*pleft).right).isBlack
                        {
                            // Java: pleft = this.copyNode(parent, pleft);
                            pleft = self.copyNode_with_parent(parent, pleft);
                            // Java: pleft.setBlack(false);
                            (*pleft).isBlack = false;
                            // Java: node = parent;
                            node = parent;
                            // Java: parent = pp;
                            parent = pp;
                            // Java: continue;
                            continue;
                        }
                        // Java: if (pleft.left().isBlack())
                        if (*(*pleft).left).isBlack
                        {
                            // Java: pleft = this.copyNode(parent, pleft);
                            pleft = self.copyNode_with_parent(parent, pleft);
                            // Java: this.copyNode(pleft, pleft.right());
                            self.copyNode_with_parent(pleft, (*pleft).right);
                            // Java: pleft.right().setBlack(true);
                            (*(*pleft).right).isBlack = true;
                            // Java: pleft.setBlack(false);
                            (*pleft).isBlack = false;
                            // Java: this.rollLeft(parent, pleft);
                            self.rollLeft_parent(parent, pleft);
                            // Java: pleft = parent.right();
                            pleft = (*parent).right;
                        }
                        // Java: if (pleft.refCount() > 1)
                        if (*pleft).refCount > 1
                        {
                            // Java: pleft = this.copyNode(parent, pleft);
                            pleft = self.copyNode_with_parent(parent, pleft);
                        }
                        // Java: pleft.setBlack(parent.isBlack());
                        (*pleft).isBlack = (*parent).isBlack;
                        // Java: parent.setBlack(true);
                        (*parent).isBlack = true;
                        // Java: if (pleft.left().refCount() > 1)
                        if (*(*pleft).left).refCount > 1
                        {
                            // Java: this.copyNode(pleft, pleft.left());
                            self.copyNode_with_parent(pleft, (*pleft).left);
                        }
                        // Java: pleft.left().setBlack(true);
                        (*(*pleft).left).isBlack = true;
                        // Java: if (pp == this.NIL)
                        if pp == nil
                        {
                            // Java: root = this.rollRight(parent);
                            root = self.rollRight(parent);
                        }
                        // Java: else
                        else
                        {
                            // Java: this.rollRight(pp, parent);
                            self.rollRight_parent(pp, parent);
                        }
                        // Java: parent = this.NIL;
                        parent = nil;
                        // Java: node = root;
                        node = root;
                    }
                }
                // Java: if (!node.isBlack())
                if !(*node).isBlack
                {
                    // Java: if (node.refCount() > 1)
                    if (*node).refCount > 1
                    {
                        // Java: if (this.NIL == parent)
                        if nil == parent
                        {
                            // Java: root = this.copyNode(node);
                            root = self.copyNode_single(node);
                            // Java: node = root;
                            node = root;
                        }
                        // Java: else
                        else
                        {
                            // Java: node = this.copyNode(parent, node);
                            node = self.copyNode_with_parent(parent, node);
                        }
                    }
                    // Java: node.setBlack(true);
                    (*node).isBlack = true;
                }
                // Java: return root;
                return root;
            }
        }
    }

    // Java: void clear()
    pub fn clear(&mut self) {
        // Java: this.updateTree(this.NIL, 0);
        let nil = self.NIL;
        self.updateTree(nil, 0);
    }

    // Java: void release()
    pub fn release(&mut self) {
        // Java: this.clear();
        self.clear();
    }

    // Java: boolean contains(K key)
    pub fn contains(&self, key: K) -> bool {
        // Java: RB_Node<K, V> root = this.increaseRefRoot();
        let root = self.increaseRefRoot();
        // Java: boolean contains = this.get(root, key) != this.NIL;
        let contains = self.get_node(root, key) != self.NIL;
        // Java: if (root != this.NIL)
        if root != self.NIL
        {
            // Java: this.decreaseRef(root);
            self.decreaseRef(root);
        }
        // Java: return contains;
        contains
    }

    // Java: K lowerBound(K key)
    pub fn lowerBound(&self, key: K) -> Option<K> {
        // Java: RB_Node<K, V> root = this.increaseRefRoot();
        let root = self.increaseRefRoot();
        // Java: RB_Node<K, V> node = this.lowerBound(root, key);
        let node = self.lowerBound_node(root, key);
        // Java: K lowerBound = (K)(null == node ? null : node.key());
        let lb = if node.is_null() { None } else { Some(unsafe { (*node).key }) };
        // Java: if (root != this.NIL)
        if root != self.NIL
        {
            // Java: this.decreaseRef(root);
            self.decreaseRef(root);
        }
        // Java: return lowerBound;
        lb
    }

    // Java: K upperBound(K key)
    pub fn upperBound(&self, key: K) -> Option<K> {
        // Java: RB_Node<K, V> root = this.increaseRefRoot();
        let root = self.increaseRefRoot();
        // Java: RB_Node<K, V> node = this.upperBound(root, key);
        let node = self.upperBound_node(root, key);
        // Java: K upperBound = (K)(null == node ? null : node.key());
        let ub = if node.is_null() { None } else { Some(unsafe { (*node).key }) };
        // Java: if (root != this.NIL)
        if root != self.NIL
        {
            // Java: this.decreaseRef(root);
            self.decreaseRef(root);
        }
        // Java: return upperBound;
        ub
    }

    // Java: K firstKey()
    pub fn firstKey(&self) -> Option<K> {
        // Java: RB_Node<K, V> root = this.increaseRefRoot();
        let root = self.increaseRefRoot();
        // Java: RB_Node<K, V> node = this.first(root);
        let node = self.first(root);
        // Java: K key = (K)(null == node ? null : node.key());
        let key = if node.is_null() { None } else { Some(unsafe { (*node).key }) };
        // Java: if (root != this.NIL)
        if root != self.NIL
        {
            // Java: this.decreaseRef(root);
            self.decreaseRef(root);
        }
        // Java: return key;
        key
    }

    // Java: K lastKey()
    pub fn lastKey(&self) -> Option<K> {
        // Java: RB_Node<K, V> root = this.increaseRefRoot();
        let root = self.increaseRefRoot();
        // Java: RB_Node<K, V> node = this.last(root);
        let node = self.last(root);
        // Java: K key = (K)(null == node ? null : node.key());
        let key = if node.is_null() { None } else { Some(unsafe { (*node).key }) };
        // Java: if (root != this.NIL)
        if root != self.NIL
        {
            // Java: this.decreaseRef(root);
            self.decreaseRef(root);
        }
        // Java: return key;
        key
    }

    pub fn getIterator(&self) -> CowTreeMapIterator {
        CowTreeMapIterator::new(self.increaseRefRoot(), self.NIL)
    }
}

// Java: public class Iterator
impl CowTreeMapIterator {
    // Java: Iterator(RB_Node<K, V> root)
    pub fn new(root: *mut TRBNode, nil: *mut TRBNode) -> Self {
        // Java: super();
        // Java: this.root = root;
        CowTreeMapIterator { root, curr: std::ptr::null_mut(), nil }
    }

    // Java: V next()
    pub fn next(&mut self, tree: &CopyOnWriteTreeMap) -> Val {
        // Java: if (null == this.curr)
        if self.curr.is_null()
        {
            // Java: this.curr = CopyOnWriteTreeMap.this.first(this.root);
            self.curr = tree.first(self.root);
        }
        // Java: else
        else
        {
            // Java: this.curr = CopyOnWriteTreeMap.this.next(this.root, this.curr);
            self.curr = tree.next(self.root, self.curr);
        }
        // Java: return (V)(null == this.curr ? null : this.curr.value());
        if self.curr.is_null() { None } else { unsafe { (*self.curr).value.clone() } }
    }

    // Java: V prev()
    pub fn prev(&mut self, tree: &CopyOnWriteTreeMap) -> Val {
        // Java: if (null == this.curr)
        if self.curr.is_null()
        {
            // Java: this.curr = CopyOnWriteTreeMap.this.last(this.root);
            self.curr = tree.last(self.root);
        }
        // Java: else
        else
        {
            // Java: this.curr = CopyOnWriteTreeMap.this.prev(this.root, this.curr);
            self.curr = tree.prev(self.root, self.curr);
        }
        // Java: return (V)(null == this.curr ? null : this.curr.value());
        if self.curr.is_null() { None } else { unsafe { (*self.curr).value.clone() } }
    }

    // Java: V value()
    pub fn value(&self) -> Val {
        // Java: return (V)(null == this.curr ? null : this.curr.value());
        if self.curr.is_null() { None } else { unsafe { (*self.curr).value.clone() } }
    }

    // Java: K key()
    pub fn key(&self) -> Option<K> {
        // Java: return (K)(null == this.curr ? null : this.curr.key());
        if self.curr.is_null() { None } else { Some(unsafe { (*self.curr).key }) }
    }

    // Java: void release()
    pub fn release(&mut self, tree: &CopyOnWriteTreeMap) {
        // Java: if (this.root != CopyOnWriteTreeMap.this.NIL)
        if self.root != tree.NIL
        {
            // Java: CopyOnWriteTreeMap.this.decreaseRef(this.root);
            tree.decreaseRef(self.root);
        }
        // Java: this.root = CopyOnWriteTreeMap.this.NIL;
        self.root = tree.NIL;
    }
}
