// Java: import com.isf.utils.BytesReader
// Java: import com.isf.utils.Multibytecoding
use super::utils::BytesReader::BytesReader;
use super::utils::Multibytecoding::Multibytecoding;
use super::TaggedStructure::TaggedStructure;

// Java: public class TagTransform extends TaggedStructure
pub struct TagTransform {
    // Java field: float mScaleX
    mScaleX: f32,
    // Java field: float mScaleY
    mScaleY: f32,
    // Java field: float mShearX
    mShearX: f32,
    // Java field: float mShearY
    mShearY: f32,
    // Java field: float mDx
    mDx: f32,
    // Java field: float mDy
    mDy: f32,
    pub(crate) base: TaggedStructure,
}

// Java: public class TagTransform extends TaggedStructure
impl TagTransform {
    // Java: TagTransform(BytesReader reader, int tagName, IsfReaderContext context)
    pub fn TagTransform(reader: &mut BytesReader, tagName: i32) -> Self {
        // Java: super(reader, tagName, context);
        let base = TaggedStructure::TaggedStructure(reader, tagName);
        // Java: this.mScaleX = Multibytecoding.decodeFloat(this.mReader);
        let mScaleX = Multibytecoding::decodeFloat(reader);
        // Java: this.mShearY = Multibytecoding.decodeFloat(this.mReader);
        let mShearY = Multibytecoding::decodeFloat(reader);
        // Java: this.mShearX = Multibytecoding.decodeFloat(this.mReader);
        let mShearX = Multibytecoding::decodeFloat(reader);
        // Java: this.mScaleY = Multibytecoding.decodeFloat(this.mReader);
        let mScaleY = Multibytecoding::decodeFloat(reader);
        // Java: this.mDx = Multibytecoding.decodeFloat(this.mReader);
        let mDx = Multibytecoding::decodeFloat(reader);
        // Java: this.mDy = Multibytecoding.decodeFloat(this.mReader);
        let mDy = Multibytecoding::decodeFloat(reader);
        Self { mScaleX, mScaleY, mShearX, mShearY, mDx, mDy, base }
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

    // Java: float shearY()
    pub fn shearY(&self) -> f32 {
        // Java: return this.mShearY;
        self.mShearY
    }

    // Java: void unpackSize()
    pub fn unpackSize(&mut self, _reader: &mut BytesReader) {
    }

    // Java: long size()
    pub fn size(&self) -> i64 {
        // Java: return 25L;
        25
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
        // Java: StringBuilder sb = new StringBuilder();
        let mut sb = String::new();
        // Java: sb.append("TAG_TRANSFORM\n");
        sb.push_str("TAG_TRANSFORM\n");
        // Java: sb.append("mScaleX: " + this.mScaleX);
        sb.push_str(&format!("mScaleX: {}", self.mScaleX));
        // Java: sb.append(' ');
        sb.push(' ');
        // Java: sb.append("mScaleY: " + this.mScaleY);
        sb.push_str(&format!("mScaleY: {}", self.mScaleY));
        // Java: sb.append(' ');
        sb.push(' ');
        // Java: sb.append("mShearX: " + this.mShearX);
        sb.push_str(&format!("mShearX: {}", self.mShearX));
        // Java: sb.append(' ');
        sb.push(' ');
        // Java: sb.append("mShearY: " + this.mShearY);
        sb.push_str(&format!("mShearY: {}", self.mShearY));
        // Java: sb.append(' ');
        sb.push(' ');
        // Java: sb.append("mDx: " + this.mDx);
        sb.push_str(&format!("mDx: {}", self.mDx));
        // Java: sb.append(' ');
        sb.push(' ');
        // Java: sb.append("mDy: " + this.mDy);
        sb.push_str(&format!("mDy: {}", self.mDy));
        // Java: sb.append('\n');
        sb.push('\n');
        // Java: return sb.toString();
        sb
    }

    // Java: int type()
    pub fn r#type(&self) -> i32 {
        // Java: return 16;
        16
    }
}
