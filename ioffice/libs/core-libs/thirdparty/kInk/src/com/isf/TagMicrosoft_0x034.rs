// Java: import com.isf.utils.BytesReader
use super::utils::BytesReader::BytesReader;
use super::TaggedStructure::TaggedStructure;

// Java: public class TagMicrosoft_0x034 extends TaggedStructure
pub struct TagMicrosoft_0x034 {
    pub(crate) base: TaggedStructure,
}

// Java: public class TagMicrosoft_0x034 extends TaggedStructure
impl TagMicrosoft_0x034 {
    // Java: TagMicrosoft_0x034(BytesReader reader, int tagName, IsfReaderContext context)
    pub fn TagMicrosoft_0x034(reader: &mut BytesReader, tagName: i32) -> Self {
        // Java: super(reader, tagName, context);
        let base = TaggedStructure::TaggedStructure(reader, tagName);
        // Java: for(int i = 0; i < 22; ++i)
        for _i in 0..22 {
            // Java: this.mReader.getByte();
            reader.getByte();
        }
        Self { base }
    }

    // Java: void unpackSize()
    pub fn unpackSize(&mut self, _reader: &mut BytesReader) {
    }

    // Java: long size()
    pub fn size(&self) -> i64 {
        // Java: return 23L;
        23
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
        // Java: return "TagMicrosoft_0x034";
        "TagMicrosoft_0x034".to_string()
    }
}
