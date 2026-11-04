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

// Java field: int DEFAULT_INITIAL_CAPACITY
static DEFAULT_INITIAL_CAPACITY: std::sync::OnceLock<std::sync::Mutex<i32>> = std::sync::OnceLock::new();
// Java field: int MINIMUM_CAPACITY
pub(crate) const MINIMUM_CAPACITY: i32 = 4;
// Java field: int MAXIMUM_CAPACITY
pub(crate) const MAXIMUM_CAPACITY: i32 = 1073741824;
// Java field: float DEFAULT_LOAD_FACTOR
pub(crate) const DEFAULT_LOAD_FACTOR: f32 = 0.75;

// Java: public class ConcurrentReaderHashMap extends AbstractMap implements Map, Cloneable, Serializable
pub struct ConcurrentReaderHashMap {
    // Java field: BarrierLock barrierLock
    pub(crate) barrierLock: Option<Box<BarrierLock>>,
    // Java field: Object lastWrite
    pub(crate) lastWrite: Option<Box<dyn std::any::Any>>,
    // Java field: Entry[] table
    pub(crate) table: Vec<Option<Box<HashEntry>>>,
    // Java field: int count
    pub(crate) count: i32,
    // Java field: int threshold
    pub(crate) threshold: i32,
    // Java field: float loadFactor
    pub(crate) loadFactor: f32,
    // Java field: Set keySet
    pub(crate) keySet: Option<Box<dyn std::any::Any>>,
    // Java field: Set entrySet
    pub(crate) entrySet: Option<Box<dyn std::any::Any>>,
    // Java field: Collection values
    pub(crate) values: Option<Box<dyn std::any::Any>>,
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
pub struct HashEntry {
    // Java field: int hash
    pub(crate) hash: i32,
    // Java field: Object key
    pub(crate) key: Option<Box<dyn std::any::Any>>,
    // Java field: Entry next
    pub(crate) next: Option<Box<HashEntry>>,
    // Java field: Object value
    pub(crate) value: Option<Box<dyn std::any::Any>>,
}

// Java: public class HashIterator
pub struct HashIterator {
    // Java field: Entry[] tab
    pub(crate) tab: Vec<Option<Box<HashEntry>>>,
    // Java field: int index
    pub(crate) index: i32,
    // Java field: Entry entry
    pub(crate) entry: Option<Box<dyn std::any::Any>>,
    // Java field: Object currentKey
    pub(crate) currentKey: Option<Box<dyn std::any::Any>>,
    // Java field: Object currentValue
    pub(crate) currentValue: Option<Box<dyn std::any::Any>>,
    // Java field: Entry lastReturned
    pub(crate) lastReturned: Option<Box<dyn std::any::Any>>,
}

// Java: public class KeyIterator
pub struct KeyIterator {
}

// Java: public class ValueIterator
pub struct ValueIterator {
}

// Java: public class ConcurrentReaderHashMap extends AbstractMap implements Map, Cloneable, Serializable
impl ConcurrentReaderHashMap {
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

    // Java: void recordModification(Object x)
    pub fn recordModification(&mut self) {
      // Java: synchronized(this.barrierLock)
      todo!();
      {
         // Java: this.lastWrite = x;
      todo!();
      }
    }

    // Java: Entry[] getTableForReading()
    pub fn getTableForReading(&mut self) -> Vec<Option<Box<dyn std::any::Any>>> {
      // Java: synchronized(this.barrierLock)
      todo!();
      {
         // Java: return this.table;
      todo!();
      }
    }

    // Java: int p2capacity(int initialCapacity)
    pub fn p2capacity(&mut self) -> i32 {
      // Java: int cap = initialCapacity;
      todo!();
      // Java: int result;
      todo!();
      // Java: if (initialCapacity <= 1073741824 && initialCapacity >= 0)
      todo!();
      {
         // Java: for(result = 4; result < cap; result <<= 1)
      todo!();
         {
         }
      }
      // Java: else
      todo!();
      {
         // Java: result = 1073741824;
      todo!();
      }
      // Java: return result;
      todo!();
    }

