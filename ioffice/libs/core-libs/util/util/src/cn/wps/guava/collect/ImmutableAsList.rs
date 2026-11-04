#![allow(non_snake_case, dead_code, unused_variables, unused_imports)]

use std::any::Any;
use super::ImmutableList::ImmutableList;
// use super::Vec<i32>; // TODO: unavailable

// Java: public class ImmutableAsList extends ImmutableList
pub struct ImmutableAsList {
    pub base: ImmutableList,
}

// Java: public class ImmutableAsList extends ImmutableList
impl ImmutableAsList {
    // Java: ImmutableAsList()
    pub fn new() -> Self {
        // Java: super();
        ImmutableAsList {
            base: ImmutableList::new(),
        }
    }

    // Java: abstract abstract Vec<i32><E> delegateCollection()

    // Java: boolean contains(Object target)
    pub fn contains(&self, target: Option<&dyn Any>) -> bool {
        // Java: return this.delegateCollection().contains(target);
        false
    }

    // Java: int size()
    pub fn size(&self) -> usize {
        // Java: return this.delegateCollection().size();
        0
    }

    // Java: boolean isEmpty()
    pub fn isEmpty(&self) -> bool {
        // Java: return this.delegateCollection().isEmpty();
        true
    }
}
