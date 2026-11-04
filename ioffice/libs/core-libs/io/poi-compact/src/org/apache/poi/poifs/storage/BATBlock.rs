// Java: import java.io.IOException
// Java: import java.io.OutputStream
// Java: import java.util.Arrays
// Java: import org.apache.poi.poifs.common.POIFSBigBlockSize
use crate::org::apache::poi::poifs::common::POIFSBigBlockSize::POIFSBigBlockSize;
// Java: import org.apache.poi.util.IntegerField

use super::BigBlock::BigBlock;

// Java field: byte _default_value
pub(crate) const _DEFAULT_VALUE: u8 = 0xFF;

struct IntegerField {
    _offset: i32,
    _value: i32,
}

impl IntegerField {
    fn new(offset: i32) -> Self {
        IntegerField { _offset: offset, _value: 0 }
    }

    fn set_in_data(&mut self, value: i32, data: &mut [u8]) {
        self._value = value;
        let offset = self._offset as usize;
        let bytes = value.to_le_bytes();
        data[offset..offset + 4].copy_from_slice(&bytes);
    }
}

// Java: public class BATBlock extends BigBlock
pub struct BATBlock {
    pub(crate) base: BigBlock,
    // Java field: IntegerField[] _fields
    _fields: Vec<IntegerField>,
    // Java field: byte[] _data
    _data: Vec<u8>,
}

// Java: public class BATBlock extends BigBlock
impl BATBlock {
    // Java: BATBlock(POIFSBigBlockSize bigBlockSize)
    fn new(bigBlockSize: POIFSBigBlockSize) -> Self {
        // Java: super(bigBlockSize);
        let base = BigBlock::new(bigBlockSize);
        // Java: int _entries_per_block = bigBlockSize.getBATEntriesPerBlock();
        let _entries_per_block = base.bigBlockSize.getBATEntriesPerBlock();
        // Java: this._data = new byte[bigBlockSize.getBigBlockSize()];
        let mut _data = vec![0u8; base.bigBlockSize.getBigBlockSize() as usize];
        // Java: Arrays.fill(this._data, (byte)-1);
        _data.fill(_DEFAULT_VALUE);
        // Java: this._fields = new IntegerField[_entries_per_block];
        let mut _fields = Vec::with_capacity(_entries_per_block as usize);
        // Java: int offset = 0;
        let mut offset = 0i32;
        // Java: for(int j = 0; j < _entries_per_block; ++j)
        for _j in 0.._entries_per_block {
            // Java: this._fields[j] = new IntegerField(offset);
            _fields.push(IntegerField::new(offset));
            // Java: offset += 4;
            offset += 4;
        }
        BATBlock { base, _fields, _data }
    }

    // Java: static BATBlock[] createBATBlocks(POIFSBigBlockSize bigBlockSize, int[] entries)
    pub fn createBATBlocks(bigBlockSize: &POIFSBigBlockSize, entries: &[i32]) -> Vec<BATBlock> {
        // Java: int block_count = calculateStorageRequirements(bigBlockSize, entries.length);
        let block_count = Self::calculateStorageRequirements(bigBlockSize, entries.len() as i32);
        // Java: BATBlock[] blocks = new BATBlock[block_count];
        let mut blocks: Vec<BATBlock> = Vec::with_capacity(block_count as usize);
        // Java: int index = 0;
        // Java: int remaining = entries.length;
        let mut remaining = entries.len() as i32;
        // Java: int _entries_per_block = bigBlockSize.getBATEntriesPerBlock();
        let _entries_per_block = bigBlockSize.getBATEntriesPerBlock();
        // Java: for(int j = 0; j < entries.length; j += _entries_per_block)
        let mut j = 0i32;
        while j < entries.len() as i32 {
            // Java: blocks[index++] = new BATBlock(bigBlockSize, entries, j, remaining > _entries_per_block ? j + _entries_per_block : entries.length);
            let end = if remaining > _entries_per_block { j + _entries_per_block } else { entries.len() as i32 };
            let bs = POIFSBigBlockSize::new(bigBlockSize.getBigBlockSize(), bigBlockSize.getHeaderValue());
            blocks.push(Self::new_with_entries(bs, entries, j, end));
            // Java: remaining -= _entries_per_block;
            remaining -= _entries_per_block;
            j += _entries_per_block;
        }
        // Java: return blocks;
        blocks
    }

