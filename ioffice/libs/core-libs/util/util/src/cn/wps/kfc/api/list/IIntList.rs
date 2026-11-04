#![allow(non_snake_case, dead_code)]

use crate::kfc::api::iterator::IIntIterator::IIntIterator;
use crate::kfc::api::procedure::IIntProcedure::IIntProcedure;

// Java: public interface IIntList
pub trait IIntList {
    // Java: abstract int getNoEntryValue()
    fn getNoEntryValue(&self) -> i32;

    // Java: abstract int size()
    fn size(&self) -> i32;

    // Java: abstract boolean isEmpty()
    fn isEmpty(&self) -> bool;

    // Java: abstract boolean add(int var1)
    fn add(&mut self, var1: i32) -> bool;

    // Java: abstract void add(int[] var1)
    fn add_array(&mut self, var1: &[i32]);

    // Java: abstract void add(int[] var1, int var2, int var3)
    fn add_array_range(&mut self, var1: &[i32], var2: i32, var3: i32);

    // Java: abstract boolean addAll(Collection<? extends Integer> var1)
    fn addAll_collection(&mut self, var1: &[i32]) -> bool;

    // Java: abstract boolean addAll(IIntList var1)
    fn addAll_list(&mut self, var1: &dyn IIntList) -> bool;

    // Java: abstract boolean addAll(int[] var1)
    fn addAll_array(&mut self, var1: &[i32]) -> bool;

    // Java: abstract void insert(int var1, int var2)
    fn insert(&mut self, var1: i32, var2: i32);

    // Java: abstract void insert(int var1, int[] var2)
    fn insert_array(&mut self, var1: i32, var2: &[i32]);

    // Java: abstract void insert(int var1, int[] var2, int var3, int var4)
    fn insert_array_range(&mut self, var1: i32, var2: &[i32], var3: i32, var4: i32);

    // Java: abstract int get(int var1)
    fn get(&self, var1: i32) -> i32;

    // Java: abstract int set(int var1, int var2)
    fn set(&mut self, var1: i32, var2: i32) -> i32;

    // Java: abstract void set(int var1, int[] var2)
    fn set_array(&mut self, var1: i32, var2: &[i32]);

    // Java: abstract void set(int var1, int[] var2, int var3, int var4)
    fn set_array_range(&mut self, var1: i32, var2: &[i32], var3: i32, var4: i32);

    // Java: abstract void clear()
    fn clear(&mut self);

    // Java: abstract boolean remove(int var1)
    fn remove(&mut self, var1: i32) -> bool;

    // Java: abstract int removeAt(int var1)
    fn removeAt(&mut self, var1: i32) -> i32;

    // Java: abstract void remove(int var1, int var2)
    fn remove_range(&mut self, var1: i32, var2: i32);

    // Java: abstract IIntList subList(int var1, int var2)
    fn subList(&self, var1: i32, var2: i32) -> Box<dyn IIntList>;

    // Java: abstract int[] toArray()
    fn toArray(&self) -> Vec<i32>;

    // Java: abstract int[] toArray(int var1, int var2)
    fn toArray_range(&self, var1: i32, var2: i32) -> Vec<i32>;

    // Java: abstract int[] toArray(int[] var1)
    fn toArray_dest(&self, var1: &mut [i32]) -> Vec<i32>;

    // Java: abstract int[] toArray(int[] var1, int var2, int var3)
    fn toArray_dest_range(&self, var1: &mut [i32], var2: i32, var3: i32) -> Vec<i32>;

    // Java: abstract int[] toArray(int[] var1, int var2, int var3, int var4)
    fn toArray_dest_range_offset(&self, var1: &mut [i32], var2: i32, var3: i32, var4: i32) -> Vec<i32>;

    // Java: abstract boolean forEach(IIntProcedure var1)
    fn forEach(&self, var1: &mut dyn IIntProcedure) -> bool;

    // Java: abstract void sort()
    fn sort(&mut self);

    // Java: abstract void sort(int var1, int var2)
    fn sort_range(&mut self, var1: i32, var2: i32);

    // Java: abstract void fill(int var1)
    fn fill(&mut self, var1: i32);

    // Java: abstract void fill(int var1, int var2, int var3)
    fn fill_range(&mut self, var1: i32, var2: i32, var3: i32);

    // Java: abstract int binarySearch(int var1)
    fn binarySearch(&self, var1: i32) -> i32;

    // Java: abstract int binarySearch(int var1, int var2, int var3)
    fn binarySearch_range(&self, var1: i32, var2: i32, var3: i32) -> i32;

    // Java: abstract int indexOf(int var1)
    fn indexOf(&self, var1: i32) -> i32;

    // Java: abstract int indexOf(int var1, int var2)
    fn indexOf_from(&self, var1: i32, var2: i32) -> i32;

    // Java: abstract int lastIndexOf(int var1)
    fn lastIndexOf(&self, var1: i32) -> i32;

    // Java: abstract int lastIndexOf(int var1, int var2)
    fn lastIndexOf_from(&self, var1: i32, var2: i32) -> i32;

    // Java: abstract boolean contains(int var1)
    fn contains(&self, var1: i32) -> bool;

    // Java: abstract IIntIterator iterator()
    fn iterator(&self) -> Box<dyn IIntIterator>;
}