    // Java: static int hash(Object x)
    pub fn hash(&mut self) -> i32 {
      // Java: int h = x.hashCode();
      todo!();
      // Java: return (h << 7) - h + (h >>> 9) + (h >>> 17);
      todo!();
    }

    // Java: boolean eq(Object x, Object y)
    pub fn eq(&mut self) -> bool {
      // Java: return x == y || x.equals(y);
      todo!();
    }

    // Java: ConcurrentReaderHashMap(int initialCapacity, float loadFactor)
    pub fn new() -> Self {
      // Java: super();
      todo!();
      // Java: this.barrierLock = new BarrierLock();
      todo!();
      // Java: this.keySet = null;
      todo!();
      // Java: this.entrySet = null;
      todo!();
      // Java: this.values = null;
      todo!();
      // Java: if (loadFactor <= 0.0F)
      todo!();
      {
         // Java: throw new IllegalArgumentException("Illegal Load factor: " + loadFactor);
      todo!();
      }
      // Java: else
      todo!();
      {
         // Java: this.loadFactor = loadFactor;
      todo!();
         // Java: int cap = this.p2capacity(initialCapacity);
      todo!();
         // Java: this.table = new Entry[cap];
      todo!();
         // Java: this.threshold = (int)((float)cap * loadFactor);
      todo!();
      }
    }

    // Java: ConcurrentReaderHashMap(int initialCapacity)
    pub fn new_1() -> Self {
      // Java: this(initialCapacity, 0.75F);
      todo!();
    }

    // Java: ConcurrentReaderHashMap()
    pub fn new_2() -> Self {
      // Java: this(DEFAULT_INITIAL_CAPACITY, 0.75F);
      todo!();
    }

    // Java: ConcurrentReaderHashMap(Map t)
    pub fn new_3() -> Self {
      // Java: this(Math.max((int)((float)t.size() / 0.75F) + 1, 16), 0.75F);
      todo!();
      // Java: this.putAll(t);
      todo!();
    }

    // Java: int size()
    pub fn size(&mut self) -> i32 {
      // Java: return this.count;
      todo!();
    }

    // Java: boolean isEmpty()
    pub fn isEmpty(&mut self) -> bool {
      // Java: return this.count == 0;
      todo!();
    }

    // Java: Object get(Object key)
    pub fn get(&mut self) -> Option<Box<dyn std::any::Any>> {
      // Java: int hash = hash(key);
      todo!();
      // Java: Entry[] tab = this.table;
      todo!();
      // Java: int index = hash & tab.length - 1;
      todo!();
      // Java: Entry first = tab[index];
      todo!();
      // Java: Entry e = first;
      todo!();
      // Java: while(true)
      todo!();
      {
         // Java: while(e != null)
      todo!();
         {
            // Java: if (e.hash == hash && this.eq(key, e.key))
      todo!();
            {
               // Java: Object value = e.value;
      todo!();
               // Java: if (value != null)
      todo!();
               {
                  // Java: return value;
      todo!();
               }
               // Java: synchronized(this)
      todo!();
               {
                  // Java: tab = this.table;
      todo!();
               }
               // Java: e = first = tab[index = hash & tab.length - 1];
      todo!();
            }
            // Java: else
      todo!();
            {
               // Java: e = e.next;
      todo!();
            }
         }
         // Java: Entry[] reread = this.getTableForReading();
      todo!();
         // Java: if (tab == reread && first == tab[index])
      todo!();
         {
            // Java: return null;
      todo!();
         }
         // Java: tab = reread;
      todo!();
         // Java: e = first = reread[index = hash & reread.length - 1];
      todo!();
      }
    }

