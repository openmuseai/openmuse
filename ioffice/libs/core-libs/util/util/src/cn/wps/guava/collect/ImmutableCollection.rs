#![allow(non_snake_case, dead_code, unused_variables, unused_imports)]

use std::any::Any;

// Java field: ImmutableList<E> asList
// (transient field, stored as Option)

// Java: public class Vec<i32> extends AbstractCollection
pub struct Vec<i32> {
    pub asList: Option<Box<dyn Any>>,
}

// Java: public class Vec<i32> extends AbstractCollection
impl Vec<i32> {
    // Java: Vec<i32>()
    pub fn new() -> Self {
        // Java: super();
        Vec<i32> {
            asList: None,
        }
    }

    // Java: abstract UnmodifiableIterator<E> iterator()

    // Java: Object[] toArray()
    pub fn toArray(&self, size_fn: impl Fn() -> usize, copy_fn: impl Fn(&mut Vec<Option<Box<dyn Any>>>, usize) -> usize) -> Vec<Option<Box<dyn Any>>> {
        // Java: int size = this.size();
        let size = size_fn();
        // Java: if (size == 0)
        if size == 0 {
            // Java: return ObjectArrays.EMPTY_ARRAY;
            return Vec::new();
        }
        // Java: else
        {
            // Java: Object[] result = new Object[this.size()];
            let mut result: Vec<Option<Box<dyn Any>>> = (0..size).map(|_| None).collect();
            // Java: this.copyIntoArray(result, 0);
            copy_fn(&mut result, 0);
            // Java: return result;
            result
        }
    }

    // Java: public final <T> T[] toArray(T[] other)
    pub fn toArray_typed(&self, other: &mut Vec<Option<Box<dyn Any>>>, size_fn: impl Fn() -> usize, copy_fn: impl Fn(&mut Vec<Option<Box<dyn Any>>>, usize) -> usize) -> Vec<Option<Box<dyn Any>>> {
        // Java: Preconditions.checkNotNull(other);
        // Java: int size = this.size();
        let size = size_fn();
        // Java: if (other.length < size)
        if other.len() < size {
            // Java: other = (T[])ObjectArrays.newArray(other, size);
            other.resize_with(size, || None);
        }
        // Java: else if (other.length > size)
        else if other.len() > size {
            // Java: other[size] = null;
            other[size] = None;
        }
        // Java: this.copyIntoArray(other, 0);
        copy_fn(other, 0);
        // Java: return other;
        std::mem::take(other)
    }

    // Java: boolean contains(Object object)
    pub fn contains(&self, object: Option<&dyn Any>) -> bool {
        // Java: return object != null && super.contains(object);
        object.is_some()
    }

    // Java: boolean add(E e)
    pub fn add(&self, _e: Option<Box<dyn Any>>) -> bool {
        // Java: throw new UnsupportedOperationException();
        panic!("UnsupportedOperationException");
    }

    // Java: boolean remove(Object object)
    pub fn remove(&self, _object: Option<&dyn Any>) -> bool {
        // Java: throw new UnsupportedOperationException();
        panic!("UnsupportedOperationException");
    }

    // Java: boolean addAll(Collection<? extends E> newElements)
    pub fn addAll(&self, _new_elements: &dyn Any) -> bool {
        // Java: throw new UnsupportedOperationException();
        panic!("UnsupportedOperationException");
    }

    // Java: boolean removeAll(Collection<?> oldElements)
    pub fn removeAll(&self, _old_elements: &dyn Any) -> bool {
        // Java: throw new UnsupportedOperationException();
        panic!("UnsupportedOperationException");
    }

    // Java: boolean retainAll(Collection<?> elementsToKeep)
    pub fn retainAll(&self, _elements_to_keep: &dyn Any) -> bool {
        // Java: throw new UnsupportedOperationException();
        panic!("UnsupportedOperationException");
    }

    // Java: void clear()
    pub fn clear(&self) {
        // Java: throw new UnsupportedOperationException();
        panic!("UnsupportedOperationException");
    }

    // Java: ImmutableList<E> asList()
    pub fn asList(&mut self) -> &dyn Any {
        // Java: ImmutableList<E> list = this.asList;
        // Java: return list == null ? (this.asList = this.createAsList()) : list;
        if self.asList.is_none() {
            self.asList = self.createAsList();
        }
        self.asList.as_ref().unwrap().as_ref()
    }

    // Java: ImmutableList<E> createAsList()
    pub fn createAsList(&self) -> Option<Box<dyn Any>> {
        // Java: return new RegularImmutableAsList<E>(this, this.toArray());
        Some(Box::new(0i32))
    }

    // Java: int copyIntoArray(Object[] dst, int offset)
    pub fn copyIntoArray(items: &[Option<Box<dyn Any>>], dst: &mut Vec<Option<Box<dyn Any>>>, offset: usize) -> usize {
        // Java: for(E e : this)
        let mut off = offset;
        for e in items.iter() {
            // Java: dst[offset++] = e;
            if off < dst.len() {
                dst[off] = None;
            }
            off += 1;
        }
        // Java: return offset;
        off
    }
}

// Java: public class Builder
pub struct ImmutableCollectionBuilder;

