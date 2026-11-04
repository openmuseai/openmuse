#![allow(non_snake_case, dead_code, unused_variables, unused_imports)]

use std::any::Any;
use super::CollectPreconditions::CollectPreconditions;
use super::Ints::Ints;

// Java: public class Lists
pub struct Lists;

// Java: public class Lists
impl Lists {
    // Java: Lists()
    // Java: super();

    // Java: static public static <E> ArrayList<E> newArrayList()
    pub fn newArrayList() -> Vec<Option<Box<dyn Any>>> {
        // Java: return new ArrayList();
        Vec::new()
    }

    // Java: static public static <E> ArrayList<E> newArrayList(E... elements)
    pub fn newArrayList_from(elements: Vec<Option<Box<dyn Any>>>) -> Vec<Option<Box<dyn Any>>> {
        // Java: Preconditions.checkNotNull(elements);
        // Java: int capacity = computeArrayListCapacity(elements.length);
        let capacity = Self::computeArrayListCapacity(elements.len() as i32);
        // Java: ArrayList<E> list = new ArrayList(capacity);
        let mut list = Vec::with_capacity(capacity as usize);
        // Java: Collections.addAll(list, elements);
        for e in elements {
            list.push(e);
        }
        // Java: return list;
        list
    }

    // Java: static int computeArrayListCapacity(int arraySize)
    pub fn computeArrayListCapacity(array_size: i32) -> i32 {
        // Java: CollectPreconditions.checkNonnegative(arraySize, "arraySize");
        CollectPreconditions::checkNonnegative(array_size, "arraySize");
        // Java: return Ints.saturatedCast(5L + (long)arraySize + (long)(arraySize / 10));
        Ints::saturatedCast(5i64 + array_size as i64 + (array_size / 10) as i64)
    }

    // Java: static public static <E> ArrayList<E> newArrayList(Iterator<? extends E> elements)
    pub fn newArrayList_from_iter() -> Vec<Option<Box<dyn Any>>> {
        // Java: ArrayList<E> list = newArrayList();
        let list = Self::newArrayList();
        // Java: Iterators.addAll(list, elements);
        // Java: return list;
        list
    }

    // Java: static public static <E> ArrayList<E> newArrayListWithExpectedSize(int estimatedSize)
    pub fn newArrayListWithExpectedSize(estimated_size: i32) -> Vec<Option<Box<dyn Any>>> {
        // Java: return new ArrayList(computeArrayListCapacity(estimatedSize));
        Vec::with_capacity(Self::computeArrayListCapacity(estimated_size) as usize)
    }

    // Java: static public static <E> LinkedList<E> newLinkedList()
    pub fn newLinkedList() -> std::collections::LinkedList<Option<Box<dyn Any>>> {
        // Java: return new LinkedList();
        std::collections::LinkedList::new()
    }

    // Java: static public static <E> List<E> asList(E first, E[] rest)
    pub fn asList_one(_first: Option<Box<dyn Any>>, _rest: Vec<Option<Box<dyn Any>>>) -> Vec<Option<Box<dyn Any>>> {
        // Java: return new OnePlusArrayList<E>(first, rest);
        Vec::new()
    }

    // Java: static public static <E> List<E> asList(E first, E second, E[] rest)
    pub fn asList_two(_first: Option<Box<dyn Any>>, _second: Option<Box<dyn Any>>, _rest: Vec<Option<Box<dyn Any>>>) -> Vec<Option<Box<dyn Any>>> {
        // Java: return new TwoPlusArrayList<E>(first, second, rest);
        Vec::new()
    }

    // Java: static ImmutableList<Character> charactersOf(String string)
    pub fn charactersOf_string(string: &str) -> StringAsImmutableList {
        // Java: return new StringAsImmutableList((String)Preconditions.checkNotNull(string));
        StringAsImmutableList::new(string.to_string())
    }

    // Java: static List<Character> charactersOf(CharSequence sequence)
    pub fn charactersOf_charseq(sequence: &str) -> CharSequenceAsList {
        // Java: return new CharSequenceAsList((CharSequence)Preconditions.checkNotNull(sequence));
        CharSequenceAsList::new(sequence.to_string())
    }