    // Java: boolean containsKey(Object key)
    pub fn containsKey(&mut self) -> bool {
      // Java: return this.get(key) != null;
      todo!();
    }

    // Java: Object put(Object key, Object value)
    pub fn put(&mut self) -> Option<Box<dyn std::any::Any>> {
      // Java: if (value == null)
      todo!();
      {
         // Java: throw new NullPointerException();
      todo!();
      }
      // Java: else
      todo!();
      {
         // Java: int hash = hash(key);
      todo!();
         // Java: Entry[] tab = this.table;
      todo!();
         // Java: int index = hash & tab.length - 1;
      todo!();
         // Java: Entry first = tab[index];
      todo!();
         // Java: Entry e;
      todo!();
         // Java: for(e = first; e != null && (e.hash != hash || !this.eq(key, e.key)); e = e.next)
      todo!();
         {
         }
         // Java: synchronized(this)
      todo!();
         {
            // Java: if (tab == this.table)
      todo!();
            {
               // Java: if (e == null)
      todo!();
               {
                  // Java: if (first == tab[index])
      todo!();
                  {
                     // Java: Entry newEntry = new Entry(hash, key, value, first);
      todo!();
                     // Java: tab[index] = newEntry;
      todo!();
                     // Java: if (++this.count >= this.threshold)
      todo!();
                     {
                        // Java: this.rehash();
      todo!();
                     }
                     // Java: else
      todo!();
                     {
                        // Java: this.recordModification(newEntry);
      todo!();
                     }
                     // Java: return null;
      todo!();
                  }
               }
               // Java: else
      todo!();
               {
                  // Java: Object oldValue = e.value;
      todo!();
                  // Java: if (first == tab[index] && oldValue != null)
      todo!();
                  {
                     // Java: e.value = value;
      todo!();
                     // Java: return oldValue;
      todo!();
                  }
               }
            }
            // Java: return this.sput(key, value, hash);
      todo!();
         }
      }
    }

    // Java: Object sput(Object key, Object value, int hash)
    pub fn sput(&mut self) -> Option<Box<dyn std::any::Any>> {
      // Java: Entry[] tab = this.table;
      todo!();
      // Java: int index = hash & tab.length - 1;
      todo!();
      // Java: Entry first = tab[index];
      todo!();
      // Java: for(Entry e = first; e != null; e = e.next)
      todo!();
      {
         // Java: if (e.hash == hash && this.eq(key, e.key))
      todo!();
         {
            // Java: Object oldValue = e.value;
      todo!();
            // Java: e.value = value;
      todo!();
            // Java: return oldValue;
      todo!();
         }
      }
      // Java: Entry newEntry = new Entry(hash, key, value, first);
      todo!();
      // Java: tab[index] = newEntry;
      todo!();
      // Java: if (++this.count >= this.threshold)
      todo!();
      {
         // Java: this.rehash();
      todo!();
      }
      // Java: else
      todo!();
      {
         // Java: this.recordModification(newEntry);
      todo!();
      }
      // Java: return null;
      todo!();
    }

