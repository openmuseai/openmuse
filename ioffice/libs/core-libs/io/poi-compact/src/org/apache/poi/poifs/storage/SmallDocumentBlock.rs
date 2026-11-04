// Java: import org.apache.poi.poifs.common.POIFSBigBlockSize
use crate::org::apache::poi::poifs::common::POIFSBigBlockSize::POIFSBigBlockSize;
use super::BlockWritable::BlockWritable;
use super::ListManagedBlock::ListManagedBlock;
use std::io::{self, Write};

const BLOCK_SHIFT: i32 = 6;
const _default_fill: u8 = 0xFFu8;
pub const _block_size: i32 = 64;
const BLOCK_MASK: i32 = 63;

// Java: public class SmallDocumentBlock implements BlockWritable, ListManagedBlock
pub struct SmallDocumentBlock {
    // Java field: int BLOCK_SHIFT
    // Java field: byte[] _data
    pub(crate) _data: Vec<u8>,
    // Java field: byte _default_fill
    // Java field: int _block_size
    // Java field: int BLOCK_MASK
    // Java field: int _blocks_per_big_block
    _blocks_per_big_block: i32,
    // Java field: POIFSBigBlockSize _bigBlockSize
    pub(crate) _bigBlockSize: POIFSBigBlockSize,
}

// Java: public class SmallDocumentBlock implements BlockWritable, ListManagedBlock
impl SmallDocumentBlock {
    // Java: SmallDocumentBlock(POIFSBigBlockSize bigBlockSize, byte[] data, int index)
    fn new_with_index(bigBlockSize: POIFSBigBlockSize, data: &[u8], index: i32) -> Self {
        // Java: this(bigBlockSize);
        let mut block = SmallDocumentBlock::new_empty(bigBlockSize);
        // Java: System.arraycopy(data, index * 64, this._data, 0, 64);
        let src_offset = (index * 64) as usize;
        block._data[..64].copy_from_slice(&data[src_offset..src_offset + 64]);
        block
    }

    // Java: SmallDocumentBlock(POIFSBigBlockSize bigBlockSize)
    fn new_empty(bigBlockSize: POIFSBigBlockSize) -> Self {
        // Java: super();
        // Java: this._bigBlockSize = bigBlockSize;
        // Java: this._blocks_per_big_block = getBlocksPerBigBlock(bigBlockSize);
        let _blocks_per_big_block = SmallDocumentBlock::getBlocksPerBigBlock(&bigBlockSize);
        // Java: this._data = new byte[64];
        SmallDocumentBlock {
            _data: vec![0u8; 64],
            _blocks_per_big_block,
            _bigBlockSize: bigBlockSize,
        }
    }

    // Java: SmallDocumentBlock(POIFSBigBlockSize bigBlockSize, byte[] data)
    pub fn new_with_data(bigBlockSize: POIFSBigBlockSize, data: Vec<u8>) -> Self {
        // Java: super();
        // Java: assert data.length == 64;
        assert!(data.len() == 64);
        // Java: this._bigBlockSize = bigBlockSize;
        // Java: this._blocks_per_big_block = getBlocksPerBigBlock(this._bigBlockSize);
        let _blocks_per_big_block = SmallDocumentBlock::getBlocksPerBigBlock(&bigBlockSize);
        // Java: this._data = data;
        SmallDocumentBlock {
            _data: data,
            _blocks_per_big_block,
            _bigBlockSize: bigBlockSize,
        }
    }

    // Java: static int getBlocksPerBigBlock(POIFSBigBlockSize bigBlockSize)
    fn getBlocksPerBigBlock(bigBlockSize: &POIFSBigBlockSize) -> i32 {
        // Java: return bigBlockSize.getBigBlockSize() / 64;
        bigBlockSize.getBigBlockSize() / 64
    }

    // Java: static SmallDocumentBlock[] convert(POIFSBigBlockSize bigBlockSize, byte[] array, int size)
    pub fn convert_from_bytes(bigBlockSize: POIFSBigBlockSize, array: &[u8], size: i32) -> Vec<SmallDocumentBlock> {
        // Java: SmallDocumentBlock[] rval = new SmallDocumentBlock[(size + 64 - 1) / 64];
        let count = ((size + 64 - 1) / 64) as usize;
        let mut rval = Vec::with_capacity(count);
        // Java: int offset = 0;
        let mut offset: usize = 0;
        // Java: for(int k = 0; k < rval.length; ++k)
        for _k in 0..count {
            // Java: rval[k] = new SmallDocumentBlock(bigBlockSize);
            let mut block = SmallDocumentBlock::new_empty(bigBlockSize.clone());
            // Java: if (offset < array.length)
            if offset < array.len() {
                // Java: int length = Math.min(64, array.length - offset);
                let length = std::cmp::min(64, array.len() - offset);
                // Java: System.arraycopy(array, offset, rval[k]._data, 0, length);
                block._data[..length].copy_from_slice(&array[offset..offset + length]);
                // Java: if (length != 64)
                if length != 64 {
                    // Java: Arrays.fill(rval[k]._data, length, 64, (byte)-1);
                    for i in length..64 {
                        block._data[i] = _default_fill;
                    }
                }
            }
            // Java: else
            else {
                // Java: Arrays.fill(rval[k]._data, (byte)-1);
                for i in 0..64 {
                    block._data[i] = _default_fill;
                }
            }
            // Java: offset += 64;
            offset += 64;
            rval.push(block);
        }
        // Java: return rval;
        rval
    }

