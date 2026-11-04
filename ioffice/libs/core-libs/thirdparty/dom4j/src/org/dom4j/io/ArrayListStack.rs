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

// Java field: Object[] EMPTY_OBJECT_ARRAY
// Java field: int MIN_CAPACITY_INCREMENT
pub(crate) const MIN_CAPACITY_INCREMENT: i32 = 12;
// Java: public class ArrayListStack extends AbstractList implements Cloneable, Serializable, RandomAccess
#[derive(Clone)]
pub struct ArrayListStack<E> {
    // Java field: int size
    // Java field: Object[] array
    pub(crate) data: Vec<E>,
    pub(crate) modCount: i32,
    // Java field: long serialVersionUID
}

// Java: public class ArrayListIterator
// Rust uses standard slice iterator instead of inner class
// Java field: int remaining
// Java field: int removalIndex
// Java field: int expectedModCount

// Java: public class ArrayListStack extends AbstractList implements Cloneable, Serializable, RandomAccess
impl<E: PartialEq + Clone> ArrayListStack<E> {
    // Java: E peek()
    pub fn peek(&self) -> &E {
      // Java: return (E)this.array[this.size - 1];
      &self.data[self.data.len() - 1]
    }

    // Java: E pop()
    pub fn pop(&mut self) -> E {
      // Java: Object[] a = this.array;
      // Java: int s = this.size;
      // Java: E result = (E)a[s - 1];
      // Java: a[s - 1] = null;
      // Java: --this.size;
      // Java: ++this.modCount;
      self.modCount += 1;
      // Java: return result;
      self.data.pop().unwrap()
    }

    // Java: ArrayListStack(int capacity)
    pub fn new_with_capacity(capacity: usize) -> Self {
      // Java: super();
      // Java: if (capacity < 0)
      {
         // Java: throw new IllegalArgumentException();
      }
      // Java: else
      {
         // Java: this.array = capacity == 0 ? EMPTY_OBJECT_ARRAY : new Object[capacity];
      }
      ArrayListStack { data: Vec::with_capacity(capacity), modCount: 0 }
    }

    // Java: ArrayListStack()
    pub fn new() -> Self {
      // Java: super();
      // Java: this.array = EMPTY_OBJECT_ARRAY;
      ArrayListStack { data: Vec::new(), modCount: 0 }
    }

    // Java: ArrayListStack(Collection<? extends E> collection)
    pub fn new_from(collection: Vec<E>) -> Self {
      // Java: super();
      let len = collection.len();
      // Java: Object[] a = collection.toArray();
      // Java: if (a.getClass() != Object[].class)
      {
         // Java: Object[] newArray = new Object[a.length];
         // Java: System.arraycopy(a, 0, newArray, 0, a.length);
         // Java: a = newArray;
      }
      // Java: this.array = a;
      // Java: this.size = a.length;
      ArrayListStack { data: collection, modCount: 0 }
    }

    // Java: boolean add(E object)
    pub fn add(&mut self, object: E) -> bool {
      // Java: Object[] a = this.array;
      // Java: int s = this.size;
      // Java: if (s == a.length)
      {
         // Java: Object[] newArray = new Object[s + (s < 6 ? 12 : s >> 1)];
         // Java: System.arraycopy(a, 0, newArray, 0, s);
         // Java: a = newArray;
         // Java: this.array = newArray;
      }
      // Java: a[s] = object;
      self.data.push(object);
      // Java: this.size = s + 1;
      // Java: ++this.modCount;
      self.modCount += 1;
      // Java: return true;
      true
    }

