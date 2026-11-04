#![allow(non_snake_case, non_upper_case_globals, dead_code, unused_variables, unused_imports, unused_mut)]

// Java: import java.util.Iterator
use std::any::Any;
use std::sync::{Arc, Mutex};
use crate::cn::wps::base::util::IntObjectHashMap::IntObjectHashMap;

// Java field: int DEFAULT_CAPACITY
pub(crate) const DEFAULT_CAPACITY: i32 = 50;

// Java: public class Node
pub struct Node {
    // Java field: int key
    pub(crate) key: i32,
    // Java field: V value
    pub(crate) value: Option<Box<dyn Any>>,
    // Java field: Node<V> pre
    pub(crate) pre: usize,
    // Java field: Node<V> next
    pub(crate) next: usize,
}

// Java: public class MRUIntKeyCache
pub struct MRUIntKeyCache {
    // Java field: IntObjectHashMap<Node<V>> map
    pub(crate) map: IntObjectHashMap,
    // Java field: Node<V> list
    list_key: i32,
    list_pre: usize,
    list_next: usize,
    nodes: Vec<Node>,
    // Java field: int capacity
    pub(crate) capacity: i32,
}

// Java: public class MRUIntKeyCache
impl MRUIntKeyCache {
    // Java: MRUIntKeyCache(int cap)
    pub fn new(cap: i32) -> Self {
      // Java: if (cap < 1)
      if cap < 1 {
         // Java: throw new IllegalArgumentException();
         panic!("IllegalArgumentException");
      }
      // Java: else
      else {
         // Java: this.capacity = cap;
         let capacity = cap;
         // Java: this.map = new IntObjectHashMap<Node<V>>();
         let mut map = IntObjectHashMap { table: Vec::new(), size: 0, threshold: 0 };
         map.IntObjectHashMap_default();
         // Java: this.list = new Node<V>(0, (V)null);
         // Java: this.list.pre = this.list;
         // Java: this.list.next = this.list;
         MRUIntKeyCache {
             map,
             list_key: 0,
             list_pre: usize::MAX,
             list_next: usize::MAX,
             nodes: Vec::new(),
             capacity,
         }
      }
    }

    // Java: MRUIntKeyCache()
    pub fn new_default() -> Self {
      // Java: this(50);
      Self::new(50)
    }

    fn find_node_index(&self, key: i32) -> Option<usize> {
        self.nodes.iter().position(|n| n.key == key)
    }

    // Java: void toArray(V[] array)
    pub fn toArray(&self, array: &mut Vec<Option<Box<dyn Any>>>) {
      // Java: int index = 0;
      let mut index = 0;
      // Java: int len = array.length;
      let len = array.len();
      // Java: if (len != 0)
      if len != 0 {
         // Java: Iterator<IntObjectHashMap.Entry<Node<V>>> iterator = this.map.iterator();
         // Java: while (iterator.hasNext() && index < len)
         let mut current = self.list_next;
         while current != usize::MAX && index < len {
            // Java: IntObjectHashMap.Entry<Node<V>> entry = iterator.next();
            // Java: array[index++] = entry.getValue().value;
            if current < self.nodes.len() {
                array[index] = None; // placeholder
                index += 1;
                current = self.nodes[current].next;
            } else {
                break;
            }
         }
      }
    }

    // Java: V get(int k)
    pub fn get(&mut self, k: i32) -> Option<&Box<dyn Any>> {
      // Java: Node<V> n = this.map.get(k);
      let idx = self.find_node_index(k);
      // Java: if (n == null)
      if idx.is_none() {
         // Java: return null;
         return None;
      }
      // Java: else
      else {
         let idx = idx.unwrap();
         // Java: this.removeNode(n);
         self.removeNode_idx(idx);
         // Java: this.pushHead(n);
         self.pushHead_idx(idx);
         // Java: return n.value;
         self.nodes[idx].value.as_ref()
      }
    }

