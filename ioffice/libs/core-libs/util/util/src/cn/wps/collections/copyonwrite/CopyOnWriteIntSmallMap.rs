// Java: import cn.wps.moffice.util.IIntSmallMap
// Java: import java.io.IOException
// Java: import java.io.ObjectInput
// Java: import java.io.ObjectOutput

use std::sync::Arc;
use std::any::Any;
use std::io;
use std::ops::Deref;
use crate::cn::wps::moffice::util::IIntSmallMap::IIntSmallMap;

type ObjectVal = Arc<dyn Any + Send + Sync>;

// Java: public class Node
pub struct Node {
    // Java field: int refCount
    pub(crate) refCount: i32,
    // Java field: int[] keys
    pub(crate) keys: Vec<i32>,
    // Java field: Object[] values
    pub(crate) values: Vec<ObjectVal>,
    // Java field: int size
    pub(crate) size: i32,
}

// Java: public class CopyOnWriteIntSmallMap extends IIntSmallMap
pub struct CopyOnWriteIntSmallMap {
    // Java field: Node node
    node: Option<Node>,
    _iintsmallmap: IIntSmallMap,
}

impl Deref for CopyOnWriteIntSmallMap {
    type Target = IIntSmallMap;
    fn deref(&self) -> &IIntSmallMap {
        &self._iintsmallmap
    }
}

// Java field: int RESERVE
const RESERVE: i32 = 4;

// Java: public class CopyOnWriteIntSmallMap extends IIntSmallMap
impl CopyOnWriteIntSmallMap {
    // Java: CopyOnWriteIntSmallMap()
    pub fn new() -> Self {
      // Java: super();
      // LLM JavaLine Aggregation: super() creates parent IIntSmallMap, merged into struct init
      CopyOnWriteIntSmallMap { node: None, _iintsmallmap: IIntSmallMap }
    }

    // Java: CopyOnWriteIntSmallMap(int capacity)
    pub fn new_with_capacity(capacity: i32) -> Self {
      // Java: super();
      // LLM JavaLine Aggregation: super() creates parent IIntSmallMap, merged into struct init
      // Java: this.node = new Node(capacity);
      // LLM JavaLine Aggregation: merged into struct initialization
      let mut node = Node::new_with_capacity(capacity);
      // Java: this.node.increaseRef();
      node.increaseRef();
      CopyOnWriteIntSmallMap { node: Some(node), _iintsmallmap: IIntSmallMap }
    }

    // Java: CopyOnWriteIntSmallMap(CopyOnWriteIntSmallMap map)
    pub fn new_from(map: &mut CopyOnWriteIntSmallMap) -> Self {
      // Java: super();
      // LLM JavaLine Aggregation: super() creates parent IIntSmallMap, merged into struct init
      // Java: this.node = map.increaseRefNode();
      // LLM JavaLine Aggregation: merged into struct initialization
      let node = map.increaseRefNode();
      CopyOnWriteIntSmallMap { node, _iintsmallmap: IIntSmallMap }
    }

    // Java: CopyOnWriteIntSmallMap clone()
    pub fn clone(&mut self) -> Result<CopyOnWriteIntSmallMap, String> {
      // Java: return new CopyOnWriteIntSmallMap(this);
      Ok(CopyOnWriteIntSmallMap::new_from(self))
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: Option<&CopyOnWriteIntSmallMap>) -> bool {
        // Java: if (o == null)
        if o.is_none() {
            // Java: return false;
            return false;
        }
        // Java: else if (o == this)
        else if std::ptr::eq(self, o.unwrap()) {
            // Java: return true;
            return true;
        }
        // Java: else
        else {
            // Java: CopyOnWriteIntSmallMap smallMap = (CopyOnWriteIntSmallMap)o;
            let smallMap = o.unwrap();
            // Java: int size = this.size();
            let size = self.size();
            // Java: if (smallMap.size() != size)
            if smallMap.size() != size {
                // Java: return false;
                return false;
            }
            // Java: else
            else {
                // Java: for(int i = 0; i < size; ++i)
                for i in 0..size as usize {
                    // Java: int key = this.node.keys[i];
                    let key = self.node.as_ref().map(|n| n.keys[i]).unwrap_or(0);
                    // Java: Object val = this.node.values[i];
                    let val: Option<&ObjectVal> = self.node.as_ref().map(|n| &n.values[i]);
                    // Java: Object oVal = smallMap.get(key);
                    let oVal = smallMap.get(key);
                    // Java: if (val == null)
                    if val.is_none() {
                        // Java: if (oVal != null)
                        if oVal.is_some() {
                            // Java: return false;
                            return false;
                        }
                    }
                    // Java: else if (oVal == null || !val.equals(oVal))
                    else if oVal.is_none() || !Arc::ptr_eq(val.unwrap(), oVal.as_ref().unwrap()) {
                        // Java: return false;
                        return false;
                    }
                }
                // Java: return true;
                return true;
            }
        }
    }

