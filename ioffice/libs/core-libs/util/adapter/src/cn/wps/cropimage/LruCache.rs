// Java: import java.lang.ref.ReferenceQueue
// Java: import java.lang.ref.WeakReference
// Java: import java.util.HashMap
use std::collections::HashMap;
// Java: import java.util.LinkedHashMap
// Java: import java.util.Map

// Java: public class LruCache
pub struct LruCache {
    // Java field: HashMap<K, V> mLruMap
    mLruMap: HashMap<String, Box<dyn std::any::Any>>,
    // Java field: HashMap<K, Entry<K, V>> mWeakMap
    mWeakMap: HashMap<String, Entry>,
    // Java field: ReferenceQueue<V> mQueue
    mQueue: Option<Box<dyn std::any::Any>>,
}

// Java: public class Entry
pub struct Entry {
    // Java field: K mKey
    pub(crate) mKey: Option<String>,
}

// Java: public class LruCache
impl LruCache {
    // Java: LruCache(final int capacity)
    pub fn new(_capacity: i32) -> Self {
      // Java: super();

      // Java: this.mLruMap = new LinkedHashMap<K, V>(16, 0.75F, true)

      {
         // Java: private static final long serialVersionUID = 7941813401882160021L;

         // Java: protected boolean removeEldestEntry(Map.Entry<K, V> eldest)

         {
            // Java: return this.size() > capacity;

         }
      }
      todo!()
    }

    // Java: void cleanUpWeakMap()
    pub fn cleanUpWeakMap(&mut self) {
      // Java: for(Entry<K, V> entry = (Entry)this.mQueue.poll(); entry != null; entry = (Entry)this.mQueue.poll())

      {
         // Java: this.mWeakMap.remove(entry.mKey);

      }
    }

    // Java: V put(K key, V value)
    pub fn put(&mut self, _key: String, _value: Box<dyn std::any::Any>) -> Option<Box<dyn std::any::Any>> {
      // Java: this.cleanUpWeakMap();

      // Java: this.mLruMap.put(key, value);

      // Java: Entry<K, V> entry = (Entry)this.mWeakMap.put(key, new Entry(key, value, this.mQueue));

      // Java: return (V)(entry == null ? null : entry.get());

      todo!()
    }

    // Java: V get(K key)
    pub fn get(&mut self, _key: &str) -> Option<&Box<dyn std::any::Any>> {
      // Java: this.cleanUpWeakMap();

      // Java: V value = (V)this.mLruMap.get(key);

      // Java: if (value != null)

      {
         // Java: return value;

      }
      // Java: else

      {
         // Java: Entry<K, V> entry = (Entry)this.mWeakMap.get(key);

         // Java: return (V)(entry == null ? null : entry.get());

      }
      todo!()
    }

    // Java: void clear()
    pub fn clear(&mut self) {
      // Java: this.mLruMap.clear();

      // Java: this.mWeakMap.clear();

      // Java: this.mQueue = new ReferenceQueue();

    }

}

// Java: public class Entry
impl Entry {
    // Java: Entry(K key, V value, ReferenceQueue<V> queue)
    pub fn new(_key: String, _value: Box<dyn std::any::Any>, _queue: Option<Box<dyn std::any::Any>>) -> Self {
         // Java: super(value, queue);

         // Java: this.mKey = key;

         Entry { mKey: Some(_key) }
    }

}
