// Java: import cn.wps.kfc.api.iterator.IIntIterator
use crate::kfc::api::iterator::IIntIterator::IIntIterator;
// Java: import cn.wps.kfc.api.procedure.IIntProcedure
use crate::kfc::api::procedure::IIntProcedure::IIntProcedure;

// Java: public interface IIntSet
pub trait IIntSet {
    // Java: abstract int getNoEntryValue()
    fn getNoEntryValue(&self) -> i32;

    // Java: abstract int size()
    fn size(&self) -> i32;

    // Java: abstract boolean isEmpty()
    fn isEmpty(&self) -> bool;

    // Java: abstract boolean contains(int var1)
    fn contains(&self, var1: i32) -> bool;

    // Java: abstract IIntIterator iterator()
    fn iterator(&self) -> Box<dyn IIntIterator>;

    // Java: abstract _toArray: &[i32]()
    fn toArray(&self) -> Vec<i32>;

    // Java: abstract _toArray: &[i32](_var1: &[i32])
    fn toArray_with(&self, var1: &[i32]) -> Vec<i32>;

    // Java: abstract boolean add(int var1)
    fn add(&mut self, var1: i32) -> bool;

    // Java: abstract boolean remove(int var1)
    fn remove(&mut self, var1: i32) -> bool;

    // Java: abstract boolean containsAll(Collection<?> var1)
    fn containsAll(&self, var1: &[i32]) -> bool;

    // Java: abstract boolean containsAll(_var1: &[i32])
    fn containsAll_arr(&self, var1: &[i32]) -> bool;

    // Java: abstract boolean addAll(Collection<? extends Integer> var1)
    fn addAll(&mut self, var1: &[i32]) -> bool;

    // Java: abstract boolean addAll(_var1: &[i32])
    fn addAll_arr(&mut self, var1: &[i32]) -> bool;

    // Java: abstract boolean addAll(IIntSet var1)
    fn addAll_set(&mut self, var1: &dyn IIntSet) -> bool;

    // Java: abstract boolean retainAll(Collection<?> var1)
    fn retainAll(&mut self, var1: &[i32]) -> bool;

    // Java: abstract boolean retainAll(_var1: &[i32])
    fn retainAll_arr(&mut self, var1: &[i32]) -> bool;

    // Java: abstract boolean removeAll(Collection<?> var1)
    fn removeAll(&mut self, var1: &[i32]) -> bool;

    // Java: abstract boolean removeAll(_var1: &[i32])
    fn removeAll_arr(&mut self, var1: &[i32]) -> bool;

    // Java: abstract void clear()
    fn clear(&mut self);

    // Java: abstract boolean forEach(IIntProcedure var1)
    fn forEach(&self, var1: &dyn IIntProcedure) -> bool;

    // Java: abstract boolean equals(Object var1)
    fn equals(&self, other: &dyn IIntSet) -> bool;

    // Java: abstract int hashCode()
    fn hashCode(&self) -> i32;
}
