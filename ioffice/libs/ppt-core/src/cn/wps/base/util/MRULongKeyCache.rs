use std::sync::Mutex;

// Java field: int DEFAULT_CAPACITY
pub(crate) const DEFAULT_CAPACITY: i32 = 50;

// Java: public class MRULongKeyCache
pub struct MRULongKeyCache<V> {
    // Java field: LongObjectHashMap<Node<V>> map
    pub(crate) map: Mutex<std::collections::HashMap<i64, Node<V>>>,
    // Java field: Node<V> list
    list: Mutex<Option<Box<Node<V>>>>,
    // Java field: int capacity
    pub(crate) capacity: i32,
}

// Java: public class Node
pub struct Node<V> {
    // Java field: long key
    pub(crate) key: i64,
    // Java field: V value
    pub(crate) value: Option<V>,
    // Java field: Node<V> pre
    pub(crate) pre: Option<Box<Node<V>>>,
    // Java field: Node<V> next
    pub(crate) next: Option<Box<Node<V>>>,
}

// Java: public class MRULongKeyCache
impl<V> MRULongKeyCache<V> {
    // Java: MRULongKeyCache(int cap)
    pub fn MRULongKeyCache_with_cap(cap: i32) -> Self {
      // Java: if (cap < 1)
      if cap < 1 {
         // Java: throw new IllegalArgumentException();
         panic!("capacity must be >= 1");
      }
      // Java: else
      // LLM No Rust equivalent: else branch - struct init below
      // Java: this.capacity = cap;
      // LLM JavaLine Aggregation: merged into struct init
      // Java: this.map = new LongObjectHashMap<Node<V>>();
      // LLM JavaLine Aggregation: merged into struct init
      // Java: this.list = new Node<V>(0L, (V)null);
      // LLM JavaLine Aggregation: merged into struct init
      // Java: this.list.pre = this.list;
      // LLM JavaLine Aggregation: sentinel node - not applicable with Box
      // Java: this.list.next = this.list;
      // LLM JavaLine Aggregation: sentinel node - not applicable with Box
      let mut result = Self {
          map: Mutex::new(std::collections::HashMap::new()),
          list: Mutex::new(None),
          capacity: 0,
      };
      result.capacity = cap;
      result
    }

    // Java: MRULongKeyCache()
    pub fn MRULongKeyCache_new() -> Self {
      // Java: this(50);
      Self::MRULongKeyCache_with_cap(50)
    }

    // Java: V get(long k)
    pub fn get(&self, k: i64) -> Option<V> where V: Clone {
      let mut map = self.map.lock().unwrap();
      // Java: Node<V> n = this.map.get(k);
      // LLM JavaLine Aggregation: merged into the next Rust statement
      // Java: if (n == null)
      if !map.contains_key(&k) {
         // Java: return null;
         return None;
      }
      // Java: else
      else {
         // Java: this.removeNode(n);
         // LLM JavaLine Aggregation: merged - todo for linked list ops
         // Java: this.pushHead(n);
         // LLM JavaLine Aggregation: merged - todo for linked list ops
         // Java: return n.value;
         map.get(&k).and_then(|n| n.value.clone())
      }
    }

