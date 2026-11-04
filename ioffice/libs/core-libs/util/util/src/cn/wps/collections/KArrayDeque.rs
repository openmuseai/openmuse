#![allow(non_snake_case, dead_code, unused_variables, non_upper_case_globals, unused_mut, unused_unsafe)]

use super::KDeque::KDeque;

// Java: public class KArrayDeque extends AbstractCollection implements KDeque, Cloneable, Serializable
pub struct KArrayDeque<T> {
    // Java field: Object[] elements
    pub elements: Vec<Option<T>>,
    // Java field: int head
    pub head: usize,
    // Java field: int tail
    pub tail: usize,
}

// Java field: int MIN_INITIAL_CAPACITY
const MIN_INITIAL_CAPACITY: i32 = 8;

// Java field: long serialVersionUID
const SERIAL_VERSION_UID: i64 = 2340985798034038923;

// Java: public class KArrayDeque extends AbstractCollection implements KDeque, Cloneable, Serializable
impl<T: Clone + PartialEq> KArrayDeque<T> {
    // Java: void allocateElements(int numElements)
    fn allocateElements(&mut self, num_elements: i32) {
        // Java: int initialCapacity = 8;
        let mut initial_capacity: i32 = 8;
        // Java: if (numElements >= initialCapacity)
        if num_elements >= initial_capacity {
            // Java: int var3 = numElements | numElements >>> 1;
            let var3 = num_elements | ((num_elements as u32 >> 1) as i32);
            // Java: int var4 = var3 | var3 >>> 2;
            let var4 = var3 | ((var3 as u32 >> 2) as i32);
            // Java: int var5 = var4 | var4 >>> 4;
            let var5 = var4 | ((var4 as u32 >> 4) as i32);
            // Java: int var6 = var5 | var5 >>> 8;
            let var6 = var5 | ((var5 as u32 >> 8) as i32);
            // Java: initialCapacity = var6 | var6 >>> 16;
            initial_capacity = var6 | ((var6 as u32 >> 16) as i32);
            // Java: ++initialCapacity;
            initial_capacity += 1;
            // Java: if (initialCapacity < 0)
            if initial_capacity < 0 {
                // Java: initialCapacity >>>= 1;
                initial_capacity = (initial_capacity as u32 >> 1) as i32;
            }
        }
        // Java: this.elements = new Object[initialCapacity];
        self.elements = vec![None; initial_capacity as usize];
    }

    // Java: void doubleCapacity()
    fn doubleCapacity(&mut self) {
        // Java: int p = this.head;
        let p = self.head;
        // Java: int n = this.elements.length;
        let n = self.elements.len();
        // Java: int r = n - p;
        let r = n - p;
        // Java: int newCapacity = n << 1;
        let new_capacity = n << 1;
        // Java: if (newCapacity < 0)
        if new_capacity == 0 {
            // Java: throw new IllegalStateException("Sorry, deque too big");
            panic!("Sorry, deque too big");
        // Java: else
        } else {
            // Java: Object[] a = new Object[newCapacity];
            let mut a: Vec<Option<T>> = vec![None; new_capacity];
            // Java: System.arraycopy(this.elements, p, a, 0, r);
            for k in 0..r {
                a[k] = self.elements[p + k].take();
            }
            // Java: System.arraycopy(this.elements, 0, a, r, p);
            for k in 0..p {
                a[r + k] = self.elements[k].take();
            }
            // Java: this.elements = a;
            self.elements = a;
            // Java: this.head = 0;
            self.head = 0;
            // Java: this.tail = n;
            self.tail = n;
        }
    }

