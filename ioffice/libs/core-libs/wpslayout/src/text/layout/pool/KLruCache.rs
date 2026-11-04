// Java: import java.util.Iterator
// TODO_RUST_CODE use ???; // java.util.Iterator
// Java: import java.util.LinkedHashMap
// TODO_RUST_CODE use ???; // java.util.LinkedHashMap
// Java: import java.util.Map
// TODO_RUST_CODE use ???; // java.util.Map

// Java: public class KLruCache
pub struct KLruCache {
    // Java field: LinkedHashMap<K, V> map
    map: LinkedHashMap<K, V>,
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
impl KLruCache {
    // Java: KLruCache(int maxSize)
    pub fn KLruCache(&self, int maxSize) {
      // Java: super();
      // TODO_RUST_CODE
      // Java: if (maxSize <= 0)
        // TODO_RUST_CODE
      {
         // Java: throw new IllegalArgumentException("maxSize <= 0");
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: this.maxSize = maxSize;
         // TODO_RUST_CODE
         // Java: this.map = new LinkedHashMap(0, 0.75F, true);
         // TODO_RUST_CODE
      }
    }

    // Java: void resize(int maxSize)
    pub fn resize(&self, int maxSize) {
      // Java: if (maxSize <= 0)
        // TODO_RUST_CODE
      {
         // Java: throw new IllegalArgumentException("maxSize <= 0");
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: synchronized(this)
            // TODO_RUST_CODE
         {
            // Java: this.maxSize = maxSize;
            // TODO_RUST_CODE
         }
         // Java: this.trimToSize(maxSize);
         // TODO_RUST_CODE
      }
    }

    // Java: V get(K key)
    pub fn get(&self, K key) -> V {
      // Java: if (key == null)
        // TODO_RUST_CODE
      {
         // Java: throw new NullPointerException("key == null");
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: synchronized(this)
            // TODO_RUST_CODE
         {
            // Java: V mapValue = (V)this.map.get(key);
            // TODO_RUST_CODE
            // Java: if (mapValue != null)
                // TODO_RUST_CODE
            {
               // Java: ++this.hitCount;
               // TODO_RUST_CODE
               // Java: return mapValue;
               // TODO_RUST_CODE
            }
            // Java: ++this.missCount;
            // TODO_RUST_CODE
         }
         // Java: V createdValue = (V)this.create(key);
         // TODO_RUST_CODE
         // Java: if (createdValue == null)
            // TODO_RUST_CODE
         {
            // Java: return null;
            // TODO_RUST_CODE
         }
         // Java: else
            // TODO_RUST_CODE
         {
            // Java: V var8;
            // TODO_RUST_CODE
            // Java: synchronized(this)
                // TODO_RUST_CODE
            {
               // Java: ++this.createCount;
               // TODO_RUST_CODE
               // Java: var8 = this.map.put(key, createdValue);
               // TODO_RUST_CODE
               // Java: if (var8 != null)
                    // TODO_RUST_CODE
               {
                  // Java: this.map.put(key, var8);
                  // TODO_RUST_CODE
               }
               // Java: else
                    // TODO_RUST_CODE
               {
                  // Java: this.size += this.safeSizeOf(key, createdValue);
                  // TODO_RUST_CODE
               }
            }
            // Java: if (var8 != null)
                // TODO_RUST_CODE
            {
               // Java: this.entryRemoved(false, key, createdValue, var8);
               // TODO_RUST_CODE
               // Java: return (V)var8;
               // TODO_RUST_CODE
            }
            // Java: else
                // TODO_RUST_CODE
            {
               // Java: this.trimToSize(this.maxSize);
               // TODO_RUST_CODE
               // Java: return createdValue;
               // TODO_RUST_CODE
            }
         }
      }
    }

    // Java: V put(K key, V value)
    pub fn put(&self, K key, V value) -> V {
      // Java: if (key != null && value != null)
        // TODO_RUST_CODE
      {
         // Java: V previous;
         // TODO_RUST_CODE
         // Java: synchronized(this)
            // TODO_RUST_CODE
         {
            // Java: ++this.putCount;
            // TODO_RUST_CODE
            // Java: this.size += this.safeSizeOf(key, value);
            // TODO_RUST_CODE
            // Java: previous = (V)this.map.put(key, value);
            // TODO_RUST_CODE
            // Java: if (previous != null)
                // TODO_RUST_CODE
            {
               // Java: this.size -= this.safeSizeOf(key, previous);
               // TODO_RUST_CODE
            }
         }
         // Java: if (previous != null)
            // TODO_RUST_CODE
         {
            // Java: this.entryRemoved(false, key, previous, value);
            // TODO_RUST_CODE
         }
         // Java: this.trimToSize(this.maxSize);
         // TODO_RUST_CODE
         // Java: return previous;
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: throw new NullPointerException("key == null || value == null");
         // TODO_RUST_CODE
      }
    }

