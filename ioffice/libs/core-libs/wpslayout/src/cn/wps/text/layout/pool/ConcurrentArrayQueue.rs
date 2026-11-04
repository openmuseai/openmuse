// Java: import java.util.concurrent.atomic.AtomicInteger
use std::sync::atomic::{AtomicUsize, Ordering};
use std::cell::UnsafeCell;

// Java: public class ConcurrentArrayQueue
pub struct ConcurrentArrayQueue<T> {
    // Java field: Object[] ring
    ring: Box<[UnsafeCell<Option<T>>]>,
    // Java field: AtomicInteger maximumReadIndex
    maximumReadIndex: AtomicUsize,
    // Java field: AtomicInteger readIndex
    readIndex: AtomicUsize,
    // Java field: AtomicInteger writeIndex
    writeIndex: AtomicUsize,
}

unsafe impl<T: Send> Send for ConcurrentArrayQueue<T> {}
unsafe impl<T: Send> Sync for ConcurrentArrayQueue<T> {}

// Java: public class ConcurrentArrayQueue
impl<T> ConcurrentArrayQueue<T> {
    // Java: ConcurrentArrayQueue(int capacity)
    pub fn new(capacity: i32) -> Self {
      // Java: super();
      // Java: if (capacity < 0)
      if capacity < 0
      {
         // Java: throw new IllegalArgumentException("illegal capacity " + capacity);
         panic!("illegal capacity {}", capacity);
      }
      // Java: else
      else
      {
         // Java: this.ring = new Object[capacity + 1];
         let len = (capacity + 1) as usize;
         let mut ring = Vec::with_capacity(len);
         for _ in 0..len {
             ring.push(UnsafeCell::new(None));
         }
         Self {
             ring: ring.into_boxed_slice(),
             maximumReadIndex: AtomicUsize::new(0),
             readIndex: AtomicUsize::new(0),
             writeIndex: AtomicUsize::new(0),
         }
      }
    }

    // Java: boolean push(T e)
    pub fn push(&self, e: T) -> bool {
      // Java: int currentWriteIndex;
      let mut current_write_index: usize;
      // Java: do
      loop
      {
         // Java: int currentReadIndex = this.readIndex.get();
         let current_read_index = self.readIndex.load(Ordering::Acquire);
         // Java: currentWriteIndex = this.writeIndex.get();
         current_write_index = self.writeIndex.load(Ordering::Acquire);
         // Java: if ((currentWriteIndex + 1) % this.ring.length == currentReadIndex % this.ring.length)
         if (current_write_index + 1) % self.ring.len() == current_read_index % self.ring.len()
         {
            // Java: return false;
            return false;
         }
         if self.writeIndex.compare_exchange(current_write_index, current_write_index + 1, Ordering::AcqRel, Ordering::Acquire).is_ok() {
             break;
         }
      }
      // Java: this.ring[currentWriteIndex % this.ring.length] = e;
      unsafe { *self.ring[current_write_index % self.ring.len()].get() = Some(e); }
      // Java: while(!this.maximumReadIndex.compareAndSet(currentWriteIndex, currentWriteIndex + 1))
      while self.maximumReadIndex.compare_exchange(current_write_index, current_write_index + 1, Ordering::AcqRel, Ordering::Acquire).is_err()
      {
         // Java: Thread.yield();
         std::thread::yield_now();
      }
      // Java: return true;
      true
    }

