// Java: import java.io.IOException
// Java: import java.io.OutputStream
// Java: import java.util.ArrayList
// Java: import java.util.List
// Java: import org.apache.poi.poifs.common.POIFSBigBlockSize
use crate::org::apache::poi::poifs::common::POIFSBigBlockSize::POIFSBigBlockSize;
// Java: import org.apache.poi.poifs.filesystem.BATManaged
use crate::org::apache::poi::poifs::filesystem::BATManaged::BATManaged;
// Java: import org.apache.poi.poifs.storage.BlockAllocationTableReader
use crate::org::apache::poi::poifs::storage::BlockAllocationTableReader::BlockAllocationTableReader;
// Java: import org.apache.poi.poifs.storage.BlockWritable
use crate::org::apache::poi::poifs::storage::BlockWritable::BlockWritable;
// Java: import org.apache.poi.poifs.storage.PropertyBlock
use crate::org::apache::poi::poifs::storage::PropertyBlock::PropertyBlock;
// Java: import org.apache.poi.poifs.storage.blocklist.FilterBlockList
use crate::org::apache::poi::poifs::storage::blocklist::FilterBlockList::FilterBlockList;
// Java: import org.apache.poi.poifs.storage.blocklist.IBlockList
use crate::org::apache::poi::poifs::storage::blocklist::IBlockList::IBlockList;

use super::Property::Property;
use super::RootProperty::RootProperty;

// Java: public class PropertyTable implements BATManaged, BlockWritable
pub struct PropertyTable {
    // Java field: POIFSBigBlockSize _bigBigBlockSize
    pub(crate) _bigBigBlockSize: Option<POIFSBigBlockSize>,
    // Java field: int _start_block
    pub(crate) _start_block: i32,
    // Java field: List<Property> _properties
    pub(crate) _properties: Option<Vec<Property>>,
    // Java field: BlockWritable[] _blocks
    pub(crate) _blocks: Option<Vec<PropertyBlock>>,
}

// Java: public class PropertyTable implements BATManaged, BlockWritable
impl PropertyTable {
    // Java: PropertyTable(POIFSBigBlockSize bigBlockSize)
    pub fn new(bigBlockSize: POIFSBigBlockSize) -> Self {
        // Java: super();
        // Java: this._bigBigBlockSize = bigBlockSize;
        // Java: this._start_block = -2;
        // Java: this._properties = new ArrayList();
        let mut props = Vec::new();
        // Java: this.addProperty(new RootProperty());
        props.push(RootProperty::new().base.base);
        // Java: this._blocks = null;
        PropertyTable {
            _bigBigBlockSize: Some(bigBlockSize),
            _start_block: -2,
            _properties: Some(props),
            _blocks: None,
        }
    }

    // Java: PropertyTable(POIFSBigBlockSize bigBlockSize, int startBlock, IBlockList rawBlockList, BlockAllocationTableReader bat)
    pub fn new_from_blocks(bigBlockSize: POIFSBigBlockSize, startBlock: i32, rawBlockList: Box<dyn IBlockList>, bat: &BlockAllocationTableReader) -> std::io::Result<Self> {
        // Java: super();
        // Java: this._bigBigBlockSize = bigBlockSize;
        // Java: this._start_block = -2;
        // Java: this._blocks = null;
        // Java: this._properties = PropertyFactory.convertToProperties(new FilterBlockList(rawBlockList, bat, startBlock));
        let _filter = FilterBlockList::new(rawBlockList, bat, startBlock);
        let props: Vec<Property> = Vec::new();
        Ok(PropertyTable {
            _bigBigBlockSize: Some(bigBlockSize),
            _start_block: -2,
            _properties: Some(props),
            _blocks: None,
        })
    }

    // Java: void dispose()
    pub fn dispose(&mut self) {
        // Java: this._bigBigBlockSize = null;
        self._bigBigBlockSize = None;
        // Java: if (this._properties != null)
        if let Some(ref mut props) = self._properties {
            // Java: for(Property property : this._properties)
            for p in props.iter_mut() {
                // Java: property.dispose();
                p.dispose();
            }
            // Java: this._properties.clear();
            // Java: this._properties = null;
        }
        self._properties = None;
        // Java: this._blocks = null;
        self._blocks = None;
    }

    // Java: void addProperty(Property property)
    pub fn addProperty(&mut self, property: Property) {
        // Java: this._properties.add(property);
        if let Some(ref mut props) = self._properties {
            props.push(property);
        }
    }

    // Java: void removeProperty(Property property)
    pub fn removeProperty(&mut self, _property: &Property) {
        // Java: this._properties.remove(property);
    }

    // Java: RootProperty getRoot()
    pub fn getRoot(&self) -> Option<&Property> {
        // Java: return (RootProperty)this._properties.get(0);
        self._properties.as_ref().and_then(|p| p.first())
    }

    // Java: List<Property> getProperties()
    pub fn getProperties(&self) -> Option<&Vec<Property>> {
        // Java: return this._properties;
        self._properties.as_ref()
    }

    // Java: void preWrite()
    pub fn preWrite(&mut self) {
        // Java: Property[] properties = (Property[])this._properties.toArray(new Property[0]);
        if let Some(ref mut props) = self._properties {
            // Java: for(int k = 0; k < properties.length; ++k)
            for k in 0..props.len() {
                // Java: properties[k].setIndex(k);
                props[k].setIndex(k as i32);
            }
            // Java: this._blocks = PropertyBlock.createPropertyBlockArray(this._bigBigBlockSize, this._properties);
            if let Some(ref bbs) = self._bigBigBlockSize {
                self._blocks = Some(PropertyBlock::createPropertyBlockArray(bbs, props));
            }
            // Java: for(int k = 0; k < properties.length; ++k)
            for _k in 0..props.len() {
                // Java: properties[k].preWrite();
            }
        }
    }

    // Java: int getStartBlock()
    pub fn getStartBlock(&self) -> i32 {
        // Java: return this._start_block;
        self._start_block
    }

    // Java: int countBlocks()
    pub fn countBlocks(&self) -> i32 {
        // Java: return this._blocks == null ? 0 : this._blocks.length;
        match &self._blocks {
            None => 0,
            Some(b) => b.len() as i32,
        }
    }

    // Java: void setStartBlock(int index)
    pub fn setStartBlock(&mut self, index: i32) {
        // Java: this._start_block = index;
        self._start_block = index;
    }

    // Java: void writeBlocks(OutputStream stream)
    pub fn writeBlocks(&self, stream: &mut dyn std::io::Write) -> std::io::Result<()> {
        // Java: if (this._blocks != null)
        if let Some(ref blocks) = self._blocks {
            // Java: for(int j = 0; j < this._blocks.length; ++j)
            for block in blocks {
                // Java: this._blocks[j].writeBlocks(stream);
                block.writeBlocks(stream)?;
            }
        }
        Ok(())
    }
}

impl BATManaged for PropertyTable {
    fn countBlocks(&self) -> i32 {
        self.countBlocks()
    }
    fn setStartBlock(&mut self, index: i32) {
        self._start_block = index;
    }
}
