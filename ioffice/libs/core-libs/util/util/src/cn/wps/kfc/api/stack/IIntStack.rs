#![allow(non_snake_case, dead_code)]

// Java: public interface IIntStack
pub trait IIntStack {
    // Java: abstract int getNoEntryValue()
    fn getNoEntryValue(&self) -> i32;

    // Java: abstract void push(int var1)
    fn push(&mut self, var1: i32);

    // Java: abstract int pop()
    fn pop(&mut self) -> i32;

    // Java: abstract int peek()
    fn peek(&self) -> i32;

    // Java: abstract int size()
    fn size(&self) -> i32;

    // Java: abstract void clear()
    fn clear(&mut self);

    // Java: abstract int[] toArray()
    fn toArray(&self) -> Vec<i32>;

    // Java: abstract void toArray(int[] var1)
    fn toArray_into(&self, var1: &mut [i32]);
}
