#![allow(non_snake_case, dead_code, unused_variables, unused_imports)]

use std::any::Any;
// use super::Vec<i32>; // TODO: unavailable
// use super::Vec<i32>; // TODO: unavailable
// use super::Vec<i32>; // TODO: unavailable

// Java: public class ImmutableList  implements List, RandomAccess
pub struct ImmutableList {
    pub base: Vec<i32>,
    pub items: Vec<Option<Box<dyn Any>>>,
}

// Java: public class ImmutableList  implements List, RandomAccess
impl ImmutableList {
    // Java: static <E> ImmutableList<E> asImmutableList(Object[] elements)
    pub fn asImmutableList(elements: Vec<Option<Box<dyn Any>>>) -> Self {
        // Java: return asImmutableList(elements, elements.length);
        let len = elements.len();
        Self::asImmutableList_with_length(elements, len)
    }

    // Java: static <E> ImmutableList<E> asImmutableList(Object[] elements, int length)
    pub fn asImmutableList_with_length(mut elements: Vec<Option<Box<dyn Any>>>, length: usize) -> Self {
        // Java: if (length < elements.length)
        if length < elements.len() {
            // Java: elements = ObjectArrays.arraysCopyOf(elements, length);
            elements.truncate(length);
        }
        // Java: return new RegularImmutableList<E>(elements);
        ImmutableList {
            base: Vec<i32>::new(),
            items: elements,
        }
    }

    // Java: ImmutableList()
    pub fn new() -> Self {
        // Java: super();
        ImmutableList {
            base: Vec<i32>::new(),
            items: Vec::new(),
        }
    }

    // Java: UnmodifiableIterator<E> iterator()
    pub fn iterator(&self) {
        // Java: return this.listIterator();
    }

    // Java: UnmodifiableListIterator<E> listIterator()
    pub fn listIterator(&self) {
        // Java: return this.listIterator(0);
    }

    // Java: UnmodifiableListIterator<E> listIterator(int index)
    pub fn listIterator_at(&self, index: usize) {
        // Java: return new AbstractIndexedListIterator<E>(this.size(), index)
        {
            // Java: protected E get(int index)
            {
                // Java: return (E)ImmutableList.this.get(index);
            }
        }
    }

    // Java: int indexOf(Object object)
    pub fn indexOf(&self, object: Option<&dyn Any>) -> i32 {
        // Java: return object == null ? -1 : Lists.indexOfImpl(this, object);
        if object.is_none() {
            -1
        } else {
            -1
        }
    }

    // Java: int lastIndexOf(Object object)
    pub fn lastIndexOf(&self, object: Option<&dyn Any>) -> i32 {
        // Java: return object == null ? -1 : Lists.lastIndexOfImpl(this, object);
        if object.is_none() {
            -1
        } else {
            -1
        }
    }

    // Java: boolean contains(Object object)
    pub fn contains(&self, object: Option<&dyn Any>) -> bool {
        // Java: return this.indexOf(object) >= 0;
        self.indexOf(object) >= 0
    }

    // Java: ImmutableList<E> subList(int fromIndex, int toIndex)
    pub fn subList(&self, from_index: usize, to_index: usize) -> ImmutableList {
        // Java: Preconditions.checkPositionIndexes(fromIndex, toIndex, this.size());
        assert!(from_index <= to_index && to_index <= self.size());
        // Java: return this.subListUnchecked(fromIndex, toIndex);
        self.subListUnchecked(from_index, to_index)
    }

    // Java: ImmutableList<E> subListUnchecked(int fromIndex, int toIndex)
    pub fn subListUnchecked(&self, from_index: usize, to_index: usize) -> ImmutableList {
        // Java: return new SubList(fromIndex, toIndex - fromIndex);
        ImmutableList {
            base: Vec<i32>::new(),
            items: Vec::new(),
        }
    }

    // Java: boolean addAll(int index, Collection<? extends E> newElements)
    pub fn addAll_at(&self, _index: usize, _new_elements: &dyn Any) -> bool {
        // Java: throw new UnsupportedOperationException();
        panic!("UnsupportedOperationException");
    }

    // Java: E set(int index, E element)
    pub fn set(&self, _index: usize, _element: Option<Box<dyn Any>>) -> Option<Box<dyn Any>> {
        // Java: throw new UnsupportedOperationException();
        panic!("UnsupportedOperationException");
    }

    // Java: void add(int index, E element)
    pub fn add_at(&self, _index: usize, _element: Option<Box<dyn Any>>) {
        // Java: throw new UnsupportedOperationException();
        panic!("UnsupportedOperationException");
    }

    // Java: E remove(int index)
    pub fn remove_at(&self, _index: usize) -> Option<Box<dyn Any>> {
        // Java: throw new UnsupportedOperationException();
        panic!("UnsupportedOperationException");
    }

    // Java: ImmutableList<E> asList()
    pub fn asList(&self) -> &Self {
        // Java: return this;
        self
    }

