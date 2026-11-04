#![allow(dead_code, unused_variables, unused_imports, non_snake_case, unused_mut, unused_assignments)]
// Java: import java.lang.Object
// CARGO_OFF_SURFACE use stubs::java::lang::Object::Object;

// java private static final int MIN_INITIAL_CAPACITY = 4;
const MIN_INITIAL_CAPACITY: i32 = 4;

// java public final class GraphArrayDeque
pub struct GraphArrayDeque<E> {
    // java private Object[] elements = new Object[4];
    elements: Vec<Option<E>>,
    // java private int head;
    head: i32,
    // java private int tail;
    tail: i32,
}

impl<E> Default for GraphArrayDeque<E> {
    fn default() -> Self {
        let mut elements = Vec::with_capacity(MIN_INITIAL_CAPACITY as usize);
        let mut i = 0;
        while i < MIN_INITIAL_CAPACITY {
            elements.push(None);
            i += 1;
        }
        Self { elements, head: 0, tail: 0 }
    }
}

impl<E> GraphArrayDeque<E> {
    pub fn new() -> Self {
        Self::default()
    }

    fn arraycopy(src: &mut Vec<Option<E>>, srcPos: i32, dest: &mut Vec<Option<E>>, destPos: i32, length: i32) {
        let mut tmp: Vec<Option<E>> = Vec::new();
        let mut i: i32 = 0;
        while i < length {
            tmp.push(src[(srcPos + i) as usize].take());
            i += 1;
        }
        i = 0;
        while i < length {
            dest[(destPos + i) as usize] = tmp[i as usize].take();
            i += 1;
        }
    }

    // java public int size()
    pub fn size(&self) -> i32 {
      // java return this.tail - this.head & this.elements.length - 1;
      return (self.tail - self.head) & ((self.elements.len() as i32) - 1);
    }

    // java public E get(int index)
    pub fn get(&self, index: i32) -> Option<&E> {
      // java E result = (E)this.elements[index + this.head & this.elements.length - 1];
      let slot = (index + self.head) & ((self.elements.len() as i32) - 1);
      let result: Option<&E> = self.elements[slot as usize].as_ref();
      // java return result;
      return result;
    }

    // java public int indexOf(E e)
    pub fn indexOf(&self, e: &E) -> i32
    where
        E: PartialEq,
    {
      // java int index = 0;
      let mut index: i32 = 0;
      // java int pos = this.head;
      let mut pos: i32 = self.head;
      // java int capacity = this.elements.length;
      let mut capacity: i32 = self.elements.len() as i32;
      // java while(pos != this.tail)
      while pos != self.tail {
        // java if (this.elements[pos] == e)
        if self.elements[pos as usize].as_ref() == Some(e) {
          // java return index;
          return index;
        }
        // java ++index;
        index += 1;
        // java ++pos;
        pos += 1;
        // java if (pos == capacity)
        if pos == capacity {
          // java pos = 0;
          pos = 0;
        }
      }
      // java return -1;
      return -1;
    }

    // java public void replace(E src, E des)
    pub fn replace(&mut self, src: &E, des: E)
    where
        E: PartialEq,
    {
      // java int pos = this.head;
      let mut pos: i32 = self.head;
      // java int capacity = this.elements.length;
      let mut capacity: i32 = self.elements.len() as i32;
      // java while(pos != this.tail)
      while pos != self.tail {
        // java if (this.elements[pos] == src)
        if self.elements[pos as usize].as_ref() == Some(src) {
          // java this.elements[pos] = des;
          self.elements[pos as usize] = Some(des);
          // java return;
          return;
        }
        // java ++pos;
        pos += 1;
        // java if (pos == capacity)
        if pos == capacity {
          // java pos = 0;
          pos = 0;
        }
      }
    }

