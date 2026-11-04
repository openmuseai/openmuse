// Java: import java.io.IOException
// Java: import java.io.OutputStream
// Java: import java.util.Arrays
// Java: import org.apache.poi.poifs.common.POIFSBigBlockSize
use crate::org::apache::poi::poifs::common::POIFSBigBlockSize::POIFSBigBlockSize;
// Java: import org.apache.poi.util.IntegerField
// Java: import org.apache.poi.util.LongField
// Java: import org.apache.poi.util.ShortField

use super::BigBlock::BigBlock;
use super::BATBlock::BATBlock;
use super::HeaderBlockConstants::HeaderBlockConstants;

// Java field: byte _default_value
pub(crate) const _DEFAULT_VALUE: u8 = 0xFF;

fn put_le_long(data: &mut [u8], offset: usize, value: i64) {
    let bytes = value.to_le_bytes();
    data[offset..offset + 8].copy_from_slice(&bytes);
}

fn put_le_int(data: &mut [u8], offset: usize, value: i32) {
    let bytes = value.to_le_bytes();
    data[offset..offset + 4].copy_from_slice(&bytes);
}

fn put_le_short(data: &mut [u8], offset: usize, value: i16) {
    let bytes = value.to_le_bytes();
    data[offset..offset + 2].copy_from_slice(&bytes);
}

struct FieldRef {
    _offset: usize,
}

impl FieldRef {
    fn new(offset: usize) -> Self {
        FieldRef { _offset: offset }
    }

    fn set(&self, value: i32, data: &mut [u8]) {
        put_le_int(data, self._offset, value);
    }
}

// Java: public class HeaderBlockWriter extends BigBlock implements HeaderBlockConstants
pub struct HeaderBlockWriter {
    pub(crate) base: BigBlock,
    // Java field: IntegerField _bat_count
    _bat_count: FieldRef,
    // Java field: IntegerField _property_start
    _property_start: FieldRef,
    // Java field: IntegerField _sbat_start
    _sbat_start: FieldRef,
    // Java field: IntegerField _sbat_block_count
    _sbat_block_count: FieldRef,
    // Java field: IntegerField _xbat_start
    _xbat_start: FieldRef,
    // Java field: IntegerField _xbat_count
    _xbat_count: FieldRef,
    // Java field: byte[] _data
    _data: Vec<u8>,
}

// Java: public class HeaderBlockWriter extends BigBlock implements HeaderBlockConstants
impl HeaderBlockWriter {
    // Java: HeaderBlockWriter(POIFSBigBlockSize bigBlockSize)
    pub fn new(bigBlockSize: POIFSBigBlockSize) -> Self {
        // Java: super(bigBlockSize);
        let base = BigBlock::new(bigBlockSize);
        // Java: this._data = new byte[bigBlockSize.getBigBlockSize()];
        let mut _data = vec![0u8; base.bigBlockSize.getBigBlockSize() as usize];
        // Java: Arrays.fill(this._data, (byte)-1);
        _data.fill(_DEFAULT_VALUE);
        // Java: new LongField(0, -2226271756974174256L, this._data);
        put_le_long(&mut _data, 0, -2226271756974174256i64);
        // Java: new IntegerField(8, 0, this._data);
        put_le_int(&mut _data, 8, 0);
        // Java: new IntegerField(12, 0, this._data);
        put_le_int(&mut _data, 12, 0);
        // Java: new IntegerField(16, 0, this._data);
        put_le_int(&mut _data, 16, 0);
        // Java: new IntegerField(20, 0, this._data);
        put_le_int(&mut _data, 20, 0);
        // Java: new ShortField(24, (short)59, this._data);
        put_le_short(&mut _data, 24, 59);
        // Java: new ShortField(26, (short)3, this._data);
        put_le_short(&mut _data, 26, 3);
        // Java: new ShortField(28, (short)-2, this._data);
        put_le_short(&mut _data, 28, -2);
        // Java: new ShortField(30, bigBlockSize.getHeaderValue(), this._data);
        put_le_short(&mut _data, 30, base.bigBlockSize.getHeaderValue());
        // Java: if (bigBlockSize.getBigBlockSize() == 4096)
        if base.bigBlockSize.getBigBlockSize() == 4096 {
            // Java: for(int i = 512; i < this._data.length; ++i)
            for i in 512.._data.len() {
                // Java: this._data[i] = 0;
                _data[i] = 0;
            }
        }
        // Java: new IntegerField(32, 6, this._data);
        put_le_int(&mut _data, 32, 6);
        // Java: new IntegerField(36, 0, this._data);
        put_le_int(&mut _data, 36, 0);
        // Java: new IntegerField(40, 0, this._data);
        put_le_int(&mut _data, 40, 0);
        // Java: this._bat_count = new IntegerField(44, 0, this._data);
        put_le_int(&mut _data, 44, 0);
        let _bat_count = FieldRef::new(44);
        // Java: this._property_start = new IntegerField(48, -2, this._data);
        put_le_int(&mut _data, 48, -2);
        let _property_start = FieldRef::new(48);
        // Java: new IntegerField(52, 0, this._data);
        put_le_int(&mut _data, 52, 0);
        // Java: new IntegerField(56, 4096, this._data);
        put_le_int(&mut _data, 56, 4096);
        // Java: this._sbat_start = new IntegerField(60, -2, this._data);
        put_le_int(&mut _data, 60, -2);
        let _sbat_start = FieldRef::new(60);
        // Java: this._sbat_block_count = new IntegerField(64, 0, this._data);
        put_le_int(&mut _data, 64, 0);
        let _sbat_block_count = FieldRef::new(64);
        // Java: this._xbat_start = new IntegerField(68, -2, this._data);
        put_le_int(&mut _data, 68, -2);
        let _xbat_start = FieldRef::new(68);
        // Java: this._xbat_count = new IntegerField(72, 0, this._data);
        put_le_int(&mut _data, 72, 0);
        let _xbat_count = FieldRef::new(72);
        HeaderBlockWriter {
            base,
            _bat_count,
            _property_start,
            _sbat_start,
            _sbat_block_count,
            _xbat_start,
            _xbat_count,
            _data,
        }
    }