    // Java: void rehash()
    pub fn rehash(&mut self) {
      // Java: Entry[] oldTable = this.table;
      todo!();
      // Java: int oldCapacity = oldTable.length;
      todo!();
      // Java: if (oldCapacity >= 1073741824)
      todo!();
      {
         // Java: this.threshold = Integer.MAX_VALUE;
      todo!();
      }
      // Java: else
      todo!();
      {
         // Java: int newCapacity = oldCapacity << 1;
      todo!();
         // Java: int mask = newCapacity - 1;
      todo!();
         // Java: this.threshold = (int)((float)newCapacity * this.loadFactor);
      todo!();
         // Java: Entry[] newTable = new Entry[newCapacity];
      todo!();
         // Java: for(int i = 0; i < oldCapacity; ++i)
      todo!();
         {
            // Java: Entry e = oldTable[i];
      todo!();
            // Java: if (e != null)
      todo!();
            {
               // Java: int idx = e.hash & mask;
      todo!();
               // Java: Entry next = e.next;
      todo!();
               // Java: if (next == null)
      todo!();
               {
                  // Java: newTable[idx] = e;
      todo!();
               }
               // Java: else
      todo!();
               {
                  // Java: Entry lastRun = e;
      todo!();
                  // Java: int lastIdx = idx;
      todo!();
                  // Java: for(Entry last = next; last != null; last = last.next)
      todo!();
                  {
                     // Java: int k = last.hash & mask;
      todo!();
                     // Java: if (k != lastIdx)
      todo!();
                     {
                        // Java: lastIdx = k;
      todo!();
                        // Java: lastRun = last;
      todo!();
                     }
                  }
                  // Java: newTable[lastIdx] = lastRun;
      todo!();
                  // Java: for(Entry p = e; p != lastRun; p = p.next)
      todo!();
                  {
                     // Java: int k = p.hash & mask;
      todo!();
                     // Java: newTable[k] = new Entry(p.hash, p.key, p.value, newTable[k]);
      todo!();
                  }
               }
            }
         }
         // Java: this.table = newTable;
      todo!();
         // Java: this.recordModification(newTable);
      todo!();
      }
    }

    // Java: Object remove(Object key)
    pub fn remove(&mut self) -> Option<Box<dyn std::any::Any>> {
      // Java: int hash = hash(key);
      todo!();
      // Java: Entry[] tab = this.table;
      todo!();
      // Java: int index = hash & tab.length - 1;
      todo!();
      // Java: Entry first = tab[index];
      todo!();
      // Java: Entry e;
      todo!();
      // Java: for(e = first; e != null && (e.hash != hash || !this.eq(key, e.key)); e = e.next)
      todo!();
      {
      }
      // Java: synchronized(this)
      todo!();
      {
         // Java: if (tab == this.table)
      todo!();
         {
            // Java: if (e == null)
      todo!();
            {
               // Java: if (first == tab[index])
      todo!();
               {
                  // Java: return null;
      todo!();
               }
            }
            // Java: else
      todo!();
            {
               // Java: Object oldValue = e.value;
      todo!();
               // Java: if (first == tab[index] && oldValue != null)
      todo!();
               {
                  // Java: e.value = null;
      todo!();
                  // Java: --this.count;
      todo!();
                  // Java: Entry head = e.next;
      todo!();
                  // Java: for(Entry p = first; p != e; p = p.next)
      todo!();
                  {
                     // Java: head = new Entry(p.hash, p.key, p.value, head);
      todo!();
                  }
                  // Java: tab[index] = head;
      todo!();
                  // Java: this.recordModification(head);
      todo!();
                  // Java: return oldValue;
      todo!();
               }
            }
         }
         // Java: return this.sremove(key, hash);
      todo!();
      }
    }

    // Java: Object sremove(Object key, int hash)
    pub fn sremove(&mut self) -> Option<Box<dyn std::any::Any>> {
      // Java: Entry[] tab = this.table;
      todo!();
      // Java: int index = hash & tab.length - 1;
      todo!();
      // Java: Entry first = tab[index];
      todo!();
      // Java: for(Entry e = first; e != null; e = e.next)
      todo!();
      {
         // Java: if (e.hash == hash && this.eq(key, e.key))
      todo!();
         {
            // Java: Object oldValue = e.value;
      todo!();
            // Java: e.value = null;
      todo!();
            // Java: --this.count;
      todo!();
            // Java: Entry head = e.next;
      todo!();
            // Java: for(Entry p = first; p != e; p = p.next)
      todo!();
            {
               // Java: head = new Entry(p.hash, p.key, p.value, head);
      todo!();
            }
            // Java: tab[index] = head;
      todo!();
            // Java: this.recordModification(head);
      todo!();
            // Java: return oldValue;
      todo!();
         }
      }
      // Java: return null;
      todo!();
    }

