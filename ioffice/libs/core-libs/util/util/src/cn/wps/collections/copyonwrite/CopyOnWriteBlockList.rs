#![allow(non_snake_case, dead_code, unused_variables, unused_mut, unused_imports, unused_assignments)]

use std::sync::Mutex;

const BLOCK_DATA_SIZE: usize = 32;

// Java field: long serialVersionUID
const SERIAL_VERSION_UID: i64 = -833788695680383654;
// Java field: boolean DEBUG
const DEBUG: bool = false;
// Java field: int BLOCK_SIZE
const BLOCK_SIZE_CONST: i32 = 32;
// Java field: Alloc alloc
static ALLOC: std::sync::OnceLock<Mutex<Alloc>> = std::sync::OnceLock::new();
// Java field: ArrayList<Block> sSharePool
static S_SHARE_POOL: std::sync::OnceLock<Mutex<Vec<Block>>> = std::sync::OnceLock::new();
// Java field: int LOCAL_CACHE_MAX
const LOCAL_CACHE_MAX: i32 = 4;
// Java field: int SHARE_CACHE_MAX
const SHARE_CACHE_MAX: i32 = 16;

// Java: public class Block
#[derive(Clone)]
pub struct Block {
    // Java field: int refCount
    refCount: i32,
    // Java field: int size
    size: i32,
    // Java field: Object[] data
    data: Vec<String>,
}

// Java: public class RecycleAlloc
pub struct RecycleAlloc {
    // Java field: ArrayList<Block> localPool
    localPool: Vec<Block>,
    // Java field: int LOCAL_CACHE_MAX
    // Java field: int SHARE_CACHE_MAX
}

// Java: public class Alloc
pub struct Alloc {
    // Java field: int _alloc_count
    _alloc_count: i32,
    // Java field: int _free_count
    _free_count: i32,
    // Java field: ThreadLocal<RecycleAlloc> alloc
    alloc_inner: Option<RecycleAlloc>,
}

// Java: public class Node
pub struct BlockNode {
    // Java field: int refCount
    refCount: i32,
    // Java field: Block[] blocks
    blocks: Option<Vec<Option<Block>>>,
    // Java field: int blockSize
    blockSize: i32,
    // Java field: int size
    size: i32,
}

// Java: public class Iterator
pub struct BlockListIterator {
    // Java field: int index
    index: i32,
}

// Java: public class CopyOnWriteBlockList extends AbstractSequentialList implements List, Cloneable, Serializable
pub struct CopyOnWriteBlockList {
    // Java field: Node<T> node
    node: Mutex<Option<BlockNode>>,
}

// Java: public class CopyOnWriteBlockList extends AbstractSequentialList implements List, Cloneable, Serializable
impl CopyOnWriteBlockList {
    // Java: CopyOnWriteBlockList()
    pub fn new() -> Self {
        // Java: super();
        Self { node: Mutex::new(None) }
    }

    // Java: CopyOnWriteBlockList(CopyOnWriteBlockList<T> list)
    pub fn from_list(list: &CopyOnWriteBlockList) -> Self {
        // Java: super();
        // Java: this.node = list.increaseRefNode();
        let n = list.increaseRefNode_internal();
        Self { node: Mutex::new(n) }
    }

    // Java: CopyOnWriteBlockList<T> clone()
    pub fn clone_list(&self) -> CopyOnWriteBlockList {
        // Java: return new CopyOnWriteBlockList<T>(this);
        CopyOnWriteBlockList::from_list(self)
    }

    // Java: boolean add(T data)
    pub fn add_data(&self, data: String) -> bool {
        let mut guard = self.node.lock().unwrap();
        // Java: Node<T> _node = this.node;
        // Java: if (null == _node)
        if guard.is_none()
        {
            // Java: _node = new Node<T>();
            let mut _node = BlockNode::new();
            // Java: _node.increaseRef();
            _node.increaseRef();
            // Java: _node.add(data);
            _node.add(data);
            // Java: return true;
            *guard = Some(_node);
            return true;
        }
        // Java: else
        else
        {
            let mut copied: Option<BlockNode> = None;
            // Java: synchronized(_node)
            {
                let node = guard.as_mut().unwrap();
                // Java: if (_node.refCount > 1)
                if node.refCount > 1
                {
                    // Java: _node = this.copyNode(_node);
                    copied = Some(Self::copyNode_static(node));
                }
                // Java: else
                else
                {
                    // Java: _node.add(data);
                    node.add(data.clone());
                }
            }
            // Java: if (_node != this.node)
            if let Some(ref mut new_node) = copied
            {
                // Java: _node.add(data);
                new_node.add(data);
                // Java: this.node.decreaseRef();
                if let Some(ref mut old) = *guard { old.decreaseRef(); }
                // Java: _node.increaseRef();
                new_node.increaseRef();
                // Java: this.node = _node;
                *guard = copied;
            }
            // Java: return true;
            return true;
        }
    }

    // Java: void addFirst(T data)
    pub fn addFirst(&self, data: String) {
        let mut guard = self.node.lock().unwrap();
        // Java: Node<T> _node = this.node;
        // Java: if (null == _node)
        if guard.is_none()
        {
            // Java: _node = new Node<T>();
            let mut _node = BlockNode::new();
            // Java: _node.increaseRef();
            _node.increaseRef();
            // Java: _node.addFirst(data);
            _node.addFirst(data);
            *guard = Some(_node);
        }
        // Java: else
        else
        {
            let mut copied: Option<BlockNode> = None;
            // Java: synchronized(_node)
            {
                let node = guard.as_mut().unwrap();
                // Java: if (_node.refCount > 1)
                if node.refCount > 1
                {
                    // Java: _node = this.copyNode(_node);
                    copied = Some(Self::copyNode_static(node));
                }
                // Java: else
                else
                {
                    // Java: _node.addFirst(data);
                    node.addFirst(data.clone());
                }
            }
            // Java: if (_node != this.node)
            if let Some(ref mut new_node) = copied
            {
                // Java: _node.addFirst(data);
                new_node.addFirst(data);
                // Java: this.node.decreaseRef();
                if let Some(ref mut old) = *guard { old.decreaseRef(); }
                // Java: _node.increaseRef();
                new_node.increaseRef();
                // Java: this.node = _node;
                *guard = copied;
            }
        }
    }

    // Java: boolean remove(Object data)
    pub fn remove_obj(&self, data: &str) -> bool {
        let mut guard = self.node.lock().unwrap();
        // Java: Node<T> _node = this.node;
        // Java: if (null == _node)
        if guard.is_none()
        {
            // Java: return false;
            return false;
        }
        // Java: else
        else
        {
            // Java: boolean result = false;
            let mut result = false;
            let mut copied: Option<BlockNode> = None;
            // Java: synchronized(_node)
            {
                let node = guard.as_mut().unwrap();
                // Java: if (_node.refCount > 1)
                if node.refCount > 1
                {
                    // Java: _node = this.copyNode(_node);
                    copied = Some(Self::copyNode_static(node));
                }
                // Java: else
                else
                {
                    // Java: result = _node.remove(data);
                    result = node.remove_obj(data);
                }
            }
            // Java: if (_node != this.node)
            if let Some(ref mut new_node) = copied
            {
                // Java: result = _node.remove(data);
                result = new_node.remove_obj(data);
                // Java: this.node.decreaseRef();
                if let Some(ref mut old) = *guard { old.decreaseRef(); }
                // Java: _node.increaseRef();
                new_node.increaseRef();
                // Java: this.node = _node;
                *guard = copied;
            }
            // Java: return result;
            return result;
        }
    }