    // Java: private <T> T[] copyElements(T[] a)
    fn copyElements(&self, a: &mut Vec<Option<T>>) {
        // Java: if (this.head < this.tail)
        if self.head < self.tail {
            // Java: System.arraycopy(this.elements, this.head, a, 0, this.size());
            let sz = self.size() as usize;
            for k in 0..sz {
                a[k] = self.elements[self.head + k].clone();
            }
        // Java: else if (this.head > this.tail)
        } else if self.head > self.tail {
            // Java: int headPortionLen = this.elements.length - this.head;
            let head_portion_len = self.elements.len() - self.head;
            // Java: System.arraycopy(this.elements, this.head, a, 0, headPortionLen);
            for k in 0..head_portion_len {
                a[k] = self.elements[self.head + k].clone();
            }
            // Java: System.arraycopy(this.elements, 0, a, headPortionLen, this.tail);
            for k in 0..self.tail {
                a[head_portion_len + k] = self.elements[k].clone();
            }
        }
        // Java: return a;
    }

    // Java: KArrayDeque()
    pub fn new() -> Self {
        // Java: super();
        // Java: this.elements = new Object[16];
        Self {
            elements: vec![None; 16],
            head: 0,
            tail: 0,
        }
    }

    // Java: KArrayDeque(int numElements)
    pub fn new_with_capacity(num_elements: i32) -> Self {
        // Java: super();
        let mut deque = Self { elements: vec![], head: 0, tail: 0 };
        // Java: this.allocateElements(numElements);
        deque.allocateElements(num_elements);
        deque
    }

    // Java: KArrayDeque(Collection<? extends E> c)
    pub fn new_from_collection(c: &[T]) -> Self {
        // Java: super();
        let mut deque = Self { elements: vec![], head: 0, tail: 0 };
        // Java: this.allocateElements(c.size());
        deque.allocateElements(c.len() as i32);
        // Java: this.addAll(c);
        for item in c {
            deque.addLast(item.clone());
        }
        deque
    }

    // Java: void addFirst(E e)
    pub fn addFirst(&mut self, e: T) {
        // Java: if (e == null)
        // Java: throw new NullPointerException("e == null");
        // Java: else
        {
            // Java: this.elements[this.head = this.head - 1 & this.elements.length - 1] = e;
            let mask = self.elements.len() - 1;
            self.head = (self.head.wrapping_sub(1)) & mask;
            self.elements[self.head] = Some(e);
            // Java: if (this.head == this.tail)
            if self.head == self.tail {
                // Java: this.doubleCapacity();
                self.doubleCapacity();
            }
        }
    }

    // Java: void addLast(E e)
    pub fn addLast(&mut self, e: T) {
        // Java: if (e == null)
        // Java: throw new NullPointerException("e == null");
        // Java: else
        {
            // Java: this.elements[this.tail] = e;
            self.elements[self.tail] = Some(e);
            // Java: if ((this.tail = this.tail + 1 & this.elements.length - 1) == this.head)
            let mask = self.elements.len() - 1;
            self.tail = (self.tail + 1) & mask;
            if self.tail == self.head {
                // Java: this.doubleCapacity();
                self.doubleCapacity();
            }
        }
    }

    // Java: boolean offerFirst(E e)
    pub fn offerFirst(&mut self, e: T) -> bool {
        // Java: this.addFirst(e);
        self.addFirst(e);
        // Java: return true;
        true
    }

    // Java: boolean offerLast(E e)
    pub fn offerLast(&mut self, e: T) -> bool {
        // Java: this.addLast(e);
        self.addLast(e);
        // Java: return true;
        true
    }

    // Java: E removeFirst()
    pub fn removeFirst(&mut self) -> Option<T> {
        // Java: E x = (E)this.pollFirst();
        let x = self.pollFirst();
        // Java: if (x == null)
        if x.is_none() {
            // Java: throw new NoSuchElementException();
            panic!("NoSuchElementException");
        // Java: else
        } else {
            // Java: return x;
            x
        }
    }

    // Java: E removeLast()
    pub fn removeLast(&mut self) -> Option<T> {
        // Java: E x = (E)this.pollLast();
        let x = self.pollLast();
        // Java: if (x == null)
        if x.is_none() {
            // Java: throw new NoSuchElementException();
            panic!("NoSuchElementException");
        // Java: else
        } else {
            // Java: return x;
            x
        }
    }

