// Java: import java.io.IOException
// Java: import java.io.InvalidObjectException
// Java: import java.io.ObjectInputStream
// Java: import java.io.ObjectOutputStream
// Java: import java.io.Serializable
// Java: import java.lang.reflect.Array
// Java: import java.util.AbstractList
// Java: import java.util.Arrays
// Java: import java.util.Collection
// Java: import java.util.ConcurrentModificationException
// Java: import java.util.Iterator
// Java: import java.util.List
// Java: import java.util.NoSuchElementException
// Java: import java.util.RandomAccess

use std::any::Any;
use std::sync::Arc;

type Element = Option<Arc<dyn Any + Send + Sync>>;

// Java: public class ArrayListStack extends AbstractList implements Cloneable, Serializable, RandomAccess
pub struct ArrayListStack {
    // Java field: Object[] EMPTY_OBJECT_ARRAY
    // Java field: int MIN_CAPACITY_INCREMENT
    // Java field: int size
    pub(crate) size: usize,
    // Java field: Object[] array
    pub(crate) array: Vec<Element>,
    // Java field: long serialVersionUID
    mod_count: i32,
}

pub(crate) const MIN_CAPACITY_INCREMENT: i32 = 12;
pub(crate) const SERIAL_VERSION_UID: i64 = 8683452581122892189i64;

// Java: public class ArrayListIterator
pub struct ArrayListIterator {
    // Java field: int remaining
    pub(crate) remaining: i32,
    // Java field: int removalIndex
    pub(crate) removalIndex: i32,
    // Java field: int expectedModCount
    pub(crate) expectedModCount: i32,
}

// Java: public class ArrayListStack extends AbstractList implements Cloneable, Serializable, RandomAccess
impl ArrayListStack {
    // Java: E peek()
    pub fn peek(&self) -> &Element {
        // Java: return (E)this.array[this.size - 1];
        &self.array[self.size - 1]
    }

    // Java: E pop()
    pub fn pop(&mut self) -> Element {
        // Java: Object[] a = this.array;
        // Java: int s = this.size;
        let s = self.size;
        // Java: E result = (E)a[s - 1];
        let result = self.array[s - 1].take();
        // Java: a[s - 1] = null;
        self.array[s - 1] = None;
        // Java: --this.size;
        self.size -= 1;
        // Java: ++this.modCount;
        self.mod_count += 1;
        // Java: return result;
        result
    }

    // Java: ArrayListStack(int capacity)
    pub fn new_with_capacity(capacity: usize) -> Self {
        // Java: super();
        // Java: if (capacity < 0)
        // (Rust: usize is always >= 0)
        {
            // Java: throw new IllegalArgumentException();
        }
        // Java: else
        {
            // Java: this.array = capacity == 0 ? EMPTY_OBJECT_ARRAY : new Object[capacity];
            ArrayListStack {
                size: 0,
                array: if capacity == 0 { Vec::new() } else { vec![None; capacity] },
                mod_count: 0,
            }
        }
    }

    // Java: ArrayListStack()
    pub fn new() -> Self {
        // Java: super();
        // Java: this.array = EMPTY_OBJECT_ARRAY;
        ArrayListStack { size: 0, array: Vec::new(), mod_count: 0 }
    }

    // Java: ArrayListStack(Collection<? extends E> collection)
    pub fn new_from(collection: Vec<Element>) -> Self {
        // Java: super();
        // Java: Object[] a = collection.toArray();
        let len = collection.len();
        // Java: if (a.getClass() != Object[].class)
        {
            // Java: Object[] newArray = new Object[a.length];
            // Java: System.arraycopy(a, 0, newArray, 0, a.length);
            // Java: a = newArray;
        }
        // Java: this.array = a;
        // Java: this.size = a.length;
        ArrayListStack { size: len, array: collection, mod_count: 0 }
    }