    // Java: BATBlock[] setBATBlocks(int blockCount, int startBlock)
    pub fn setBATBlocks(&mut self, blockCount: i32, startBlock: i32) -> Vec<BATBlock> {
        // Java: this._bat_count.set(blockCount, this._data);
        self._bat_count.set(blockCount, &mut self._data);
        // Java: int limit = Math.min(blockCount, 109);
        let limit = blockCount.min(109);
        // Java: int offset = 76;
        let mut offset: usize = 76;
        // Java: for(int j = 0; j < limit; ++j)
        for j in 0..limit {
            // Java: new IntegerField(offset, startBlock + j, this._data);
            put_le_int(&mut self._data, offset, startBlock + j);
            // Java: offset += 4;
            offset += 4;
        }
        // Java: BATBlock[] rvalue;
        let rvalue;
        // Java: if (blockCount > 109)
        if blockCount > 109 {
            // Java: int excess_blocks = blockCount - 109;
            let excess_blocks = blockCount - 109;
            // Java: int[] excess_block_array = new int[excess_blocks];
            let mut excess_block_array = vec![0i32; excess_blocks as usize];
            // Java: for(int j = 0; j < excess_blocks; ++j)
            for j in 0..excess_blocks {
                // Java: excess_block_array[j] = startBlock + j + 109;
                excess_block_array[j as usize] = startBlock + j + 109;
            }
            // Java: rvalue = BATBlock.createXBATBlocks(this.bigBlockSize, excess_block_array, startBlock + blockCount);
            rvalue = BATBlock::createXBATBlocks(&self.base.bigBlockSize, &excess_block_array, startBlock + blockCount);
            // Java: this._xbat_start.set(startBlock + blockCount, this._data);
            self._xbat_start.set(startBlock + blockCount, &mut self._data);
        }
        // Java: else
        else {
            // Java: rvalue = BATBlock.createXBATBlocks(this.bigBlockSize, new int[0], 0);
            rvalue = BATBlock::createXBATBlocks(&self.base.bigBlockSize, &[], 0);
            // Java: this._xbat_start.set(-2, this._data);
            self._xbat_start.set(-2, &mut self._data);
        }
        // Java: this._xbat_count.set(rvalue.length, this._data);
        self._xbat_count.set(rvalue.len() as i32, &mut self._data);
        // Java: return rvalue;
        rvalue
    }

    // Java: void setPropertyStart(int startBlock)
    pub fn setPropertyStart(&mut self, startBlock: i32) {
        // Java: this._property_start.set(startBlock, this._data);
        self._property_start.set(startBlock, &mut self._data);
    }

    // Java: void setSBATStart(int startBlock)
    pub fn setSBATStart(&mut self, startBlock: i32) {
        // Java: this._sbat_start.set(startBlock, this._data);
        self._sbat_start.set(startBlock, &mut self._data);
    }

    // Java: void setSBATBlockCount(int count)
    pub fn setSBATBlockCount(&mut self, count: i32) {
        // Java: this._sbat_block_count.set(count, this._data);
        self._sbat_block_count.set(count, &mut self._data);
    }

    // Java: static int calculateXBATStorageRequirements(POIFSBigBlockSize bigBlockSize, int blockCount)
    pub fn calculateXBATStorageRequirements(bigBlockSize: &POIFSBigBlockSize, blockCount: i32) -> i32 {
        // Java: return blockCount > 109 ? BATBlock.calculateXBATStorageRequirements(bigBlockSize, blockCount - 109) : 0;
        if blockCount > 109 {
            BATBlock::calculateXBATStorageRequirements(bigBlockSize, blockCount - 109)
        } else {
            0
        }
    }

    // Java: void writeData(OutputStream stream)
    pub fn writeData(&self, stream: &mut dyn std::io::Write) {
        // Java: this.doWriteData(stream, this._data);
        BigBlock::doWriteData(stream, &self._data);
    }
}

impl HeaderBlockConstants for HeaderBlockWriter {}
