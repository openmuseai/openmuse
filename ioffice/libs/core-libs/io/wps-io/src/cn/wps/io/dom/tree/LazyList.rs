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

// Java field: long serialVersionUID
pub(crate) const serialVersionUID: i64 = 0;

// Java: public class LazyList extends AbstractSequentialList implements Serializable, Cloneable
pub struct LazyList {
    // Java field: E[] indexedList
    pub(crate) indexedList: Option<Vec<Box<dyn std::any::Any>>>,
    // Java field: Entry<E> header
    pub(crate) header: Option<Box<LazyListEntry>>,
    // Java field: int size
    pub(crate) size: i32,
}

// Java: public class Entry
pub struct LazyListEntry {
    // Java field: E element
    pub(crate) element: Option<Box<dyn std::any::Any>>,
    // Java field: Entry<E> next
    pub(crate) next: Option<Box<LazyListEntry>>,
    // Java field: Entry<E> previous
    pub(crate) previous: Option<Box<LazyListEntry>>,
}

// Java: public class Range
pub struct LazyListRange {
    // Java field: Entry<E> lower
    pub(crate) lower: Option<Box<LazyListEntry>>,
    // Java field: Entry<E> upper
    pub(crate) upper: Option<Box<LazyListEntry>>,
}

// Java: public class LazyListIterator
pub struct LazyListIterator {
    // Java field: Entry<E> lastReturned
    pub(crate) lastReturned: Option<Box<LazyListEntry>>,
    // Java field: Entry<E> next
    pub(crate) next: Option<Box<LazyListEntry>>,
    // Java field: int indexNext
    pub(crate) indexNext: i32,
    // Java field: int expectedModCount
    pub(crate) expectedModCount: i32,
}

// Java: public class LazyList extends AbstractSequentialList implements Serializable, Cloneable
impl LazyList {
    // Java: LazyList()
    pub fn new() -> Self {
      // Java: super();
      todo!();
      // Java: this.header = new Entry<E>(null, (Entry)null, (Entry)null);
      todo!();
      // Java: this.header.next = this.header;
      todo!();
      // Java: this.header.previous = this.header;
      todo!();
    }

    // Java: LazyList(Entry<E> header)
    pub fn new_1() -> Self {
      // Java: super();
      todo!();
      // Java: this.header = header;
      todo!();
    }

    // Java: boolean add(E element)
    pub fn add(&mut self) -> bool {
      // Java: this.addElement(element, this.header);
      todo!();
      // Java: return true;
      todo!();
    }

    // Java: void add(int index, E element)
    pub fn add_2(&mut self) {
      // Java: this.addElement(element, index == this.size ? this.header : this.getEntry(index));
      todo!();
    }

    // Java: boolean addAll(Collection<? extends E> collection)
    pub fn addAll(&mut self) -> bool {
      // Java: return this.addAll(0, collection);
      todo!();
    }

    // Java: boolean addAll(int index, Collection<? extends E> collection)
    pub fn addAll_2(&mut self) -> bool {
      // Java: if (collection.isEmpty())
      todo!();
      {
         // Java: return false;
      todo!();
      }
      // Java: else
      todo!();
      {
         // Java: Entry<E> entry = this.getEntryHeader(index).next;
      todo!();
         // Java: for(E element : collection)
      todo!();
         {
            // Java: this.addElement(element, entry);
      todo!();
         }
         // Java: return true;
      todo!();
      }
    }

    // Java: void clear()
    pub fn clear(&mut self) {
      // Java: this.indexedList = null;
      todo!();
      // Java: this.size = 0;
      todo!();
      // Java: this.header.element = null;
      todo!();
      // Java: this.header.next = this.header;
      todo!();
      // Java: this.header.previous = this.header;
      todo!();
      // Java: ++this.modCount;
      todo!();
    }

    // Java: E get(int index)
    pub fn get(&mut self) -> Option<Box<dyn std::any::Any>> {
      // Java: this.createIndexedList();
      todo!();
      // Java: return (E)this.indexedList[index];
      todo!();
    }

    // Java: ListIterator<E> listIterator(int index)
    pub fn listIterator(&mut self) -> LazyListIterator {
      // Java: return new LazyListIterator(index);
      todo!();
    }

    // Java: E remove(int index)
    pub fn remove(&mut self) -> Option<Box<dyn std::any::Any>> {
      // Java: return (E)this.removeEntry(this.getEntry(index));
      todo!();
    }

