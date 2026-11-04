#![allow(non_snake_case, dead_code, unused_variables)]

// Java: import cn.wps.kfc.api.iterator.ILongIterator
use crate::kfc::api::iterator::ILongIterator::ILongIterator;
// Java: import cn.wps.kfc.api.procedure.ILongProcedure
use crate::kfc::api::procedure::ILongProcedure::ILongProcedure;
// Java: import java.util.Collection

// Java: public interface ILongList
pub trait ILongList {
    // Java: abstract long getNoEntryValue()
    fn getNoEntryValue(&self) -> i64;

    // Java: abstract int size()
    fn size(&self) -> i32;

    // Java: abstract boolean isEmpty()
    fn isEmpty(&self) -> bool;

    // Java: abstract boolean add(long var1)
    fn add(&mut self, var1: i64) -> bool;

    // Java: abstract void add(long[] var1)
    fn add_array(&mut self, var1: &[i64]);

    // Java: abstract void add(long[] var1, int var2, int var3)
    fn add_array_range(&mut self, var1: &[i64], var2: i32, var3: i32);

    // Java: abstract boolean addAll(Collection<? extends Long> var1)
    fn addAll_collection(&mut self, var1: &[i64]) -> bool;

    // Java: abstract boolean addAll(ILongList var1)
    fn addAll_list(&mut self, var1: &dyn ILongList) -> bool;

    // Java: abstract boolean addAll(long[] var1)
    fn addAll_array(&mut self, var1: &[i64]) -> bool;

    // Java: abstract void insert(int var1, long var2)
    fn insert(&mut self, var1: i32, var2: i64);

    // Java: abstract void insert(int var1, long[] var2)
    fn insert_array(&mut self, var1: i32, var2: &[i64]);

    // Java: abstract void insert(int var1, long[] var2, int var3, int var4)
    fn insert_array_range(&mut self, var1: i32, var2: &[i64], var3: i32, var4: i32);

    // Java: abstract long get(int var1)
    fn get(&self, var1: i32) -> i64;

    // Java: abstract long set(int var1, long var2)
    fn set(&mut self, var1: i32, var2: i64) -> i64;

    // Java: abstract void set(int var1, long[] var2)
    fn set_array(&mut self, var1: i32, var2: &[i64]);

    // Java: abstract void set(int var1, long[] var2, int var3, int var4)
    fn set_array_range(&mut self, var1: i32, var2: &[i64], var3: i32, var4: i32);

    // Java: abstract void clear()
    fn clear(&mut self);

    // Java: abstract boolean remove(long var1)
    fn remove_value(&mut self, var1: i64) -> bool;

    // Java: abstract long removeAt(int var1)
    fn removeAt(&mut self, var1: i32) -> i64;

    // Java: abstract void remove(int var1, int var2)
    fn remove_range(&mut self, var1: i32, var2: i32);

    // Java: abstract ILongList subList(int var1, int var2)
    fn subList(&self, var1: i32, var2: i32) -> Box<dyn ILongList>;

    // Java: abstract long[] toArray()
    fn toArray(&self) -> Vec<i64>;

    // Java: abstract long[] toArray(int var1, int var2)
    fn toArray_range(&self, var1: i32, var2: i32) -> Vec<i64>;

    // Java: abstract long[] toArray(long[] var1)
    fn toArray_dest(&self, var1: &mut [i64]) -> Vec<i64>;

    // Java: abstract long[] toArray(long[] var1, int var2, int var3)
    fn toArray_dest_range(&self, var1: &mut [i64], var2: i32, var3: i32) -> Vec<i64>;

    // Java: abstract long[] toArray(long[] var1, int var2, int var3, int var4)
    fn toArray_dest_range_offset(&self, var1: &mut [i64], var2: i32, var3: i32, var4: i32) -> Vec<i64>;

    // Java: abstract boolean forEach(ILongProcedure var1)
    fn forEach(&self, var1: &mut dyn ILongProcedure) -> bool;

    // Java: abstract void sort()
    fn sort(&mut self);

    // Java: abstract void sort(int var1, int var2)
    fn sort_range(&mut self, var1: i32, var2: i32);

    // Java: abstract void fill(long var1)
    fn fill(&mut self, var1: i64);

    // Java: abstract void fill(int var1, int var2, long var3)
    fn fill_range(&mut self, var1: i32, var2: i32, var3: i64);

    // Java: abstract int binarySearch(long var1)
    fn binarySearch(&self, var1: i64) -> i32;

    // Java: abstract int binarySearch(long var1, int var3, int var4)
    fn binarySearch_range(&self, var1: i64, var3: i32, var4: i32) -> i32;

    // Java: abstract int indexOf(long var1)
    fn indexOf(&self, var1: i64) -> i32;

    // Java: abstract int indexOf(int var1, long var2)
    fn indexOf_from(&self, var1: i32, var2: i64) -> i32;

    // Java: abstract int lastIndexOf(long var1)
    fn lastIndexOf(&self, var1: i64) -> i32;

    // Java: abstract int lastIndexOf(int var1, long var2)
    fn lastIndexOf_from(&self, var1: i32, var2: i64) -> i32;

    // Java: abstract boolean contains(long var1)
    fn contains(&self, var1: i64) -> bool;

    // Java: abstract ILongIterator iterator()
    fn iterator(&self) -> Box<dyn ILongIterator>;
}