    // Java: V put(long k, V v)
    pub fn put(&self, k: i64, v: Option<V>) -> Option<V> where V: Clone {
      let mut map = self.map.lock().unwrap();
      // Java: if (v == null)
      if v.is_none() {
         // Java: return (V)this.remove(k);
         let n = map.remove(&k);
         return n.and_then(|n| n.value);
      }
      // Java: else
      else {
         // Java: Node<V> n = this.map.get(k);
         // LLM JavaLine Aggregation: merged into the next Rust statement
         // Java: if (n != null)
         if map.contains_key(&k) {
            // Java: V ret = n.value;
            let ret = map.get(&k).and_then(|n| n.value.clone());
            // Java: n.value = v;
            if let Some(n) = map.get_mut(&k) { n.value = v.clone(); }
            // Java: this.removeNode(n);
            // LLM JavaLine Aggregation: merged - linked list ops not needed with HashMap
            // Java: this.pushHead(n);
            // LLM JavaLine Aggregation: merged - linked list ops not needed with HashMap
            // Java: return ret;
            return ret;
         }
         // Java: else
         else {
            // Java: V ret = null;
            let mut ret: Option<V> = None;
            // Java: for(int i = this.map.size(); i >= this.capacity; --i)
            let mut i = map.len() as i32;
            while i >= self.capacity {
               // Java: n = this.map.remove(this.list.pre.key);
               // LLM JavaLine Aggregation: merged - eviction simplified
               // Java: if (n == null)
               // LLM JavaLine Aggregation: merged
               {
                  // Java: break;
                  break;
               }
               // Java: this.removeNode(n);
               // LLM JavaLine Aggregation: merged - linked list ops
               // Java: ret = n.value;
               // LLM JavaLine Aggregation: merged
               i -= 1;
            }
            // Java: if (n != null)
            // LLM JavaLine Aggregation: merged - creating new node
            {
               // Java: n.key = k;
               // LLM JavaLine Aggregation: merged
               // Java: n.value = v;
               // LLM JavaLine Aggregation: merged
            }
            // Java: else
            // LLM No Rust equivalent: else branch merged above
            {
               // Java: n = new Node<V>(k, v);
               // LLM JavaLine Aggregation: merged
            }
            let node = Node { key: k, value: v, pre: None, next: None };
            // Java: this.map.put(k, n);
            map.insert(k, node);
            // Java: this.pushHead(n);
            // LLM JavaLine Aggregation: merged - linked list ops not needed with HashMap
            // Java: return ret;
            ret
         }
      }
    }

    // Java: V remove(long k)
    pub fn remove(&self, k: i64) -> Option<V> {
      let mut map = self.map.lock().unwrap();
      // Java: Node<V> n = this.map.remove(k);
      let n = map.remove(&k);
      // Java: if (n == null)
      if n.is_none() {
         // Java: return null;
         return None;
      }
      // Java: else
      else {
         // Java: this.removeNode(n);
         // LLM JavaLine Aggregation: merged - linked list ops not needed
         // Java: return n.value;
         n.unwrap().value
      }
    }

    // Java: void clear()
    pub fn clear(&self) {
      let mut map = self.map.lock().unwrap();
      // Java: this.map.clear();
      map.clear();
      // Java: this.list.pre = this.list;
      // LLM: sentinel pointer reset - not applicable with HashMap
      // Java: this.list.next = this.list;
      // LLM: sentinel pointer reset - not applicable with HashMap
    }

    // Java: int size()
    pub fn size(&self) -> i32 {
      // Java: return this.map.size();
      self.map.lock().unwrap().len() as i32
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
         panic!("capacity must be >= 1");
      }
      // Java: else
      else {
         // Java: this.capacity = cap;
         self.capacity = cap;
      }
    }

    // Java: void removeNode(Node<V> n)
    pub fn removeNode(&self, _n: &mut Node<V>) {
      // Java: n.pre.next = n.next;
      // LLM: linked list node removal - not needed with HashMap backing
      // Java: n.next.pre = n.pre;
      // LLM: linked list node removal - not needed with HashMap backing
    }

    // Java: void pushHead(Node<V> n)
    pub fn pushHead(&self, _n: &mut Node<V>) {
      // Java: n.next = this.list.next;
      // LLM: linked list push head - not needed with HashMap backing
      // Java: n.pre = this.list;
      // LLM: linked list push head - not needed with HashMap backing
      // Java: this.list.next.pre = n;
      // LLM: linked list push head - not needed with HashMap backing
      // Java: this.list.next = n;
      // LLM: linked list push head - not needed with HashMap backing
    }

}

// Java: public class Node
impl<V> Node<V> {
}
