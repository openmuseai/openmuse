#![allow(non_snake_case, dead_code)]

use crate::kfc::api::iterator::IDoubleIterator::IDoubleIterator;
use crate::kfc::api::procedure::IDoubleProcedure::IDoubleProcedure;

// Java: public interface IDoubleList
pub trait IDoubleList {
    // Java: abstract double getNoEntryValue()
    fn getNoEntryValue(&self) -> f64;

    // Java: abstract int size()
    fn size(&self) -> i32;

    // Java: abstract boolean isEmpty()
    fn isEmpty(&self) -> bool;

    // Java: abstract boolean add(double var1)
    fn add(&mut self, var1: f64) -> bool;

    // Java: abstract void add(double[] var1)
    fn add_array(&mut self, var1: &[f64]);

    // Java: abstract void add(double[] var1, int var2, int var3)
    fn add_array_range(&mut self, var1: &[f64], var2: i32, var3: i32);

    // Java: abstract boolean addAll(Collection<? extends Double> var1)
    fn addAll_collection(&mut self, var1: &[f64]) -> bool;

    // Java: abstract boolean addAll(IDoubleList var1)
    fn addAll_list(&mut self, var1: &dyn IDoubleList) -> bool;

    // Java: abstract boolean addAll(double[] var1)
    fn addAll_array(&mut self, var1: &[f64]) -> bool;

    // Java: abstract void insert(int var1, double var2)
    fn insert(&mut self, var1: i32, var2: f64);

    // Java: abstract void insert(int var1, double[] var2)
    fn insert_array(&mut self, var1: i32, var2: &[f64]);

    // Java: abstract void insert(int var1, double[] var2, int var3, int var4)
    fn insert_array_range(&mut self, var1: i32, var2: &[f64], var3: i32, var4: i32);

    // Java: abstract double get(int var1)
    fn get(&self, var1: i32) -> f64;

    // Java: abstract double set(int var1, double var2)
    fn set(&mut self, var1: i32, var2: f64) -> f64;

    // Java: abstract void set(int var1, double[] var2)
    fn set_array(&mut self, var1: i32, var2: &[f64]);

    // Java: abstract void set(int var1, double[] var2, int var3, int var4)
    fn set_array_range(&mut self, var1: i32, var2: &[f64], var3: i32, var4: i32);

    // Java: abstract void clear()
    fn clear(&mut self);

    // Java: abstract boolean remove(double var1)
    fn remove(&mut self, var1: f64) -> bool;

    // Java: abstract double removeAt(int var1)
    fn removeAt(&mut self, var1: i32) -> f64;

    // Java: abstract void remove(int var1, int var2)
    fn remove_range(&mut self, var1: i32, var2: i32);

    // Java: abstract IDoubleList subList(int var1, int var2)
    fn subList(&self, var1: i32, var2: i32) -> Box<dyn IDoubleList>;

    // Java: abstract double[] toArray()
    fn toArray(&self) -> Vec<f64>;

    // Java: abstract double[] toArray(int var1, int var2)
    fn toArray_range(&self, var1: i32, var2: i32) -> Vec<f64>;

    // Java: abstract double[] toArray(double[] var1)
    fn toArray_dest(&self, var1: &mut [f64]) -> Vec<f64>;

    // Java: abstract double[] toArray(double[] var1, int var2, int var3)
    fn toArray_dest_range(&self, var1: &mut [f64], var2: i32, var3: i32) -> Vec<f64>;

    // Java: abstract double[] toArray(double[] var1, int var2, int var3, int var4)
    fn toArray_dest_range_offset(&self, var1: &mut [f64], var2: i32, var3: i32, var4: i32) -> Vec<f64>;

    // Java: abstract boolean forEach(IDoubleProcedure var1)
    fn forEach(&self, var1: &mut dyn IDoubleProcedure) -> bool;

    // Java: abstract void sort()
    fn sort(&mut self);

    // Java: abstract void sort(int var1, int var2)
    fn sort_range(&mut self, var1: i32, var2: i32);

    // Java: abstract void fill(double var1)
    fn fill(&mut self, var1: f64);

    // Java: abstract void fill(int var1, int var2, double var3)
    fn fill_range(&mut self, var1: i32, var2: i32, var3: f64);

    // Java: abstract int binarySearch(double var1)
    fn binarySearch(&self, var1: f64) -> i32;

    // Java: abstract int binarySearch(double var1, int var3, int var4)
    fn binarySearch_range(&self, var1: f64, var3: i32, var4: i32) -> i32;

    // Java: abstract int indexOf(double var1)
    fn indexOf(&self, var1: f64) -> i32;

    // Java: abstract int indexOf(int var1, double var2)
    fn indexOf_from(&self, var1: i32, var2: f64) -> i32;

    // Java: abstract int lastIndexOf(double var1)
    fn lastIndexOf(&self, var1: f64) -> i32;

    // Java: abstract int lastIndexOf(int var1, double var2)
    fn lastIndexOf_from(&self, var1: i32, var2: f64) -> i32;

    // Java: abstract boolean contains(double var1)
    fn contains(&self, var1: f64) -> bool;

    // Java: abstract IDoubleIterator iterator()
    fn iterator(&self) -> Box<dyn IDoubleIterator>;
}
