#![allow(non_snake_case, dead_code, unused_variables)]

use std::any::Any;
use std::fmt;

// Java: public class CollectPreconditions
pub struct CollectPreconditions;

// Java: public class CollectPreconditions
impl CollectPreconditions {
    // Java: CollectPreconditions()
    // Java: super();

    // Java: static void checkEntryNotNull(Object key, Object value)
    pub fn checkEntryNotNull(key: &dyn Any, value: &dyn Any) {
        // Java: if (key == null)
        // Note: In Rust, references are never null, so this is a no-op
        // {
            // Java: throw new NullPointerException("null key in entry: null=" + value);
        // }
        // Java: else if (value == null)
        // {
            // Java: throw new NullPointerException("null value in entry: " + key + "=null");
        // }
    }

    // Java: static int checkNonnegative(int value, String name)
    pub fn checkNonnegative(value: i32, name: &str) -> i32 {
        // Java: if (value < 0)
        if value < 0
        {
            // Java: throw new IllegalArgumentException(name + " cannot be negative but was: " + value);
            panic!("{} cannot be negative but was: {}", name, value);
        }
        // Java: else
        {
            // Java: return value;
            value
        }
    }

    // Java: static void checkRemove(boolean canRemove)
    pub fn checkRemove(canRemove: bool) {
        // Java: Preconditions.checkState(canRemove, "no calls to next() since the last call to remove()");
        assert!(canRemove, "no calls to next() since the last call to remove()");
    }
}