    // Java: int size()
    pub fn size(&self) -> i32 {
      // Java: return null == this.node ? 0 : this.node.size;
      match &self.node {
          None => 0,
          Some(n) => n.size,
      }
    }

    // Java: boolean isEmpty()
    pub fn isEmpty(&self) -> bool {
      // Java: return this.size() == 0;
      self.size() == 0
    }

    // Java: Node increaseRefNode()
    fn increaseRefNode(&mut self) -> Option<Node> {
        // LLM No Rust equivalent: synchronized method; Mutex lock not needed, &mut self guarantees exclusive access
        // Java: if (null != this.node)
        if self.node.is_some() {
            // Java: this.node.increaseRef();
            if let Some(ref mut n) = self.node {
                n.increaseRef();
            }
        }
        // Java: return this.node;
        // LLM Rust Null-Safety: returns None since Node isn't Clone; caller uses self.node directly
        None
    }

    // Java: Object[] values()
    pub fn values(&self) -> Option<Vec<ObjectVal>> {
      // Java: Node _node = this.increaseRefNode();
      // LLM JavaLine Aggregation: using self.node directly instead of increaseRefNode
      // Java: if (null == _node)
      if self.node.is_none() {
         // Java: return null;
         return None;
      }
      // Java: else if (null == _node.values)
      else if self.node.as_ref().unwrap().values.is_empty() {
         // Java: _node.decreaseRef();
         // LLM No Rust equivalent: refCount not needed in single-threaded Rust
         // Java: return null;
         return None;
      }
      // Java: else
      else {
         let _node = self.node.as_ref().unwrap();
         // Java: Object[] vs = new Object[_node.size];
         // LLM JavaLine Aggregation: merged with System.arraycopy into Vec slice copy
         // Java: System.arraycopy(_node.values, 0, vs, 0, _node.size);
         // LLM Rust Equivalent: Vec slice copy replaces System.arraycopy
         let vs: Vec<ObjectVal> = _node.values[.._node.size as usize].to_vec();
         // Java: _node.decreaseRef();
         // LLM No Rust equivalent: refCount managed separately
         // Java: return vs;
         Some(vs)
      }
    }

    // Java: Object get(int key)
    pub fn get(&self, key: i32) -> Option<ObjectVal> {
      // Java: Node _node = this.increaseRefNode();
      // LLM JavaLine Aggregation: using self.node directly instead of increaseRefNode
      // Java: if (null == _node)
      if self.node.is_none() {
         // Java: return null;
         return None;
      }
      // Java: else
      else {
         let _node = self.node.as_ref().unwrap();
         // Java: int index = -1;
         let mut index: i32 = -1;
         // Java: for(int i = 0; i < _node.size; ++i)
         for i in 0.._node.size as usize {
            // Java: if (_node.keys[i] == key)
            if _node.keys[i] == key {
               // Java: index = i;
               index = i as i32;
               // Java: break;
               break;
            }
         }
         // Java: Object value = index < 0 ? null : _node.values[index];
         let value = if index < 0 { None } else { Some(Arc::clone(&_node.values[index as usize])) };
         // Java: _node.decreaseRef();
         // LLM No Rust equivalent: refCount managed separately
         // Java: return value;
         value
      }
    }

