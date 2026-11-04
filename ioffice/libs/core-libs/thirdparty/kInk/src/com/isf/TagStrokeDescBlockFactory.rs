// Java: import cn.wps.base.assertion.KSAssert
// Java: import com.isf.utils.BytesReader
use super::utils::BytesReader::BytesReader;
use super::TaggedStructure::TaggedStructure;

// Java: public class TagStrokeDescBlockFactory extends DefaultTaggedStructureFactory
pub struct TagStrokeDescBlockFactory {
}

// Java: public class TagStrokeDescBlockFactory extends DefaultTaggedStructureFactory
impl TagStrokeDescBlockFactory {
    // Java: TagStrokeDescBlockFactory(IsfReaderContext context)
    pub fn TagStrokeDescBlockFactory() -> Self {
        // Java: super(context);
        Self {}
    }

    // Java: TaggedStructure processDefault(BytesReader reader, int tagName)
    pub fn processDefault(&self, reader: &mut BytesReader, tagName: i32) -> TaggedStructure {
        // Java: KSAssert.assertNotNull("reader should not be null!", reader);
        // Java: switch (tagName)
        // Java: case 6:
        // Java: case 7:
        // Java: case 8:
        // Java: case 9:
        // Java: case 10:
        // Java: case 11:
        // Java: default:
        // Java: return new TagEmpty(reader, tagName, this.mContext);
        TaggedStructure::TaggedStructure(reader, tagName)
    }
}
