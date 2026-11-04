#![allow(unused_imports, dead_code, non_snake_case, unused_variables, unused_mut, unused_assignments)]

// Java: public interface NeighbourhoodSizeFunction
pub trait NeighbourhoodSizeFunction {
    // Java: abstract int value(long numCall)
    fn value(&self, numCall: i64) -> i32;

}
