// Java: import java.io.IOException
// Java: import java.io.ObjectInputStream
// Java: import java.io.ObjectOutputStream
// Java: import java.io.Serializable
// Java: import java.text.MessageFormat
// Java: import java.util.AbstractSequentialList
// Java: import java.util.Collection
// Java: import java.util.ConcurrentModificationException
// Java: import java.util.List
// Java: import java.util.ListIterator
// Java: import java.util.NoSuchElementException

use std::sync::{Arc, Mutex};

// Java field: long serialVersionUID

// Java: public class LazyList extends AbstractSequentialList implements Serializable, Cloneable
pub struct LazyList<E: Clone> {
    // Java field: E[] indexedList
    // Java field: Entry<E> header
    // Java field: int size
    pub(crate) data: Vec<E>,
    pub(crate) mod_count: i32,
}

// Java: public class Entry
pub struct Entry<E: Clone> {
    // Java field: E element
    pub(crate) element: Option<E>,
    // Java field: Entry<E> next
    pub(crate) next: Option<Box<Entry<E>>>,
    // Java field: Entry<E> previous
    pub(crate) previous: Option<Box<Entry<E>>>,
}

// Java: public class Range
pub struct Range {
    // Java field: Entry<E> lower
    pub(crate) lower: usize,
    // Java field: Entry<E> upper
    pub(crate) upper: usize,
}

// Java: public class LazyListIterator
pub struct LazyListIterator<'a, E: Clone> {
    // Java field: Entry<E> lastReturned
    pub(crate) lastReturned: Option<usize>,
    // Java field: Entry<E> next
    pub(crate) next_idx: usize,
    // Java field: int indexNext
    pub(crate) indexNext: usize,
    // Java field: int expectedModCount
    pub(crate) expectedModCount: i32,
    pub(crate) list: &'a LazyList<E>,
}

// Java: public class LazyList extends AbstractSequentialList implements Serializable, Cloneable
impl<E: Clone> LazyList<E> {
    // Java: LazyList()
    pub fn new() -> Self {
        // Java: super();
        // Java: this.header = new Entry<E>(null, (Entry)null, (Entry)null);
        // Java: this.header.next = this.header;
        // Java: this.header.previous = this.header;
        LazyList {
            data: Vec::new(),
            mod_count: 0,
        }
    }

    // Java: LazyList(Entry<E> header)
    pub fn new_with_data(data: Vec<E>) -> Self {
        // Java: super();
        // Java: this.header = header;
        LazyList {
            data,
            mod_count: 0,
        }
    }

    // Java: boolean add(E element)
    pub fn add(&mut self, element: E) -> bool {
        // Java: this.addElement(element, this.header);
        self.data.push(element);
        self.mod_count += 1;
        // Java: return true;
        true
    }

    // Java: void add(int index, E element)
    pub fn add_at(&mut self, index: usize, element: E) {
        // Java: this.addElement(element, index == this.size ? this.header : this.getEntry(index));
        self.data.insert(index, element);
        self.mod_count += 1;
    }

    // Java: boolean addAll(Collection<? extends E> collection)
    pub fn addAll(&mut self, collection: Vec<E>) -> bool {
        // Java: return this.addAll(0, collection);
        self.addAll_at(self.data.len(), collection)
    }

    // Java: boolean addAll(int index, Collection<? extends E> collection)
    pub fn addAll_at(&mut self, index: usize, collection: Vec<E>) -> bool {
        // Java: if (collection.isEmpty())
        if collection.is_empty()
        {
            // Java: return false;
            return false;
        }
        // Java: else
        {
            // Java: Entry<E> entry = this.getEntryHeader(index).next;
            let mut insert_idx = index;
            // Java: for(E element : collection)
            for element in collection
            {
                // Java: this.addElement(element, entry);
                self.data.insert(insert_idx, element);
                insert_idx += 1;
            }
            self.mod_count += 1;
            // Java: return true;
            true
        }
    }

    // Java: void clear()
    pub fn clear(&mut self) {
        // Java: this.indexedList = null;
        // Java: this.size = 0;
        // Java: this.header.element = null;
        // Java: this.header.next = this.header;
        // Java: this.header.previous = this.header;
        // Java: ++this.modCount;
        self.data.clear();
        self.mod_count += 1;
    }

    // Java: E get(int index)
    pub fn get(&self, index: usize) -> &E {
        // Java: this.createIndexedList();
        // Java: return (E)this.indexedList[index];
        &self.data[index]
    }

