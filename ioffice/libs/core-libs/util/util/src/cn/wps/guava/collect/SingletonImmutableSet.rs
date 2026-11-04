#![allow(non_snake_case, dead_code, unused_variables, unused_imports)]

use std::any::Any;
use super::ImmutableSet::ImmutableSet;
use super::ImmutableCollection::ImmutableCollection;

// Java: import java.util.Set

// Java: public class SingletonImmutableSet extends ImmutableSet
pub struct SingletonImmutableSet {
    pub base: ImmutableSet,
    // Java field: E element
    pub element: Box<dyn Any>,
    // Java field: int cachedHashCode
    pub cachedHashCode: i32,
}

// Java: public class SingletonImmutableSet extends ImmutableSet
impl SingletonImmutableSet {
    // Java: SingletonImmutableSet(E element)
    pub fn new(element: Box<dyn Any>) -> Self {
        // Java: super();
        // Java: if (element == null)
        // Java: throw new NullPointerException();
        // Java: else
        // Java: this.element = element;
        SingletonImmutableSet {
            base: ImmutableSet::new(),
            element,
            cachedHashCode: 0,
        }
    }

    // Java: SingletonImmutableSet(E element, int hashCode)
    pub fn with_hash(element: Box<dyn Any>, hashCode: i32) -> Self {
        // Java: super();
        // Java: this.element = element;
        // Java: this.cachedHashCode = hashCode;
        SingletonImmutableSet {
            base: ImmutableSet::new(),
            element,
            cachedHashCode: hashCode,
        }
    }

    // Java: int size()
    pub fn size(&self) -> i32 {
        // Java: return 1;
        1
    }

    // Java: boolean isEmpty()
    pub fn isEmpty(&self) -> bool {
        // Java: return false;
        false
    }

    // Java: boolean contains(Object target)
    pub fn contains(&self, _target: &dyn Any) -> bool {
        // Java: return this.element.equals(target);
        false
    }

    // Java: UnmodifiableIterator<E> iterator()
    pub fn iterator(&self) {
        // Java: return Iterators.<E>singletonIterator(this.element);
    }

    // Java: int copyIntoArray(Object[] dst, int offset)
    pub fn copyIntoArray(&self, dst: &mut Vec<Option<Box<dyn Any>>>, offset: i32) -> i32 {
        // Java: dst[offset] = this.element;
        dst[offset as usize] = None;
        // Java: return offset + 1;
        offset + 1
    }

    // Java: boolean equals(Object object)
    pub fn equals(&self, _object: &dyn Any) -> bool {
        // Java: if (object == this)
        {
            // Java: return true;
        }
        // Java: else if (!(object instanceof Set))
        {
            // Java: return false;
        }
        // Java: else
        {
            // Java: Set<?> that = (Set)object;
            // Java: return that.size() == 1 && this.element.equals(that.iterator().next());
            false
        }
    }

    // Java: int hashCode()
    pub fn hashCode(&mut self) -> i32 {
        // Java: int code = this.cachedHashCode;
        let mut code = self.cachedHashCode;
        // Java: if (code == 0)
        if code == 0 {
            // Java: this.cachedHashCode = code = this.element.hashCode();
            code = 0;
            self.cachedHashCode = code;
        }
        // Java: return code;
        code
    }

    // Java: boolean isHashCodeFast()
    pub fn isHashCodeFast(&self) -> bool {
        // Java: return this.cachedHashCode != 0;
        self.cachedHashCode != 0
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
        // Java: String elementToString = this.element.toString();
        let elementToString = String::from("element");
        // Java: return (new StringBuilder(elementToString.length() + 2)).append('[').append(elementToString).append(']').toString();
        format!("[{}]", elementToString)
    }
}
