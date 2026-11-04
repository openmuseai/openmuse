use std::{
    collections::{HashMap, VecDeque},
    sync::Mutex,
};
// java public class MRUIntKeyCache<V>
// rust: Owned key links replace Java Node references. Head/tail order, capacity changes, replacement/eviction return values and synchronized access follow the original list.
pub struct MRUIntKeyCache<V: Clone> {
    state: Mutex<State<V>>,
}
struct State<V> {
    map: HashMap<i32, V>,
    list: VecDeque<i32>,
    capacity: i32,
}
impl<V: Clone> MRUIntKeyCache<V> {
    // java public MRUIntKeyCache(int cap) {
    // java if (cap < 1) {
    // java throw new IllegalArgumentException();
    // java } else {
    // java this.capacity = cap;
    // java this.map = new IntObjectHashMap<Node<V>>();
    // java this.list = new Node<V>(0, null);
    // java this.list.pre = this.list;
    // java this.list.next = this.list;
    // java }
    // java }
    pub fn new(cap: i32) -> Self {
        assert!(cap >= 1, "IllegalArgumentException");
        Self {
            state: Mutex::new(State {
                map: HashMap::new(),
                list: VecDeque::new(),
                capacity: cap,
            }),
        }
    }
    // java public synchronized V get(int k) {
    // java Node<V> n = this.map.get(k);
    // java if (n == null) {
    // java return null;
    // java } else {
    // java this.removeNode(n);
    // java this.pushHead(n);
    // java return n.value;
    // java }
    // java }
    pub fn get(&self, k: i32) -> Option<V> {
        let mut s = self.state.lock().unwrap();
        let value = s.map.get(&k).cloned()?;
        let n = s.list.iter().position(|x| *x == k).unwrap();
        s.list.remove(n);
        s.list.push_front(k);
        Some(value)
    }
    // java public synchronized V put(int k, V v) {
    // java if (v == null) {
    // java return (V)this.remove(k);
    // java } else {
    // java Node<V> n = this.map.get(k);
    // java if (n != null) {
    // java V ret = n.value;
    // java n.value = v;
    // java this.removeNode(n);
    // java this.pushHead(n);
    // java return ret;
    // java } else {
    // java V ret = null;
    // java
    // java for(int i = this.map.size(); i >= this.capacity; --i) {
    // java n = this.map.remove(this.list.pre.key);
    // java if (n == null) {
    // java break;
    // java }
    // java
    // java this.removeNode(n);
    // java ret = n.value;
    // java }
    // java
    // java if (n != null) {
    // java n.key = k;
    // java n.value = v;
    // java } else {
    // java n = new Node<V>(k, v);
    // java }
    // java
    // java this.map.put(k, n);
    // java this.pushHead(n);
    // java return ret;
    // java }
    // java }
    // java }
    pub fn put(&self, k: i32, v: Option<V>) -> Option<V> {
        let Some(v) = v else {
            return self.remove(k);
        };
        let mut s = self.state.lock().unwrap();
        if let Some(old) = s.map.insert(k, v.clone()) {
            let n = s.list.iter().position(|x| *x == k).unwrap();
            s.list.remove(n);
            s.list.push_front(k);
            return Some(old);
        }
        let mut ret = None;
        while s.map.len() as i32 > s.capacity {
            let Some(tail) = s.list.pop_back() else {
                break;
            };
            ret = s.map.remove(&tail);
        }
        s.list.push_front(k);
        ret
    }
    // java public synchronized V remove(int k) {
    // java Node<V> n = this.map.remove(k);
    // java if (n == null) {
    // java return null;
    // java } else {
    // java this.removeNode(n);
    // java return n.value;
    // java }
    // java }
    pub fn remove(&self, k: i32) -> Option<V> {
        let mut s = self.state.lock().unwrap();
        let old = s.map.remove(&k)?;
        let n = s.list.iter().position(|x| *x == k).unwrap();
        s.list.remove(n);
        Some(old)
    }
    // java public synchronized void clear() {
    // java this.map.clear();
    // java this.list.pre = this.list;
    // java this.list.next = this.list;
    // java }
    pub fn clear(&self) {
        let mut s = self.state.lock().unwrap();
        s.map.clear();
        s.list.clear();
    }
    // java public int size() {
    // java return this.map.size();
    // java }
    pub fn size(&self) -> i32 {
        self.state.lock().unwrap().map.len() as i32
    }
    // java public int getCapacity() {
    // java return this.capacity;
    // java }
    pub fn getCapacity(&self) -> i32 {
        self.state.lock().unwrap().capacity
    }
    // java public void setCapacity(int cap) {
    // java if (cap < 1) {
    // java throw new IllegalArgumentException();
    // java } else {
    // java this.capacity = cap;
    // java }
    // java }
    pub fn setCapacity(&self, cap: i32) {
        assert!(cap >= 1, "IllegalArgumentException");
        self.state.lock().unwrap().capacity = cap;
    }
    // java public boolean containsKey(int key) {
    // java return this.map.containsKey(key);
    // java }
    pub fn containsKey(&self, key: i32) -> bool {
        self.state.lock().unwrap().map.contains_key(&key)
    }
}