    // Java: static int hashCodeImpl(List<?> list)
    pub fn hashCodeImpl(list: &[Option<Box<dyn Any>>]) -> i32 {
        // Java: int hashCode = 1;
        let mut hash_code: i32 = 1;
        // Java: for(Object o : list)
        for o in list.iter() {
            // Java: hashCode = 31 * hashCode + (o == null ? 0 : o.hashCode());
            hash_code = hash_code.wrapping_mul(31).wrapping_add(if o.is_none() { 0 } else { 0 });
            // Java: hashCode = ~(~hashCode);
            hash_code = !(!hash_code);
        }
        // Java: return hashCode;
        hash_code
    }

    // Java: static boolean equalsImpl(List<?> list, Object object)
    pub fn equalsImpl(_list: &dyn Any, _object: Option<&dyn Any>) -> bool {
        // Java: if (object == Preconditions.checkNotNull(list))
        {
            // Java: return true;
        }
        // Java: else if (!(object instanceof List))
        {
            // Java: return false;
        }
        // Java: else
        {
            // Java: List<?> o = (List)object;
            // Java: return list.size() == o.size() && Iterators.elementsEqual(list.iterator(), o.iterator());
            false
        }
    }

    // Java: static <E> boolean addAllImpl(List<E> list, int index, Iterable<? extends E> elements)
    pub fn addAllImpl(_list: &mut Vec<Option<Box<dyn Any>>>, _index: usize, _elements: &[Option<Box<dyn Any>>]) -> bool {
        // Java: boolean changed = false;
        let mut changed = false;
        // Java: ListIterator<E> listIterator = list.listIterator(index);
        // Java: for(E e : elements)
        for _e in _elements.iter() {
            // Java: listIterator.add(e);
            // Java: changed = true;
            changed = true;
        }
        // Java: return changed;
        changed
    }

    // Java: static int indexOfImpl(List<?> list, Object element)
    pub fn indexOfImpl(_list: &[Option<Box<dyn Any>>], _element: Option<&dyn Any>) -> i32 {
        // Java: ListIterator<?> listIterator = list.listIterator();
        // Java: while(listIterator.hasNext())
        {
            // Java: if (Objects.equal(element, listIterator.next()))
            {
                // Java: return listIterator.previousIndex();
            }
        }
        // Java: return -1;
        -1
    }

    // Java: static int lastIndexOfImpl(List<?> list, Object element)
    pub fn lastIndexOfImpl(_list: &[Option<Box<dyn Any>>], _element: Option<&dyn Any>) -> i32 {
        // Java: ListIterator<?> listIterator = list.listIterator(list.size());
        // Java: while(listIterator.hasPrevious())
        {
            // Java: if (Objects.equal(element, listIterator.previous()))
            {
                // Java: return listIterator.nextIndex();
            }
        }
        // Java: return -1;
        -1
    }

    // Java: static <E> ListIterator<E> listIteratorImpl(List<E> list, int index)
    pub fn listIteratorImpl(_list: &dyn Any, _index: usize) {
        // Java: return (new AbstractListWrapper(list)).listIterator(index);
    }

    // Java: static <E> List<E> subListImpl(List<E> list, int fromIndex, int toIndex)
    pub fn subListImpl(_list: &dyn Any, from_index: usize, to_index: usize) {
        // Java: List<E> wrapper;
        // Java: if (list instanceof RandomAccess)
        {
            // Java: wrapper = new RandomAccessListWrapper<E>(list)
            {
                // Java: public ListIterator<E> listIterator(int index)
                {
                    // Java: return this.backingList.listIterator(index);
                }
            }
        }
        // Java: else
        {
            // Java: wrapper = new AbstractListWrapper<E>(list)
            {
                // Java: public ListIterator<E> listIterator(int index)
                {
                    // Java: return this.backingList.listIterator(index);
                }
            }
        }
        // Java: return wrapper.subList(fromIndex, toIndex);
    }

    // Java: static <T> List<T> cast(Iterable<T> iterable)
    pub fn cast(_iterable: &dyn Any) {
        // Java: return (List)iterable;
    }
}