    // Java: boolean add(E object)
    pub fn add(&mut self, object: Element) -> bool {
        // Java: Object[] a = this.array;
        // Java: int s = this.size;
        let s = self.size;
        // Java: if (s == a.length)
        if s == self.array.len() {
            // Java: Object[] newArray = new Object[s + (s < 6 ? 12 : s >> 1)];
            let new_cap = s + if s < 6 { 12 } else { s >> 1 };
            // Java: System.arraycopy(a, 0, newArray, 0, s);
            self.array.resize_with(new_cap, || None);
            // Java: a = newArray;
            // Java: this.array = newArray;
        }
        // Java: a[s] = object;
        self.array[s] = object;
        // Java: this.size = s + 1;
        self.size = s + 1;
        // Java: ++this.modCount;
        self.mod_count += 1;
        // Java: return true;
        true
    }

    // Java: void add(int index, E object)
    pub fn add_at(&mut self, index: usize, object: Element) {
        // Java: Object[] a = this.array;
        // Java: int s = this.size;
        let s = self.size;
        // Java: if (index > s || index < 0)
        if index > s {
            // Java: throwIndexOutOfBoundsException(index, s);
            Self::throwIndexOutOfBoundsException(index, s);
        }
        // Java: if (s < a.length)
        if s < self.array.len() {
            // Java: System.arraycopy(a, index, a, index + 1, s - index);
            let mut i = s;
            while i > index { self.array[i] = self.array[i - 1].take(); i -= 1; }
        }
        // Java: else
        else {
            // Java: Object[] newArray = new Object[newCapacity(s)];
            let new_cap = Self::newCapacity(s);
            self.array.resize_with(new_cap, || None);
            // Java: System.arraycopy(a, 0, newArray, 0, index);
            // Java: System.arraycopy(a, index, newArray, index + 1, s - index);
            let mut i = s;
            while i > index { self.array[i] = self.array[i - 1].take(); i -= 1; }
            // Java: a = newArray;
            // Java: this.array = newArray;
        }
        // Java: a[index] = object;
        self.array[index] = object;
        // Java: this.size = s + 1;
        self.size = s + 1;
        // Java: ++this.modCount;
        self.mod_count += 1;
    }

    // Java: static int newCapacity(int currentCapacity)
    fn newCapacity(currentCapacity: usize) -> usize {
        // Java: int increment = currentCapacity < 6 ? 12 : currentCapacity >> 1;
        let increment = if currentCapacity < 6 { 12 } else { currentCapacity >> 1 };
        // Java: return currentCapacity + increment;
        currentCapacity + increment
    }

    // Java: boolean addAll(Collection<? extends E> collection)
    pub fn addAll(&mut self, collection: Vec<Element>) -> bool {
        // Java: Object[] newPart = collection.toArray();
        // Java: int newPartSize = newPart.length;
        let newPartSize = collection.len();
        // Java: if (newPartSize == 0)
        if newPartSize == 0 {
            // Java: return false;
            return false;
        }
        // Java: else
        {
            // Java: Object[] a = this.array;
            // Java: int s = this.size;
            let s = self.size;
            // Java: int newSize = s + newPartSize;
            let newSize = s + newPartSize;
            // Java: if (newSize > a.length)
            if newSize > self.array.len() {
                // Java: int newCapacity = newCapacity(newSize - 1);
                let new_cap = Self::newCapacity(newSize - 1);
                // Java: Object[] newArray = new Object[newCapacity];
                // Java: System.arraycopy(a, 0, newArray, 0, s);
                self.array.resize_with(new_cap, || None);
                // Java: a = newArray;
                // Java: this.array = newArray;
            }
            // Java: System.arraycopy(newPart, 0, a, s, newPartSize);
            for (j, item) in collection.into_iter().enumerate() {
                self.array[s + j] = item;
            }
            // Java: this.size = newSize;
            self.size = newSize;
            // Java: ++this.modCount;
            self.mod_count += 1;
            // Java: return true;
            true
        }
    }