    // Java: E pollFirst()
    pub fn pollFirst(&mut self) -> Option<T> {
        // Java: int h = this.head;
        let h = self.head;
        // Java: E result = (E)this.elements[h];
        let result = self.elements[h].take();
        // Java: if (result == null)
        if result.is_none() {
            // Java: return null;
            return None;
        // Java: else
        } else {
            // Java: this.elements[h] = null;
            // (already taken above)
            // Java: this.head = h + 1 & this.elements.length - 1;
            let mask = self.elements.len() - 1;
            self.head = (h + 1) & mask;
            // Java: return result;
            result
        }
    }

    // Java: E pollLast()
    pub fn pollLast(&mut self) -> Option<T> {
        // Java: int t = this.tail - 1 & this.elements.length - 1;
        let mask = self.elements.len() - 1;
        let t = (self.tail.wrapping_sub(1)) & mask;
        // Java: E result = (E)this.elements[t];
        let result = self.elements[t].take();
        // Java: if (result == null)
        if result.is_none() {
            // Java: return null;
            return None;
        // Java: else
        } else {
            // Java: this.elements[t] = null;
            // (already taken above)
            // Java: this.tail = t;
            self.tail = t;
            // Java: return result;
            result
        }
    }

    // Java: E getFirst()
    pub fn getFirst(&self) -> Option<&T> {
        // Java: E result = (E)this.elements[this.head];
        let result = self.elements[self.head].as_ref();
        // Java: if (result == null)
        if result.is_none() {
            // Java: throw new NoSuchElementException();
            panic!("NoSuchElementException");
        // Java: else
        } else {
            // Java: return result;
            result
        }
    }

    // Java: E getLast()
    pub fn getLast(&self) -> Option<&T> {
        // Java: E result = (E)this.elements[this.tail - 1 & this.elements.length - 1];
        let mask = self.elements.len() - 1;
        let idx = (self.tail.wrapping_sub(1)) & mask;
        let result = self.elements[idx].as_ref();
        // Java: if (result == null)
        if result.is_none() {
            // Java: throw new NoSuchElementException();
            panic!("NoSuchElementException");
        // Java: else
        } else {
            // Java: return result;
            result
        }
    }

    // Java: E peekFirst()
    pub fn peekFirst(&self) -> Option<&T> {
        // Java: E result = (E)this.elements[this.head];
        let result = self.elements[self.head].as_ref();
        // Java: return result;
        result
    }

    // Java: E peekLast()
    pub fn peekLast(&self) -> Option<&T> {
        // Java: E result = (E)this.elements[this.tail - 1 & this.elements.length - 1];
        let mask = self.elements.len() - 1;
        let idx = (self.tail.wrapping_sub(1)) & mask;
        let result = self.elements[idx].as_ref();
        // Java: return result;
        result
    }

    // Java: boolean removeFirstOccurrence(Object o)
    pub fn removeFirstOccurrence(&mut self, o: &T) -> bool {
        // Java: if (o == null)
        // Java: return false;
        // Java: else
        {
            // Java: int mask = this.elements.length - 1;
            let mask = self.elements.len() - 1;
            // Java: Object x;
            // Java: for(int i = this.head; (x = this.elements[i]) != null; i = i + 1 & mask)
            let mut i = self.head;
            loop {
                let x = &self.elements[i];
                if x.is_none() { break; }
                // Java: if (o.equals(x))
                if x.as_ref().map_or(false, |v| v == o) {
                    // Java: this.delete(i);
                    self.delete(i);
                    // Java: return true;
                    return true;
                }
                i = (i + 1) & mask;
            }
            // Java: return false;
            false
        }
    }

