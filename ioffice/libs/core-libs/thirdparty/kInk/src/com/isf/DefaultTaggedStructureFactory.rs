// Java: import cn.wps.base.assertion.KSAssert
// Java: import com.isf.utils.BytesReader
use super::utils::BytesReader::BytesReader;
use super::TaggedStructure::TaggedStructure;

// Java: public class DefaultTaggedStructureFactory
pub struct DefaultTaggedStructureFactory {
    // Java field: IsfReaderContext mContext
}

// Java: public class DefaultTaggedStructureFactory
impl DefaultTaggedStructureFactory {
    // Java: DefaultTaggedStructureFactory(IsfReaderContext context)
    pub fn DefaultTaggedStructureFactory() -> Self {
        // Java: super();
        // Java: KSAssert.assertNotNull("context should not be null!", context);
        // Java: this.mContext = context;
        Self {}
    }

    // Java: TaggedStructure build(BytesReader reader)
    pub fn build(&self, reader: &mut BytesReader) -> TaggedStructure {
        // Java: KSAssert.assertNotNull("reader should not be null!", reader);
        // Java: int tagName = reader.getByte();
        let tagName = reader.getByte() as i32;
        // Java: switch (tagName)
        match tagName {
            // Java: case 0:
            // Java: return new TagInkSpaceRect(reader, tagName, this.mContext);
            0 => TaggedStructure::TaggedStructure(reader, tagName),
            // Java: case 1:
            // Java: return new TagGuidTable(reader, tagName, this.mContext);
            1 => TaggedStructure::TaggedStructure(reader, tagName),
            // Java: case 2:
            // Java: return new TagDrawAttrsTable(reader, tagName, this.mContext);
            2 => TaggedStructure::TaggedStructure(reader, tagName),
            // Java: case 3:
            // Java: return new TagDrawAttrsBlock(reader, tagName, this.mContext);
            3 => TaggedStructure::TaggedStructure(reader, tagName),
            // Java: case 5:
            // Java: return new TagStrokeDescBlock(reader, tagName, this.mContext);
            5 => TaggedStructure::TaggedStructure(reader, tagName),
            // Java: case 9:
            // Java: return new TagDidx(reader, tagName, this.mContext);
            9 => TaggedStructure::TaggedStructure(reader, tagName),
            // Java: case 10:
            // Java: return new TagStroke(reader, tagName, this.mContext);
            10 => TaggedStructure::TaggedStructure(reader, tagName),
            // Java: case 11:
            // Java: return new TagStrokePropertyList(reader, tagName, this.mContext);
            11 => TaggedStructure::TaggedStructure(reader, tagName),
            // Java: case 13:
            // Java: return new TagSidx(reader, tagName, this.mContext);
            13 => TaggedStructure::TaggedStructure(reader, tagName),
            // Java: case 16:
            // Java: return new TagTransform(reader, tagName, this.mContext);
            16 => TaggedStructure::TaggedStructure(reader, tagName),
            // Java: case 17:
            // Java: return new TagTransformIsotropicScale(reader, tagName, this.mContext);
            17 => TaggedStructure::TaggedStructure(reader, tagName),
            // Java: case 18:
            // Java: return new TagTransformAnisotropicScale(reader, tagName, this.mContext);
            18 => TaggedStructure::TaggedStructure(reader, tagName),
            // Java: case 21:
            // Java: return new TagTransformScaleAndTranslate(reader, tagName, this.mContext);
            21 => TaggedStructure::TaggedStructure(reader, tagName),
            // Java: case 23:
            // Java: return new TagTidx(reader, tagName, this.mContext);
            23 => TaggedStructure::TaggedStructure(reader, tagName),
            // Java: case 25:
            // Java: return new TagMetricBlock(reader, tagName, this.mContext);
            25 => TaggedStructure::TaggedStructure(reader, tagName),
            // Java: case 26:
            // Java: return new TagMidx(reader, tagName, this.mContext);
            26 => TaggedStructure::TaggedStructure(reader, tagName),
            // Java: case 29:
            // Java: return new TagHimetricSize(reader, tagName, this.mContext);
            29 => TaggedStructure::TaggedStructure(reader, tagName),
            // Java: case 30:
            // Java: default:
            // Java: return this.processDefault(reader, tagName);
            // Java: case 52:
            // Java: return new TagMicrosoft_0x034(reader, tagName, this.mContext);
            52 => TaggedStructure::TaggedStructure(reader, tagName),
            // Java: case 100:
            // Java: return new TagEmpty(reader, tagName, this.mContext);
            100 => TaggedStructure::TaggedStructure(reader, tagName),
            // Java: case 104:
            // Java: return new TagCustomUserIdTag(reader, tagName, this.mContext);
            104 => TaggedStructure::TaggedStructure(reader, tagName),
            _ => self.processDefault(reader, tagName),
        }
    }

    // Java: TaggedStructure processDefault(BytesReader reader, int tagName)
    pub fn processDefault(&self, reader: &mut BytesReader, tagName: i32) -> TaggedStructure {
        // Java: KSAssert.assertNotNull("reader should not be null!", reader);
        // Java: return (TaggedStructure)(tagName >= 100 ? new TagCustomTag(reader, tagName, this.mContext) : new TagUnsupported(reader, tagName, this.mContext));
        TaggedStructure::TaggedStructure(reader, tagName)
    }
}
