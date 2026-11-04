#![allow(dead_code, non_snake_case)]

// Java: public class StringIntMap
pub struct StringIntMap {
    // Java field: Entry[] table
    table: Vec<Option<Box<Entry>>>,
    // Java field: int count
    pub(crate) count: i32,
}

// Java: public class Entry
pub struct Entry {
    // Java field: int hash
    pub(crate) hash: i32,
    // Java field: int value
    pub(crate) value: i32,
    // Java field: Entry next
    pub(crate) next: Option<Box<Entry>>,
}

// Java: public class StringIntMap
impl StringIntMap {
    // Java: StringIntMap(int exp)
    pub fn new(_exp: i32) -> Self {
      // Java: super();
      // Java: assert exp > 0;
      assert!(_exp > 0);
      // Java: this.table = new Entry[(int)Math.pow((double)2.0F, (double)exp)];
      let size = 2i32.pow(_exp as u32) as usize;
      StringIntMap {
          table: (0..size).map(|_| None).collect(),
          count: 0,
      }
    }

    // Java: int size()
    pub fn size(&self) -> i32 {
      // Java: return this.count;
      self.count
    }

    // Java: boolean isEmpty()
    pub fn isEmpty(&self) -> bool {
      // Java: return this.count == 0;
      self.count == 0
    }

    // Java: boolean contains(int value)
    pub fn contains(&self, _value: i32) -> bool {
      // Java: Entry[] tab = this.table;
      // Java: int i = tab.length;
      // Java: while(i-- > 0)
      for i in 0..self.table.len()
      {
         // Java: for(Entry e = tab[i]; e != null; e = e.next)
         let mut e = &self.table[i];
         while let Some(ref entry) = e
         {
            // Java: if (e.value == value)
            if entry.value == _value
            {
               // Java: return true;
               return true;
            }
            e = &entry.next;
         }
      }
      // Java: return false;
      false
    }

    // Java: boolean containsValue(int value)
    pub fn containsValue(&self, _value: i32) -> bool {
      // Java: return this.contains(value);
      self.contains(_value)
    }

    // Java: boolean containsKey(String key)
    pub fn containsKey(&self, _key: &str) -> bool {
      // Java: Entry[] tab = this.table;
      // Java: int hash = key.hashCode();
      let hash = Self::hash_str(_key);
      // Java: int index = hash & Integer.MAX_VALUE & this.table.length - 1;
      let index = (hash & i32::MAX) as usize & (self.table.len() - 1);
      // Java: for(Entry e = tab[index]; e != null; e = e.next)
      let mut e = &self.table[index];
      while let Some(ref entry) = e
      {
         // Java: if (e.hash == hash)
         if entry.hash == hash
         {
            // Java: return true;
            return true;
         }
         e = &entry.next;
      }
      // Java: return false;
      false
    }

    // Java: int get(String key)
    pub fn get(&self, _key: &str) -> i32 {
      // Java: Entry[] tab = this.table;
      // Java: int hash = key.hashCode();
      let hash = Self::hash_str(_key);
      // Java: int index = hash & Integer.MAX_VALUE & this.table.length - 1;
      let index = (hash & i32::MAX) as usize & (self.table.len() - 1);
      // Java: for(Entry e = tab[index]; e != null; e = e.next)
      let mut e = &self.table[index];
      while let Some(ref entry) = e
      {
         // Java: if (e.hash == hash)
         if entry.hash == hash
         {
            // Java: return e.value;
            return entry.value;
         }
         e = &entry.next;
      }
      // Java: return -1;
      -1
    }

    // Java: int get(int key)
    pub fn get_by_int(&self, _key: i32) -> i32 {
      // Java: Entry[] tab = this.table;
      // Java: int hash = key;
      let hash = _key;
      // Java: int index = key & Integer.MAX_VALUE & this.table.length - 1;
      let index = (_key & i32::MAX) as usize & (self.table.len() - 1);
      // Java: for(Entry e = tab[index]; e != null; e = e.next)
      let mut e = &self.table[index];
      while let Some(ref entry) = e
      {
         // Java: if (e.hash == hash)
         if entry.hash == hash
         {
            // Java: return e.value;
            return entry.value;
         }
         e = &entry.next;
      }
      // Java: return -1;
      -1
    }

