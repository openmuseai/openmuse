#![allow(non_snake_case, dead_code, unused_variables, unused_mut, unused_imports, unused_assignments)]

use std::sync::Mutex;

// Java field: long serialVersionUID
const SERIAL_VERSION_UID: i64 = -3706626890315791729;
// Java field: long serialVersionUID
const NODE_SERIAL_VERSION_UID: i64 = -6707527913614105488;

// Java: public class Node
pub struct Node<T: Clone + PartialEq> {
    // Java field: int refCount
    refCount: i32,
    data: Vec<T>,
}

// Java: public class Node
impl<T: Clone + PartialEq> Node<T> {
    // Java: Node()
    pub fn new() -> Self {
        // Java: super();
        Self { refCount: 0, data: Vec::new() }
    }

    // Java: Node(Node<T> node)
    pub fn from_node(node: &Node<T>) -> Self {
        // Java: super(node);
        Self { refCount: 0, data: node.data.clone() }
    }

    // Java: Node(int capacity)
    pub fn with_capacity(capacity: usize) -> Self {
        // Java: super(capacity);
        Self { refCount: 0, data: Vec::with_capacity(capacity) }
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

    // Java: void removeRange(int fromIndex, int toIndex)
    pub fn removeRange(&mut self, from_index: usize, to_index: usize) {
        // Java: super.removeRange(fromIndex, toIndex);
        self.data.drain(from_index..to_index);
    }

    pub fn add_at(&mut self, index: usize, data: T) { self.data.insert(index, data); }
    pub fn remove_obj(&mut self, data: &T) -> bool {
        if let Some(pos) = self.data.iter().position(|x| x == data) { self.data.remove(pos); true } else { false }
    }
    pub fn remove_at(&mut self, index: usize) -> T { self.data.remove(index) }
    pub fn contains_obj(&self, data: &T) -> bool { self.data.contains(data) }
    pub fn get_at(&self, index: usize) -> &T { &self.data[index] }
    pub fn set_at(&mut self, index: usize, element: T) -> T { std::mem::replace(&mut self.data[index], element) }
    pub fn size(&self) -> i32 { self.data.len() as i32 }
    pub fn clear_data(&mut self) { self.data.clear(); }
    pub fn indexOf_obj(&self, o: &T) -> i32 { self.data.iter().position(|x| x == o).map(|i| i as i32).unwrap_or(-1) }
    pub fn lastIndexOf_obj(&self, o: &T) -> i32 { self.data.iter().rposition(|x| x == o).map(|i| i as i32).unwrap_or(-1) }
    pub fn toArray(&self) -> Vec<T> { self.data.clone() }
}

// Java: public class CopyOnWriteArrayList extends AbstractList implements List, RandomAccess, Cloneable, Serializable
pub struct CopyOnWriteArrayList<T: Clone + PartialEq> {
    // Java field: Node<T> node
    node: Mutex<Option<Node<T>>>,
}

// Java: public class CopyOnWriteArrayList extends AbstractList implements List, RandomAccess, Cloneable, Serializable
impl<T: Clone + PartialEq> CopyOnWriteArrayList<T> {
    // Java: CopyOnWriteArrayList()
    pub fn new() -> Self {
        // Java: super();
        Self { node: Mutex::new(None) }
    }

    // Java: CopyOnWriteArrayList(int capacity)
    pub fn with_capacity(capacity: usize) -> Self {
        // Java: super();
        let mut n = Node::with_capacity(capacity);
        // Java: this.node = new Node<T>(capacity);
        // Java: this.node.increaseRef();
        n.increaseRef();
        Self { node: Mutex::new(Some(n)) }
    }

    // Java: CopyOnWriteArrayList(CopyOnWriteArrayList<T> list)
    pub fn from_list(list: &CopyOnWriteArrayList<T>) -> Self {
        // Java: super();
        // Java: this.node = list.increaseRefNode();
        let n = list.increaseRefNode_internal();
        Self { node: Mutex::new(n) }
    }

    // Java: CopyOnWriteArrayList<T> clone()
    pub fn clone_list(&self) -> CopyOnWriteArrayList<T> {
        // Java: return new CopyOnWriteArrayList<T>(this);
        CopyOnWriteArrayList::from_list(self)
    }

