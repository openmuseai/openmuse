#![allow(non_snake_case, dead_code, unused_variables, non_upper_case_globals)]

// Java: import cn.wps.kfc.api.iterator.IIntObjectIterator
// Java: import cn.wps.kfc.api.map.IIntObjectMap
// Java: import cn.wps.kfc.api.procedure.IIntObjectProcedure
// Java: import cn.wps.kfc.api.procedure.IIntProcedure
// Java: import cn.wps.kfc.api.procedure.IObjectProcedure
// Java: import cn.wps.kfc.api.set.IIntSet
// Java: import java.io.IOException
// Java: import java.io.ObjectOutputStream
// Java: import java.io.Serializable
// Java: import java.util.Collection
// Java: import java.util.Map
use std::sync::Mutex;
use std::collections::HashMap;

use crate::kfc::api::map::IIntObjectMap::IIntObjectMap;
use crate::kfc::api::iterator::IIntObjectIterator::IIntObjectIterator;
use crate::kfc::api::procedure::IIntObjectProcedure::IIntObjectProcedure;
use crate::kfc::api::procedure::IIntProcedure::IIntProcedure;
use crate::kfc::api::procedure::IObjectProcedure::IObjectProcedure;
use crate::kfc::api::set::IIntSet::IIntSet;

// Java field: long serialVersionUID
const SERIAL_VERSION_UID: i64 = 1978198479659022715;

// Java: public class SyncIntObjectMap implements IIntObjectMap, Serializable
pub struct SyncIntObjectMap<V> {
    // Java field: IIntObjectMap<V> map
    map: Box<dyn IIntObjectMap<V>>,
    // Java field: Object mutex
    mutex: Mutex<()>,
    // Java field: IIntSet keySet
    keySet: Option<Box<dyn IIntSet>>,
    // Java field: Collection<V> values
    cached_values: Option<Vec<V>>,
}

// Java: public class SyncIntObjectMap implements IIntObjectMap, Serializable
impl<V> SyncIntObjectMap<V> {
    // Java: SyncIntObjectMap(IIntObjectMap<V> m)
    pub fn new(m: Box<dyn IIntObjectMap<V>>) -> Self {
        // Java: super();
        // Java: if (m == null)
        // Java: throw new NullPointerException();
        // Java: else
        // Java: this.map = m;
        // Java: this.mutex = this;
        Self {
            map: m,
            mutex: Mutex::new(()),
            keySet: None,
            cached_values: None,
        }
    }

    // Java: SyncIntObjectMap(IIntObjectMap<V> m, Object mutex)
    pub fn new_with_mutex(m: Box<dyn IIntObjectMap<V>>) -> Self {
        // Java: super();
        // Java: this.map = m;
        // Java: this.mutex = mutex;
        Self {
            map: m,
            mutex: Mutex::new(()),
            keySet: None,
            cached_values: None,
        }
    }

    // Java: int size()
    pub fn size(&self) -> i32 {
        // Java: synchronized(this.mutex)
        let _lock = self.mutex.lock().unwrap();
        // Java: return this.map.size();
        self.map.size()
    }

    // Java: boolean isEmpty()
    pub fn isEmpty(&self) -> bool {
        // Java: synchronized(this.mutex)
        let _lock = self.mutex.lock().unwrap();
        // Java: return this.map.isEmpty();
        self.map.isEmpty()
    }

    // Java: boolean containsKey(int key)
    pub fn containsKey(&self, key: i32) -> bool {
        // Java: synchronized(this.mutex)
        let _lock = self.mutex.lock().unwrap();
        // Java: return this.map.containsKey(key);
        self.map.containsKey(key)
    }

    // Java: boolean containsValue(Object value)
    pub fn containsValue(&self, value: &V) -> bool {
        // Java: synchronized(this.mutex)
        let _lock = self.mutex.lock().unwrap();
        // Java: return this.map.containsValue(value);
        self.map.containsValue(value)
    }

    // Java: V get(int key)
    pub fn get(&self, key: i32) -> Option<&V> {
        // Java: synchronized(this.mutex)
        // Java: return this.map.get(key);
        self.map.get(key)
    }

    // Java: V put(int key, V value)
    pub fn put(&mut self, key: i32, value: V) -> Option<V> {
        // Java: synchronized(this.mutex)
        let _lock = self.mutex.lock().unwrap();
        // Java: return this.map.put(key, value);
        self.map.put(key, value)
    }

    // Java: V remove(int key)
    pub fn remove(&mut self, key: i32) -> Option<V> {
        // Java: synchronized(this.mutex)
        let _lock = self.mutex.lock().unwrap();
        // Java: return this.map.remove(key);
        self.map.remove(key)
    }

    // Java: void putAll(Map<? extends Integer, ? extends V> map)
    pub fn putAll_map(&mut self, map: &HashMap<i32, V>) where V: Clone {
        // Java: synchronized(this.mutex)
        let _lock = self.mutex.lock().unwrap();
        // Java: this.map.putAll(map);
        self.map.putAll_map(map);
    }

