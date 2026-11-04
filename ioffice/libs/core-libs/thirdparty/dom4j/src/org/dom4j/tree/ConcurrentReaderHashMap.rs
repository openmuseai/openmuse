// Java: import java.io.IOException
// Java: import java.io.ObjectInputStream
// Java: import java.io.ObjectOutputStream
// Java: import java.io.Serializable
// Java: import java.util.AbstractCollection
// Java: import java.util.AbstractMap
// Java: import java.util.AbstractSet
// Java: import java.util.Collection
// Java: import java.util.Enumeration
// Java: import java.util.Iterator
// Java: import java.util.Map
// Java: import java.util.NoSuchElementException
// Java: import java.util.Set

use std::collections::HashMap;
use std::sync::RwLock;
use std::any::Any;
use std::sync::Arc;

type Object = Arc<dyn Any + Send + Sync>;

pub const DEFAULT_INITIAL_CAPACITY_VAL: i32 = 32;
pub(crate) const MINIMUM_CAPACITY: i32 = 4;
pub(crate) const MAXIMUM_CAPACITY: i32 = 1073741824;
pub(crate) const DEFAULT_LOAD_FACTOR: f32 = 0.75;

// Java: public class ConcurrentReaderHashMap extends AbstractMap implements Map, Cloneable, Serializable
pub struct ConcurrentReaderHashMap {
    // Java field: BarrierLock barrierLock
    // Java field: Object lastWrite
    // Java field: int DEFAULT_INITIAL_CAPACITY
    // Java field: int MINIMUM_CAPACITY
    // Java field: int MAXIMUM_CAPACITY
    // Java field: float DEFAULT_LOAD_FACTOR
    // Java field: Entry[] table
    // Java field: int count
    // Java field: int threshold
    // Java field: float loadFactor
    // Java field: Set keySet
    // Java field: Set entrySet
    // Java field: Collection values
    pub(crate) inner: RwLock<HashMap<String, Object>>,
    pub(crate) loadFactor: f32,
}

// Java: public class BarrierLock
pub struct BarrierLock {
}

// Java: public class KeySet
pub struct KeySet {
}

// Java: public class Values
pub struct Values {
}

// Java: public class EntrySet
pub struct EntrySet {
}

// Java: public class Entry
pub struct Entry {
    // Java field: int hash
    pub(crate) hash: i32,
    // Java field: Object key
    pub(crate) key: String,
    // Java field: Entry next
    pub(crate) next: Option<Box<Entry>>,
    // Java field: Object value
    pub(crate) value: Option<Object>,
}

// Java: public class HashIterator
pub struct HashIterator {
    // Java field: Entry[] tab
    // Java field: int index
    pub(crate) index: i32,
    // Java field: Entry entry
    // Java field: Object currentKey
    pub(crate) currentKey: Option<String>,
    // Java field: Object currentValue
    pub(crate) currentValue: Option<Object>,
    // Java field: Entry lastReturned
}

// Java: public class KeyIterator
pub struct KeyIterator {
}

// Java: public class ValueIterator
pub struct ValueIterator {
}

// Java: public class ConcurrentReaderHashMap extends AbstractMap implements Map, Cloneable, Serializable
impl ConcurrentReaderHashMap {
    // Java: void recordModification(Object x)
    pub fn recordModification(&self, _x: &Object) {
        // Java: synchronized(this.barrierLock)
        {
            // Java: this.lastWrite = x;
        }
    }

    // Java: Entry[] getTableForReading()
    pub fn getTableForReading(&self) {
        // Java: synchronized(this.barrierLock)
        {
            // Java: return this.table;
        }
    }

    // Java: int p2capacity(int initialCapacity)
    pub fn p2capacity(&self, initial_capacity: i32) -> i32 {
        // Java: int cap = initialCapacity;
        let cap = initial_capacity;
        // Java: int result;
        let result;
        // Java: if (initialCapacity <= 1073741824 && initialCapacity >= 0)
        if initial_capacity <= MAXIMUM_CAPACITY && initial_capacity >= 0
        {
            // Java: for(result = 4; result < cap; result <<= 1)
            result = MINIMUM_CAPACITY;
            let mut r = result;
            while r < cap {
                r <<= 1;
            }
            return r;
        }
        // Java: else
        else
        {
            // Java: result = 1073741824;
            result = MAXIMUM_CAPACITY;
        }
        // Java: return result;
        result
    }

