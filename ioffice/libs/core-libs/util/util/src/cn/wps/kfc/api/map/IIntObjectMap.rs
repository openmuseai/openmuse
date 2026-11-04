// Java: import cn.wps.kfc.api.iterator.IIntObjectIterator
use crate::kfc::api::iterator::IIntObjectIterator::IIntObjectIterator;
// Java: import cn.wps.kfc.api.procedure.IIntObjectProcedure
use crate::kfc::api::procedure::IIntObjectProcedure::IIntObjectProcedure;
// Java: import cn.wps.kfc.api.procedure.IIntProcedure
use crate::kfc::api::procedure::IIntProcedure::IIntProcedure;
// Java: import cn.wps.kfc.api.procedure.IObjectProcedure
use crate::kfc::api::procedure::IObjectProcedure::IObjectProcedure;
// Java: import cn.wps.kfc.api.set.IIntSet
use crate::kfc::api::set::IIntSet::IIntSet;
use std::collections::HashMap;

// Java: public interface IIntObjectMap<V>
pub trait IIntObjectMap<V> {
    // Java: abstract int getNoEntryKey()
    fn getNoEntryKey(&self) -> i32;

    // Java: abstract int size()
    fn size(&self) -> i32;

    // Java: abstract boolean isEmpty()
    fn isEmpty(&self) -> bool;

    // Java: abstract boolean containsKey(int var1)
    fn containsKey(&self, var1: i32) -> bool;

    // Java: abstract boolean containsValue(Object var1)
    fn containsValue(&self, var1: &V) -> bool;

    // Java: abstract V get(int var1)
    fn get(&self, var1: i32) -> Option<&V>;

    // Java: abstract V put(int var1, V var2)
    fn put(&mut self, var1: i32, var2: V) -> Option<V>;

    // Java: abstract V remove(int var1)
    fn remove(&mut self, var1: i32) -> Option<V>;

    // Java: abstract void putAll(Map<? extends Integer, ? extends V> var1)
    fn putAll_map(&mut self, var1: &HashMap<i32, V>) where V: Clone;

    // Java: abstract void putAll(IIntObjectMap<? extends V> var1)
    fn putAll(&mut self, var1: &dyn IIntObjectMap<V>) where V: Clone;

    // Java: abstract void clear()
    fn clear(&mut self);

    // Java: abstract IIntSet keySet()
    fn keySet(&self) -> Box<dyn IIntSet>;

    // Java: abstract _keys: &[i32]()
    fn keys(&self) -> Vec<i32>;

    // Java: abstract _keys: &[i32](_var1: &[i32])
    fn keys_with_dest(&self, var1: &mut [i32]) -> Vec<i32>;

    // Java: abstract Collection<V> valueCollection()
    fn valueCollection(&self) -> Vec<&V>;

    // Java: abstract Object[] values()
    fn values(&self) -> Vec<&V>;

    // Java: abstract V[] values(V[] var1)
    fn values_with_dest(&self, var1: &mut [V]) -> Vec<V> where V: Clone;

    // Java: abstract IIntObjectIterator<V> iterator()
    fn iterator(&self) -> Box<dyn IIntObjectIterator<V>>;

    // Java: abstract boolean forEachKey(IIntProcedure var1)
    fn forEachKey(&self, var1: &dyn IIntProcedure) -> bool;

    // Java: abstract boolean forEachValue(IObjectProcedure<? super V> var1)
    fn forEachValue(&self, var1: &dyn IObjectProcedure<V>) -> bool;

    // Java: abstract boolean forEachEntry(IIntObjectProcedure<? super V> var1)
    fn forEachEntry(&self, var1: &dyn IIntObjectProcedure<V>) -> bool;

    // Java: abstract boolean equals(Object var1)
    fn equals(&self, var1: &dyn IIntObjectMap<V>) -> bool;

    // Java: abstract int hashCode()
    fn hashCode(&self) -> i32;
}