    // Java: void removeRange(int fromIndex, int toIndex)
    pub fn removeRange(&mut self) {
      // Java: Range<E> range = this.getRange(fromIndex, toIndex);
      todo!();
      // Java: range.lower.previous.next = range.upper;
      todo!();
      // Java: range.upper.previous = range.lower.previous;
      todo!();
    }

    // Java: E set(int index, E element)
    pub fn set(&mut self) -> Option<Box<dyn std::any::Any>> {
      // Java: Entry<E> entry = this.getEntry(index);
      todo!();
      // Java: E oldValue = entry.element;
      todo!();
      // Java: entry.element = element;
      todo!();
      // Java: if (this.indexedList != null)
      todo!();
      {
         // Java: this.indexedList[index] = element;
      todo!();
      }
      // Java: return oldValue;
      todo!();
    }

    // Java: List<E> subList(int fromIndex, int toIndex)
    pub fn subList(&mut self) -> Vec<Box<dyn std::any::Any>> {
      // Java: Range<E> range = this.getRange(fromIndex, toIndex);
      todo!();
      // Java: Entry<E> newHeader = new Entry<E>(null, range.lower, range.upper);
      todo!();
      // Java: Entry<E> lastEntry;
      todo!();
      // Java: for(lastEntry = newHeader; range.lower != range.upper; range.lower = range.lower.next)
      todo!();
      {
         // Java: lastEntry.next = new Entry<E>(range.lower.element, (Entry)null, lastEntry);
      todo!();
         // Java: lastEntry = lastEntry.next;
      todo!();
      }
      // Java: newHeader.previous = lastEntry;
      todo!();
      // Java: return new LazyList<E>(newHeader);
      todo!();
    }

    // Java: int size()
    pub fn size(&mut self) -> i32 {
      // Java: return this.size;
      todo!();
    }

    // Java: LazyList<E> clone()
    pub fn clone(&mut self) -> LazyList {
      // Java: try
      todo!();
      {
         // Java: LazyList<E> clone = (LazyList)super.clone();
      todo!();
         // Java: for(Entry<E> entry = this.header.next; entry != this.header; entry = entry.next)
      todo!();
         {
            // Java: clone.add(entry.element);
      todo!();
         }
         // Java: return clone;
      todo!();
      }
      // Java: catch (CloneNotSupportedException e)
      todo!();
      {
         // Java: throw new AssertionError(e);
      todo!();
      }
    }

    // Java: void createIndexedList()
    pub fn createIndexedList(&mut self) {
      // Java: if (this.indexedList == null)
      todo!();
      {
         // Java: this.indexedList = (E[])(new Object[this.size]);
      todo!();
         // Java: int index = 0;
      todo!();
         // Java: for(E element : this)
      todo!();
         {
            // Java: this.indexedList[index++] = element;
      todo!();
         }
      }
    }

    // Java: Entry<E> addElement(E e, Entry<E> entry)
    pub fn addElement(&mut self) -> Option<Box<LazyListEntry>> {
      // Java: this.indexedList = null;
      todo!();
      // Java: Entry<E> newEntry = new Entry<E>(e, entry, entry.previous);
      todo!();
      // Java: newEntry.previous.next = newEntry;
      todo!();
      // Java: newEntry.next.previous = newEntry;
      todo!();
      // Java: ++this.size;
      todo!();
      // Java: ++this.modCount;
      todo!();
      // Java: return newEntry;
      todo!();
    }

    // Java: Entry<E> getEntry(int index)
    pub fn getEntry(&mut self) -> Option<Box<LazyListEntry>> {
      // Java: if (index >= 0 && index < this.size)
      todo!();
      {
         // Java: if (index == 0)
      todo!();
         {
            // Java: return this.header.next;
      todo!();
         }
         // Java: else
      todo!();
         {
            // Java: Entry<E> e = this.header;
      todo!();
            // Java: if (index < this.size >> 1)
      todo!();
            {
               // Java: for(int i = 0; i <= index; ++i)
      todo!();
               {
                  // Java: e = e.next;
      todo!();
               }
            }
            // Java: else
      todo!();
            {
               // Java: for(int i = this.size; i > index; --i)
      todo!();
               {
                  // Java: e = e.previous;
      todo!();
               }
            }
            // Java: return e;
      todo!();
         }
      }
      // Java: else
      todo!();
      {
         // Java: throw new IndexOutOfBoundsException(MessageFormat.format("Index: {0}, Size: {1}", index, this.size));
      todo!();
      }
    }