    // Java: ListIterator<E> listIterator(int index)
    pub fn listIterator(&self, index: usize) -> LazyListIterator<E> {
        // Java: return new LazyListIterator(index);
        LazyListIterator {
            lastReturned: None,
            next_idx: index,
            indexNext: index,
            expectedModCount: self.mod_count,
            list: self,
        }
    }

    // Java: E remove(int index)
    pub fn remove_at(&mut self, index: usize) -> E {
        // Java: return (E)this.removeEntry(this.getEntry(index));
        self.mod_count += 1;
        self.data.remove(index)
    }

    // Java: void removeRange(int fromIndex, int toIndex)
    pub fn removeRange(&mut self, from_index: usize, to_index: usize) {
        // Java: Range<E> range = this.getRange(fromIndex, toIndex);
        // Java: range.lower.previous.next = range.upper;
        // Java: range.upper.previous = range.lower.previous;
        self.data.drain(from_index..to_index);
        self.mod_count += 1;
    }

    // Java: E set(int index, E element)
    pub fn set(&mut self, index: usize, element: E) -> E {
        // Java: Entry<E> entry = this.getEntry(index);
        // Java: E oldValue = entry.element;
        // Java: entry.element = element;
        // Java: if (this.indexedList != null)
        {
            // Java: this.indexedList[index] = element;
        }
        // Java: return oldValue;
        std::mem::replace(&mut self.data[index], element)
    }

    // Java: List<E> subList(int fromIndex, int toIndex)
    pub fn subList(&self, from_index: usize, to_index: usize) -> LazyList<E> {
        // Java: Range<E> range = this.getRange(fromIndex, toIndex);
        // Java: Entry<E> newHeader = new Entry<E>(null, range.lower, range.upper);
        // Java: Entry<E> lastEntry;
        // Java: for(lastEntry = newHeader; range.lower != range.upper; range.lower = range.lower.next)
        {
            // Java: lastEntry.next = new Entry<E>(range.lower.element, (Entry)null, lastEntry);
            // Java: lastEntry = lastEntry.next;
        }
        // Java: newHeader.previous = lastEntry;
        // Java: return new LazyList<E>(newHeader);
        LazyList::new_with_data(self.data[from_index..to_index].to_vec())
    }

    // Java: int size()
    pub fn size(&self) -> i32 {
        // Java: return this.size;
        self.data.len() as i32
    }

    // Java: LazyList<E> clone()
    pub fn clone_list(&self) -> LazyList<E> {
        // Java: LazyList<E> clone = new LazyList<E>();
        // Java: for(Entry<E> entry = this.header.next; entry != this.header; entry = entry.next)
        {
            // Java: clone.add(entry.element);
        }
        // Java: return clone;
        LazyList::new_with_data(self.data.clone())
    }

    // Java: void createIndexedList()
    pub fn createIndexedList(&self) {
        // Java: if (this.indexedList == null)
        {
            // Java: this.indexedList = (E[])(new Object[this.size]);
            // Java: int index = 0;
            // Java: for(E element : this)
            {
                // Java: this.indexedList[index++] = element;
            }
        }
    }

    // Java: Entry<E> addElement(E e, Entry<E> entry)
    fn addElement(&mut self, e: E, index: usize) {
        // Java: this.indexedList = null;
        // Java: Entry<E> newEntry = new Entry<E>(e, entry, entry.previous);
        // Java: newEntry.previous.next = newEntry;
        // Java: newEntry.next.previous = newEntry;
        // Java: ++this.size;
        // Java: ++this.modCount;
        // Java: return newEntry;
        self.data.insert(index, e);
        self.mod_count += 1;
    }

    // Java: Entry<E> getEntry(int index)
    fn getEntry(&self, index: usize) -> &E {
        // Java: if (index >= 0 && index < this.size)
        if index < self.data.len()
        {
            // Java: if (index == 0)
            {
                // Java: return this.header.next;
            }
            // Java: else
            {
                // Java: Entry<E> e = this.header;
                // Java: if (index < this.size >> 1)
                {
                    // Java: for(int i = 0; i <= index; ++i)
                    {
                        // Java: e = e.next;
                    }
                }
                // Java: else
                {
                    // Java: for(int i = this.size; i > index; --i)
                    {
                        // Java: e = e.previous;
                    }
                }
                // Java: return e;
            }
            &self.data[index]
        }
        // Java: else
        else
        {
            // Java: throw new IndexOutOfBoundsException(MessageFormat.format("Index: {0}, Size: {1}", index, this.size));
            panic!("Index: {}, Size: {}", index, self.data.len());
        }
    }