    // Java: boolean containsValue(Object value)
    pub fn containsValue(&mut self) -> bool {
      // Java: if (value == null)
      todo!();
      {
         // Java: throw new NullPointerException();
      todo!();
      }
      // Java: else
      todo!();
      {
         // Java: Entry[] tab = this.getTableForReading();
      todo!();
         // Java: for(int i = 0; i < tab.length; ++i)
      todo!();
         {
            // Java: for(Entry e = tab[i]; e != null; e = e.next)
      todo!();
            {
               // Java: if (value.equals(e.value))
      todo!();
               {
                  // Java: return true;
      todo!();
               }
            }
         }
         // Java: return false;
      todo!();
      }
    }

    // Java: boolean contains(Object value)
    pub fn contains(&mut self) -> bool {
      // Java: return this.containsValue(value);
      todo!();
    }

    // Java: void putAll(Map t)
    pub fn putAll(&mut self) {
      // Java: int n = t.size();
      todo!();
      // Java: if (n != 0)
      todo!();
      {
         // Java: while(n >= this.threshold)
      todo!();
         {
            // Java: this.rehash();
      todo!();
         }
         // Java: for(Object obj : t.entrySet())
      todo!();
         {
            // Java: Map.Entry entry = (Map.Entry)obj;
      todo!();
            // Java: Object key = entry.getKey();
      todo!();
            // Java: Object value = entry.getValue();
      todo!();
            // Java: this.put(key, value);
      todo!();
         }
      }
    }

    // Java: void clear()
    pub fn clear(&mut self) {
      // Java: Entry[] tab = this.table;
      todo!();
      // Java: for(int i = 0; i < tab.length; ++i)
      todo!();
      {
         // Java: for(Entry e = tab[i]; e != null; e = e.next)
      todo!();
         {
            // Java: e.value = null;
      todo!();
         }
         // Java: tab[i] = null;
      todo!();
      }
      // Java: this.count = 0;
      todo!();
      // Java: this.recordModification(tab);
      todo!();
    }

    // Java: Object clone()
    pub fn clone(&mut self) -> Option<Box<dyn std::any::Any>> {
      // Java: try
      todo!();
      {
         // Java: ConcurrentReaderHashMap t = (ConcurrentReaderHashMap)super.clone();
      todo!();
         // Java: t.keySet = null;
      todo!();
         // Java: t.entrySet = null;
      todo!();
         // Java: t.values = null;
      todo!();
         // Java: Entry[] tab = this.table;
      todo!();
         // Java: t.table = new Entry[tab.length];
      todo!();
         // Java: Entry[] ttab = t.table;
      todo!();
         // Java: for(int i = 0; i < tab.length; ++i)
      todo!();
         {
            // Java: Entry first = null;
      todo!();
            // Java: for(Entry e = tab[i]; e != null; e = e.next)
      todo!();
            {
               // Java: first = new Entry(e.hash, e.key, e.value, first);
      todo!();
            }
            // Java: ttab[i] = first;
      todo!();
         }
         // Java: return t;
      todo!();
      }
      // Java: catch (CloneNotSupportedException var7)
      todo!();
      {
         // Java: throw new InternalError();
      todo!();
      }
    }

    // Java: Set keySet()
    pub fn keySet(&mut self) -> Vec<Box<dyn std::any::Any>> {
      // Java: Set ks = this.keySet;
      todo!();
      // Java: return ks != null ? ks : (this.keySet = new KeySet());
      todo!();
    }

    // Java: Collection values()
    pub fn values(&mut self) -> Vec<Box<dyn std::any::Any>> {
      // Java: Collection vs = this.values;
      todo!();
      // Java: return vs != null ? vs : (this.values = new Values());
      todo!();
    }

