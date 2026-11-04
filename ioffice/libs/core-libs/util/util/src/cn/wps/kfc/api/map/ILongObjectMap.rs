// Java: import cn.wps.kfc.api.iterator.ILongObjectIterator
// Java: import cn.wps.kfc.api.procedure.ILongObjectProcedure
// Java: import cn.wps.kfc.api.procedure.ILongProcedure
// Java: import cn.wps.kfc.api.procedure.IObjectProcedure
// Java: import cn.wps.kfc.api.set.ILongSet
// Java: import java.util.Collection
// Java: import java.util.Map

// Java: public interface ILongObjectMap<V>
pub trait ILongObjectMap<V> {
    // Java: abstract long getNoEntryKey()
    fn getNoEntryKey(&self) -> i64;
    // Java: abstract int size()
    fn size(&self) -> i32;
    // Java: abstract boolean isEmpty()
    fn isEmpty(&self) -> bool;
    // Java: abstract boolean containsKey(long var1)
    fn containsKey(&self, key: i64) -> bool;
    // Java: abstract boolean containsValue(Object var1)
    fn containsValue(&self, value: &V) -> bool;
    // Java: abstract V get(long var1)
    fn get(&self, key: i64) -> Option<&V>;
    // Java: abstract V put(long var1, V var3)
    fn put(&mut self, key: i64, value: V) -> Option<V>;
    // Java: abstract V remove(long var1)
    fn remove(&mut self, key: i64) -> Option<V>;
    // Java: abstract void putAll(Map<? extends Long, ? extends V> var1)
    fn putAll_map(&mut self, map: &std::collections::HashMap<i64, V>) where V: Clone;
    // Java: abstract void putAll(ILongObjectMap<? extends V> var1)
    fn putAll(&mut self, map: &dyn ILongObjectMap<V>) where V: Clone;
    // Java: abstract void clear()
    fn clear(&mut self);
    // Java: abstract ILongSet keySet()
    fn keySet(&self) -> Vec<i64>;
    // Java: abstract long[] keys()
    fn keys(&self) -> Vec<i64>;
    // Java: abstract long[] keys(long[] var1)
    fn keys_into(&self, dest: &mut Vec<i64>);
    // Java: abstract Collection<V> valueCollection()
    fn valueCollection(&self) -> Vec<&V>;
    // Java: abstract Object[] values()
    fn values(&self) -> Vec<&V>;
    // Java: abstract V[] values(V[] var1)
    fn values_into(&self, dest: &mut Vec<V>) where V: Clone;
    // Java: abstract ILongObjectIterator<V> iterator()
    fn iterator(&self) -> Box<dyn Iterator<Item = (i64, &V)> + '_>;
    // Java: abstract boolean forEachKey(ILongProcedure var1)
    fn forEachKey(&self, procedure: &mut dyn FnMut(i64) -> bool) -> bool;
    // Java: abstract boolean forEachValue(IObjectProcedure<? super V> var1)
    fn forEachValue(&self, procedure: &mut dyn FnMut(&V) -> bool) -> bool;
    // Java: abstract boolean forEachEntry(ILongObjectProcedure<? super V> var1)
    fn forEachEntry(&self, procedure: &mut dyn FnMut(i64, &V) -> bool) -> bool;
    // Java: abstract boolean retainEntries(ILongObjectProcedure<? super V> var1)
    fn retainEntries(&mut self, procedure: &mut dyn FnMut(i64, &V) -> bool) -> bool;
    // Java: abstract boolean equals(Object var1)
    fn equals(&self, other: &dyn ILongObjectMap<V>) -> bool;
    // Java: abstract int hashCode()
    fn hashCode(&self) -> i32;
}
