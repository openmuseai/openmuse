// Java: import com.isf.utils.BytesReader
use super::utils::BytesReader::BytesReader;
use super::TaggedStructure::TaggedStructure;

// Java: public class TagStrokePropertyList extends TaggedStructure
pub struct TagStrokePropertyList {
    pub(crate) base: TaggedStructure,
}

// Java: public class TagStrokePropertyList extends TaggedStructure
impl TagStrokePropertyList {
    // Java: TagStrokePropertyList(BytesReader reader, int tagName, IsfReaderContext context)
    pub fn TagStrokePropertyList(reader: &mut BytesReader, tagName: i32) -> Self {
        // Java: super(reader, tagName, context);
        let base = TaggedStructure::TaggedStructure(reader, tagName);
        Self { base }
    }

    // Java: void unpackSize()
    pub fn unpackSize(&mut self, _reader: &mut BytesReader) {
    }

    // Java: long size()
    pub fn size(&self) -> i64 {
        // Java: return 1L;
        1
    }
}
