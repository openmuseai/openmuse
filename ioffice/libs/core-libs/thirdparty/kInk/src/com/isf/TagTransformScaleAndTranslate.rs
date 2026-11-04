// Java: import com.isf.utils.BytesReader
// Java: import com.isf.utils.Multibytecoding
use super::utils::BytesReader::BytesReader;
use super::utils::Multibytecoding::Multibytecoding;
use super::TaggedStructure::TaggedStructure;

// Java: public class TagTransformScaleAndTranslate extends TaggedStructure
pub struct TagTransformScaleAndTranslate {
    // Java field: float mScaleX
    mScaleX: f32,
    // Java field: float mScaleY
    mScaleY: f32,
    // Java field: float mDX
    mDX: f32,
    // Java field: float mDY
    mDY: f32,
    pub(crate) base: TaggedStructure,
}

// Java: public class TagTransformScaleAndTranslate extends TaggedStructure
impl TagTransformScaleAndTranslate {
    // Java: TagTransformScaleAndTranslate(BytesReader reader, int tagName, IsfReaderContext context)
    pub fn TagTransformScaleAndTranslate(reader: &mut BytesReader, tagName: i32) -> Self {
        // Java: super(reader, tagName, context);
        let base = TaggedStructure::TaggedStructure(reader, tagName);
        // Java: this.mScaleX = Multibytecoding.decodeFloat(this.mReader);
        let mScaleX = Multibytecoding::decodeFloat(reader);
        // Java: this.mScaleY = Multibytecoding.decodeFloat(this.mReader);
        let mScaleY = Multibytecoding::decodeFloat(reader);
        // Java: this.mDX = Multibytecoding.decodeFloat(this.mReader);
        let mDX = Multibytecoding::decodeFloat(reader);
        // Java: this.mDY = Multibytecoding.decodeFloat(this.mReader);
        let mDY = Multibytecoding::decodeFloat(reader);
        Self { mScaleX, mScaleY, mDX, mDY, base }
    }

    // Java: float scaleX()
    pub fn scaleX(&self) -> f32 {
        // Java: return this.mScaleX;
        self.mScaleX
    }

    // Java: float scaleY()
    pub fn scaleY(&self) -> f32 {
        // Java: return this.mScaleY;
        self.mScaleY
    }

    // Java: float dX()
    pub fn dX(&self) -> f32 {
        // Java: return this.mDX;
        self.mDX
    }

    // Java: float dY()
    pub fn dY(&self) -> f32 {
        // Java: return this.mDY;
        self.mDY
    }

    // Java: void unpackSize()
    pub fn unpackSize(&mut self, _reader: &mut BytesReader) {
    }

    // Java: long size()
    pub fn size(&self) -> i64 {
        // Java: return 17L;
        17
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
        // Java: StringBuilder sb = new StringBuilder();
        let mut sb = String::new();
        // Java: sb.append("TAG_TRANSFORM_SCALE_AND_TRANSLATE\n");
        sb.push_str("TAG_TRANSFORM_SCALE_AND_TRANSLATE\n");
        // Java: sb.append("mScaleX: " + this.mScaleX);
        sb.push_str(&format!("mScaleX: {}", self.mScaleX));
        // Java: sb.append(' ');
        sb.push(' ');
        // Java: sb.append("mScaleY: " + this.mScaleY);
        sb.push_str(&format!("mScaleY: {}", self.mScaleY));
        // Java: sb.append(' ');
        sb.push(' ');
        // Java: sb.append("mDX: " + this.mDX);
        sb.push_str(&format!("mDX: {}", self.mDX));
        // Java: sb.append(' ');
        sb.push(' ');
        // Java: sb.append("mDY: " + this.mDY);
        sb.push_str(&format!("mDY: {}", self.mDY));
        // Java: sb.append('\n');
        sb.push('\n');
        // Java: return sb.toString();
        sb
    }

    // Java: int type()
    pub fn r#type(&self) -> i32 {
        // Java: return 21;
        21
    }
}