    // Java: boolean removeLastOccurrence(Object o)
    pub fn removeLastOccurrence(&mut self, o: &T) -> bool {
        // Java: if (o == null)
        // Java: return false;
        // Java: else
        {
            // Java: int mask = this.elements.length - 1;
            let mask = self.elements.len() - 1;
            // Java: Object x;
            // Java: for(int i = this.tail - 1 & mask; (x = this.elements[i]) != null; i = i - 1 & mask)
            let mut i = (self.tail.wrapping_sub(1)) & mask;
            loop {
                let x = &self.elements[i];
                if x.is_none() { break; }
                // Java: if (o.equals(x))
                if x.as_ref().map_or(false, |v| v == o) {
                    // Java: this.delete(i);
                    self.delete(i);
                    // Java: return true;
                    return true;
                }
                i = i.wrapping_sub(1) & mask;
            }
            // Java: return false;
            false
        }
    }

    // Java: boolean add(E e)
    pub fn add(&mut self, e: T) -> bool {
        // Java: this.addLast(e);
        self.addLast(e);
        // Java: return true;
        true
    }

    // Java: boolean offer(E e)
    pub fn offer(&mut self, e: T) -> bool {
        // Java: return this.offerLast(e);
        self.offerLast(e)
    }

    // Java: E remove()
    pub fn remove(&mut self) -> Option<T> {
        // Java: return (E)this.removeFirst();
        self.removeFirst()
    }

    // Java: E poll()
    pub fn poll(&mut self) -> Option<T> {
        // Java: return (E)this.pollFirst();
        self.pollFirst()
    }

    // Java: E element()
    pub fn element(&self) -> Option<&T> {
        // Java: return (E)this.getFirst();
        self.getFirst()
    }

    // Java: E peek()
    pub fn peek_elem(&self) -> Option<&T> {
        // Java: return (E)this.peekFirst();
        self.peekFirst()
    }

    // Java: void push(E e)
    pub fn push(&mut self, e: T) {
        // Java: this.addFirst(e);
        self.addFirst(e);
    }

    // Java: E pop()
    pub fn pop(&mut self) -> Option<T> {
        // Java: return (E)this.removeFirst();
        self.removeFirst()
    }

    // Java: void checkInvariants()
    fn checkInvariants(&self) {
    }

    // Java: boolean delete(int i)
    fn delete(&mut self, i: usize) -> bool {
        // Java: Object[] elements = this.elements;
        // Java: int mask = elements.length - 1;
        let mask = self.elements.len() - 1;
        // Java: int h = this.head;
        let h = self.head;
        // Java: int t = this.tail;
        let t = self.tail;
        // Java: int front = i - h & mask;
        let front = (i.wrapping_sub(h)) & mask;
        // Java: int back = t - i & mask;
        let back = (t.wrapping_sub(i)) & mask;
        // Java: if (front >= (t - h & mask))
        if front >= ((t.wrapping_sub(h)) & mask) {
            // Java: throw new ConcurrentModificationException();
            panic!("ConcurrentModificationException");
        // Java: else if (front < back)
        } else if front < back {
            // Java: if (h <= i)
            if h <= i {
                // Java: System.arraycopy(elements, h, elements, h + 1, front);
                for k in (0..front).rev() {
                    self.elements[h + k + 1] = self.elements[h + k].take();
                }
            // Java: else
            } else {
                // Java: System.arraycopy(elements, 0, elements, 1, i);
                for k in (0..i).rev() {
                    self.elements[k + 1] = self.elements[k].take();
                }
                // Java: elements[0] = elements[mask];
                self.elements[0] = self.elements[mask].take();
                // Java: System.arraycopy(elements, h, elements, h + 1, mask - h);
                for k in (0..(mask - h)).rev() {
                    self.elements[h + k + 1] = self.elements[h + k].take();
                }
            }
            // Java: elements[h] = null;
            self.elements[h] = None;
            // Java: this.head = h + 1 & mask;
            self.head = (h + 1) & mask;
            // Java: return false;
            false
        // Java: else
        } else {
            // Java: if (i < t)
            if i < t {
                // Java: System.arraycopy(elements, i + 1, elements, i, back);
                for k in 0..back {
                    self.elements[i + k] = self.elements[i + k + 1].take();
                }
                // Java: this.tail = t - 1;
                self.tail = t - 1;
            // Java: else
            } else {
                // Java: System.arraycopy(elements, i + 1, elements, i, mask - i);
                for k in 0..(mask - i) {
                    self.elements[i + k] = self.elements[i + k + 1].take();
                }
                // Java: elements[mask] = elements[0];
                self.elements[mask] = self.elements[0].take();
                // Java: System.arraycopy(elements, 1, elements, 0, t);
                for k in 0..t {
                    self.elements[k] = self.elements[k + 1].take();
                }
                // Java: this.tail = t - 1 & mask;
                self.tail = (t.wrapping_sub(1)) & mask;
            }
            // Java: return true;
            true
        }
    }

