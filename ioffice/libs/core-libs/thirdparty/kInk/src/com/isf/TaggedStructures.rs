// Java: import cn.wps.base.assertion.KSAssert
// Java: import com.isf.utils.BytesReader
// Java: import java.util.LinkedList
use super::utils::BytesReader::BytesReader;
use super::TaggedStructure::TaggedStructure;
use super::DefaultTaggedStructureFactory::DefaultTaggedStructureFactory;

// Java: public class TaggedStructures extends LinkedList<TaggedStructure>
pub struct TaggedStructures {
    // Java field: long serialVersionUID
    // Java field: BytesReader mReader
    // Java field: long mSize
    mSize: i64,
    // Java field: IsfReaderContext mContext
    // Java field: DefaultTaggedStructureFactory mFactory
    mFactory: DefaultTaggedStructureFactory,
    items: Vec<TaggedStructure>,
}

// Java: public class TaggedStructures extends LinkedList<TaggedStructure>
impl TaggedStructures {
    // Java: TaggedStructures(BytesReader reader, long size, IsfReaderContext context, DefaultTaggedStructureFactory factory)
    pub fn TaggedStructures(reader: &mut BytesReader, size: i64, factory: DefaultTaggedStructureFactory) -> Self {
        // Java: super();
        // Java: KSAssert.assertNotNull("reader should not be null!", reader);
        // Java: KSAssert.assertNotNull("context should not be null!", context);
        // Java: KSAssert.assertNotNull("factory should not be null!", factory);
        // Java: this.mReader = reader;
        // Java: this.mSize = size;
        // Java: this.mContext = context;
        // Java: this.mFactory = factory;
        // Java: this.process();
        let mut s = Self {
            mSize: size,
            mFactory: factory,
            items: Vec::new(),
        };
        s.process(reader);
        s
    }

    // Java: void process()
    pub fn process(&mut self, reader: &mut BytesReader) {
        // Java: KSAssert.assertNotNull("mFactory should not be null!", this.mFactory);
        // Java: int pos;
        // Java: TaggedStructure tag;
        // Java: for(pos = 0; (long)pos < this.mSize; pos = (int)((long)pos + tag.size()))
        let mut pos: i64 = 0;
        while pos < self.mSize {
            // Java: tag = this.mFactory.build(this.mReader);
            let tag = self.mFactory.build(reader);
            // Java: this.add(tag);
            let tag_size = tag.size();
            self.items.push(tag);
            pos += tag_size;
        }
        // Java: KSAssert.assertTrue("pos == mSize should be true!", (long)pos == this.mSize);
    }

    // Java: long sizeInBytes()
    pub fn sizeInBytes(&self) -> i64 {
        // Java: return this.mSize;
        self.mSize
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
        // Java: StringBuilder sb = new StringBuilder();
        let mut sb = String::new();
        // Java: for(TaggedStructure tag : this)
        for tag in &self.items {
            // Java: sb.append(tag.toString() + "\n");
            sb.push_str(&tag.toString());
            sb.push('\n');
        }
        // Java: return sb.toString();
        sb
    }
}