    // Java: boolean removeBackward(Object data)
    pub fn removeBackward_obj(&self, data: &str) -> bool {
        let mut guard = self.node.lock().unwrap();
        // Java: Node<T> _node = this.node;
        // Java: if (null == _node)
        if guard.is_none()
        {
            // Java: return false;
            return false;
        }
        // Java: else
        else
        {
            // Java: boolean result = false;
            let mut result = false;
            let mut copied: Option<BlockNode> = None;
            // Java: synchronized(_node)
            {
                let node = guard.as_mut().unwrap();
                // Java: if (_node.refCount > 1)
                if node.refCount > 1
                {
                    // Java: _node = this.copyNode(_node);
                    copied = Some(Self::copyNode_static(node));
                }
                // Java: else
                else
                {
                    // Java: result = _node.removeBackward(data);
                    result = node.removeBackward_obj(data);
                }
            }
            // Java: if (_node != this.node)
            if let Some(ref mut new_node) = copied
            {
                // Java: result = _node.removeBackward(data);
                result = new_node.removeBackward_obj(data);
                // Java: this.node.decreaseRef();
                if let Some(ref mut old) = *guard { old.decreaseRef(); }
                // Java: _node.increaseRef();
                new_node.increaseRef();
                // Java: this.node = _node;
                *guard = copied;
            }
            // Java: return result;
            return result;
        }
    }

    // Java: int removeAll(T data)
    pub fn removeAll(&self, data: &str) -> i32 {
        let mut guard = self.node.lock().unwrap();
        // Java: Node<T> _node = this.node;
        // Java: if (null == _node)
        if guard.is_none()
        {
            // Java: return 0;
            return 0;
        }
        // Java: else
        else
        {
            // Java: int count = 0;
            let mut count = 0i32;
            let mut copied: Option<BlockNode> = None;
            // Java: synchronized(_node)
            {
                let node = guard.as_mut().unwrap();
                // Java: if (_node.refCount > 1)
                if node.refCount > 1
                {
                    // Java: _node = this.copyNode(_node);
                    copied = Some(Self::copyNode_static(node));
                }
                // Java: else
                else
                {
                    // Java: count = _node.removeAll(data);
                    count = node.removeAll(data);
                }
            }
            // Java: if (_node != this.node)
            if let Some(ref mut new_node) = copied
            {
                // Java: count = _node.removeAll(data);
                count = new_node.removeAll(data);
                // Java: this.node.decreaseRef();
                if let Some(ref mut old) = *guard { old.decreaseRef(); }
                // Java: _node.increaseRef();
                new_node.increaseRef();
                // Java: this.node = _node;
                *guard = copied;
            }
            // Java: return count;
            return count;
        }
    }

    // Java: Node<T> increaseRefNode()
    fn increaseRefNode_internal(&self) -> Option<BlockNode> {
        let mut guard = self.node.lock().unwrap();
        // Java: if (null != this.node)
        if let Some(ref mut node) = *guard
        {
            // Java: this.node.increaseRef();
            node.increaseRef();
            // Java: return this.node;
            return Some(node.shallow_clone());
        }
        // Java: return this.node;
        None
    }

    // Java: boolean contains(Object data)
    pub fn contains(&self, data: &str) -> bool {
        // Java: Node<T> _node = this.increaseRefNode();
        let mut _node = self.increaseRefNode_internal();
        // Java: if (null == _node)
        if _node.is_none()
        {
            // Java: return false;
            return false;
        }
        // Java: else
        else
        {
            let node = _node.as_ref().unwrap();
            // Java: boolean result = _node.contains(data);
            let result = node.contains_obj(data);
            // Java: _node.decreaseRef();
            // Java: return result;
            return result;
        }
    }

    // Java: void insert(int index, T data)
    pub fn insert(&self, index: i32, data: String) {
        let mut guard = self.node.lock().unwrap();
        // Java: Node<T> _node = this.node;
        // Java: if (null == _node)
        if guard.is_none()
        {
            // Java: _node = new Node<T>();
            let mut _node = BlockNode::new();
            // Java: _node.increaseRef();
            _node.increaseRef();
            // Java: _node.insert(index, data);
            _node.insert(index, data);
            // Java: this.node = _node;
            *guard = Some(_node);
        }
        // Java: else
        else
        {
            let mut copied: Option<BlockNode> = None;
            // Java: synchronized(_node)
            {
                let node = guard.as_mut().unwrap();
                // Java: if (_node.refCount > 1)
                if node.refCount > 1
                {
                    // Java: _node = this.copyNode(_node);
                    copied = Some(Self::copyNode_static(node));
                }
                // Java: else
                else
                {
                    // Java: _node.insert(index, data);
                    node.insert(index, data.clone());
                }
            }
            // Java: if (_node != this.node)
            if let Some(ref mut new_node) = copied
            {
                // Java: _node.insert(index, data);
                new_node.insert(index, data);
                // Java: this.node.decreaseRef();
                if let Some(ref mut old) = *guard { old.decreaseRef(); }
                // Java: _node.increaseRef();
                new_node.increaseRef();
                // Java: this.node = _node;
                *guard = copied;
            }
        }
    }

    // Java: T remove(int index)
    pub fn remove_at(&self, index: i32) -> Option<String> {
        let mut guard = self.node.lock().unwrap();
        // Java: Node<T> _node = this.node;
        // Java: if (null == _node)
        if guard.is_none()
        {
            // Java: return null;
            return None;
        }
        // Java: else
        else
        {
            // Java: T v = null;
            let mut v: Option<String> = None;
            let mut copied: Option<BlockNode> = None;
            // Java: synchronized(_node)
            {
                let node = guard.as_mut().unwrap();
                // Java: if (_node.refCount > 1)
                if node.refCount > 1
                {
                    // Java: _node = this.copyNode(_node);
                    copied = Some(Self::copyNode_static(node));
                }
                // Java: else
                else
                {
                    // Java: v = _node.remove(index);
                    v = node.remove_at_idx(index);
                }
            }
            // Java: if (_node != this.node)
            if let Some(ref mut new_node) = copied
            {
                // Java: v = _node.remove(index);
                v = new_node.remove_at_idx(index);
                // Java: this.node.decreaseRef();
                if let Some(ref mut old) = *guard { old.decreaseRef(); }
                // Java: _node.increaseRef();
                new_node.increaseRef();
                // Java: this.node = _node;
                *guard = copied;
            }
            // Java: return v;
            return v;
        }
    }

