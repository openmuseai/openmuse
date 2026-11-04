// Java: import java.io.Serializable
// LLM No Rust equivalent: Serializable not needed in Rust
// Java: import java.util.Arrays
// LLM No Rust equivalent: using Rust native operations
// Java: import java.util.Iterator
// LLM No Rust equivalent: using Rust Iterator trait
// Java: import java.util.NoSuchElementException
// LLM No Rust equivalent: using Option/panic in Rust

use std::any::Any;

// Java field: long serialVersionUID
pub(crate) const serialVersionUID: i64 = 1;
// Java field: int MINIMUM_CAPACITY
pub(crate) const MINIMUM_CAPACITY: i32 = 4;
// Java field: int MAXIMUM_CAPACITY
pub(crate) const MAXIMUM_CAPACITY: i32 = 1073741824;

// Java: public class Entry
pub struct Entry {
    // Java field: int key
    pub(crate) key: i32,
    // Java field: V value
    pub(crate) value: Option<Box<dyn Any>>,
    // Java field: Entry<V> next
    pub(crate) next: Option<Box<Entry>>,
}

// Java: public class IntObjectHashMap implements Cloneable, Serializable, Iterable
pub struct IntObjectHashMap {
    // Java field: Entry<V>[] table
    pub(crate) table: Vec<Option<Box<Entry>>>,
    // Java field: int size
    pub(crate) size: i32,
    // Java field: int threshold
    pub(crate) threshold: i32,
}

// Java: public class IntObjectHashMap implements Cloneable, Serializable, Iterable
impl IntObjectHashMap {
    // Java: IntObjectHashMap()
    pub fn IntObjectHashMap_default(&mut self) {
      // Java: this(4);
      self.IntObjectHashMap_with_capacity(4);
    }

    // Java: IntObjectHashMap(int capacity)
    pub fn IntObjectHashMap_with_capacity(&mut self, mut capacity: i32) {
      // Java: if (capacity < 4)
      if capacity < 4 {
         // Java: capacity = 4;
         capacity = 4;
      }
      // Java: else if (capacity > 1073741824)
      else if capacity > 1073741824 {
         // Java: capacity = 1073741824;
         capacity = 1073741824;
      }
      // Java: else
      else {
         // Java: capacity = roundUpToPowerOfTwo(capacity);
         capacity = Self::roundUpToPowerOfTwo(capacity);
      }
      // Java: this.table = new Entry[capacity];
      self.table = (0..capacity).map(|_| None).collect();
      // Java: this.threshold = (capacity >> 1) + (capacity >> 2);
      self.threshold = (capacity >> 1) + (capacity >> 2);
    }

    // Java: V put(int key, V value)
    pub fn put(&mut self, key: i32, value: Option<Box<dyn Any>>) -> Option<Box<dyn Any>> {
      // Java: int h = hash(key);
      let h = Self::hash(key);
      // Java: Entry<V>[] t = this.table;
      let t_len = self.table.len();
      // Java: int index = h & t.length - 1;
      let index = (h & (t_len as i32 - 1)) as usize;
      // Java: for(Entry<V> e = t[index]; e != null; e = e.next)
      let mut current = &mut self.table[index];
      loop {
        match current {
          Some(e) => {
            // Java: if (e.key == key)
            if e.key == key {
              // Java: V oldValue = e.value;
              let oldValue = e.value.take();
              // Java: e.value = value;
              e.value = value;
              // Java: return oldValue;
              return oldValue;
            }
            current = &mut e.next;
          }
          None => break,
        }
      }
      // Java: if (++this.size > this.threshold)
      self.size += 1;
      if self.size > self.threshold {
         // Java: this.doubleCapacity();
         self.doubleCapacity();
         // Java: t = this.table;
         // Java: index = h & t.length - 1;
         let t_len2 = self.table.len();
         let index2 = (h & (t_len2 as i32 - 1)) as usize;
         // Java: t[index] = new Entry(key, value, t[index]);
         let old = self.table[index2].take();
         self.table[index2] = Some(Box::new(Entry { key, value, next: old }));
      } else {
        // Java: t[index] = new Entry(key, value, t[index]);
        let old = self.table[index].take();
        self.table[index] = Some(Box::new(Entry { key, value, next: old }));
      }
      // Java: return null;
      None
    }

    // Java: void putAll(IntObjectHashMap<? extends V> m)
    pub fn putAll(&mut self, _m: &IntObjectHashMap) {
      // Java: if (m.size != 0)
      // LLM: complex generic iteration; simplified
      todo!()
    }