    // Java: boolean addAll(int index, Collection<? extends E> collection)
    pub fn addAll_at(&mut self, index: usize, collection: Vec<Element>) -> bool {
        // Java: int s = this.size;
        let s = self.size;
        // Java: if (index > s || index < 0)
        if index > s {
            // Java: throwIndexOutOfBoundsException(index, s);
            Self::throwIndexOutOfBoundsException(index, s);
        }
        // Java: Object[] newPart = collection.toArray();
        // Java: int newPartSize = newPart.length;
        let newPartSize = collection.len();
        // Java: if (newPartSize == 0)
        if newPartSize == 0 {
            // Java: return false;
            return false;
        }
        // Java: else
        {
            // Java: Object[] a = this.array;
            // Java: int newSize = s + newPartSize;
            let newSize = s + newPartSize;
            // Java: if (newSize <= a.length)
            if newSize <= self.array.len() {
                // Java: System.arraycopy(a, index, a, index + newPartSize, s - index);
                let mut i = s;
                while i > index {
                    self.array[i + newPartSize - 1] = self.array[i - 1].take();
                    i -= 1;
                }
            }
            // Java: else
            else {
                // Java: int newCapacity = newCapacity(newSize - 1);
                let new_cap = Self::newCapacity(newSize - 1);
                // Java: Object[] newArray = new Object[newCapacity];
                self.array.resize_with(new_cap, || None);
                // Java: System.arraycopy(a, 0, newArray, 0, index);
                // Java: System.arraycopy(a, index, newArray, index + newPartSize, s - index);
                let mut i = s;
                while i > index {
                    self.array[i + newPartSize - 1] = self.array[i - 1].take();
                    i -= 1;
                }
                // Java: a = newArray;
                // Java: this.array = newArray;
            }
            // Java: System.arraycopy(newPart, 0, a, index, newPartSize);
            for (j, item) in collection.into_iter().enumerate() {
                self.array[index + j] = item;
            }
            // Java: this.size = newSize;
            self.size = newSize;
            // Java: ++this.modCount;
            self.mod_count += 1;
            // Java: return true;
            true
        }
    }

    // Java: static void throwIndexOutOfBoundsException(int index, int size)
    fn throwIndexOutOfBoundsException(index: usize, size: usize) {
        // Java: throw new IndexOutOfBoundsException("Invalid index " + index + ", size is " + size);
        panic!("Invalid index {}, size is {}", index, size);
    }

    // Java: void clear()
    pub fn clear(&mut self) {
        // Java: if (this.size != 0)
        if self.size != 0 {
            // Java: Arrays.fill(this.array, 0, this.size, null);
            for i in 0..self.size { self.array[i] = None; }
            // Java: this.size = 0;
            self.size = 0;
            // Java: ++this.modCount;
            self.mod_count += 1;
        }
    }

    // Java: Object clone()
    pub fn clone_stack(&self) -> ArrayListStack {
        // Java: try
        {
            // Java: ArrayListStack<?> result = (ArrayListStack)super.clone();
            // Java: result.array = this.array.clone();
            // Java: return result;
            return ArrayListStack {
                size: self.size,
                array: self.array.clone(),
                mod_count: self.mod_count,
            };
        }
        // Java: catch (CloneNotSupportedException var2)
        // Java: throw new AssertionError();
    }

    // Java: void ensureCapacity(int minimumCapacity)
    pub fn ensureCapacity(&mut self, minimumCapacity: usize) {
        // Java: Object[] a = this.array;
        // Java: if (a.length < minimumCapacity)
        if self.array.len() < minimumCapacity {
            // Java: Object[] newArray = new Object[minimumCapacity];
            // Java: System.arraycopy(a, 0, newArray, 0, this.size);
            self.array.resize_with(minimumCapacity, || None);
            // Java: this.array = newArray;
            // Java: ++this.modCount;
            self.mod_count += 1;
        }
    }

    // Java: E get(int index)
    pub fn get(&self, index: usize) -> &Element {
        // Java: if (index >= this.size)
        if index >= self.size {
            // Java: throwIndexOutOfBoundsException(index, this.size);
            Self::throwIndexOutOfBoundsException(index, self.size);
        }
        // Java: return (E)this.array[index];
        &self.array[index]
    }

