#![allow(non_snake_case, dead_code, unused_variables, unused_imports)]

use std::any::Any;

// Java: public class Iterators
pub struct Iterators;

// Java: public class Iterators
impl Iterators {
    // Java: Iterators()
    // Java: super();

    // Java: static public static <T> UnmodifiableIterator<T> emptyIterator()
    pub fn emptyIterator() {
        // Java: return emptyListIterator();
    }

    // Java: static public static <T> UnmodifiableIterator<T> unmodifiableIterator(final Iterator<T> iterator)
    pub fn unmodifiableIterator(_iterator: &dyn Any) {
        // Java: Preconditions.checkNotNull(iterator);
        // Java: return iterator instanceof UnmodifiableIterator ? (UnmodifiableIterator)iterator : new UnmodifiableIterator<T>()
        {
            // Java: public boolean hasNext()
            {
                // Java: return iterator.hasNext();
            }
            // Java: public T next()
            {
                // Java: return (T)iterator.next();
            }
        }
    }

    // Java: static public static <T> UnmodifiableIterator<T> unmodifiableIterator(UnmodifiableIterator<T> iterator)
    pub fn unmodifiableIterator_from_unmodifiable(_iterator: &dyn Any) {
        // Java: return (UnmodifiableIterator)Preconditions.checkNotNull(iterator);
    }

    // Java: static int size(Iterator<?> iterator)
    pub fn size(iterator: &mut dyn Iterator<Item = Option<Box<dyn Any>>>) -> i32 {
        // Java: int count;
        let mut count = 0i32;
        // Java: for(count = 0; iterator.hasNext(); ++count)
        while let Some(_) = iterator.next() {
            // Java: iterator.next();
            count += 1;
        }
        // Java: return count;
        count
    }

    // Java: static boolean elementsEqual(Iterator<?> iterator1, Iterator<?> iterator2)
    pub fn elementsEqual(_iterator1: &dyn Any, _iterator2: &dyn Any) -> bool {
        // Java: while(true)
        {
            // Java: if (iterator1.hasNext())
            {
                // Java: if (!iterator2.hasNext())
                {
                    // Java: return false;
                }
                // Java: Object o1 = iterator1.next();
                // Java: Object o2 = iterator2.next();
                // Java: if (Objects.equal(o1, o2))
                {
                    // Java: continue;
                }
                // Java: return false;
            }
            // Java: return !iterator2.hasNext();
            false
        }
    }

    // Java: static public static <T> T getOnlyElement(Iterator<T> iterator)
    pub fn getOnlyElement(iterator: &mut dyn Iterator<Item = Option<Box<dyn Any>>>) -> Option<Box<dyn Any>> {
        // Java: T first = (T)iterator.next();
        let first = iterator.next().flatten();
        // Java: if (!iterator.hasNext())
        if iterator.next().is_none() {
            // Java: return first;
            return first;
        }
        // Java: else
        {
            // Java: StringBuilder sb = new StringBuilder();
            let mut sb = String::new();
            // Java: sb.append("expected one element but was: <" + first);
            sb.push_str("expected one element but was: <");
            // Java: for(int i = 0; i < 4 && iterator.hasNext(); ++i)
            for _i in 0..4 {
                // Java: sb.append(", " + iterator.next());
                if iterator.next().is_some() {
                    sb.push_str(", ...");
                }
            }
            // Java: if (iterator.hasNext())
            if iterator.next().is_some() {
                // Java: sb.append(", ...");
                sb.push_str(", ...");
            }
            // Java: sb.append('>');
            sb.push('>');
            // Java: throw new IllegalArgumentException(sb.toString());
            panic!("{}", sb);
        }
    }

