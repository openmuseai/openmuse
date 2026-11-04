#![allow(dead_code, non_snake_case, unused_variables, unused_assignments, unused_mut)]

// Java: import java.util.ArrayList

use std::sync::{Arc, Mutex};
use std::any::Any;

type V = Option<Arc<dyn Any + Send + Sync>>;

unsafe impl Send for RBNode {}
unsafe impl Sync for RBNode {}
unsafe impl Send for CopyOnWriteIntTreeMap {}
unsafe impl Sync for CopyOnWriteIntTreeMap {}
unsafe impl Send for TreeMapIterator {}
unsafe impl Sync for TreeMapIterator {}

// Java field: boolean DEBUG
pub(crate) const DEBUG: bool = false;
// Java field: ArrayList<RB_Node<?>> sSharePool
struct SendPtr(*mut RBNode);
unsafe impl Send for SendPtr {}
unsafe impl Sync for SendPtr {}
static S_SHARE_POOL: Mutex<Vec<SendPtr>> = Mutex::new(Vec::new());
// Java field: int LOCAL_CACHE_MAX
pub(crate) const LOCAL_CACHE_MAX: i32 = 128;
// Java field: int SHARE_CACHE_MAX
pub(crate) const SHARE_CACHE_MAX: i32 = 4096;

// Java: public class RB_Node
pub struct RBNode {
    // Java field: int key
    pub(crate) key: i32,
    // Java field: V value
    pub(crate) value: V,
    // Java field: RB_Node<V> left
    pub(crate) left: *mut RBNode,
    // Java field: RB_Node<V> right
    pub(crate) right: *mut RBNode,
    // Java field: boolean isBlack
    pub(crate) isBlack: bool,
    // Java field: int refCount
    pub(crate) refCount: i32,
}

// Java: public class CopyOnWriteIntTreeMap
pub struct CopyOnWriteIntTreeMap {
    // Java field: RB_Node<V> NIL
    pub(crate) NIL: *mut RBNode,
    // Java field: Object _rootLock
    _rootLock: Mutex<()>,
    // Java field: RB_Node<V> _root
    _root: *mut RBNode,
    // Java field: int _size
    pub(crate) _size: i32,
    // Java field: Alloc<V> _alloc
    pub(crate) _alloc: Alloc,
}

// Java: public class RecycleAlloc
pub struct RecycleAlloc {
    // Java field: ArrayList<RB_Node<V>> localPool
    pub(crate) localPool: Vec<*mut RBNode>,
    // Java field: int LOCAL_CACHE_MAX
    // Java field: int SHARE_CACHE_MAX
}

// Java: public class Alloc
pub struct Alloc {
    // Java field: ThreadLocal<RecycleAlloc<V>> alloc
    pub(crate) alloc: Mutex<Option<RecycleAlloc>>,
}

// Java: public class Iterator
pub struct TreeMapIterator {
    // Java field: RB_Node<V> root
    pub(crate) root: *mut RBNode,
    // Java field: RB_Node<V> curr
    pub(crate) curr: *mut RBNode,
    pub(crate) nil: *mut RBNode,
}

// Java: public class RB_Node
impl RBNode {
    // Java: RB_Node()
    pub fn new() -> Self {
        // Java: super();
        RBNode {
            key: 0,
            value: None,
            left: std::ptr::null_mut(),
            right: std::ptr::null_mut(),
            isBlack: false,
            refCount: 0,
        }
    }

    // Java: int key()
    pub fn key(&self) -> i32 {
        // Java: return this.key;
        self.key
    }

    // Java: V value()
    pub fn value(&self) -> &V {
        // Java: return this.value;
        &self.value
    }

    // Java: RB_Node<V> left()
    pub fn left(&self) -> *mut RBNode {
        // Java: return this.left;
        self.left
    }

    // Java: RB_Node<V> right()
    pub fn right(&self) -> *mut RBNode {
        // Java: return this.right;
        self.right
    }

    // Java: boolean isBlack()
    pub fn isBlack(&self) -> bool {
        // Java: return this.isBlack;
        self.isBlack
    }

