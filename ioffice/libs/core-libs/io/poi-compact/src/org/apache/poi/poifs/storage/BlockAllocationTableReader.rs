// Java: import cn.wps.moffice.crash.FileDamagedException
// Java: import java.io.IOException
// Java: import java.nio.ByteBuffer
// Java: import java.nio.ByteOrder
// Java: import java.util.ArrayList
// Java: import java.util.List
// Java: import org.apache.poi.poifs.common.BlockBuf
use crate::org::apache::poi::poifs::common::BlockBuf::BlockBuf;
// Java: import org.apache.poi.poifs.common.POIFSBigBlockSize
use crate::org::apache::poi::poifs::common::POIFSBigBlockSize::POIFSBigBlockSize;
// Java: import org.apache.poi.poifs.storage.blocklist.IBlockList
use crate::org::apache::poi::poifs::storage::blocklist::IBlockList::IBlockList;
// Java: import org.apache.poi.util.IntList
// Java: import org.apache.poi.util.LittleEndian

fn le_get_int(data: &[u8], offset: usize) -> i32 {
    (data[offset] as i32) | ((data[offset + 1] as i32) << 8) | ((data[offset + 2] as i32) << 16) | ((data[offset + 3] as i32) << 24)
}

fn buf_to_int_vec(data: &[u8]) -> Vec<i32> {
    let count = data.len() / 4;
    let mut result = Vec::with_capacity(count);
    for i in 0..count {
        result.push(le_get_int(data, i * 4));
    }
    result
}

// Java field: int MAX_BLOCK_COUNT
pub(crate) const MAX_BLOCK_COUNT: i32 = 65535;

// Java: public class BlockAllocationTableReader
pub struct BlockAllocationTableReader {
    // Java field: IntList _entries
    pub(crate) _entries: Vec<i32>,
    // Java field: POIFSBigBlockSize bigBlockSize
    pub(crate) bigBlockSize: POIFSBigBlockSize,
}

// Java: public class BlockAllocationTableReader
impl BlockAllocationTableReader {
    // Java: BlockAllocationTableReader(POIFSBigBlockSize bigBlockSize, int block_count, int[] block_array, int xbat_count, int xbat_index, IBlockList raw_block_list)
    pub fn new(bigBlockSize: POIFSBigBlockSize, block_count: i32, block_array: &[i32], xbat_count: i32, xbat_index: i32, raw_block_list: &mut dyn IBlockList) -> std::io::Result<Self> {
        // Java: this(bigBlockSize);
        let mut result = Self::new_empty(bigBlockSize);
        // Java: if (block_count <= 0)
        if block_count <= 0 {
            // Java: throw new IOException("Illegal block count; minimum count is 1, got " + block_count + " instead");
            return Err(std::io::Error::new(std::io::ErrorKind::Other, format!("Illegal block count; minimum count is 1, got {} instead", block_count)));
        }
        // Java: else if (block_count > 65535)
        else if block_count > 65535 {
            // Java: throw new IOException("Block count " + block_count + " is too high. POI maximum is " + '\uffff' + ".");
            return Err(std::io::Error::new(std::io::ErrorKind::Other, format!("Block count {} is too high. POI maximum is 65535.", block_count)));
        }
        // Java: else
        else {
            // Java: int limit = Math.min(block_count, block_array.length);
            let limit = std::cmp::min(block_count, block_array.len() as i32);
            // Java: int block_index;
            let mut block_index: i32;
            // Java: for(block_index = 0; block_index < limit; ++block_index)
            block_index = 0;
            while block_index < limit {
                // Java: int nextOffset = block_array[block_index];
                let nextOffset = block_array[block_index as usize];
                // Java: if (nextOffset > raw_block_list.getBlockCount())
                if nextOffset > raw_block_list.getBlockCount() {
                    // Java: throw new FileDamagedException("Your file contains " + raw_block_list.getBlockCount() + " sectors, but the initial DIFAT array at index " + block_index + " referenced block # " + nextOffset + ". This isn't allowed and  your file is corrupt");
                    return Err(std::io::Error::new(std::io::ErrorKind::Other, format!("Your file contains {} sectors, but the initial DIFAT array at index {} referenced block # {}. This isn't allowed and  your file is corrupt", raw_block_list.getBlockCount(), block_index, nextOffset)));
                }
                // Java: BlockBuf blockBuf = raw_block_list.getBlockAt(nextOffset);
                let mut blockBuf = raw_block_list.getBlockAt(nextOffset);
                // Java: if (null != blockBuf)
                // Java: this.setEntries(blockBuf);
                result.setEntries_buf(&mut blockBuf);
                block_index += 1;
            }
            // Java: if (block_index < block_count)
            if block_index < block_count {
                // Java: if (xbat_index < 0)
                if xbat_index < 0 {
                    // Java: throw new IOException("BAT count exceeds limit, yet XBAT index indicates no valid entries");
                    return Err(std::io::Error::new(std::io::ErrorKind::Other, "BAT count exceeds limit, yet XBAT index indicates no valid entries"));
                }
                // Java: int chain_index = xbat_index;
                let mut chain_index = xbat_index;
                // Java: int max_entries_per_block = bigBlockSize.getXBATEntriesPerBlock();
                let max_entries_per_block = result.bigBlockSize.getXBATEntriesPerBlock();
                // Java: int chain_index_offset = bigBlockSize.getNextXBATChainOffset();
                let chain_index_offset = result.bigBlockSize.getNextXBATChainOffset() as usize;
                // Java: for(int j = 0; j < xbat_count; ++j)
                for _j in 0..xbat_count {
                    // Java: limit = Math.min(block_count - block_index, max_entries_per_block);
                    let limit = std::cmp::min(block_count - block_index, max_entries_per_block);
                    // Java: BlockBuf buf = raw_block_list.getBlockAt(chain_index);
                    let mut buf = raw_block_list.getBlockAt(chain_index);
                    // Java: if (null == buf)
                    // Java: throw new FileDamagedException("BlockBuf is null!");
                    // Java: byte[] byteBuf = buf.getBuf();
                    let byteBuf = buf.getBuf().to_vec();
                    // Java: int offset = 0;
                    let mut offset: usize = 0;
                    // Java: for(int k = 0; k < limit; ++k)
                    for _k in 0..limit {
                        // Java: ++block_index;
                        block_index += 1;
                        // Java: this.setEntries(raw_block_list.getBlockAt(LittleEndian.getInt(byteBuf, offset)));
                        let idx = le_get_int(&byteBuf, offset);
                        let mut blk = raw_block_list.getBlockAt(idx);
                        result.setEntries_buf(&mut blk);
                        // Java: offset += 4;
                        offset += 4;
                    }
                    // Java: chain_index = LittleEndian.getInt(byteBuf, chain_index_offset);
                    chain_index = le_get_int(&byteBuf, chain_index_offset);
                    // Java: buf.recycle();
                    buf.recycle();
                    // Java: if (chain_index == -2)
                    if chain_index == -2 {
                        // Java: break;
                        break;
                    }
                }
            }
            // Java: if (block_index != block_count)
            if block_index != block_count {
                // Java: throw new IOException("Could not find all blocks");
                return Err(std::io::Error::new(std::io::ErrorKind::Other, "Could not find all blocks"));
            }
        }
        Ok(result)
    }