    // Java: void trimToSize(int maxSize)
    pub fn trimToSize(&self, int maxSize) {
      // Java: while(true)
        // TODO_RUST_CODE
      {
         // Java: K key;
         // TODO_RUST_CODE
         // Java: V value;
         // TODO_RUST_CODE
         // Java: synchronized(this)
            // TODO_RUST_CODE
         {
            // Java: label53:
                // TODO_RUST_CODE
            {
               // Java: if (this.size < 0 || this.map.isEmpty() && this.size != 0)
                    // TODO_RUST_CODE
               {
                  // Java: throw new IllegalStateException(this.getClass().getName() + ".sizeOf() is reporting inconsistent results!");
                  // TODO_RUST_CODE
               }
               // Java: if (this.size > maxSize)
                    // TODO_RUST_CODE
               {
                  // Java: Map.Entry<K, V> toEvict = null;
                  // TODO_RUST_CODE
                  // Java: Iterator var6 = this.map.entrySet().iterator();
                  // TODO_RUST_CODE
                  // Java: if (var6.hasNext())
                        // TODO_RUST_CODE
                  {
                     // Java: Map.Entry<K, V> entry = (Map.Entry)var6.next();
                     // TODO_RUST_CODE
                     // Java: toEvict = entry;
                     // TODO_RUST_CODE
                  }
                  // Java: if (toEvict != null)
                        // TODO_RUST_CODE
                  {
                     // Java: key = (K)toEvict.getKey();
                     // TODO_RUST_CODE
                     // Java: value = (V)toEvict.getValue();
                     // TODO_RUST_CODE
                     // Java: this.map.remove(key);
                     // TODO_RUST_CODE
                     // Java: this.size -= this.safeSizeOf(key, value);
                     // TODO_RUST_CODE
                     // Java: ++this.evictionCount;
                     // TODO_RUST_CODE
                     // Java: break label53;
                     // TODO_RUST_CODE
                  }
               }
               // Java: return;
               // TODO_RUST_CODE
            }
         }
         // Java: this.entryRemoved(true, key, value, null);
         // TODO_RUST_CODE
      }
    }

    // Java: V remove(K key)
    pub fn remove(&self, K key) -> V {
      // Java: if (key == null)
        // TODO_RUST_CODE
      {
         // Java: throw new NullPointerException("key == null");
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: V previous;
         // TODO_RUST_CODE
         // Java: synchronized(this)
            // TODO_RUST_CODE
         {
            // Java: previous = (V)this.map.remove(key);
            // TODO_RUST_CODE
            // Java: if (previous != null)
                // TODO_RUST_CODE
            {
               // Java: this.size -= this.safeSizeOf(key, previous);
               // TODO_RUST_CODE
            }
         }
         // Java: if (previous != null)
            // TODO_RUST_CODE
         {
            // Java: this.entryRemoved(false, key, previous, null);
            // TODO_RUST_CODE
         }
         // Java: return previous;
         // TODO_RUST_CODE
      }
    }

    // Java: void entryRemoved(boolean evicted, K key, V oldValue, V newValue)
    pub fn entryRemoved(&self, boolean evicted, K key, V oldValue, V newValue) {
    // TODO_RUST_CODE
    }

    // Java: V create(K key)
    pub fn create(&self, K key) -> V {
      // Java: return null;
      // TODO_RUST_CODE
    }

    // Java: int safeSizeOf(K key, V value)
    pub fn safeSizeOf(&self, K key, V value) -> i32 {
      // Java: int result = this.sizeOf(key, value);
      // TODO_RUST_CODE
      // Java: if (result < 0)
        // TODO_RUST_CODE
      {
         // Java: throw new IllegalStateException("Negative size: " + key + "=" + value);
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: return result;
         // TODO_RUST_CODE
      }
    }

    // Java: int sizeOf(K key, V value)
    pub fn sizeOf(&self, K key, V value) -> i32 {
      // Java: return 1;
      // TODO_RUST_CODE
    }

    // Java: void valueToArray(V[] values)
    pub fn valueToArray(&self, V[] values) {
      // Java: synchronized(this)
        // TODO_RUST_CODE
      {
         // Java: this.map.values().toArray(values);
         // TODO_RUST_CODE
      }
    }

    // Java: void evictAll()
    pub fn evictAll(&self) {
      // Java: this.trimToSize(-1);
      // TODO_RUST_CODE
    }

    // Java: int size()
    pub fn size(&self) -> i32 {
      // Java: return this.size;
      // TODO_RUST_CODE
    }

    // Java: int maxSize()
    pub fn maxSize(&self) -> i32 {
      // Java: return this.maxSize;
      // TODO_RUST_CODE
    }

    // Java: int hitCount()
    pub fn hitCount(&self) -> i32 {
      // Java: return this.hitCount;
      // TODO_RUST_CODE
    }

    // Java: int missCount()
    pub fn missCount(&self) -> i32 {
      // Java: return this.missCount;
      // TODO_RUST_CODE
    }

    // Java: int createCount()
    pub fn createCount(&self) -> i32 {
      // Java: return this.createCount;
      // TODO_RUST_CODE
    }

    // Java: int putCount()
    pub fn putCount(&self) -> i32 {
      // Java: return this.putCount;
      // TODO_RUST_CODE
    }

    // Java: int evictionCount()
    pub fn evictionCount(&self) -> i32 {
      // Java: return this.evictionCount;
      // TODO_RUST_CODE
    }

    // Java: public final synchronized Map<K, V> snapshot()
    pub fn snapshot(&self) -> public final synchronized Map<K, V> {
      // Java: return new LinkedHashMap(this.map);
      // TODO_RUST_CODE
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
      // Java: int accesses = this.hitCount + this.missCount;
      // TODO_RUST_CODE
      // Java: int hitPercent = accesses != 0 ? 100 * this.hitCount / accesses : 0;
      // TODO_RUST_CODE
      // Java: return String.format("LruCache[maxSize=%d,hits=%d,misses=%d,hitRate=%d%%]", this.maxSize, this.hitCount, this.missCount, hitPercent);
      // TODO_RUST_CODE
    }

}