    // Java: static public static <T> boolean addAll(Collection<T> addTo, Iterator<? extends T> iterator)
    pub fn addAll(_add_to: &mut Vec<Option<Box<dyn Any>>>, _iterator: &mut dyn Iterator<Item = Option<Box<dyn Any>>>) -> bool {
        // Java: Preconditions.checkNotNull(addTo);
        // Java: Preconditions.checkNotNull(iterator);
        // Java: boolean wasModified;
        let mut was_modified = false;
        // Java: for(wasModified = false; iterator.hasNext(); wasModified |= addTo.add(iterator.next()))
        for item in _iterator {
            _add_to.push(item);
            was_modified = true;
        }
        {
        }
        // Java: return wasModified;
        was_modified
    }

    // Java: static public static <T> T get(Iterator<T> iterator, int position)
    pub fn get(iterator: &mut dyn Iterator<Item = Option<Box<dyn Any>>>, position: i32) -> Option<Box<dyn Any>> {
        // Java: checkNonnegative(position);
        Self::checkNonnegative(position);
        // Java: int skipped = advance(iterator, position);
        let skipped = Self::advance(iterator, position);
        // Java: if (!iterator.hasNext())
        match iterator.next() {
            None => {
                // Java: throw new IndexOutOfBoundsException("position (" + position + ") must be less than the number of elements that remained (" + skipped + ")");
                panic!("position ({}) must be less than the number of elements that remained ({})", position, skipped);
            }
            // Java: else
            Some(val) => {
                // Java: return (T)iterator.next();
                val
            }
        }
    }

    // Java: static void checkNonnegative(int position)
    pub fn checkNonnegative(position: i32) {
        // Java: if (position < 0)
        if position < 0 {
            // Java: throw new IndexOutOfBoundsException("position (" + position + ") must not be negative");
            panic!("position ({}) must not be negative", position);
        }
    }

    // Java: static int advance(Iterator<?> iterator, int numberToAdvance)
    pub fn advance(iterator: &mut dyn Iterator<Item = Option<Box<dyn Any>>>, number_to_advance: i32) -> i32 {
        // Java: Preconditions.checkNotNull(iterator);
        // Java: Preconditions.checkArgument(numberToAdvance >= 0, "numberToAdvance must be nonnegative");
        assert!(number_to_advance >= 0, "numberToAdvance must be nonnegative");
        // Java: int i;
        let mut i = 0i32;
        // Java: for(i = 0; i < numberToAdvance && iterator.hasNext(); ++i)
        while i < number_to_advance {
            if iterator.next().is_none() { break; }
            // Java: iterator.next();
            i += 1;
        }
        // Java: return i;
        i
    }

    // Java: static public static <T> T getNext(Iterator<? extends T> iterator, T defaultValue)
    pub fn getNext(iterator: &mut dyn Iterator<Item = Option<Box<dyn Any>>>, default_value: Option<Box<dyn Any>>) -> Option<Box<dyn Any>> {
        // Java: return (T)(iterator.hasNext() ? iterator.next() : defaultValue);
        iterator.next().flatten().or(default_value)
    }

    // Java: static public static <T> T getLast(Iterator<T> iterator)
    pub fn getLast(iterator: &mut dyn Iterator<Item = Option<Box<dyn Any>>>) -> Option<Box<dyn Any>> {
        // Java: T current;
        let mut current: Option<Box<dyn Any>> = None;
        // Java: do
        loop {
            // Java: current = (T)iterator.next();
            match iterator.next() {
                Some(val) => current = val,
                None => break,
            }
        }
        // Java: return current;
        current
    }

    // Java: static public static <T> UnmodifiableIterator<T> forArray(T... array)
    pub fn forArray(_array: &[Option<Box<dyn Any>>]) {
        // Java: return forArray(array, 0, array.length, 0);
    }

    // Java: static public static <T> T getLast(Iterator<? extends T> iterator, T defaultValue)
    pub fn getLast_with_default(iterator: &mut dyn Iterator<Item = Option<Box<dyn Any>>>, default_value: Option<Box<dyn Any>>) -> Option<Box<dyn Any>> {
        // Java: return (T)(iterator.hasNext() ? getLast(iterator) : defaultValue);
        Self::getLast(iterator).or(default_value)
    }

