#![allow(non_snake_case, dead_code)]

// Java: public interface IDoubleStack
pub trait IDoubleStack {
    // Java: abstract double getNoEntryValue()
    fn getNoEntryValue(&self) -> f64;

    // Java: abstract void push(double var1)
    fn push(&mut self, var1: f64);

    // Java: abstract double pop()
    fn pop(&mut self) -> f64;

    // Java: abstract double peek()
    fn peek(&self) -> f64;

    // Java: abstract int size()
    fn size(&self) -> i32;

    // Java: abstract void clear()
    fn clear(&mut self);

    // Java: abstract double[] toArray()
    fn toArray(&self) -> Vec<f64>;

    // Java: abstract void toArray(double[] var1)
    fn toArray_into(&self, var1: &mut [f64]);
}