    // Java: int size()
    pub fn size(&self) -> usize {
        // Java: return this.size;
        self.size
    }

    // Java: boolean isEmpty()
    pub fn isEmpty(&self) -> bool {
        // Java: return this.size == 0;
        self.size == 0
    }

    // Java: boolean contains(Object object)
    pub fn contains_element(&self, object: &Element) -> bool {
        // Java: Object[] a = this.array;
        // Java: int s = this.size;
        // Java: if (object != null)
        if object.is_some() {
            // Java: for(int i = 0; i < s; ++i)
            for i in 0..self.size {
                // Java: if (object.equals(a[i]))
                if std::ptr::eq(object as *const _, &self.array[i] as *const _) {
                    // Java: return true;
                    return true;
                }
            }
        }
        // Java: else
        else {
            // Java: for(int i = 0; i < s; ++i)
            for i in 0..self.size {
                // Java: if (a[i] == null)
                if self.array[i].is_none() {
                    // Java: return true;
                    return true;
                }
            }
        }
        // Java: return false;
        false
    }

    // Java: int indexOf(Object object)
    pub fn indexOf(&self, object: &Element) -> i32 {
        // Java: Object[] a = this.array;
        // Java: int s = this.size;
        // Java: if (object != null)
        if object.is_some() {
            // Java: for(int i = 0; i < s; ++i)
            for i in 0..self.size {
                // Java: if (object.equals(a[i]))
                if std::ptr::eq(object as *const _, &self.array[i] as *const _) {
                    // Java: return i;
                    return i as i32;
                }
            }
        }
        // Java: else
        else {
            // Java: for(int i = 0; i < s; ++i)
            for i in 0..self.size {
                // Java: if (a[i] == null)
                if self.array[i].is_none() {
                    // Java: return i;
                    return i as i32;
                }
            }
        }
        // Java: return -1;
        -1
    }

    // Java: int lastIndexOf(Object object)
    pub fn lastIndexOf(&self, object: &Element) -> i32 {
        // Java: Object[] a = this.array;
        // Java: if (object != null)
        if object.is_some() {
            // Java: for(int i = this.size - 1; i >= 0; --i)
            let mut i = self.size as i32 - 1;
            while i >= 0 {
                // Java: if (object.equals(a[i]))
                if std::ptr::eq(object as *const _, &self.array[i as usize] as *const _) {
                    // Java: return i;
                    return i;
                }
                i -= 1;
            }
        }
        // Java: else
        else {
            // Java: for(int i = this.size - 1; i >= 0; --i)
            let mut i = self.size as i32 - 1;
            while i >= 0 {
                // Java: if (a[i] == null)
                if self.array[i as usize].is_none() {
                    // Java: return i;
                    return i;
                }
                i -= 1;
            }
        }
        // Java: return -1;
        -1
    }

    // Java: E remove(int index)
    pub fn remove_at(&mut self, index: usize) -> Element {
        // Java: Object[] a = this.array;
        // Java: int s = this.size;
        let mut s = self.size;
        // Java: if (index >= s)
        if index >= s {
            // Java: throwIndexOutOfBoundsException(index, s);
            Self::throwIndexOutOfBoundsException(index, s);
        }
        // Java: E result = (E)a[index];
        let result = self.array[index].take();
        // Java: int var10001 = index + 1;
        // Java: --s;
        s -= 1;
        // Java: System.arraycopy(a, var10001, a, index, s - index);
        for i in index..s { self.array[i] = self.array[i + 1].take(); }
        // Java: a[s] = null;
        self.array[s] = None;
        // Java: this.size = s;
        self.size = s;
        // Java: ++this.modCount;
        self.mod_count += 1;
        // Java: return result;
        result
    }