    // Java: T removeBackward(int index)
    pub fn removeBackward_at(&self, index: i32) -> Option<String> {
        let mut guard = self.node.lock().unwrap();
        // Java: Node<T> _node = this.node;
        // Java: if (null == _node)
        if guard.is_none()
        {
            // Java: return null;
            return None;
        }
        // Java: else
        else
        {
            // Java: T v = null;
            let mut v: Option<String> = None;
            let mut copied: Option<BlockNode> = None;
            // Java: synchronized(_node)
            {
                let node = guard.as_mut().unwrap();
                // Java: if (_node.refCount > 1)
                if node.refCount > 1
                {
                    // Java: _node = this.copyNode(_node);
                    copied = Some(Self::copyNode_static(node));
                }
                // Java: else
                else
                {
                    // Java: v = _node.removeBackward(index);
                    v = node.removeBackward_at(index);
                }
            }
            // Java: if (_node != this.node)
            if let Some(ref mut new_node) = copied
            {
                // Java: v = _node.removeBackward(index);
                v = new_node.removeBackward_at(index);
                // Java: this.node.decreaseRef();
                if let Some(ref mut old) = *guard { old.decreaseRef(); }
                // Java: _node.increaseRef();
                new_node.increaseRef();
                // Java: this.node = _node;
                *guard = copied;
            }
            // Java: return v;
            return v;
        }
    }

    // Java: T set(int index, T data)
    pub fn set(&self, index: i32, data: String) -> Option<String> {
        let mut guard = self.node.lock().unwrap();
        // Java: Node<T> _node = this.node;
        // Java: if (null == _node)
        if guard.is_none()
        {
            // Java: return null;
            return None;
        }
        // Java: else
        else
        {
            // Java: Object v = null;
            let mut v: Option<String> = None;
            let mut copied: Option<BlockNode> = None;
            // Java: synchronized(_node)
            {
                let node = guard.as_mut().unwrap();
                // Java: if (_node.refCount > 1)
                if node.refCount > 1
                {
                    // Java: _node = this.copyNode(_node);
                    copied = Some(Self::copyNode_static(node));
                }
                // Java: else
                else
                {
                    // Java: v = _node.set(index, data);
                    v = node.set_at(index, data.clone());
                }
            }
            // Java: if (_node != this.node)
            if let Some(ref mut new_node) = copied
            {
                // Java: v = _node.set(index, data);
                v = new_node.set_at(index, data);
                // Java: this.node.decreaseRef();
                if let Some(ref mut old) = *guard { old.decreaseRef(); }
                // Java: _node.increaseRef();
                new_node.increaseRef();
                // Java: this.node = _node;
                *guard = copied;
            }
            // Java: return (T)v;
            return v;
        }
    }

    // Java: Node<T> copyNode(Node<T> node)
    fn copyNode_static(node: &BlockNode) -> BlockNode {
        // Java: Node<T> _node = new Node<T>();
        let mut _node = BlockNode::new();
        // Java: if (null != node.blocks)
        if let Some(ref blocks) = node.blocks
        {
            // Java: _node.blocks = new Block[node.blocks.length];
            let mut new_blocks = Vec::with_capacity(blocks.len());
            // Java: System.arraycopy(node.blocks, 0, _node.blocks, 0, node.blockSize);
            for i in 0..blocks.len() {
                if (i as i32) < node.blockSize {
                    new_blocks.push(blocks[i].clone());
                } else {
                    new_blocks.push(None);
                }
            }
            _node.blocks = Some(new_blocks);
        }
        // Java: _node.blockSize = node.blockSize;
        _node.blockSize = node.blockSize;
        // Java: _node.size = node.size;
        _node.size = node.size;
        // Java: return _node;
        _node
    }

    // Java: void clear()
    pub fn clear(&self) {
        let mut guard = self.node.lock().unwrap();
        // Java: Node<T> _node = this.node;
        // Java: if (null != _node)
        if guard.is_some()
        {
            // Java: synchronized(_node)
            {
                let node = guard.as_mut().unwrap();
                // Java: if (_node.refCount > 1)
                if node.refCount > 1
                {
                    // Java: _node.decreaseRef();
                    node.decreaseRef();
                    // Java: this.node = null;
                    *guard = None;
                }
                // Java: else
                else
                {
                    // Java: _node.clear();
                    node.clear();
                }
            }
        }
    }

    // Java: void release()
    pub fn release(&self) {
        // Java: this.clear();
        self.clear();
    }

    // Java: int size()
    pub fn size(&self) -> i32 {
        let guard = self.node.lock().unwrap();
        // Java: return null == this.node ? 0 : this.node.size;
        match *guard { None => 0, Some(ref n) => n.size }
    }

    // Java: T get(int index)
    pub fn get(&self, index: i32) -> Option<String> {
        // Java: Node<T> _node = this.increaseRefNode();
        let _node = self.increaseRefNode_internal();
        // Java: if (null == _node)
        if _node.is_none()
        {
            // Java: return null;
            return None;
        }
        // Java: else
        else
        {
            let node = _node.as_ref().unwrap();
            // Java: T v = _node.get(index);
            let v = node.get_at(index);
            // Java: _node.decreaseRef();
            // Java: return v;
            return v;
        }
    }

    // Java: T getBackward(int index)
    pub fn getBackward(&self, index: i32) -> Option<String> {
        // Java: Node<T> _node = this.increaseRefNode();
        let _node = self.increaseRefNode_internal();
        // Java: if (null == _node)
        if _node.is_none()
        {
            // Java: return null;
            return None;
        }
        // Java: else
        else
        {
            let node = _node.as_ref().unwrap();
            // Java: T v = _node.getBackward(index);
            let v = node.getBackward(index);
            // Java: _node.decreaseRef();
            // Java: return v;
            return v;
        }
    }

    // Java: ListIterator<T> listIterator(int index)
    pub fn listIterator_at(&self, index: i32) -> BlockListIterator {
        // Java: return new Iterator(index);
        BlockListIterator::new(index)
    }

    // Java: boolean isEmpty()
    pub fn isEmpty(&self) -> bool {
        // Java: return this.size() == 0;
        self.size() == 0
    }

    // Java: void add(int index, T element)
    pub fn add_at(&self, index: i32, element: String) {
        // Java: this.insert(index, element);
        self.insert(index, element);
    }

    // Java: int indexOf(Object o)
    pub fn indexOf(&self, o: &str) -> i32 {
        // Java: Node<T> _node = this.increaseRefNode();
        let _node = self.increaseRefNode_internal();
        // Java: if (null == _node)
        if _node.is_none()
        {
            // Java: return -1;
            return -1;
        }
        // Java: else
        else
        {
            let node = _node.as_ref().unwrap();
            // Java: int index = _node.indexOf(o);
            let index = node.indexOf(o);
            // Java: _node.decreaseRef();
            // Java: return index;
            return index;
        }
    }

    // Java: int lastIndexOf(Object o)
    pub fn lastIndexOf(&self, o: &str) -> i32 {
        // Java: Node<T> _node = this.increaseRefNode();
        let _node = self.increaseRefNode_internal();
        // Java: if (null == _node)
        if _node.is_none()
        {
            // Java: return -1;
            return -1;
        }
        // Java: else
        else
        {
            let node = _node.as_ref().unwrap();
            // Java: int index = _node.lastIndexOf(o);
            let index = node.lastIndexOf(o);
            // Java: _node.decreaseRef();
            // Java: return index;
            return index;
        }
    }

    // Java: ListIterator<T> listIterator()
    pub fn listIterator(&self) -> BlockListIterator {
        // Java: return new Iterator(0);
        BlockListIterator::new(0)
    }

