// Java: import java.io.Serializable
// LLM No Rust equivalent: no serialization trait needed
// Java: import java.util.Arrays
// LLM No Rust equivalent: using slice methods
// Java: import java.util.Iterator
// LLM No Rust equivalent: using Rust iterators
// Java: import java.util.NoSuchElementException
// LLM No Rust equivalent: using Option/panic

// Java field: long serialVersionUID
pub(crate) const serialVersionUID: i64 = 1;
// Java field: int MINIMUM_CAPACITY
pub(crate) const MINIMUM_CAPACITY: i32 = 4;
// Java field: int MAXIMUM_CAPACITY
pub(crate) const MAXIMUM_CAPACITY: i32 = 1073741824;
// Java field: long serialVersionUID
pub(crate) const ENTRY_SERIAL_VERSION_UID: i64 = 1;

// Java: public class LongObjectHashMap implements Cloneable, Serializable, Iterable
pub struct LongObjectHashMap<V> {
    // Java field: Entry<V>[] table
    table: Vec<Option<Box<Entry<V>>>>,
    // Java field: int size
    pub(crate) size: i32,
    // Java field: int threshold
    threshold: i32,
}

// Java: public class Entry
pub struct Entry<V> {
    // Java field: long key
    pub(crate) key: i64,
    // Java field: V value
    pub(crate) value: V,
    // Java field: Entry<V> next
    pub(crate) next: Option<Box<Entry<V>>>,
}

// Java: public class LongObjectHashMap implements Cloneable, Serializable, Iterable
impl<V> LongObjectHashMap<V> {
    // Java: LongObjectHashMap()
    pub fn LongObjectHashMap_new() -> Self {
      // Java: this(4);
      Self::LongObjectHashMap_with_capacity(4)
    }

    // Java: LongObjectHashMap(int capacity)
    pub fn LongObjectHashMap_with_capacity(mut capacity: i32) -> Self {
      // Java: if (capacity < 4)
      if capacity < 4
      {
         // Java: capacity = 4;
         capacity = 4;
      }
      // Java: else if (capacity > 1073741824)
      else if capacity > 1073741824
      {
         // Java: capacity = 1073741824;
         capacity = 1073741824;
      }
      // Java: else
      else
      {
         // Java: capacity = roundUpToPowerOfTwo(capacity);
         capacity = Self::roundUpToPowerOfTwo(capacity);
      }
      // Java: this.table = new Entry[capacity];
      let mut table = Vec::with_capacity(capacity as usize);
      for _ in 0..capacity { table.push(None); }
      // Java: this.threshold = (capacity >> 1) + (capacity >> 2);
      let threshold = (capacity >> 1) + (capacity >> 2);
      Self { table, size: 0, threshold }
    }

    // Java: V put(long key, V value)
    pub fn put(&mut self, key: i64, value: V) -> Option<V> {
      // Java: int h = hash(key);
      let h = Self::hash(key);
      // Java: Entry<V>[] t = this.table;
      // LLM JavaLine Aggregation: using self.table directly
      // Java: int index = h & t.length - 1;
      let mut index = (h & (self.table.len() as i32 - 1)) as usize;
      // Java: for(Entry<V> e = t[index]; e != null; e = e.next)
      {
         let mut cur = &mut self.table[index];
         loop {
            match cur {
               Some(e) => {
                  // Java: if (e.key == key)
                  if e.key == key
                  {
                     // Java: V oldValue = e.value;
                     // LLM JavaLine Aggregation: merged into replace
                     // Java: e.value = value;
                     // LLM JavaLine Aggregation: merged into replace
                     // Java: return oldValue;
                     let old = std::mem::replace(&mut e.value, value);
                     return Some(old);
                  }
                  cur = &mut cur.as_mut().unwrap().next;
               }
               None => break,
            }
         }
      }
      // Java: if (++this.size > this.threshold)
      self.size += 1;
      if self.size > self.threshold
      {
         // Java: this.doubleCapacity();
         self.doubleCapacity();
         // Java: t = this.table;
         // LLM JavaLine Aggregation: using self.table directly
         // Java: index = h & t.length - 1;
         index = (h & (self.table.len() as i32 - 1)) as usize;
      }
      // Java: t[index] = new Entry(key, value, t[index]);
      let old_head = self.table[index].take();
      self.table[index] = Some(Box::new(Entry { key, value, next: old_head }));
      // Java: return null;
      None
    }

