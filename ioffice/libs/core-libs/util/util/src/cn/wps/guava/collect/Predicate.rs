#![allow(non_snake_case, dead_code, unused_variables)]

use std::any::Any;

// Java: public interface Predicate
pub trait Predicate {
    // Java: abstract boolean apply(T var1)
    fn apply(&self, var1: &dyn Any) -> bool;

    // Java: abstract boolean equals(Object var1)
    fn equals(&self, var1: &dyn Any) -> bool;
}
