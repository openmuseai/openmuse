// Java: import cn.wps.base.log.IOLog
// Java: import cn.wps.core.runtime.Platform
// Java: import cn.wps.moffice.kfs.File
// Java: import cn.wps.moffice.kfs.FileOutputStream
// Java: import java.io.IOException
// Java: import java.io.InputStream
// Java: import java.io.OutputStream
// Java: import org.apache.poi.poifs.common.BlockBuf
// Java: import org.apache.poi.poifs.common.POIFSBigBlockSize
use crate::org::apache::poi::poifs::common::POIFSBigBlockSize::POIFSBigBlockSize;
// Java: import org.apache.poi.poifs.common.POIFSConstants
// Java: import org.apache.poi.poifs.property.DocumentProperty
use crate::org::apache::poi::poifs::property::DocumentProperty::DocumentProperty;
// Java: import org.apache.poi.poifs.storage.BlockWritable
use crate::org::apache::poi::poifs::storage::BlockWritable::BlockWritable;
// Java: import org.apache.poi.poifs.storage.DataInputBlock
use crate::org::apache::poi::poifs::storage::DataInputBlock::DataInputBlock;
// Java: import org.apache.poi.poifs.storage.SmallDocumentBlock
// Java: import org.apache.poi.poifs.storage.blocklist.ByteArrayBlockList
// Java: import org.apache.poi.poifs.storage.blocklist.IBlockList
use crate::org::apache::poi::poifs::storage::blocklist::IBlockList::IBlockList;
// Java: import org.apache.poi.poifs.storage.blocklist.SequencialBlockList

use super::BATManaged::BATManaged;

// Java field: String TAG
pub(crate) const TAG: &str = "POIFSDocument";
// Java field: SmallDocumentBlock[] EMPTY_SMALL_BLOCK_ARRAY

// Java: public class POIFSDocument implements BATManaged, BlockWritable
pub struct POIFSDocument {
    // Java field: DocumentProperty _property
    pub(crate) _property: Option<DocumentProperty>,
    // Java field: int _size
    pub(crate) _size: i32,
    // Java field: POIFSBigBlockSize _bigBigBlockSize
    pub(crate) _bigBigBlockSize: POIFSBigBlockSize,
    // Java field: IBlockList _blocks
    pub(crate) _blocks: Option<Box<dyn IBlockList>>,
}

// Java: public class POIFSDocument implements BATManaged, BlockWritable
impl POIFSDocument {
    // Java: POIFSDocument(String name, IBlockList blocks, int length, POIFSBigBlockSize bigBlockSize)
    pub fn new(name: String, blocks: Box<dyn IBlockList>, length: i32, bigBlockSize: POIFSBigBlockSize) -> Self {
        // Java: super();
        // Java: this._size = length;
        // Java: this._blocks = blocks;
        // Java: this._bigBigBlockSize = bigBlockSize;
        // Java: this._property = new DocumentProperty(name, this._size);
        let mut prop = DocumentProperty::new(name, length);
        // Java: this._property.setDocument(this);
        prop.setDocument();
        POIFSDocument {
            _property: Some(prop),
            _size: length,
            _bigBigBlockSize: bigBlockSize,
            _blocks: Some(blocks),
        }
    }

    // Java: POIFSDocument(DocumentProperty property, IBlockList blocks, int length, POIFSBigBlockSize bigBlockSize)
    pub fn new_from_property(mut property: DocumentProperty, blocks: Box<dyn IBlockList>, length: i32, bigBlockSize: POIFSBigBlockSize) -> Self {
        // Java: super();
        // Java: this._size = length;
        // Java: this._blocks = blocks;
        // Java: this._bigBigBlockSize = bigBlockSize;
        // Java: this._property = property;
        // Java: this._property.setDocument(this);
        property.setDocument();
        POIFSDocument {
            _property: Some(property),
            _size: length,
            _bigBigBlockSize: bigBlockSize,
            _blocks: Some(blocks),
        }
    }

    // Java: void dispose()
    pub fn dispose(&mut self) {
        // Java: if (this._property != null)
        if let Some(ref mut p) = self._property {
            // Java: this._property.dispose();
            p.dispose();
            // Java: this._property = null;
        }
        self._property = None;
        // Java: if (null != this._blocks)
        if let Some(ref mut b) = self._blocks {
            // Java: this._blocks.dispose();
            b.dispose();
            // Java: this._blocks = null;
        }
        self._blocks = None;
    }