    // Java: void putAll(IIntObjectMap<? extends V> map)
    pub fn putAll(&mut self, map: &dyn IIntObjectMap<V>) where V: Clone {
        // Java: synchronized(this.mutex)
        let _lock = self.mutex.lock().unwrap();
        // Java: this.map.putAll(map);
        self.map.putAll(map);
    }

    // Java: void clear()
    pub fn clear(&mut self) {
        // Java: synchronized(this.mutex)
        let _lock = self.mutex.lock().unwrap();
        // Java: this.map.clear();
        self.map.clear();
    }

    // Java: IIntSet keySet()
    pub fn keySet(&self) -> Box<dyn IIntSet> {
        // Java: synchronized(this.mutex)
        let _lock = self.mutex.lock().unwrap();
        // Java: if (this.keySet == null)
        // Java: this.keySet = new SyncIntSet(this.map.keySet(), this.mutex);
        // Java: return this.keySet;
        self.map.keySet()
    }

    // Java: _keys: &[i32]()
    pub fn keys(&self) -> Vec<i32> {
        // Java: synchronized(this.mutex)
        let _lock = self.mutex.lock().unwrap();
        // Java: return this.map.keys();
        self.map.keys()
    }

    // Java: _keys: &[i32](_array: &[i32])
    pub fn keys_with_dest(&self, array: &mut [i32]) -> Vec<i32> {
        // Java: synchronized(this.mutex)
        let _lock = self.mutex.lock().unwrap();
        // Java: return this.map.keys(array);
        self.map.keys_with_dest(array)
    }

    // Java: Collection<V> valueCollection()
    pub fn valueCollection(&self) -> Vec<&V> {
        // Java: synchronized(this.mutex)
        let _lock = self.mutex.lock().unwrap();
        // Java: if (this.values == null)
        // Java: this.values = new SyncCollection<V>(this.map.valueCollection(), this.mutex);
        // Java: return this.values;
        self.map.valueCollection()
    }

    // Java: Object[] values()
    pub fn values(&self) -> Vec<&V> {
        // Java: synchronized(this.mutex)
        let _lock = self.mutex.lock().unwrap();
        // Java: return this.map.values();
        self.map.values()
    }

    // Java: V[] values(V[] array)
    pub fn values_with_dest(&self, array: &mut [V]) -> Vec<V> where V: Clone {
        // Java: synchronized(this.mutex)
        let _lock = self.mutex.lock().unwrap();
        // Java: return (V[])this.map.values(array);
        self.map.values_with_dest(array)
    }

    // Java: IIntObjectIterator<V> iterator()
    pub fn iterator(&self) -> Box<dyn IIntObjectIterator<V>> {
        // Java: return this.map.iterator();
        self.map.iterator()
    }

    // Java: int getNoEntryKey()
    pub fn getNoEntryKey(&self) -> i32 {
        // Java: return this.map.getNoEntryKey();
        self.map.getNoEntryKey()
    }

    // Java: boolean forEachKey(IIntProcedure procedure)
    pub fn forEachKey(&self, procedure: &dyn IIntProcedure) -> bool {
        // Java: synchronized(this.mutex)
        let _lock = self.mutex.lock().unwrap();
        // Java: return this.map.forEachKey(procedure);
        self.map.forEachKey(procedure)
    }

    // Java: boolean forEachValue(IObjectProcedure<? super V> procedure)
    pub fn forEachValue(&self, procedure: &dyn IObjectProcedure<V>) -> bool {
        // Java: synchronized(this.mutex)
        let _lock = self.mutex.lock().unwrap();
        // Java: return this.map.forEachValue(procedure);
        self.map.forEachValue(procedure)
    }

    // Java: boolean forEachEntry(IIntObjectProcedure<? super V> procedure)
    pub fn forEachEntry(&self, procedure: &dyn IIntObjectProcedure<V>) -> bool {
        // Java: synchronized(this.mutex)
        let _lock = self.mutex.lock().unwrap();
        // Java: return this.map.forEachEntry(procedure);
        self.map.forEachEntry(procedure)
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &dyn IIntObjectMap<V>) -> bool {
        // Java: synchronized(this.mutex)
        let _lock = self.mutex.lock().unwrap();
        // Java: return this.map.equals(o);
        self.map.equals(o)
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
        // Java: synchronized(this.mutex)
        let _lock = self.mutex.lock().unwrap();
        // Java: return this.map.hashCode();
        self.map.hashCode()
    }

    // Java: String toString()
    pub fn to_string(&self) -> String {
        // Java: synchronized(this.mutex)
        let _lock = self.mutex.lock().unwrap();
        // Java: return this.map.toString();
        String::new()
    }

    // Java: void writeObject(ObjectOutputStream s)
    fn writeObject(&self) {
        // Java: synchronized(this.mutex)
        let _lock = self.mutex.lock().unwrap();
        // Java: s.defaultWriteObject();
    }
}
