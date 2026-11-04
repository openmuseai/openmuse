// Java: import com.isf.utils.BytesReader
// Java: import java.util.ArrayList
use super::utils::BytesReader::BytesReader;
use super::TaggedStructure::TaggedStructure;
use super::TagDrawAttrsBlock::TagDrawAttrsBlock;

// Java: public class TagDrawAttrsTable extends TaggedStructure
pub struct TagDrawAttrsTable {
    // Java field: ArrayList<TagDrawAttrsBlock> mDrawAttrsBlocks
    mDrawAttrsBlocks: Vec<TagDrawAttrsBlock>,
    pub(crate) base: TaggedStructure,
}

// Java: public class TagDrawAttrsTable extends TaggedStructure
impl TagDrawAttrsTable {
    // Java: TagDrawAttrsTable(BytesReader reader, int tagName, IsfReaderContext context)
    pub fn TagDrawAttrsTable(reader: &mut BytesReader, tagName: i32) -> Self {
        // Java: super(reader, tagName, context);
        let base = TaggedStructure::TaggedStructure(reader, tagName);
        // Java: this.process();
        let mut s = Self {
            mDrawAttrsBlocks: Vec::new(),
            base,
        };
        s.process(reader);
        s
    }

    // Java: int type()
    pub fn r#type(&self) -> i32 {
        // Java: return 2;
        2
    }

    // Java: TagDrawAttrsBlock item(int index)
    pub fn item(&self, index: i32) -> &TagDrawAttrsBlock {
        // Java: return (TagDrawAttrsBlock)this.mDrawAttrsBlocks.get(index);
        &self.mDrawAttrsBlocks[index as usize]
    }

    // Java: int countTagDrawAttrsBlock()
    pub fn countTagDrawAttrsBlock(&self) -> i32 {
        // Java: return this.mDrawAttrsBlocks.size();
        self.mDrawAttrsBlocks.len() as i32
    }

    // Java: void process()
    fn process(&mut self, reader: &mut BytesReader) {
        // Java: for(long nRemaining = this.mSize; nRemaining > 0L; ++nRemaining)
        let mut nRemaining = self.base.mSize;
        while nRemaining > 0 {
            // Java: TagDrawAttrsBlock block = new TagDrawAttrsBlock(this.mReader, 3, this.mContext);
            let block = TagDrawAttrsBlock::TagDrawAttrsBlock(reader, 3);
            // Java: this.mDrawAttrsBlocks.add(block);
            // Java: nRemaining -= block.size();
            nRemaining -= block.base.size();
            self.mDrawAttrsBlocks.push(block);
            nRemaining += 1;
        }
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
        // Java: StringBuilder sb = new StringBuilder();
        let mut sb = String::new();
        // Java: sb.append("TAG_DRAW_ATTRS_TABLE\n");
        sb.push_str("TAG_DRAW_ATTRS_TABLE\n");
        // Java: int size = this.mDrawAttrsBlocks.size();
        let size = self.mDrawAttrsBlocks.len();
        // Java: for(int i = 0; i < size; ++i)
        for i in 0..size {
            // Java: sb.append(((TagDrawAttrsBlock)this.mDrawAttrsBlocks.get(i)).toString());
            sb.push_str(&self.mDrawAttrsBlocks[i].toString());
        }
        // Java: return sb.toString();
        sb
    }
}