    // Java: boolean remove(Object object)
    pub fn remove_obj(&mut self, object: &Element) -> bool {
        // Java: Object[] a = this.array;
        // Java: int s = this.size;
        let mut s = self.size;
        // Java: if (object != null)
        if object.is_some() {
            // Java: for(int i = 0; i < s; ++i)
            for i in 0..s {
                // Java: if (object.equals(a[i]))
                if std::ptr::eq(object as *const _, &self.array[i] as *const _) {
                    // Java: int var10001 = i + 1;
                    // Java: --s;
                    s -= 1;
                    // Java: System.arraycopy(a, var10001, a, i, s - i);
                    for j in i..s { self.array[j] = self.array[j + 1].take(); }
                    // Java: a[s] = null;
                    self.array[s] = None;
                    // Java: this.size = s;
                    self.size = s;
                    // Java: ++this.modCount;
                    self.mod_count += 1;
                    // Java: return true;
                    return true;
                }
            }
        }
        // Java: else
        else {
            // Java: for(int i = 0; i < s; ++i)
            for i in 0..s {
                // Java: if (a[i] == null)
                if self.array[i].is_none() {
                    // Java: int var8 = i + 1;
                    // Java: --s;
                    s -= 1;
                    // Java: System.arraycopy(a, var8, a, i, s - i);
                    for j in i..s { self.array[j] = self.array[j + 1].take(); }
                    // Java: a[s] = null;
                    self.array[s] = None;
                    // Java: this.size = s;
                    self.size = s;
                    // Java: ++this.modCount;
                    self.mod_count += 1;
                    // Java: return true;
                    return true;
                }
            }
        }
        // Java: return false;
        false
    }

    // Java: void removeRange(int fromIndex, int toIndex)
    pub fn removeRange(&mut self, fromIndex: usize, toIndex: usize) {
        // Java: if (fromIndex != toIndex)
        if fromIndex != toIndex {
            // Java: Object[] a = this.array;
            // Java: int s = this.size;
            let s = self.size;
            // Java: if (fromIndex >= s)
            if fromIndex >= s {
                // Java: throw new IndexOutOfBoundsException("fromIndex " + fromIndex + " >= size " + this.size);
                panic!("fromIndex {} >= size {}", fromIndex, s);
            }
            // Java: else if (toIndex > s)
            else if toIndex > s {
                // Java: throw new IndexOutOfBoundsException("toIndex " + toIndex + " > size " + this.size);
                panic!("toIndex {} > size {}", toIndex, s);
            }
            // Java: else if (fromIndex > toIndex)
            else if fromIndex > toIndex {
                // Java: throw new IndexOutOfBoundsException("fromIndex " + fromIndex + " > toIndex " + toIndex);
                panic!("fromIndex {} > toIndex {}", fromIndex, toIndex);
            }
            // Java: else
            else {
                // Java: System.arraycopy(a, toIndex, a, fromIndex, s - toIndex);
                for i in 0..(s - toIndex) { self.array[fromIndex + i] = self.array[toIndex + i].take(); }
                // Java: int rangeSize = toIndex - fromIndex;
                let rangeSize = toIndex - fromIndex;
                // Java: Arrays.fill(a, s - rangeSize, s, null);
                for i in (s - rangeSize)..s { self.array[i] = None; }
                // Java: this.size = s - rangeSize;
                self.size = s - rangeSize;
                // Java: ++this.modCount;
                self.mod_count += 1;
            }
        }
    }

    // Java: E set(int index, E object)
    pub fn set(&mut self, index: usize, object: Element) -> Element {
        // Java: Object[] a = this.array;
        // Java: if (index >= this.size)
        if index >= self.size {
            // Java: throwIndexOutOfBoundsException(index, this.size);
            Self::throwIndexOutOfBoundsException(index, self.size);
        }
        // Java: E result = (E)a[index];
        let result = self.array[index].take();
        // Java: a[index] = object;
        self.array[index] = object;
        // Java: return result;
        result
    }

    // Java: Object[] toArray()
    pub fn toArray(&self) -> Vec<Element> {
        // Java: int s = this.size;
        let s = self.size;
        // Java: Object[] result = new Object[s];
        // Java: System.arraycopy(this.array, 0, result, 0, s);
        let result = self.array[..s].to_vec();
        // Java: return result;
        result
    }

