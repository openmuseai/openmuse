#![allow(non_snake_case, non_camel_case_types, unused_variables, unused_imports, dead_code, unreachable_code, unused_mut)]

use std::sync::OnceLock;

// Java field: int[] primes
static PRIMES: OnceLock<Vec<i32>> = OnceLock::new();

// Java: public class CalendarCache
pub struct CalendarCache {
    // Java field: int pIndex
    pIndex: i32,
    // Java field: int size
    size: i32,
    // Java field: int arraySize
    arraySize: i32,
    // Java field: int threshold
    threshold: i32,
    // Java field: long[] keys
    keys: Vec<i64>,
    // Java field: long[] values
    pub(crate) values: Vec<i64>,
}

// Java field: long EMPTY
pub(crate) const EMPTY: i64 = i64::MIN;

// Java: public class CalendarCache
impl CalendarCache {
    // Java: CalendarCache()
    pub fn new() -> Self {
        // Java: super();
        // Java: this.arraySize = primes[this.pIndex];
        // Java: this.threshold = this.arraySize * 3 / 4;
        // Java: this.keys = new long[this.arraySize];
        // Java: this.values = new long[this.arraySize];
        // Java: this.makeArrays(this.arraySize);
        let mut c = CalendarCache {
            pIndex: 0,
            size: 0,
            arraySize: 0,
            threshold: 0,
            keys: Vec::new(),
            values: Vec::new(),
        };
        todo!("CalendarCache::new")
    }

    // Java: void makeArrays(int newSize)
    pub fn makeArrays(&mut self, _new_size: i32) {
        // Java: this.keys = new long[newSize];
        // Java: this.values = new long[newSize];
        // Java: for(int i = 0; i < newSize; ++i)
        {
            // Java: this.values[i] = EMPTY;
        }
        // Java: this.arraySize = newSize;
        // Java: this.threshold = (int)((double)this.arraySize * (double)0.75F);
        // Java: this.size = 0;
        todo!("CalendarCache::makeArrays")
    }

    // Java: long get(long key)
    pub fn get(&self, _key: i64) -> i64 {
        // Java: return this.values[this.findIndex(key)];
        todo!("CalendarCache::get")
    }

    // Java: void put(long key, long value)
    pub fn put(&mut self, _key: i64, _value: i64) {
        // Java: if (this.size >= this.threshold)
        {
            // Java: this.rehash();
        }
        // Java: int index = this.findIndex(key);
        // Java: this.keys[index] = key;
        // Java: this.values[index] = value;
        // Java: ++this.size;
        todo!("CalendarCache::put")
    }

    // Java: int findIndex(long key)
    pub fn findIndex(&self, _key: i64) -> i32 {
        // Java: int index = this.hash(key);
        // Java: for(int delta = 0; this.values[index] != EMPTY && this.keys[index] != key; index = (index + delta) % this.arraySize)
        {
            // Java: if (delta == 0)
            {
                // Java: delta = this.hash2(key);
            }
        }
        // Java: return index;
        todo!("CalendarCache::findIndex")
    }

    // Java: void rehash()
    pub fn rehash(&mut self) {
        // Java: int oldSize = this.arraySize;
        // Java: long[] oldKeys = this.keys;
        // Java: long[] oldValues = this.values;
        // Java: if (this.pIndex < primes.length - 1)
        {
            // Java: this.arraySize = primes[++this.pIndex];
        }
        // Java: else
        {
            // Java: this.arraySize = this.arraySize * 2 + 1;
        }
        // Java: this.size = 0;
        // Java: this.makeArrays(this.arraySize);
        // Java: for(int i = 0; i < oldSize; ++i)
        {
            // Java: if (oldValues[i] != EMPTY)
            {
                // Java: this.put(oldKeys[i], oldValues[i]);
            }
        }
        // Java: long[] var6 = null;
        // Java: long[] var5 = null;
        todo!("CalendarCache::rehash")
    }

    // Java: int hash(long key)
    pub fn hash(&self, _key: i64) -> i32 {
        // Java: int h = (int)((key * 15821L + 1L) % (long)this.arraySize);
        // Java: if (h < 0)
        {
            // Java: h += this.arraySize;
        }
        // Java: return h;
        todo!("CalendarCache::hash")
    }

    // Java: int hash2(long key)
    pub fn hash2(&self, _key: i64) -> i32 {
        // Java: return this.arraySize - 2 - (int)(key % (long)(this.arraySize - 2));
        todo!("CalendarCache::hash2")
    }
}
