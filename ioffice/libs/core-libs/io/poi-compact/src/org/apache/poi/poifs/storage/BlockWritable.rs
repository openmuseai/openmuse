// Java: import java.io.IOException
// Java: import java.io.OutputStream
use std::io::{self, Write};

// Java: public interface BlockWritable
pub trait BlockWritable {
    // Java: abstract void writeBlocks(OutputStream var1)
    fn writeBlocks(&self, var1: &mut dyn Write) -> io::Result<()>;
}
