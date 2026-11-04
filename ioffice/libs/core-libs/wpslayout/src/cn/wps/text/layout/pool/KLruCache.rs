// Java: import java.util.Iterator
// Java: import java.util.LinkedHashMap
use std::collections::HashMap;
// Java: import java.util.Map
use std::hash::Hash;

// Java: public class KLruCache
pub struct KLruCache<K: Eq + Hash + Clone, V: Clone> {
    // Java field: LinkedHashMap<K, V> map
    map: Vec<(K, V)>,
    // Java field: int size
    pub(crate) size: i32,
    // Java field: int maxSize
    pub(crate) maxSize: i32,
    // Java field: int putCount
    pub(crate) putCount: i32,
    // Java field: int createCount
    pub(crate) createCount: i32,
    // Java field: int evictionCount
    pub(crate) evictionCount: i32,
    // Java field: int hitCount
    pub(crate) hitCount: i32,
    // Java field: int missCount
    pub(crate) missCount: i32,
}

// Java: public class KLruCache
impl<K: Eq + Hash + Clone, V: Clone> KLruCache<K, V> {
    // Java: KLruCache(int maxSize)
    pub fn new(maxSize: i32) -> Self {
        // Java: super();
        // Java: if (maxSize <= 0)
        if maxSize <= 0 {
            // Java: throw new IllegalArgumentException("maxSize <= 0");
            panic!("maxSize <= 0");
        }
        // Java: else
        {
            // Java: this.maxSize = maxSize;
            // Java: this.map = new LinkedHashMap(0, 0.75F, true);
            KLruCache {
                map: Vec::new(),
                size: 0,
                maxSize,
                putCount: 0,
                createCount: 0,
                evictionCount: 0,
                hitCount: 0,
                missCount: 0,
            }
        }
    }

    // Java: void resize(int maxSize)
    pub fn resize(&mut self, maxSize: i32) {
        // Java: if (maxSize <= 0)
        if maxSize <= 0 {
            // Java: throw new IllegalArgumentException("maxSize <= 0");
            panic!("maxSize <= 0");
        }
        // Java: else
        {
            // Java: synchronized(this)
            {
                // Java: this.maxSize = maxSize;
                self.maxSize = maxSize;
            }
            // Java: this.trimToSize(maxSize);
            self.trimToSize(maxSize);
        }
    }

    // Java: V get(K key)
    pub fn get(&mut self, key: &K) -> Option<V> {
        // Java: if (key == null)
        {
            // Java: throw new NullPointerException("key == null");
        }
        // Java: else
        {
            // Java: synchronized(this)
            {
                // Java: V mapValue = (V)this.map.get(key);
                let pos = self.map.iter().position(|(k, _)| k == key);
                // Java: if (mapValue != null)
                if let Some(idx) = pos {
                    // Java: ++this.hitCount;
                    self.hitCount += 1;
                    // Java: return mapValue;
                    let val = self.map[idx].1.clone();
                    let entry = self.map.remove(idx);
                    self.map.push(entry);
                    return Some(val);
                }
                // Java: ++this.missCount;
                self.missCount += 1;
            }
            // Java: V createdValue = (V)this.create(key);
            let createdValue = self.create(key);
            // Java: if (createdValue == null)
            if createdValue.is_none() {
                // Java: return null;
                return None;
            }
            // Java: else
            {
                let cv = createdValue.unwrap();
                // Java: V var8;
                // Java: synchronized(this)
                {
                    // Java: ++this.createCount;
                    self.createCount += 1;
                    // Java: var8 = this.map.put(key, createdValue);
                    let var8 = self.map_put(key.clone(), cv.clone());
                    // Java: if (var8 != null)
                    if let Some(ref old) = var8 {
                        // Java: this.map.put(key, var8);
                        self.map_put(key.clone(), old.clone());
                    }
                    // Java: else
                    else {
                        // Java: this.size += this.safeSizeOf(key, createdValue);
                        self.size += self.safeSizeOf(key, &cv);
                    }

                    // Java: if (var8 != null)
                    if let Some(old) = var8 {
                        // Java: this.entryRemoved(false, key, createdValue, var8);
                        self.entryRemoved(false, key, &cv, Some(&old));
                        // Java: return (V)var8;
                        return Some(old);
                    }
                    // Java: else
                    {
                        // Java: this.trimToSize(this.maxSize);
                        let ms = self.maxSize;
                        self.trimToSize(ms);
                        // Java: return createdValue;
                        return Some(cv);
                    }
                }
            }
        }
    }

