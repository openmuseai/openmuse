// Java: import com.isf.utils.BytesReader
// Java: import com.isf.utils.Multibytecoding
use super::utils::BytesReader::BytesReader;
use super::utils::Multibytecoding::Multibytecoding;
use super::TaggedStructure::TaggedStructure;

// Java: public class TagTransformAnisotropicScale extends TaggedStructure
pub struct TagTransformAnisotropicScale {
    // Java field: float mScaleX
    mScaleX: f32,
    // Java field: float mScaleY
    mScaleY: f32,
    pub(crate) base: TaggedStructure,
}

// Java: public class TagTransformAnisotropicScale extends TaggedStructure
impl TagTransformAnisotropicScale {
    // Java: TagTransformAnisotropicScale(BytesReader reader, int tagName, IsfReaderContext context)
    pub fn TagTransformAnisotropicScale(reader: &mut BytesReader, tagName: i32) -> Self {
        // Java: super(reader, tagName, context);
        let base = TaggedStructure::TaggedStructure(reader, tagName);
        // Java: this.mScaleX = Multibytecoding.decodeFloat(this.mReader);
        let mScaleX = Multibytecoding::decodeFloat(reader);
        // Java: this.mScaleY = Multibytecoding.decodeFloat(this.mReader);
        let mScaleY = Multibytecoding::decodeFloat(reader);
        Self { mScaleX, mScaleY, base }
    }

    // Java: void unpackSize()
    pub fn unpackSize(&mut self, _reader: &mut BytesReader) {
    }

    // Java: long size()
    pub fn size(&self) -> i64 {
        // Java: return 9L;
        9
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
        // Java: StringBuilder sb = new StringBuilder();
        let mut sb = String::new();
        // Java: sb.append("TAG_TRANSFORM_ANISOTROPIC_SCALE\n");
        sb.push_str("TAG_TRANSFORM_ANISOTROPIC_SCALE\n");
        // Java: sb.append("mScaleX: " + this.mScaleX);
        sb.push_str(&format!("mScaleX: {}", self.mScaleX));
        // Java: sb.append(' ');
        sb.push(' ');
        // Java: sb.append("mScaleY: " + this.mScaleY);
        sb.push_str(&format!("mScaleY: {}", self.mScaleY));
        // Java: sb.append('\n');
        sb.push('\n');
        // Java: return sb.toString();
        sb
    }
}