    // Java: BlockAllocationTableReader(POIFSBigBlockSize bigBlockSize, IBlockList blocks)
    pub fn new_from_blocks(bigBlockSize: POIFSBigBlockSize, blocks: &mut dyn IBlockList) -> std::io::Result<Self> {
        // Java: this(bigBlockSize);
        let mut result = Self::new_empty(bigBlockSize);
        // Java: this.setEntries(blocks);
        result.setEntries_blocks(blocks);
        Ok(result)
    }

    fn new_empty(bigBlockSize: POIFSBigBlockSize) -> Self {
        // Java: super();
        // Java: this.bigBlockSize = bigBlockSize;
        // Java: this._entries = new IntList();
        BlockAllocationTableReader {
            _entries: Vec::new(),
            bigBlockSize,
        }
    }

    // Java: ListManagedBlock[] fetchBlocks(int startBlock, int headerPropertiesStartBlock, BlockList blockList)
    pub fn fetchBlocks(&self, _startBlock: i32, _headerPropertiesStartBlock: i32) -> Vec<i32> {
        // Java: List<ListManagedBlock> blocks = new ArrayList();
        let mut blocks = Vec::new();
        // Java: int currentBlock = startBlock;
        let mut currentBlock = _startBlock;
        // Java: boolean firstPass = true;
        let mut _firstPass = true;
        // Java: ListManagedBlock dataBlock = null;
        // Java: int block_count = blockList.blockCount();
        let block_count = self._entries.len() as i32;
        // Java: while(currentBlock != -2 && currentBlock >= 0 && currentBlock < block_count)
        while currentBlock != -2 && currentBlock >= 0 && currentBlock < block_count {
            // Java: try
            // Java: dataBlock = blockList.remove(currentBlock);
            // Java: blocks.add(dataBlock);
            blocks.push(currentBlock);
            // Java: currentBlock = this._entries.get(currentBlock);
            currentBlock = self._entries[currentBlock as usize];
            // Java: firstPass = false;
            _firstPass = false;
            // Java: catch (IOException e)
            // Java: if (currentBlock == headerPropertiesStartBlock)
            // Java: System.err.println("Warning, header block comes after data blocks in POIFS block listing");
            // Java: currentBlock = -2;
            // Java: else
            // Java: if (currentBlock != 0 || !firstPass)
            // Java: throw e;
            // Java: System.err.println("Warning, incorrectly terminated empty data blocks in POIFS block listing (should end at -2, ended at 0)");
            // Java: currentBlock = -2;
        }
        // Java: return (ListManagedBlock[])blocks.toArray(new ListManagedBlock[blocks.size()]);
        blocks
    }