    // Java: V put(K key, V value)
    pub fn put(&mut self, key: K, value: V) -> Option<V> {
        // Java: if (key != null && value != null)
        {
            // Java: V previous;
            let previous;
            // Java: synchronized(this)
            {
                // Java: ++this.putCount;
                self.putCount += 1;
                // Java: this.size += this.safeSizeOf(key, value);
                self.size += self.safeSizeOf(&key, &value);
                // Java: previous = (V)this.map.put(key, value);
                previous = self.map_put(key.clone(), value.clone());
                // Java: if (previous != null)
                if let Some(ref prev) = previous {
                    // Java: this.size -= this.safeSizeOf(key, previous);
                    self.size -= self.safeSizeOf(&key, prev);
                }
            }
            // Java: if (previous != null)
            if let Some(ref prev) = previous {
                // Java: this.entryRemoved(false, key, previous, value);
                self.entryRemoved(false, &key, prev, Some(&value));
            }
            // Java: this.trimToSize(this.maxSize);
            let ms = self.maxSize;
            self.trimToSize(ms);
            // Java: return previous;
            return previous;
        }
        // Java: else
        {
            // Java: throw new NullPointerException("key == null || value == null");
            panic!("key == null || value == null");
        }
    }

    // Java: void trimToSize(int maxSize)
    pub fn trimToSize(&mut self, maxSize: i32) {
        // Java: while(true)
        loop {
            // Java: K key;
            let mut key: Option<K> = None;
            // Java: V value;
            let mut value: Option<V> = None;
            // Java: synchronized(this)
            {
                // Java: label53:
                'label53:
                {
                    // Java: if (this.size < 0 || this.map.isEmpty() && this.size != 0)
                    if self.size < 0 || (self.map.is_empty() && self.size != 0) {
                        // Java: throw new IllegalStateException(this.getClass().getName() + ".sizeOf() is reporting inconsistent results!");
                        panic!("sizeOf() is reporting inconsistent results!");
                    }
                    // Java: if (this.size > maxSize)
                    if self.size > maxSize {
                        // Java: Map.Entry<K, V> toEvict = null;
                        let mut toEvict: Option<(K, V)> = None;
                        // Java: Iterator var6 = this.map.entrySet().iterator();
                        // Java: if (var6.hasNext())
                        if !self.map.is_empty() {
                            // Java: Map.Entry<K, V> entry = (Map.Entry)var6.next();
                            let entry = self.map[0].clone();
                            // Java: toEvict = entry;
                            toEvict = Some(entry);
                        }
                        // Java: if (toEvict != null)
                        if let Some((k, v)) = toEvict {
                            // Java: key = (K)toEvict.getKey();
                            key = Some(k.clone());
                            // Java: value = (V)toEvict.getValue();
                            value = Some(v.clone());
                            // Java: this.map.remove(key);
                            self.map.retain(|(mk, _)| mk != &k);
                            // Java: this.size -= this.safeSizeOf(key, value);
                            self.size -= self.safeSizeOf(&k, &v);
                            // Java: ++this.evictionCount;
                            self.evictionCount += 1;
                            // Java: break label53;
                            break 'label53;
                        }
                    }
                    // Java: return;
                    return;
                }
            }
            // Java: this.entryRemoved(true, key, value, null);
            if let (Some(ref k), Some(ref v)) = (&key, &value) {
                self.entryRemoved(true, k, v, None);
            }
        }
    }

    // Java: V remove(K key)
    pub fn remove(&mut self, key: &K) -> Option<V> {
        // Java: if (key == null)
        {
            // Java: throw new NullPointerException("key == null");
        }
        // Java: else
        {
            // Java: V previous;
            let previous;
            // Java: synchronized(this)
            {
                // Java: previous = (V)this.map.remove(key);
                let pos = self.map.iter().position(|(k, _)| k == key);
                previous = pos.map(|idx| self.map.remove(idx).1);
                // Java: if (previous != null)
                if let Some(ref prev) = previous {
                    // Java: this.size -= this.safeSizeOf(key, previous);
                    self.size -= self.safeSizeOf(key, prev);
                }
            }
            // Java: if (previous != null)
            if let Some(ref prev) = previous {
                // Java: this.entryRemoved(false, key, previous, null);
                self.entryRemoved(false, key, prev, None);
            }
            // Java: return previous;
            previous
        }
    }

