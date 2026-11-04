// Java: import java.io.IOException
// Java: import java.io.OutputStream
// Java: import java.util.List
// Java: import org.apache.poi.poifs.common.POIFSBigBlockSize
use crate::org::apache::poi::poifs::common::POIFSBigBlockSize::POIFSBigBlockSize;
// Java: import org.apache.poi.poifs.property.Property
use crate::org::apache::poi::poifs::property::Property::Property;
use super::BigBlock::BigBlock;
use super::BlockWritable::BlockWritable;

// Java: public class PropertyBlock extends BigBlock
pub struct PropertyBlock {
    pub(crate) base: BigBlock,
    // Java field: Property[] _properties
    pub(crate) _properties: Vec<Property>,
}

// Java: public class PropertyBlock extends BigBlock
impl PropertyBlock {
    // Java: PropertyBlock(POIFSBigBlockSize bigBlockSize, Property[] properties, int offset)
    fn new(bigBlockSize: POIFSBigBlockSize, properties: &[Property], offset: i32) -> Self {
        // Java: super(bigBlockSize);
        let base = BigBlock::new(bigBlockSize);
        // Java: this._properties = new Property[bigBlockSize.getPropertiesPerBlock()];
        let ppb = base.bigBlockSize.getPropertiesPerBlock() as usize;
        let off = offset as usize;
        let mut _properties = Vec::with_capacity(ppb);
        // Java: for(int j = 0; j < this._properties.length; ++j)
        for j in 0..ppb {
            // Java: this._properties[j] = properties[j + offset];
            _properties.push(properties[j + off].clone());
        }
        PropertyBlock { base, _properties }
    }

    // Java: static BlockWritable[] createPropertyBlockArray(POIFSBigBlockSize bigBlockSize, List properties)
    pub fn createPropertyBlockArray(bigBlockSize: &POIFSBigBlockSize, properties: &[Property]) -> Vec<PropertyBlock> {
        // Java: int _properties_per_block = bigBlockSize.getPropertiesPerBlock();
        let _properties_per_block = bigBlockSize.getPropertiesPerBlock() as usize;
        // Java: int block_count = (properties.size() + _properties_per_block - 1) / _properties_per_block;
        let block_count = (properties.len() + _properties_per_block - 1) / _properties_per_block;
        // Java: Property[] to_be_written = new Property[block_count * _properties_per_block];
        let total = block_count * _properties_per_block;
        let mut to_be_written: Vec<Property> = Vec::with_capacity(total);
        // Java: System.arraycopy(properties.toArray(new Property[0]), 0, to_be_written, 0, properties.size());
        for p in properties {
            to_be_written.push(p.clone());
        }
        // Java: for(int j = properties.size(); j < to_be_written.length; ++j)
        for _j in properties.len()..total {
            // Java: to_be_written[j] = new Property()
            // Java: protected void preWrite()
            // Java: public boolean isDirectory()
            // Java: return false;
            to_be_written.push(Property::new());
        }
        // Java: BlockWritable[] rvalue = new BlockWritable[block_count];
        let mut rvalue: Vec<PropertyBlock> = Vec::with_capacity(block_count);
        // Java: for(int j = 0; j < block_count; ++j)
        for j in 0..block_count {
            // Java: rvalue[j] = new PropertyBlock(bigBlockSize, to_be_written, j * _properties_per_block);
            rvalue.push(PropertyBlock::new(bigBlockSize.clone(), &to_be_written, (j * _properties_per_block) as i32));
        }
        // Java: return rvalue;
        rvalue
    }

    // Java: void writeData(OutputStream stream)
    pub fn writeData(&self, stream: &mut dyn std::io::Write) {
        // Java: int _properties_per_block = this.bigBlockSize.getPropertiesPerBlock();
        let _properties_per_block = self.base.bigBlockSize.getPropertiesPerBlock();
        // Java: for(int j = 0; j < _properties_per_block; ++j)
        for j in 0.._properties_per_block as usize {
            // Java: this._properties[j].writeData(stream);
            let _ = self._properties[j].writeData(stream);
        }
    }
}

impl BlockWritable for PropertyBlock {
    fn writeBlocks(&self, var1: &mut dyn std::io::Write) -> std::io::Result<()> {
        self.writeData(var1);
        Ok(())
    }
}