    // Java: Entry<E> getEntryHeader(int index)
    fn getEntryHeader(&self, index: usize) -> &E {
        // Java: if (index >= 0 && index <= this.size)
        if index <= self.data.len()
        {
            // Java: if (index == 0)
            {
                // Java: return this.header.next;
            }
            // Java: else
            {
                // Java: Entry<E> e = this.header;
                // Java: if (index < this.size >> 1)
                {
                    // Java: for(int i = 0; i < index; ++i)
                    {
                        // Java: e = e.next;
                    }
                }
                // Java: else
                {
                    // Java: for(int i = this.size; i > index; --i)
                    {
                        // Java: e = e.previous;
                    }
                }
                // Java: return e;
            }
            if index < self.data.len() {
                &self.data[index]
            } else {
                panic!("Index: {}, Size: {}", index, self.data.len());
            }
        }
        // Java: else
        else
        {
            // Java: throw new IndexOutOfBoundsException(MessageFormat.format("Index: {0}, Size: {1}", index, this.size));
            panic!("Index: {}, Size: {}", index, self.data.len());
        }
    }

    // Java: Range<E> getRange(int fromIndex, int toIndex)
    fn getRange(&self, from_index: usize, to_index: usize) -> Range {
        // Java: if (fromIndex >= 0 && toIndex <= this.size && fromIndex <= toIndex)
        if to_index <= self.data.len() && from_index <= to_index
        {
            // Java: int[] length = new int[]{fromIndex, toIndex - fromIndex, this.size - toIndex};
            // Java: Range<E> range = new Range<E>();
            // Java: if (length[0] < length[2])
            {
                // Java: range.lower = this.header;
                // Java: for(int i = 0; i <= fromIndex; ++i)
                {
                    // Java: range.lower = range.lower.next;
                }
                // Java: if (length[1] < length[2])
                {
                    // Java: range.upper = range.lower;
                    // Java: for(int i = fromIndex; i <= toIndex; ++i)
                    {
                        // Java: range.upper = range.upper.next;
                    }
                }
                // Java: else
                {
                    // Java: range.upper = this.header;
                    // Java: for(int i = this.size; i > toIndex; --i)
                    {
                        // Java: range.upper = range.upper.previous;
                    }
                }
            }
            // Java: else
            {
                // Java: range.upper = this.header;
                // Java: for(int i = this.size; i > toIndex; --i)
                {
                    // Java: range.upper = range.upper.previous;
                }
                // Java: if (length[0] < length[1])
                {
                    // Java: range.lower = this.header;
                    // Java: for(int i = 0; i <= fromIndex; ++i)
                    {
                        // Java: range.lower = range.lower.next;
                    }
                }
                // Java: else
                {
                    // Java: range.lower = range.upper;
                    // Java: for(int i = toIndex; i > fromIndex; --i)
                    {
                        // Java: range.lower = range.lower.previous;
                    }
                }
            }
            // Java: return range;
            Range { lower: from_index, upper: to_index }
        }
        // Java: else
        else
        {
            // Java: throw new IndexOutOfBoundsException(MessageFormat.format("FromIndex: {0}, ToIndex: {1}, Size: {2}", fromIndex, toIndex, this.size));
            panic!("FromIndex: {}, ToIndex: {}, Size: {}", from_index, to_index, self.data.len());
        }
    }

    // Java: E removeEntry(Entry<E> entry)
    fn removeEntry(&mut self, index: usize) -> E {
        // Java: if (entry == this.header)
        if index >= self.data.len()
        {
            // Java: throw new NoSuchElementException();
            panic!("NoSuchElementException");
        }
        // Java: else
        else
        {
            // Java: this.indexedList = null;
            // Java: entry.previous.next = entry.next;
            // Java: entry.next.previous = entry.previous;
            // Java: --this.size;
            // Java: ++this.modCount;
            // Java: return entry.element;
            self.mod_count += 1;
            self.data.remove(index)
        }
    }

    // Java: void writeObject(ObjectOutputStream stream)
    pub fn writeObject(&self) {
        // Java: stream.defaultWriteObject();
        // Java: stream.writeInt(this.size);
        // Java: for(Entry<E> entry = this.header.next; entry != this.header; entry = entry.next)
        {
            // Java: stream.writeObject(entry.element);
        }
    }

    // Java: void readObject(ObjectInputStream stream)
    pub fn readObject(&mut self) {
        // Java: stream.defaultReadObject();
        // Java: CloneHelper.setFinalField(LazyList.class, this, "header", new Entry(null, (Entry)null, (Entry)null));
        // Java: assert this.header != null;
        // Java: this.header.next = this.header.previous = this.header;
        // Java: int size = stream.readInt();
        // Java: E[] tempIndexedList = (E[])(new Object[size]);
        // Java: for(int i = 0; i < size; ++i)
        {
            // Java: E element = (E)stream.readObject();
            // Java: this.addElement(element, this.header);
            // Java: tempIndexedList[i] = element;
        }
        // Java: this.indexedList = tempIndexedList;
    }
}