    // Java: static BATBlock[] createXBATBlocks(POIFSBigBlockSize bigBlockSize, int[] entries, int startBlock)
    pub fn createXBATBlocks(bigBlockSize: &POIFSBigBlockSize, entries: &[i32], startBlock: i32) -> Vec<BATBlock> {
        // Java: int block_count = calculateXBATStorageRequirements(bigBlockSize, entries.length);
        let block_count = Self::calculateXBATStorageRequirements(bigBlockSize, entries.len() as i32);
        // Java: BATBlock[] blocks = new BATBlock[block_count];
        let mut blocks: Vec<BATBlock> = Vec::with_capacity(block_count as usize);
        // Java: int index = 0;
        // Java: int remaining = entries.length;
        let mut remaining = entries.len() as i32;
        // Java: int _entries_per_xbat_block = bigBlockSize.getXBATEntriesPerBlock();
        let _entries_per_xbat_block = bigBlockSize.getXBATEntriesPerBlock();
        // Java: if (block_count != 0)
        if block_count != 0 {
            // Java: for(int j = 0; j < entries.length; j += _entries_per_xbat_block)
            let mut j = 0i32;
            while j < entries.len() as i32 {
                // Java: blocks[index++] = new BATBlock(bigBlockSize, entries, j, remaining > _entries_per_xbat_block ? j + _entries_per_xbat_block : entries.length);
                let end = if remaining > _entries_per_xbat_block { j + _entries_per_xbat_block } else { entries.len() as i32 };
                let bs = POIFSBigBlockSize::new(bigBlockSize.getBigBlockSize(), bigBlockSize.getHeaderValue());
                blocks.push(Self::new_with_entries(bs, entries, j, end));
                // Java: remaining -= _entries_per_xbat_block;
                remaining -= _entries_per_xbat_block;
                j += _entries_per_xbat_block;
            }
            // Java: for(index = 0; index < blocks.length - 1; ++index)
            for index in 0..blocks.len().saturating_sub(1) {
                // Java: blocks[index].setXBATChain(bigBlockSize, startBlock + index + 1);
                blocks[index].setXBATChain(bigBlockSize, startBlock + index as i32 + 1);
            }
            // Java: blocks[index].setXBATChain(bigBlockSize, -2);
            let last = blocks.len() - 1;
            blocks[last].setXBATChain(bigBlockSize, -2);
        }
        // Java: return blocks;
        blocks
    }

    // Java: static int calculateStorageRequirements(POIFSBigBlockSize bigBlockSize, int entryCount)
    pub fn calculateStorageRequirements(bigBlockSize: &POIFSBigBlockSize, entryCount: i32) -> i32 {
        // Java: int _entries_per_block = bigBlockSize.getBATEntriesPerBlock();
        let _entries_per_block = bigBlockSize.getBATEntriesPerBlock();
        // Java: return (entryCount + _entries_per_block - 1) / _entries_per_block;
        (entryCount + _entries_per_block - 1) / _entries_per_block
    }

    // Java: static int calculateXBATStorageRequirements(POIFSBigBlockSize bigBlockSize, int entryCount)
    pub fn calculateXBATStorageRequirements(bigBlockSize: &POIFSBigBlockSize, entryCount: i32) -> i32 {
        // Java: int _entries_per_xbat_block = bigBlockSize.getXBATEntriesPerBlock();
        let _entries_per_xbat_block = bigBlockSize.getXBATEntriesPerBlock();
        // Java: return (entryCount + _entries_per_xbat_block - 1) / _entries_per_xbat_block;
        (entryCount + _entries_per_xbat_block - 1) / _entries_per_xbat_block
    }

    // Java: void setXBATChain(POIFSBigBlockSize bigBlockSize, int chainIndex)
    fn setXBATChain(&mut self, bigBlockSize: &POIFSBigBlockSize, chainIndex: i32) {
        // Java: int _entries_per_xbat_block = bigBlockSize.getXBATEntriesPerBlock();
        let _entries_per_xbat_block = bigBlockSize.getXBATEntriesPerBlock() as usize;
        // Java: this._fields[_entries_per_xbat_block].set(chainIndex, this._data);
        self._fields[_entries_per_xbat_block].set_in_data(chainIndex, &mut self._data);
    }

    // Java: BATBlock(POIFSBigBlockSize bigBlockSize, int[] entries, int start_index, int end_index)
    fn new_with_entries(bigBlockSize: POIFSBigBlockSize, entries: &[i32], start_index: i32, end_index: i32) -> Self {
        // Java: this(bigBlockSize);
        let mut block = Self::new(bigBlockSize);
        // Java: for(int k = start_index; k < end_index; ++k)
        for k in start_index..end_index {
            // Java: this._fields[k - start_index].set(entries[k], this._data);
            let idx = (k - start_index) as usize;
            let value = entries[k as usize];
            block._fields[idx].set_in_data(value, &mut block._data);
        }
        block
    }

    // Java: void writeData(OutputStream stream)
    pub fn writeData(&self, stream: &mut dyn std::io::Write) {
        // Java: this.doWriteData(stream, this._data);
        BigBlock::doWriteData(stream, &self._data);
    }
}
