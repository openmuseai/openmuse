// Java: import com.isf.utils.BytesReader
// Java: import com.isf.utils.Multibytecoding
use super::utils::BytesReader::BytesReader;
use super::utils::Multibytecoding::Multibytecoding;
use super::TaggedStructure::TaggedStructure;

// Java: public class TagTidx extends TaggedStructure
pub struct TagTidx {
    // Java field: long mIndex
    pub(crate) mIndex: i64,
    // Java field: long mNumBytesIndex
    mNumBytesIndex: i64,
    pub(crate) base: TaggedStructure,
}

// Java: public class TagTidx extends TaggedStructure
impl TagTidx {
    // Java: TagTidx(BytesReader reader, int tagName, IsfReaderContext context)
    pub fn TagTidx(reader: &mut BytesReader, tagName: i32) -> Self {
        // Java: super(reader, tagName, context);
        let base = TaggedStructure::TaggedStructure(reader, tagName);
        // Java: int start = this.mReader.pos();
        let start = reader.pos();
        // Java: this.mIndex = Multibytecoding.decodeUInt(this.mReader);
        let mIndex = Multibytecoding::decodeUInt(reader);
        // Java: this.mNumBytesIndex = (long)(this.mReader.pos() - start);
        let mNumBytesIndex = (reader.pos() - start) as i64;
        Self { mIndex, mNumBytesIndex, base }
    }

    // Java: void unpackSize()
    pub fn unpackSize(&mut self, _reader: &mut BytesReader) {
    }

    // Java: long size()
    pub fn size(&self) -> i64 {
        // Java: return 1L + this.mNumBytesIndex;
        1 + self.mNumBytesIndex
    }

    // Java: int type()
    pub fn r#type(&self) -> i32 {
        // Java: return 23;
        23
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
        // Java: StringBuilder sb = new StringBuilder();
        let mut sb = String::new();
        // Java: sb.append("TAG_TIDX\n");
        sb.push_str("TAG_TIDX\n");
        // Java: sb.append(this.mIndex);
        sb.push_str(&self.mIndex.to_string());
        // Java: return sb.toString();
        sb
    }
}