    // Java: public <T> T[] toArray(T[] contents)
    pub fn toArray_typed(&self, contents: &mut Vec<Element>) -> Vec<Element> {
        // Java: int s = this.size;
        let s = self.size;
        // Java: if (contents.length < s)
        if contents.len() < s {
            // Java: T[] newArray = (T[])((Object[])Array.newInstance(contents.getClass().getComponentType(), s));
            // Java: contents = newArray;
            contents.resize_with(s, || None);
        }
        // Java: System.arraycopy(this.array, 0, contents, 0, s);
        for i in 0..s { contents[i] = self.array[i].clone(); }
        // Java: if (contents.length > s)
        if contents.len() > s {
            // Java: contents[s] = null;
            contents[s] = None;
        }
        // Java: return contents;
        contents.clone()
    }

    // Java: void trimToSize()
    pub fn trimToSize(&mut self) {
        // Java: int s = this.size;
        let s = self.size;
        // Java: if (s != this.array.length)
        if s != self.array.len() {
            // Java: if (s == 0)
            if s == 0 {
                // Java: this.array = EMPTY_OBJECT_ARRAY;
                self.array = Vec::new();
            }
            // Java: else
            else {
                // Java: Object[] newArray = new Object[s];
                // Java: System.arraycopy(this.array, 0, newArray, 0, s);
                self.array.truncate(s);
                // Java: this.array = newArray;
                self.array.shrink_to_fit();
            }
            // Java: ++this.modCount;
            self.mod_count += 1;
        }
    }

    // Java: Iterator<E> iterator()
    pub fn iter(&self) -> std::slice::Iter<'_, Element> {
        // Java: return new ArrayListIterator();
        self.array[..self.size].iter()
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
        // Java: Object[] a = this.array;
        // Java: int hashCode = 1;
        let mut hash_code: i32 = 1;
        // Java: int i = 0;
        // Java: for(int s = this.size; i < s; ++i)
        for i in 0..self.size {
            // Java: Object e = a[i];
            let e = &self.array[i];
            // Java: hashCode = 31 * hashCode + (e == null ? 0 : e.hashCode());
            hash_code = 31i32.wrapping_mul(hash_code).wrapping_add(
                if e.is_none() { 0 } else { i as i32 }
            );
        }
        // Java: return hashCode;
        hash_code
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, other: &ArrayListStack) -> bool {
        // Java: if (o == this)
        if std::ptr::eq(self, other) {
            // Java: return true;
            return true;
        }
        // Java: else if (!(o instanceof List))
        // (Rust: type is statically known)
        {
            // Java: return false;
            // (skipped: Rust type is always ArrayListStack)
        }
        // Java: else
        {
            // Java: List<?> that = (List)o;
            // Java: int s = this.size;
            let s = self.size;
            // Java: if (that.size() != s)
            if other.size != s {
                // Java: return false;
                return false;
            }
            // Java: else
            {
                // Java: Object[] a = this.array;
                // Java: if (that instanceof RandomAccess)
                {
                    // Java: for(int i = 0; i < s; ++i)
                    for i in 0..s {
                        // Java: Object eThis = a[i];
                        let e_this = &self.array[i];
                        // Java: Object ethat = that.get(i);
                        let e_that = &other.array[i];
                        // Java: if (eThis == null)
                        if e_this.is_none() {
                            // Java: if (ethat != null)
                            if e_that.is_some() {
                                // Java: return false;
                                return false;
                            }
                        }
                        // Java: else if (!eThis.equals(ethat))
                        else if e_that.is_none() {
                            // Java: return false;
                            return false;
                        }
                    }
                }
                // Java: else
                {
                    // Java: Iterator<?> it = that.iterator();
                    // Java: for(int i = 0; i < s; ++i)
                    for i in 0..s {
                        // Java: Object eThis = a[i];
                        let e_this = &self.array[i];
                        // Java: Object eThat = it.next();
                        let e_that = &other.array[i];
                        // Java: if (eThis == null)
                        if e_this.is_none() {
                            // Java: if (eThat != null)
                            if e_that.is_some() {
                                // Java: return false;
                                return false;
                            }
                        }
                        // Java: else if (!eThis.equals(eThat))
                        else if e_that.is_none() {
                            // Java: return false;
                            return false;
                        }
                    }
                }
                // Java: return true;
                true
            }
        }
    }

    // Java: void writeObject(ObjectOutputStream stream)
    // (Rust: serialization handled differently)
    // Java: stream.defaultWriteObject();
    // Java: stream.writeInt(this.array.length);
    // Java: for(int i = 0; i < this.size; ++i)
    // Java: stream.writeObject(this.array[i]);

    // Java: void readObject(ObjectInputStream stream)
    // (Rust: deserialization handled differently)
    // Java: stream.defaultReadObject();
    // Java: int cap = stream.readInt();
    // Java: if (cap < this.size)
    // Java: throw new InvalidObjectException("Capacity: " + cap + " < size: " + this.size);
    // Java: else
    // Java: this.array = cap == 0 ? EMPTY_OBJECT_ARRAY : new Object[cap];
    // Java: for(int i = 0; i < this.size; ++i)
    // Java: this.array[i] = stream.readObject();
}