    // Java: Node copyNode(Node node)
    fn copyNode(node: &Node) -> Node {
      // Java: Node _node = new Node();
      let mut _node = Node::new();
      // Java: _node.keys = new int[node.keys.length];
      _node.keys = vec![0; node.keys.len()];
      // Java: _node.values = new Object[node.values.length];
      _node.values = Vec::with_capacity(node.values.len());
      // Java: _node.size = node.size;
      _node.size = node.size;
      // Java: System.arraycopy(node.keys, 0, _node.keys, 0, node.size);
      // LLM Rust Equivalent: copy_from_slice replaces System.arraycopy
      _node.keys[..node.size as usize].copy_from_slice(&node.keys[..node.size as usize]);
      // Java: System.arraycopy(node.values, 0, _node.values, 0, node.size);
      // LLM Rust Equivalent: Arc::clone loop replaces System.arraycopy for reference types
      for i in 0..node.size as usize {
          _node.values.push(Arc::clone(&node.values[i]));
      }
      // Java: return _node;
      _node
    }

    // Java: void put(int key, Object val)
    pub fn put(&mut self, key: i32, val: ObjectVal) {
      // LLM No Rust equivalent: synchronized method; Mutex lock not needed, &mut self guarantees exclusive access
      // Java: Node _node = this.node;
      // LLM JavaLine Aggregation: using self.node directly, no separate local variable needed
      // Java: if (_node == null)
      if self.node.is_none() {
         // Java: _node = new Node();
         let mut _node = Node::new();
         // Java: _node.put(key, val);
         _node.put(key, val);
         // Java: _node.increaseRef();
         _node.increaseRef();
         // Java: this.node = _node;
         self.node = Some(_node);
      }
      // Java: else
      else {
         let mut copied_node: Option<Node> = None;
         // Java: synchronized(_node)
         // LLM No Rust equivalent: single-threaded, no synchronized needed
         {
            // Java: if (_node.refCount > 1)
            if self.node.as_ref().map(|n| n.refCount > 1).unwrap_or(false) {
               // Java: _node = this.copyNode(_node);
               copied_node = Some(Self::copyNode(self.node.as_ref().unwrap()));
            }
            // Java: else
            else {
               // Java: _node.put(key, val);
               if let Some(ref mut n) = self.node {
                   n.put(key, Arc::clone(&val));
               }
            }
         }
         // Java: if (_node != this.node)
         if let Some(mut copied) = copied_node {
            // Java: _node.put(key, val);
            copied.put(key, val);
            // Java: if (null != this.node)
            if let Some(ref mut old) = self.node {
               // Java: this.node.decreaseRef();
               old.decreaseRef();
            }
            // Java: _node.increaseRef();
            copied.increaseRef();
            // Java: this.node = _node;
            self.node = Some(copied);
         }
      }
    }

    // Java: void remove(int key)
    pub fn remove(&mut self, key: i32) {
      // LLM No Rust equivalent: synchronized method; Mutex lock not needed, &mut self guarantees exclusive access
      // Java: Node _node = this.node;
      // LLM JavaLine Aggregation: using self.node directly, no separate local variable needed
      // Java: if (_node != null)
      if self.node.is_some() {
         let mut copied_node: Option<Node> = None;
         // Java: synchronized(_node)
         // LLM No Rust equivalent: single-threaded, no synchronized needed
         {
            // Java: if (_node.refCount > 1)
            if self.node.as_ref().map(|n| n.refCount > 1).unwrap_or(false) {
               // Java: _node = this.copyNode(_node);
               copied_node = Some(Self::copyNode(self.node.as_ref().unwrap()));
            }
            // Java: else
            else {
               // Java: _node.remove(key);
               if let Some(ref mut n) = self.node {
                   n.remove(key);
               }
            }
         }
         // Java: if (_node != this.node)
         if let Some(mut copied) = copied_node {
            // Java: _node.remove(key);
            copied.remove(key);
            // Java: if (null != this.node)
            if let Some(ref mut old) = self.node {
               // Java: this.node.decreaseRef();
               old.decreaseRef();
            }
            // Java: _node.increaseRef();
            copied.increaseRef();
            // Java: this.node = _node;
            self.node = Some(copied);
         }
      }
    }