    // Java: V get(int key)
    pub fn get(&self, key: i32) -> Option<&Box<dyn Any>> {
      // Java: int h = hash(key);
      let h = Self::hash(key);
      // Java: Entry<V>[] t = this.table;
      let t_len = self.table.len();
      // Java: for(Entry<V> e = t[h & t.length - 1]; e != null; e = e.next)
      let index = (h & (t_len as i32 - 1)) as usize;
      let mut current = &self.table[index];
      loop {
        match current {
          Some(e) => {
            // Java: if (e.key == key)
            if e.key == key {
              // Java: return e.value;
              return e.value.as_ref();
            }
            current = &e.next;
          }
          None => break,
        }
      }
      // Java: return null;
      None
    }

    // Java: V remove(int key)
    pub fn remove(&mut self, key: i32) -> Option<Box<dyn Any>> {
      // Java: int h = hash(key);
      let h = Self::hash(key);
      // Java: Entry<V>[] t = this.table;
      let t_len = self.table.len();
      // Java: int index = h & t.length - 1;
      let index = (h & (t_len as i32 - 1)) as usize;
      // Java: Entry<V> e = t[index];
      // Java: for(Entry<V> pre = null; e != null; e = e.next)
      // Java: if (e.key == key)
      // Java: if (pre == null)
      // Java: t[index] = e.next;
      // Java: else
      // Java: pre.next = e.next;
      // Java: --this.size;
      // Java: return e.value;
      // Java: pre = e;
      // Java: return null;
      // LLM: linked list removal implemented via Rust ownership
      let mut current = self.table[index].take();
      let mut prev_chain: Option<Box<Entry>> = None;
      while let Some(mut e) = current {
        if e.key == key {
          self.size -= 1;
          let val = e.value.take();
          let rest = e.next.take();
          match prev_chain {
            None => { self.table[index] = rest; }
            Some(mut p) => { p.next = rest; self.table[index] = Some(p); }
          }
          return val;
        }
        current = e.next.take();
        e.next = prev_chain;
        prev_chain = Some(e);
      }
      self.table[index] = prev_chain;
      None
    }

    // Java: void clear()
    pub fn clear(&mut self) {
      // Java: Arrays.fill(this.table, (Object)null);
      for slot in self.table.iter_mut() { *slot = None; }
      // Java: this.size = 0;
      self.size = 0;
    }

    // Java: Iterator<Entry<V>> iterator()
    // Java: return new Iterator<Entry<V>>()
    // Java: int nextIndex;
    // Java: Entry<V> currentEntry;
    // Java: Entry<V> nextEntry;
    // Java: Entry<V>[] t = IntObjectHashMap.this.table;
    // Java: int i = 0;
    // Java: for(int len = t.length; i < len; ++i)
    // Java: if (t[i] != null)
    // Java: this.nextIndex = i;
    // Java: this.nextEntry = t[i];
    // Java: break;
    // Java: private void advance()
    // Java: this.currentEntry = this.nextEntry;
    // Java: this.nextEntry = this.nextEntry.next;
    // Java: if (this.nextEntry == null)
    // Java: Entry<V>[] t = IntObjectHashMap.this.table;
    // Java: while(++this.nextIndex < t.length)
    // Java: if (t[this.nextIndex] != null)
    // Java: this.nextEntry = t[this.nextIndex];
    // Java: break;
    // Java: public boolean hasNext()
    // Java: return this.nextEntry != null;
    // Java: public Entry<V> next()
    // Java: if (!this.hasNext())
    // Java: throw new NoSuchElementException();
    // Java: else
    // Java: this.advance();
    // Java: return this.currentEntry;
    // Java: public void remove()
    // Java: IntObjectHashMap.this.remove(this.currentEntry.key);
    pub fn iterator(&self) { todo!() }

    // Java: boolean containsKey(int key)
    pub fn containsKey(&self, key: i32) -> bool {
      // Java: int h = hash(key);
      let h = Self::hash(key);
      // Java: Entry<V>[] t = this.table;
      let t_len = self.table.len();
      // Java: for(Entry<V> e = t[h & t.length - 1]; e != null; e = e.next)
      let index = (h & (t_len as i32 - 1)) as usize;
      let mut current = &self.table[index];
      loop {
        match current {
          Some(e) => {
            // Java: if (e.key == key)
            if e.key == key {
              // Java: return true;
              return true;
            }
            current = &e.next;
          }
          None => break,
        }
      }
      // Java: return false;
      false
    }

