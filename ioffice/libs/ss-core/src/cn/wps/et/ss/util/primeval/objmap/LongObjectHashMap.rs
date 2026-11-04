// java package cn.wps.et.ss.util.primeval.objmap;

// java public class LongObjectHashMap<V> implements Cloneable, Serializable, Iterable<Entry<V>>
pub struct LongObjectHashMap<V> {
    // java private Entry<V>[] table;
    table: Vec<Vec<(i64, V)>>,
    // java private int size;
    size: usize,
    // java private transient int threshold;
    threshold: usize,
}

impl<V> LongObjectHashMap<V> {
    // java public LongObjectHashMap() { this(4); }
    pub fn LongObjectHashMap() -> Self { Self::with_capacity(4) }

    // java public LongObjectHashMap(int capacity)
    pub fn with_capacity(capacity: i32) -> Self {
        // java clamp [4, 1073741824], then roundUpToPowerOfTwo.
        let capacity = (capacity.clamp(4, 1 << 30) as usize).next_power_of_two();
        let mut table = Vec::with_capacity(capacity);
        table.resize_with(capacity, Vec::new);
        // java threshold = (capacity >> 1) + (capacity >> 2);
        Self { table, size: 0, threshold: (capacity >> 1) + (capacity >> 2) }
    }

    // java private static int hash(long v)
    fn hash(v: i64) -> u32 {
        // java int h = (int)(v ^ v >>> 32); h ^= h >>> 20 ^ h >>> 12;
        let v = v as u64;
        let h = (v ^ (v >> 32)) as u32;
        let h = h ^ (h >> 20) ^ (h >> 12);
        // java return h ^ h >>> 7 ^ h >>> 4;
        h ^ (h >> 7) ^ (h >> 4)
    }

    // java public synchronized V put(long key, V value)
    pub fn put(&mut self, key: i64, value: V) -> Option<V> {
        let mut index = Self::hash(key) as usize & (self.table.len() - 1);
        // java replace an existing Entry in place.
        for (entry_key, entry_value) in &mut self.table[index] {
            if *entry_key == key { return Some(std::mem::replace(entry_value, value)); }
        }
        // java if (++size > threshold) doubleCapacity();
        self.size += 1;
        if self.size > self.threshold {
            self.doubleCapacity();
            index = Self::hash(key) as usize & (self.table.len() - 1);
        }
        // java t[index] = new Entry(key, value, t[index]);
        self.table[index].insert(0, (key, value));
        None
    }

    // java public synchronized V get(long key)
    pub fn get(&self, key: i64) -> Option<&V> {
        self.table[Self::hash(key) as usize & (self.table.len() - 1)]
            .iter().find(|(entry_key, _)| *entry_key == key).map(|(_, value)| value)
    }

    // rust: mutable adapter for Java's mutable Entry.value reference.
    pub fn get_mut(&mut self, key: i64) -> Option<&mut V> {
        let index = Self::hash(key) as usize & (self.table.len() - 1);
        self.table[index].iter_mut().find(|(entry_key, _)| *entry_key == key).map(|(_, value)| value)
    }

    // java public V remove(long key)
    pub fn remove(&mut self, key: i64) -> Option<V> {
        let index = Self::hash(key) as usize & (self.table.len() - 1);
        let position = self.table[index].iter().position(|(entry_key, _)| *entry_key == key)?;
        self.size -= 1;
        Some(self.table[index].remove(position).1)
    }

    // java public synchronized void clear() { if (size != 0) { Arrays.fill(table, null); size = 0; } }
    pub fn clear(&mut self) {if self.size!=0 {for bucket in &mut self.table {bucket.clear();}self.size=0;}}

    // java public Iterator<Entry<V>> iterator()
    pub fn iterator(&self) -> impl Iterator<Item = (i64, &V)> {
        // java scan buckets ascending and each bucket's head-first chain.
        self.table.iter().flat_map(|bucket| bucket.iter().map(|(key, value)| (*key, value)))
    }

    // java private void doubleCapacity()
    fn doubleCapacity(&mut self) {
        // java if (table.length == 1073741824) return;
        if self.table.len() == 1 << 30 { return; }
        let capacity = self.table.len() * 2;
        let mut next = Vec::with_capacity(capacity);
        next.resize_with(capacity, Vec::new);
        self.threshold = (capacity >> 1) + (capacity >> 2);
        // java each old chain entry is prepended to its new bucket.
        for bucket in &mut self.table {
            for entry in bucket.drain(..) {
                let index = Self::hash(entry.0) as usize & (capacity - 1);
                next[index].insert(0, entry);
            }
        }
        self.table = next;
    }
}
