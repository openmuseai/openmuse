// Java: import org.apache.poi.poifs.common.POIFSBigBlockSize
use crate::org::apache::poi::poifs::common::POIFSBigBlockSize::POIFSBigBlockSize;
use std::io::Write;

// Java: public class BigBlock implements BlockWritable
pub struct BigBlock {
    // Java field: POIFSBigBlockSize bigBlockSize
    pub(crate) bigBlockSize: POIFSBigBlockSize,
}

// Java: public class BigBlock implements BlockWritable
impl BigBlock {
    // Java: BigBlock(POIFSBigBlockSize bigBlockSize)
    pub fn new(bigBlockSize: POIFSBigBlockSize) -> Self {
        // Java: super();
        // Java: this.bigBlockSize = bigBlockSize;
        BigBlock { bigBlockSize }
    }

    // Java: void doWriteData(OutputStream stream, byte[] data)
    pub fn doWriteData(stream: &mut dyn Write, data: &[u8]) {
        // Java: stream.write(data);
        stream.write_all(data).unwrap();
    }

    // Java: abstract abstract void writeData(OutputStream var1)

    // Java: void writeBlocks(OutputStream stream)
    pub fn writeBlocks(&self, _stream: &mut dyn Write) {
        // Java: this.writeData(stream);
        panic!("writeData must be implemented by subclass");
    }
}