    // Java: java.util.Iterator<T> iterator()
    pub fn iterator(&self) -> BlockListIterator {
        // Java: return this.listIterator();
        self.listIterator()
    }
}

// Java: public class Block
impl Block {
    // Java: Block()
    pub fn new() -> Self {
        // Java: super();
        // Java: this.data = new Object[32];
        Self { refCount: 0, size: 0, data: vec![String::new(); BLOCK_DATA_SIZE] }
    }

    // Java: void increaseRef()
    pub fn increaseRef(&mut self) {
        // Java: ++this.refCount;
        self.refCount += 1;
    }

    // Java: void decreaseRef()
    pub fn decreaseRef(&mut self) {
        // Java: if (--this.refCount <= 0)
        self.refCount -= 1;
        if self.refCount <= 0
        {
            // Java: CopyOnWriteBlockList.alloc.free(this);
        }
    }
}

// Java: public class RecycleAlloc
impl RecycleAlloc {
    // Java: RecycleAlloc()
    pub fn new() -> Self {
        // Java: super();
        // Java: this.localPool = new ArrayList();
        Self { localPool: Vec::new() }
    }

    // Java: Block alloc()
    pub fn alloc(&mut self) -> Block {
        // Java: if (this.localPool.isEmpty() && sSharePool.size() > 2)
        {
            // Java: synchronized(sSharePool)
            {
                // Java: for(int count = Math.min(sSharePool.size(), 2); count > 0; --count)
                {
                    // Java: this.localPool.add((Block)sSharePool.remove(sSharePool.size() - 1));
                }
            }
        }
        // Java: Block node = null;
        let node: Block;
        // Java: if (this.localPool.size() > 0)
        if !self.localPool.is_empty()
        {
            // Java: node = (Block)this.localPool.remove(this.localPool.size() - 1);
            node = self.localPool.pop().unwrap();
        }
        // Java: else
        else
        {
            // Java: node = new Block();
            node = Block::new();
        }
        // Java: return node;
        node
    }

    // Java: void free(Block node)
    pub fn free_recycle(&mut self, node: Block) {
        // Java: if (this.localPool.size() >= 4 && sSharePool.size() < 16)
        {
            // Java: synchronized(sSharePool)
            {
                // Java: for(int count = 2; count > 0; --count)
                {
                    // Java: sSharePool.add((Block)this.localPool.remove(this.localPool.size() - 1));
                }
            }
        }
        // Java: if (this.localPool.size() < 4)
        if (self.localPool.len() as i32) < LOCAL_CACHE_MAX
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
        Self { _alloc_count: 0, _free_count: 0, alloc_inner: None }
    }

    // Java: Block alloc()
    pub fn alloc(&mut self) -> Block {
        // Java: ++this._alloc_count;
        self._alloc_count += 1;
        // Java: RecycleAlloc _alloc = (RecycleAlloc)this.alloc.get();
        // Java: if (null == _alloc)
        if self.alloc_inner.is_none()
        {
            // Java: _alloc = new RecycleAlloc();
            // Java: this.alloc.set(_alloc);
            self.alloc_inner = Some(RecycleAlloc::new());
        }
        // Java: return _alloc.alloc();
        self.alloc_inner.as_mut().unwrap().alloc()
    }

    // Java: void free(Block node)
    pub fn free(&mut self, mut node: Block) {
        // Java: for(int i = node.size - 1; i >= 0; --i)
        let mut i = node.size - 1;
        while i >= 0
        {
            // Java: node.data[i] = null;
            node.data[i as usize] = String::new();
            i -= 1;
        }
        // Java: node.size = 0;
        node.size = 0;
        // Java: ++this._free_count;
        self._free_count += 1;
        // Java: RecycleAlloc _alloc = (RecycleAlloc)this.alloc.get();
        // Java: if (null != _alloc)
        if let Some(ref mut _alloc) = self.alloc_inner
        {
            // Java: _alloc.free(node);
            _alloc.free(node);
        }
    }
}

// Java: public class Node
impl BlockNode {
    fn new() -> Self {
        Self { refCount: 0, blocks: None, blockSize: 0, size: 0 }
    }

    fn shallow_clone(&self) -> Self {
        Self {
            refCount: self.refCount,
            blocks: self.blocks.clone(),
            blockSize: self.blockSize,
            size: self.size,
        }
    }

    // Java: void increaseRef()
    pub fn increaseRef(&mut self) {
        // Java: ++this.refCount;
        self.refCount += 1;
    }

    // Java: void decreaseRef()
    pub fn decreaseRef(&mut self) {
        // Java: --this.refCount;
        self.refCount -= 1;
    }

    // Java: void add(T data)
    pub fn add(&mut self, data: String) {
        // Java: Block block;
        let block_idx: usize;
        // Java: if (null == this.blocks)
        if self.blocks.is_none()
        {
            // Java: this.blocks = new Block[1];
            let mut b = Block::new();
            // Java: block = CopyOnWriteBlockList.alloc.alloc();
            // Java: block.increaseRef();
            b.increaseRef();
            // Java: this.blocks[0] = block;
            self.blocks = Some(vec![Some(b)]);
            // Java: this.blockSize = 1;
            self.blockSize = 1;
            block_idx = 0;
        }
        // Java: else
        else
        {
            let blocks = self.blocks.as_mut().unwrap();
            block_idx = (self.blockSize - 1) as usize;
            // Java: block = this.blocks[this.blockSize - 1];
            let block = blocks[block_idx].as_ref().unwrap();
            // Java: if (block.size >= block.data.length)
            if block.size >= block.data.len() as i32
            {
                // Java: if (this.blocks.length >= this.blockSize)
                if blocks.len() as i32 <= self.blockSize
                {
                    // Java: Block[] newBlocks = new Block[this.blocks.length * 2];
                    // Java: System.arraycopy(this.blocks, 0, newBlocks, 0, this.blockSize);
                    // Java: this.blocks = newBlocks;
                    blocks.resize(blocks.len() * 2, None);
                }
                // Java: block = CopyOnWriteBlockList.alloc.alloc();
                let mut new_b = Block::new();
                // Java: block.increaseRef();
                new_b.increaseRef();
                // Java: this.blocks[this.blockSize] = block;
                blocks[self.blockSize as usize] = Some(new_b);
                // Java: ++this.blockSize;
                self.blockSize += 1;
            }
        }
        let blocks = self.blocks.as_mut().unwrap();
        let bi = (self.blockSize - 1) as usize;
        let block = blocks[bi].as_mut().unwrap();
        // Java: if (block.refCount > 1)
        // Java: block = this.copyBlock(block, this.blockSize - 1);
        // Java: block.data[block.size] = data;
        let s = block.size as usize;
        if s < block.data.len() {
            block.data[s] = data;
        } else {
            block.data.push(data);
        }
        // Java: ++block.size;
        block.size += 1;
        // Java: ++this.size;
        self.size += 1;
    }