    // Java: IntList fetchBlockList(int startBlock)
    pub fn fetchBlockList(&self, startBlock: i32) -> Vec<i32> {
        // Java: int capacity = this.estimateCapacity(startBlock);
        let capacity = self.estimateCapacity(startBlock);
        // Java: IntList ret = new IntList(capacity);
        let mut ret = Vec::with_capacity(capacity as usize);
        // Java: int current_block = startBlock;
        let mut current_block = startBlock;
        // Java: int entries_count = this._entries.size();
        let entries_count = self._entries.len() as i32;
        // Java: if (capacity <= entries_count)
        if capacity <= entries_count {
            // Java: while(current_block != -2 && current_block >= 0 && current_block < entries_count)
            while current_block != -2 && current_block >= 0 && current_block < entries_count {
                // Java: ret.add(current_block);
                ret.push(current_block);
                // Java: current_block = this._entries.get(current_block);
                current_block = self._entries[current_block as usize];
            }
        }
        // Java: else
        else {
            // Java: while(current_block != -2 && current_block >= 0 && current_block < entries_count)
            while current_block != -2 && current_block >= 0 && current_block < entries_count {
                // Java: ret.add(current_block);
                ret.push(current_block);
                // Java: current_block = this._entries.get(current_block);
                current_block = self._entries[current_block as usize];
                // Java: if (ret.contains(current_block))
                if ret.contains(&current_block) {
                    // Java: break;
                    break;
                }
            }
        }
        // Java: return ret;
        ret
    }

    // Java: int estimateCapacity(int startBlockIndex)
    fn estimateCapacity(&self, startBlockIndex: i32) -> i32 {
        // Java: int currentIndex = startBlockIndex;
        let mut currentIndex = startBlockIndex;
        // Java: int entryCount = this._entries.size();
        let entryCount = self._entries.len() as i32;
        // Java: int ret = 0;
        let mut ret = 0;
        // Java: while(currentIndex != -2 && currentIndex >= 0 && currentIndex < entryCount)
        while currentIndex != -2 && currentIndex >= 0 && currentIndex < entryCount {
            // Java: currentIndex = this._entries.get(currentIndex);
            currentIndex = self._entries[currentIndex as usize];
            // Java: ++ret;
            ret += 1;
            // Java: if (ret > entryCount)
            if ret > entryCount {
                // Java: break;
                break;
            }
        }
        // Java: return ret;
        ret
    }

    // Java: boolean isUsed(int index)
    pub fn isUsed(&self, index: i32) -> bool {
        // Java: try
        let idx = index as usize;
        if idx < self._entries.len() {
            // Java: return this._entries.get(index) != -1;
            self._entries[idx] != -1
        }
        // Java: catch (IndexOutOfBoundsException var3)
        else {
            // Java: return false;
            false
        }
    }

    // Java: int getNextBlockIndex(int index)
    pub fn getNextBlockIndex(&self, index: i32) -> std::io::Result<i32> {
        // Java: if (this.isUsed(index))
        if self.isUsed(index) {
            // Java: return this._entries.get(index);
            Ok(self._entries[index as usize])
        }
        // Java: else
        else {
            // Java: throw new IOException("index " + index + " is unused");
            Err(std::io::Error::new(std::io::ErrorKind::Other, format!("index {} is unused", index)))
        }
    }

    // Java: void setEntries(IBlockList blocks)
    fn setEntries_blocks(&mut self, blocks: &mut dyn IBlockList) {
        // Java: int block_index = 0;
        let mut block_index = 0;
        // Java: for(int count = blocks.getBlockCount(); block_index < count; ++block_index)
        let count = blocks.getBlockCount();
        while block_index < count {
            // Java: BlockBuf buf = blocks.getBlockAt(block_index);
            let mut buf = blocks.getBlockAt(block_index);
            // Java: if (null == buf)
            // Java: return;
            // Java: this._entries.addAll(ByteBuffer.wrap(buf.getBuf()).order(ByteOrder.LITTLE_ENDIAN).asIntBuffer());
            let ints = buf_to_int_vec(buf.getBuf());
            self._entries.extend_from_slice(&ints);
            // Java: buf.recycle();
            buf.recycle();
            block_index += 1;
        }
    }

    // Java: void setEntries(BlockBuf buf)
    fn setEntries_buf(&mut self, buf: &mut BlockBuf) {
        // Java: if (null != buf)
        // Java: this._entries.addAll(ByteBuffer.wrap(buf.getBuf()).order(ByteOrder.LITTLE_ENDIAN).asIntBuffer());
        let ints = buf_to_int_vec(buf.getBuf());
        self._entries.extend_from_slice(&ints);
        // Java: buf.recycle();
    }
}
