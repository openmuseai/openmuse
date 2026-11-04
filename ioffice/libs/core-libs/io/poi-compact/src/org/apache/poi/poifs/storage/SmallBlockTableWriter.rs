#![allow(non_snake_case, dead_code, unused_variables, non_upper_case_globals)]

// Java: import java.io.IOException
// Java: import java.io.OutputStream
// Java: import java.util.ArrayList
// Java: import java.util.Iterator
// Java: import java.util.List
// Java: import org.apache.poi.poifs.common.POIFSBigBlockSize
use crate::org::apache::poi::poifs::common::POIFSBigBlockSize::POIFSBigBlockSize;
// Java: import org.apache.poi.poifs.filesystem.BATManaged
use crate::org::apache::poi::poifs::filesystem::BATManaged::BATManaged;
// Java: import org.apache.poi.poifs.filesystem.POIFSDocument
use crate::org::apache::poi::poifs::filesystem::POIFSDocument::POIFSDocument;
// Java: import org.apache.poi.poifs.property.RootProperty
use crate::org::apache::poi::poifs::property::RootProperty::RootProperty;
use super::BlockAllocationTableWriter::BlockAllocationTableWriter;
use super::BlockWritable::BlockWritable;
use super::SmallDocumentBlock::SmallDocumentBlock;

use std::io::{self, Write};

// Java: public class SmallBlockTableWriter implements BlockWritable, BATManaged
pub struct SmallBlockTableWriter {
    // Java field: BlockAllocationTableWriter _sbat
    pub(crate) _sbat: BlockAllocationTableWriter,
    // Java field: List<BlockWritable> _small_blocks
    _small_blocks: Vec<SmallDocumentBlock>,
    // Java field: int _big_block_count
    pub(crate) _big_block_count: i32,
    // Java field: RootProperty _root
    _root: RootProperty,
}

// Java: public class SmallBlockTableWriter implements BlockWritable, BATManaged
impl SmallBlockTableWriter {
    // Java: SmallBlockTableWriter(POIFSBigBlockSize bigBlockSize, List<POIFSDocument> documents, RootProperty root)
    pub fn new(bigBlockSize: POIFSBigBlockSize, documents: &mut Vec<POIFSDocument>, mut root: RootProperty) -> Self {
        // Java: super();
        // Java: this._sbat = new BlockAllocationTableWriter(bigBlockSize);
        let mut _sbat = BlockAllocationTableWriter::new(POIFSBigBlockSize::new(bigBlockSize.bigBlockSize, bigBlockSize.headerValue));
        // Java: this._small_blocks = new ArrayList();
        let mut _small_blocks: Vec<SmallDocumentBlock> = Vec::new();
        // Java: this._root = root;
        // Java: for(POIFSDocument doc : documents)
        for doc in documents.iter_mut()
        {
            // Java: BlockWritable[] blocks = doc.getSmallBlocks();
            let blocks = doc.getSmallBlocks();
            // Java: if (blocks.length != 0)
            if !blocks.is_empty()
            {
                // Java: doc.setStartBlock(this._sbat.allocateSpace(blocks.length));
                let len = blocks.len() as i32;
                doc.setStartBlock_impl(_sbat.allocateSpace(len));
                // Java: for(int j = 0; j < blocks.length; ++j)
                for block in blocks
                {
                    // Java: this._small_blocks.add(blocks[j]);
                }
            }
            // Java: else
            else
            {
                // Java: doc.setStartBlock(-2);
                doc.setStartBlock_impl(-2);
            }
        }
        // Java: this._sbat.simpleCreateBlocks();
        _sbat.simpleCreateBlocks();
        // Java: this._root.setSize(this._small_blocks.size());
        root.setSize(_small_blocks.len() as i32);
        // Java: this._big_block_count = SmallDocumentBlock.fill(bigBlockSize, this._small_blocks);
        let _big_block_count = SmallDocumentBlock::fill(bigBlockSize, &mut _small_blocks);
        SmallBlockTableWriter { _sbat, _small_blocks, _big_block_count, _root: root }
    }

    // Java: int getSBATBlockCount()
    pub fn getSBATBlockCount(&self) -> i32 {
        // Java: return (this._big_block_count + 15) / 16;
        (self._big_block_count + 15) / 16
    }

    // Java: BlockAllocationTableWriter getSBAT()
    pub fn getSBAT(&self) -> &BlockAllocationTableWriter {
        // Java: return this._sbat;
        &self._sbat
    }

    // Java: int countBlocks()
    pub fn countBlocks_impl(&self) -> i32 {
        // Java: return this._big_block_count;
        self._big_block_count
    }

    // Java: void setStartBlock(int start_block)
    pub fn setStartBlock_impl(&mut self, start_block: i32) {
        // Java: this._root.setStartBlock(start_block);
        self._root.base.base.setStartBlock(start_block);
    }

    // Java: void writeBlocks(OutputStream stream)
    pub fn writeBlocks_impl(&self, stream: &mut dyn Write) -> io::Result<()> {
        // Java: Iterator iter = this._small_blocks.iterator();
        // Java: while(iter.hasNext())
        for block in &self._small_blocks
        {
            // Java: ((BlockWritable)iter.next()).writeBlocks(stream);
            block.writeBlocks(stream)?;
        }
        Ok(())
    }
}

impl BlockWritable for SmallBlockTableWriter {
    fn writeBlocks(&self, var1: &mut dyn Write) -> io::Result<()> {
        self.writeBlocks_impl(var1)
    }
}

impl BATManaged for SmallBlockTableWriter {
    fn countBlocks(&self) -> i32 {
        self.countBlocks_impl()
    }
    fn setStartBlock(&mut self, var1: i32) {
        self.setStartBlock_impl(var1);
    }
}
