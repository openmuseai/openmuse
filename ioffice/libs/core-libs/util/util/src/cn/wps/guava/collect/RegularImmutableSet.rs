#![allow(non_snake_case, dead_code, unused_variables, unused_imports)]

use std::any::Any;
use super::ImmutableSet::ImmutableSet;
use super::ImmutableCollection::ImmutableCollection;

// Java: public class RegularImmutableSet extends ImmutableSet
pub struct RegularImmutableSet {
    pub base: ImmutableSet,
    // Java field: Object[] elements
    pub elements: Vec<Option<Box<dyn Any>>>,
    // Java field: Object[] table
    pub table: Vec<Option<Box<dyn Any>>>,
    // Java field: int mask
    pub mask: i32,
    // Java field: int hashCode
    pub hashCode: i32,
}

// Java: public class RegularImmutableSet extends ImmutableSet
impl RegularImmutableSet {
    // Java: RegularImmutableSet(Object[] elements, int hashCode, Object[] table, int mask)
    pub fn new(elements: Vec<Option<Box<dyn Any>>>, hashCode: i32, table: Vec<Option<Box<dyn Any>>>, mask: i32) -> Self {
        // Java: super();
        // Java: this.elements = elements;
        // Java: this.table = table;
        // Java: this.mask = mask;
        // Java: this.hashCode = hashCode;
        RegularImmutableSet {
            base: ImmutableSet::new(),
            elements,
            table,
            mask,
            hashCode,
        }
    }

    // Java: boolean contains(Object target)
    pub fn contains(&self, _target: &dyn Any) -> bool {
        // Java: if (target == null)
        {
            // Java: return false;
        }
        // Java: else
        {
            // Java: int i = Hashing.smear(target.hashCode());
            let mut i: i32 = 0;
            // Java: while(true)
            loop {
                // Java: Object candidate = this.table[i & this.mask];
                let idx = (i & self.mask) as usize;
                if idx >= self.table.len() {
                    return false;
                }
                // Java: if (candidate == null)
                if self.table[idx].is_none() {
                    // Java: return false;
                    return false;
                }
                // Java: if (candidate.equals(target))
                {
                    // Java: return true;
                }
                // Java: ++i;
                i += 1;
            }
        }
    }

    // Java: int size()
    pub fn size(&self) -> i32 {
        // Java: return this.elements.length;
        self.elements.len() as i32
    }

    // Java: UnmodifiableIterator<E> iterator()
    pub fn iterator(&self) {
        // Java: return Iterators.<E>forArray((E[])this.elements);
    }

    // Java: int copyIntoArray(Object[] dst, int offset)
    pub fn copyIntoArray(&self, dst: &mut Vec<Option<Box<dyn Any>>>, offset: i32) -> i32 {
        // Java: System.arraycopy(this.elements, 0, dst, offset, this.elements.length);
        let len = self.elements.len();
        for i in 0..len {
            dst[offset as usize + i] = None;
        }
        // Java: return offset + this.elements.length;
        offset + len as i32
    }

    // Java: ImmutableList<E> createAsList()
    pub fn createAsList(&self) {
        // Java: return new RegularImmutableAsList<E>(this, this.elements);
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
        // Java: return this.hashCode;
        self.hashCode
    }

    // Java: boolean isHashCodeFast()
    pub fn isHashCodeFast(&self) -> bool {
        // Java: return true;
        true
    }
}
