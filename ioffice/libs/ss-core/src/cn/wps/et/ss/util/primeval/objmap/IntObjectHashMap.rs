// java package cn.wps.et.ss.util.primeval.objmap;

// java public class IntObjectHashMap<V> implements Cloneable, Serializable, Iterable<Entry<V>>
pub struct IntObjectHashMap<V> {
    // java private Entry<V>[] table;
    table: Vec<Vec<(i32, V)>>,
    // java private int size;
    size: usize,
    // java private transient int threshold;
    threshold: usize,
}

impl<V> IntObjectHashMap<V> {
    // java public IntObjectHashMap() { this(4); }
    pub fn IntObjectHashMap() -> Self { Self::with_capacity(4) }

    // java public IntObjectHashMap(int capacity)
    pub fn with_capacity(capacity: i32) -> Self {
        // java clamp to [4, 1073741824], otherwise round up to a power of two.
        let capacity = capacity.clamp(4, 1 << 30) as usize;
        let capacity = capacity.next_power_of_two();
        let mut table = Vec::with_capacity(capacity);
        table.resize_with(capacity, Vec::new);
        // java threshold = (capacity >> 1) + (capacity >> 2).
        Self { table, size: 0, threshold: (capacity >> 1) + (capacity >> 2) }
    }

    // java private static int hash(int v)
    fn hash(v: i32) -> u32 {
        // java int h = v ^ v >>> 20 ^ v >>> 12; return h ^ h >>> 7 ^ h >>> 4;
        let v = v as u32;
        let h = v ^ (v >> 20) ^ (v >> 12);
        h ^ (h >> 7) ^ (h >> 4)
    }

    // java public V put(int key, V value)
    pub fn put(&mut self, key: i32, value: V) -> Option<V> {
        // java find existing key in hash bucket; replace value without changing chain order.
        let mut index = (Self::hash(key) as usize) & (self.table.len() - 1);
        for (existing_key, existing_value) in &mut self.table[index] {
            if *existing_key == key { return Some(std::mem::replace(existing_value, value)); }
        }
        // java if (++size > threshold) doubleCapacity() before new Entry insertion.
        self.size += 1;
        if self.size > self.threshold { self.doubleCapacity(); index = (Self::hash(key) as usize) & (self.table.len() - 1); }
        // java t[index] = new Entry(key,value,t[index]) -- prepend to chain.
        self.table[index].insert(0, (key, value));
        None
    }

    // java public V get(int key)
    pub fn get(&self, key: i32) -> Option<&V> {
        let index = (Self::hash(key) as usize) & (self.table.len() - 1);
        self.table[index].iter().find(|(k, _)| *k == key).map(|(_, v)| v)
    }

    // rust: mutable adapter for Java's reference returned by get.
    pub fn get_mut(&mut self, key: i32) -> Option<&mut V> {
        let index = (Self::hash(key) as usize) & (self.table.len() - 1);
        self.table[index].iter_mut().find(|(k, _)| *k == key).map(|(_, v)| v)
    }

    // java public V remove(int key)
    pub fn remove(&mut self, key: i32) -> Option<V> {
        let index = (Self::hash(key) as usize) & (self.table.len() - 1);
        let position = self.table[index].iter().position(|(k, _)| *k == key)?;
        self.size -= 1;
        Some(self.table[index].remove(position).1)
    }

    // java public void clear()
    pub fn clear(&mut self) { for bucket in &mut self.table { bucket.clear(); } self.size = 0; }

    // java public boolean containsKey(int key)
    pub fn containsKey(&self, key: i32) -> bool { self.get(key).is_some() }

    // java public int size(); public boolean isEmpty()
    pub fn size(&self) -> usize { self.size }
    pub fn isEmpty(&self) -> bool { self.size == 0 }

    // java public Iterator<Entry<V>> iterator()
    pub fn iterator(&self) -> impl Iterator<Item = (i32, &V)> {
        // java scan table indexes ascending, then linked Entry chains from head to tail.
        self.table.iter().flat_map(|bucket| bucket.iter().map(|(key, value)| (*key, value)))
    }

    // java private void doubleCapacity()
    fn doubleCapacity(&mut self) {
        // java if (table.length == 1073741824) return;
        if self.table.len() == 1 << 30 { return; }
        let new_capacity = self.table.len() * 2;
        let mut new_table = Vec::with_capacity(new_capacity);
        new_table.resize_with(new_capacity, Vec::new);
        self.threshold = (new_capacity >> 1) + (new_capacity >> 2);
        // java iterate old buckets and chains; e.next=newTable[index]; newTable[index]=e.
        for bucket in &mut self.table {
            for entry in bucket.drain(..) {
                let index = (Self::hash(entry.0) as usize) & (new_capacity - 1);
                new_table[index].insert(0, entry);
            }
        }
        self.table = new_table;
    }
}
