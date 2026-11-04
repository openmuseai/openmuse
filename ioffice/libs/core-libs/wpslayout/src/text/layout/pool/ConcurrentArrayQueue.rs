// Java: import java.util.concurrent.atomic.AtomicInteger
// TODO_RUST_CODE use ???; // java.util.concurrent.atomic.AtomicInteger

// Java: public class ConcurrentArrayQueue
pub struct ConcurrentArrayQueue {
    // Java field: Object[] ring
    ring: Vec<Object>,
    // Java field: AtomicInteger maximumReadIndex
    maximumReadIndex: AtomicInteger,
    // Java field: AtomicInteger readIndex
    readIndex: AtomicInteger,
    // Java field: AtomicInteger writeIndex
    writeIndex: AtomicInteger,
}

// Java: public class ConcurrentArrayQueue
impl ConcurrentArrayQueue {
    // Java: ConcurrentArrayQueue(int capacity)
    pub fn ConcurrentArrayQueue(&self, int capacity) {
      // Java: super();
      // TODO_RUST_CODE
      // Java: if (capacity < 0)
        // TODO_RUST_CODE
      {
         // Java: throw new IllegalArgumentException("illegal capacity " + capacity);
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: this.ring = new Object[capacity + 1];
         // TODO_RUST_CODE
      }
    }

    // Java: boolean push(T e)
    pub fn push(&self, T e) -> bool {
      // Java: int currentWriteIndex;
      // TODO_RUST_CODE
      // Java: do
        // TODO_RUST_CODE
      {
         // Java: int currentReadIndex = this.readIndex.get();
         // TODO_RUST_CODE
         // Java: currentWriteIndex = this.writeIndex.get();
         // TODO_RUST_CODE
         // Java: if ((currentWriteIndex + 1) % this.ring.length == currentReadIndex % this.ring.length)
            // TODO_RUST_CODE
         {
            // Java: return false;
            // TODO_RUST_CODE
         }
      }
      // Java: this.ring[currentWriteIndex % this.ring.length] = e;
      // TODO_RUST_CODE
      // Java: while(!this.maximumReadIndex.compareAndSet(currentWriteIndex, currentWriteIndex + 1))
        // TODO_RUST_CODE
      {
         // Java: Thread.yield();
         // TODO_RUST_CODE
      }
      // Java: return true;
      // TODO_RUST_CODE
    }

    // Java: boolean tryPush(T e)
    pub fn tryPush(&self, T e) -> bool {
      // Java: int currentReadIndex = this.readIndex.get();
      // TODO_RUST_CODE
      // Java: int currentWriteIndex = this.writeIndex.get();
      // TODO_RUST_CODE
      // Java: if ((currentWriteIndex + 1) % this.ring.length == currentReadIndex % this.ring.length)
        // TODO_RUST_CODE
      {
         // Java: return false;
         // TODO_RUST_CODE
      }
      // Java: else if (!this.writeIndex.compareAndSet(currentWriteIndex, currentWriteIndex + 1))
        // TODO_RUST_CODE
      {
         // Java: return false;
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: this.ring[currentWriteIndex % this.ring.length] = e;
         // TODO_RUST_CODE
         // Java: while(!this.maximumReadIndex.compareAndSet(currentWriteIndex, currentWriteIndex + 1))
            // TODO_RUST_CODE
         {
            // Java: Thread.yield();
            // TODO_RUST_CODE
         }
         // Java: return true;
         // TODO_RUST_CODE
      }
    }

    // Java: T pop()
    pub fn pop(&self) -> T {
      // Java: int currentReadIndex;
      // TODO_RUST_CODE
      // Java: T ret;
      // TODO_RUST_CODE
      // Java: do
        // TODO_RUST_CODE
      {
         // Java: currentReadIndex = this.readIndex.get();
         // TODO_RUST_CODE
         // Java: int currentMaximumReadIndex = this.maximumReadIndex.get();
         // TODO_RUST_CODE
         // Java: if (currentReadIndex % this.ring.length == currentMaximumReadIndex % this.ring.length)
            // TODO_RUST_CODE
         {
            // Java: return null;
            // TODO_RUST_CODE
         }
         // Java: ret = (T)this.ring[currentReadIndex % this.ring.length];
         // TODO_RUST_CODE
      }
      // Java: return ret;
      // TODO_RUST_CODE
    }

    // Java: T tryPop()
    pub fn tryPop(&self) -> T {
      // Java: int currentReadIndex = this.readIndex.get();
      // TODO_RUST_CODE
      // Java: int currentMaximumReadIndex = this.maximumReadIndex.get();
      // TODO_RUST_CODE
      // Java: if (currentReadIndex % this.ring.length == currentMaximumReadIndex % this.ring.length)
        // TODO_RUST_CODE
      {
         // Java: return null;
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: T ret = (T)this.ring[currentReadIndex % this.ring.length];
         // TODO_RUST_CODE
         // Java: return (T)(this.readIndex.compareAndSet(currentReadIndex, currentReadIndex + 1) ? ret : null);
         // TODO_RUST_CODE
      }
    }

    // Java: int size()
    pub fn size(&self) -> i32 {
      // Java: int ret = this.writeIndex.get() - this.readIndex.get();
      // TODO_RUST_CODE
      // Java: return ret < 0 ? 0 : ret;
      // TODO_RUST_CODE
    }

    // Java: void clear()
    pub fn clear(&self) {
      // Java: while(this.size() > 0)
        // TODO_RUST_CODE
      {
         // Java: this.pop();
         // TODO_RUST_CODE
      }
    }

}