    // Java: POIFSDocument(String name, POIFSBigBlockSize bigBlockSize, InputStream stream, boolean cacheStream)
    pub fn new_from_stream(name: String, bigBlockSize: POIFSBigBlockSize, stream: &mut dyn std::io::Read, size: i32, cacheStream: bool) -> std::io::Result<Self> {
        // Java: super();
        // Java: this._bigBigBlockSize = bigBlockSize;
        // Java: this._size = stream.available();
        let _size = size;
        // Java: this._property = new DocumentProperty(name, this._size);
        let mut prop = DocumentProperty::new(name, _size);
        // Java: int blockSize = bigBlockSize.getBigBlockSize();
        let mut blockSize = bigBlockSize.getBigBlockSize();
        // Java: if (this._property.shouldUseSmallBlocks())
        if prop.shouldUseSmallBlocks() {
            // Java: blockSize = 64;
            blockSize = 64;
        }
        // Java: if (cacheStream)
        // Java: this._blocks = new ByteArrayBlockList(stream, blockSize);
        // Java: else
        // Java: this._blocks = new SequencialBlockList(stream, blockSize);
        let _blocks: Option<Box<dyn IBlockList>> = None;
        // Java: this._property.setDocument(this);
        prop.setDocument();
        Ok(POIFSDocument {
            _property: Some(prop),
            _size,
            _bigBigBlockSize: bigBlockSize,
            _blocks,
        })
    }

    // Java: POIFSDocument(String name, InputStream stream, boolean cacheStream)
    // Java: this(name, POIFSConstants.SMALLER_BIG_BLOCK_SIZE_DETAILS, stream, cacheStream);

    // Java: BlockWritable[] getSmallBlocks()
    pub fn getSmallBlocks(&mut self) -> Vec<Box<dyn BlockWritable>> {
        // Java: if (!this._property.shouldUseSmallBlocks())
        if let Some(ref p) = self._property {
            if !p.shouldUseSmallBlocks() {
                // Java: return EMPTY_SMALL_BLOCK_ARRAY;
                return Vec::new();
            }
        }
        // Java: else
        // Java: int count = this._blocks.getBlockCount();
        // Java: BlockWritable[] ret = new BlockWritable[count];
        // Java: for(int i = 0; i < count; ++i)
        // Java: ret[i] = new SmallDocumentBlock(this._bigBigBlockSize, this._blocks.getBlockAt(i).getBuf());
        // Java: return ret;
        Vec::new()
    }

    // Java: int getSize()
    pub fn getSize(&self) -> i32 {
        // Java: return this._size;
        self._size
    }

    // Java: String getName()
    pub fn getName(&self) -> String {
        // Java: return this._property != null ? this._property.getName() : "_NONAME_";
        match &self._property {
            Some(p) => p.getName(),
            None => "_NONAME_".to_string(),
        }
    }

    // Java: void read(byte[] buffer, int offset)
    pub fn read(&mut self, buffer: &mut [u8], offset: i32) -> std::io::Result<()> {
        // Java: int len = buffer.length;
        let len = buffer.len() as i32;
        // Java: BlockBuf buf = BlockBuf.obtain(this.getInputBlockSize());
        // Java: DataInputBlock currentBlock = new DataInputBlock(buf, offset);
        // Java: try
        // Java: this.readBlock(offset, currentBlock);
        // Java: int blockAvailable = currentBlock.available();
        // Java: if (blockAvailable <= len)
        {
            // Java: int remaining = len;
            // Java: int writePos = 0;
            // Java: int currentOffset = offset;
            // Java: while(remaining > 0)
            {
                // Java: boolean blockIsExpiring = remaining >= blockAvailable;
                // Java: int reqSize = blockIsExpiring ? blockAvailable : remaining;
                // Java: currentBlock.readFully(buffer, writePos, reqSize);
                // Java: remaining -= reqSize;
                // Java: writePos += reqSize;
                // Java: currentOffset += reqSize;
                // Java: if (blockIsExpiring)
                {
                    // Java: if (currentOffset == this._size)
                    {
                        // Java: if (remaining > 0)
                        {
                            // Java: throw new IllegalStateException("reached end of document stream unexpectedly");
                        }
                        // Java: return;
                    }
                    // Java: this.readBlock(currentOffset, currentBlock);
                    // Java: blockAvailable = currentBlock.available();
                }
            }
            // Java: return;
        }
        // Java: currentBlock.readFully(buffer, 0, len);
        // Java: finally
        // Java: currentBlock.dispose();
        let _ = len;
        Ok(())
    }

