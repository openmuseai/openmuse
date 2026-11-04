// Java: import org.apache.poi.poifs.filesystem.POIFSDocument

use super::Property::Property;

// Java: public class DocumentProperty extends Property
pub struct DocumentProperty {
    pub(crate) base: Property,
    // Java field: POIFSDocument _document
    pub(crate) _document_set: bool,
}

// Java: public class DocumentProperty extends Property
impl DocumentProperty {
    // Java: DocumentProperty(String name, int size)
    pub fn new(name: String, size: i32) -> Self {
        // Java: super(name, (byte)2, (byte)1, 0, size);
        let base = Property::new_with_params(name, 2, 1, 0, size);
        DocumentProperty {
            base,
            _document_set: false,
        }
    }

    // Java: DocumentProperty(int index, byte[] array, int offset)
    pub fn new_from_array(index: i32, array: &[u8], offset: i32) -> Self {
        // Java: super(index, array, offset);
        let base = Property::new_from_array(index, array, offset);
        DocumentProperty {
            base,
            _document_set: false,
        }
    }

    // Java: void dispose()
    pub fn dispose(&mut self) {
        // Java: super.dispose();
        self.base.dispose();
        // Java: this._document = null;
        self._document_set = false;
    }

    // Java: void setDocument(POIFSDocument doc)
    pub fn setDocument(&mut self) {
        // Java: this._document = doc;
        self._document_set = true;
    }

    // Java: POIFSDocument getDocument()
    // Java: return this._document;

    // Java: boolean shouldUseSmallBlocks()
    pub fn shouldUseSmallBlocks(&self) -> bool {
        // Java: return super.shouldUseSmallBlocks();
        self.base.shouldUseSmallBlocks()
    }

    // Java: boolean isDirectory()
    pub fn isDirectory(&self) -> bool {
        // Java: return false;
        false
    }

    // Java: void preWrite()
    pub fn preWrite(&mut self) {
    }

    pub fn getName(&self) -> String {
        self.base.getName().to_string()
    }

    pub fn setStartBlock(&mut self, index: i32) {
        self.base.setStartBlock(index);
    }
}
