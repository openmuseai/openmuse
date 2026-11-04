#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

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

fn string_hash_code(s: &str) -> i32 {
    let mut h: i32 = 0;
    for c in s.chars() {
        h = h.wrapping_mul(31).wrapping_add(c as i32);
    }
    h
}

// Java: public class StringIntMap
impl StringIntMap {
    // Java: StringIntMap(int exp)
    pub fn new(_exp: i32) -> Self {
        // Java: super();
        // Java: assert exp > 0;
        assert!(_exp > 0);
        // Java: this.table = new Entry[(int)Math.pow((double)2.0F, (double)exp)];
        let size = 2_i32.pow(_exp as u32) as usize;
        let mut table = Vec::with_capacity(size);
        for _ in 0..size {
            table.push(None);
        }
        StringIntMap { table, count: 0 }
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
        let tab = &self.table;
        // Java: int i = tab.length;
        let mut i = tab.len() as i32;
        // Java: while(i-- > 0)
        loop {
            i -= 1;
            if i < 0 { break; }
            {
                // Java: for(Entry e = tab[i]; e != null; e = e.next)
                let mut e = &tab[i as usize];
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
        let tab = &self.table;
        // Java: int hash = key.hashCode();
        let hash = string_hash_code(_key);
        // Java: int index = hash & Integer.MAX_VALUE & this.table.length - 1;
        let index = (hash & i32::MAX) as usize & (self.table.len() - 1);
        // Java: for(Entry e = tab[index]; e != null; e = e.next)
        let mut e = &tab[index];
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
        let tab = &self.table;
        // Java: int hash = key.hashCode();
        let hash = string_hash_code(_key);
        // Java: int index = hash & Integer.MAX_VALUE & this.table.length - 1;
        let index = (hash & i32::MAX) as usize & (self.table.len() - 1);
        // Java: for(Entry e = tab[index]; e != null; e = e.next)
        let mut e = &tab[index];
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
    pub fn get_int(&self, _key: i32) -> i32 {
        // Java: Entry[] tab = this.table;
        let tab = &self.table;
        // Java: int hash = key;
        let hash = _key;
        // Java: int index = key & Integer.MAX_VALUE & this.table.length - 1;
        let index = (_key & i32::MAX) as usize & (self.table.len() - 1);
        // Java: for(Entry e = tab[index]; e != null; e = e.next)
        let mut e = &tab[index];
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
        let old_capacity = self.table.len();
        // Java: Entry[] oldMap = this.table;
        let mut old_map: Vec<Option<Box<Entry>>> = std::mem::replace(&mut self.table, Vec::new());
        // Java: int newCapacity = oldCapacity * 2;
        let new_capacity = old_capacity * 2;
        // Java: Entry[] newMap = new Entry[newCapacity];
        let mut new_map: Vec<Option<Box<Entry>>> = Vec::with_capacity(new_capacity);
        for _ in 0..new_capacity {
            new_map.push(None);
        }
        // Java: int i = oldCapacity;
        let mut i = old_capacity as i32;
        // Java: Entry e;
        // Java: int index;
        // Java: while(i-- > 0)
        loop {
            i -= 1;
            if i < 0 { break; }
            {
                // Java: for(Entry old = oldMap[i]; old != null; newMap[index] = e)
                let mut node = old_map[i as usize].take();
                while let Some(mut entry) = node
                {
                    // Java: e = old;
                    // Java: old = old.next;
                    node = entry.next.take();
                    // Java: index = e.hash & Integer.MAX_VALUE & newCapacity - 1;
                    let index = (entry.hash & i32::MAX) as usize & (new_capacity - 1);
                    // Java: e.next = newMap[index];
                    entry.next = new_map[index].take();
                    new_map[index] = Some(entry);
                }
            }
        }
        self.table = new_map;
    }

    // Java: int put(String key, int value)
    pub fn put(&mut self, _key: &str, _value: i32) -> i32 {
        // Java: Entry[] tab = this.table;
        // Java: int hash = key.hashCode();
        let hash = string_hash_code(_key);
        // Java: int index = hash & Integer.MAX_VALUE & this.table.length - 1;
        let mut index = (hash & i32::MAX) as usize & (self.table.len() - 1);
        // Java: for(Entry e = tab[index]; e != null; e = e.next)
        {
            let mut e = &mut self.table[index];
            while let Some(ref mut entry) = e
            {
                // Java: if (e.hash == hash)
                if entry.hash == hash
                {
                    // Java: int old = e.value;
                    let old = entry.value;
                    // Java: e.value = value;
                    entry.value = _value;
                    // Java: return old;
                    return old;
                }
                e = &mut entry.next;
            }
        }
        // Java: if (this.count >= this.table.length)
        if self.count >= self.table.len() as i32
        {
            // Java: this.rehash();
            self.rehash();
            // Java: tab = this.table;
            // Java: index = hash & Integer.MAX_VALUE & this.table.length - 1;
            index = (hash & i32::MAX) as usize & (self.table.len() - 1);
        }
        // Java: Entry e = new Entry(hash, key, value, tab[index]);
        let old_head = self.table[index].take();
        let e = Box::new(Entry { hash, value: _value, next: old_head });
        // Java: tab[index] = e;
        self.table[index] = Some(e);
        // Java: ++this.count;
        self.count += 1;
        // Java: return -1;
        -1
    }

    // Java: Object remove(String key)
    pub fn remove(&mut self, _key: &str) -> i32 {
        // Java: Entry[] tab = this.table;
        // Java: int hash = key.hashCode();
        let hash = string_hash_code(_key);
        // Java: int index = hash & Integer.MAX_VALUE & this.table.length - 1;
        let index = (hash & i32::MAX) as usize & (self.table.len() - 1);
        // Java: Entry e = tab[index];
        // Java: for(Entry prev = null; e != null; e = e.next)
        let mut prev: *mut Option<Box<Entry>> = &mut self.table[index];
        unsafe {
            while let Some(ref mut entry) = *prev
            {
                // Java: if (e.hash == hash)
                if entry.hash == hash
                {
                    // Java: if (prev != null)
                    // Java: prev.next = e.next;
                    // Java: else
                    // Java: tab[index] = e.next;
                    let next = entry.next.take();
                    // Java: --this.count;
                    self.count -= 1;
                    // Java: int oldValue = e.value;
                    let old_value = entry.value;
                    // Java: e.value = -1;
                    // Java: return oldValue;
                    *prev = next;
                    return old_value;
                }
                // Java: prev = e;
                prev = &mut entry.next;
            }
        }
        // Java: return -1;
        -1
    }

    // Java: void clear()
    pub fn clear(&mut self) {
        // Java: Entry[] tab = this.table;
        // Java: int index = tab.length;
        let mut index = self.table.len() as i32;
        // Java: while(true)
        loop
        {
            // Java: --index;
            index -= 1;
            // Java: if (index < 0)
            if index < 0
            {
                // Java: this.count = 0;
                self.count = 0;
                // Java: return;
                return;
            }
            // Java: tab[index] = null;
            self.table[index as usize] = None;
        }
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