// Java: public class Entry
impl<E: Clone> Entry<E> {
}

// Java: public class Range
impl Range {
    // Java: Range()
    pub fn new() -> Self {
        // Java: super();
        Range { lower: 0, upper: 0 }
    }
}

// Java: public class LazyListIterator
impl<'a, E: Clone> LazyListIterator<'a, E> {
    // Java: boolean hasNext()
    pub fn hasNext(&self) -> bool {
        // Java: return this.indexNext != LazyList.this.size;
        (self.indexNext as i32) != self.list.size()
    }

    // Java: E next()
    pub fn next_item(&mut self) -> &E {
        // Java: this.checkForComodification();
        self.checkForComodification();
        // Java: if (this.indexNext == LazyList.this.size)
        if (self.indexNext as i32) == self.list.size()
        {
            // Java: throw new NoSuchElementException();
            panic!("NoSuchElementException");
        }
        // Java: else
        else
        {
            // Java: this.lastReturned = this.next;
            self.lastReturned = Some(self.indexNext);
            // Java: this.next = this.next.next;
            // Java: ++this.indexNext;
            self.indexNext += 1;
            // Java: return this.lastReturned.element;
            &self.list.data[self.indexNext - 1]
        }
    }

    // Java: boolean hasPrevious()
    pub fn hasPrevious(&self) -> bool {
        // Java: return this.indexNext != 0;
        self.indexNext != 0
    }

    // Java: E previous()
    pub fn previous(&mut self) -> &E {
        // Java: this.checkForComodification();
        self.checkForComodification();
        // Java: if (this.indexNext == 0)
        if self.indexNext == 0
        {
            // Java: throw new NoSuchElementException();
            panic!("NoSuchElementException");
        }
        // Java: else
        else
        {
            // Java: this.next = this.next.previous;
            // Java: this.lastReturned = this.next;
            // Java: --this.indexNext;
            self.indexNext -= 1;
            self.lastReturned = Some(self.indexNext);
            // Java: return this.lastReturned.element;
            &self.list.data[self.indexNext]
        }
    }

    // Java: int nextIndex()
    pub fn nextIndex(&self) -> i32 {
        // Java: return this.indexNext;
        self.indexNext as i32
    }

    // Java: int previousIndex()
    pub fn previousIndex(&self) -> i32 {
        // Java: return this.indexNext - 1;
        self.indexNext as i32 - 1
    }

    // Java: void remove()
    pub fn remove(&mut self) {
        // Java: this.checkForOutside();
        self.checkForOutside();
        // Java: this.checkForComodification();
        self.checkForComodification();
        // Java: Entry<E> lastNext = this.lastReturned.next;
        // Java: LazyList.this.removeEntry(this.lastReturned);
        // Java: if (this.next == this.lastReturned)
        {
            // Java: this.next = lastNext;
        }
        // Java: else
        {
            // Java: --this.indexNext;
        }
        // Java: this.lastReturned = LazyList.this.header;
        self.lastReturned = None;
        // Java: ++this.expectedModCount;
        self.expectedModCount += 1;
    }

    // Java: void set(E e)
    pub fn set(&self, _e: E) {
        // Java: this.checkForOutside();
        self.checkForOutside();
        // Java: this.checkForComodification();
        self.checkForComodification();
        // Java: this.lastReturned.element = e;
    }

    // Java: void add(E e)
    pub fn add(&mut self, _e: E) {
        // Java: this.checkForComodification();
        self.checkForComodification();
        // Java: this.lastReturned = LazyList.this.header;
        self.lastReturned = None;
        // Java: LazyList.this.addElement(e, this.next);
        // Java: ++this.indexNext;
        self.indexNext += 1;
        // Java: ++this.expectedModCount;
        self.expectedModCount += 1;
    }

    // Java: void checkForComodification()
    pub fn checkForComodification(&self) {
        // Java: if (LazyList.this.modCount != this.expectedModCount)
        if self.list.mod_count != self.expectedModCount
        {
            // Java: throw new ConcurrentModificationException();
            panic!("ConcurrentModificationException");
        }
    }

    // Java: void checkForOutside()
    pub fn checkForOutside(&self) {
        // Java: if (this.lastReturned == LazyList.this.header)
        if self.lastReturned.is_none()
        {
            // Java: throw new IllegalStateException();
            panic!("IllegalStateException");
        }
    }
}
