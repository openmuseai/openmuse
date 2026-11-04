#![allow(non_snake_case, dead_code, unused_variables, non_upper_case_globals)]

// Java: import cn.wps.kfc.api.iterator.IIntIterator
use crate::kfc::api::iterator::IIntIterator::IIntIterator;
// Java: import cn.wps.kfc.api.procedure.IIntProcedure
use crate::kfc::api::procedure::IIntProcedure::IIntProcedure;
// Java: import cn.wps.kfc.api.set.IIntSet
use crate::kfc::api::set::IIntSet::IIntSet;
// Java: import java.io.IOException
// Java: import java.io.ObjectOutputStream
// Java: import java.io.Serializable
// Java: import java.util.Collection
use std::sync::Mutex;

// Java field: long serialVersionUID
const SERIAL_VERSION_UID: i64 = 487447009682186044;

// Java: public class SyncIntSet implements IIntSet, Serializable
pub struct SyncIntSet {
    // Java field: IIntSet intSet
    pub(crate) intSet: Box<dyn IIntSet>,
    // Java field: Object mutex
    mutex: Mutex<()>,
}

// Java: public class SyncIntSet implements IIntSet, Serializable
impl SyncIntSet {
    // Java: SyncIntSet(IIntSet s)
    pub fn new(s: Box<dyn IIntSet>) -> Self {
        // Java: super();
        // Java: this.intSet = s;
        // Java: this.mutex = this;
        Self {
            intSet: s,
            mutex: Mutex::new(()),
        }
    }

    // Java: SyncIntSet(IIntSet s, Object mutex)
    pub fn new_with_mutex(s: Box<dyn IIntSet>) -> Self {
        // Java: super();
        // Java: this.intSet = s;
        // Java: this.mutex = mutex;
        Self {
            intSet: s,
            mutex: Mutex::new(()),
        }
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &dyn IIntSet) -> bool {
        // Java: synchronized(this.mutex)
        let _lock = self.mutex.lock().unwrap();
        {
            // Java: return this.intSet.equals(o);
            self.intSet.equals(o)
        }
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
        // Java: synchronized(this.mutex)
        let _lock = self.mutex.lock().unwrap();
        {
            // Java: return this.intSet.hashCode();
            self.intSet.hashCode()
        }
    }

    // Java: int size()
    pub fn size(&self) -> i32 {
        // Java: synchronized(this.mutex)
        let _lock = self.mutex.lock().unwrap();
        {
            // Java: return this.intSet.size();
            self.intSet.size()
        }
    }

    // Java: boolean isEmpty()
    pub fn isEmpty(&self) -> bool {
        // Java: synchronized(this.mutex)
        let _lock = self.mutex.lock().unwrap();
        {
            // Java: return this.intSet.isEmpty();
            self.intSet.isEmpty()
        }
    }

    // Java: boolean contains(int o)
    pub fn contains(&self, o: i32) -> bool {
        // Java: synchronized(this.mutex)
        let _lock = self.mutex.lock().unwrap();
        {
            // Java: return this.intSet.contains(o);
            self.intSet.contains(o)
        }
    }

    // Java: int[] toArray()
    pub fn toArray(&self) -> Vec<i32> {
        // Java: synchronized(this.mutex)
        let _lock = self.mutex.lock().unwrap();
        {
            // Java: return this.intSet.toArray();
            self.intSet.toArray()
        }
    }

    // Java: int[] toArray(int[] a)
    pub fn toArray_with(&self, a: &[i32]) -> Vec<i32> {
        // Java: synchronized(this.mutex)
        let _lock = self.mutex.lock().unwrap();
        {
            // Java: return this.intSet.toArray(a);
            self.intSet.toArray_with(a)
        }
    }

    // Java: IIntIterator iterator()
    pub fn iterator(&self) -> Box<dyn IIntIterator> {
        // Java: return this.intSet.iterator();
        self.intSet.iterator()
    }

    // Java: boolean add(int e)
    pub fn add(&mut self, e: i32) -> bool {
        // Java: synchronized(this.mutex)
        let _lock = self.mutex.lock().unwrap();
        {
            // Java: return this.intSet.add(e);
            self.intSet.add(e)
        }
    }

