// Java field: ThreadLocal<byte[]> threadLocal
use std::cell::RefCell;

thread_local! {
    static THREAD_LOCAL: RefCell<Option<Vec<u8>>> = RefCell::new(None);
}

// Java: public class ThreadBufferUtils
pub struct ThreadBufferUtils {
}

// Java: public class ThreadBufferUtils
impl ThreadBufferUtils {
    // Java: ThreadBufferUtils()
    pub fn new() -> Self {
      // Java: super();
      Self {}
    }

    // Java: static _getBytes: &[u8]()
    pub fn getBytes() -> Vec<u8> {
      // Java: _bytes: &[u8] = (byte[])threadLocal.get();
      THREAD_LOCAL.with(|tl| {
        let mut tl = tl.borrow_mut();
        // Java: if (bytes == null)
        if tl.is_none()
        {
           // Java: bytes = new byte[10240];
           // Java: threadLocal.set(bytes);
           *tl = Some(vec![0u8; 10240]);
        }
        // Java: return bytes;
        tl.as_ref().unwrap().clone()
      })
    }

}