    // Java: void rehash()
    pub fn rehash(&mut self) {
      // Java: int oldCapacity = this.table.length;
      // Java: Entry[] oldMap = this.table;
      // Java: int newCapacity = oldCapacity * 2;
      // Java: Entry[] newMap = new Entry[newCapacity];
      // Java: int i = oldCapacity;
      // Java: Entry e;
      // Java: int index;
      // Java: while(i-- > 0)
      {
         // Java: for(Entry old = oldMap[i]; old != null; newMap[index] = e)
         {
            // Java: e = old;
            // Java: old = old.next;
            // Java: index = e.hash & Integer.MAX_VALUE & newCapacity - 1;
            // Java: e.next = newMap[index];
         }
      }
      todo!();
    }

    // Java: int put(String key, int value)
    pub fn put(&mut self, _key: &str, _value: i32) -> i32 {
      // Java: Entry[] tab = this.table;
      // Java: int hash = key.hashCode();
      let hash = Self::hash_str(_key);
      // Java: int index = hash & Integer.MAX_VALUE & this.table.length - 1;
      let index = (hash & i32::MAX) as usize & (self.table.len() - 1);
      // Java: for(Entry e = tab[index]; e != null; e = e.next)
      {
         // Java: if (e.hash == hash)
         {
            // Java: int old = e.value;
            // Java: e.value = value;
            // Java: return old;
         }
      }
      // Java: if (this.count >= this.table.length)
      {
         // Java: this.rehash();
         // Java: tab = this.table;
         // Java: index = hash & Integer.MAX_VALUE & this.table.length - 1;
      }
      // Java: Entry e = new Entry(hash, key, value, tab[index]);
      // Java: tab[index] = e;
      // Java: ++this.count;
      // Java: return -1;
      todo!();
    }

    // Java: Object remove(String key)
    pub fn remove(&mut self, _key: &str) -> i32 {
      // Java: Entry[] tab = this.table;
      // Java: int hash = key.hashCode();
      // Java: int index = hash & Integer.MAX_VALUE & this.table.length - 1;
      // Java: Entry e = tab[index];
      // Java: for(Entry prev = null; e != null; e = e.next)
      {
         // Java: if (e.hash == hash)
         {
            // Java: if (prev != null)
            {
               // Java: prev.next = e.next;
            }
            // Java: else
            {
               // Java: tab[index] = e.next;
            }
            // Java: --this.count;
            // Java: int oldValue = e.value;
            // Java: e.value = -1;
            // Java: return oldValue;
         }
         // Java: prev = e;
      }
      // Java: return -1;
      todo!();
    }

    // Java: void clear()
    pub fn clear(&mut self) {
      // Java: Entry[] tab = this.table;
      // Java: int index = tab.length;
      // Java: while(true)
      {
         // Java: --index;
         // Java: if (index < 0)
         {
            // Java: this.count = 0;
            // Java: return;
         }
         // Java: tab[index] = null;
      }
      for i in 0..self.table.len() {
          self.table[i] = None;
      }
      self.count = 0;
    }

    fn hash_str(s: &str) -> i32 {
        let mut h: i32 = 0;
        for c in s.chars() {
            h = h.wrapping_mul(31).wrapping_add(c as i32);
        }
        h
    }

}

// Java: public class Entry
impl Entry {
    // Java: Entry(int hash, String key, int value, Entry next)
    pub fn new(_hash: i32, _key: &str, _value: i32, _next: Option<Box<Entry>>) -> Self {
         // Java: super();
         // Java: this.hash = hash;
         // Java: this.value = value;
         // Java: this.next = next;
         Entry { hash: _hash, value: _value, next: _next }
    }

}