    // Java: boolean remove(int o)
    pub fn remove(&mut self, o: i32) -> bool {
        // Java: synchronized(this.mutex)
        let _lock = self.mutex.lock().unwrap();
        {
            // Java: return this.intSet.remove(o);
            self.intSet.remove(o)
        }
    }

    // Java: boolean containsAll(Collection<?> coll)
    pub fn containsAll(&self, coll: &[i32]) -> bool {
        // Java: synchronized(this.mutex)
        let _lock = self.mutex.lock().unwrap();
        {
            // Java: return this.intSet.containsAll(coll);
            self.intSet.containsAll(coll)
        }
    }

    // Java: boolean containsAll(int[] array)
    pub fn containsAll_arr(&self, array: &[i32]) -> bool {
        // Java: synchronized(this.mutex)
        let _lock = self.mutex.lock().unwrap();
        {
            // Java: return this.intSet.containsAll(array);
            self.intSet.containsAll_arr(array)
        }
    }

    // Java: boolean addAll(Collection<? extends Integer> coll)
    pub fn addAll(&mut self, coll: &[i32]) -> bool {
        // Java: synchronized(this.mutex)
        let _lock = self.mutex.lock().unwrap();
        {
            // Java: return this.intSet.addAll(coll);
            self.intSet.addAll(coll)
        }
    }

    // Java: boolean addAll(int[] array)
    pub fn addAll_arr(&mut self, array: &[i32]) -> bool {
        // Java: synchronized(this.mutex)
        let _lock = self.mutex.lock().unwrap();
        {
            // Java: return this.intSet.addAll(array);
            self.intSet.addAll_arr(array)
        }
    }

    // Java: boolean addAll(IIntSet intSet)
    pub fn addAll_set(&mut self, int_set: &dyn IIntSet) -> bool {
        // Java: synchronized(this.mutex)
        let _lock = self.mutex.lock().unwrap();
        {
            // Java: return intSet.addAll(intSet);
            self.intSet.addAll_set(int_set)
        }
    }

    // Java: boolean removeAll(Collection<?> coll)
    pub fn removeAll(&mut self, coll: &[i32]) -> bool {
        // Java: synchronized(this.mutex)
        let _lock = self.mutex.lock().unwrap();
        {
            // Java: return this.intSet.removeAll(coll);
            self.intSet.removeAll(coll)
        }
    }

    // Java: boolean removeAll(int[] array)
    pub fn removeAll_arr(&mut self, array: &[i32]) -> bool {
        // Java: synchronized(this.mutex)
        let _lock = self.mutex.lock().unwrap();
        {
            // Java: return this.intSet.removeAll(array);
            self.intSet.removeAll_arr(array)
        }
    }

    // Java: boolean retainAll(Collection<?> coll)
    pub fn retainAll(&mut self, coll: &[i32]) -> bool {
        // Java: synchronized(this.mutex)
        let _lock = self.mutex.lock().unwrap();
        {
            // Java: return this.intSet.retainAll(coll);
            self.intSet.retainAll(coll)
        }
    }

    // Java: boolean retainAll(int[] array)
    pub fn retainAll_arr(&mut self, array: &[i32]) -> bool {
        // Java: synchronized(this.mutex)
        let _lock = self.mutex.lock().unwrap();
        {
            // Java: return this.intSet.retainAll(array);
            self.intSet.retainAll_arr(array)
        }
    }

    // Java: int getNoEntryValue()
    pub fn getNoEntryValue(&self) -> i32 {
        // Java: return this.intSet.getNoEntryValue();
        self.intSet.getNoEntryValue()
    }

    // Java: boolean forEach(IIntProcedure procedure)
    pub fn forEach(&self, procedure: &dyn IIntProcedure) -> bool {
        // Java: synchronized(this.mutex)
        let _lock = self.mutex.lock().unwrap();
        {
            // Java: return this.intSet.forEach(procedure);
            self.intSet.forEach(procedure)
        }
    }

    // Java: void clear()
    pub fn clear(&mut self) {
        // Java: synchronized(this.mutex)
        let _lock = self.mutex.lock().unwrap();
        {
            // Java: this.intSet.clear();
            self.intSet.clear();
        }
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
        // Java: synchronized(this.mutex)
        let _lock = self.mutex.lock().unwrap();
        {
            // Java: return this.intSet.toString();
            String::new()
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
