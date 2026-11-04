// Java: import com.isf.utils.BytesReader
// Java: import com.isf.utils.Multibytecoding
use super::utils::BytesReader::BytesReader;
use super::utils::Multibytecoding::Multibytecoding;
use super::TaggedStructure::TaggedStructure;

// Java: public class TagHimetricSize extends TaggedStructure
pub struct TagHimetricSize {
    // Java field: int mWidth
    mWidth: i32,
    // Java field: int mHeight
    mHeight: i32,
    pub(crate) base: TaggedStructure,
}

// Java: public class TagHimetricSize extends TaggedStructure
impl TagHimetricSize {
    // Java: TagHimetricSize(BytesReader reader, int tagName, IsfReaderContext context)
    pub fn TagHimetricSize(reader: &mut BytesReader, tagName: i32) -> Self {
        // Java: super(reader, tagName, context);
        let base = TaggedStructure::TaggedStructure(reader, tagName);
        // Java: this.mWidth = (int)Multibytecoding.decodeInt(this.mReader);
        let mWidth = Multibytecoding::decodeInt(reader) as i32;
        // Java: this.mHeight = (int)Multibytecoding.decodeInt(this.mReader);
        let mHeight = Multibytecoding::decodeInt(reader) as i32;
        Self { mWidth, mHeight, base }
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
        // Java: StringBuilder sb = new StringBuilder();
        let mut sb = String::new();
        // Java: sb.append("TAG_HIMETRIC_SIZE\n");
        sb.push_str("TAG_HIMETRIC_SIZE\n");
        // Java: sb.append("mWidth: " + this.mWidth);
        sb.push_str(&format!("mWidth: {}", self.mWidth));
        // Java: sb.append(' ');
        sb.push(' ');
        // Java: sb.append("mHeight: " + this.mHeight);
        sb.push_str(&format!("mHeight: {}", self.mHeight));
        // Java: sb.append('\n');
        sb.push('\n');
        // Java: return sb.toString();
        sb
    }
}
