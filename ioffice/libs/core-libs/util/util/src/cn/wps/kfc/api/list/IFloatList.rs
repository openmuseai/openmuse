#![allow(non_snake_case, dead_code)]

use crate::kfc::api::iterator::IFloatIterator::IFloatIterator;
use crate::kfc::api::procedure::IFloatProcedure::IFloatProcedure;

// Java: public interface IFloatList
pub trait IFloatList {
    // Java: abstract float getNoEntryValue()
    fn getNoEntryValue(&self) -> f32;

    // Java: abstract int size()
    fn size(&self) -> i32;

    // Java: abstract boolean isEmpty()
    fn isEmpty(&self) -> bool;

    // Java: abstract boolean add(float var1)
    fn add(&mut self, var1: f32) -> bool;

    // Java: abstract void add(float[] var1)
    fn add_array(&mut self, var1: &[f32]);

    // Java: abstract void add(float[] var1, int var2, int var3)
    fn add_array_range(&mut self, var1: &[f32], var2: i32, var3: i32);

    // Java: abstract boolean addAll(Collection<? extends Float> var1)
    fn addAll_collection(&mut self, var1: &[f32]) -> bool;

    // Java: abstract boolean addAll(IFloatList var1)
    fn addAll_list(&mut self, var1: &dyn IFloatList) -> bool;

    // Java: abstract boolean addAll(float[] var1)
    fn addAll_array(&mut self, var1: &[f32]) -> bool;

    // Java: abstract void insert(int var1, float var2)
    fn insert(&mut self, var1: i32, var2: f32);

    // Java: abstract void insert(int var1, float[] var2)
    fn insert_array(&mut self, var1: i32, var2: &[f32]);

    // Java: abstract void insert(int var1, float[] var2, int var3, int var4)
    fn insert_array_range(&mut self, var1: i32, var2: &[f32], var3: i32, var4: i32);

    // Java: abstract float get(int var1)
    fn get(&self, var1: i32) -> f32;

    // Java: abstract float set(int var1, float var2)
    fn set(&mut self, var1: i32, var2: f32) -> f32;

    // Java: abstract void set(int var1, float[] var2)
    fn set_array(&mut self, var1: i32, var2: &[f32]);

    // Java: abstract void set(int var1, float[] var2, int var3, int var4)
    fn set_array_range(&mut self, var1: i32, var2: &[f32], var3: i32, var4: i32);

    // Java: abstract void clear()
    fn clear(&mut self);

    // Java: abstract boolean remove(float var1)
    fn remove(&mut self, var1: f32) -> bool;

    // Java: abstract float removeAt(int var1)
    fn removeAt(&mut self, var1: i32) -> f32;

    // Java: abstract void remove(int var1, int var2)
    fn remove_range(&mut self, var1: i32, var2: i32);

    // Java: abstract boolean removeAll(Collection<?> var1)
    fn removeAll_collection(&mut self, var1: &[f32]) -> bool;

    // Java: abstract boolean removeAll(IFloatList var1)
    fn removeAll_list(&mut self, var1: &dyn IFloatList) -> bool;

    // Java: abstract boolean removeAll(float[] var1)
    fn removeAll_array(&mut self, var1: &[f32]) -> bool;

    // Java: abstract void reverse()
    fn reverse(&mut self);

    // Java: abstract void reverse(int var1, int var2)
    fn reverse_range(&mut self, var1: i32, var2: i32);

    // Java: abstract IFloatList subList(int var1, int var2)
    fn subList(&self, var1: i32, var2: i32) -> Box<dyn IFloatList>;

    // Java: abstract float[] toArray()
    fn toArray(&self) -> Vec<f32>;

    // Java: abstract float[] toArray(int var1, int var2)
    fn toArray_range(&self, var1: i32, var2: i32) -> Vec<f32>;

    // Java: abstract float[] toArray(float[] var1)
    fn toArray_dest(&self, var1: &mut [f32]) -> Vec<f32>;

    // Java: abstract float[] toArray(float[] var1, int var2, int var3)
    fn toArray_dest_range(&self, var1: &mut [f32], var2: i32, var3: i32) -> Vec<f32>;

    // Java: abstract float[] toArray(float[] var1, int var2, int var3, int var4)
    fn toArray_dest_range_offset(&self, var1: &mut [f32], var2: i32, var3: i32, var4: i32) -> Vec<f32>;

    // Java: abstract boolean forEach(IFloatProcedure var1)
    fn forEach(&self, var1: &mut dyn IFloatProcedure) -> bool;

    // Java: abstract void sort()
    fn sort(&mut self);

    // Java: abstract void sort(int var1, int var2)
    fn sort_range(&mut self, var1: i32, var2: i32);

    // Java: abstract void fill(float var1)
    fn fill(&mut self, var1: f32);

    // Java: abstract void fill(int var1, int var2, float var3)
    fn fill_range(&mut self, var1: i32, var2: i32, var3: f32);

    // Java: abstract int binarySearch(float var1)
    fn binarySearch(&self, var1: f32) -> i32;

    // Java: abstract int binarySearch(float var1, int var2, int var3)
    fn binarySearch_range(&self, var1: f32, var2: i32, var3: i32) -> i32;

    // Java: abstract int indexOf(float var1)
    fn indexOf(&self, var1: f32) -> i32;

    // Java: abstract int indexOf(int var1, float var2)
    fn indexOf_from(&self, var1: i32, var2: f32) -> i32;

    // Java: abstract int lastIndexOf(float var1)
    fn lastIndexOf(&self, var1: f32) -> i32;

    // Java: abstract int lastIndexOf(int var1, float var2)
    fn lastIndexOf_from(&self, var1: i32, var2: f32) -> i32;

    // Java: abstract boolean contains(float var1)
    fn contains(&self, var1: f32) -> bool;

    // Java: abstract float max()
    fn max(&self) -> f32;

    // Java: abstract float min()
    fn min(&self) -> f32;

    // Java: abstract float sum()
    fn sum(&self) -> f32;

    // Java: abstract IFloatIterator iterator()
    fn iterator(&self) -> Box<dyn IFloatIterator>;
}