    // Java: void putAll(LongObjectHashMap<? extends V> m)
    pub fn putAll(&mut self, m: &LongObjectHashMap<V>) where V: Clone {
      // Java: if (m.size != 0)
      if m.size != 0
      {
         // Java: Entry<? extends V>[] t = m.table;
         // LLM JavaLine Aggregation: using m.table directly
         // Java: int i = 0;
         // LLM JavaLine Aggregation: merged into for loop
         // Java: for(int len = t.length; i < len; ++i)
         for i in 0..m.table.len()
         {
            // Java: for(Entry<? extends V> e = t[i]; e != null; e = e.next)
            let mut cur = &m.table[i];
            while let Some(e) = cur
            {
               // Java: this.put(e.key, e.value);
               self.put(e.key, e.value.clone());
               cur = &e.next;
            }
         }
      }
    }

    // Java: V get(long key)
    pub fn get(&self, key: i64) -> Option<&V> {
      // Java: int h = hash(key);
      let h = Self::hash(key);
      // Java: Entry<V>[] t = this.table;
      // LLM JavaLine Aggregation: using self.table directly
      // Java: for(Entry<V> e = t[h & t.length - 1]; e != null; e = e.next)
      let index = (h & (self.table.len() as i32 - 1)) as usize;
      let mut cur = &self.table[index];
      while let Some(e) = cur
      {
         // Java: if (e.key == key)
         if e.key == key
         {
            // Java: return e.value;
            return Some(&e.value);
         }
         cur = &e.next;
      }
      // Java: return null;
      None
    }

