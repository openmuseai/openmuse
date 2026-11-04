// Java: import java.io.IOException
// Java: import java.io.InputStream
// Java: import java.util.ArrayList
// Java: import java.util.List
// Java: import org.apache.poi.poifs.common.BlockBuf
use crate::org::apache::poi::poifs::common::BlockBuf::BlockBuf;
use super::IBlockList::IBlockList;

// Java: public class ByteArrayBlockList implements IBlockList
pub struct ByteArrayBlockList {
    // Java field: int blockSize
    pub(crate) blockSize: i32,
    // Java field: List<BlockBuf> byteArrays
    pub(crate) byteArrays: Vec<BlockBuf>,
}

// Java: public class ByteArrayBlockList implements IBlockList
impl ByteArrayBlockList {
    // Java: ByteArrayBlockList(int blockSize)
    pub fn new(blockSize: i32) -> Self {
        // Java: super();
        // Java: this.blockSize = blockSize;
        // Java: this.byteArrays = new ArrayList();
        ByteArrayBlockList {
            blockSize,
            byteArrays: Vec::new(),
        }
    }

    // Java: ByteArrayBlockList(InputStream in, int blockSize)
    pub fn new_from_stream(input: &mut dyn std::io::Read, blockSize: i32, size: i32) -> std::io::Result<Self> {
        // Java: this(blockSize);
        let mut result = Self::new(blockSize);
        // Java: int size = in.available();
        let size = size as usize;
        // Java: int readed = 0;
        let mut readed: usize = 0;
        // Java: while(readed < size)
        while readed < size {
            // Java: BlockBuf buf = BlockBuf.obtain(blockSize);
            let mut buf = BlockBuf::obtain(blockSize);
            // Java: buf.setRecyclable(false);
            buf.setRecyclable(false);
            // Java: int l = in.read(buf.getBuf());
            let l = input.read(buf.getBuf()).unwrap_or(0);
            // Java: if (l > 0)
            if l > 0 {
                // Java: this.byteArrays.add(buf);
                result.byteArrays.push(buf);
                // Java: readed += l;
                readed += l;
            }
            // Java: if (l != blockSize)
            if l != blockSize as usize {
                // Java: break;
                break;
            }
        }
        // Java: if (readed != size)
        if readed != size {
            // Java: throw new IOException();
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "read size mismatch"));
        }
        Ok(result)
    }

    // Java: void dispose()
    pub fn dispose(&mut self) {
        // Java: if (null != this.byteArrays)
        // Java: int i = 0;
        // Java: for(int n = this.byteArrays.size(); i < n; ++i)
        for buf in self.byteArrays.drain(..) {
            // Java: BlockBuf buf = (BlockBuf)this.byteArrays.get(i);
            // Java: buf.setRecyclable(true);
            // Java: buf.recycle();
            let mut b = buf;
            b.setRecyclable(true);
            b.recycle();
        }
        // Java: this.byteArrays = null;
    }

    // Java: void add(BlockBuf buf)
    pub fn add(&mut self, mut buf: BlockBuf) {
        // Java: if (buf.getBuf().length != this.blockSize)
        if buf.getBuf().len() != self.blockSize as usize {
            // Java: throw new IllegalArgumentException("Invalid length of byte array");
            panic!("Invalid length of byte array");
        }
        // Java: else
        else {
            // Java: this.byteArrays.add(buf);
            self.byteArrays.push(buf);
        }
    }
}

impl IBlockList for ByteArrayBlockList {
    // Java: BlockBuf getBlockAt(int block_index)
    fn getBlockAt(&mut self, block_index: i32) -> BlockBuf {
        // Java: return (BlockBuf)this.byteArrays.get(block_index);
        let idx = block_index as usize;
        std::mem::replace(&mut self.byteArrays[idx], BlockBuf::obtain(self.blockSize))
    }

    // Java: int getBlockCount()
    fn getBlockCount(&self) -> i32 {
        // Java: return this.byteArrays.size();
        self.byteArrays.len() as i32
    }

    // Java: int getBlockSize()
    fn getBlockSize(&self) -> i32 {
        // Java: return this.blockSize;
        self.blockSize
    }

    // Java: boolean readBlock(int blockIndex, BlockBuf buf)
    fn readBlock(&mut self, blockIndex: i32, mut buf: BlockBuf) -> bool {
        // Java: byte[] src = ((BlockBuf)this.byteArrays.get(blockIndex)).getBuf();
        let idx = blockIndex as usize;
        let src = self.byteArrays[idx].getBuf().to_vec();
        // Java: System.arraycopy(src, 0, buf.getBuf(), 0, src.length);
        let len = src.len();
        buf.getBuf()[..len].copy_from_slice(&src);
        // Java: return true;
        true
    }

    fn dispose(&mut self) {
        self.dispose();
    }
}
