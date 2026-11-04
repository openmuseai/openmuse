// Java: import java.nio.ByteBuffer
// Java: import org.apache.poi.poifs.common.BlockBuf
use crate::org::apache::poi::poifs::common::BlockBuf::BlockBuf;
// Java: import org.apache.poi.poifs.common.POIFSBigBlockSize
use crate::org::apache::poi::poifs::common::POIFSBigBlockSize::POIFSBigBlockSize;
use super::IBlockList::IBlockList;

// Java: public class ByteBufferBlockList implements IBlockList
pub struct ByteBufferBlockList {
    // Java field: ByteBuffer byteBuffer
    pub(crate) byteBuffer: Vec<u8>,
    // Java field: int blockSize
    pub(crate) blockSize: i32,
    // Java field: int startPos
    pub(crate) startPos: i32,
    // Java field: int length
    pub(crate) length: i32,
}

// Java: public class ByteBufferBlockList implements IBlockList
impl ByteBufferBlockList {
    // Java: ByteBufferBlockList(ByteBuffer bb, POIFSBigBlockSize bsize)
    pub fn new(bb: Vec<u8>, startPos: i32, bsize: &POIFSBigBlockSize) -> Self {
        // Java: super();
        // Java: this.byteBuffer = bb;
        // Java: this.blockSize = bsize.getBigBlockSize();
        let blockSize = bsize.getBigBlockSize();
        // Java: this.startPos = bb.position();
        // Java: this.length = bb.remaining() + this.startPos;
        let length = bb.len() as i32;
        ByteBufferBlockList {
            byteBuffer: bb,
            blockSize,
            startPos,
            length,
        }
    }

    // Java: void dispose()
    pub fn dispose_impl(&mut self) {
        // Java: if (null != this.byteBuffer)
        // Java: this.byteBuffer = null;
        self.byteBuffer.clear();
    }

    // Java: BlockBuf getBlockAt(int block_index)
    pub fn getBlockAt_impl(&mut self, block_index: i32) -> BlockBuf {
        // Java: int pos = this.startPos + block_index * this.blockSize;
        let pos = self.startPos + block_index * self.blockSize;
        let p = pos as usize;
        let bs = self.blockSize as usize;
        let len = self.length as usize;
        // Java: this.byteBuffer.position(pos);
        // Java: BlockBuf buf = BlockBuf.obtain(this.blockSize);
        let mut buf = BlockBuf::obtain(self.blockSize);
        // Java: if (pos + this.blockSize <= this.length)
        if p + bs <= len {
            // Java: this.byteBuffer.get(buf.getBuf());
            buf.getBuf()[..bs].copy_from_slice(&self.byteBuffer[p..p + bs]);
        }
        // Java: else
        else {
            // Java: if (pos >= this.length)
            if p >= len {
                // Java: throw new RuntimeException("position out of range");
                panic!("position out of range");
            }
            // Java: this.byteBuffer.get(buf.getBuf(), 0, this.length - pos);
            let remaining = len - p;
            buf.getBuf()[..remaining].copy_from_slice(&self.byteBuffer[p..len]);
        }
        // Java: return buf;
        buf
    }

    // Java: int getBlockCount()
    pub fn getBlockCount_impl(&self) -> i32 {
        // Java: return (this.length - this.startPos + this.blockSize - 1) / this.blockSize;
        (self.length - self.startPos + self.blockSize - 1) / self.blockSize
    }

    // Java: int getBlockSize()
    pub fn getBlockSize_impl(&self) -> i32 {
        // Java: return this.blockSize;
        self.blockSize
    }

    // Java: boolean readBlock(int blockIndex, BlockBuf buf)
    pub fn readBlock_impl(&self, blockIndex: i32, buf: &mut BlockBuf) -> bool {
        // Java: int pos = this.startPos + blockIndex * this.blockSize;
        let pos = self.startPos + blockIndex * self.blockSize;
        let p = pos as usize;
        let bs = self.blockSize as usize;
        let len = self.length as usize;
        // Java: this.byteBuffer.position(pos);
        // Java: if (pos + this.blockSize <= this.length)
        if p + bs <= len {
            // Java: this.byteBuffer.get(buf.getBuf());
            buf.getBuf()[..bs].copy_from_slice(&self.byteBuffer[p..p + bs]);
        }
        // Java: else
        else {
            // Java: if (pos >= this.length)
            if p >= len {
                // Java: throw new RuntimeException("position out of range");
                panic!("position out of range");
            }
            // Java: this.byteBuffer.get(buf.getBuf(), 0, this.length - pos);
            let remaining = len - p;
            buf.getBuf()[..remaining].copy_from_slice(&self.byteBuffer[p..len]);
        }
        // Java: return true;
        true
    }
}

impl IBlockList for ByteBufferBlockList {
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