    // Java: static <T> T pollNext(Iterator<T> iterator)
    pub fn pollNext(iterator: &mut dyn Iterator<Item = Option<Box<dyn Any>>>) -> Option<Box<dyn Any>> {
        // Java: if (iterator.hasNext())
        match iterator.next() {
            Some(val) => {
                // Java: T result = (T)iterator.next();
                // Java: iterator.remove();
                // Java: return result;
                val
            }
            // Java: else
            None => {
                // Java: return null;
                None
            }
        }
    }

    // Java: static void clear(Iterator<?> iterator)
    pub fn clear(iterator: &mut dyn Iterator<Item = Option<Box<dyn Any>>>) {
        // Java: Preconditions.checkNotNull(iterator);
        // Java: while(iterator.hasNext())
        while let Some(_) = iterator.next() {
            // Java: iterator.next();
            // Java: iterator.remove();
        }
    }

    // Java: static public static <T> UnmodifiableIterator<T> singletonIterator(final T value)
    pub fn singletonIterator(_value: Option<Box<dyn Any>>) {
        // Java: return new UnmodifiableIterator<T>()
        {
            // Java: boolean done;
            // Java: public boolean hasNext()
            {
                // Java: return !this.done;
            }
            // Java: public T next()
            {
                // Java: if (this.done)
                {
                    // Java: throw new NoSuchElementException();
                }
                // Java: else
                {
                    // Java: this.done = true;
                    // Java: return value;
                }
            }
        }
    }

    // Java: static public static <F, T> Iterator<T> transform(Iterator<F> fromIterator, final Function<? super F, ? extends T> function)
    pub fn transform(_from_iterator: &dyn Any, _function: &dyn Any) {
        // Java: Preconditions.checkNotNull(function);
        // Java: return new TransformedIterator<F, T>(fromIterator)
        {
            // Java: T transform(F from)
            {
                // Java: return function.apply(from);
            }
        }
    }

    // Java: static <T> UnmodifiableListIterator<T> forArray(final T[] array, final int offset, int length, int index)
    pub fn forArray_with_range(_array: &[Option<Box<dyn Any>>], offset: usize, length: usize, index: usize) {
        // Java: Preconditions.checkArgument(length >= 0);
        assert!(length >= 0);
        // Java: int end = offset + length;
        let end = offset + length;
        // Java: Preconditions.checkPositionIndexes(offset, end, array.length);
        assert!(offset <= end && end <= _array.len());
        // Java: Preconditions.checkPositionIndex(index, length);
        assert!(index <= length);
        // Java: return (UnmodifiableListIterator<T>)(length == 0 ? emptyListIterator() : new AbstractIndexedListIterator<T>(length, index)
        {
            // Java: protected T get(int index)
            {
                // Java: return (T)array[offset + index];
            }
        }
    }

    // Java: static <T> UnmodifiableListIterator<T> emptyListIterator()
    pub fn emptyListIterator() {
        // Java: return (UnmodifiableListIterator<T>)EMPTY_LIST_ITERATOR;
    }

    // Java: static boolean contains(Iterator<?> iterator, Object element)
    pub fn contains(_iterator: &dyn Any, _element: Option<&dyn Any>) -> bool {
        // Java: return any(iterator, Predicates.equalTo(element));
        false
    }

    // Java: static public static <T> boolean any(Iterator<T> iterator, Predicate<? super T> predicate)
    pub fn any(_iterator: &dyn Any, _predicate: &dyn Any) -> bool {
        // Java: return indexOf(iterator, predicate) != -1;
        false
    }

    // Java: static public static <T> int indexOf(Iterator<T> iterator, Predicate<? super T> predicate)
    pub fn indexOf(_iterator: &dyn Any, _predicate: &dyn Any) -> i32 {
        // Java: Assert.assertNotNull("predicate", predicate);
        // Java: for(int i = 0; iterator.hasNext(); ++i)
        {
            // Java: T current = (T)iterator.next();
            // Java: if (predicate.apply(current))
            {
                // Java: return i;
            }
        }
        // Java: return -1;
        -1
    }
}
