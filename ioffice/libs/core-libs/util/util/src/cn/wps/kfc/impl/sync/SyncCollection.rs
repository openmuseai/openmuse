#![allow(non_snake_case, dead_code, unused_variables, non_upper_case_globals)]

// Java: import java.io.IOException
// Java: import java.io.ObjectOutputStream
// Java: import java.io.Serializable
// Java: import java.util.Collection
// Java: import java.util.Iterator
use std::sync::Mutex;

// Java field: long serialVersionUID
const SERIAL_VERSION_UID: i64 = 3053995032091335093;

// Java: public class SyncCollection implements Collection, Serializable
pub struct SyncCollection<E> {
    // Java field: Collection<E> collection
    pub(crate) collection: Vec<E>,
    // Java field: Object mutex
    mutex: Mutex<()>,
}

// Java: public class SyncCollection implements Collection, Serializable
impl<E: Clone + PartialEq + std::fmt::Debug> SyncCollection<E> {
    // Java: SyncCollection(Collection<E> c, Object mutex)
    pub fn new(c: Vec<E>) -> Self {
        Self {
            collection: c,
            mutex: Mutex::new(()),
        }
    }

    // Java: int size()
    pub fn size(&self) -> i32 {
        // Java: synchronized(this.mutex)
        let _lock = self.mutex.lock().unwrap();
        {
            // Java: return this.collection.size();
            self.collection.len() as i32
        }
    }

    // Java: boolean isEmpty()
    pub fn isEmpty(&self) -> bool {
        // Java: synchronized(this.mutex)
        let _lock = self.mutex.lock().unwrap();
        {
            // Java: return this.collection.isEmpty();
            self.collection.is_empty()
        }
    }

    // Java: boolean contains(Object o)
    pub fn contains(&self, o: &E) -> bool {
        // Java: synchronized(this.mutex)
        let _lock = self.mutex.lock().unwrap();
        {
            // Java: return this.collection.contains(o);
            self.collection.contains(o)
        }
    }

    // Java: Object[] toArray()
    pub fn toArray(&self) -> Vec<E> {
        // Java: synchronized(this.mutex)
        let _lock = self.mutex.lock().unwrap();
        {
            // Java: return this.collection.toArray();
            self.collection.clone()
        }
    }

    // Java: public <T> T[] toArray(T[] a)
    pub fn toArray_typed(&self, a: &mut Vec<E>) -> Vec<E> {
        // Java: synchronized(this.mutex)
        let _lock = self.mutex.lock().unwrap();
        {
            // Java: return (T[])this.collection.toArray(a);
            self.collection.clone()
        }
    }

    // Java: Iterator<E> iterator()
    pub fn iterator(&self) -> std::slice::Iter<'_, E> {
        // Java: return this.collection.iterator();
        self.collection.iter()
    }

    // Java: boolean add(E e)
    pub fn add(&mut self, e: E) -> bool {
        // Java: synchronized(this.mutex)
        let _lock = self.mutex.lock().unwrap();
        {
            // Java: return this.collection.add(e);
            self.collection.push(e);
            true
        }
    }

    // Java: boolean remove(Object o)
    pub fn remove(&mut self, o: &E) -> bool {
        // Java: synchronized(this.mutex)
        let _lock = self.mutex.lock().unwrap();
        {
            // Java: return this.collection.remove(o);
            if let Some(pos) = self.collection.iter().position(|x| x == o) {
                self.collection.remove(pos);
                true
            } else {
                false
            }
        }
    }

    // Java: boolean containsAll(Collection<?> coll)
    pub fn containsAll(&self, coll: &[E]) -> bool {
        // Java: synchronized(this.mutex)
        let _lock = self.mutex.lock().unwrap();
        {
            // Java: return this.collection.containsAll(coll);
            coll.iter().all(|item| self.collection.contains(item))
        }
    }

    // Java: boolean addAll(Collection<? extends E> coll)
    pub fn addAll(&mut self, coll: &[E]) -> bool {
        // Java: synchronized(this.mutex)
        let _lock = self.mutex.lock().unwrap();
        {
            // Java: return this.collection.addAll(coll);
            if coll.is_empty() {
                return false;
            }
            self.collection.extend_from_slice(coll);
            true
        }
    }

    // Java: boolean removeAll(Collection<?> coll)
    pub fn removeAll(&mut self, coll: &[E]) -> bool {
        // Java: synchronized(this.mutex)
        let _lock = self.mutex.lock().unwrap();
        {
            // Java: return this.collection.removeAll(coll);
            let old_len = self.collection.len();
            self.collection.retain(|x| !coll.contains(x));
            self.collection.len() != old_len
        }
    }

    // Java: boolean retainAll(Collection<?> coll)
    pub fn retainAll(&mut self, coll: &[E]) -> bool {
        // Java: synchronized(this.mutex)
        let _lock = self.mutex.lock().unwrap();
        {
            // Java: return this.collection.retainAll(coll);
            let old_len = self.collection.len();
            self.collection.retain(|x| coll.contains(x));
            self.collection.len() != old_len
        }
    }

    // Java: void clear()
    pub fn clear(&mut self) {
        // Java: synchronized(this.mutex)
        let _lock = self.mutex.lock().unwrap();
        {
            // Java: this.collection.clear();
            self.collection.clear();
        }
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
        // Java: synchronized(this.mutex)
        let _lock = self.mutex.lock().unwrap();
        {
            // Java: return this.collection.toString();
            format!("{:?}", self.collection)
        }
    }

    // Java: void writeObject(ObjectOutputStream s)
    fn writeObject(&self) {
        // Java: synchronized(this.mutex)
        let _lock = self.mutex.lock().unwrap();
        {
            // Java: s.defaultWriteObject();
        }
    }
}