    // Java: void add(int index, E object)
    pub fn add_at(&mut self, index: usize, object: E) {
      // Java: Object[] a = this.array;
      // Java: int s = this.size;
      // Java: if (index > s || index < 0)
      if index > self.data.len() {
         // Java: throwIndexOutOfBoundsException(index, s);
         Self::throwIndexOutOfBoundsException(index, self.data.len());
      }
      // Java: if (s < a.length)
      {
         // Java: System.arraycopy(a, index, a, index + 1, s - index);
      }
      // Java: else
      {
         // Java: Object[] newArray = new Object[newCapacity(s)];
         // Java: System.arraycopy(a, 0, newArray, 0, index);
         // Java: System.arraycopy(a, index, newArray, index + 1, s - index);
         // Java: a = newArray;
         // Java: this.array = newArray;
      }
      // Java: a[index] = object;
      self.data.insert(index, object);
      // Java: this.size = s + 1;
      // Java: ++this.modCount;
      self.modCount += 1;
    }

    // Java: static int newCapacity(int currentCapacity)
    fn newCapacity(current_capacity: usize) -> usize {
      // Java: int increment = currentCapacity < 6 ? 12 : currentCapacity >> 1;
      let increment = if current_capacity < 6 { 12 } else { current_capacity >> 1 };
      // Java: return currentCapacity + increment;
      current_capacity + increment
    }

    // Java: boolean addAll(Collection<? extends E> collection)
    pub fn addAll(&mut self, collection: Vec<E>) -> bool {
      // Java: Object[] newPart = collection.toArray();
      // Java: int newPartSize = newPart.length;
      let new_part_size = collection.len();
      // Java: if (newPartSize == 0)
      if new_part_size == 0 {
         // Java: return false;
         return false;
      }
      // Java: else
      {
         // Java: Object[] a = this.array;
         // Java: int s = this.size;
         // Java: int newSize = s + newPartSize;
         // Java: if (newSize > a.length)
         {
            // Java: int newCapacity = newCapacity(newSize - 1);
            // Java: Object[] newArray = new Object[newCapacity];
            // Java: System.arraycopy(a, 0, newArray, 0, s);
            // Java: a = newArray;
            // Java: this.array = newArray;
         }
         // Java: System.arraycopy(newPart, 0, a, s, newPartSize);
         self.data.extend(collection);
         // Java: this.size = newSize;
         // Java: ++this.modCount;
         self.modCount += 1;
         // Java: return true;
         return true;
      }
    }

