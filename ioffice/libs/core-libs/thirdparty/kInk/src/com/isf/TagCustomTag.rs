// Java: import com.isf.utils.BytesReader
use super::utils::BytesReader::BytesReader;
use super::TaggedStructure::TaggedStructure;

// Java: public class TagCustomTag extends TaggedStructure
pub struct TagCustomTag {
    pub(crate) base: TaggedStructure,
}

// Java: public class TagCustomTag extends TaggedStructure
impl TagCustomTag {
    // Java: TagCustomTag(BytesReader reader, int tagName, IsfReaderContext context)
    pub fn TagCustomTag(reader: &mut BytesReader, tagName: i32) -> Self {
        // Java: super(reader, tagName, context);
        let base = TaggedStructure::TaggedStructure(reader, tagName);
        // Java: for(int i = 0; (long)i < this.mSize + 1L; ++i)
        for _i in 0..(base.mSize + 1) {
            // Java: this.mReader.getByte();
            reader.getByte();
        }
        Self { base }
    }

    // Java: long size()
    pub fn size(&self) -> i64 {
        // Java: return super.size() + 1L;
        self.base.size() + 1
    }
}