    // Java: Entry<E> getEntryHeader(int index)
    pub fn getEntryHeader(&mut self) -> Option<Box<LazyListEntry>> {
      // Java: if (index >= 0 && index <= this.size)
      todo!();
      {
         // Java: if (index == 0)
      todo!();
         {
            // Java: return this.header.next;
      todo!();
         }
         // Java: else
      todo!();
         {
            // Java: Entry<E> e = this.header;
      todo!();
            // Java: if (index < this.size >> 1)
      todo!();
            {
               // Java: for(int i = 0; i < index; ++i)
      todo!();
               {
                  // Java: e = e.next;
      todo!();
               }
            }
            // Java: else
      todo!();
            {
               // Java: for(int i = this.size; i > index; --i)
      todo!();
               {
                  // Java: e = e.previous;
      todo!();
               }
            }
            // Java: return e;
      todo!();
         }
      }
      // Java: else
      todo!();
      {
         // Java: throw new IndexOutOfBoundsException(MessageFormat.format("Index: {0}, Size: {1}", index, this.size));
      todo!();
      }
    }

    // Java: Range<E> getRange(int fromIndex, int toIndex)
    pub fn getRange(&mut self) -> Option<Box<LazyListRange>> {
      // Java: if (fromIndex >= 0 && toIndex <= this.size && fromIndex <= toIndex)
      todo!();
      {
         // Java: int[] length = new int[]{fromIndex, toIndex - fromIndex, this.size - toIndex};
      todo!();
         // Java: Range<E> range = new Range<E>();
      todo!();
         // Java: if (length[0] < length[2])
      todo!();
         {
            // Java: range.lower = this.header;
      todo!();
            // Java: for(int i = 0; i <= fromIndex; ++i)
      todo!();
            {
               // Java: range.lower = range.lower.next;
      todo!();
            }
            // Java: if (length[1] < length[2])
      todo!();
            {
               // Java: range.upper = range.lower;
      todo!();
               // Java: for(int i = fromIndex; i <= toIndex; ++i)
      todo!();
               {
                  // Java: range.upper = range.upper.next;
      todo!();
               }
            }
            // Java: else
      todo!();
            {
               // Java: range.upper = this.header;
      todo!();
               // Java: for(int i = this.size; i > toIndex; --i)
      todo!();
               {
                  // Java: range.upper = range.upper.previous;
      todo!();
               }
            }
         }
         // Java: else
      todo!();
         {
            // Java: range.upper = this.header;
      todo!();
            // Java: for(int i = this.size; i > toIndex; --i)
      todo!();
            {
               // Java: range.upper = range.upper.previous;
      todo!();
            }
            // Java: if (length[0] < length[1])
      todo!();
            {
               // Java: range.lower = this.header;
      todo!();
               // Java: for(int i = 0; i <= fromIndex; ++i)
      todo!();
               {
                  // Java: range.lower = range.lower.next;
      todo!();
               }
            }
            // Java: else
      todo!();
            {
               // Java: range.lower = range.upper;
      todo!();
               // Java: for(int i = toIndex; i > fromIndex; --i)
      todo!();
               {
                  // Java: range.lower = range.lower.previous;
      todo!();
               }
            }
         }
         // Java: return range;
      todo!();
      }
      // Java: else
      todo!();
      {
         // Java: throw new IndexOutOfBoundsException(MessageFormat.format("FromIndex: {0}, ToIndex: {1}, Size: {2}", fromIndex, toIndex, this.size));
      todo!();
      }
    }

    // Java: E removeEntry(Entry<E> entry)
    pub fn removeEntry(&mut self) -> Option<Box<dyn std::any::Any>> {
      // Java: if (entry == this.header)
      todo!();
      {
         // Java: throw new NoSuchElementException();
      todo!();
      }
      // Java: else
      todo!();
      {
         // Java: this.indexedList = null;
      todo!();
         // Java: entry.previous.next = entry.next;
      todo!();
         // Java: entry.next.previous = entry.previous;
      todo!();
         // Java: --this.size;
      todo!();
         // Java: ++this.modCount;
      todo!();
         // Java: return entry.element;
      todo!();
      }
    }

    // Java: void writeObject(ObjectOutputStream stream)
    pub fn writeObject(&mut self) {
      // Java: stream.defaultWriteObject();
      todo!();
      // Java: stream.writeInt(this.size);
      todo!();
      // Java: for(Entry<E> entry = this.header.next; entry != this.header; entry = entry.next)
      todo!();
      {
         // Java: stream.writeObject(entry.element);
      todo!();
      }
    }

