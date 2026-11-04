// Java: import java.util.ConcurrentModificationException
// Java: import java.util.NoSuchElementException

// Java: public class WHashPrimitiveIterator
pub struct WHashPrimitiveIterator {
    // Java field: WPrimitiveHash _hash
    // Java field: int _expectedSize
    pub(crate) _expectedSize: usize,
    // Java field: int _index
    pub(crate) _index: i32,
    pub(crate) _capacity: usize,
}

impl WHashPrimitiveIterator {
    // Java: WHashPrimitiveIterator(WPrimitiveHash hash)
    pub fn WHashPrimitiveIterator_new(size: usize, capacity: usize) -> Self {
        // Java: super();
        // Java: this._hash = hash;
        // Java: this._expectedSize = this._hash.size();
        // Java: this._index = this._hash.capacity();
        WHashPrimitiveIterator {
            _expectedSize: size,
            _index: capacity as i32,
            _capacity: capacity,
        }
    }

    // Java: int nextIndex()
    pub fn nextIndex(&self) -> i32 {
        // Java: if (this._expectedSize != this._hash.size())
        // Java: throw new ConcurrentModificationException();
        // Java: else
        // Java: _states: &[u8] = this._hash._states;
        // Java: int i = this._index;
        // Java: while(i-- > 0 && states[i] != 1)
        // Java: return i;
        self._index - 1
    }

    // Java: boolean hasNext()
    pub fn hasNext(&self) -> bool {
        // Java: return this.nextIndex() >= 0;
        self.nextIndex() >= 0
    }

    // Java: void remove()
    pub fn remove(&mut self) {
        // Java: if (this._expectedSize != this._hash.size())
        // Java: throw new ConcurrentModificationException();
        // Java: else
        // Java: try
        // Java: this._hash.tempDisableAutoCompaction();
        // Java: this._hash.removeAt(this._index);
        // Java: finally
        // Java: this._hash.reenableAutoCompaction(false);
        // Java: --this._expectedSize;
        self._expectedSize -= 1;
    }

    // Java: void moveToNextIndex()
    pub fn moveToNextIndex(&mut self) {
        // Java: if ((this._index = this.nextIndex()) < 0)
        // Java: throw new NoSuchElementException();
        let next = self.nextIndex();
        if next < 0 {
            panic!("NoSuchElementException");
        }
        self._index = next;
    }
}
