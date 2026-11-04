// Java: import com.isf.utils.BytesReader
use super::utils::BytesReader::BytesReader;
use super::TaggedStructure::TaggedStructure;

// Java: public class TagUnsupported extends TaggedStructure
pub struct TagUnsupported {
    pub(crate) base: TaggedStructure,
}

// Java: public class TagUnsupported extends TaggedStructure
impl TagUnsupported {
    // Java: TagUnsupported(BytesReader reader, int tagName, IsfReaderContext context)
    pub fn TagUnsupported(reader: &mut BytesReader, tagName: i32) -> Self {
        // Java: super(reader, tagName, context);
        let base = TaggedStructure::TaggedStructure(reader, tagName);
        // Java: for(int i = 0; (long)i < this.mSize; ++i)
        for _i in 0..(base.mSize as i32) {
            // Java: this.mReader.getByte();
            reader.getByte();
        }
        Self { base }
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
        // Java: return "TagUnsupported";
        "TagUnsupported".to_string()
    }
}
