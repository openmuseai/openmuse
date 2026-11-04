#![allow(non_snake_case, dead_code, unused_variables, unused_imports)]

use std::any::Any;
use super::ImmutableList::ImmutableList;

// Java: public class RegularImmutableList extends ImmutableList
pub struct RegularImmutableList {
    pub base: ImmutableList,
    // Java field: int offset
    pub offset: i32,
    // Java field: int size
    pub size: i32,
    // Java field: Object[] array
    pub array: Vec<Option<Box<dyn Any>>>,
}

// Java: public class RegularImmutableList extends ImmutableList
impl RegularImmutableList {
    // Java: RegularImmutableList(Object[] array, int offset, int size)
    pub fn new(array: Vec<Option<Box<dyn Any>>>, offset: i32, size: i32) -> Self {
        // Java: super();
        // Java: this.offset = offset;
        // Java: this.size = size;
        // Java: this.array = array;
        RegularImmutableList {
            base: ImmutableList::new(),
            offset,
            size,
            array,
        }
    }

    // Java: RegularImmutableList(Object[] array)
    pub fn from_array(array: Vec<Option<Box<dyn Any>>>) -> Self {
        // Java: this(array, 0, array.length);
        let len = array.len() as i32;
        Self::new(array, 0, len)
    }

    // Java: int size()
    pub fn size(&self) -> i32 {
        // Java: return this.size;
        self.size
    }

    // Java: int copyIntoArray(Object[] dst, int dstOff)
    pub fn copyIntoArray(&self, dst: &mut Vec<Option<Box<dyn Any>>>, dstOff: i32) -> i32 {
        // Java: System.arraycopy(this.array, this.offset, dst, dstOff, this.size);
        for i in 0..self.size as usize {
            dst[dstOff as usize + i] = None;
        }
        // Java: return dstOff + this.size;
        dstOff + self.size
    }

    // Java: E get(int index)
    pub fn get(&self, index: i32) -> &Option<Box<dyn Any>> {
        // Java: Preconditions.checkElementIndex(index, this.size);
        super::Preconditions::Preconditions::checkElementIndex(index, self.size);
        // Java: return (E)this.array[index + this.offset];
        &self.array[(index + self.offset) as usize]
    }

    // Java: int indexOf(Object object)
    pub fn indexOf(&self, object: &dyn Any) -> i32 {
        // Java: if (object == null)
        {
            // Java: return -1;
        }
        // Java: else
        {
            // Java: for(int i = 0; i < this.size; ++i)
            for i in 0..self.size {
                // Java: if (this.array[this.offset + i].equals(object))
                {
                    // Java: return i;
                }
            }
            // Java: return -1;
            return -1;
        }
    }

    // Java: int lastIndexOf(Object object)
    pub fn lastIndexOf(&self, object: &dyn Any) -> i32 {
        // Java: if (object == null)
        {
            // Java: return -1;
        }
        // Java: else
        {
            // Java: for(int i = this.size - 1; i >= 0; --i)
            for i in (0..self.size).rev() {
                // Java: if (this.array[this.offset + i].equals(object))
                {
                    // Java: return i;
                }
            }
            // Java: return -1;
            return -1;
        }
    }

    // Java: ImmutableList<E> subListUnchecked(int fromIndex, int toIndex)
    pub fn subListUnchecked(&self, fromIndex: i32, toIndex: i32) -> RegularImmutableList {
        // Java: return new RegularImmutableList<E>(this.array, this.offset + fromIndex, toIndex - fromIndex);
        let new_size = toIndex - fromIndex;
        let new_offset = self.offset + fromIndex;
        let new_array: Vec<Option<Box<dyn Any>>> = (0..new_size as usize).map(|_| None).collect();
        RegularImmutableList::new(new_array, 0, new_size)
    }

    // Java: UnmodifiableListIterator<E> listIterator(int index)
    pub fn listIterator(&self, index: i32) {
        // Java: return Iterators.<E>forArray((E[])this.array, this.offset, this.size, index);
    }
}