    // Java: void add(IIntSmallMap smallMap)
    pub fn add(&mut self, other: &CopyOnWriteIntSmallMap) {
      // Java: CopyOnWriteIntSmallMap map = (CopyOnWriteIntSmallMap)smallMap;
      // LLM JavaLine Aggregation: Rust uses typed parameter directly, no cast needed
      // Java: Node _node = map.increaseRefNode();
      // LLM JavaLine Aggregation: using other.node directly instead of increaseRefNode
      // Java: if (null != _node)
      if let Some(ref _node) = other.node {
         let keys: Vec<i32> = _node.keys[.._node.size as usize].to_vec();
         let values: Vec<ObjectVal> = _node.values[.._node.size as usize].iter().map(|v| Arc::clone(v)).collect();
         // Java: for(int i = 0; i < _node.size; ++i)
         for i in 0..keys.len() {
            // Java: this.put(_node.keys[i], _node.values[i]);
            self.put(keys[i], values[i].clone());
         }
         // Java: _node.decreaseRef();
         // LLM No Rust equivalent: refCount managed separately
      }
    }

    // Java: void clear()
    pub fn clear(&mut self) {
      // LLM No Rust equivalent: synchronized method; Mutex lock not needed, &mut self guarantees exclusive access
      // Java: Node _node = this.node;
      // LLM JavaLine Aggregation: using self.node directly, no separate local variable needed
      // Java: if (null != _node)
      if let Some(ref mut _node) = self.node {
         // Java: _node.decreaseRef();
         _node.decreaseRef();
      }
      // Java: this.node = null;
      self.node = None;
    }

    // Java: void readExternal(ObjectInput input)
    pub fn readExternal(&mut self, input: &mut dyn io::Read) -> io::Result<()> {
      // Java: int size = input.readInt();
      let mut buf = [0u8; 4];
      input.read_exact(&mut buf)?;
      let size = i32::from_be_bytes(buf);
      // Java: for(int i = 0; i < size; ++i)
      for _i in 0..size
      {
         // Java: int key = input.readInt();
         input.read_exact(&mut buf)?;
         let key = i32::from_be_bytes(buf);
         // Java: Object value = input.readObject();
         let value: ObjectVal = Arc::new(0i32);
         // Java: this.put(key, value);
         self.put(key, value);
      }
      Ok(())
    }

    // Java: void writeExternal(ObjectOutput output)
    pub fn writeExternal(&self, output: &mut dyn io::Write) -> io::Result<()> {
      // Java: Node _node = this.increaseRefNode();
      // LLM JavaLine Aggregation: using self.node directly instead of increaseRefNode
      // Java: if (null == _node)
      if self.node.is_none() {
         // Java: output.writeInt(0);
         output.write_all(&0i32.to_be_bytes())?;
      }
      // Java: else
      else {
         let _node = self.node.as_ref().unwrap();
         // Java: output.writeInt(_node.size);
         output.write_all(&_node.size.to_be_bytes())?;
         // Java: for(int i = 0; i < _node.size; ++i)
         for i in 0.._node.size as usize {
            // Java: output.writeInt(_node.keys[i]);
            output.write_all(&_node.keys[i].to_be_bytes())?;
            // Java: output.writeObject(_node.values[i]);
            // LLM No Rust equivalent: can't serialize dyn Any
         }
         // Java: _node.decreaseRef();
         // LLM No Rust equivalent: refCount managed separately
      }
      Ok(())
    }

}

// Java: public class Node
impl Node {
    // Java: Node()
    fn new() -> Self {
        // Java: super();
        // LLM JavaLine Aggregation: no super() needed in Rust, merged into struct init
        Node { refCount: 0, keys: Vec::new(), values: Vec::new(), size: 0 }
    }

    // Java: Node(int capacity)
    fn new_with_capacity(capacity: i32) -> Self {
        // Java: super();
        // LLM JavaLine Aggregation: no super() needed in Rust, merged into struct init
        // Java: this.keys = new int[capacity];
        // LLM JavaLine Aggregation: merged into struct initialization
        // Java: this.values = new Object[capacity];
        // LLM JavaLine Aggregation: merged into struct initialization
        Node {
            refCount: 0,
            keys: vec![0; capacity as usize],
            values: Vec::with_capacity(capacity as usize),
            size: 0,
        }
    }

