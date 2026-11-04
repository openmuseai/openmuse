// Java: import java.io.IOException
// Java: import java.io.InputStream
// Java: import org.apache.poi.poifs.common.BlockBuf
use crate::org::apache::poi::poifs::common::BlockBuf::BlockBuf;
use super::IBlockList::IBlockList;

// Java: public class SequencialBlockList implements IBlockList
pub struct SequencialBlockList {
    // Java field: int blockSize
    pub(crate) blockSize: i32,
    // Java field: int nextBlockIndex
    pub(crate) nextBlockIndex: i32,
    // Java field: int totalLength
    pub(crate) totalLength: i32,
    // Java field: InputStream inputStream
    pub(crate) inputStream: Box<dyn std::io::Read>,
}

// Java: public class SequencialBlockList implements IBlockList
impl SequencialBlockList {
    // Java: SequencialBlockList(InputStream is, int blockSize)
    pub fn new(is: Box<dyn std::io::Read>, blockSize: i32, totalLength: i32) -> Self {
        // Java: super();
        // Java: this.inputStream = is;
        // Java: try
        // Java: this.totalLength = is.available();
        // Java: catch (IOException e)
        // Java: throw new RuntimeException(e);
        // Java: this.blockSize = blockSize;
        // Java: this.nextBlockIndex = 0;
        SequencialBlockList {
            blockSize,
            nextBlockIndex: 0,
            totalLength,
            inputStream: is,
        }
    }

    // Java: void dispose()
    pub fn dispose_impl(&mut self) {
    }

    // Java: BlockBuf getBlockAt(int block_index)
    pub fn getBlockAt_impl(&mut self, block_index: i32) -> BlockBuf {
        // Java: if (block_index != this.nextBlockIndex)
        if block_index != self.nextBlockIndex {
            // Java: throw new IllegalArgumentException("No random access allowed for SequencialBlockList");
            panic!("No random access allowed for SequencialBlockList");
        }
        // Java: else
        else {
            // Java: BlockBuf buf = BlockBuf.obtain(this.blockSize);
            let mut buf = BlockBuf::obtain(self.blockSize);
            // Java: byte[] byteBuf = buf.getBuf();
            let byteBuf = buf.getBuf();
            // Java: int remained;
            let mut remained: i32;
            // Java: int readed;
            let mut readed: i32;
            // Java: for(remained = this.blockSize; remained > 0; remained -= readed)
            remained = self.blockSize;
            readed = 0;
            while remained > 0 {
                remained -= readed;
                let offset = (self.blockSize - remained) as usize;
                let rem = remained as usize;
                // Java: try
                // Java: readed = this.inputStream.read(byteBuf, this.blockSize - remained, remained);
                readed = match self.inputStream.read(&mut byteBuf[offset..offset + rem]) {
                    Ok(n) => n as i32,
                    // Java: catch (IOException e)
                    // Java: throw new RuntimeException(e);
                    Err(e) => panic!("IOException: {}", e),
                };
                // Java: if (readed <= 0)
                if readed <= 0 {
                    // Java: break;
                    break;
                }
            }
            // Java: if (remained == this.blockSize)
            if remained == self.blockSize {
                // Java: throw new IllegalStateException("Nothing readed from InputStream");
                panic!("Nothing readed from InputStream");
            }
            // Java: else
            else {
                // Java: ++this.nextBlockIndex;
                self.nextBlockIndex += 1;
                // Java: return buf;
                buf
            }
        }
    }

    // Java: int getBlockCount()
    pub fn getBlockCount_impl(&self) -> i32 {
        // Java: return (this.totalLength + this.blockSize - 1) / this.blockSize;
        (self.totalLength + self.blockSize - 1) / self.blockSize
    }

    // Java: int getBlockSize()
    pub fn getBlockSize_impl(&self) -> i32 {
        // Java: return this.blockSize;
        self.blockSize
    }

    // Java: boolean readBlock(int blockIndex, BlockBuf buf)
    pub fn readBlock_impl(&mut self, blockIndex: i32, buf: &mut BlockBuf) -> bool {
        // Java: if (blockIndex != this.nextBlockIndex)
        if blockIndex != self.nextBlockIndex {
            // Java: throw new IllegalArgumentException("No random access allowed for SequencialBlockList");
            panic!("No random access allowed for SequencialBlockList");
        }
        // Java: else
        else {
            // Java: byte[] byteBuf = buf.getBuf();
            let byteBuf = buf.getBuf();
            // Java: int remained;
            let mut remained: i32;
            // Java: int readed;
            let mut readed: i32;
            // Java: for(remained = this.blockSize; remained > 0; remained -= readed)
            remained = self.blockSize;
            readed = 0;
            while remained > 0 {
                remained -= readed;
                let offset = (self.blockSize - remained) as usize;
                let rem = remained as usize;
                // Java: try
                // Java: readed = this.inputStream.read(byteBuf, this.blockSize - remained, remained);
                readed = match self.inputStream.read(&mut byteBuf[offset..offset + rem]) {
                    Ok(n) => n as i32,
                    // Java: catch (IOException e)
                    // Java: throw new RuntimeException(e);
                    Err(e) => panic!("IOException: {}", e),
                };
                // Java: if (readed <= 0)
                if readed <= 0 {
                    // Java: break;
                    break;
                }
            }
            // Java: if (remained == this.blockSize)
            if remained == self.blockSize {
                // Java: throw new IllegalStateException("Nothing readed from InputStream");
                panic!("Nothing readed from InputStream");
            }
            // Java: else
            else {
                // Java: ++this.nextBlockIndex;
                self.nextBlockIndex += 1;
                // Java: return true;
                true
            }
        }
    }
}

impl IBlockList for SequencialBlockList {
    fn getBlockAt(&mut self, block_index: i32) -> BlockBuf {
        self.getBlockAt_impl(block_index)
    }
    fn readBlock(&mut self, var1: i32, var2: BlockBuf) -> bool {
        let mut b = var2;
        self.readBlock_impl(var1, &mut b)
    }
    fn getBlockCount(&self) -> i32 {
        self.getBlockCount_impl()
    }
    fn getBlockSize(&self) -> i32 {
        self.getBlockSize_impl()
    }
    fn dispose(&mut self) {
        self.dispose_impl();
    }
}