    // Java: Set entrySet()
    pub fn entrySet(&mut self) -> Vec<Box<dyn std::any::Any>> {
      // Java: Set es = this.entrySet;
      todo!();
      // Java: return es != null ? es : (this.entrySet = new EntrySet());
      todo!();
    }

    // Java: boolean findAndRemoveEntry(Map.Entry entry)
    pub fn findAndRemoveEntry(&mut self) -> bool {
      // Java: Object key = entry.getKey();
      todo!();
      // Java: Object v = this.get(key);
      todo!();
      // Java: if (v != null && v.equals(entry.getValue()))
      todo!();
      {
         // Java: this.remove(key);
      todo!();
         // Java: return true;
      todo!();
      }
      // Java: else
      todo!();
      {
         // Java: return false;
      todo!();
      }
    }

    // Java: Enumeration keys()
    pub fn keys(&mut self) -> Box<dyn Iterator<Item = Box<dyn std::any::Any>>> {
      // Java: return new KeyIterator();
      todo!();
    }

    // Java: Enumeration elements()
    pub fn elements(&mut self) -> Box<dyn Iterator<Item = Box<dyn std::any::Any>>> {
      // Java: return new ValueIterator();
      todo!();
    }

    // Java: void writeObject(ObjectOutputStream s)
    pub fn writeObject(&mut self) {
      // Java: s.defaultWriteObject();
      todo!();
      // Java: s.writeInt(this.table.length);
      todo!();
      // Java: s.writeInt(this.count);
      todo!();
      // Java: for(int index = this.table.length - 1; index >= 0; --index)
      todo!();
      {
         // Java: for(Entry entry = this.table[index]; entry != null; entry = entry.next)
      todo!();
         {
            // Java: s.writeObject(entry.key);
      todo!();
            // Java: s.writeObject(entry.value);
      todo!();
         }
      }
    }

    // Java: void readObject(ObjectInputStream s)
    pub fn readObject(&mut self) {
      // Java: s.defaultReadObject();
      todo!();
      // Java: int numBuckets = s.readInt();
      todo!();
      // Java: this.table = new Entry[numBuckets];
      todo!();
      // Java: int size = s.readInt();
      todo!();
      // Java: for(int i = 0; i < size; ++i)
      todo!();
      {
         // Java: Object key = s.readObject();
      todo!();
         // Java: Object value = s.readObject();
      todo!();
         // Java: this.put(key, value);
      todo!();
      }
    }

    // Java: int capacity()
    pub fn capacity(&mut self) -> i32 {
      // Java: return this.table.length;
      todo!();
    }

    // Java: float loadFactor()
    pub fn loadFactor(&mut self) -> f32 {
      // Java: return this.loadFactor;
      todo!();
    }

}

// Java: public class BarrierLock
impl BarrierLock {
    // Java: BarrierLock()
    pub fn BarrierLock_ctor(&mut self) {
         // Java: super();
      todo!();
    }

}

// Java: public class KeySet
impl KeySet {
    // Java: KeySet()
    pub fn KeySet_ctor(&mut self) {
         // Java: super();
      todo!();
    }

    // Java: Iterator iterator()
    pub fn iterator(&mut self) -> Box<dyn Iterator<Item = Box<dyn std::any::Any>>> {
         // Java: return ConcurrentReaderHashMap.this.new KeyIterator();
      todo!();
    }

    // Java: int size()
    pub fn size_2(&mut self) -> i32 {
         // Java: return ConcurrentReaderHashMap.this.size();
      todo!();
    }

    // Java: boolean contains(Object o)
    pub fn contains_2(&mut self) -> bool {
         // Java: return ConcurrentReaderHashMap.this.containsKey(o);
      todo!();
    }

    // Java: boolean remove(Object o)
    pub fn remove_2(&mut self) -> bool {
         // Java: return ConcurrentReaderHashMap.this.remove(o) != null;
      todo!();
    }

