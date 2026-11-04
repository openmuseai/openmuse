// Java: import org.apache.poi.poifs.storage.SmallDocumentBlock
use crate::org::apache::poi::poifs::storage::SmallDocumentBlock::SmallDocumentBlock;
use super::DirectoryProperty::DirectoryProperty;

// Java: public class RootProperty extends DirectoryProperty
pub struct RootProperty {
    pub(crate) base: DirectoryProperty,
}

// Java: public class RootProperty extends DirectoryProperty
impl RootProperty {
    // Java: RootProperty()
    pub fn new() -> Self {
        // Java: super("Root Entry");
        let mut base = DirectoryProperty::new("Root Entry".to_string());
        // Java: this.setNodeColor((byte)1);
        base.base.setNodeColor(1);
        // Java: this.setPropertyType((byte)5);
        base.base.setPropertyType(5);
        // Java: this.setStartBlock(-2);
        base.base.setStartBlock(-2);
        RootProperty { base }
    }

    // Java: RootProperty(int index, byte[] array, int offset)
    pub fn new_from_array(index: i32, array: &[u8], offset: i32) -> Self {
        // Java: super(index, array, offset);
        let base = DirectoryProperty::new_from_array(index, array, offset);
        RootProperty { base }
    }

    // Java: void setSize(int size)
    pub fn setSize(&mut self, size: i32) {
        // Java: super.setSize(SmallDocumentBlock.calcSize(size));
        self.base.base.setSize(SmallDocumentBlock::calcSize(size));
    }
}