    // Java: static int hash(Object x)
    pub fn hash_key(x: &str) -> i32 {
        // Java: int h = x.hashCode();
        use std::hash::{Hash, Hasher};
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        x.hash(&mut hasher);
        let h = hasher.finish() as i32;
        // Java: return (h << 7) - h + (h >>> 9) + (h >>> 17);
        (h << 7).wrapping_sub(h).wrapping_add((h as u32 >> 9) as i32).wrapping_add((h as u32 >> 17) as i32)
    }

    // Java: boolean eq(Object x, Object y)
    pub fn eq(x: &str, y: &str) -> bool {
        // Java: return x == y || x.equals(y);
        x == y
    }

    // Java: ConcurrentReaderHashMap(int initialCapacity, float loadFactor)
    pub fn new_with_capacity_and_load(initial_capacity: i32, load_factor: f32) -> Self {
        // Java: super();
        // Java: this.barrierLock = new BarrierLock();
        // Java: this.keySet = null;
        // Java: this.entrySet = null;
        // Java: this.values = null;
        // Java: if (loadFactor <= 0.0F)
        if load_factor <= 0.0
        {
            // Java: throw new IllegalArgumentException("Illegal Load factor: " + loadFactor);
            panic!("Illegal Load factor: {}", load_factor);
        }
        // Java: else
        {
            // Java: this.loadFactor = loadFactor;
            // Java: int cap = this.p2capacity(initialCapacity);
            // Java: this.table = new Entry[cap];
            // Java: this.threshold = (int)((float)cap * loadFactor);
            ConcurrentReaderHashMap {
                inner: RwLock::new(HashMap::with_capacity(initial_capacity as usize)),
                loadFactor: load_factor,
            }
        }
    }

    // Java: ConcurrentReaderHashMap(int initialCapacity)
    pub fn new_with_capacity(initial_capacity: i32) -> Self {
        // Java: this(initialCapacity, 0.75F);
        Self::new_with_capacity_and_load(initial_capacity, DEFAULT_LOAD_FACTOR)
    }

    // Java: ConcurrentReaderHashMap()
    pub fn new() -> Self {
        // Java: this(DEFAULT_INITIAL_CAPACITY, 0.75F);
        Self::new_with_capacity_and_load(DEFAULT_INITIAL_CAPACITY_VAL, DEFAULT_LOAD_FACTOR)
    }

    // Java: ConcurrentReaderHashMap(Map t)
    pub fn new_from_map(t: HashMap<String, Object>) -> Self {
        // Java: this(Math.max((int)((float)t.size() / 0.75F) + 1, 16), 0.75F);
        let capacity = std::cmp::max((t.len() as f32 / DEFAULT_LOAD_FACTOR) as i32 + 1, 16);
        let mut map = Self::new_with_capacity_and_load(capacity, DEFAULT_LOAD_FACTOR);
        // Java: this.putAll(t);
        *map.inner.write().unwrap() = t;
        map
    }

    // Java: int size()
    pub fn size(&self) -> i32 {
        // Java: return this.count;
        self.inner.read().unwrap().len() as i32
    }

    // Java: boolean isEmpty()
    pub fn isEmpty(&self) -> bool {
        // Java: return this.count == 0;
        self.inner.read().unwrap().is_empty()
    }