    // Java: void clear()
    pub fn clear_2(&mut self) {
         // Java: ConcurrentReaderHashMap.this.clear();
      todo!();
    }

}

// Java: public class Values
impl Values {
    // Java: Values()
    pub fn Values_ctor(&mut self) {
         // Java: super();
      todo!();
    }

    // Java: Iterator iterator()
    pub fn iterator_2(&mut self) -> Box<dyn Iterator<Item = Box<dyn std::any::Any>>> {
         // Java: return ConcurrentReaderHashMap.this.new ValueIterator();
      todo!();
    }

    // Java: int size()
    pub fn size_3(&mut self) -> i32 {
         // Java: return ConcurrentReaderHashMap.this.size();
      todo!();
    }

    // Java: boolean contains(Object o)
    pub fn contains_3(&mut self) -> bool {
         // Java: return ConcurrentReaderHashMap.this.containsValue(o);
      todo!();
    }

    // Java: void clear()
    pub fn clear_3(&mut self) {
         // Java: ConcurrentReaderHashMap.this.clear();
      todo!();
    }

}

// Java: public class EntrySet
impl EntrySet {
    // Java: EntrySet()
    pub fn EntrySet_ctor(&mut self) {
         // Java: super();
      todo!();
    }

    // Java: Iterator iterator()
    pub fn iterator_3(&mut self) -> Box<dyn Iterator<Item = Box<dyn std::any::Any>>> {
         // Java: return ConcurrentReaderHashMap.this.new HashIterator();
      todo!();
    }

    // Java: boolean contains(Object o)
    pub fn contains_4(&mut self) -> bool {
         // Java: if (!(o instanceof Map.Entry))
      todo!();
         {
            // Java: return false;
      todo!();
         }
         // Java: else
      todo!();
         {
            // Java: Map.Entry entry = (Map.Entry)o;
      todo!();
            // Java: Object v = ConcurrentReaderHashMap.this.get(entry.getKey());
      todo!();
            // Java: return v != null && v.equals(entry.getValue());
      todo!();
         }
    }

    // Java: boolean remove(Object o)
    pub fn remove_3(&mut self) -> bool {
         // Java: return !(o instanceof Map.Entry) ? false : ConcurrentReaderHashMap.this.findAndRemoveEntry((Map.Entry)o);
      todo!();
    }

    // Java: int size()
    pub fn size_4(&mut self) -> i32 {
         // Java: return ConcurrentReaderHashMap.this.size();
      todo!();
    }

    // Java: void clear()
    pub fn clear_4(&mut self) {
         // Java: ConcurrentReaderHashMap.this.clear();
      todo!();
    }

}

// Java: public class Entry
impl HashEntry {
    // Java: Object getKey()
    pub fn getKey(&mut self) -> Option<Box<dyn std::any::Any>> {
         // Java: return this.key;
      todo!();
    }

    // Java: Object getValue()
    pub fn getValue(&mut self) -> Option<Box<dyn std::any::Any>> {
         // Java: return this.value;
      todo!();
    }

    // Java: Object setValue(Object value)
    pub fn setValue(&mut self) -> Option<Box<dyn std::any::Any>> {
         // Java: if (value == null)
      todo!();
         {
            // Java: throw new NullPointerException();
      todo!();
         }
         // Java: else
      todo!();
         {
            // Java: Object oldValue = this.value;
      todo!();
            // Java: this.value = value;
      todo!();
            // Java: return oldValue;
      todo!();
         }
    }

    // Java: boolean equals(Object o)
    pub fn equals(&mut self) -> bool {
         // Java: if (!(o instanceof Map.Entry))
      todo!();
         {
            // Java: return false;
      todo!();
         }
         // Java: else
      todo!();
         {
            // Java: Map.Entry e = (Map.Entry)o;
      todo!();
            // Java: return this.key.equals(e.getKey()) && this.value.equals(e.getValue());
      todo!();
         }
    }

