// Java: import com.isf.utils.BytesReader
use super::utils::BytesReader::BytesReader;
use super::TaggedStructure::TaggedStructure;

// Java field: String EMPTY
const EMPTY: &str = "";

// Java: public class TagCustomUserIdTag extends TaggedStructure
pub struct TagCustomUserIdTag {
    // Java field: byte[] mUserId
    mUserId: Option<Vec<u8>>,
    pub(crate) base: TaggedStructure,
}

// Java: public class TagCustomUserIdTag extends TaggedStructure
impl TagCustomUserIdTag {
    // Java: TagCustomUserIdTag(BytesReader reader, int tagName, IsfReaderContext context)
    pub fn TagCustomUserIdTag(reader: &mut BytesReader, tagName: i32) -> Self {
        // Java: super(reader, tagName, context);
        let base = TaggedStructure::TaggedStructure(reader, tagName);
        // Java: this.readUserId();
        let mut s = Self {
            mUserId: None,
            base,
        };
        s.readUserId(reader);
        s
    }

    // Java: void readUserId()
    fn readUserId(&mut self, reader: &mut BytesReader) {
        // Java: long len = this.mSize;
        let len = self.base.mSize;
        // Java: this.mUserId = new byte[(int)len];
        self.mUserId = Some(vec![0u8; len as usize]);
        // Java: this.mReader.getByte();
        reader.getByte();
        // Java: for(int i = 0; (long)i < this.mSize; ++i)
        for i in 0..(self.base.mSize as usize) {
            // Java: this.mUserId[i] = this.mReader.getSingleByte();
            if let Some(ref mut user_id) = self.mUserId {
                user_id[i] = reader.getSingleByte() as u8;
            }
        }
        // Java: this.processRemaining();
        self.base.processRemaining(reader);
    }

    // Java: String getUserId()
    pub fn getUserId(&self) -> String {
        // Java: try
        // Java: return null != this.mUserId && this.mUserId.length > 0 ? new String(this.mUserId) : "";
        // Java: catch (Exception var2)
        // Java: return "";
        match &self.mUserId {
            Some(data) if !data.is_empty() => {
                String::from_utf8(data.clone()).unwrap_or_else(|_| String::new())
            }
            _ => String::new(),
        }
    }

    // Java: int type()
    pub fn r#type(&self) -> i32 {
        // Java: return 104;
        104
    }

    // Java: long size()
    pub fn size(&self) -> i64 {
        // Java: return super.size() + 1L;
        self.base.size() + 1
    }
}
