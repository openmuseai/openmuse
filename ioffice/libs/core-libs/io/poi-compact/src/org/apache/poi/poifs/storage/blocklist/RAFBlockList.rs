// Java: import cn.wps.base.assertion.IOAssert
// Java: import cn.wps.base.log.IOLog
// Java: import cn.wps.moffice.kfs.RandomAccessFile
// Java: import cn.wps.moffice.util.StreamUtil
// Java: import java.io.IOException
// Java: import org.apache.poi.poifs.common.BlockBuf
use crate::org::apache::poi::poifs::common::BlockBuf::BlockBuf;
// Java: import org.apache.poi.poifs.common.POIFSBigBlockSize
use crate::org::apache::poi::poifs::common::POIFSBigBlockSize::POIFSBigBlockSize;
use super::IBlockList::IBlockList;

use std::io::{Read, Seek, SeekFrom};

// Java field: String TAG
pub(crate) const TAG: &str = "RAFBlockList";

// Java: public class RAFBlockList implements IBlockList
pub struct RAFBlockList {
    // Java field: RandomAccessFile randomAccessFile
    pub(crate) randomAccessFile: Option<Box<dyn ReadSeek>>,
    // Java field: int blockSize
    pub(crate) blockSize: i32,
    // Java field: int length
    pub(crate) length: i32,
}

pub trait ReadSeek: Read + Seek {}
impl<T: Read + Seek> ReadSeek for T {}

// Java: public class RAFBlockList implements IBlockList
impl RAFBlockList {
    // Java: RAFBlockList(RandomAccessFile raf, POIFSBigBlockSize bsize)
    pub fn new(mut raf: Box<dyn ReadSeek>, bsize: &POIFSBigBlockSize) -> Self {
        // Java: super();
        // Java: this.randomAccessFile = raf;
        // Java: this.blockSize = bsize.getBigBlockSize();
        let blockSize = bsize.getBigBlockSize();
        // Java: try
        // Java: this.length = (int)raf.length();
        let length = raf.seek(SeekFrom::End(0)).unwrap_or(0) as i32;
        let _ = raf.seek(SeekFrom::Start(0));
        // Java: catch (IOException e)
        // Java: throw new RuntimeException(e);
        RAFBlockList {
            randomAccessFile: Some(raf),
            blockSize,
            length,
        }
    }

    // Java: void dispose()
    pub fn dispose_impl(&mut self) {
        // Java: if (null != this.randomAccessFile)
        if self.randomAccessFile.is_some() {
            // Java: StreamUtil.close(this.randomAccessFile);
            // Java: this.randomAccessFile = null;
            self.randomAccessFile = None;
        }
    }

    // Java: BlockBuf getBlockAt(int block_index)
    pub fn getBlockAt_impl(&mut self, block_index: i32) -> BlockBuf {
        // Java: IOAssert.assertTrue("(block_index >= 0) should be true.", block_index >= 0);
        assert!(block_index >= 0, "(block_index >= 0) should be true.");
        // Java: try
        let raf = self.randomAccessFile.as_mut().unwrap();
        // Java: long pos = (long)((block_index + 1) * this.blockSize);
        let pos = ((block_index + 1) * self.blockSize) as i64;
        // Java: this.randomAccessFile.seek(pos);
        let _ = raf.seek(SeekFrom::Start(pos as u64));
        // Java: BlockBuf buf = BlockBuf.obtain(this.blockSize);
        let mut buf = BlockBuf::obtain(self.blockSize);
        // Java: if (pos < (long)this.length && (long)this.length < pos + (long)this.blockSize)
        if pos < self.length as i64 && (self.length as i64) < pos + self.blockSize as i64 {
            // Java: this.randomAccessFile.read(buf.getBuf());
            let _ = raf.read(buf.getBuf());
        }
        // Java: else
        else {
            // Java: this.randomAccessFile.readFully(buf.getBuf(), 0, this.blockSize);
            let bs = self.blockSize as usize;
            let mut total = 0;
            while total < bs {
                match raf.read(&mut buf.getBuf()[total..bs]) {
                    Ok(0) => break,
                    Ok(n) => total += n,
                    Err(_) => break,
                }
            }
        }
        // Java: return buf;
        buf
        // Java: catch (IOException e)
        // Java: IOLog.e(TAG, "IOException", e);
        // Java: return null;
    }

    // Java: boolean readBlock(int blockIndex, BlockBuf buf)
    pub fn readBlock_impl(&mut self, blockIndex: i32, buf: &mut BlockBuf) -> bool {
        // Java: long pos = (long)((blockIndex + 1) * this.blockSize);
        let pos = ((blockIndex + 1) * self.blockSize) as i64;
        // Java: synchronized(this)
        let raf = match self.randomAccessFile.as_mut() {
            Some(r) => r,
            None => return false,
        };
        // Java: boolean var10000;
        // Java: try
        // Java: this.randomAccessFile.seek(pos);
        if raf.seek(SeekFrom::Start(pos as u64)).is_err() {
            // Java: catch (IOException e)
            // Java: IOLog.e(TAG, "IOException", e);
            // Java: return false;
            return false;
        }
        // Java: if (pos < (long)this.length && pos + (long)this.blockSize > (long)this.length)
        if pos < self.length as i64 && pos + self.blockSize as i64 > self.length as i64 {
            // Java: this.randomAccessFile.read(buf.getBuf());
            let _ = raf.read(buf.getBuf());
        }
        // Java: else
        else {
            // Java: this.randomAccessFile.readFully(buf.getBuf(), 0, this.blockSize);
            let bs = self.blockSize as usize;
            let mut total = 0;
            while total < bs {
                match raf.read(&mut buf.getBuf()[total..bs]) {
                    Ok(0) => break,
                    Ok(n) => total += n,
                    Err(_) => return false,
                }
            }
        }
        // Java: var10000 = true;
        // Java: catch (IOException e)
        // Java: IOLog.e(TAG, "IOException", e);
        // Java: return false;
        // Java: return var10000;
        true
    }

    // Java: int getBlockCount()
    pub fn getBlockCount_impl(&self) -> i32 {
        // Java: return (this.length + this.blockSize - 1) / this.blockSize;
        (self.length + self.blockSize - 1) / self.blockSize
    }

    // Java: int getBlockSize()
    pub fn getBlockSize_impl(&self) -> i32 {
        // Java: return this.blockSize;
        self.blockSize
    }
}

impl IBlockList for RAFBlockList {
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