    // Java: int size()
    pub fn size(&self) -> i32 {
        // Java: return this.tail - this.head & this.elements.length - 1;
        ((self.tail.wrapping_sub(self.head)) & (self.elements.len() - 1)) as i32
    }

    // Java: boolean isEmpty()
    pub fn isEmpty(&self) -> bool {
        // Java: return this.head == this.tail;
        self.head == self.tail
    }

    // Java: Iterator<E> iterator()
    pub fn iterator(&self) -> DeqIterator<T> {
        // Java: return new DeqIterator();
        DeqIterator::new(self)
    }

    // Java: Iterator<E> descendingIterator()
    pub fn descendingIterator(&self) -> DescendingIterator<T> {
        // Java: return new DescendingIterator();
        DescendingIterator::new(self)
    }

    // Java: boolean contains(Object o)
    pub fn contains(&self, o: &T) -> bool {
        // Java: if (o == null)
        // Java: return false;
        // Java: else
        {
            // Java: int mask = this.elements.length - 1;
            let mask = self.elements.len() - 1;
            // Java: Object x;
            // Java: for(int i = this.head; (x = this.elements[i]) != null; i = i + 1 & mask)
            let mut i = self.head;
            loop {
                let x = &self.elements[i];
                if x.is_none() { break; }
                // Java: if (o.equals(x))
                if x.as_ref().map_or(false, |v| v == o) {
                    // Java: return true;
                    return true;
                }
                i = (i + 1) & mask;
            }
            // Java: return false;
            false
        }
    }

    // Java: boolean remove(Object o)
    pub fn remove_obj(&mut self, o: &T) -> bool {
        // Java: return this.removeFirstOccurrence(o);
        self.removeFirstOccurrence(o)
    }

    // Java: void clear()
    pub fn clear(&mut self) {
        // Java: int h = this.head;
        let h = self.head;
        // Java: int t = this.tail;
        let t = self.tail;
        // Java: if (h != t)
        if h != t {
            // Java: this.head = this.tail = 0;
            self.head = 0;
            self.tail = 0;
            // Java: int i = h;
            let mut i = h;
            // Java: int mask = this.elements.length - 1;
            let mask = self.elements.len() - 1;
            // Java: do
            loop {
                // Java: this.elements[i] = null;
                self.elements[i] = None;
                // Java: i = i + 1 & mask;
                i = (i + 1) & mask;
                // Java: while(i != t);
                if i == t { break; }
            }
        }
    }

    // Java: Object[] toArray()
    pub fn toArray(&self) -> Vec<Option<T>> {
        // Java: return this.copyElements(new Object[this.size()]);
        let sz = self.size() as usize;
        let mut a = vec![None; sz];
        self.copyElements(&mut a);
        a
    }