    // Java: int hashCode()
    pub fn hashCode(&mut self) -> i32 {
         // Java: return this.key.hashCode() ^ this.value.hashCode();
      todo!();
    }

    // Java: String toString()
    pub fn toString(&mut self) -> String {
         // Java: return this.key + "=" + this.value;
      todo!();
    }

}

// Java: public class HashIterator
impl HashIterator {
    // Java: HashIterator()
    pub fn HashIterator_ctor(&mut self) {
         // Java: super();
      todo!();
         // Java: this.index = this.tab.length - 1;
      todo!();
    }

    // Java: boolean hasMoreElements()
    pub fn hasMoreElements(&mut self) -> bool {
         // Java: return this.hasNext();
      todo!();
    }

    // Java: Object nextElement()
    pub fn nextElement(&mut self) -> Option<Box<dyn std::any::Any>> {
         // Java: return this.next();
      todo!();
    }

    // Java: boolean hasNext()
    pub fn hasNext(&mut self) -> bool {
         // Java: do
      todo!();
         {
            // Java: if (this.entry != null)
      todo!();
            {
               // Java: Object v = this.entry.value;
      todo!();
               // Java: if (v != null)
      todo!();
               {
                  // Java: this.currentKey = this.entry.key;
      todo!();
                  // Java: this.currentValue = v;
      todo!();
                  // Java: return true;
      todo!();
               }
               // Java: this.entry = this.entry.next;
      todo!();
            }
            // Java: while(this.entry == null && this.index >= 0)
      todo!();
            {
               // Java: this.entry = this.tab[this.index--];
      todo!();
            }
         }
         // Java: this.currentKey = this.currentValue = null;
      todo!();
         // Java: return false;
      todo!();
    }

    // Java: Object returnValueOfNext()
    pub fn returnValueOfNext(&mut self) -> Option<Box<dyn std::any::Any>> {
         // Java: return this.entry;
      todo!();
    }

    // Java: Object next()
    pub fn next(&mut self) -> Option<Box<dyn std::any::Any>> {
         // Java: if (this.currentKey == null && !this.hasNext())
      todo!();
         {
            // Java: throw new NoSuchElementException();
      todo!();
         }
         // Java: else
      todo!();
         {
            // Java: Object result = this.returnValueOfNext();
      todo!();
            // Java: this.lastReturned = this.entry;
      todo!();
            // Java: this.currentKey = this.currentValue = null;
      todo!();
            // Java: this.entry = this.entry.next;
      todo!();
            // Java: return result;
      todo!();
         }
    }

    // Java: void remove()
    pub fn remove_4(&mut self) {
         // Java: if (this.lastReturned == null)
      todo!();
         {
            // Java: throw new IllegalStateException();
      todo!();
         }
         // Java: else
      todo!();
         {
            // Java: ConcurrentReaderHashMap.this.remove(this.lastReturned.key);
      todo!();
            // Java: this.lastReturned = null;
      todo!();
         }
    }

}

// Java: public class KeyIterator
impl KeyIterator {
    // Java: KeyIterator()
    pub fn KeyIterator_ctor(&mut self) {
         // Java: super();
      todo!();
    }

    // Java: Object returnValueOfNext()
    pub fn returnValueOfNext_2(&mut self) -> Option<Box<dyn std::any::Any>> {
         // Java: return this.currentKey;
      todo!();
    }

}

// Java: public class ValueIterator
impl ValueIterator {
    // Java: ValueIterator()
    pub fn ValueIterator_ctor(&mut self) {
         // Java: super();
      todo!();
    }

    // Java: Object returnValueOfNext()
    pub fn returnValueOfNext_3(&mut self) -> Option<Box<dyn std::any::Any>> {
         // Java: return this.currentValue;
      todo!();
    }

}

// impl Map for ConcurrentReaderHashMap {}

// impl Cloneable for ConcurrentReaderHashMap {}