    // Java: boolean addAll(int index, Collection<? extends E> collection)
    pub fn addAll_at(&mut self, index: usize, collection: Vec<E>) -> bool {
      // Java: int s = this.size;
      let s = self.data.len();
      // Java: if (index > s || index < 0)
      if index > s {
         // Java: throwIndexOutOfBoundsException(index, s);
         Self::throwIndexOutOfBoundsException(index, s);
      }
      // Java: Object[] newPart = collection.toArray();
      // Java: int newPartSize = newPart.length;
      let new_part_size = collection.len();
      // Java: if (newPartSize == 0)
      if new_part_size == 0 {
         // Java: return false;
         return false;
      }
      // Java: else
      {
         // Java: Object[] a = this.array;
         // Java: int newSize = s + newPartSize;
         // Java: if (newSize <= a.length)
         {
            // Java: System.arraycopy(a, index, a, index + newPartSize, s - index);
         }
         // Java: else
         {
            // Java: int newCapacity = newCapacity(newSize - 1);
            // Java: Object[] newArray = new Object[newCapacity];
            // Java: System.arraycopy(a, 0, newArray, 0, index);
            // Java: System.arraycopy(a, index, newArray, index + newPartSize, s - index);
            // Java: a = newArray;
            // Java: this.array = newArray;
         }
         // Java: System.arraycopy(newPart, 0, a, index, newPartSize);
         let mut ins_idx = index;
         for item in collection {
             self.data.insert(ins_idx, item);
             ins_idx += 1;
         }
         // Java: this.size = newSize;
         // Java: ++this.modCount;
         self.modCount += 1;
         // Java: return true;
         return true;
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
      if !self.data.is_empty() {
         // Java: Arrays.fill(this.array, 0, this.size, null);
         // Java: this.size = 0;
         self.data.clear();
         // Java: ++this.modCount;
         self.modCount += 1;
      }
    }

    // Java: Object clone()
    #[allow(unreachable_code)]
    pub fn clone_stack(&self) -> Self {
      // Java: try
      {
         // Java: ArrayListStack<?> result = (ArrayListStack)super.clone();
         // Java: result.array = this.array.clone();
         // Java: return result;
         return self.clone();
      }
      // Java: catch (CloneNotSupportedException var2)
      {
         // Java: throw new AssertionError();
         unreachable!();
      }
    }

    // Java: void ensureCapacity(int minimumCapacity)
    pub fn ensureCapacity(&mut self, minimumCapacity: usize) {
      // Java: Object[] a = this.array;
      // Java: if (a.length < minimumCapacity)
      if self.data.capacity() < minimumCapacity {
         // Java: Object[] newArray = new Object[minimumCapacity];
         // Java: System.arraycopy(a, 0, newArray, 0, this.size);
         self.data.reserve(minimumCapacity - self.data.len());
         // Java: this.array = newArray;
         // Java: ++this.modCount;
         self.modCount += 1;
      }
    }

    // Java: E get(int index)
    pub fn get(&self, index: usize) -> &E {
      // Java: if (index >= this.size)
      if index >= self.data.len() {
         // Java: throwIndexOutOfBoundsException(index, this.size);
         Self::throwIndexOutOfBoundsException(index, self.data.len());
      }
      // Java: return (E)this.array[index];
      &self.data[index]
    }

    // Java: int size()
    pub fn size(&self) -> usize {
      // Java: return this.size;
      self.data.len()
    }

    // Java: boolean isEmpty()
    pub fn isEmpty(&self) -> bool {
      // Java: return this.size == 0;
      self.data.is_empty()
    }

    // Java: boolean contains(Object object)
    pub fn contains(&self, object: &E) -> bool {
      // Java: Object[] a = this.array;
      // Java: int s = this.size;
      // Java: if (object != null)
      {
         // Java: for(int i = 0; i < s; ++i)
         for item in &self.data {
            // Java: if (object.equals(a[i]))
            if object == item {
               // Java: return true;
               return true;
            }
         }
      }
      // Java: else
      {
         // Java: for(int i = 0; i < s; ++i)
         {
            // Java: if (a[i] == null)
            {
               // Java: return true;
            }
         }
      }
      // Java: return false;
      false
    }

    // Java: int indexOf(Object object)
    pub fn indexOf(&self, object: &E) -> i32 {
      // Java: Object[] a = this.array;
      // Java: int s = this.size;
      // Java: if (object != null)
      {
         // Java: for(int i = 0; i < s; ++i)
         for (i, item) in self.data.iter().enumerate() {
            // Java: if (object.equals(a[i]))
            if object == item {
               // Java: return i;
               return i as i32;
            }
         }
      }
      // Java: else
      {
         // Java: for(int i = 0; i < s; ++i)
         {
            // Java: if (a[i] == null)
            {
               // Java: return i;
            }
         }
      }
      // Java: return -1;
      -1
    }

    // Java: int lastIndexOf(Object object)
    pub fn lastIndexOf(&self, object: &E) -> i32 {
      // Java: Object[] a = this.array;
      // Java: if (object != null)
      {
         // Java: for(int i = this.size - 1; i >= 0; --i)
         for i in (0..self.data.len()).rev() {
            // Java: if (object.equals(a[i]))
            if object == &self.data[i] {
               // Java: return i;
               return i as i32;
            }
         }
      }
      // Java: else
      {
         // Java: for(int i = this.size - 1; i >= 0; --i)
         {
            // Java: if (a[i] == null)
            {
               // Java: return i;
            }
         }
      }
      // Java: return -1;
      -1
    }

    // Java: E remove(int index)
    pub fn remove_at(&mut self, index: usize) -> E {
      // Java: Object[] a = this.array;
      // Java: int s = this.size;
      let s = self.data.len();
      // Java: if (index >= s)
      if index >= s {
         // Java: throwIndexOutOfBoundsException(index, s);
         Self::throwIndexOutOfBoundsException(index, s);
      }
      // Java: E result = (E)a[index];
      // Java: int var10001 = index + 1;
      // Java: --s;
      // Java: System.arraycopy(a, var10001, a, index, s - index);
      // Java: a[s] = null;
      let result = self.data.remove(index);
      // Java: this.size = s;
      // Java: ++this.modCount;
      self.modCount += 1;
      // Java: return result;
      result
    }

    // Java: boolean remove(Object object)
    pub fn remove_obj(&mut self, object: &E) -> bool {
      // Java: Object[] a = this.array;
      // Java: int s = this.size;
      // Java: if (object != null)
      {
         // Java: for(int i = 0; i < s; ++i)
         if let Some(pos) = self.data.iter().position(|x| x == object) {
            // Java: if (object.equals(a[i]))
            {
               // Java: int var10001 = i + 1;
               // Java: --s;
               // Java: System.arraycopy(a, var10001, a, i, s - i);
               // Java: a[s] = null;
               self.data.remove(pos);
               // Java: this.size = s;
               // Java: ++this.modCount;
               self.modCount += 1;
               // Java: return true;
               return true;
            }
         }
      }
      // Java: else
      {
         // Java: for(int i = 0; i < s; ++i)
         {
            // Java: if (a[i] == null)
            {
               // Java: int var8 = i + 1;
               // Java: --s;
               // Java: System.arraycopy(a, var8, a, i, s - i);
               // Java: a[s] = null;
               // Java: this.size = s;
               // Java: ++this.modCount;
               // Java: return true;
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
         let s = self.data.len();
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
            // Java: int rangeSize = toIndex - fromIndex;
            // Java: Arrays.fill(a, s - rangeSize, s, null);
            self.data.drain(fromIndex..toIndex);
            // Java: this.size = s - rangeSize;
            // Java: ++this.modCount;
            self.modCount += 1;
         }
      }
    }

    // Java: E set(int index, E object)
    pub fn set(&mut self, index: usize, object: E) -> E {
      // Java: Object[] a = this.array;
      // Java: if (index >= this.size)
      if index >= self.data.len() {
         // Java: throwIndexOutOfBoundsException(index, this.size);
         Self::throwIndexOutOfBoundsException(index, self.data.len());
      }
      // Java: E result = (E)a[index];
      // Java: a[index] = object;
      let result = std::mem::replace(&mut self.data[index], object);
      // Java: return result;
      result
    }

    // Java: Object[] toArray()
    pub fn toArray(&self) -> Vec<E> {
      // Java: int s = this.size;
      // Java: Object[] result = new Object[s];
      // Java: System.arraycopy(this.array, 0, result, 0, s);
      // Java: return result;
      self.data.clone()
    }

    // Java: public <T> T[] toArray(T[] contents)
    pub fn toArray_into(&self, contents: &mut Vec<E>) {
      // Java: int s = this.size;
      let s = self.data.len();
      // Java: if (contents.length < s)
      if contents.len() < s {
         // Java: T[] newArray = (T[])((Object[])Array.newInstance(contents.getClass().getComponentType(), s));
         // Java: contents = newArray;
         contents.clear();
      }
      // Java: System.arraycopy(this.array, 0, contents, 0, s);
      contents.clone_from(&self.data);
      // Java: if (contents.length > s)
      {
         // Java: contents[s] = null;
      }
      // Java: return contents;
    }

    // Java: void trimToSize()
    pub fn trimToSize(&mut self) {
      // Java: int s = this.size;
      let s = self.data.len();
      // Java: if (s != this.array.length)
      if s != self.data.capacity() {
         // Java: if (s == 0)
         if s == 0 {
            // Java: this.array = EMPTY_OBJECT_ARRAY;
            self.data = Vec::new();
         }
         // Java: else
         else {
            // Java: Object[] newArray = new Object[s];
            // Java: System.arraycopy(this.array, 0, newArray, 0, s);
            // Java: this.array = newArray;
            self.data.shrink_to_fit();
         }
         // Java: ++this.modCount;
         self.modCount += 1;
      }
    }

    // Java: Iterator<E> iterator()
    pub fn iter(&self) -> std::slice::Iter<'_, E> {
      // Java: return new ArrayListIterator();
      self.data.iter()
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32
    where E: std::hash::Hash {
      // Java: Object[] a = this.array;
      // Java: int hashCode = 1;
      let mut hash_code: i32 = 1;
      // Java: int i = 0;
      // Java: for(int s = this.size; i < s; ++i)
      for item in &self.data {
         // Java: Object e = a[i];
         // Java: hashCode = 31 * hashCode + (e == null ? 0 : e.hashCode());
         use std::hash::Hasher;
         let mut hasher = std::collections::hash_map::DefaultHasher::new();
         item.hash(&mut hasher);
         let h = hasher.finish() as i32;
         hash_code = 31i32.wrapping_mul(hash_code).wrapping_add(h);
      }
      // Java: return hashCode;
      hash_code
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &ArrayListStack<E>) -> bool {
      // Java: if (o == this)
      if std::ptr::eq(self, o) {
         // Java: return true;
         return true;
      }
      // Java: else if (!(o instanceof List))
      // Java: return false;
      // Java: else
      {
         // Java: List<?> that = (List)o;
         // Java: int s = this.size;
         let s = self.data.len();
         // Java: if (that.size() != s)
         if o.data.len() != s {
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
                  // Java: Object ethat = that.get(i);
                  // Java: if (eThis == null)
                  // Java: if (ethat != null)
                  // Java: return false;
                  // Java: else if (!eThis.equals(ethat))
                  if self.data[i] != o.data[i] {
                     // Java: return false;
                     return false;
                  }
               }
            }
            // Java: else
            {
               // Java: Iterator<?> it = that.iterator();
               // Java: for(int i = 0; i < s; ++i)
               {
                  // Java: Object eThis = a[i];
                  // Java: Object eThat = it.next();
                  // Java: if (eThis == null)
                  {
                     // Java: if (eThat != null)
                     {
                        // Java: return false;
                     }
                  }
                  // Java: else if (!eThis.equals(eThat))
                  {
                     // Java: return false;
                  }
               }
            }
            // Java: return true;
            return true;
         }
      }
    }