    // Java: void addFirst(T data)
    pub fn addFirst(&mut self, data: String) {
        // Java: Block block;
        // Java: if (null == this.blocks)
        if self.blocks.is_none()
        {
            // Java: this.blocks = new Block[1];
            let mut b = Block::new();
            // Java: block = CopyOnWriteBlockList.alloc.alloc();
            // Java: block.increaseRef();
            b.increaseRef();
            // Java: this.blocks[0] = block;
            self.blocks = Some(vec![Some(b)]);
            // Java: this.blockSize = 1;
            self.blockSize = 1;
        }
        // Java: else
        else
        {
            let blocks = self.blocks.as_mut().unwrap();
            // Java: block = this.blocks[0];
            let block = blocks[0].as_ref().unwrap();
            // Java: if (block.size >= block.data.length)
            if block.size >= block.data.len() as i32
            {
                // Java: if (this.blocks.length >= this.blockSize)
                if blocks.len() as i32 <= self.blockSize
                {
                    // Java: Block[] newBlocks = new Block[this.blocks.length * 2];
                    // Java: System.arraycopy(this.blocks, 0, newBlocks, 0, this.blockSize);
                    // Java: this.blocks = newBlocks;
                    blocks.resize(blocks.len() * 2, None);
                }
                // Java: block = CopyOnWriteBlockList.alloc.alloc();
                let mut new_b = Block::new();
                // Java: block.increaseRef();
                new_b.increaseRef();
                // Java: for(int i = this.blockSize; i > 0; --i)
                let mut i = self.blockSize;
                while i > 0
                {
                    // Java: this.blocks[i] = this.blocks[i - 1];
                    blocks[i as usize] = blocks[(i - 1) as usize].clone();
                    i -= 1;
                }
                // Java: this.blocks[0] = block;
                blocks[0] = Some(new_b);
                // Java: ++this.blockSize;
                self.blockSize += 1;
            }
        }
        let blocks = self.blocks.as_mut().unwrap();
        let block = blocks[0].as_mut().unwrap();
        // Java: if (block.refCount > 1)
        // Java: block = this.copyBlock(block, 0);
        // Java: for(int i = block.size; i > 0; --i)
        let mut i = block.size;
        while i > 0
        {
            // Java: block.data[i] = block.data[i - 1];
            if (i as usize) < block.data.len() {
                block.data[i as usize] = block.data[(i - 1) as usize].clone();
            }
            i -= 1;
        }
        // Java: block.data[0] = data;
        block.data[0] = data;
        // Java: ++block.size;
        block.size += 1;
        // Java: ++this.size;
        self.size += 1;
    }

    // Java: Block copyBlock(Block block, int blockIndex)
    fn copyBlock(&mut self, block_index: usize) -> Block {
        // Java: Block newBlock = CopyOnWriteBlockList.alloc.alloc();
        let blocks = self.blocks.as_mut().unwrap();
        let old = blocks[block_index].as_ref().unwrap();
        let mut new_block = Block::new();
        // Java: newBlock.increaseRef();
        new_block.increaseRef();
        // Java: System.arraycopy(block.data, 0, newBlock.data, 0, block.size);
        for i in 0..old.size as usize { new_block.data[i] = old.data[i].clone(); }
        // Java: newBlock.size = block.size;
        new_block.size = old.size;
        // Java: this.blocks[blockIndex] = newBlock;
        let ret = new_block.clone();
        blocks[block_index] = Some(new_block);
        // Java: block.decreaseRef();
        // Java: return newBlock;
        ret
    }

    // Java: boolean remove(Object data)
    pub fn remove_obj(&mut self, data: &str) -> bool {
        // Java: if (null == this.blocks)
        if self.blocks.is_none()
        {
            // Java: return false;
            return false;
        }
        // Java: else
        else
        {
            // Java: Block block = null;
            // Java: int blockIndex = -1;
            let mut block_index: i32 = -1;
            // Java: int index = -1;
            let mut index: i32 = -1;
            // Java: for(int i = 0; i < this.blockSize; ++i)
            let blocks = self.blocks.as_ref().unwrap();
            for i in 0..self.blockSize as usize
            {
                // Java: block = this.blocks[i];
                let block = blocks[i].as_ref().unwrap();
                // Java: for(int j = 0; j < block.size; ++j)
                for j in 0..block.size as usize
                {
                    // Java: Object d = block.data[j];
                    let d = &block.data[j];
                    // Java: if (d == data || null != d && d.equals(data))
                    if d == data
                    {
                        // Java: index = j;
                        index = j as i32;
                        // Java: break;
                        break;
                    }
                }
                // Java: if (index >= 0)
                if index >= 0
                {
                    // Java: blockIndex = i;
                    block_index = i as i32;
                    // Java: break;
                    break;
                }
            }
            // Java: if (index == -1)
            if index == -1
            {
                // Java: return false;
                return false;
            }
            // Java: else
            else
            {
                // Java: this.remove(blockIndex, index);
                self.remove_block_data(block_index, index);
                // Java: return true;
                return true;
            }
        }
    }

    // Java: int indexOf(Object data)
    pub fn indexOf(&self, data: &str) -> i32 {
        // Java: if (null == this.blocks)
        if self.blocks.is_none()
        {
            // Java: return -1;
            return -1;
        }
        // Java: else
        else
        {
            // Java: Block block = null;
            // Java: int currSize = 0;
            let mut curr_size = 0i32;
            let blocks = self.blocks.as_ref().unwrap();
            // Java: for(int i = 0; i < this.blockSize; ++i)
            for i in 0..self.blockSize as usize
            {
                // Java: block = this.blocks[i];
                let block = blocks[i].as_ref().unwrap();
                // Java: for(int j = 0; j < block.size; ++j)
                for j in 0..block.size as usize
                {
                    // Java: Object d = block.data[j];
                    let d = &block.data[j];
                    // Java: if (d == data || null != d && d.equals(data))
                    if d == data
                    {
                        // Java: return currSize + j;
                        return curr_size + j as i32;
                    }
                }
                // Java: currSize += block.size;
                curr_size += block.size;
            }
            // Java: return -1;
            return -1;
        }
    }

    // Java: int lastIndexOf(Object data)
    pub fn lastIndexOf(&self, data: &str) -> i32 {
        // Java: if (null == this.blocks)
        if self.blocks.is_none()
        {
            // Java: return -1;
            return -1;
        }
        // Java: else
        else
        {
            // Java: Block block = null;
            // Java: int currSize = 0;
            let mut curr_size = 0i32;
            let blocks = self.blocks.as_ref().unwrap();
            // Java: for(int i = this.blockSize - 1; i >= 0; --i)
            let mut i = self.blockSize - 1;
            while i >= 0
            {
                // Java: block = this.blocks[i];
                let block = blocks[i as usize].as_ref().unwrap();
                // Java: for(int j = block.size - 1; j >= 0; --j)
                let mut j = block.size - 1;
                while j >= 0
                {
                    // Java: Object d = block.data[j];
                    let d = &block.data[j as usize];
                    // Java: if (d == data || null != d && d.equals(data))
                    if d == data
                    {
                        // Java: return this.size - currSize - block.size + j;
                        return self.size - curr_size - block.size + j;
                    }
                    j -= 1;
                }
                // Java: currSize += block.size;
                curr_size += block.size;
                i -= 1;
            }
            // Java: return -1;
            return -1;
        }
    }

