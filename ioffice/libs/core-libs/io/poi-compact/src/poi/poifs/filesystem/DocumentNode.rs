// Java: import org.apache.poi.poifs.property.DocumentProperty
use io_poi_compact::org::apache::poi::poifs::property::DocumentProperty;

// Java: public class DocumentNode extends EntryNode implements DocumentEntry
pub struct DocumentNode {
    // Java field: POIFSDocument _document
    pub(crate) _document: Arc<Mutex<POIFSDocument>>,
}

// Java: public class DocumentNode extends EntryNode implements DocumentEntry
impl DocumentNode {
    // Java: void dispose()
    pub fn dispose(&self) {
      // Java: if (this._document != null)
        // TODO_RUST_CODE
      {
         // Java: this._document.dispose();
         // TODO_RUST_CODE
         // Java: this._document = null;
         // TODO_RUST_CODE
      }
    }

    // Java: POIFSDocument getDocument()
    pub fn getDocument(&self) -> POIFSDocument {
      // Java: return this._document;
      // TODO_RUST_CODE
    }

    // Java: int getSize()
    pub fn getSize(&self) -> i32 {
      // Java: return this.getProperty().getSize();
      // TODO_RUST_CODE
    }

    // Java: boolean isDocumentEntry()
    pub fn isDocumentEntry(&self) -> bool {
      // Java: return true;
      // TODO_RUST_CODE
    }

    // Java: boolean isDeleteOK()
    pub fn isDeleteOK(&self) -> bool {
      // Java: return true;
      // TODO_RUST_CODE
    }

}

impl DocumentEntry for DocumentNode {
}