    // Java: void readBlock(int offset, DataInputBlock block)
    pub fn readBlock(&mut self, offset: i32, block: &mut DataInputBlock) -> std::io::Result<()> {
        // Java: if (offset > this._size)
        if offset > self._size {
            // Java: throw new RuntimeException("Request for Offset " + offset + " doc size is " + this._size);
            return Err(std::io::Error::new(std::io::ErrorKind::Other, format!("Request for Offset {} doc size is {}", offset, self._size)));
        }
        // Java: else if (offset != this._size)
        else if offset != self._size {
            // Java: int block_size = this._blocks.getBlockSize();
            let block_size = self._blocks.as_ref().unwrap().getBlockSize();
            // Java: int block_index = offset / block_size;
            let block_index = offset / block_size;
            // Java: int block_offset = offset % block_size;
            let block_offset = offset % block_size;
            // Java: this._blocks.readBlock(block_index, block.getBlockBuf());
            if let Some(ref mut blocks) = self._blocks {
                if let Some(bb) = block._blockBuf.take() {
                    blocks.readBlock(block_index, bb);
                }
            }
            // Java: block.setPosition(block_offset);
            block.setPosition(block_offset);
        }
        Ok(())
    }

    // Java: int getInputBlockSize()
    pub fn getInputBlockSize(&self) -> i32 {
        // Java: return this._blocks.getBlockSize();
        self._blocks.as_ref().unwrap().getBlockSize()
    }

    // Java: DocumentProperty getDocumentProperty()
    pub fn getDocumentProperty(&self) -> Option<&DocumentProperty> {
        // Java: return this._property;
        self._property.as_ref()
    }

    // Java: void writeBlocks(OutputStream stream)
    pub fn writeBlocks_impl(&mut self, stream: &mut dyn std::io::Write) -> std::io::Result<()> {
        // Java: if (!this._property.shouldUseSmallBlocks())
        if let Some(ref p) = self._property {
            if !p.shouldUseSmallBlocks() {
                // Java: int i = 0;
                let mut i = 0;
                // Java: for(int count = this._blocks.getBlockCount(); i < count; ++i)
                let count = self._blocks.as_ref().unwrap().getBlockCount();
                while i < count {
                    // Java: BlockBuf buf = this._blocks.getBlockAt(i);
                    let mut buf = self._blocks.as_mut().unwrap().getBlockAt(i);
                    // Java: stream.write(buf.getBuf());
                    stream.write_all(buf.getBuf())?;
                    // Java: buf.recycle();
                    buf.recycle();
                    i += 1;
                }
            }
        }
        Ok(())
    }

    // Java: int countBlocks()
    pub fn countBlocks_impl(&self) -> i32 {
        // Java: return this._property.shouldUseSmallBlocks() ? 0 : this._blocks.getBlockCount();
        if let Some(ref p) = self._property {
            if p.shouldUseSmallBlocks() {
                return 0;
            }
        }
        self._blocks.as_ref().map_or(0, |b| b.getBlockCount())
    }

    // Java: void setStartBlock(int index)
    pub fn setStartBlock_impl(&mut self, index: i32) {
        // Java: this._property.setStartBlock(index);
        if let Some(ref mut p) = self._property {
            p.setStartBlock(index);
        }
    }

    // Java: String serialize()
    pub fn serialize(&self) -> Option<String> {
        // Java: File file = Platform.createTempFile(this.getName(), "");
        // Java: FileOutputStream fos = null;
        // Java: DocumentInputStream is = null;
        // Java: try
        // Java: fos = new FileOutputStream(file);
        // Java: is = new DocumentInputStream(this);
        // Java: byte[] buff = new byte[4096];
        // Java: int n = 0;
        // Java: while(true)
        // Java: n = is.read(buff);
        // Java: if (-1 == n)
        // Java: return file.getAbsolutePath();
        // Java: fos.write(buff, 0, n);
        // Java: catch (Exception e)
        // Java: IOLog.e(TAG, "Exception", e);
        // Java: finally
        // Java: if (null != is)
        // Java: is.close();
        // Java: if (null != fos)
        // Java: try
        // Java: fos.close();
        // Java: catch (IOException e1)
        // Java: IOLog.e(TAG, "IOException", e1);
        // Java: return null;
        None
    }
}

impl BATManaged for POIFSDocument {
    fn countBlocks(&self) -> i32 {
        self.countBlocks_impl()
    }
    fn setStartBlock(&mut self, index: i32) {
        self.setStartBlock_impl(index);
    }
}

impl BlockWritable for POIFSDocument {
    fn writeBlocks(&self, _stream: &mut dyn std::io::Write) -> std::io::Result<()> {
        Ok(())
    }
}
