#![allow(non_snake_case, dead_code, unused_variables)]

use std::any::Any;

// Java: public interface Function
pub trait Function {
    // Java: abstract T apply(F var1)
    fn apply(&self, var1: &dyn Any) -> Option<Box<dyn Any>>;

    // Java: abstract boolean equals(Object var1)
    fn equals(&self, var1: &dyn Any) -> bool;
}