// Java: public class ArrayListIterator
impl ArrayListIterator {
    // Java: ArrayListIterator()
    pub fn new(size: i32, mod_count: i32) -> Self {
        // Java: super();
        // Java: this.remaining = ArrayListStack.this.size;
        // Java: this.removalIndex = -1;
        // Java: this.expectedModCount = ArrayListStack.this.modCount;
        ArrayListIterator {
            remaining: size,
            removalIndex: -1,
            expectedModCount: mod_count,
        }
    }

    // Java: boolean hasNext()
    pub fn hasNext(&self) -> bool {
        // Java: return this.remaining != 0;
        self.remaining != 0
    }

    // Java: E next()
    pub fn next(&mut self, list: &ArrayListStack) -> Element {
        // Java: ArrayListStack<E> ourList = ArrayListStack.this;
        // Java: int rem = this.remaining;
        let rem = self.remaining;
        // Java: if (ourList.modCount != this.expectedModCount)
        if list.mod_count != self.expectedModCount {
            // Java: throw new ConcurrentModificationException();
            panic!("ConcurrentModificationException");
        }
        // Java: else if (rem == 0)
        else if rem == 0 {
            // Java: throw new NoSuchElementException();
            panic!("NoSuchElementException");
        }
        // Java: else
        else {
            // Java: this.remaining = rem - 1;
            self.remaining = rem - 1;
            // Java: return (E)ourList.array[this.removalIndex = ourList.size - rem];
            let idx = list.size as i32 - rem;
            self.removalIndex = idx;
            list.array[idx as usize].clone()
        }
    }

    // Java: void remove()
    pub fn remove(&mut self, list: &mut ArrayListStack) {
        // Java: Object[] a = ArrayListStack.this.array;
        // Java: int removalIdx = this.removalIndex;
        let removalIdx = self.removalIndex;
        // Java: if (ArrayListStack.this.modCount != this.expectedModCount)
        if list.mod_count != self.expectedModCount {
            // Java: throw new ConcurrentModificationException();
            panic!("ConcurrentModificationException");
        }
        // Java: else if (removalIdx < 0)
        else if removalIdx < 0 {
            // Java: throw new IllegalStateException();
            panic!("IllegalStateException");
        }
        // Java: else
        else {
            // Java: System.arraycopy(a, removalIdx + 1, a, removalIdx, this.remaining);
            let ridx = removalIdx as usize;
            for i in ridx..(ridx + self.remaining as usize) {
                list.array[i] = list.array[i + 1].take();
            }
            // Java: a[--ArrayListStack.this.size] = null;
            list.size -= 1;
            list.array[list.size] = None;
            // Java: this.removalIndex = -1;
            self.removalIndex = -1;
            // Java: this.expectedModCount = ++ArrayListStack.this.modCount;
            list.mod_count += 1;
            self.expectedModCount = list.mod_count;
        }
    }
}
