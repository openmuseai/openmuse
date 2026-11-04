// Java: import com.isf.utils.BytesReader
use super::utils::BytesReader::BytesReader;
use super::TaggedStructure::TaggedStructure;

// Java: public class TagGuidTable extends TaggedStructure
pub struct TagGuidTable {
    pub(crate) base: TaggedStructure,
}

// Java: public class TagGuidTable extends TaggedStructure
impl TagGuidTable {
    // Java: TagGuidTable(BytesReader reader, int tagName, IsfReaderContext context)
    pub fn TagGuidTable(reader: &mut BytesReader, tagName: i32) -> Self {
        // Java: super(reader, tagName, context);
        let base = TaggedStructure::TaggedStructure(reader, tagName);
        // Java: for(int i = 0; (long)i < this.mSize; ++i)
        for _i in 0..(base.mSize as i32) {
            // Java: this.mReader.getByte();
            reader.getByte();
        }
        Self { base }
    }
}