    // Java: boolean removeBackward(Object data)
    pub fn removeBackward_obj(&mut self, data: &str) -> bool {
        // Java: if (null == this.blocks)
        if self.blocks.is_none()
        {
            // Java: return false;
            return false;
        }
        // Java: else
        else
        {
            // Java: Block block = null;
            // Java: int blockIndex = -1;
            let mut block_index: i32 = -1;
            // Java: int index = -1;
            let mut index: i32 = -1;
            let blocks = self.blocks.as_ref().unwrap();
            // Java: for(int i = this.blockSize - 1; i >= 0; --i)
            let mut i = self.blockSize - 1;
            while i >= 0
            {
                // Java: block = this.blocks[i];
                let block = blocks[i as usize].as_ref().unwrap();
                // Java: for(int j = block.size - 1; j >= 0; --j)
                let mut j = block.size - 1;
                while j >= 0
                {
                    // Java: Object d = block.data[j];
                    let d = &block.data[j as usize];
                    // Java: if (d == data || null != d && d.equals(data))
                    if d == data
                    {
                        // Java: index = j;
                        index = j;
                        // Java: break;
                        break;
                    }
                    j -= 1;
                }
                // Java: if (index >= 0)
                if index >= 0
                {
                    // Java: blockIndex = i;
                    block_index = i;
                    // Java: break;
                    break;
                }
                i -= 1;
            }
            // Java: if (index == -1)
            if index == -1
            {
                // Java: return false;
                return false;
            }
            // Java: else
            else
            {
                // Java: this.remove(blockIndex, index);
                self.remove_block_data(block_index, index);
                // Java: return true;
                return true;
            }
        }
    }

    // Java: int removeAll(T data)
    pub fn removeAll(&mut self, data: &str) -> i32 {
        // Java: if (null == this.blocks)
        if self.blocks.is_none()
        {
            // Java: return 0;
            return 0;
        }
        // Java: else
        else
        {
            // Java: int removeCount = 0;
            let mut remove_count = 0i32;
            // Java: Block block = null;
            // Java: for(int i = 0; i < this.blockSize; ++i)
            let mut i = 0i32;
            while i < self.blockSize
            {
                // Java: block = this.blocks[i];
                // Java: int j = 0;
                let mut j = 0;
                // Java: for(int size = block.size; j < size; ++j)
                let mut size = self.blocks.as_ref().unwrap()[i as usize].as_ref().unwrap().size;
                while j < size
                {
                    // Java: Object d = block.data[j];
                    let d = self.blocks.as_ref().unwrap()[i as usize].as_ref().unwrap().data[j as usize].clone();
                    // Java: if (block == d || null != d && d.equals(data))
                    if d == data
                    {
                        // Java: this.remove(i, j);
                        self.remove_block_data(i, j);
                        // Java: ++removeCount;
                        remove_count += 1;
                        // Java: block = this.blocks[i];
                        // Java: --size;
                        size -= 1;
                        // Java: --j;
                        j -= 1;
                    }
                    j += 1;
                }
                i += 1;
            }
            // Java: return removeCount;
            return remove_count;
        }
    }

    // Java: boolean contains(Object data)
    pub fn contains_obj(&self, data: &str) -> bool {
        // Java: if (null == this.blocks)
        if self.blocks.is_none()
        {
            // Java: return false;
            return false;
        }
        // Java: else
        else
        {
            let blocks = self.blocks.as_ref().unwrap();
            // Java: for(int i = 0; i < this.blockSize; ++i)
            for i in 0..self.blockSize as usize
            {
                // Java: Block block = this.blocks[i];
                let block = blocks[i].as_ref().unwrap();
                // Java: for(int j = 0; j < block.size; ++j)
                for j in 0..block.size as usize
                {
                    // Java: Object d = block.data[j];
                    let d = &block.data[j];
                    // Java: if (data == d || null != d && d.equals(data))
                    if d == data
                    {
                        // Java: return true;
                        return true;
                    }
                }
            }
            // Java: return false;
            return false;
        }
    }

    // Java: void insert(int index, T data)
    pub fn insert(&mut self, index: i32, data: String) {
        // Java: if (index <= 0)
        if index <= 0
        {
            // Java: this.addFirst(data);
            self.addFirst(data);
        }
        // Java: else if (index >= this.size)
        else if index >= self.size
        {
            // Java: this.add(data);
            self.add(data);
        }
        // Java: else
        else
        {
            // simplified: use flat insert logic
            // Java: Block block = null;
            // Java: int blockIndex = -1;
            // Java: int dataIndex = -1;
            // Java: int currSize = 0;
            let mut block_index: i32 = -1;
            let mut data_index: i32 = -1;
            let mut curr_size = 0i32;
            // Java: for(int i = 0; i < this.blockSize; ++i)
            {
                let blocks = self.blocks.as_ref().unwrap();
                for i in 0..self.blockSize as usize
                {
                    // Java: block = this.blocks[i];
                    let block = blocks[i].as_ref().unwrap();
                    // Java: if (index < currSize + block.size)
                    if index < curr_size + block.size
                    {
                        // Java: blockIndex = i;
                        block_index = i as i32;
                        // Java: dataIndex = index - currSize;
                        data_index = index - curr_size;
                        // Java: break;
                        break;
                    }
                    // Java: currSize += block.size;
                    curr_size += block.size;
                }
            }
            if block_index < 0 { return; }
            // Java: if (block.size >= 32)
            {
                // Java: if (blockIndex < this.blockSize - 1 && this.blocks[blockIndex + 1].size < 24)
                {
                    // Java: Block dstBlock = this.blocks[blockIndex + 1];
                    // Java: int moveSize = Math.max(1, (32 - dstBlock.size) / 2);
                    // Java: this.blockDataMove(blockIndex, moveSize, false);
                    // Java: block = this.blocks[blockIndex];
                    // Java: if (dataIndex >= block.size)
                    {
                        // Java: dataIndex -= block.size;
                        // Java: ++blockIndex;
                        // Java: block = this.blocks[blockIndex];
                    }
                }
                // Java: else if (blockIndex > 0 && this.blocks[blockIndex - 1].size < 24)
                {
                    // Java: Block dstBlock = this.blocks[blockIndex - 1];
                    // Java: int moveSize = Math.max(1, (32 - dstBlock.size) / 2);
                    // Java: this.blockDataMove(blockIndex, moveSize, true);
                    // Java: if (dataIndex < moveSize)
                    {
                        // Java: --blockIndex;
                        // Java: block = this.blocks[blockIndex];
                        // Java: dataIndex += block.size - moveSize;
                    }
                    // Java: else
                    {
                        // Java: block = this.blocks[blockIndex];
                        // Java: dataIndex -= moveSize;
                    }
                }
                // Java: else
                {
                    // Java: if (this.blockSize >= this.blocks.length)
                    {
                        // Java: Block[] newBlocks = new Block[this.blocks.length * 2];
                        // Java: System.arraycopy(this.blocks, 0, newBlocks, 0, this.blockSize);
                        // Java: this.blocks = newBlocks;
                    }
                    // Java: Block _block = CopyOnWriteBlockList.alloc.alloc();
                    // Java: _block.increaseRef();
                    // Java: for(int i = this.blockSize - 1; i > blockIndex; --i)
                    {
                        // Java: this.blocks[i + 1] = this.blocks[i];
                    }
                    // Java: this.blocks[blockIndex + 1] = _block;
                    // Java: ++this.blockSize;
                    // Java: int moveSize = 16;
                    // Java: this.blockDataMove(blockIndex, moveSize, false);
                    // Java: block = this.blocks[blockIndex];
                    // Java: if (dataIndex >= block.size)
                    {
                        // Java: dataIndex -= block.size;
                        // Java: ++blockIndex;
                        // Java: block = this.blocks[blockIndex];
                    }
                }
            }
            // Java: if (block.refCount > 1)
            {
                // Java: block = this.copyBlock(block, blockIndex);
            }
            // Java: for(int i = block.size; i > dataIndex; --i)
            {
                // Java: block.data[i] = block.data[i - 1];
            }
            // Java: block.data[dataIndex] = data;
            let blocks = self.blocks.as_mut().unwrap();
            let block = blocks[block_index as usize].as_mut().unwrap();
            let di = data_index as usize;
            if block.size as usize >= block.data.len() { block.data.push(String::new()); }
            let mut i = block.size;
            while i > data_index {
                block.data[i as usize] = block.data[(i - 1) as usize].clone();
                i -= 1;
            }
            block.data[di] = data;
            // Java: ++block.size;
            block.size += 1;
            // Java: ++this.size;
            self.size += 1;
        }
    }