    // Java: boolean tryPush(T e)
    pub fn tryPush(&self, e: T) -> bool {
      // Java: int currentReadIndex = this.readIndex.get();
      let current_read_index = self.readIndex.load(Ordering::Acquire);
      // Java: int currentWriteIndex = this.writeIndex.get();
      let current_write_index = self.writeIndex.load(Ordering::Acquire);
      // Java: if ((currentWriteIndex + 1) % this.ring.length == currentReadIndex % this.ring.length)
      if (current_write_index + 1) % self.ring.len() == current_read_index % self.ring.len()
      {
         // Java: return false;
         return false;
      }
      // Java: else if (!this.writeIndex.compareAndSet(currentWriteIndex, currentWriteIndex + 1))
      else if self.writeIndex.compare_exchange(current_write_index, current_write_index + 1, Ordering::AcqRel, Ordering::Acquire).is_err()
      {
         // Java: return false;
         return false;
      }
      // Java: else
      else
      {
         // Java: this.ring[currentWriteIndex % this.ring.length] = e;
         unsafe { *self.ring[current_write_index % self.ring.len()].get() = Some(e); }
         // Java: while(!this.maximumReadIndex.compareAndSet(currentWriteIndex, currentWriteIndex + 1))
         while self.maximumReadIndex.compare_exchange(current_write_index, current_write_index + 1, Ordering::AcqRel, Ordering::Acquire).is_err()
         {
            // Java: Thread.yield();
            std::thread::yield_now();
         }
         // Java: return true;
         return true;
      }
    }

    // Java: T pop()
    pub fn pop(&self) -> Option<T> {
      // Java: int currentReadIndex;
      let mut current_read_index: usize;
      // Java: T ret;
      let ret: T;
      // Java: do
      loop
      {
         // Java: currentReadIndex = this.readIndex.get();
         current_read_index = self.readIndex.load(Ordering::Acquire);
         // Java: int currentMaximumReadIndex = this.maximumReadIndex.get();
         let current_maximum_read_index = self.maximumReadIndex.load(Ordering::Acquire);
         // Java: if (currentReadIndex % this.ring.length == currentMaximumReadIndex % this.ring.length)
         if current_read_index % self.ring.len() == current_maximum_read_index % self.ring.len()
         {
            // Java: return null;
            return None;
         }
         // Java: ret = (T)this.ring[currentReadIndex % this.ring.length];
         let candidate = unsafe { &mut *self.ring[current_read_index % self.ring.len()].get() };
         if self.readIndex.compare_exchange(current_read_index, current_read_index + 1, Ordering::AcqRel, Ordering::Acquire).is_ok() {
             ret = candidate.take().unwrap();
             break;
         }
      }
      // Java: return ret;
      Some(ret)
    }

    // Java: T tryPop()
    pub fn tryPop(&self) -> Option<T> {
      // Java: int currentReadIndex = this.readIndex.get();
      let current_read_index = self.readIndex.load(Ordering::Acquire);
      // Java: int currentMaximumReadIndex = this.maximumReadIndex.get();
      let current_maximum_read_index = self.maximumReadIndex.load(Ordering::Acquire);
      // Java: if (currentReadIndex % this.ring.length == currentMaximumReadIndex % this.ring.length)
      if current_read_index % self.ring.len() == current_maximum_read_index % self.ring.len()
      {
         // Java: return null;
         return None;
      }
      // Java: else
      else
      {
         // Java: T ret = (T)this.ring[currentReadIndex % this.ring.length];
         let candidate = unsafe { &mut *self.ring[current_read_index % self.ring.len()].get() };
         // Java: return (T)(this.readIndex.compareAndSet(currentReadIndex, currentReadIndex + 1) ? ret : null);
         if self.readIndex.compare_exchange(current_read_index, current_read_index + 1, Ordering::AcqRel, Ordering::Acquire).is_ok() {
             candidate.take()
         } else {
             None
         }
      }
    }

    // Java: int size()
    pub fn size(&self) -> i32 {
      // Java: int ret = this.writeIndex.get() - this.readIndex.get();
      let ret = self.writeIndex.load(Ordering::Acquire) as i32 - self.readIndex.load(Ordering::Acquire) as i32;
      // Java: return ret < 0 ? 0 : ret;
      if ret < 0 { 0 } else { ret }
    }

    // Java: void clear()
    pub fn clear(&self) {
      // Java: while(this.size() > 0)
      while self.size() > 0
      {
         // Java: this.pop();
         self.pop();
      }
    }

}