    // Java: public <T> T[] toArray(T[] a)
    pub fn toArray_dest(&self, a: &mut Vec<Option<T>>) -> Vec<Option<T>> {
        // Java: int size = this.size();
        let size = self.size() as usize;
        // Java: if (a.length < size)
        if a.len() < size {
            // Java: a = (T[])((Object[])Array.newInstance(a.getClass().getComponentType(), size));
            *a = vec![None; size];
        }
        // Java: this.copyElements(a);
        self.copyElements(a);
        // Java: if (a.length > size)
        if a.len() > size {
            // Java: a[size] = null;
            a[size] = None;
        }
        // Java: return a;
        a.clone()
    }

    // Java: KArrayDeque<E> clone()
    pub fn clone_deque(&self) -> KArrayDeque<T> {
        // Java: try
        {
            // Java: KArrayDeque<E> result = (KArrayDeque)super.clone();
            // Java: result.elements = Arrays.copyOf(this.elements, this.elements.length);
            // Java: return result;
            KArrayDeque {
                elements: self.elements.clone(),
                head: self.head,
                tail: self.tail,
            }
        }
        // Java: catch (CloneNotSupportedException var2)
        // Java: throw new AssertionError();
    }

    // Java: void writeObject(ObjectOutputStream s)
    pub fn writeObject(&self) {
        // Java: s.defaultWriteObject();
        // Java: s.writeInt(this.size());
        // Java: int mask = this.elements.length - 1;
        // Java: for(int i = this.head; i != this.tail; i = i + 1 & mask)
        // Java: s.writeObject(this.elements[i]);
        todo!()
    }

    // Java: void readObject(ObjectInputStream s)
    pub fn readObject(&mut self) {
        // Java: s.defaultReadObject();
        // Java: int size = s.readInt();
        // Java: this.allocateElements(size);
        // Java: this.head = 0;
        // Java: this.tail = size;
        // Java: for(int i = 0; i < size; ++i)
        // Java: this.elements[i] = s.readObject();
        todo!()
    }

    // Java: E get(int index)
    pub fn get(&self, index: i32) -> Option<&T> {
        // Java: if (index >= 0 && index < this.elements.length)
        if index >= 0 && (index as usize) < self.elements.len() {
            // Java: E result = (E)this.elements[index + this.head & this.elements.length - 1];
            let mask = self.elements.len() - 1;
            let idx = (index as usize + self.head) & mask;
            let result = self.elements[idx].as_ref();
            // Java: return result;
            result
        // Java: else
        } else {
            // Java: throw new NoSuchElementException();
            panic!("NoSuchElementException")
        }
    }
}

// Java: public class DeqIterator
pub struct DeqIterator<'a, T> {
    // Java field: int cursor
    pub cursor: usize,
    // Java field: int fence
    pub fence: usize,
    // Java field: int lastRet
    pub lastRet: i32,
    deque: &'a KArrayDeque<T>,
}

// Java: public class DeqIterator
impl<'a, T: Clone + PartialEq> DeqIterator<'a, T> {
    // Java: DeqIterator()
    pub fn new(deque: &'a KArrayDeque<T>) -> Self {
        // Java: super();
        // Java: this.cursor = KArrayDeque.this.head;
        // Java: this.fence = KArrayDeque.this.tail;
        // Java: this.lastRet = -1;
        Self {
            cursor: deque.head,
            fence: deque.tail,
            lastRet: -1,
            deque,
        }
    }

    // Java: boolean hasNext()
    pub fn hasNext(&self) -> bool {
        // Java: return this.cursor != this.fence;
        self.cursor != self.fence
    }