    // Java: Object get(Object key)
    pub fn get(&self, key: &str) -> Option<Object> {
        // Java: int hash = hash(key);
        // Java: Entry[] tab = this.table;
        // Java: int index = hash & tab.length - 1;
        // Java: Entry first = tab[index];
        // Java: Entry e = first;
        // Java: while(true)
        {
            // Java: while(e != null)
            {
                // Java: if (e.hash == hash && this.eq(key, e.key))
                {
                    // Java: Object value = e.value;
                    // Java: if (value != null)
                    {
                        // Java: return value;
                    }
                    // Java: synchronized(this)
                    {
                        // Java: tab = this.table;
                    }
                    // Java: e = first = tab[index = hash & tab.length - 1];
                }
                // Java: else
                {
                    // Java: e = e.next;
                }
            }
            // Java: Entry[] reread = this.getTableForReading();
            // Java: if (tab == reread && first == tab[index])
            {
                // Java: return null;
            }
            // Java: tab = reread;
            // Java: e = first = reread[index = hash & reread.length - 1];
        }
        self.inner.read().unwrap().get(key).cloned()
    }

    // Java: boolean containsKey(Object key)
    pub fn containsKey(&self, key: &str) -> bool {
        // Java: return this.get(key) != null;
        self.get(key).is_some()
    }

    // Java: Object put(Object key, Object value)
    pub fn put(&self, key: String, value: Object) -> Option<Object> {
        // Java: if (value == null)
        // Java: throw new NullPointerException();
        // Java: else
        {
            // Java: int hash = hash(key);
            // Java: Entry[] tab = this.table;
            // Java: int index = hash & tab.length - 1;
            // Java: Entry first = tab[index];
            // Java: Entry e;
            // Java: for(e = first; e != null && (e.hash != hash || !this.eq(key, e.key)); e = e.next)
            // Java: synchronized(this)
            {
                // Java: if (tab == this.table)
                {
                    // Java: if (e == null)
                    {
                        // Java: if (first == tab[index])
                        {
                            // Java: Entry newEntry = new Entry(hash, key, value, first);
                            // Java: tab[index] = newEntry;
                            // Java: if (++this.count >= this.threshold)
                            {
                                // Java: this.rehash();
                            }
                            // Java: else
                            {
                                // Java: this.recordModification(newEntry);
                            }
                            // Java: return null;
                        }
                    }
                    // Java: else
                    {
                        // Java: Object oldValue = e.value;
                        // Java: if (first == tab[index] && oldValue != null)
                        {
                            // Java: e.value = value;
                            // Java: return oldValue;
                        }
                    }
                }
                // Java: return this.sput(key, value, hash);
            }
        }
        self.inner.write().unwrap().insert(key, value)
    }

    // Java: Object sput(Object key, Object value, int hash)
    fn sput(&self, _key: &str, _value: Object, _hash: i32) -> Option<Object> {
        // Java: Entry[] tab = this.table;
        // Java: int index = hash & tab.length - 1;
        // Java: Entry first = tab[index];
        // Java: for(Entry e = first; e != null; e = e.next)
        {
            // Java: if (e.hash == hash && this.eq(key, e.key))
            {
                // Java: Object oldValue = e.value;
                // Java: e.value = value;
                // Java: return oldValue;
            }
        }
        // Java: Entry newEntry = new Entry(hash, key, value, first);
        // Java: tab[index] = newEntry;
        // Java: if (++this.count >= this.threshold)
        {
            // Java: this.rehash();
        }
        // Java: else
        {
            // Java: this.recordModification(newEntry);
        }
        // Java: return null;
        None
    }

    // Java: void rehash()
    fn rehash(&self) {
        // Java: Entry[] oldTable = this.table;
        // Java: int oldCapacity = oldTable.length;
        // Java: if (oldCapacity >= 1073741824)
        {
            // Java: this.threshold = Integer.MAX_VALUE;
        }
        // Java: else
        {
            // Java: int newCapacity = oldCapacity << 1;
            // Java: int mask = newCapacity - 1;
            // Java: this.threshold = (int)((float)newCapacity * this.loadFactor);
            // Java: Entry[] newTable = new Entry[newCapacity];
            // Java: for(int i = 0; i < oldCapacity; ++i)
            {
                // Java: Entry e = oldTable[i];
                // Java: if (e != null)
                {
                    // Java: int idx = e.hash & mask;
                    // Java: Entry next = e.next;
                    // Java: if (next == null)
                    {
                        // Java: newTable[idx] = e;
                    }
                    // Java: else
                    {
                        // Java: Entry lastRun = e;
                        // Java: int lastIdx = idx;
                        // Java: for(Entry last = next; last != null; last = last.next)
                        {
                            // Java: int k = last.hash & mask;
                            // Java: if (k != lastIdx)
                            {
                                // Java: lastIdx = k;
                                // Java: lastRun = last;
                            }
                        }
                        // Java: newTable[lastIdx] = lastRun;
                        // Java: for(Entry p = e; p != lastRun; p = p.next)
                        {
                            // Java: int k = p.hash & mask;
                            // Java: newTable[k] = new Entry(p.hash, p.key, p.value, newTable[k]);
                        }
                    }
                }
            }
            // Java: this.table = newTable;
            // Java: this.recordModification(newTable);
        }
    }