    // Java: void entryRemoved(boolean evicted, K key, V oldValue, V newValue)
    pub fn entryRemoved(&self, _evicted: bool, _key: &K, _oldValue: &V, _newValue: Option<&V>) {
    }

    // Java: V create(K key)
    pub fn create(&self, _key: &K) -> Option<V> {
        // Java: return null;
        None
    }

    // Java: int safeSizeOf(K key, V value)
    pub fn safeSizeOf(&self, key: &K, value: &V) -> i32 {
        // Java: int result = this.sizeOf(key, value);
        let result = self.sizeOf(key, value);
        // Java: if (result < 0)
        if result < 0 {
            // Java: throw new IllegalStateException("Negative size: " + key + "=" + value);
            panic!("Negative size");
        }
        // Java: else
        {
            // Java: return result;
            result
        }
    }

    // Java: int sizeOf(K key, V value)
    pub fn sizeOf(&self, _key: &K, _value: &V) -> i32 {
        // Java: return 1;
        1
    }

    // Java: void valueToArray(V[] values)
    pub fn valueToArray(&self, values: &mut Vec<V>) {
        // Java: synchronized(this)
        {
            // Java: this.map.values().toArray(values);
            values.clear();
            for (_, v) in &self.map {
                values.push(v.clone());
            }
        }
    }

    // Java: void evictAll()
    pub fn evictAll(&mut self) {
        // Java: this.trimToSize(-1);
        self.trimToSize(-1);
    }

    // Java: int size()
    pub fn size(&self) -> i32 {
        // Java: return this.size;
        self.size
    }

    // Java: int maxSize()
    pub fn maxSize(&self) -> i32 {
        // Java: return this.maxSize;
        self.maxSize
    }

    // Java: int hitCount()
    pub fn hitCount(&self) -> i32 {
        // Java: return this.hitCount;
        self.hitCount
    }

    // Java: int missCount()
    pub fn missCount(&self) -> i32 {
        // Java: return this.missCount;
        self.missCount
    }

    // Java: int createCount()
    pub fn createCount(&self) -> i32 {
        // Java: return this.createCount;
        self.createCount
    }

    // Java: int putCount()
    pub fn putCount(&self) -> i32 {
        // Java: synchronized(this)
        {
            // Java: return this.putCount;
            self.putCount
        }
    }

    // Java: int evictionCount()
    pub fn evictionCount(&self) -> i32 {
        // Java: return this.evictionCount;
        self.evictionCount
    }

    // Java: public final synchronized Map<K, V> snapshot()
    pub fn snapshot(&self) -> HashMap<K, V> {
        // Java: return new LinkedHashMap(this.map);
        let mut m = HashMap::new();
        for (k, v) in &self.map {
            m.insert(k.clone(), v.clone());
        }
        m
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
        // Java: int accesses = this.hitCount + this.missCount;
        let accesses = self.hitCount + self.missCount;
        // Java: int hitPercent = accesses != 0 ? 100 * this.hitCount / accesses : 0;
        let hitPercent = if accesses != 0 { 100 * self.hitCount / accesses } else { 0 };
        // Java: return String.format("LruCache[maxSize=%d,hits=%d,misses=%d,hitRate=%d%%]", this.maxSize, this.hitCount, this.missCount, hitPercent);
        format!("LruCache[maxSize={},hits={},misses={},hitRate={}%]", self.maxSize, self.hitCount, self.missCount, hitPercent)
    }

    fn map_put(&mut self, key: K, value: V) -> Option<V> {
        let pos = self.map.iter().position(|(k, _)| k == &key);
        if let Some(idx) = pos {
            let old = self.map.remove(idx);
            self.map.push((key, value));
            Some(old.1)
        } else {
            self.map.push((key, value));
            None
        }
    }
}