    // Java: void writeObject(ObjectOutputStream stream)
    // (Serialization not used in Rust)
    // Java: stream.defaultWriteObject();
    // Java: stream.writeInt(this.array.length);
    // Java: for(int i = 0; i < this.size; ++i)
    // Java: stream.writeObject(this.array[i]);

    // Java: void readObject(ObjectInputStream stream)
    // (Deserialization not used in Rust)
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
// Rust uses standard Iterator trait via .iter()
// Java: ArrayListIterator()
// Java: super();
// Java: this.remaining = ArrayListStack.this.size;
// Java: this.removalIndex = -1;
// Java: this.expectedModCount = ArrayListStack.this.modCount;

// Java: boolean hasNext()
// Java: return this.remaining != 0;

// Java: E next()
// Java: ArrayListStack<E> ourList = ArrayListStack.this;
// Java: int rem = this.remaining;
// Java: if (ourList.modCount != this.expectedModCount)
// Java: throw new ConcurrentModificationException();
// Java: else if (rem == 0)
// Java: throw new NoSuchElementException();
// Java: else
// Java: this.remaining = rem - 1;
// Java: return (E)ourList.array[this.removalIndex = ourList.size - rem];

// Java: void remove()
// Java: Object[] a = ArrayListStack.this.array;
// Java: int removalIdx = this.removalIndex;
// Java: if (ArrayListStack.this.modCount != this.expectedModCount)
// Java: throw new ConcurrentModificationException();
// Java: else if (removalIdx < 0)
// Java: throw new IllegalStateException();
// Java: else
// Java: System.arraycopy(a, removalIdx + 1, a, removalIdx, this.remaining);
// Java: a[--ArrayListStack.this.size] = null;
// Java: this.removalIndex = -1;
// Java: this.expectedModCount = ++ArrayListStack.this.modCount;