    // Java: Object remove(Object key)
    pub fn remove(&self, key: &str) -> Option<Object> {
        // Java: int hash = hash(key);
        // Java: Entry[] tab = this.table;
        // Java: int index = hash & tab.length - 1;
        // Java: Entry first = tab[index];
        // Java: Entry e;
        // Java: for(e = first; e != null && (e.hash != hash || !this.eq(key, e.key)); e = e.next)
        // Java: synchronized(this)
        {
            // Java: if (tab == this.table)
            {
                // Java: if (e == null)
                {
                    // Java: if (first == tab[index])
                    {
                        // Java: return null;
                    }
                }
                // Java: else
                {
                    // Java: Object oldValue = e.value;
                    // Java: if (first == tab[index] && oldValue != null)
                    {
                        // Java: e.value = null;
                        // Java: --this.count;
                        // Java: Entry head = e.next;
                        // Java: for(Entry p = first; p != e; p = p.next)
                        {
                            // Java: head = new Entry(p.hash, p.key, p.value, head);
                        }
                        // Java: tab[index] = head;
                        // Java: this.recordModification(head);
                        // Java: return oldValue;
                    }
                }
            }
            // Java: return this.sremove(key, hash);
        }
        self.inner.write().unwrap().remove(key)
    }

    // Java: Object sremove(Object key, int hash)
    fn sremove(&self, _key: &str, _hash: i32) -> Option<Object> {
        // Java: Entry[] tab = this.table;
        // Java: int index = hash & tab.length - 1;
        // Java: Entry first = tab[index];
        // Java: for(Entry e = first; e != null; e = e.next)
        {
            // Java: if (e.hash == hash && this.eq(key, e.key))
            {
                // Java: Object oldValue = e.value;
                // Java: e.value = null;
                // Java: --this.count;
                // Java: Entry head = e.next;
                // Java: for(Entry p = first; p != e; p = p.next)
                {
                    // Java: head = new Entry(p.hash, p.key, p.value, head);
                }
                // Java: tab[index] = head;
                // Java: this.recordModification(head);
                // Java: return oldValue;
            }
        }
        // Java: return null;
        None
    }

    // Java: boolean containsValue(Object value)
    pub fn containsValue(&self, _value: &Object) -> bool {
        // Java: if (value == null)
        {
            // Java: throw new NullPointerException();
        }
        // Java: else
        {
            // Java: Entry[] tab = this.getTableForReading();
            // Java: for(int i = 0; i < tab.length; ++i)
            {
                // Java: for(Entry e = tab[i]; e != null; e = e.next)
                {
                    // Java: if (value.equals(e.value))
                    {
                        // Java: return true;
                    }
                }
            }
            // Java: return false;
        }
        false
    }

    // Java: boolean contains(Object value)
    pub fn contains(&self, value: &Object) -> bool {
        // Java: return this.containsValue(value);
        self.containsValue(value)
    }

    // Java: void putAll(Map t)
    pub fn putAll(&self, t: HashMap<String, Object>) {
        // Java: int n = t.size();
        // Java: if (n != 0)
        if !t.is_empty()
        {
            // Java: while(n >= this.threshold)
            {
                // Java: this.rehash();
            }
            // Java: for(Object obj : t.entrySet())
            for (key, value) in t
            {
                // Java: Map.Entry entry = (Map.Entry)obj;
                // Java: Object key = entry.getKey();
                // Java: Object value = entry.getValue();
                // Java: this.put(key, value);
                self.put(key, value);
            }
        }
    }