    // Java: V put(int k, V v)
    pub fn put(&mut self, k: i32, v: Option<Box<dyn Any>>) -> Option<Box<dyn Any>> {
      // Java: if (v == null)
      if v.is_none() {
         // Java: return (V)this.remove(k);
         return self.remove(k);
      }
      // Java: else
      else {
         // Java: Node<V> n = this.map.get(k);
         let idx = self.find_node_index(k);
         // Java: if (n != null)
         if let Some(idx) = idx {
            // Java: V ret = n.value;
            let ret = self.nodes[idx].value.take();
            // Java: n.value = v;
            self.nodes[idx].value = v;
            // Java: this.removeNode(n);
            self.removeNode_idx(idx);
            // Java: this.pushHead(n);
            self.pushHead_idx(idx);
            // Java: return ret;
            return ret;
         }
         // Java: else
         else {
            // Java: V ret = null;
            let mut ret: Option<Box<dyn Any>> = None;
            // Java: for(int i = this.map.size(); i >= this.capacity; --i)
            let mut i = self.map.size as i32;
            while i >= self.capacity {
               // Java: n = this.map.remove(this.list.pre.key);
               let tail_idx = self.list_pre;
               if tail_idx == usize::MAX { break; }
               let tail_key = self.nodes[tail_idx].key;
               self.map.remove(tail_key);
               // Java: if (n == null)
               // Java: break;
               // Java: this.removeNode(n);
               self.removeNode_idx(tail_idx);
               // Java: ret = n.value;
               ret = self.nodes[tail_idx].value.take();
               i -= 1;
            }
            // Java: n = new Node<V>(k, v);
            let new_node = Node { key: k, value: v, pre: usize::MAX, next: usize::MAX };
            let new_idx = self.nodes.len();
            self.nodes.push(new_node);
            // Java: this.map.put(k, n);
            self.map.put(k, Some(Box::new(new_idx as i32)));
            // Java: this.pushHead(n);
            self.pushHead_idx(new_idx);
            // Java: return ret;
            return ret;
         }
      }
    }

    // Java: V remove(int k)
    pub fn remove(&mut self, k: i32) -> Option<Box<dyn Any>> {
      // Java: Node<V> n = this.map.remove(k);
      let idx = self.find_node_index(k);
      // Java: if (n == null)
      if idx.is_none() {
         // Java: return null;
         return None;
      }
      // Java: else
      else {
         let idx = idx.unwrap();
         // Java: this.removeNode(n);
         self.removeNode_idx(idx);
         // Java: return n.value;
         self.nodes[idx].value.take()
      }
    }

    // Java: void clear()
    pub fn clear(&mut self) {
      // Java: this.map.clear();
      self.map.clear();
      // Java: this.list.pre = this.list;
      self.list_pre = usize::MAX;
      // Java: this.list.next = this.list;
      self.list_next = usize::MAX;
      self.nodes.clear();
    }

    // Java: int size()
    pub fn size(&self) -> i32 {
      // Java: return this.map.size();
      self.map.size
    }

    // Java: int getCapacity()
    pub fn getCapacity(&self) -> i32 {
      // Java: return this.capacity;
      self.capacity
    }

    // Java: void setCapacity(int cap)
    pub fn setCapacity(&mut self, cap: i32) {
      // Java: if (cap < 1)
      if cap < 1 {
         // Java: throw new IllegalArgumentException();
         panic!("IllegalArgumentException");
      }
      // Java: else
      else {
         // Java: this.capacity = cap;
         self.capacity = cap;
      }
    }

    // Java: boolean containsKey(int key)
    pub fn containsKey(&self, key: i32) -> bool {
      // Java: return this.map.containsKey(key);
      self.map.containsKey(key)
    }

    // Java: void removeNode(Node<V> n)
    fn removeNode_idx(&mut self, idx: usize) {
      // Java: n.pre.next = n.next;
      let pre = self.nodes[idx].pre;
      let next = self.nodes[idx].next;
      if pre == usize::MAX {
          self.list_next = next;
      } else {
          self.nodes[pre].next = next;
      }
      // Java: n.next.pre = n.pre;
      if next == usize::MAX {
          self.list_pre = pre;
      } else {
          self.nodes[next].pre = pre;
      }
      self.nodes[idx].pre = usize::MAX;
      self.nodes[idx].next = usize::MAX;
    }

    // Java: void pushHead(Node<V> n)
    fn pushHead_idx(&mut self, idx: usize) {
      // Java: n.next = this.list.next;
      let old_head = self.list_next;
      self.nodes[idx].next = old_head;
      // Java: n.pre = this.list;
      self.nodes[idx].pre = usize::MAX;
      // Java: this.list.next.pre = n;
      if old_head != usize::MAX {
          self.nodes[old_head].pre = idx;
      } else {
          self.list_pre = idx;
      }
      // Java: this.list.next = n;
      self.list_next = idx;
    }

}

// Java: public class Node
impl Node {
    pub fn new(key: i32, value: Option<Box<dyn Any>>) -> Self {
        Node { key, value, pre: usize::MAX, next: usize::MAX }
    }
}
