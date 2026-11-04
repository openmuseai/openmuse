#![allow(non_snake_case, dead_code)]

// Java: public interface ILongStack
pub trait ILongStack {
    // Java: abstract long getNoEntryValue()
    fn getNoEntryValue(&self) -> i64;

    // Java: abstract void push(long var1)
    fn push(&mut self, var1: i64);

    // Java: abstract long pop()
    fn pop(&mut self) -> i64;

    // Java: abstract long peek()
    fn peek(&self) -> i64;

    // Java: abstract int size()
    fn size(&self) -> i32;

    // Java: abstract void clear()
    fn clear(&mut self);

    // Java: abstract long[] toArray()
    fn toArray(&self) -> Vec<i64>;

    // Java: abstract void toArray(long[] var1)
    fn toArray_into(&self, var1: &mut [i64]);
}
