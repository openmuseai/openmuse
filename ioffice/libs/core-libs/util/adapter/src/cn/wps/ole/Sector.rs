// Java: public class Sector
pub struct Sector {
    // Java field: int index
    index: i32,
    // Java field: boolean _dirty
    pub(crate) _dirty: bool,
    // Java field: byte[] content
    pub(crate) content: Vec<u8>,
    // Java field: int priority
    priority: i32,
    // Java field: int mQueueIndex
    mQueueIndex: i32,
    // Java field: int lockCount
    pub(crate) lockCount: i32,
}

// Java: public class Sector
impl Sector {
    // Java: Sector(int index, int size)
    pub fn new(_index: i32, _size: i32) -> Self {
      // Java: super();

      // Java: this._dirty = false;

      // Java: this.priority = 0;

      // Java: this.mQueueIndex = 0;

      // Java: this.lockCount = 0;

      // Java: if (size <= 0)

      {
         // Java: throw new IllegalArgumentException("Invalid sector size " + size);

      }
      // Java: else

      {
         // Java: this.index = index;

         // Java: this.content = new byte[size];

      }
      todo!()
    }

    // Java: Sector(int size)
    pub fn new_with_size(_size: i32) -> Self {
      // Java: this(0, size);

      Self::new(0, _size)
    }

    // Java: void reset()
    pub fn reset(&mut self) {
      // Java: this.index = 0;

      // Java: this._dirty = false;

      // Java: this.priority = 0;

      // Java: this.mQueueIndex = 0;

    }

    // Java: int size()
    pub fn size(&self) -> i32 {
      // Java: return this.content.length;

      self.content.len() as i32
    }

    // Java: byte[] getContentBuffer()
    pub fn getContentBuffer(&self) -> &[u8] {
      // Java: return this.content;

      &self.content
    }

    // Java: boolean isDirty()
    pub fn isDirty(&self) -> bool {
      // Java: return this._dirty;

      self._dirty
    }

    // Java: void setDirty()
    pub fn setDirty(&mut self) {
      // Java: this._dirty = true;

      self._dirty = true;
    }

    // Java: void clearDirty()
    pub fn clearDirty(&mut self) {
      // Java: this._dirty = false;

      self._dirty = false;
    }

    // Java: void lock()
    pub fn lock(&mut self) {
      // Java: ++this.lockCount;

      self.lockCount += 1;
    }

    // Java: void unlock()
    pub fn unlock(&mut self) {
      // Java: --this.lockCount;

      self.lockCount -= 1;
    }

    // Java: boolean isLocked()
    pub fn isLocked(&self) -> bool {
      // Java: return this.lockCount > 0;

      self.lockCount > 0
    }

}