    // Java: void blockDataMove(int blockIndex, int count, boolean moveFront)
    fn blockDataMove(&mut self, _block_index: i32, _count: i32, _move_front: bool) {
        // Java: if (moveFront)
        {
            // Java: Block srcBlock = this.blocks[blockIndex];
            // Java: Block dstBlock = this.blocks[blockIndex - 1];
            // Java: if (srcBlock.refCount > 1)
            {
                // Java: srcBlock = this.copyBlock(srcBlock, blockIndex);
            }
            // Java: if (dstBlock.refCount > 1)
            {
                // Java: dstBlock = this.copyBlock(dstBlock, blockIndex - 1);
            }
            // Java: System.arraycopy(srcBlock.data, 0, dstBlock.data, dstBlock.size, count);
            // Java: dstBlock.size += count;
            // Java: int i = count;
            // Java: for(int size = srcBlock.size; i < size; ++i)
            {
                // Java: srcBlock.data[i - count] = srcBlock.data[i];
            }
            // Java: i = srcBlock.size - count;
            // Java: for(int size = srcBlock.size; i < size; ++i)
            {
                // Java: srcBlock.data[i] = null;
            }
            // Java: srcBlock.size -= count;
        }
        // Java: else
        {
            // Java: Block srcBlock = this.blocks[blockIndex];
            // Java: Block dstBlock = this.blocks[blockIndex + 1];
            // Java: if (srcBlock.refCount > 1)
            {
                // Java: srcBlock = this.copyBlock(srcBlock, blockIndex);
            }
            // Java: if (dstBlock.refCount > 1)
            {
                // Java: dstBlock = this.copyBlock(dstBlock, blockIndex + 1);
            }
            // Java: for(int i = dstBlock.size + count - 1; i >= count; --i)
            {
                // Java: dstBlock.data[i] = dstBlock.data[i - count];
            }
            // Java: dstBlock.size += count;
            // Java: System.arraycopy(srcBlock.data, srcBlock.size - count, dstBlock.data, 0, count);
            // Java: int i = srcBlock.size - count;
            // Java: for(int size = srcBlock.size; i < size; ++i)
            {
                // Java: srcBlock.data[i] = null;
            }
            // Java: srcBlock.size -= count;
        }
    }

    // Java: Object set(int index, Object data)
    pub fn set_at(&mut self, index: i32, data: String) -> Option<String> {
        // Java: if (index >= 0 && index < this.size)
        if index >= 0 && index < self.size
        {
            // Java: Block block = null;
            // Java: int blockIndex = -1;
            // Java: int dataIndex = -1;
            // Java: int currSize = 0;
            let mut block_index: i32 = -1;
            let mut data_index: i32 = -1;
            let mut curr_size = 0i32;
            let blocks = self.blocks.as_mut().unwrap();
            // Java: for(int i = 0; i < this.blockSize; ++i)
            for i in 0..self.blockSize as usize
            {
                // Java: block = this.blocks[i];
                let block = blocks[i].as_ref().unwrap();
                // Java: if (index < currSize + block.size)
                if index < curr_size + block.size
                {
                    // Java: blockIndex = i;
                    block_index = i as i32;
                    // Java: dataIndex = index - currSize;
                    data_index = index - curr_size;
                    // Java: break;
                    break;
                }
                // Java: currSize += block.size;
                curr_size += block.size;
            }
            // Java: if (block.refCount > 1)
            // Java: block = this.copyBlock(block, blockIndex);
            let block = blocks[block_index as usize].as_mut().unwrap();
            // Java: Object oldValue = block.data[dataIndex];
            let old_value = block.data[data_index as usize].clone();
            // Java: block.data[dataIndex] = data;
            block.data[data_index as usize] = data;
            // Java: return oldValue;
            return Some(old_value);
        }
        // Java: else
        else
        {
            // Java: return null;
            return None;
        }
    }

    // Java: T remove(int index)
    pub fn remove_at_idx(&mut self, index: i32) -> Option<String> {
        // Java: if (index >= 0 && index < this.size)
        if index >= 0 && index < self.size
        {
            // Java: Block block = null;
            // Java: int blockIndex = -1;
            // Java: int dataIndex = -1;
            // Java: int currSize = 0;
            let mut block_index: i32 = -1;
            let mut data_index: i32 = -1;
            let mut curr_size = 0i32;
            {
                let blocks = self.blocks.as_ref().unwrap();
                // Java: for(int i = 0; i < this.blockSize; ++i)
                for i in 0..self.blockSize as usize
                {
                    // Java: block = this.blocks[i];
                    let block = blocks[i].as_ref().unwrap();
                    // Java: if (index < currSize + block.size)
                    if index < curr_size + block.size
                    {
                        // Java: blockIndex = i;
                        block_index = i as i32;
                        // Java: dataIndex = index - currSize;
                        data_index = index - curr_size;
                        // Java: break;
                        break;
                    }
                    // Java: currSize += block.size;
                    curr_size += block.size;
                }
            }
            // Java: return (T)this.remove(blockIndex, dataIndex);
            return self.remove_block_data(block_index, data_index);
        }
        // Java: else
        else
        {
            // Java: return null;
            return None;
        }
    }

    // Java: T removeBackward(int index)
    pub fn removeBackward_at(&mut self, index: i32) -> Option<String> {
        // Java: if (index >= 0 && index < this.size)
        if index >= 0 && index < self.size
        {
            // Java: Block block = null;
            // Java: int blockIndex = -1;
            // Java: int dataIndex = -1;
            // Java: int currSize = 0;
            let mut block_index: i32 = -1;
            let mut data_index: i32 = -1;
            let mut curr_size = 0i32;
            {
                let blocks = self.blocks.as_ref().unwrap();
                // Java: for(int i = this.blockSize - 1; i >= 0; --i)
                let mut i = self.blockSize - 1;
                while i >= 0
                {
                    // Java: block = this.blocks[i];
                    let block = blocks[i as usize].as_ref().unwrap();
                    // Java: if (index < currSize + block.size)
                    if index < curr_size + block.size
                    {
                        // Java: blockIndex = i;
                        block_index = i;
                        // Java: dataIndex = block.size - 1 - (index - currSize);
                        data_index = block.size - 1 - (index - curr_size);
                        // Java: break;
                        break;
                    }
                    // Java: currSize += block.size;
                    curr_size += block.size;
                    i -= 1;
                }
            }
            // Java: return (T)this.remove(blockIndex, dataIndex);
            return self.remove_block_data(block_index, data_index);
        }
        // Java: else
        else
        {
            // Java: return null;
            return None;
        }
    }