    // Java: void clear()
    pub fn clear(&self) {
        // Java: Entry[] tab = this.table;
        // Java: for(int i = 0; i < tab.length; ++i)
        {
            // Java: for(Entry e = tab[i]; e != null; e = e.next)
            {
                // Java: e.value = null;
            }
            // Java: tab[i] = null;
        }
        // Java: this.count = 0;
        // Java: this.recordModification(tab);
        self.inner.write().unwrap().clear();
    }

    // Java: Object clone()
    pub fn clone_map(&self) -> Self {
        // Java: try
        {
            // Java: ConcurrentReaderHashMap t = (ConcurrentReaderHashMap)super.clone();
            // Java: t.keySet = null;
            // Java: t.entrySet = null;
            // Java: t.values = null;
            // Java: Entry[] tab = this.table;
            // Java: t.table = new Entry[tab.length];
            // Java: Entry[] ttab = t.table;
            // Java: for(int i = 0; i < tab.length; ++i)
            {
                // Java: Entry first = null;
                // Java: for(Entry e = tab[i]; e != null; e = e.next)
                {
                    // Java: first = new Entry(e.hash, e.key, e.value, first);
                }
                // Java: ttab[i] = first;
            }
            // Java: return t;
        }
        // Java: catch (CloneNotSupportedException var7)
        {
            // Java: throw new InternalError();
        }
        ConcurrentReaderHashMap {
            inner: RwLock::new(self.inner.read().unwrap().clone()),
            loadFactor: self.loadFactor,
        }
    }

    // Java: Set keySet()
    pub fn key_set(&self) -> Vec<String> {
        // Java: Set ks = this.keySet;
        // Java: return ks != null ? ks : (this.keySet = new KeySet());
        self.inner.read().unwrap().keys().cloned().collect()
    }

    // Java: Collection values()
    pub fn values_list(&self) -> Vec<Object> {
        // Java: Collection vs = this.values;
        // Java: return vs != null ? vs : (this.values = new Values());
        self.inner.read().unwrap().values().cloned().collect()
    }

    // Java: Set entrySet()
    pub fn entry_set(&self) -> Vec<(String, Object)> {
        // Java: Set es = this.entrySet;
        // Java: return es != null ? es : (this.entrySet = new EntrySet());
        self.inner.read().unwrap().iter().map(|(k, v)| (k.clone(), v.clone())).collect()
    }

    // Java: boolean findAndRemoveEntry(Map.Entry entry)
    pub fn findAndRemoveEntry(&self, key: &str, expected_value: &Object) -> bool {
        // Java: Object key = entry.getKey();
        // Java: Object v = this.get(key);
        let v = self.get(key);
        // Java: if (v != null && v.equals(entry.getValue()))
        if v.is_some()
        {
            // Java: this.remove(key);
            self.remove(key);
            // Java: return true;
            true
        }
        // Java: else
        else
        {
            // Java: return false;
            false
        }
    }

    // Java: Enumeration keys()
    pub fn keys(&self) -> Vec<String> {
        // Java: return new KeyIterator();
        self.key_set()
    }

    // Java: Enumeration elements()
    pub fn elements(&self) -> Vec<Object> {
        // Java: return new ValueIterator();
        self.values_list()
    }

    // Java: void writeObject(ObjectOutputStream s)
    pub fn writeObject(&self) {
        // Java: s.defaultWriteObject();
        // Java: s.writeInt(this.table.length);
        // Java: s.writeInt(this.count);
        // Java: for(int index = this.table.length - 1; index >= 0; --index)
        {
            // Java: for(Entry entry = this.table[index]; entry != null; entry = entry.next)
            {
                // Java: s.writeObject(entry.key);
                // Java: s.writeObject(entry.value);
            }
        }
    }