// Java: public class OnePlusArrayList
pub struct OnePlusArrayList {
    // Java field: E first
    pub first: Option<Box<dyn Any>>,
    // Java field: E[] rest
    pub rest: Vec<Option<Box<dyn Any>>>,
}

// Java: public class OnePlusArrayList
impl OnePlusArrayList {
    // Java: OnePlusArrayList(E first, E[] rest)
    pub fn new(first: Option<Box<dyn Any>>, rest: Vec<Option<Box<dyn Any>>>) -> Self {
        // Java: super();
        // Java: this.first = first;
        // Java: this.rest = (E[])((Object[])Preconditions.checkNotNull(rest));
        OnePlusArrayList { first, rest }
    }

    // Java: int size()
    pub fn size(&self) -> usize {
        // Java: return this.rest.length + 1;
        self.rest.len() + 1
    }

    // Java: E get(int index)
    pub fn get(&self, index: usize) -> Option<&dyn Any> {
        // Java: Preconditions.checkElementIndex(index, this.size());
        assert!(index < self.size());
        // Java: return (E)(index == 0 ? this.first : this.rest[index - 1]);
        if index == 0 {
            self.first.as_deref()
        } else {
            self.rest[index - 1].as_deref()
        }
    }
}

// Java: public class TwoPlusArrayList
pub struct TwoPlusArrayList {
    // Java field: E first
    pub first: Option<Box<dyn Any>>,
    // Java field: E second
    pub second: Option<Box<dyn Any>>,
    // Java field: E[] rest
    pub rest: Vec<Option<Box<dyn Any>>>,
}

// Java: public class TwoPlusArrayList
impl TwoPlusArrayList {
    // Java: TwoPlusArrayList(E first, E second, E[] rest)
    pub fn new(first: Option<Box<dyn Any>>, second: Option<Box<dyn Any>>, rest: Vec<Option<Box<dyn Any>>>) -> Self {
        // Java: super();
        // Java: this.first = first;
        // Java: this.second = second;
        // Java: this.rest = (E[])((Object[])Preconditions.checkNotNull(rest));
        TwoPlusArrayList { first, second, rest }
    }

    // Java: int size()
    pub fn size(&self) -> usize {
        // Java: return this.rest.length + 2;
        self.rest.len() + 2
    }

    // Java: E get(int index)
    pub fn get(&self, index: usize) -> Option<&dyn Any> {
        // Java: switch (index)
        match index {
            // Java: case 0:
            0 => {
                // Java: return this.first;
                self.first.as_deref()
            }
            // Java: case 1:
            1 => {
                // Java: return this.second;
                self.second.as_deref()
            }
            // Java: default:
            _ => {
                // Java: Preconditions.checkElementIndex(index, this.size());
                assert!(index < self.size());
                // Java: return (E)this.rest[index - 2];
                self.rest[index - 2].as_deref()
            }
        }
    }
}

// Java: public class StringAsImmutableList
pub struct StringAsImmutableList {
    // Java field: String string
    pub string: String,
}

// Java: public class StringAsImmutableList
impl StringAsImmutableList {
    // Java: StringAsImmutableList(String string)
    pub fn new(string: String) -> Self {
        // Java: super();
        // Java: this.string = string;
        StringAsImmutableList { string }
    }

    // Java: int indexOf(Object object)
    pub fn indexOf(&self, object: Option<char>) -> i32 {
        // Java: return object instanceof Character ? this.string.indexOf((Character)object) : -1;
        match object {
            Some(c) => self.string.find(c).map(|i| i as i32).unwrap_or(-1),
            None => -1,
        }
    }

    // Java: int lastIndexOf(Object object)
    pub fn lastIndexOf(&self, object: Option<char>) -> i32 {
        // Java: return object instanceof Character ? this.string.lastIndexOf((Character)object) : -1;
        match object {
            Some(c) => self.string.rfind(c).map(|i| i as i32).unwrap_or(-1),
            None => -1,
        }
    }

