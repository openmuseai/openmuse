// Java: import java.io.IOException
// Java: import java.io.OutputStream
// Java: import org.apache.poi.poifs.common.POIFSBigBlockSize
use crate::org::apache::poi::poifs::common::POIFSBigBlockSize::POIFSBigBlockSize;
// Java: import org.apache.poi.poifs.filesystem.BATManaged
use crate::org::apache::poi::poifs::filesystem::BATManaged::BATManaged;
// Java: import org.apache.poi.util.IntList
use super::BATBlock::BATBlock;
use super::BlockWritable::BlockWritable;
use super::HeaderBlockWriter::HeaderBlockWriter;

// Java: public class BlockAllocationTableWriter implements BlockWritable, BATManaged
pub struct BlockAllocationTableWriter {
    // Java field: IntList _entries
    pub(crate) _entries: Vec<i32>,
    // Java field: BATBlock[] _blocks
    pub(crate) _blocks: Vec<BATBlock>,
    // Java field: int _start_block
    pub(crate) _start_block: i32,
    // Java field: POIFSBigBlockSize _bigBlockSize
    pub(crate) _bigBlockSize: POIFSBigBlockSize,
}

// Java: public class BlockAllocationTableWriter implements BlockWritable, BATManaged
impl BlockAllocationTableWriter {
    // Java: BlockAllocationTableWriter(POIFSBigBlockSize bigBlockSize)
    pub fn new(bigBlockSize: POIFSBigBlockSize) -> Self {
        // Java: super();
        // Java: this._bigBlockSize = bigBlockSize;
        // Java: this._start_block = -2;
        // Java: this._entries = new IntList();
        // Java: this._blocks = new BATBlock[0];
        BlockAllocationTableWriter {
            _entries: Vec::new(),
            _blocks: Vec::new(),
            _start_block: -2,
            _bigBlockSize: bigBlockSize,
        }
    }

    // Java: int createBlocks()
    pub fn createBlocks(&mut self) -> i32 {
        // Java: int xbat_blocks = 0;
        let mut xbat_blocks = 0;
        // Java: int bat_blocks = 0;
        let mut bat_blocks = 0;
        // Java: while(true)
        loop {
            // Java: int calculated_bat_blocks = BATBlock.calculateStorageRequirements(this._bigBlockSize, bat_blocks + xbat_blocks + this._entries.size());
            let calculated_bat_blocks = BATBlock::calculateStorageRequirements(&self._bigBlockSize, bat_blocks + xbat_blocks + self._entries.len() as i32);
            // Java: int calculated_xbat_blocks = HeaderBlockWriter.calculateXBATStorageRequirements(this._bigBlockSize, calculated_bat_blocks);
            let calculated_xbat_blocks = HeaderBlockWriter::calculateXBATStorageRequirements(&self._bigBlockSize, calculated_bat_blocks);
            // Java: if (bat_blocks == calculated_bat_blocks && xbat_blocks == calculated_xbat_blocks)
            if bat_blocks == calculated_bat_blocks && xbat_blocks == calculated_xbat_blocks {
                // Java: calculated_bat_blocks = this.allocateSpace(bat_blocks);
                let calculated_bat_blocks = self.allocateSpace(bat_blocks);
                // Java: this.allocateSpace(xbat_blocks);
                self.allocateSpace(xbat_blocks);
                // Java: this.simpleCreateBlocks();
                self.simpleCreateBlocks();
                // Java: return calculated_bat_blocks;
                return calculated_bat_blocks;
            }
            // Java: bat_blocks = calculated_bat_blocks;
            bat_blocks = calculated_bat_blocks;
            // Java: xbat_blocks = calculated_xbat_blocks;
            xbat_blocks = calculated_xbat_blocks;
        }
    }

    // Java: int allocateSpace(int blockCount)
    pub fn allocateSpace(&mut self, blockCount: i32) -> i32 {
        // Java: int startBlock = this._entries.size();
        let startBlock = self._entries.len() as i32;
        // Java: if (blockCount > 0)
        if blockCount > 0 {
            // Java: int limit = blockCount - 1;
            let limit = blockCount - 1;
            // Java: int index = startBlock + 1;
            let mut index = startBlock + 1;
            // Java: for(int k = 0; k < limit; ++k)
            for _k in 0..limit {
                // Java: this._entries.add(index++);
                self._entries.push(index);
                index += 1;
            }
            // Java: this._entries.add(-2);
            self._entries.push(-2);
        }
        // Java: return startBlock;
        startBlock
    }

    // Java: int getStartBlock()
    pub fn getStartBlock(&self) -> i32 {
        // Java: return this._start_block;
        self._start_block
    }

    // Java: void simpleCreateBlocks()
    pub fn simpleCreateBlocks(&mut self) {
        // Java: this._blocks = BATBlock.createBATBlocks(this._bigBlockSize, this._entries.toArray());
        self._blocks = BATBlock::createBATBlocks(&self._bigBlockSize, &self._entries);
    }

    // Java: void writeBlocks(OutputStream stream)
    pub fn writeBlocks_impl(&self, stream: &mut dyn std::io::Write) {
        // Java: for(int j = 0; j < this._blocks.length; ++j)
        for j in 0..self._blocks.len() {
            // Java: this._blocks[j].writeBlocks(stream);
            self._blocks[j].writeData(stream);
        }
    }

    // Java: int countBlocks()
    pub fn countBlocks_impl(&self) -> i32 {
        // Java: return this._blocks.length;
        self._blocks.len() as i32
    }

    // Java: void setStartBlock(int start_block)
    pub fn setStartBlock_impl(&mut self, start_block: i32) {
        // Java: this._start_block = start_block;
        self._start_block = start_block;
    }
}

impl BlockWritable for BlockAllocationTableWriter {
    fn writeBlocks(&self, var1: &mut dyn std::io::Write) -> std::io::Result<()> {
        self.writeBlocks_impl(var1);
        Ok(())
    }
}

impl BATManaged for BlockAllocationTableWriter {
    fn countBlocks(&self) -> i32 {
        self.countBlocks_impl()
    }
    fn setStartBlock(&mut self, var1: i32) {
        self.setStartBlock_impl(var1);
    }
}