    // Java: boolean remove(Object data)
    pub fn remove_obj(&self, data: &T) -> bool {
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
            let mut copied: Option<Node<T>> = None;
            // Java: synchronized(_node)
            {
                let node = guard.as_mut().unwrap();
                // Java: if (_node.refCount > 1)
                if node.refCount > 1
                {
                    // Java: _node = this.copyNode(_node);
                    copied = Some(Node::from_node(node));
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

    // Java: Node<T> increaseRefNode()
    fn increaseRefNode_internal(&self) -> Option<Node<T>> {
        let mut guard = self.node.lock().unwrap();
        // Java: if (null != this.node)
        if let Some(ref mut node) = *guard
        {
            // Java: this.node.increaseRef();
            node.increaseRef();
            // Java: return this.node;
            return Some(Node { refCount: 1, data: node.data.clone() });
        }
        // Java: return this.node;
        None
    }

    // Java: boolean contains(Object data)
    pub fn contains(&self, data: &T) -> bool {
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

    // Java: void add(int index, T data)
    pub fn add(&self, index: usize, data: T) {
        let mut guard = self.node.lock().unwrap();
        // Java: Node<T> _node = this.node;
        // Java: if (null == _node)
        if guard.is_none()
        {
            // Java: _node = new Node<T>();
            let mut _node = Node::new();
            // Java: _node.increaseRef();
            _node.increaseRef();
            // Java: _node.add(index, data);
            _node.add_at(index, data);
            // Java: this.node = _node;
            *guard = Some(_node);
        }
        // Java: else
        else
        {
            let mut copied: Option<Node<T>> = None;
            // Java: synchronized(_node)
            {
                let node = guard.as_mut().unwrap();
                // Java: if (_node.refCount > 1)
                if node.refCount > 1
                {
                    // Java: _node = this.copyNode(_node);
                    copied = Some(Node::from_node(node));
                }
                // Java: else
                else
                {
                    // Java: _node.add(index, data);
                    node.add_at(index, data.clone());
                }
            }
            // Java: if (_node != this.node)
            if let Some(ref mut new_node) = copied
            {
                // Java: _node.add(index, data);
                new_node.add_at(index, data);
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
    pub fn remove_at(&self, index: usize) -> T {
        let mut guard = self.node.lock().unwrap();
        // Java: Node<T> _node = this.node;
        // Java: if (null == _node)
        if guard.is_none()
        {
            // Java: throw new IndexOutOfBoundsException(this.outOfBoundsMsg(index));
            panic!("{}", Self::outOfBoundsMsg(index as i32, 0));
        }
        // Java: else
        else
        {
            // Java: T v = null;
            let mut v: Option<T> = None;
            let mut copied: Option<Node<T>> = None;
            // Java: synchronized(_node)
            {
                let node = guard.as_mut().unwrap();
                // Java: if (_node.refCount > 1)
                if node.refCount > 1
                {
                    // Java: _node = this.copyNode(_node);
                    copied = Some(Node::from_node(node));
                }
                // Java: else
                else
                {
                    // Java: v = (T)_node.remove(index);
                    v = Some(node.remove_at(index));
                }
            }
            // Java: if (_node != this.node)
            if let Some(ref mut new_node) = copied
            {
                // Java: v = (T)_node.remove(index);
                v = Some(new_node.remove_at(index));
                // Java: this.node.decreaseRef();
                if let Some(ref mut old) = *guard { old.decreaseRef(); }
                // Java: _node.increaseRef();
                new_node.increaseRef();
                // Java: this.node = _node;
                *guard = copied;
            }
            // Java: return v;
            return v.unwrap();
        }
    }

    // Java: Node<T> copyNode(Node<T> node)
    fn copyNode(node: &Node<T>) -> Node<T> {
        // Java: return new Node<T>(node);
        Node::from_node(node)
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
                    node.clear_data();
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
        // Java: return null == this.node ? 0 : this.node.size();
        match *guard {
            None => 0,
            Some(ref node) => node.size(),
        }
    }

    // Java: boolean isEmpty()
    pub fn isEmpty(&self) -> bool {
        // Java: return this.size() == 0;
        self.size() == 0
    }

    // Java: T get(int index)
    pub fn get(&self, index: usize) -> T {
        // Java: Node<T> _node = this.increaseRefNode();
        let _node = self.increaseRefNode_internal();
        // Java: if (null == _node)
        if _node.is_none()
        {
            // Java: throw new IndexOutOfBoundsException(this.outOfBoundsMsg(index));
            panic!("{}", Self::outOfBoundsMsg(index as i32, 0));
        }
        // Java: else
        else
        {
            let node = _node.as_ref().unwrap();
            // Java: T v = (T)_node.get(index);
            let v = node.get_at(index).clone();
            // Java: _node.decreaseRef();
            // Java: return v;
            return v;
        }
    }

    // Java: int indexOf(Object o)
    pub fn indexOf(&self, o: &T) -> i32 {
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
            let index = node.indexOf_obj(o);
            // Java: _node.decreaseRef();
            // Java: return index;
            return index;
        }
    }

    // Java: int lastIndexOf(Object o)
    pub fn lastIndexOf(&self, o: &T) -> i32 {
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
            let index = node.lastIndexOf_obj(o);
            // Java: _node.decreaseRef();
            // Java: return index;
            return index;
        }
    }

    // Java: void removeRange(int fromIndex, int toIndex)
    pub fn removeRange(&self, from_index: usize, to_index: usize) {
        let mut guard = self.node.lock().unwrap();
        // Java: Node<T> _node = this.node;
        // Java: if (null != _node)
        if guard.is_some()
        {
            let mut copied: Option<Node<T>> = None;
            // Java: synchronized(_node)
            {
                let node = guard.as_mut().unwrap();
                // Java: if (_node.refCount > 1)
                if node.refCount > 1
                {
                    // Java: _node = this.copyNode(_node);
                    copied = Some(Node::from_node(node));
                }
                // Java: else
                else
                {
                    // Java: _node.removeRange(fromIndex, toIndex);
                    node.removeRange(from_index, to_index);
                }
            }
            // Java: if (_node != this.node)
            if let Some(ref mut new_node) = copied
            {
                // Java: _node.removeRange(fromIndex, toIndex);
                new_node.removeRange(from_index, to_index);
                // Java: this.node.decreaseRef();
                if let Some(ref mut old) = *guard { old.decreaseRef(); }
                // Java: _node.increaseRef();
                new_node.increaseRef();
                // Java: this.node = _node;
                *guard = copied;
            }
        }
    }

    // Java: Object[] toArray()
    pub fn toArray(&self) -> Vec<T> {
        // Java: Node<T> _node = this.increaseRefNode();
        let _node = self.increaseRefNode_internal();
        // Java: if (null == _node)
        if _node.is_none()
        {
            // Java: return new Object[0];
            return Vec::new();
        }
        // Java: else
        else
        {
            let node = _node.as_ref().unwrap();
            // Java: Object[] array = _node.toArray();
            let array = node.toArray();
            // Java: _node.decreaseRef();
            // Java: return array;
            return array;
        }
    }

    // Java: public <T> T[] toArray(T[] a)
    pub fn toArray_with_arr(&self, a: &mut Vec<T>) -> Vec<T> {
        // Java: Node _node = this.increaseRefNode();
        let _node = self.increaseRefNode_internal();
        // Java: if (null == _node)
        if _node.is_none()
        {
            // Java: if (a.length > 0)
            if !a.is_empty()
            {
                // Java: a[0] = null;
                a.clear();
            }
            // Java: return a;
            return a.clone();
        }
        // Java: else
        else
        {
            let node = _node.as_ref().unwrap();
            // Java: Object[] array = _node.toArray(a);
            let array = node.toArray();
            // Java: _node.decreaseRef();
            // Java: return (T[])array;
            return array;
        }
    }

    // Java: String outOfBoundsMsg(int index)
    fn outOfBoundsMsg(index: i32, size: i32) -> String {
        // Java: return "Index: " + index + ", Size: " + this.size();
        format!("Index: {}, Size: {}", index, size)
    }

    // Java: T set(int index, T element)
    pub fn set(&self, index: usize, element: T) -> T {
        let mut guard = self.node.lock().unwrap();
        // Java: Node<T> _node = this.increaseRefNode();
        // Java: if (null == _node)
        if guard.is_none()
        {
            // Java: throw new IndexOutOfBoundsException(this.outOfBoundsMsg(index));
            panic!("{}", Self::outOfBoundsMsg(index as i32, 0));
        }
        // Java: else
        else
        {
            let node = guard.as_mut().unwrap();
            // Java: T oldValue = (T)_node.set(index, element);
            let old_value = node.set_at(index, element);
            // Java: _node.decreaseRef();
            // Java: return oldValue;
            return old_value;
        }
    }
}