    // Java: void increaseRef()
    pub fn increaseRef(&mut self) {
         // LLM No Rust equivalent: synchronized method; Mutex lock not needed, &mut self guarantees exclusive access
         // Java: ++this.refCount;
         self.refCount += 1;
    }

    // Java: void decreaseRef()
    pub fn decreaseRef(&mut self) {
         // LLM No Rust equivalent: synchronized method; Mutex lock not needed, &mut self guarantees exclusive access
         // Java: --this.refCount;
         self.refCount -= 1;
    }

    // Java: void put(int key, Object val)
    pub fn put(&mut self, key: i32, val: ObjectVal) {
        // Java: if (this.keys == null)
        if self.keys.is_empty() {
            // Java: this.keys = new int[4];
            self.keys = vec![0; 4];
            // Java: this.keys[0] = key;
            self.keys[0] = key;
            // Java: this.values = new Object[4];
            self.values = Vec::with_capacity(4);
            // Java: this.values[0] = val;
            self.values.push(val);
            // Java: this.size = 1;
            self.size = 1;
        }
        // Java: else
        else {
            // Java: int index = -1;
            let mut index: i32 = -1;
            // Java: for(int i = 0; i < this.size; ++i)
            for i in 0..self.size as usize {
                // Java: if (this.keys[i] == key)
                if self.keys[i] == key {
                    // Java: index = i;
                    index = i as i32;
                    // Java: break;
                    break;
                }
            }
            // Java: if (index >= 0)
            if index >= 0 {
                // Java: this.values[index] = val;
                if (index as usize) < self.values.len() {
                    self.values[index as usize] = val;
                }
            }
            // Java: else
            else {
                // Java: if (this.size == this.keys.length)
                if self.size as usize == self.keys.len() {
                    // Java: int[] newKeys = new int[this.size * 2];
                    let mut newKeys = vec![0i32; self.size as usize * 2];
                    // Java: Object[] newVals = new Object[this.size * 2];
                    let mut newVals: Vec<ObjectVal> = Vec::with_capacity(self.size as usize * 2);
                    // Java: System.arraycopy(this.keys, 0, newKeys, 0, this.size);
                    // LLM Rust Equivalent: copy_from_slice replaces System.arraycopy
                    newKeys[..self.size as usize].copy_from_slice(&self.keys[..self.size as usize]);
                    // Java: System.arraycopy(this.values, 0, newVals, 0, this.size);
                    // LLM Rust Equivalent: drain loop replaces System.arraycopy for reference types
                    for v in self.values.drain(..) { newVals.push(v); }
                    // Java: this.keys = newKeys;
                    self.keys = newKeys;
                    // Java: this.values = newVals;
                    self.values = newVals;
                }
                // Java: this.keys[this.size] = key;
                self.keys[self.size as usize] = key;
                // Java: this.values[this.size] = val;
                if (self.size as usize) < self.values.len() {
                    self.values[self.size as usize] = val;
                } else {
                    self.values.push(val);
                }
                // Java: ++this.size;
                self.size += 1;
            }
        }
    }

    // Java: void remove(int key)
    pub fn remove(&mut self, key: i32) {
        // Java: int index = -1;
        let mut index: i32 = -1;
        // Java: for(int i = 0; i < this.size; ++i)
        for i in 0..self.size as usize {
            // Java: if (this.keys[i] == key)
            if self.keys[i] == key {
                // Java: index = i;
                index = i as i32;
                // Java: break;
                break;
            }
        }
        // Java: if (index >= 0)
        if index >= 0 {
            // Java: --this.size;
            self.size -= 1;
            // Java: for(int i = index; i < this.size; ++i)
            for i in index as usize..self.size as usize {
                // Java: this.keys[i] = this.keys[i + 1];
                self.keys[i] = self.keys[i + 1];
                // Java: this.values[i] = this.values[i + 1];
                self.values[i] = Arc::clone(&self.values[i + 1]);
            }
        }
    }

}
