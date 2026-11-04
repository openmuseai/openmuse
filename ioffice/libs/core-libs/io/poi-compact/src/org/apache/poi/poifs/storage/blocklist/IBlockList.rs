// Java: import org.apache.poi.poifs.common.BlockBuf
use crate::org::apache::poi::poifs::common::BlockBuf::BlockBuf;

// Java: public interface IBlockList
pub trait IBlockList {
    // Java: abstract BlockBuf getBlockAt(int var1)
    fn getBlockAt(&mut self, var1: i32) -> BlockBuf;

    // Java: abstract boolean readBlock(int var1, BlockBuf var2)
    fn readBlock(&mut self, var1: i32, var2: BlockBuf) -> bool;

    // Java: abstract int getBlockCount()
    fn getBlockCount(&self) -> i32;

    // Java: abstract int getBlockSize()
    fn getBlockSize(&self) -> i32;

    // Java: abstract void dispose()
    fn dispose(&mut self);
}
