// Java: import com.isf.utils.BytesReader
// Java: import com.isf.utils.Multibytecoding
use super::utils::BytesReader::BytesReader;
use super::utils::Multibytecoding::Multibytecoding;
use super::TaggedStructure::TaggedStructure;

// Java: public class TagInkSpaceRect extends TaggedStructure
pub struct TagInkSpaceRect {
    pub(crate) base: TaggedStructure,
}

// Java: public class TagInkSpaceRect extends TaggedStructure
impl TagInkSpaceRect {
    // Java: TagInkSpaceRect(BytesReader reader, int tagName, IsfReaderContext context)
    pub fn TagInkSpaceRect(reader: &mut BytesReader, tagName: i32) -> Self {
        // Java: super(reader, tagName, context);
        let mut base = TaggedStructure::TaggedStructure(reader, tagName);
        // Java: int start = this.mReader.pos();
        let start = reader.pos();
        // Java: Multibytecoding.decodeInt(reader);
        Multibytecoding::decodeInt(reader);
        // Java: Multibytecoding.decodeInt(reader);
        Multibytecoding::decodeInt(reader);
        // Java: Multibytecoding.decodeInt(reader);
        Multibytecoding::decodeInt(reader);
        // Java: Multibytecoding.decodeInt(reader);
        Multibytecoding::decodeInt(reader);
        // Java: this.mNumBytesStreamSize = (long)(this.mReader.pos() - start);
        base.mNumBytesStreamSize = (reader.pos() - start) as i64;
        Self { base }
    }

    // Java: void unpackSize()
    pub fn unpackSize(&mut self, _reader: &mut BytesReader) {
    }

    // Java: long size()
    pub fn size(&self) -> i64 {
        // Java: return 1L + this.mNumBytesStreamSize;
        1 + self.base.mNumBytesStreamSize
    }
}