    // Java: void readObject(ObjectInputStream s)
    pub fn readObject(&self) {
        // Java: s.defaultReadObject();
        // Java: int numBuckets = s.readInt();
        // Java: this.table = new Entry[numBuckets];
        // Java: int size = s.readInt();
        // Java: for(int i = 0; i < size; ++i)
        {
            // Java: Object key = s.readObject();
            // Java: Object value = s.readObject();
            // Java: this.put(key, value);
        }
    }

    // Java: int capacity()
    pub fn capacity(&self) -> i32 {
        // Java: return this.table.length;
        self.inner.read().unwrap().capacity() as i32
    }

    // Java: float loadFactor()
    pub fn loadFactor(&self) -> f32 {
        // Java: return this.loadFactor;
        self.loadFactor
    }
}

// Java: public class BarrierLock
impl BarrierLock {
    // Java: BarrierLock()
    pub fn new() -> Self {
        // Java: super();
        BarrierLock {}
    }
}

// Java: public class KeySet
impl KeySet {
    // Java: KeySet()
    pub fn new() -> Self {
        // Java: super();
        KeySet {}
    }

    // Java: Iterator iterator()
    pub fn iterator(&self) {
        // Java: return ConcurrentReaderHashMap.this.new KeyIterator();
    }

    // Java: int size()
    pub fn size(&self) -> i32 {
        // Java: return ConcurrentReaderHashMap.this.size();
        0
    }

    // Java: boolean contains(Object o)
    pub fn contains(&self, _o: &Object) -> bool {
        // Java: return ConcurrentReaderHashMap.this.containsKey(o);
        false
    }

    // Java: boolean remove(Object o)
    pub fn remove(&self, _o: &Object) -> bool {
        // Java: return ConcurrentReaderHashMap.this.remove(o) != null;
        false
    }

    // Java: void clear()
    pub fn clear(&self) {
        // Java: ConcurrentReaderHashMap.this.clear();
    }
}

// Java: public class Values
impl Values {
    // Java: Values()
    pub fn new() -> Self {
        // Java: super();
        Values {}
    }

    // Java: Iterator iterator()
    pub fn iterator(&self) {
        // Java: return ConcurrentReaderHashMap.this.new ValueIterator();
    }

    // Java: int size()
    pub fn size(&self) -> i32 {
        // Java: return ConcurrentReaderHashMap.this.size();
        0
    }

    // Java: boolean contains(Object o)
    pub fn contains(&self, _o: &Object) -> bool {
        // Java: return ConcurrentReaderHashMap.this.containsValue(o);
        false
    }

    // Java: void clear()
    pub fn clear(&self) {
        // Java: ConcurrentReaderHashMap.this.clear();
    }
}

// Java: public class EntrySet
impl EntrySet {
    // Java: EntrySet()
    pub fn new() -> Self {
        // Java: super();
        EntrySet {}
    }

    // Java: Iterator iterator()
    pub fn iterator(&self) {
        // Java: return ConcurrentReaderHashMap.this.new HashIterator();
    }

    // Java: boolean contains(Object o)
    pub fn contains(&self, _o: &Object) -> bool {
        // Java: if (!(o instanceof Map.Entry))
        {
            // Java: return false;
        }
        // Java: else
        {
            // Java: Map.Entry entry = (Map.Entry)o;
            // Java: Object v = ConcurrentReaderHashMap.this.get(entry.getKey());
            // Java: return v != null && v.equals(entry.getValue());
        }
        false
    }

    // Java: boolean remove(Object o)
    pub fn remove(&self, _o: &Object) -> bool {
        // Java: return !(o instanceof Map.Entry) ? false : ConcurrentReaderHashMap.this.findAndRemoveEntry((Map.Entry)o);
        false
    }

    // Java: int size()
    pub fn size(&self) -> i32 {
        // Java: return ConcurrentReaderHashMap.this.size();
        0
    }

    // Java: void clear()
    pub fn clear(&self) {
        // Java: ConcurrentReaderHashMap.this.clear();
    }
}

// Java: public class Entry
impl Entry {
    // Java: Object getKey()
    pub fn getKey(&self) -> &str {
        // Java: return this.key;
        &self.key
    }

