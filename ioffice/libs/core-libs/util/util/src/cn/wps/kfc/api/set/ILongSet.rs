// Java: import cn.wps.kfc.api.iterator.ILongIterator
// Java: import cn.wps.kfc.api.procedure.ILongProcedure
// Java: import java.util.Collection

// Java: public interface ILongSet
pub trait ILongSet {
    // Java: abstract long getNoEntryValue()
    fn getNoEntryValue(&self) -> i64;
    // Java: abstract int size()
    fn size(&self) -> i32;
    // Java: abstract boolean isEmpty()
    fn isEmpty(&self) -> bool;
    // Java: abstract boolean contains(long var1)
    fn contains(&self, value: i64) -> bool;
    // Java: abstract ILongIterator iterator()
    fn iterator(&self) -> Box<dyn Iterator<Item = i64> + '_>;
    // Java: abstract long[] toArray()
    fn toArray(&self) -> Vec<i64>;
    // Java: abstract long[] toArray(long[] var1)
    fn toArray_into(&self, dest: &mut Vec<i64>) -> Vec<i64>;
    // Java: abstract boolean add(long var1)
    fn add(&mut self, value: i64) -> bool;
    // Java: abstract boolean remove(long var1)
    fn remove(&mut self, value: i64) -> bool;
    // Java: abstract boolean containsAll(Collection<?> var1)
    fn containsAll(&self, collection: &[i64]) -> bool;
    // Java: abstract boolean containsAll(long[] var1)
    fn containsAll_array(&self, array: &[i64]) -> bool;
    // Java: abstract boolean addAll(Collection<? extends Long> var1)
    fn addAll(&mut self, collection: &[i64]) -> bool;
    // Java: abstract boolean addAll(long[] var1)
    fn addAll_array(&mut self, array: &[i64]) -> bool;
    // Java: abstract boolean addAll(ILongSet var1)
    fn addAll_set(&mut self, set: &dyn ILongSet) -> bool;
    // Java: abstract boolean retainAll(Collection<?> var1)
    fn retainAll(&mut self, collection: &[i64]) -> bool;
    // Java: abstract boolean retainAll(long[] var1)
    fn retainAll_array(&mut self, array: &[i64]) -> bool;
    // Java: abstract boolean removeAll(Collection<?> var1)
    fn removeAll(&mut self, collection: &[i64]) -> bool;
    // Java: abstract boolean removeAll(long[] var1)
    fn removeAll_array(&mut self, array: &[i64]) -> bool;
    // Java: abstract void clear()
    fn clear(&mut self);
    // Java: abstract boolean forEach(ILongProcedure var1)
    fn forEach(&self, procedure: &mut dyn FnMut(i64) -> bool) -> bool;
    // Java: abstract boolean equals(Object var1)
    fn equals(&self, other: &dyn ILongSet) -> bool;
    // Java: abstract int hashCode()
    fn hashCode(&self) -> i32;
}
