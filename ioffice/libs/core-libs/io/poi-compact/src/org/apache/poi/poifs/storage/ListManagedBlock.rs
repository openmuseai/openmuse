// Java: import java.io.IOException
use std::io;

// Java: public interface ListManagedBlock
pub trait ListManagedBlock {
    // Java: abstract byte[] getData()
    fn getData(&self) -> io::Result<Vec<u8>>;
}
