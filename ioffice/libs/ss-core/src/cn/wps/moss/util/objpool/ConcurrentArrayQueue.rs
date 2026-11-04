use std::sync::{Mutex,atomic::{AtomicI32,Ordering::SeqCst}};
// java public class ConcurrentArrayQueue<T> {
pub struct ConcurrentArrayQueue<T:Clone>{
    // java private final Object[] ring;
    ring:Vec<Mutex<Option<T>>>,
    // java private final AtomicInteger maximumReadIndex = new AtomicInteger(0);
    maximumReadIndex:AtomicI32,
    // java private final AtomicInteger readIndex = new AtomicInteger(0);
    readIndex:AtomicI32,
    // java private final AtomicInteger writeIndex = new AtomicInteger(0);
    writeIndex:AtomicI32,
}
impl<T:Clone> ConcurrentArrayQueue<T>{
    // java public ConcurrentArrayQueue(int capacity) {
    pub fn ConcurrentArrayQueue(capacity:i32)->Self{
        // java if (capacity < 0) { throw new IllegalArgumentException("illegal capacity " + capacity); } else { this.ring = new Object[capacity + 1]; }
        assert!(capacity>=0,"IllegalArgumentException illegal capacity {capacity}");let size=capacity.checked_add(1).expect("NegativeArraySizeException");Self{ring:(0..size).map(|_|Mutex::new(None)).collect(),maximumReadIndex:AtomicI32::new(0),readIndex:AtomicI32::new(0),writeIndex:AtomicI32::new(0)}
    }
    // java public boolean push(T e) {
    pub fn push(&self,e:Option<T>)->bool{
        // java int currentWriteIndex; do { int currentReadIndex = this.readIndex.get(); currentWriteIndex = this.writeIndex.get();
        let length=self.ring.len()as i32;let currentWriteIndex=loop{let currentReadIndex=self.readIndex.load(SeqCst);let currentWriteIndex=self.writeIndex.load(SeqCst);
            // java if ((currentWriteIndex + 1) % this.ring.length == currentReadIndex % this.ring.length) { return false; }
            if currentWriteIndex.wrapping_add(1)%length==currentReadIndex%length{return false;}
            // java } while(!this.writeIndex.compareAndSet(currentWriteIndex, currentWriteIndex + 1));
            if self.writeIndex.compare_exchange(currentWriteIndex,currentWriteIndex.wrapping_add(1),SeqCst,SeqCst).is_ok(){break currentWriteIndex;}
        };
        // java this.ring[currentWriteIndex % this.ring.length] = e;
        *self.ring[(currentWriteIndex%length)as usize].lock().unwrap()=e;
        // java while(!this.maximumReadIndex.compareAndSet(currentWriteIndex, currentWriteIndex + 1)) { Thread.yield(); }
        while self.maximumReadIndex.compare_exchange(currentWriteIndex,currentWriteIndex.wrapping_add(1),SeqCst,SeqCst).is_err(){std::thread::yield_now();}
        // java return true;
        true
    }
    // java public boolean tryPush(T e) {
    pub fn tryPush(&self,e:Option<T>)->bool{
        // java int currentReadIndex = this.readIndex.get(); int currentWriteIndex = this.writeIndex.get();
        let currentReadIndex=self.readIndex.load(SeqCst);let currentWriteIndex=self.writeIndex.load(SeqCst);let length=self.ring.len()as i32;
        // java if ((currentWriteIndex + 1) % this.ring.length == currentReadIndex % this.ring.length) { return false; }
        if currentWriteIndex.wrapping_add(1)%length==currentReadIndex%length{return false;}
        // java else if (!this.writeIndex.compareAndSet(currentWriteIndex, currentWriteIndex + 1)) { return false; }
        if self.writeIndex.compare_exchange(currentWriteIndex,currentWriteIndex.wrapping_add(1),SeqCst,SeqCst).is_err(){return false;}
        // java this.ring[currentWriteIndex % this.ring.length] = e;
        *self.ring[(currentWriteIndex%length)as usize].lock().unwrap()=e;
        // java while(!this.maximumReadIndex.compareAndSet(currentWriteIndex, currentWriteIndex + 1)) { Thread.yield(); }
        while self.maximumReadIndex.compare_exchange(currentWriteIndex,currentWriteIndex.wrapping_add(1),SeqCst,SeqCst).is_err(){std::thread::yield_now();}
        // java return true;
        true
    }
    // java public T pop() {
    pub fn pop(&self)->Option<T>{
        // java int currentReadIndex; T ret; do {
        let length=self.ring.len()as i32;loop{
            // java currentReadIndex = this.readIndex.get(); int currentMaximumReadIndex = this.maximumReadIndex.get();
            let currentReadIndex=self.readIndex.load(SeqCst);let currentMaximumReadIndex=self.maximumReadIndex.load(SeqCst);
            // java if (currentReadIndex % this.ring.length == currentMaximumReadIndex % this.ring.length) { return null; }
            if currentReadIndex%length==currentMaximumReadIndex%length{return None;}
            // java ret = (T)this.ring[currentReadIndex % this.ring.length];
            let ret=self.ring[(currentReadIndex%length)as usize].lock().unwrap().clone();
            // java } while(!this.readIndex.compareAndSet(currentReadIndex, currentReadIndex + 1)); return ret;
            if self.readIndex.compare_exchange(currentReadIndex,currentReadIndex.wrapping_add(1),SeqCst,SeqCst).is_ok(){return ret;}
        }
    }
    // java public T tryPop() {
    pub fn tryPop(&self)->Option<T>{
        // java int currentReadIndex = this.readIndex.get(); int currentMaximumReadIndex = this.maximumReadIndex.get();
        let currentReadIndex=self.readIndex.load(SeqCst);let currentMaximumReadIndex=self.maximumReadIndex.load(SeqCst);let length=self.ring.len()as i32;
        // java if (currentReadIndex % this.ring.length == currentMaximumReadIndex % this.ring.length) { return null; }
        if currentReadIndex%length==currentMaximumReadIndex%length{return None;}
        // java T ret = (T)this.ring[currentReadIndex % this.ring.length];
        let ret=self.ring[(currentReadIndex%length)as usize].lock().unwrap().clone();
        // java return (T)(this.readIndex.compareAndSet(currentReadIndex, currentReadIndex + 1) ? ret : null);
        if self.readIndex.compare_exchange(currentReadIndex,currentReadIndex.wrapping_add(1),SeqCst,SeqCst).is_ok(){ret}else{None}
    }
    // java public int size() { int ret = this.writeIndex.get() - this.readIndex.get(); return ret < 0 ? 0 : ret; }
    pub fn size(&self)->i32{let ret=self.writeIndex.load(SeqCst).wrapping_sub(self.readIndex.load(SeqCst));if ret<0{0}else{ret}}
    // java public void clear() { while(this.size() > 0) { this.pop(); } }
    pub fn clear(&self){while self.size()>0{self.pop();}}
}