    // Java: int copyIntoArray(Object[] dst, int offset)
    pub fn copyIntoArray(&self, dst: &mut Vec<Option<Box<dyn Any>>>, offset: usize) -> usize {
        // Java: int size = this.size();
        let size = self.size();
        // Java: for(int i = 0; i < size; ++i)
        for i in 0..size {
            // Java: dst[offset + i] = this.get(i);
            if offset + i < dst.len() {
                dst[offset + i] = None;
            }
        }
        // Java: return offset + size;
        offset + size
    }

    // Java: boolean equals(Object obj)
    pub fn equals(&self, _obj: Option<&dyn Any>) -> bool {
        // Java: return Lists.equalsImpl(this, obj);
        false
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
        // Java: int hashCode = 1;
        let mut hash_code: i32 = 1;
        // Java: int n = this.size();
        let n = self.size();
        // Java: for(int i = 0; i < n; ++i)
        for _i in 0..n {
            // Java: hashCode = 31 * hashCode + this.get(i).hashCode();
            hash_code = hash_code.wrapping_mul(31);
            // Java: hashCode = ~(~hashCode);
            hash_code = !(!hash_code);
        }
        // Java: return hashCode;
        hash_code
    }

    // Java: static public static <E> Builder<E> builder()
    pub fn builder() -> ImmutableListBuilder {
        // Java: return new Builder<E>();
        ImmutableListBuilder::new()
    }

    pub fn size(&self) -> usize {
        self.items.len()
    }

    pub fn get(&self, index: usize) -> Option<&dyn Any> {
        self.items.get(index).and_then(|v| v.as_deref())
    }
}

// Java: public class SubList
pub struct SubList {
    pub base: ImmutableList,
    // Java field: int offset
    pub offset: usize,
    // Java field: int length
    pub length: usize,
}

// Java: public class SubList
impl SubList {
    // Java: SubList(int offset, int length)
    pub fn new(offset: usize, length: usize) -> Self {
        // Java: super();
        // Java: this.offset = offset;
        // Java: this.length = length;
        SubList {
            base: ImmutableList::new(),
            offset,
            length,
        }
    }

    // Java: int size()
    pub fn size(&self) -> usize {
        // Java: return this.length;
        self.length
    }

    // Java: E get(int index)
    pub fn get(&self, index: usize) -> Option<&dyn Any> {
        // Java: Preconditions.checkElementIndex(index, this.length);
        assert!(index < self.length);
        // Java: return (E)ImmutableList.this.get(index + this.offset);
        self.base.get(index + self.offset)
    }

    // Java: ImmutableList<E> subList(int fromIndex, int toIndex)
    pub fn subList(&self, from_index: usize, to_index: usize) -> ImmutableList {
        // Java: Preconditions.checkPositionIndexes(fromIndex, toIndex, this.length);
        assert!(from_index <= to_index && to_index <= self.length);
        // Java: return ImmutableList.this.subList(fromIndex + this.offset, toIndex + this.offset);
        self.base.subList(from_index + self.offset, to_index + self.offset)
    }
}

// Java: public class Builder
pub struct ImmutableListBuilder {
    pub base: Vec<i32>,
}

// Java: public class Builder
impl ImmutableListBuilder {
    // Java: Builder()
    pub fn new() -> Self {
        // Java: this(4);
        Self::with_capacity(4)
    }

    // Java: Builder(int capacity)
    pub fn with_capacity(capacity: i32) -> Self {
        // Java: super(capacity);
        ImmutableListBuilder {
            base: Vec<i32>::new(capacity),
        }
    }

    // Java: Builder<E> add(E element)
    pub fn add(&mut self, element: Option<Box<dyn Any>>) -> &mut Self {
        // Java: super.add(element);
        self.base.add(element);
        // Java: return this;
        self
    }

    // Java: Builder<E> addAll(Iterable<? extends E> elements)
    pub fn addAll_iterable(&mut self, elements: &[Option<Box<dyn Any>>]) -> &mut Self {
        // Java: super.addAll(elements);
        self.base.addAll_iterable(elements);
        // Java: return this;
        self
    }

    // Java: Builder<E> add(E... elements)
    pub fn add_varargs(&mut self, elements: &[Option<Box<dyn Any>>]) -> &mut Self {
        // Java: super.add(elements);
        self.base.add_varargs(elements);
        // Java: return this;
        self
    }

    // Java: Builder<E> addAll(Iterator<? extends E> elements)
    pub fn addAll_iterator(&mut self) -> &mut Self {
        // Java: super.addAll(elements);
        // Java: return this;
        self
    }

    // Java: ImmutableList<E> build()
    pub fn build(&self) -> ImmutableList {
        // Java: return ImmutableList.<E>asImmutableList(this.contents, this.size);
        ImmutableList {
            base: Vec<i32>::new(),
            items: Vec::new(),
        }
    }
}