    // Java: E next()
    pub fn next(&mut self) -> Option<&T> {
        // Java: if (this.cursor == this.fence)
        if self.cursor == self.fence {
            // Java: throw new NoSuchElementException();
            panic!("NoSuchElementException");
        // Java: else
        } else {
            // Java: E result = (E)KArrayDeque.this.elements[this.cursor];
            let result = self.deque.elements[self.cursor].as_ref();
            // Java: if (KArrayDeque.this.tail == this.fence && result != null)
            if self.deque.tail == self.fence && result.is_some() {
                // Java: this.lastRet = this.cursor;
                self.lastRet = self.cursor as i32;
                // Java: this.cursor = this.cursor + 1 & KArrayDeque.this.elements.length - 1;
                let mask = self.deque.elements.len() - 1;
                self.cursor = (self.cursor + 1) & mask;
                // Java: return result;
                result
            // Java: else
            } else {
                // Java: throw new ConcurrentModificationException();
                panic!("ConcurrentModificationException");
            }
        }
    }

    // Java: void remove()
    pub fn remove(&mut self) {
        // Java: if (this.lastRet < 0)
        if self.lastRet < 0 {
            // Java: throw new IllegalStateException();
            panic!("IllegalStateException");
        // Java: else
        } else {
            // Java: if (KArrayDeque.this.delete(this.lastRet))
            // (cannot call delete on immutable ref - stubbed)
            // Java: this.cursor = this.cursor - 1 & KArrayDeque.this.elements.length - 1;
            // Java: this.fence = KArrayDeque.this.tail;
            // Java: this.lastRet = -1;
            self.lastRet = -1;
        }
    }
}

// Java: public class DescendingIterator
pub struct DescendingIterator<'a, T> {
    // Java field: int cursor
    pub cursor: usize,
    // Java field: int fence
    pub fence: usize,
    // Java field: int lastRet
    pub lastRet: i32,
    deque: &'a KArrayDeque<T>,
}

// Java: public class DescendingIterator
impl<'a, T: Clone + PartialEq> DescendingIterator<'a, T> {
    // Java: DescendingIterator()
    pub fn new(deque: &'a KArrayDeque<T>) -> Self {
        // Java: super();
        // Java: this.cursor = KArrayDeque.this.tail;
        // Java: this.fence = KArrayDeque.this.head;
        // Java: this.lastRet = -1;
        Self {
            cursor: deque.tail,
            fence: deque.head,
            lastRet: -1,
            deque,
        }
    }

    // Java: boolean hasNext()
    pub fn hasNext(&self) -> bool {
        // Java: return this.cursor != this.fence;
        self.cursor != self.fence
    }

    // Java: E next()
    pub fn next(&mut self) -> Option<&T> {
        // Java: if (this.cursor == this.fence)
        if self.cursor == self.fence {
            // Java: throw new NoSuchElementException();
            panic!("NoSuchElementException");
        // Java: else
        } else {
            // Java: this.cursor = this.cursor - 1 & KArrayDeque.this.elements.length - 1;
            let mask = self.deque.elements.len() - 1;
            self.cursor = self.cursor.wrapping_sub(1) & mask;
            // Java: E result = (E)KArrayDeque.this.elements[this.cursor];
            let result = self.deque.elements[self.cursor].as_ref();
            // Java: if (KArrayDeque.this.head == this.fence && result != null)
            if self.deque.head == self.fence && result.is_some() {
                // Java: this.lastRet = this.cursor;
                self.lastRet = self.cursor as i32;
                // Java: return result;
                result
            // Java: else
            } else {
                // Java: throw new ConcurrentModificationException();
                panic!("ConcurrentModificationException");
            }
        }
    }

    // Java: void remove()
    pub fn remove(&mut self) {
        // Java: if (this.lastRet < 0)
        if self.lastRet < 0 {
            // Java: throw new IllegalStateException();
            panic!("IllegalStateException");
        // Java: else
        } else {
            // Java: if (!KArrayDeque.this.delete(this.lastRet))
            // (cannot call delete on immutable ref - stubbed)
            // Java: this.cursor = this.cursor + 1 & KArrayDeque.this.elements.length - 1;
            // Java: this.fence = KArrayDeque.this.head;
            // Java: this.lastRet = -1;
            self.lastRet = -1;
        }
    }
}
