use crate::java_runtime::RefCell;

thread_local! {
    static LOCAL_FML_BYTES: RefCell<Vec<u8>> = RefCell::new(vec![0u8; 11]);
}

// java final class LocalFmlBytes extends ThreadLocal<byte[]>
pub struct LocalFmlBytes;

// java final class LocalFmlBytes extends ThreadLocal<byte[]>
impl LocalFmlBytes {
    // java final class LocalFmlBytes extends ThreadLocal<byte[]>
    pub fn LocalFmlBytes() -> Self {
        let init = Self::initialValue();
        LOCAL_FML_BYTES.with(|cell| {
            *cell.borrow_mut() = init;
        });
        LocalFmlBytes
    }

    // java protected byte[] initialValue()
    pub fn initialValue() -> Vec<u8> {
        // java return new byte[11];
        vec![0u8; 11]
    }

    pub fn with_bytes<R>(&self, f: impl FnOnce(&mut [u8]) -> R) -> R {
        LOCAL_FML_BYTES.with(|cell| {
            let mut buf = cell.borrow_mut();
            if buf.len() != 11 {
                *buf = Self::initialValue();
            }
            f(&mut buf)
        })
    }
}