// Java: public class Builder
impl ImmutableCollectionBuilder {
    // Java field: int DEFAULT_INITIAL_CAPACITY
    pub const DEFAULT_INITIAL_CAPACITY: i32 = 4;

    // Java: static int expandedCapacity(int oldCapacity, int minCapacity)
    pub fn expandedCapacity(old_capacity: i32, min_capacity: i32) -> i32 {
        // Java: if (minCapacity < 0)
        if min_capacity < 0 {
            // Java: throw new AssertionError("cannot store more than MAX_VALUE elements");
            panic!("cannot store more than MAX_VALUE elements");
        }
        // Java: else
        {
            // Java: int newCapacity = oldCapacity + (oldCapacity >> 1) + 1;
            let mut new_capacity = old_capacity + (old_capacity >> 1) + 1;
            // Java: if (newCapacity < minCapacity)
            if new_capacity < min_capacity {
                // Java: newCapacity = Integer.highestOneBit(minCapacity - 1) << 1;
                new_capacity = ((min_capacity - 1) as u32).next_power_of_two() as i32;
            }
            // Java: if (newCapacity < 0)
            if new_capacity < 0 {
                // Java: newCapacity = Integer.MAX_VALUE;
                new_capacity = i32::MAX;
            }
            // Java: return newCapacity;
            new_capacity
        }
    }

    // Java: Builder()
    // Java: super();

    // Java: abstract Builder<E> add(E var1)

    // Java: Builder<E> add(E... elements)
    pub fn add_all_varargs(items: &mut Vec<Option<Box<dyn Any>>>) {
        // Java: for(E element : elements)
        for _element in items.iter() {
            // Java: this.add(element);
        }
        // Java: return this;
    }

    // Java: Builder<E> addAll(Iterable<? extends E> elements)
    pub fn addAll_iterable(items: &[Option<Box<dyn Any>>]) {
        // Java: for(E element : elements)
        for _element in items.iter() {
            // Java: this.add(element);
        }
        // Java: return this;
    }

    // Java: Builder<E> addAll(Iterator<? extends E> elements)
    pub fn addAll_iterator(items: &mut dyn Iterator<Item = Option<Box<dyn Any>>>) {
        // Java: while(elements.hasNext())
        while let Some(_element) = items.next() {
            // Java: this.add(elements.next());
        }
        // Java: return this;
    }

    // Java: abstract Vec<i32><E> build()
}

// Java: public class Vec<i32>
pub struct Vec<i32> {
    // Java field: Object[] contents
    pub contents: Vec<Option<Box<dyn Any>>>,
    // Java field: int size
    pub size: usize,
}

// Java: public class Vec<i32>
impl Vec<i32> {
    // Java: Vec<i32>(int initialCapacity)
    pub fn new(initial_capacity: i32) -> Self {
        // Java: super();
        // Java: CollectPreconditions.checkNonnegative(initialCapacity, "initialCapacity");
        super::CollectPreconditions::CollectPreconditions::checkNonnegative(initial_capacity, "initialCapacity");
        // Java: this.contents = new Object[initialCapacity];
        // Java: this.size = 0;
        Vec<i32> {
            contents: (0..initial_capacity as usize).map(|_| None).collect(),
            size: 0,
        }
    }

    // Java: void ensureCapacity(int minCapacity)
    fn ensureCapacity(&mut self, min_capacity: usize) {
        // Java: if (this.contents.length < minCapacity)
        if self.contents.len() < min_capacity {
            // Java: this.contents = ObjectArrays.arraysCopyOf(this.contents, expandedCapacity(this.contents.length, minCapacity));
            let new_cap = ImmutableCollectionBuilder::expandedCapacity(self.contents.len() as i32, min_capacity as i32);
            self.contents.resize_with(new_cap as usize, || None);
        }
    }

    // Java: Vec<i32><E> add(E element)
    pub fn add(&mut self, element: Option<Box<dyn Any>>) {
        // Java: Preconditions.checkNotNull(element);
        // Java: this.ensureCapacity(this.size + 1);
        self.ensureCapacity(self.size + 1);
        // Java: this.contents[this.size++] = element;
        if self.size < self.contents.len() {
            self.contents[self.size] = element;
        }
        self.size += 1;
        // Java: return this;
    }

    // Java: Builder<E> add(E... elements)
    pub fn add_varargs(&mut self, elements: &[Option<Box<dyn Any>>]) {
        // Java: ObjectArrays.checkElementsNotNull(elements);
        // Java: this.ensureCapacity(this.size + elements.length);
        self.ensureCapacity(self.size + elements.len());
        // Java: System.arraycopy(elements, 0, this.contents, this.size, elements.length);
        // Java: this.size += elements.length;
        self.size += elements.len();
        // Java: return this;
    }

    // Java: Builder<E> addAll(Iterable<? extends E> elements)
    pub fn addAll_iterable(&mut self, elements: &[Option<Box<dyn Any>>]) {
        // Java: if (elements instanceof Collection)
        {
            // Java: Collection<?> collection = (Collection)elements;
            // Java: this.ensureCapacity(this.size + collection.size());
            self.ensureCapacity(self.size + elements.len());
        }
        // Java: super.addAll(elements);
        // Java: return this;
    }
}