    // Java: void setValue(V value)
    pub fn setValue(&mut self, value: V) {
        // Java: this.value = value;
        self.value = value;
    }

    // Java: void setData(int key, V value, boolean isBlack)
    pub fn setData(&mut self, key: i32, value: V, isBlack: bool) {
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
impl RecycleAlloc {
    // Java: RecycleAlloc()
    pub fn new() -> Self {
        // Java: super();
        // Java: this.localPool = new ArrayList();
        RecycleAlloc { localPool: Vec::new() }
    }

    // Java: RB_Node<V> alloc()
    pub fn alloc(&mut self) -> *mut RBNode {
        // Java: if (this.localPool.isEmpty() && sSharePool.size() > 64)
        if self.localPool.is_empty()
        {
            let share_big = S_SHARE_POOL.lock().map(|s| s.len() > 64).unwrap_or(false);
            if share_big {
                // Java: synchronized(sSharePool)
                {
                    let mut share = S_SHARE_POOL.lock().unwrap();
                    // Java: for(int count = Math.min(sSharePool.size(), 64); count > 0; --count)
                    let count = share.len().min(64);
                    for _ in 0..count
                    {
                        // Java: this.localPool.add((RB_Node)sSharePool.remove(sSharePool.size() - 1));
                        if let Some(SendPtr(n)) = share.pop() {
                            self.localPool.push(n);
                        }
                    }
                }
            }
        }
        // Java: RB_Node<V> node = null;
        let node: *mut RBNode;
        // Java: if (this.localPool.size() > 0)
        if !self.localPool.is_empty()
        {
            // Java: node = (RB_Node)this.localPool.remove(this.localPool.size() - 1);
            node = self.localPool.pop().unwrap();
        }
        // Java: else
        else
        {
            // Java: node = new RB_Node<V>();
            node = Box::into_raw(Box::new(RBNode::new()));
        }
        // Java: return node;
        node
    }

    // Java: void free(RB_Node<V> node)
    pub fn free(&mut self, node: *mut RBNode) {
        // Java: if (this.localPool.size() >= 128 && sSharePool.size() < 4096)
        {
            let share_small = S_SHARE_POOL.lock().map(|s| s.len() < SHARE_CACHE_MAX as usize).unwrap_or(false);
            if self.localPool.len() >= LOCAL_CACHE_MAX as usize && share_small {
                // Java: synchronized(sSharePool)
                {
                    let mut share = S_SHARE_POOL.lock().unwrap();
                    // Java: for(int count = 64; count > 0; --count)
                    for _ in 0..64
                    {
                        // Java: sSharePool.add((RB_Node)this.localPool.remove(this.localPool.size() - 1));
                        if let Some(n) = self.localPool.pop() {
                            share.push(SendPtr(n));
                        }
                    }
                }
            }
        }
        // Java: if (this.localPool.size() < 128)
        if (self.localPool.len()) < LOCAL_CACHE_MAX as usize
        {
            // Java: this.localPool.add(node);
            self.localPool.push(node);
        }
    }
}

// Java: public class Alloc
impl Alloc {
    // Java: Alloc()
    pub fn new() -> Self {
        // Java: super();
        // Java: this.alloc = new ThreadLocal();
        Alloc { alloc: Mutex::new(None) }
    }

    // Java: RB_Node<V> alloc()
    pub fn alloc(&self) -> *mut RBNode {
        // Java: RecycleAlloc<V> _alloc = (RecycleAlloc)this.alloc.get();
        let mut guard = self.alloc.lock().unwrap();
        // Java: if (null == _alloc)
        if guard.is_none()
        {
            // Java: _alloc = new RecycleAlloc<V>();
            // Java: this.alloc.set(_alloc);
            *guard = Some(RecycleAlloc::new());
        }
        // Java: return _alloc.alloc();
        guard.as_mut().unwrap().alloc()
    }

    // Java: void free(RB_Node<V> node)
    pub fn free(&self, node: *mut RBNode) {
        // Java: RecycleAlloc<V> _alloc = (RecycleAlloc)this.alloc.get();
        let mut guard = self.alloc.lock().unwrap();
        // Java: if (null != _alloc)
        if let Some(ref mut _alloc) = *guard
        {
            // Java: _alloc.free(node);
            _alloc.free(node);
        }
    }
}

// Java: public class CopyOnWriteIntTreeMap
impl CopyOnWriteIntTreeMap {
    // Java: CopyOnWriteIntTreeMap()
    pub fn new() -> Self {
        // Java: super();
        // Java: this.NIL = new RB_Node<V>();
        let nil = Box::into_raw(Box::new(RBNode::new()));
        unsafe {
            // Java: this._root = this.NIL;
            // Java: this._alloc = new Alloc<V>();
            // Java: this.NIL.left = this.NIL;
            (*nil).left = nil;
            // Java: this.NIL.right = this.NIL;
            (*nil).right = nil;
            // Java: this.NIL.isBlack = true;
            (*nil).isBlack = true;
        }
        CopyOnWriteIntTreeMap {
            NIL: nil,
            _rootLock: Mutex::new(()),
            _root: nil,
            _size: 0,
            _alloc: Alloc::new(),
        }
    }

    // Java: CopyOnWriteIntTreeMap(CopyOnWriteIntTreeMap<V> clone)
    pub fn new_from(clone: &mut CopyOnWriteIntTreeMap) -> Self {
        // Java: super();
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
        CopyOnWriteIntTreeMap {
            NIL: nil,
            _rootLock: Mutex::new(()),
            _root,
            _size,
            _alloc: Alloc::new(),
        }
    }

    // Java: void copyFrom(CopyOnWriteIntTreeMap<V> clone)
    pub fn copyFrom(&mut self, clone: &mut CopyOnWriteIntTreeMap) {
        // Java: this.clear();
        self.clear();
        // Java: this.NIL = clone.NIL;
        self.NIL = clone.NIL;
        // Java: synchronized(clone._rootLock)
        {
            let _lock = clone._rootLock.lock().unwrap();
            // Java: this._root = clone._root;
            self._root = clone._root;
            // Java: if (this._root != this.NIL)
            if self._root != self.NIL
            {
                // Java: this.increaseRef(this._root);
                self.increaseRef(self._root);
            }
            // Java: this._size = clone._size;
            self._size = clone._size;
        }
        // Java: this._alloc = clone._alloc;
    }

    // Java: void updateTree(RB_Node<V> root, int size)
    fn updateTree(&mut self, root: *mut RBNode, size: i32) {
        // Java: RB_Node<V> oldRoot = null;
        let mut oldRoot: *mut RBNode = std::ptr::null_mut();
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

    // Java: RB_Node<V> root()
    fn root(&self) -> *mut RBNode {
        // Java: return this._root;
        self._root
    }

    // Java: RB_Node<V> increaseRefRoot()
    fn increaseRefRoot(&self) -> *mut RBNode {
        // Java: synchronized(this._rootLock)
        {
            let _lock = self._rootLock.lock().unwrap();
            // Java: if (this._root != this.NIL)
            if self._root != self.NIL
            {
                // Java: this.increaseRef(this._root);
                unsafe { (*(self._root)).refCount += 1; }
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

    // Java: RB_Node<V> alloc()
    fn alloc(&self) -> *mut RBNode {
        // Java: RB_Node<V> node = this._alloc.alloc();
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

    // Java: void _free(RB_Node<V> node)
    fn _free(&self, node: *mut RBNode) {
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

    // Java: void setLeft(RB_Node<V> parent, RB_Node<V> left)
    fn setLeft(&self, parent: *mut RBNode, left: *mut RBNode) {
        unsafe {
            // Java: RB_Node<V> oldLeft = parent.left;
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

    // Java: void setRight(RB_Node<V> parent, RB_Node<V> right)
    fn setRight(&self, parent: *mut RBNode, right: *mut RBNode) {
        unsafe {
            // Java: RB_Node<V> oldRight = parent.right;
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

    // Java: void increaseRef(RB_Node<V> node)
    fn increaseRef(&self, node: *mut RBNode) {
        // Java: synchronized(node)
        unsafe {
            // Java: ++node.refCount;
            (*node).refCount += 1;
        }
    }

    // Java: void decreaseRef(RB_Node<V> node)
    fn decreaseRef(&self, node: *mut RBNode) {
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

    // Java: void rollLeft(RB_Node<V> parent, RB_Node<V> node)
    fn rollLeft_parent(&self, parent: *mut RBNode, node: *mut RBNode) {
        // Java: RB_Node<V> c = this.rollLeft(node);
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

    // Java: RB_Node<V> rollLeft(RB_Node<V> node)
    fn rollLeft(&self, node: *mut RBNode) -> *mut RBNode {
        unsafe {
            // Java: RB_Node<V> right = node.right();
            let right = (*node).right;
            // Java: node.right = right.left;
            (*node).right = (*right).left;
            // Java: right.left = node;
            (*right).left = node;
            // Java: return right;
            right
        }
    }

    // Java: void rollRight(RB_Node<V> parent, RB_Node<V> node)
    fn rollRight_parent(&self, parent: *mut RBNode, node: *mut RBNode) {
        // Java: RB_Node<V> c = this.rollRight(node);
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

    // Java: RB_Node<V> rollRight(RB_Node<V> node)
    fn rollRight(&self, node: *mut RBNode) -> *mut RBNode {
        unsafe {
            // Java: RB_Node<V> left = node.left();
            let left = (*node).left;
            // Java: node.left = left.right;
            (*node).left = (*left).right;
            // Java: left.right = node;
            (*left).right = node;
            // Java: return left;
            left
        }
    }

    // Java: RB_Node<V> first(RB_Node<V> root)
    fn first(&self, root: *mut RBNode) -> *mut RBNode {
        // Java: RB_Node<V> node = null;
        let mut node: *mut RBNode = std::ptr::null_mut();
        // Java: for(RB_Node<V> curr = root; curr != this.NIL; curr = curr.left())
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

    // Java: RB_Node<V> last(RB_Node<V> root)
    fn last(&self, root: *mut RBNode) -> *mut RBNode {
        // Java: RB_Node<V> node = null;
        let mut node: *mut RBNode = std::ptr::null_mut();
        // Java: for(RB_Node<V> curr = root; curr != this.NIL; curr = curr.right())
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

    // Java: RB_Node<V> next(RB_Node<V> root, RB_Node<V> node)
    fn next(&self, root: *mut RBNode, node: *mut RBNode) -> *mut RBNode {
        unsafe {
            // Java: if (node.right() != this.NIL)
            if (*node).right != self.NIL
            {
                // Java: RB_Node<V> next = node.right();
                let mut next = (*node).right;
                // Java: for(RB_Node<V> curr = next; curr != this.NIL; curr = curr.left())
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
                // Java: RB_Node<V> next = null;
                let mut next: *mut RBNode = std::ptr::null_mut();
                // Java: RB_Node<V> curr = root;
                let mut curr = root;
                // Java: while(curr != this.NIL)
                while curr != self.NIL
                {
                    // Java: if (curr.key() < node.key())
                    if (*curr).key < (*node).key
                    {
                        // Java: curr = curr.right();
                        curr = (*curr).right;
                    }
                    // Java: else
                    else
                    {
                        // Java: if (curr.key() <= node.key())
                        if (*curr).key <= (*node).key
                        {
                            // Java: break;
                            break;
                        }
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

    // Java: RB_Node<V> prev(RB_Node<V> root, RB_Node<V> node)
    fn prev(&self, root: *mut RBNode, node: *mut RBNode) -> *mut RBNode {
        unsafe {
            // Java: if (node.left() != this.NIL)
            if (*node).left != self.NIL
            {
                // Java: RB_Node<V> prev = node.left();
                let mut prev = (*node).left;
                // Java: for(RB_Node<V> curr = prev; curr != this.NIL; curr = curr.right())
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
                // Java: RB_Node<V> prev = null;
                let mut prev: *mut RBNode = std::ptr::null_mut();
                // Java: RB_Node<V> curr = root;
                let mut curr = root;
                // Java: while(curr != this.NIL)
                while curr != self.NIL
                {
                    // Java: if (curr.key() < node.key())
                    if (*curr).key < (*node).key
                    {
                        // Java: prev = curr;
                        prev = curr;
                        // Java: curr = curr.right();
                        curr = (*curr).right;
                    }
                    // Java: else
                    else
                    {
                        // Java: if (curr.key() <= node.key())
                        if (*curr).key <= (*node).key
                        {
                            // Java: break;
                            break;
                        }
                        // Java: curr = curr.left();
                        curr = (*curr).left;
                    }
                }
                // Java: return prev;
                return prev;
            }
        }
    }

    // Java: RB_Node<V> parent(RB_Node<V> root, int key)
    fn parent(&self, root: *mut RBNode, key: i32) -> *mut RBNode {
        // Java: RB_Node<V> parent = this.NIL;
        let mut parent = self.NIL;
        // Java: for(RB_Node<V> curr = root; curr != this.NIL && curr.key() != key; curr = curr.key() < key ? curr.right() : curr.left())
        let mut curr = root;
        unsafe {
            while curr != self.NIL && (*curr).key != key
            {
                // Java: parent = curr;
                parent = curr;
                curr = if (*curr).key < key { (*curr).right } else { (*curr).left };
            }
        }
        // Java: return parent;
        parent
    }

    // Java: V get(int key)
    pub fn get(&self, key: i32) -> V {
        // Java: RB_Node<V> root = this.increaseRefRoot();
        let root = self.increaseRefRoot();
        // Java: RB_Node<V> node = this.get(root, key);
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

    // Java: RB_Node<V> get(RB_Node<V> root, int key)
    fn get_node(&self, root: *mut RBNode, key: i32) -> *mut RBNode {
        // Java: RB_Node<V> curr;
        // Java: for(curr = root; curr != this.NIL && curr.key() != key; curr = curr.key() < key ? curr.right() : curr.left())
        let mut curr = root;
        unsafe {
            while curr != self.NIL && (*curr).key != key
            {
                curr = if (*curr).key < key { (*curr).right } else { (*curr).left };
            }
        }
        // Java: return curr;
        curr
    }

    // Java: RB_Node<V> lowerBound(RB_Node<V> root, int key)
    fn lowerBound_node(&self, root: *mut RBNode, key: i32) -> *mut RBNode {
        // Java: RB_Node<V> parent = this.NIL;
        let mut parent = self.NIL;
        // Java: RB_Node<V> curr;
        // Java: for(curr = root; curr != this.NIL && curr.key() != key; curr = curr.key() < key ? curr.right() : curr.left())
        let mut curr = root;
        unsafe {
            while curr != self.NIL && (*curr).key != key
            {
                // Java: parent = curr;
                parent = curr;
                curr = if (*curr).key < key { (*curr).right } else { (*curr).left };
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
            // Java: return parent.key() > key ? parent : this.next(root, parent);
            unsafe {
                if (*parent).key > key { parent } else { self.next(root, parent) }
            }
        }
    }

    // Java: RB_Node<V> upperBound(RB_Node<V> root, int key)
    fn upperBound_node(&self, root: *mut RBNode, key: i32) -> *mut RBNode {
        // Java: RB_Node<V> parent = this.NIL;
        let mut parent = self.NIL;
        // Java: RB_Node<V> curr;
        // Java: for(curr = root; curr != this.NIL && curr.key() != key; curr = curr.key() < key ? curr.right() : curr.left())
        let mut curr = root;
        unsafe {
            while curr != self.NIL && (*curr).key != key
            {
                // Java: parent = curr;
                parent = curr;
                curr = if (*curr).key < key { (*curr).right } else { (*curr).left };
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
            // Java: return parent.key() > key ? parent : this.next(root, parent);
            unsafe {
                if (*parent).key > key { parent } else { self.next(root, parent) }
            }
        }
    }

    // Java: RB_Node<V> copyNode(RB_Node<V> copyNode)
    fn copyNode_single(&self, copy_node: *mut RBNode) -> *mut RBNode {
        // Java: RB_Node<V> _node = this.alloc();
        let _node = self.alloc();
        unsafe {
            // Java: _node.setData(copyNode.key(), copyNode.value(), copyNode.isBlack());
            (*_node).key = (*copy_node).key;
            (*_node).value = (*copy_node).value.clone();
            (*_node).isBlack = (*copy_node).isBlack;
        }
        // Java: this.setLeft(_node, copyNode.left());
        self.setLeft(_node, unsafe { (*copy_node).left });
        // Java: this.setRight(_node, copyNode.right());
        self.setRight(_node, unsafe { (*copy_node).right });
        // Java: return _node;
        _node
    }

    // Java: RB_Node<V> copyNode(RB_Node<V> parent, RB_Node<V> copyNode)
    fn copyNode_with_parent(&self, parent: *mut RBNode, copy_node: *mut RBNode) -> *mut RBNode {
        // Java: RB_Node<V> _node = this.alloc();
        let _node = self.alloc();
        unsafe {
            // Java: _node.setData(copyNode.key(), copyNode.value(), copyNode.isBlack());
            (*_node).key = (*copy_node).key;
            (*_node).value = (*copy_node).value.clone();
            (*_node).isBlack = (*copy_node).isBlack;
        }
        // Java: this.setLeft(_node, copyNode.left());
        self.setLeft(_node, unsafe { (*copy_node).left });
        // Java: this.setRight(_node, copyNode.right());
        self.setRight(_node, unsafe { (*copy_node).right });
        unsafe {
            // Java: if (copyNode == parent.left())
            if copy_node == (*parent).left
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

    // Java: CopyOnWriteIntTreeMap<V> clone()
    pub fn clone(&mut self) -> CopyOnWriteIntTreeMap {
        // Java: return new CopyOnWriteIntTreeMap<V>(this);
        CopyOnWriteIntTreeMap::new_from(self)
    }

    // Java: V insert(int key, V value)
    pub fn insert(&mut self, key: i32, value: V) -> V {
        // Java: RB_Node<V> oldRoot = this.root();
        let oldRoot = self.root();
        // Java: RB_Node<V> root = this.NIL;
        let mut root = self.NIL;
        // Java: RB_Node<V> parent = this.NIL;
        let mut parent = self.NIL;
        let nil = self.NIL;
        // Java: for(RB_Node<V> curr = oldRoot; curr != this.NIL; curr = curr.key() < key ? curr.right() : curr.left())
        let mut curr = oldRoot;
        unsafe {
            while curr != nil
            {
                // Java: RB_Node<V> node;
                let node: *mut RBNode;
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
                // Java: if (curr.key() == key)
                if (*curr).key == key
                {
                    // Java: break;
                    break;
                }
                // Java: parent = node;
                parent = node;
                curr = if (*curr).key < key { (*curr).right } else { (*curr).left };
            }
        }
        // Java: V oldValue = null;
        let mut oldValue: V = None;
        // Java: RB_Node<V> node = null;
        let mut node: *mut RBNode = std::ptr::null_mut();
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
            // Java: else if (parent.key() < key)
            else if (*parent).key < key
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

    // Java: RB_Node<V> insertFixup(RB_Node<V> root, RB_Node<V> parent, RB_Node<V> node)
    fn insertFixup(&self, mut root: *mut RBNode, mut parent: *mut RBNode, mut node: *mut RBNode) -> *mut RBNode {
        let nil = self.NIL;
        // Java: while(true)
        loop
        {
            unsafe {
                // Java: if (!parent.isBlack())
                if !(*parent).isBlack
                {
                    // Java: RB_Node<V> pp = this.parent(root, parent.key());
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
                        // Java: RB_Node<V> ppright = pp.right();
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
                            // Java: RB_Node<V> ppp = this.parent(root, pp.key());
                            let ppp = self.parent(root, (*pp).key);
                            // Java: if (this.NIL == ppp)
                            if nil == ppp
                            {
                                // Java: this.rollRight(pp);
                                let r = self.rollRight(pp);
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
                        // Java: RB_Node<V> ppleft = pp.left();
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
                            // Java: RB_Node<V> ppp = this.parent(root, pp.key());
                            let ppp = self.parent(root, (*pp).key);
                            // Java: if (this.NIL == ppp)
                            if nil == ppp
                            {
                                // Java: this.rollLeft(pp);
                                let r = self.rollLeft(pp);
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
            }
            // Java: return root;
            return root;
        }
    }

    // Java: V delete(int key)
    pub fn delete(&mut self, key: i32) -> V {
        let nil = self.NIL;
        // Java: RB_Node<V> root = this.root();
        let mut root = self.root();
        // Java: RB_Node<V> curr = this.get(root, key);
        let mut curr = self.get_node(root, key);
        // Java: if (this.NIL == curr)
        if nil == curr
        {
            // Java: return null;
            return None;
        }
        // Java: else
        {
            unsafe {
                // Java: RB_Node<V> y;
                let mut y: *mut RBNode;
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
                // Java: RB_Node<V> x;
                let x: *mut RBNode;
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
                // Java: RB_Node<V> py = this.parent(root, y.key());
                let mut py = self.parent(root, (*y).key);
                // Java: if (this.NIL == py)
                if nil == py
                {
                    // Java: RB_Node<V> node = this.alloc();
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
                    // Java: RB_Node<V> _curr = root;
                    let mut _curr = root;
                    // Java: root = this.NIL;
                    root = nil;
                    // Java: for(RB_Node<V> parent = this.NIL; _curr != this.NIL; _curr = _curr.key() < py.key() ? _curr.right() : _curr.left())
                    let mut parent_d = nil;
                    while _curr != nil
                    {
                        // Java: RB_Node<V> node;
                        let node: *mut RBNode;
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
                        // Java: if (_curr.key() == py.key())
                        if (*_curr).key == (*py).key
                        {
                            // Java: py = node;
                            py = node;
                            // Java: break;
                            break;
                        }
                        // Java: parent = node;
                        parent_d = node;
                        _curr = if (*_curr).key < (*py).key { (*_curr).right } else { (*_curr).left };
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
    }

    // Java: boolean _isParent(RB_Node<V> parent, RB_Node<V> node)
    fn _isParent(&self, parent: *mut RBNode, node: *mut RBNode) -> bool {
        // Java: return parent.left() == node || parent.right() == node;
        unsafe { (*parent).left == node || (*parent).right == node }
    }

    // Java: RB_Node<V> deleteFixup(RB_Node<V> root, RB_Node<V> parent, RB_Node<V> node)
    fn deleteFixup(&self, mut root: *mut RBNode, mut parent: *mut RBNode, mut node: *mut RBNode) -> *mut RBNode {
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
                        // Java: RB_Node<V> pright = parent.right();
                        let mut pright = (*parent).right;
                        // Java: RB_Node<V> pp = this.parent(root, parent.key());
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
                        // Java: RB_Node<V> pleft = parent.left();
                        let mut pleft = (*parent).left;
                        // Java: RB_Node<V> pp = this.parent(root, parent.key());
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
            }
            // Java: return root;
            return root;
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

    // Java: boolean contains(int key)
    pub fn contains(&self, key: i32) -> bool {
        // Java: RB_Node<V> root = this.increaseRefRoot();
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

    // Java: Integer lowerBound(int key)
    pub fn lowerBound(&self, key: i32) -> Option<i32> {
        // Java: RB_Node<V> root = this.increaseRefRoot();
        let root = self.increaseRefRoot();
        // Java: RB_Node<V> node = this.lowerBound(root, key);
        let node = self.lowerBound_node(root, key);
        // Java: Integer lowerBound = null == node ? null : node.key();
        let lower_bound = if node.is_null() { None } else { Some(unsafe { (*node).key }) };
        // Java: if (root != this.NIL)
        if root != self.NIL
        {
            // Java: this.decreaseRef(root);
            self.decreaseRef(root);
        }
        // Java: return lowerBound;
        lower_bound
    }

    // Java: Integer upperBound(int key)
    pub fn upperBound(&self, key: i32) -> Option<i32> {
        // Java: RB_Node<V> root = this.increaseRefRoot();
        let root = self.increaseRefRoot();
        // Java: RB_Node<V> node = this.upperBound(root, key);
        let node = self.upperBound_node(root, key);
        // Java: Integer upperBound = null == node ? null : node.key();
        let upper_bound = if node.is_null() { None } else { Some(unsafe { (*node).key }) };
        // Java: if (root != this.NIL)
        if root != self.NIL
        {
            // Java: this.decreaseRef(root);
            self.decreaseRef(root);
        }
        // Java: return upperBound;
        upper_bound
    }

    // Java: Integer firstKey()
    pub fn firstKey(&self) -> Option<i32> {
        // Java: RB_Node<V> root = this.increaseRefRoot();
        let root = self.increaseRefRoot();
        // Java: RB_Node<V> node = this.first(root);
        let node = self.first(root);
        // Java: Integer key = null == node ? null : node.key();
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

    // Java: Integer lastKey()
    pub fn lastKey(&self) -> Option<i32> {
        // Java: RB_Node<V> root = this.increaseRefRoot();
        let root = self.increaseRefRoot();
        // Java: RB_Node<V> node = this.last(root);
        let node = self.last(root);
        // Java: Integer key = null == node ? null : node.key();
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

    pub fn getIterator(&self) -> TreeMapIterator {
        TreeMapIterator::new(self.increaseRefRoot(), self.NIL)
    }
}

// Java: public class Iterator
impl TreeMapIterator {
    // Java: Iterator(RB_Node<V> root)
    pub fn new(root: *mut RBNode, nil: *mut RBNode) -> Self {
        // Java: super();
        // Java: this.root = root;
        TreeMapIterator {
            root,
            curr: std::ptr::null_mut(),
            nil,
        }
    }

    // Java: V next()
    pub fn next(&mut self, tree: &CopyOnWriteIntTreeMap) -> V {
        // Java: if (null == this.curr)
        if self.curr.is_null()
        {
            // Java: this.curr = CopyOnWriteIntTreeMap.this.first(this.root);
            self.curr = tree.first(self.root);
        }
        // Java: else
        else
        {
            // Java: this.curr = CopyOnWriteIntTreeMap.this.next(this.root, this.curr);
            self.curr = tree.next(self.root, self.curr);
        }
        // Java: return (V)(null == this.curr ? null : this.curr.value());
        if self.curr.is_null() { None } else { unsafe { (*self.curr).value.clone() } }
    }

    // Java: V prev()
    pub fn prev(&mut self, tree: &CopyOnWriteIntTreeMap) -> V {
        // Java: if (null == this.curr)
        if self.curr.is_null()
        {
            // Java: this.curr = CopyOnWriteIntTreeMap.this.last(this.root);
            self.curr = tree.last(self.root);
        }
        // Java: else
        else
        {
            // Java: this.curr = CopyOnWriteIntTreeMap.this.prev(this.root, this.curr);
            self.curr = tree.prev(self.root, self.curr);
        }
        // Java: return (V)(null == this.curr ? null : this.curr.value());
        if self.curr.is_null() { None } else { unsafe { (*self.curr).value.clone() } }
    }

    // Java: V value()
    pub fn value(&self) -> V {
        // Java: return (V)(null == this.curr ? null : this.curr.value());
        if self.curr.is_null() { None } else { unsafe { (*self.curr).value.clone() } }
    }

    // Java: int key()
    pub fn key(&self) -> Option<i32> {
        // Java: return null == this.curr ? null : this.curr.key();
        if self.curr.is_null() { None } else { Some(unsafe { (*self.curr).key }) }
    }

    // Java: void release()
    pub fn release(&mut self, tree: &CopyOnWriteIntTreeMap) {
        // Java: if (this.root != CopyOnWriteIntTreeMap.this.NIL)
        if self.root != tree.NIL
        {
            // Java: CopyOnWriteIntTreeMap.this.decreaseRef(this.root);
            tree.decreaseRef(self.root);
        }
        // Java: this.root = CopyOnWriteIntTreeMap.this.NIL;
        self.root = tree.NIL;
    }
}
