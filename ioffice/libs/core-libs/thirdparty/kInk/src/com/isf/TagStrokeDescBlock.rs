// Java: import com.isf.utils.BytesReader
use super::utils::BytesReader::BytesReader;
use super::TaggedStructure::TaggedStructure;
use super::TaggedStructures::TaggedStructures;
use super::StrokeInfo::StrokeInfo;
use super::IsfReaderContext::IsfReaderContext;

// Java: public class TagStrokeDescBlock extends TaggedStructure
pub struct TagStrokeDescBlock {
    // Java field: TaggedStructures mTags
    mTags: TaggedStructures,
    pub(crate) base: TaggedStructure,
}

// Java: public class TagStrokeDescBlock extends TaggedStructure
impl TagStrokeDescBlock {
    // Java: TagStrokeDescBlock(BytesReader reader, int tagName, IsfReaderContext context)
    pub fn TagStrokeDescBlock(reader: &mut BytesReader, tagName: i32, context: &mut IsfReaderContext) -> Self {
        // Java: super(reader, tagName, context);
        let base = TaggedStructure::TaggedStructure(reader, tagName);
        // Java: if (null == this.mContext.mCurrentStrokeInfo)
        if context.mCurrentStrokeInfo.is_none() {
            // Java: this.mContext.mCurrentStrokeInfo = new StrokeInfo();
            context.mCurrentStrokeInfo = Some(StrokeInfo::StrokeInfo());
        }
        // Java: this.mTags = new TaggedStructures(this.mReader, this.mSize, context, context.mTagStrokeDescBlockFactory);
        let factory = std::mem::replace(
            &mut context.mDefaultTaggedStructureFactory,
            super::DefaultTaggedStructureFactory::DefaultTaggedStructureFactory::DefaultTaggedStructureFactory(),
        );
        let mTags = TaggedStructures::TaggedStructures(reader, base.mSize, factory);
        Self { mTags, base }
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
        // Java: StringBuilder sb = new StringBuilder();
        let mut sb = String::new();
        // Java: sb.append("TAG_STROKE_DESC_BLOCK - begin\n  ");
        sb.push_str("TAG_STROKE_DESC_BLOCK - begin\n  ");
        // Java: sb.append(this.mTags.toString());
        sb.push_str(&self.mTags.toString());
        // Java: sb.append("TAG_STROKE_DESC_BLOCK - end\n");
        sb.push_str("TAG_STROKE_DESC_BLOCK - end\n");
        // Java: return sb.toString();
        sb
    }
}