    // Java: void readObject(ObjectInputStream stream)
    pub fn readObject(&mut self) {
      // Java: stream.defaultReadObject();
      todo!();
      // Java: CloneHelper.setFinalField(LazyList.class, this, "header", new Entry(null, (Entry)null, (Entry)null));
      todo!();
      // Java: assert this.header != null;
      todo!();
      // Java: this.header.next = this.header.previous = this.header;
      todo!();
      // Java: int size = stream.readInt();
      todo!();
      // Java: E[] tempIndexedList = (E[])(new Object[size]);
      todo!();
      // Java: for(int i = 0; i < size; ++i)
      todo!();
      {
         // Java: E element = (E)stream.readObject();
      todo!();
         // Java: this.addElement(element, this.header);
      todo!();
         // Java: tempIndexedList[i] = element;
      todo!();
      }
      // Java: this.indexedList = tempIndexedList;
      todo!();
    }

}

// Java: public class Entry
impl LazyListEntry {
}

// Java: public class Range
impl LazyListRange {
    // Java: Range()
    pub fn Range(&mut self) {
         // Java: super();
      todo!();
    }

}

// Java: public class LazyListIterator
impl LazyListIterator {
    // Java: boolean hasNext()
    pub fn hasNext(&mut self) -> bool {
         // Java: return this.indexNext != LazyList.this.size;
      todo!();
    }

    // Java: E next()
    pub fn next(&mut self) -> Option<Box<dyn std::any::Any>> {
         // Java: this.checkForComodification();
      todo!();
         // Java: if (this.indexNext == LazyList.this.size)
      todo!();
         {
            // Java: throw new NoSuchElementException();
      todo!();
         }
         // Java: else
      todo!();
         {
            // Java: this.lastReturned = this.next;
      todo!();
            // Java: this.next = this.next.next;
      todo!();
            // Java: ++this.indexNext;
      todo!();
            // Java: return this.lastReturned.element;
      todo!();
         }
    }

    // Java: boolean hasPrevious()
    pub fn hasPrevious(&mut self) -> bool {
         // Java: return this.indexNext != 0;
      todo!();
    }

    // Java: E previous()
    pub fn previous(&mut self) -> Option<Box<dyn std::any::Any>> {
         // Java: this.checkForComodification();
      todo!();
         // Java: if (this.indexNext == 0)
      todo!();
         {
            // Java: throw new NoSuchElementException();
      todo!();
         }
         // Java: else
      todo!();
         {
            // Java: this.next = this.next.previous;
      todo!();
            // Java: this.lastReturned = this.next;
      todo!();
            // Java: --this.indexNext;
      todo!();
            // Java: return this.lastReturned.element;
      todo!();
         }
    }

    // Java: int nextIndex()
    pub fn nextIndex(&mut self) -> i32 {
         // Java: return this.indexNext;
      todo!();
    }

    // Java: int previousIndex()
    pub fn previousIndex(&mut self) -> i32 {
         // Java: return this.indexNext - 1;
      todo!();
    }

    // Java: void remove()
    pub fn remove_2(&mut self) {
         // Java: this.checkForOutside();
      todo!();
         // Java: this.checkForComodification();
      todo!();
         // Java: Entry<E> lastNext = this.lastReturned.next;
      todo!();
         // Java: LazyList.this.removeEntry(this.lastReturned);
      todo!();
         // Java: if (this.next == this.lastReturned)
      todo!();
         {
            // Java: this.next = lastNext;
      todo!();
         }
         // Java: else
      todo!();
         {
            // Java: --this.indexNext;
      todo!();
         }
         // Java: this.lastReturned = LazyList.this.header;
      todo!();
         // Java: ++this.expectedModCount;
      todo!();
    }

    // Java: void set(E e)
    pub fn set_2(&mut self) {
         // Java: this.checkForOutside();
      todo!();
         // Java: this.checkForComodification();
      todo!();
         // Java: this.lastReturned.element = e;
      todo!();
    }

    // Java: void add(E e)
    pub fn add_3(&mut self) {
         // Java: this.checkForComodification();
      todo!();
         // Java: this.lastReturned = LazyList.this.header;
      todo!();
         // Java: LazyList.this.addElement(e, this.next);
      todo!();
         // Java: ++this.indexNext;
      todo!();
         // Java: ++this.expectedModCount;
      todo!();
    }

    // Java: void checkForComodification()
    pub fn checkForComodification(&mut self) {
         // Java: if (LazyList.this.modCount != this.expectedModCount)
      todo!();
         {
            // Java: throw new ConcurrentModificationException();
      todo!();
         }
    }

    // Java: void checkForOutside()
    pub fn checkForOutside(&mut self) {
         // Java: if (this.lastReturned == LazyList.this.header)
      todo!();
         {
            // Java: throw new IllegalStateException();
      todo!();
         }
    }

}

// impl Serializable for LazyList {}