    // Java: boolean containsValue(V value)
    pub fn containsValue(&self, _value: &dyn Any) -> bool {
      // Java: Entry<V>[] t = this.table;
      // Java: int i = 0;
      // Java: for(int len = t.length; i < len; ++i)
      // Java: for(Entry<V> e = t[i]; e != null; e = e.next)
      // Java: if (e.value == value || value != null && value.equals(e.value))
      // Java: return true;
      // Java: return false;
      todo!()
    }

    // Java: int size()
    pub fn size(&self) -> i32 {
      // Java: return this.size;
      self.size
    }

    // Java: boolean isEmpty()
    pub fn isEmpty(&self) -> bool {
      // Java: return this.size == 0;
      self.size == 0
    }

    // Java: static int hash(int v)
    pub fn hash(v: i32) -> i32 {
      // Java: int h = v ^ v >>> 20 ^ v >>> 12;
      let h = v ^ ((v as u32 >> 20) as i32) ^ ((v as u32 >> 12) as i32);
      // Java: return h ^ h >>> 7 ^ h >>> 4;
      h ^ ((h as u32 >> 7) as i32) ^ ((h as u32 >> 4) as i32)
    }

    // Java: static int roundUpToPowerOfTwo(int i)
    pub fn roundUpToPowerOfTwo(mut i: i32) -> i32 {
      // Java: --i;
      i -= 1;
      // Java: i |= i >>> 1;
      i |= (i as u32 >> 1) as i32;
      // Java: i |= i >>> 2;
      i |= (i as u32 >> 2) as i32;
      // Java: i |= i >>> 4;
      i |= (i as u32 >> 4) as i32;
      // Java: i |= i >>> 8;
      i |= (i as u32 >> 8) as i32;
      // Java: i |= i >>> 16;
      i |= (i as u32 >> 16) as i32;
      // Java: return i + 1;
      i + 1
    }

    // Java: void doubleCapacity()
    pub fn doubleCapacity(&mut self) {
      // Java: if (this.table.length != 1073741824)
      if self.table.len() != 1073741824 {
         // Java: int newCapacity = this.table.length * 2;
         let newCapacity = self.table.len() * 2;
         // Java: Entry<V>[] newTable = new Entry[newCapacity];
         let mut newTable: Vec<Option<Box<Entry>>> = (0..newCapacity).map(|_| None).collect();
         // Java: this.threshold = (newCapacity >> 1) + (newCapacity >> 2);
         self.threshold = ((newCapacity >> 1) + (newCapacity >> 2)) as i32;
         // Java: if (this.size == 0)
         if self.size == 0 {
            // Java: this.table = newTable;
            self.table = newTable;
         }
         // Java: else
         else {
            // Java: int i = 0;
            let mut i = 0;
            // Java: Entry<V> next;
            // Java: for(int len = this.table.length; i < len; ++i)
            let len = self.table.len();
            while i < len {
               // Java: for(Entry<V> e = this.table[i]; e != null; e = next)
               let mut current = self.table[i].take();
               while let Some(mut e) = current {
                  // Java: next = e.next;
                  let next = e.next.take();
                  // Java: int index = hash(e.key) & newTable.length - 1;
                  let index = (Self::hash(e.key) & (newTable.len() as i32 - 1)) as usize;
                  // Java: e.next = newTable[index];
                  e.next = newTable[index].take();
                  // Java: newTable[index] = e;
                  newTable[index] = Some(e);
                  current = next;
               }
               i += 1;
            }
            // Java: this.table = newTable;
            self.table = newTable;
         }
      }
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
      // Java: StringBuilder sb = new StringBuilder("{");
      let mut sb = String::from("{");
      // Java: for(Entry<V> e : this)
      // LLM: iterating over table entries directly
      for slot in &self.table {
        let mut current = slot;
        while let Some(e) = current {
         // Java: if (sb.length() != 1)
         if sb.len() != 1 {
            // Java: sb.append(", ");
            sb.push_str(", ");
         }
         // Java: sb.append(Integer.toString(e.key));
         sb.push_str(&e.key.to_string());
         // Java: sb.append(":");
         sb.push_str(":");
         // Java: sb.append(e.value);
         sb.push_str("<value>");
         current = &e.next;
        }
      }
      // Java: sb.append("}");
      sb.push_str("}");
      // Java: return sb.toString();
      sb
    }

}

// Java: public class Entry
impl Entry {
    // Java: int getKey()
    pub fn getKey(&self) -> i32 {
         // Java: return this.key;
         self.key
    }

    // Java: V getValue()
    pub fn getValue(&self) -> &Option<Box<dyn Any>> {
         // Java: return this.value;
         &self.value
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
         // Java: return Integer.toString(this.key) + ":" + this.value;
         format!("{}:<value>", self.key)
    }

}