    // Java: V remove(long key)
    pub fn remove(&mut self, key: i64) -> Option<V> {
      // Java: int h = hash(key);
      let h = Self::hash(key);
      // Java: Entry<V>[] t = this.table;
      // LLM JavaLine Aggregation: using self.table directly
      // Java: int index = h & t.length - 1;
      let index = (h & (self.table.len() as i32 - 1)) as usize;
      // Java: Entry<V> e = t[index];
      // LLM JavaLine Aggregation: merged into cursor loop
      // Java: for(Entry<V> pre = null; e != null; e = e.next)
      let mut cur = &mut self.table[index];
      loop {
         match cur {
            Some(e) if e.key == key => {
               // Java: if (e.key == key)
               // LLM JavaLine Aggregation: merged into match guard
               // Java: if (pre == null)
               // LLM JavaLine Aggregation: merged - Rust handles both cases
               // Java: t[index] = e.next;
               // LLM JavaLine Aggregation: merged into take/replace
               // Java: else
               // LLM No Rust equivalent: else branch
               // Java: pre.next = e.next;
               // LLM JavaLine Aggregation: merged into take/replace
               let mut removed = cur.take().unwrap();
               *cur = removed.next.take();
               // Java: --this.size;
               self.size -= 1;
               // Java: return e.value;
               return Some(removed.value);
            }
            Some(_) => {
               // Java: pre = e;
               // LLM JavaLine Aggregation: advancing cursor
               cur = &mut cur.as_mut().unwrap().next;
            }
            None => break,
         }
      }
      // Java: return null;
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
    pub fn iter(&self) -> Vec<(i64, &V)> {
      // LLM: Java anonymous Iterator inner class has no direct Rust equivalent.
      // LLM: All anonymous class lines (return new Iterator, nextIndex, currentEntry, nextEntry,
      // LLM: advance(), hasNext(), next(), remove()) are replaced by a simple collection loop.
      // LLM JavaLine Aggregation: entire anonymous Iterator class body merged into collection loop
      let mut result = Vec::new();
      for slot in &self.table {
         let mut cur = slot;
         while let Some(e) = cur {
            result.push((e.key, &e.value));
            cur = &e.next;
         }
      }
      result
    }

    // Java: boolean containsKey(long key)
    pub fn containsKey(&self, key: i64) -> bool {
      // Java: int h = hash(key);
      let h = Self::hash(key);
      // Java: Entry<V>[] t = this.table;
      // LLM JavaLine Aggregation: using self.table directly
      // Java: for(Entry<V> e = t[h & t.length - 1]; e != null; e = e.next)
      let index = (h & (self.table.len() as i32 - 1)) as usize;
      let mut cur = &self.table[index];
      while let Some(e) = cur
      {
         // Java: if (e.key == key)
         if e.key == key
         {
            // Java: return true;
            return true;
         }
         cur = &e.next;
      }
      // Java: return false;
      false
    }

    // Java: boolean containsValue(V value)
    pub fn containsValue(&self, value: &V) -> bool where V: PartialEq {
      // Java: Entry<V>[] t = this.table;
      // LLM JavaLine Aggregation: using self.table directly
      // Java: int i = 0;
      // LLM JavaLine Aggregation: merged into for loop
      // Java: for(int len = t.length; i < len; ++i)
      for i in 0..self.table.len()
      {
         // Java: for(Entry<V> e = t[i]; e != null; e = e.next)
         let mut cur = &self.table[i];
         while let Some(e) = cur
         {
            // Java: if (e.value == value || value != null && value.equals(e.value))
            if &e.value == value
            {
               // Java: return true;
               return true;
            }
            cur = &e.next;
         }
      }
      // Java: return false;
      false
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

    // Java: static int hash(long v)
    fn hash(v: i64) -> i32 {
      // Java: int h = (int)(v ^ v >>> 32);
      let mut h = (v ^ ((v as u64 >> 32) as i64)) as i32;
      // Java: h ^= h >>> 20 ^ h >>> 12;
      h ^= ((h as u32 >> 20) as i32) ^ ((h as u32 >> 12) as i32);
      // Java: return h ^ h >>> 7 ^ h >>> 4;
      h ^ ((h as u32 >> 7) as i32) ^ ((h as u32 >> 4) as i32)
    }

    // Java: static int roundUpToPowerOfTwo(int i)
    fn roundUpToPowerOfTwo(mut i: i32) -> i32 {
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
      if self.table.len() != 1073741824
      {
         // Java: int newCapacity = this.table.length * 2;
         let new_capacity = self.table.len() * 2;
         // Java: Entry<V>[] newTable = new Entry[newCapacity];
         let mut new_table: Vec<Option<Box<Entry<V>>>> = Vec::with_capacity(new_capacity);
         for _ in 0..new_capacity { new_table.push(None); }
         // Java: this.threshold = (newCapacity >> 1) + (newCapacity >> 2);
         self.threshold = ((new_capacity >> 1) + (new_capacity >> 2)) as i32;
         // Java: if (this.size == 0)
         if self.size == 0
         {
            // Java: this.table = newTable;
            self.table = new_table;
         }
         // Java: else
         else
         {
            // Java: int i = 0;
            // LLM JavaLine Aggregation: merged into for loop
            // Java: Entry<V> next;
            // LLM JavaLine Aggregation: merged into while loop
            // Java: for(int len = this.table.length; i < len; ++i)
            let old_len = self.table.len();
            for i in 0..old_len
            {
               // Java: for(Entry<V> e = this.table[i]; e != null; e = next)
               let mut e_opt = self.table[i].take();
               while let Some(mut e) = e_opt
               {
                  // Java: next = e.next;
                  let next = e.next.take();
                  // Java: int index = hash(e.key) & newTable.length - 1;
                  let idx = (Self::hash(e.key) & (new_capacity as i32 - 1)) as usize;
                  // Java: e.next = newTable[index];
                  e.next = new_table[idx].take();
                  // Java: newTable[index] = e;
                  new_table[idx] = Some(e);
                  e_opt = next;
               }
            }
            // Java: this.table = newTable;
            self.table = new_table;
         }
      }
    }

    // Java: String toString()
    pub fn toString(&self) -> String where V: std::fmt::Display {
      // Java: StringBuilder sb = new StringBuilder("{");
      let mut sb = String::from("{");
      // Java: for(Entry<V> e : this)
      for slot in &self.table {
         let mut cur = slot;
         while let Some(e) = cur
      {
         // Java: if (sb.length() != 1)
         if sb.len() != 1
         {
            // Java: sb.append(", ");
            sb.push_str(", ");
         }
         // Java: sb.append(Long.toString(e.key));
         sb.push_str(&e.key.to_string());
         // Java: sb.append(":");
         sb.push(':');
         // Java: sb.append(e.value);
         sb.push_str(&e.value.to_string());
         cur = &e.next;
      }
      }
      // Java: sb.append("}");
      sb.push('}');
      // Java: return sb.toString();
      sb
    }

}

// Java: public class Entry
impl<V> Entry<V> {
    // Java: Entry(long k, V v, Entry<V> n)
    pub fn Entry_new(k: i64, v: V, n: Option<Box<Entry<V>>>) -> Self {
         // Java: this.key = k;
         // LLM JavaLine Aggregation: merged into struct init
         // Java: this.value = v;
         // LLM JavaLine Aggregation: merged into struct init
         // Java: this.next = n;
         // LLM JavaLine Aggregation: merged into struct init
         Entry { key: k, value: v, next: n }
    }

    // Java: long getKey()
    pub fn getKey(&self) -> i64 {
         // Java: return this.key;
         self.key
    }

    // Java: V getValue()
    pub fn getValue(&self) -> &V {
         // Java: return this.value;
         &self.value
    }

    // Java: String toString()
    pub fn toString(&self) -> String where V: std::fmt::Display {
         // Java: return Long.toString(this.key) + ":" + this.value;
         format!("{}:{}", self.key, self.value)
    }

}

// LLM: Cloneable, Serializable, Iterable trait stubs
trait Cloneable {}
impl<V> Cloneable for LongObjectHashMap<V> {}
trait Serializable {}
impl<V> Serializable for LongObjectHashMap<V> {}
