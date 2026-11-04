// Java: import com.isf.utils.BytesReader
// Java: import com.isf.utils.Multibytecoding
use super::utils::BytesReader::BytesReader;
use super::utils::Multibytecoding::Multibytecoding;
use super::TaggedStructure::TaggedStructure;

// Java: public class TagTransformIsotropicScale extends TaggedStructure
pub struct TagTransformIsotropicScale {
    // Java field: float mScaleAmount
    mScaleAmount: f32,
    pub(crate) base: TaggedStructure,
}

// Java: public class TagTransformIsotropicScale extends TaggedStructure
impl TagTransformIsotropicScale {
    // Java: TagTransformIsotropicScale(BytesReader reader, int tagName, IsfReaderContext context)
    pub fn TagTransformIsotropicScale(reader: &mut BytesReader, tagName: i32) -> Self {
        // Java: super(reader, tagName, context);
        let base = TaggedStructure::TaggedStructure(reader, tagName);
        // Java: this.mScaleAmount = Multibytecoding.decodeFloat(this.mReader);
        let mScaleAmount = Multibytecoding::decodeFloat(reader);
        Self { mScaleAmount, base }
    }

    // Java: float getScaleAmount()
    pub fn getScaleAmount(&self) -> f32 {
        // Java: return this.mScaleAmount;
        self.mScaleAmount
    }

    // Java: void unpackSize()
    pub fn unpackSize(&mut self, _reader: &mut BytesReader) {
    }

    // Java: long size()
    pub fn size(&self) -> i64 {
        // Java: return 5L;
        5
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
        // Java: StringBuilder sb = new StringBuilder();
        let mut sb = String::new();
        // Java: sb.append("TAG_TRANSFORM_ISOTROPIC_SCALE\n");
        sb.push_str("TAG_TRANSFORM_ISOTROPIC_SCALE\n");
        // Java: sb.append("mScaleAmount: " + this.mScaleAmount);
        sb.push_str(&format!("mScaleAmount: {}", self.mScaleAmount));
        // Java: sb.append('\n');
        sb.push('\n');
        // Java: return sb.toString();
        sb
    }

    // Java: int type()
    pub fn r#type(&self) -> i32 {
        // Java: return 17;
        17
    }
}