    // Java: Object getValue()
    pub fn getValue(&self) -> &Option<Object> {
        // Java: return this.value;
        &self.value
    }

    // Java: Object setValue(Object value)
    pub fn setValue(&mut self, value: Object) -> Option<Object> {
        // Java: if (value == null)
        {
            // Java: throw new NullPointerException();
        }
        // Java: else
        {
            // Java: Object oldValue = this.value;
            let old_value = self.value.take();
            // Java: this.value = value;
            self.value = Some(value);
            // Java: return oldValue;
            return old_value;
        }
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, _o: &Entry) -> bool {
        // Java: if (!(o instanceof Map.Entry))
        {
            // Java: return false;
        }
        // Java: else
        {
            // Java: Map.Entry e = (Map.Entry)o;
            // Java: return this.key.equals(e.getKey()) && this.value.equals(e.getValue());
        }
        false
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
        // Java: return this.key.hashCode() ^ this.value.hashCode();
        0
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
        // Java: return this.key + "=" + this.value;
        format!("{}=<value>", self.key)
    }
}

// Java: public class HashIterator
impl HashIterator {
    // Java: HashIterator()
    pub fn new() -> Self {
        // Java: super();
        // Java: this.index = this.tab.length - 1;
        HashIterator {
            index: 0,
            currentKey: None,
            currentValue: None,
        }
    }

    // Java: boolean hasMoreElements()
    pub fn hasMoreElements(&self) -> bool {
        // Java: return this.hasNext();
        self.hasNext()
    }

    // Java: Object nextElement()
    pub fn nextElement(&self) -> Option<Object> {
        // Java: return this.next();
        None
    }

    // Java: boolean hasNext()
    pub fn hasNext(&self) -> bool {
        // Java: do
        {
            // Java: if (this.entry != null)
            {
                // Java: Object v = this.entry.value;
                // Java: if (v != null)
                {
                    // Java: this.currentKey = this.entry.key;
                    // Java: this.currentValue = v;
                    // Java: return true;
                }
                // Java: this.entry = this.entry.next;
            }
            // Java: while(this.entry == null && this.index >= 0)
            {
                // Java: this.entry = this.tab[this.index--];
            }
        }
        // Java: this.currentKey = this.currentValue = null;
        // Java: return false;
        false
    }

    // Java: Object returnValueOfNext()
    pub fn returnValueOfNext(&self) -> Option<Object> {
        // Java: return this.entry;
        None
    }

    // Java: Object next()
    pub fn next_item(&mut self) -> Option<Object> {
        // Java: if (this.currentKey == null && !this.hasNext())
        if self.currentKey.is_none() && !self.hasNext()
        {
            // Java: throw new NoSuchElementException();
            panic!("NoSuchElementException");
        }
        // Java: else
        else
        {
            // Java: Object result = this.returnValueOfNext();
            let result = self.returnValueOfNext();
            // Java: this.lastReturned = this.entry;
            // Java: this.currentKey = this.currentValue = null;
            self.currentKey = None;
            self.currentValue = None;
            // Java: this.entry = this.entry.next;
            // Java: return result;
            result
        }
    }

    // Java: void remove()
    pub fn remove(&self) {
        // Java: if (this.lastReturned == null)
        {
            // Java: throw new IllegalStateException();
        }
        // Java: else
        {
            // Java: ConcurrentReaderHashMap.this.remove(this.lastReturned.key);
            // Java: this.lastReturned = null;
        }
    }
}

// Java: public class KeyIterator
impl KeyIterator {
    // Java: KeyIterator()
    pub fn new() -> Self {
        // Java: super();
        KeyIterator {}
    }

    // Java: Object returnValueOfNext()
    pub fn returnValueOfNext(&self) -> Option<String> {
        // Java: return this.currentKey;
        None
    }
}

// Java: public class ValueIterator
impl ValueIterator {
    // Java: ValueIterator()
    pub fn new() -> Self {
        // Java: super();
        ValueIterator {}
    }

    // Java: Object returnValueOfNext()
    pub fn returnValueOfNext(&self) -> Option<Object> {
        // Java: return this.currentValue;
        None
    }
}
