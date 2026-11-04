// Java: import com.isf.utils.BytesReader
use super::utils::BytesReader::BytesReader;
use super::TaggedStructure::TaggedStructure;

// Java: public class TagEmpty extends TaggedStructure
pub struct TagEmpty {
    pub(crate) base: TaggedStructure,
}

// Java: public class TagEmpty extends TaggedStructure
impl TagEmpty {
    // Java: TagEmpty(BytesReader reader, int tagName, IsfReaderContext context)
    pub fn TagEmpty(reader: &mut BytesReader, tagName: i32) -> Self {
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

    // Java: String toString()
    pub fn toString(&self) -> String {
        // Java: return "TagEmpty" + super.mTagName;
        format!("TagEmpty{}", self.base.mTagName)
    }
}
