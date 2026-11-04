// Java: import org.apache.poi.poifs.common.BlockBuf
use crate::org::apache::poi::poifs::common::BlockBuf::BlockBuf;
// Java: import org.apache.poi.poifs.common.POIFSBigBlockSize
use crate::org::apache::poi::poifs::common::POIFSBigBlockSize::POIFSBigBlockSize;
use super::IBlockList::IBlockList;

// Java field: int SMALL_BLOCK_SIZE
pub(crate) const SMALL_BLOCK_SIZE: i32 = 64;

// Java: public class RawSmallBlockList2 implements IBlockList
pub struct RawSmallBlockList2 {
    // Java field: IBlockList shortStreamContainer
    pub(crate) shortStreamContainer: Option<Box<dyn IBlockList>>,
    // Java field: int bigBlockSize
    pub(crate) bigBlockSize: i32,
    // Java field: int smallBlocksPerBigBlock
    pub(crate) smallBlocksPerBigBlock: i32,
    // Java field: BlockBuf currentBigBlock
    pub(crate) currentBigBlock: Option<BlockBuf>,
    // Java field: int currentBigBlockIndex
    pub(crate) currentBigBlockIndex: i32,
}

// Java: public class RawSmallBlockList2 implements IBlockList
impl RawSmallBlockList2 {
    // Java: RawSmallBlockList2(IBlockList shortStreamContianer, POIFSBigBlockSize bigBlockSize)
    pub fn new(shortStreamContianer: Box<dyn IBlockList>, bigBlockSize: &POIFSBigBlockSize) -> Self {
        // Java: super();
        // Java: this.bigBlockSize = bigBlockSize.getBigBlockSize();
        let bs = bigBlockSize.getBigBlockSize();
        // Java: this.shortStreamContainer = shortStreamContianer;
        // Java: this.smallBlocksPerBigBlock = this.bigBlockSize / 64;
        let smallBlocksPerBigBlock = bs / 64;
        RawSmallBlockList2 {
            shortStreamContainer: Some(shortStreamContianer),
            bigBlockSize: bs,
            smallBlocksPerBigBlock,
            currentBigBlock: None,
            currentBigBlockIndex: -1,
        }
    }

    // Java: void dispose()
    pub fn dispose_impl(&mut self) {
        // Java: if (null != this.shortStreamContainer)
        if let Some(ref mut ssc) = self.shortStreamContainer {
            // Java: this.shortStreamContainer.dispose();
            ssc.dispose();
            // Java: this.shortStreamContainer = null;
        }
        self.shortStreamContainer = None;
        // Java: if (null != this.currentBigBlock)
        if let Some(bb) = self.currentBigBlock.take() {
            // Java: this.currentBigBlock.recycle();
            bb.recycle();
            // Java: this.currentBigBlock = null;
        }
    }

    // Java: BlockBuf getBlockAt(int small_block_index)
    pub fn getBlockAt_impl(&mut self, small_block_index: i32) -> BlockBuf {
        // Java: int bigBlockIndex = small_block_index / this.smallBlocksPerBigBlock;
        let bigBlockIndex = small_block_index / self.smallBlocksPerBigBlock;
        // Java: if (bigBlockIndex != this.currentBigBlockIndex)
        if bigBlockIndex != self.currentBigBlockIndex {
            // Java: if (null != this.currentBigBlock)
            if let Some(bb) = self.currentBigBlock.take() {
                // Java: this.currentBigBlock.recycle();
                bb.recycle();
            }
            // Java: this.currentBigBlockIndex = bigBlockIndex;
            self.currentBigBlockIndex = bigBlockIndex;
            // Java: this.currentBigBlock = this.shortStreamContainer.getBlockAt(this.currentBigBlockIndex);
            if let Some(ref mut ssc) = self.shortStreamContainer {
                self.currentBigBlock = Some(ssc.getBlockAt(self.currentBigBlockIndex));
            }
        }
        // Java: BlockBuf buf = BlockBuf.obtain(64);
        let mut buf = BlockBuf::obtain(64);
        // Java: System.arraycopy(this.currentBigBlock.getBuf(), small_block_index % this.smallBlocksPerBigBlock * 64, buf.getBuf(), 0, 64);
        if let Some(ref mut cbb) = self.currentBigBlock {
            let src_offset = (small_block_index % self.smallBlocksPerBigBlock * 64) as usize;
            let src = cbb.getBuf();
            buf.getBuf()[..64].copy_from_slice(&src[src_offset..src_offset + 64]);
        }
        // Java: return buf;
        buf
    }

    // Java: int getBlockCount()
    pub fn getBlockCount_impl(&self) -> i32 {
        // Java: return this.shortStreamContainer.getBlockCount() * this.smallBlocksPerBigBlock;
        if let Some(ref ssc) = self.shortStreamContainer {
            ssc.getBlockCount() * self.smallBlocksPerBigBlock
        } else {
            0
        }
    }

    // Java: int getBlockSize()
    pub fn getBlockSize_impl(&self) -> i32 {
        // Java: return 64;
        64
    }

    // Java: boolean readBlock(int smallBlockIndex, BlockBuf buf)
    pub fn readBlock_impl(&mut self, smallBlockIndex: i32, buf: &mut BlockBuf) -> bool {
        // Java: int bigBlockIndex = smallBlockIndex / this.smallBlocksPerBigBlock;
        let bigBlockIndex = smallBlockIndex / self.smallBlocksPerBigBlock;
        // Java: synchronized(this)
        // Java: if (bigBlockIndex != this.currentBigBlockIndex)
        if bigBlockIndex != self.currentBigBlockIndex {
            // Java: if (this.currentBigBlock == null)
            if self.currentBigBlock.is_none() {
                // Java: this.currentBigBlock = BlockBuf.obtain(this.bigBlockSize);
                self.currentBigBlock = Some(BlockBuf::obtain(self.bigBlockSize));
            }
            // Java: this.currentBigBlockIndex = bigBlockIndex;
            self.currentBigBlockIndex = bigBlockIndex;
            // Java: this.shortStreamContainer.readBlock(this.currentBigBlockIndex, this.currentBigBlock);
            if let (Some(ref mut ssc), Some(cbb)) = (&mut self.shortStreamContainer, self.currentBigBlock.take()) {
                ssc.readBlock(self.currentBigBlockIndex, cbb);
                self.currentBigBlock = Some(BlockBuf::obtain(self.bigBlockSize));
            }
        }
        // Java: System.arraycopy(this.currentBigBlock.getBuf(), smallBlockIndex % this.smallBlocksPerBigBlock * 64, buf.getBuf(), 0, 64);
        if let Some(ref mut cbb) = self.currentBigBlock {
            let src_offset = (smallBlockIndex % self.smallBlocksPerBigBlock * 64) as usize;
            let src = cbb.getBuf().to_vec();
            buf.getBuf()[..64].copy_from_slice(&src[src_offset..src_offset + 64]);
        }
        // Java: return true;
        true
    }
}

impl IBlockList for RawSmallBlockList2 {
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