    // Java: ImmutableList<Character> subList(int fromIndex, int toIndex)
    pub fn subList(&self, from_index: usize, to_index: usize) -> StringAsImmutableList {
        // Java: Preconditions.checkPositionIndexes(fromIndex, toIndex, this.size());
        assert!(from_index <= to_index && to_index <= self.size());
        // Java: return Lists.charactersOf(this.string.substring(fromIndex, toIndex));
        StringAsImmutableList::new(self.string[from_index..to_index].to_string())
    }

    // Java: Character get(int index)
    pub fn get(&self, index: usize) -> char {
        // Java: Preconditions.checkElementIndex(index, this.size());
        assert!(index < self.size());
        // Java: return this.string.charAt(index);
        self.string.chars().nth(index).unwrap_or('\0')
    }

    // Java: int size()
    pub fn size(&self) -> usize {
        // Java: return this.string.length();
        self.string.len()
    }
}

// Java: public class CharSequenceAsList
pub struct CharSequenceAsList {
    // Java field: CharSequence sequence
    pub sequence: String,
}

// Java: public class CharSequenceAsList
impl CharSequenceAsList {
    // Java: CharSequenceAsList(CharSequence sequence)
    pub fn new(sequence: String) -> Self {
        // Java: super();
        // Java: this.sequence = sequence;
        CharSequenceAsList { sequence }
    }

    // Java: Character get(int index)
    pub fn get(&self, index: usize) -> char {
        // Java: Preconditions.checkElementIndex(index, this.size());
        assert!(index < self.size());
        // Java: return this.sequence.charAt(index);
        self.sequence.chars().nth(index).unwrap_or('\0')
    }

    // Java: int size()
    pub fn size(&self) -> usize {
        // Java: return this.sequence.length();
        self.sequence.len()
    }
}

// Java: public class AbstractListWrapper
pub struct AbstractListWrapper {
    // Java field: List<E> backingList
    pub backingList: Vec<Option<Box<dyn Any>>>,
}

// Java: public class AbstractListWrapper
impl AbstractListWrapper {
    // Java: AbstractListWrapper(List backingList)
    pub fn new(backing_list: Vec<Option<Box<dyn Any>>>) -> Self {
        // Java: super();
        // Java: this.backingList = (List)Preconditions.checkNotNull(backingList);
        AbstractListWrapper { backingList: backing_list }
    }

    // Java: void add(int index, E element)
    pub fn add(&mut self, index: usize, element: Option<Box<dyn Any>>) {
        // Java: this.backingList.add(index, element);
        self.backingList.insert(index, element);
    }

    // Java: boolean addAll(int index, Collection<? extends E> c)
    pub fn addAll(&mut self, _index: usize, _c: &dyn Any) -> bool {
        // Java: return this.backingList.addAll(index, c);
        false
    }

    // Java: E get(int index)
    pub fn get(&self, index: usize) -> Option<&dyn Any> {
        // Java: return (E)this.backingList.get(index);
        self.backingList.get(index).and_then(|v| v.as_deref())
    }

    // Java: E remove(int index)
    pub fn remove(&mut self, index: usize) -> Option<Box<dyn Any>> {
        // Java: return (E)this.backingList.remove(index);
        self.backingList.remove(index)
    }

    // Java: E set(int index, E element)
    pub fn set(&mut self, index: usize, element: Option<Box<dyn Any>>) -> Option<Box<dyn Any>> {
        // Java: return (E)this.backingList.set(index, element);
        std::mem::replace(&mut self.backingList[index], element)
    }

    // Java: boolean contains(Object o)
    pub fn contains(&self, _o: Option<&dyn Any>) -> bool {
        // Java: return this.backingList.contains(o);
        false
    }

    // Java: int size()
    pub fn size(&self) -> usize {
        // Java: return this.backingList.size();
        self.backingList.len()
    }
}

// Java: public class RandomAccessListWrapper
pub struct RandomAccessListWrapper {
    pub base: AbstractListWrapper,
}

// Java: public class RandomAccessListWrapper
impl RandomAccessListWrapper {
    // Java: RandomAccessListWrapper(List backingList)
    pub fn new(backing_list: Vec<Option<Box<dyn Any>>>) -> Self {
        // Java: super(backingList);
        RandomAccessListWrapper {
            base: AbstractListWrapper::new(backing_list),
        }
    }
}