    // Java: T remove(int blockIndex, int dataIndex)
    fn remove_block_data(&mut self, block_index: i32, data_index: i32) -> Option<String> {
        let blocks = self.blocks.as_mut().unwrap();
        // Java: Block block = this.blocks[blockIndex];
        let block = blocks[block_index as usize].as_ref().unwrap();
        // Java: if (block.size == 1 && this.blockSize > 1)
        if block.size == 1 && self.blockSize > 1
        {
            // Java: T oldValue = (T)block.data[dataIndex];
            let old_value = block.data[data_index as usize].clone();
            // Java: block.decreaseRef();
            // Java: for(int i = blockIndex + 1; i < this.blockSize; ++i)
            for i in (block_index + 1)..self.blockSize
            {
                // Java: this.blocks[i - 1] = this.blocks[i];
                blocks[(i - 1) as usize] = blocks[i as usize].clone();
            }
            // Java: --this.blockSize;
            self.blockSize -= 1;
            // Java: this.blocks[this.blockSize] = null;
            blocks[self.blockSize as usize] = None;
            // Java: --this.size;
            self.size -= 1;
            // Java: return oldValue;
            return Some(old_value);
        }
        // Java: else
        else
        {
            // Java: if (block.refCount > 1)
            // Java: block = this.copyBlock(block, blockIndex);
            let block = blocks[block_index as usize].as_mut().unwrap();
            // Java: T oldValue = (T)block.data[dataIndex];
            let old_value = block.data[data_index as usize].clone();
            // Java: int i = dataIndex + 1;
            let mut i = data_index + 1;
            // Java: for(int size = block.size; i < size; ++i)
            while i < block.size
            {
                // Java: block.data[i - 1] = block.data[i];
                block.data[(i - 1) as usize] = block.data[i as usize].clone();
                i += 1;
            }
            // Java: --block.size;
            block.size -= 1;
            // Java: block.data[block.size] = null;
            block.data[block.size as usize] = String::new();
            // Java: --this.size;
            self.size -= 1;
            // Java: return oldValue;
            return Some(old_value);
        }
    }

    // Java: void clear()
    pub fn clear(&mut self) {
        // Java: if (null != this.blocks)
        if self.blocks.is_some()
        {
            // Java: for(int i = 0; i < this.blockSize; ++i)
            let blocks = self.blocks.as_mut().unwrap();
            for i in 0..self.blockSize as usize
            {
                // Java: Block block = this.blocks[i];
                // Java: this.blocks[i] = null;
                blocks[i] = None;
                // Java: block.decreaseRef();
            }
            // Java: this.blockSize = 0;
            self.blockSize = 0;
            // Java: this.size = 0;
            self.size = 0;
        }
    }

    // Java: T get(int index)
    pub fn get_at(&self, index: i32) -> Option<String> {
        // Java: if (index >= 0 && index < this.size)
        if index >= 0 && index < self.size
        {
            // Java: int currSize = 0;
            let mut curr_size = 0i32;
            let blocks = self.blocks.as_ref().unwrap();
            // Java: for(int i = 0; i < this.blockSize; ++i)
            for i in 0..self.blockSize as usize
            {
                // Java: Block block = this.blocks[i];
                let block = blocks[i].as_ref().unwrap();
                // Java: if (index < currSize + block.size)
                if index < curr_size + block.size
                {
                    // Java: return (T)block.data[index - currSize];
                    return Some(block.data[(index - curr_size) as usize].clone());
                }
                // Java: currSize += block.size;
                curr_size += block.size;
            }
            // Java: return null;
            return None;
        }
        // Java: else
        else
        {
            // Java: return null;
            return None;
        }
    }

    // Java: T getBackward(int index)
    pub fn getBackward(&self, index: i32) -> Option<String> {
        // Java: if (index >= 0 && index < this.size)
        if index >= 0 && index < self.size
        {
            // Java: int currSize = 0;
            let mut curr_size = 0i32;
            let blocks = self.blocks.as_ref().unwrap();
            // Java: for(int i = 0; i < this.blockSize; ++i)
            for i in 0..self.blockSize as usize
            {
                // Java: Block block = this.blocks[i];
                let block = blocks[i].as_ref().unwrap();
                // Java: if (index < currSize + block.size)
                if index < curr_size + block.size
                {
                    // Java: return (T)block.data[block.size - 1 - (index - currSize)];
                    return Some(block.data[(block.size - 1 - (index - curr_size)) as usize].clone());
                }
                // Java: currSize += block.size;
                curr_size += block.size;
            }
            // Java: return null;
            return None;
        }
        // Java: else
        else
        {
            // Java: return null;
            return None;
        }
    }
}

// Java: public class Iterator
impl BlockListIterator {
    // Java: Iterator(int index)
    pub fn new(index: i32) -> Self {
        // Java: super();
        // Java: this.index = index - 1;
        Self { index: index - 1 }
    }

    // Java: boolean hasNext()
    pub fn hasNext(&self, listsize: i32) -> bool {
        // Java: return this.index + 1 < CopyOnWriteBlockList.this.size();
        self.index + 1 < list_size
    }

    // Java: T next()
    pub fn next(&mut self, list: &CopyOnWriteBlockList) -> Option<String> {
        // Java: ++this.index;
        self.index += 1;
        // Java: return (T)CopyOnWriteBlockList.this.get(this.index);
        list.get(self.index)
    }

    // Java: boolean hasPrevious()
    pub fn hasPrevious(&self, listsize: i32) -> bool {
        // Java: return CopyOnWriteBlockList.this.size() > 0 && this.index >= 0;
        list_size > 0 && self.index >= 0
    }

    // Java: T previous()
    pub fn previous(&mut self, list: &CopyOnWriteBlockList) -> Option<String> {
        // Java: T v = (T)CopyOnWriteBlockList.this.get(this.index);
        let v = list.get(self.index);
        // Java: --this.index;
        self.index -= 1;
        // Java: return v;
        v
    }

    // Java: int nextIndex()
    pub fn nextIndex(&self) -> i32 {
        // Java: return this.index + 1;
        self.index + 1
    }

    // Java: int previousIndex()
    pub fn previousIndex(&self) -> i32 {
        // Java: return this.index;
        self.index
    }

    // Java: void remove()
    pub fn remove(&self, list: &CopyOnWriteBlockList) {
        // Java: CopyOnWriteBlockList.this.remove(this.index);
        list.remove_at(self.index);
    }

    // Java: void set(T e)
    pub fn set(&self, list: &CopyOnWriteBlockList, e: String) {
        // Java: CopyOnWriteBlockList.this.set(this.index, e);
        list.set(self.index, e);
    }

    // Java: void add(T e)
    pub fn add(&self, list: &CopyOnWriteBlockList, e: String) {
        // Java: CopyOnWriteBlockList.this.insert(this.index, e);
        list.insert(self.index, e);
    }
}
