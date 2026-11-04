use std::sync::Mutex;

// Java field: int SMALL_PAGE_BUFFER_SIZE
pub const SMALL_PAGE_BUFFER_SIZE: i32 = 64;
// Java field: int BIG_PAGE_BUFFER_SIZE
pub const BIG_PAGE_BUFFER_SIZE: i32 = 512;
// Java field: byte[] zeroByteArray16
pub static ZERO_BYTE_ARRAY_16: [u8; 16] = [0u8; 16];
// Java field: int PAGE_BUFFER_SIZE
pub const PAGE_BUFFER_SIZE: i32 = 512;
// Java field: int MAX_HOLD_COUNT
pub const MAX_HOLD_COUNT: i32 = 4;

// Java: public class PageBuffer
pub struct PageBuffer {
    // Java field: byte[] _buffer
    pub(crate) _buffer: Vec<u8>,
    // Java field: PageBuffer _next
    pub(crate) _next: Option<Box<PageBuffer>>,
}

// Java: public enum Pool
pub struct Pool {
    // Java field: int _pageSize
    pub(crate) _pageSize: i32,
    // Java field: PageBuffer _header
    pub(crate) _header: Option<Box<PageBuffer>>,
    // Java field: int _count
    pub(crate) _count: i32,
}

static INSTANCE_SMALL: Mutex<Option<Pool>> = Mutex::new(None);
static INSTANCE_BIG: Mutex<Option<Pool>> = Mutex::new(None);

fn get_or_init_small() -> &'static Mutex<Option<Pool>> {
    let mut lock = INSTANCE_SMALL.lock().unwrap();
    if lock.is_none() {
        *lock = Some(Pool::new(SMALL_PAGE_BUFFER_SIZE));
    }
    drop(lock);
    &INSTANCE_SMALL
}

fn get_or_init_big() -> &'static Mutex<Option<Pool>> {
    let mut lock = INSTANCE_BIG.lock().unwrap();
    if lock.is_none() {
        *lock = Some(Pool::new(BIG_PAGE_BUFFER_SIZE));
    }
    drop(lock);
    &INSTANCE_BIG
}

// Java: public class PageBuffer
impl PageBuffer {
    // Java: PageBuffer(int pageSize)
    pub fn new(pageSize: i32) -> Self {
        // Java: super();
        // Java: this._buffer = new byte[pageSize];
        PageBuffer {
            _buffer: vec![0u8; pageSize as usize],
            _next: None,
        }
    }

    // Java: byte[] getByteArray()
    pub fn getByteArray(&self) -> &[u8] {
        // Java: return this._buffer;
        &self._buffer
    }

    pub fn getByteArrayMut(&mut self) -> &mut [u8] {
        &mut self._buffer
    }

    // Java: static PageBuffer obtain(int size)
    pub fn obtain(size: i32) -> PageBuffer {
        // Java: if (size <= 64)
        if size <= 64 {
            // Java: return PageBuffer.Pool.InstanceSmall.obtain();
            let mut lock = get_or_init_small().lock().unwrap();
            return lock.as_mut().unwrap().obtain();
        }
        // Java: else
        else {
            // Java: return size <= 512 ? PageBuffer.Pool.InstanceBig.obtain() : new PageBuffer(size);
            if size <= 512 {
                let mut lock = get_or_init_big().lock().unwrap();
                return lock.as_mut().unwrap().obtain();
            } else {
                return PageBuffer::new(size);
            }
        }
    }

    // Java: void recycle()
    pub fn recycle(mut self) {
        // Java: if (this._buffer.length <= 64)
        if self._buffer.len() <= 64 {
            // Java: PageBuffer.Pool.InstanceSmall.recycle(this);
            let mut lock = get_or_init_small().lock().unwrap();
            lock.as_mut().unwrap().recycle(self);
        }
        // Java: else if (this._buffer.length <= 512)
        else if self._buffer.len() <= 512 {
            // Java: PageBuffer.Pool.InstanceBig.recycle(this);
            let mut lock = get_or_init_big().lock().unwrap();
            lock.as_mut().unwrap().recycle(self);
        }
    }

    // Java: static void clear()
    pub fn clear_pools() {
        // Java: PageBuffer.Pool.InstanceBig.clear();
        let mut lock = get_or_init_big().lock().unwrap();
        lock.as_mut().unwrap().clear();
        drop(lock);
        // Java: PageBuffer.Pool.InstanceSmall.clear();
        let mut lock = get_or_init_small().lock().unwrap();
        lock.as_mut().unwrap().clear();
    }

    // Java: static void fillBy0(PageBuffer pb)
    pub fn fillBy0(pb: &mut PageBuffer) {
        // Java: int size = pb._buffer.length;
        let size = pb._buffer.len();
        // Java: int end = size & -16;
        let end = size & !15;
        // Java: int offset = 0;
        let mut offset = 0;
        let remaining = size & 15;
        // Java: for(size &= 15; offset < end; offset += 16)
        while offset < end {
            // Java: System.arraycopy(zeroByteArray16, 0, pb._buffer, offset, 16);
            pb._buffer[offset..offset + 16].copy_from_slice(&ZERO_BYTE_ARRAY_16);
            offset += 16;
        }
        // Java: if (size != 0)
        if remaining != 0 {
            // Java: System.arraycopy(zeroByteArray16, 0, pb._buffer, offset, size);
            pb._buffer[offset..offset + remaining].copy_from_slice(&ZERO_BYTE_ARRAY_16[..remaining]);
        }
    }
}

// Java: public enum Pool
impl Pool {
    // Java: Pool(int pageSize)
    pub fn new(pageSize: i32) -> Self {
        // Java: this._pageSize = pageSize;
        Pool {
            _pageSize: pageSize,
            _header: None,
            _count: 0,
        }
    }

    // Java: PageBuffer obtain()
    pub fn obtain(&mut self) -> PageBuffer {
        // Java: if (this._header != null)
        if self._header.is_some() {
            // Java: --this._count;
            self._count -= 1;
            // Java: PageBuffer ret = this._header;
            let mut ret = self._header.take().unwrap();
            // Java: this._header = this._header._next;
            self._header = ret._next.take();
            // Java: return ret;
            return *ret;
        }
        // Java: else
        else {
            // Java: return new PageBuffer(this._pageSize);
            return PageBuffer::new(self._pageSize);
        }
    }

    // Java: void recycle(PageBuffer buf)
    pub fn recycle(&mut self, mut buf: PageBuffer) {
        // Java: if (this._count != 4)
        if self._count != MAX_HOLD_COUNT {
            // Java: ++this._count;
            self._count += 1;
            // Java: buf._next = this._header;
            buf._next = self._header.take();
            // Java: this._header = buf;
            self._header = Some(Box::new(buf));
        }
    }

    // Java: int getPageSize()
    pub fn getPageSize(&self) -> i32 {
        // Java: return this._pageSize;
        self._pageSize
    }

    // Java: void clear()
    pub fn clear(&mut self) {
        // Java: while(this._header != null)
        while self._header.is_some() {
            // Java: PageBuffer n = this._header._next;
            let mut header = self._header.take().unwrap();
            // Java: this._header._next = null;
            let n = header._next.take();
            // Java: this._header = n;
            self._header = n;
        }
        // Java: this._count = 0;
        self._count = 0;
    }
}
