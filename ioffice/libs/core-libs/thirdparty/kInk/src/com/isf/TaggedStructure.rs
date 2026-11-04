// Java: import cn.wps.base.assertion.KSAssert
// Java: import com.isf.utils.BytesReader
// Java: import com.isf.utils.Multibytecoding
use super::utils::BytesReader::BytesReader;
use super::utils::Multibytecoding::Multibytecoding;
use super::PredefinedTag::PredefinedTag;

// Java field: long TAG_NAME_SIZE
pub(crate) const TAG_NAME_SIZE: i64 = 1;

// Java: public class TaggedStructure
pub struct TaggedStructure {
    // Java field: BytesReader mReader
    // Java field: int mTagName
    pub(crate) mTagName: i32,
    // Java field: long mSize
    pub(crate) mSize: i64,
    // Java field: long mNumBytesStreamSize
    pub(crate) mNumBytesStreamSize: i64,
    // Java field: IsfReaderContext mContext
    // Java field: int mStartPos
    pub(crate) mStartPos: i32,
}

// Java: public class TaggedStructure
impl TaggedStructure {
    // Java: TaggedStructure(BytesReader reader, int tagName, IsfReaderContext context)
    pub fn TaggedStructure(reader: &mut BytesReader, tagName: i32) -> Self {
        // Java: super();
        // Java: KSAssert.assertNotNull("reader should not be null!", reader);
        // Java: KSAssert.assertNotNull("context should not be null!", context);
        // Java: this.mReader = reader;
        // Java: this.mTagName = tagName;
        // Java: this.mContext = context;
        // Java: this.unpackSize();
        let start = reader.pos();
        let size = Multibytecoding::decodeUInt(reader);
        let num_bytes_stream_size = (reader.pos() - start) as i64;
        let start_pos = reader.pos();
        Self {
            mTagName: tagName,
            mSize: size,
            mNumBytesStreamSize: num_bytes_stream_size,
            mStartPos: start_pos,
        }
    }

    // Java: int type()
    pub fn r#type(&self) -> i32 {
        // Java: return 100;
        100
    }

    // Java: void unpackSize()
    pub fn unpackSize(&mut self, reader: &mut BytesReader) {
        // Java: int start = this.mReader.pos();
        let start = reader.pos();
        // Java: this.mSize = Multibytecoding.decodeUInt(this.mReader);
        self.mSize = Multibytecoding::decodeUInt(reader);
        // Java: this.mNumBytesStreamSize = (long)(this.mReader.pos() - start);
        self.mNumBytesStreamSize = (reader.pos() - start) as i64;
        // Java: this.mStartPos = this.mReader.pos();
        self.mStartPos = reader.pos();
    }

    // Java: long size()
    pub fn size(&self) -> i64 {
        // Java: return 1L + this.mNumBytesStreamSize + this.mSize;
        1 + self.mNumBytesStreamSize + self.mSize
    }

    // Java: void processRemaining()
    pub fn processRemaining(&self, reader: &mut BytesReader) {
        // Java: KSAssert.assertNotNull("mReader should not be null!", this.mReader);
        // Java: long n = this.mSize - (long)(this.mReader.pos() - this.mStartPos);
        let n = self.mSize - (reader.pos() - self.mStartPos) as i64;
        // Java: for(long i = 0L; i < n; ++i)
        for _i in 0..n {
            // Java: this.mReader.getByte();
            reader.getByte();
        }
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
        // Java: return PredefinedTag.toName(this.mTagName);
        PredefinedTag::toName(self.mTagName)
    }
}