    // java public E removeAt(int index)
    pub fn removeAt(&mut self, index: i32) -> Option<E> {
      // java if (index >= 0 && index < this.size())
      if index >= 0 && index < self.size() {
        // java int mask = this.elements.length - 1;
        let mut mask: i32 = (self.elements.len() as i32) - 1;
        // java int pos = index + this.head & mask;
        let mut pos: i32 = (index + self.head) & mask;
        // java E e = (E)this.elements[pos];
        let e: Option<E> = self.elements[pos as usize].take();
        // java if (pos == this.head)
        if pos == self.head {
          // java this.head = this.head + 1 & mask;
          self.head = (self.head + 1) & mask;
        }
        // java else if (pos == this.tail - 1)
        else if pos == self.tail - 1 {
          // java this.tail = pos;
          self.tail = pos;
        }
        // java else if (pos == mask && this.tail == 0)
        else if pos == mask && self.tail == 0 {
          // java this.tail = pos;
          self.tail = pos;
        }
        // java else
        else {
          // java if (pos < this.tail)
          if pos < self.tail {
            // java System.arraycopy(this.elements, pos + 1, this.elements, pos, this.tail - pos - 1);
            let n = self.tail - pos - 1;
            let mut tmp: Vec<Option<E>> = Vec::new();
            let mut i: i32 = 0;
            while i < n {
                tmp.push(self.elements[(pos + 1 + i) as usize].take());
                i += 1;
            }
            i = 0;
            while i < n {
                self.elements[(pos + i) as usize] = tmp[i as usize].take();
                i += 1;
            }
          }
          // java else
          else {
            // java System.arraycopy(this.elements, pos + 1, this.elements, pos, mask - pos);
            let n = mask - pos;
            let mut tmp: Vec<Option<E>> = Vec::new();
            let mut i: i32 = 0;
            while i < n {
                tmp.push(self.elements[(pos + 1 + i) as usize].take());
                i += 1;
            }
            i = 0;
            while i < n {
                self.elements[(pos + i) as usize] = tmp[i as usize].take();
                i += 1;
            }
            // java if (this.tail > 0)
            if self.tail > 0 {
              // java this.elements[mask] = this.elements[0];
              self.elements[mask as usize] = self.elements[0].take();
              // java if (this.tail > 1)
              if self.tail > 1 {
                // java System.arraycopy(this.elements, 1, this.elements, 0, this.tail - 1);
                let n2 = self.tail - 1;
                let mut tmp2: Vec<Option<E>> = Vec::new();
                i = 0;
                while i < n2 {
                    tmp2.push(self.elements[(1 + i) as usize].take());
                    i += 1;
                }
                i = 0;
                while i < n2 {
                    self.elements[i as usize] = tmp2[i as usize].take();
                    i += 1;
                }
              }
            }
          }
          // java this.tail = this.tail - 1 & mask;
          self.tail = (self.tail - 1) & mask;
        }
        // java return e;
        return e;
      }
      // java else
      else {
        // java return null;
        return None;
      }
    }

    // java public void add(E e)
    pub fn add(&mut self, e: E) {
      // java this.elements[this.tail] = e;
      self.elements[self.tail as usize] = Some(e);
      // java if ((this.tail = this.tail + 1 & this.elements.length - 1) == this.head)
      self.tail = (self.tail + 1) & ((self.elements.len() as i32) - 1);
      if self.tail == self.head {
        // java this.doubleCapacity();
        self.doubleCapacity();
      }
    }

    // java private void doubleCapacity()
    pub fn doubleCapacity(&mut self) {
      // java int p = this.head;
      let mut p: i32 = self.head;
      // java int n = this.elements.length;
      let mut n: i32 = self.elements.len() as i32;
      // java int r = n - p;
      let mut r: i32 = n - p;
      // java int newCapacity = n << 1;
      let mut newCapacity: i32 = n << 1;
      // java if (newCapacity < 0)
      if newCapacity < 0 {
        // java throw new IllegalStateException("Sorry, deque too big");
        panic!("Sorry, deque too big");
      }
      // java else
      else {
        // java Object[] a = new Object[newCapacity];
        let mut a: Vec<Option<E>> = Vec::new();
        let mut zi: i32 = 0;
        while zi < newCapacity {
            a.push(None);
            zi += 1;
        }
        // java System.arraycopy(this.elements, p, a, 0, r);
        {
            let mut i: i32 = 0;
            while i < r {
                a[i as usize] = self.elements[(p + i) as usize].take();
                i += 1;
            }
        }
        // java System.arraycopy(this.elements, 0, a, r, p);
        {
            let mut i: i32 = 0;
            while i < p {
                a[(r + i) as usize] = self.elements[i as usize].take();
                i += 1;
            }
        }
        // java this.elements = a;
        self.elements = a;
        // java this.head = 0;
        self.head = 0;
        // java this.tail = n;
        self.tail = n;
      }
    }

    // java public void clear()
    pub fn clear(&mut self) {
      // java this.head = 0;
      self.head = 0;
      // java this.tail = 0;
      self.tail = 0;
    }

}