    // Java: static int fill(POIFSBigBlockSize bigBlockSize, List blocks)
    pub fn fill(bigBlockSize: POIFSBigBlockSize, blocks: &mut Vec<SmallDocumentBlock>) -> i32 {
        // Java: int _blocks_per_big_block = getBlocksPerBigBlock(bigBlockSize);
        let _blocks_per_big_block = SmallDocumentBlock::getBlocksPerBigBlock(&bigBlockSize);
        // Java: int count = blocks.size();
        let mut count = blocks.len() as i32;
        // Java: int big_block_count = (count + _blocks_per_big_block - 1) / _blocks_per_big_block;
        let big_block_count = (count + _blocks_per_big_block - 1) / _blocks_per_big_block;
        // Java: for(int full_count = big_block_count * _blocks_per_big_block; count < full_count; ++count)
        let full_count = big_block_count * _blocks_per_big_block;
        while count < full_count {
            // Java: blocks.add(makeEmptySmallDocumentBlock(bigBlockSize));
            blocks.push(SmallDocumentBlock::makeEmptySmallDocumentBlock(bigBlockSize.clone()));
            count += 1;
        }
        // Java: return big_block_count;
        big_block_count
    }

    // Java: static SmallDocumentBlock[] convert(POIFSBigBlockSize bigBlockSize, BlockWritable[] store, int size)
    pub fn convert_from_store(bigBlockSize: POIFSBigBlockSize, store: &[Box<dyn BlockWritable>], size: i32) -> Vec<SmallDocumentBlock> {
        // Java: ByteArrayOutputStream stream = new ByteArrayOutputStream();
        let mut stream: Vec<u8> = Vec::new();
        // Java: for(int j = 0; j < store.length; ++j)
        for j in 0..store.len() {
            // Java: store[j].writeBlocks(stream);
            store[j].writeBlocks(&mut stream).unwrap();
        }
        // Java: byte[] data = stream.toByteArray();
        let data = stream;
        // Java: SmallDocumentBlock[] rval = new SmallDocumentBlock[convertToBlockCount(size)];
        let block_count = SmallDocumentBlock::convertToBlockCount(size) as usize;
        let mut rval = Vec::with_capacity(block_count);
        // Java: for(int index = 0; index < rval.length; ++index)
        for index in 0..block_count {
            // Java: rval[index] = new SmallDocumentBlock(bigBlockSize, data, index);
            rval.push(SmallDocumentBlock::new_with_index(bigBlockSize.clone(), &data, index as i32));
        }
        // Java: return rval;
        rval
    }

    // Java: static List extract(POIFSBigBlockSize bigBlockSize, ListManagedBlock[] blocks)
    pub fn extract(bigBlockSize: POIFSBigBlockSize, blocks: &[Box<dyn ListManagedBlock>]) -> Vec<SmallDocumentBlock> {
        // Java: int _blocks_per_big_block = getBlocksPerBigBlock(bigBlockSize);
        let _blocks_per_big_block = SmallDocumentBlock::getBlocksPerBigBlock(&bigBlockSize);
        // Java: List sdbs = new ArrayList();
        let mut sdbs = Vec::new();
        // Java: for(int j = 0; j < blocks.length; ++j)
        for j in 0..blocks.len() {
            // Java: byte[] data = blocks[j].getData();
            let data = blocks[j].getData().unwrap();
            // Java: for(int k = 0; k < _blocks_per_big_block; ++k)
            for k in 0.._blocks_per_big_block {
                // Java: sdbs.add(new SmallDocumentBlock(bigBlockSize, data, k));
                sdbs.push(SmallDocumentBlock::new_with_index(bigBlockSize.clone(), &data, k));
            }
        }
        // Java: return sdbs;
        sdbs
    }

    // Java: static int calcSize(int size)
    pub fn calcSize(size: i32) -> i32 {
        // Java: return size * 64;
        size * 64
    }

    // Java: static SmallDocumentBlock makeEmptySmallDocumentBlock(POIFSBigBlockSize bigBlockSize)
    fn makeEmptySmallDocumentBlock(bigBlockSize: POIFSBigBlockSize) -> SmallDocumentBlock {
        // Java: SmallDocumentBlock block = new SmallDocumentBlock(bigBlockSize);
        let mut block = SmallDocumentBlock::new_empty(bigBlockSize);
        // Java: Arrays.fill(block._data, (byte)-1);
        for i in 0..64 {
            block._data[i] = _default_fill;
        }
        // Java: return block;
        block
    }

    // Java: static int convertToBlockCount(int size)
    fn convertToBlockCount(size: i32) -> i32 {
        // Java: return (size + 64 - 1) / 64;
        (size + 64 - 1) / 64
    }

    // Java: void writeBlocks(OutputStream stream)
    pub fn writeBlocks_impl(&self, stream: &mut dyn Write) -> io::Result<()> {
        // Java: stream.write(this._data);
        stream.write_all(&self._data)
    }

    // Java: byte[] getData()
    pub fn getData_impl(&self) -> Vec<u8> {
        // Java: return this._data;
        self._data.clone()
    }

    // Java: POIFSBigBlockSize getBigBlockSize()
    pub fn getBigBlockSize(&self) -> &POIFSBigBlockSize {
        // Java: return this._bigBlockSize;
        &self._bigBlockSize
    }
}

impl Clone for POIFSBigBlockSize {
    fn clone(&self) -> Self {
        POIFSBigBlockSize::new(self.bigBlockSize, self.headerValue)
    }
}

impl BlockWritable for SmallDocumentBlock {
    fn writeBlocks(&self, var1: &mut dyn Write) -> io::Result<()> {
        self.writeBlocks_impl(var1)
    }
}

impl ListManagedBlock for SmallDocumentBlock {
    fn getData(&self) -> io::Result<Vec<u8>> {
        Ok(self.getData_impl())
    }
}
