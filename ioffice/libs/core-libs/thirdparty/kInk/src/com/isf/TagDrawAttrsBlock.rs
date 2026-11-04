// Java: import cn.wps.base.assertion.KSAssert
// Java: import com.isf.utils.BytesReader
// Java: import com.isf.utils.Multibytecoding
use super::utils::BytesReader::BytesReader;
use super::utils::Multibytecoding::Multibytecoding;
use super::TaggedStructure::TaggedStructure;

// Java: public class TagDrawAttrsBlock extends TaggedStructure
pub struct TagDrawAttrsBlock {
    // Java field: int mPenWidth
    mPenWidth: i32,
    // Java field: int mPenColor
    mPenColor: i32,
    // Java field: int mPenHeight
    mPenHeight: i32,
    // Java field: boolean mPenTip
    mPenTip: bool,
    // Java field: int mPenTransparency
    mPenTransparency: i32,
    // Java field: boolean mIsHighlighter
    mIsHighlighter: bool,
    pub(crate) base: TaggedStructure,
}

// Java: public class TagDrawAttrsBlock extends TaggedStructure
impl TagDrawAttrsBlock {
    // Java: TagDrawAttrsBlock(BytesReader reader, int tagName, IsfReaderContext context)
    pub fn TagDrawAttrsBlock(reader: &mut BytesReader, tagName: i32) -> Self {
        // Java: super(reader, tagName, context);
        let base = TaggedStructure::TaggedStructure(reader, tagName);
        // Java: this.process();
        let mut s = Self {
            mPenWidth: 0,
            mPenColor: 0,
            mPenHeight: 0,
            mPenTip: false,
            mPenTransparency: 0,
            mIsHighlighter: false,
            base,
        };
        s.process(reader);
        s
    }

    // Java: int penWidth()
    pub fn penWidth(&self) -> i32 {
        // Java: return this.mPenWidth;
        self.mPenWidth
    }

    // Java: int penHeight()
    pub fn penHeight(&self) -> i32 {
        // Java: return this.mPenHeight;
        self.mPenHeight
    }

    // Java: int penColor()
    pub fn penColor(&self) -> i32 {
        // Java: return this.mPenColor;
        self.mPenColor
    }

    // Java: int penTransparency()
    pub fn penTransparency(&self) -> i32 {
        // Java: return this.mPenTransparency;
        self.mPenTransparency
    }

    // Java: boolean isRetangleTip()
    pub fn isRetangleTip(&self) -> bool {
        // Java: return this.mPenTip;
        self.mPenTip
    }

    // Java: boolean isHighLighter()
    pub fn isHighLighter(&self) -> bool {
        // Java: return this.mIsHighlighter;
        self.mIsHighlighter
    }

    // Java: int type()
    pub fn r#type(&self) -> i32 {
        // Java: return 3;
        3
    }

    // Java: void process()
    fn process(&mut self, reader: &mut BytesReader) {
        // Java: KSAssert.assertNotNull("mReader should not be null!", this.mReader);
        // Java: int end = (int)((long)this.mReader.pos() + this.mSize);
        let end = (reader.pos() as i64 + self.base.mSize) as i32;
        // Java: while(this.mReader.pos() < end)
        while reader.pos() < end {
            // Java: int property = (int)Multibytecoding.decodeUInt(this.mReader);
            let property = Multibytecoding::decodeUInt(reader) as i32;
            // Java: int value = (int)Multibytecoding.decodeUInt(this.mReader);
            let value = Multibytecoding::decodeUInt(reader) as i32;
            // Java: switch (property)
            match property {
                // Java: case 27:
                // Java: case 100:
                // Java: case 101:
                27 | 100 | 101 => {
                    // Java: for(int i = 0; i < value + 1; ++i)
                    for _i in 0..(value + 1) {
                        // Java: this.mReader.getByte();
                        reader.getByte();
                    }
                }
                // Java: case 67:
                // Java: case 72:
                // Java: break;
                67 | 72 => {}
                // Java: case 68:
                68 => {
                    // Java: this.mPenColor = value;
                    self.mPenColor = value;
                    // Java: break;
                }
                // Java: case 69:
                69 => {
                    // Java: this.mPenWidth = value;
                    self.mPenWidth = value;
                    // Java: break;
                }
                // Java: case 70:
                70 => {
                    // Java: this.mPenHeight = value;
                    self.mPenHeight = value;
                    // Java: break;
                }
                // Java: case 71:
                71 => {
                    // Java: if (1 == value)
                    if 1 == value {
                        // Java: this.mPenTip = true;
                        self.mPenTip = true;
                    }
                    // Java: break;
                }
                // Java: case 80:
                80 => {
                    // Java: this.mPenTransparency = value;
                    self.mPenTransparency = value;
                    // Java: break;
                }
                // Java: case 87:
                87 => {
                    // Java: if (9 == value)
                    if 9 == value {
                        // Java: this.mIsHighlighter = true;
                        self.mIsHighlighter = true;
                    }
                    // Java: break;
                }
                // Java: default:
                _ => {
                    // Java: int tagSize = (int)Multibytecoding.decodeUInt(this.mReader);
                    let tagSize = Multibytecoding::decodeUInt(reader) as i32;
                    // Java: for(int i = 0; i < tagSize && this.mReader.pos() < end; ++i)
                    for _i in 0..tagSize {
                        if reader.pos() >= end {
                            break;
                        }
                        // Java: this.mReader.getByte();
                        reader.getByte();
                    }
                }
            }
        }
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
        // Java: StringBuilder sb = new StringBuilder();
        let mut sb = String::new();
        // Java: sb.append("TAG_DRAW_ATRRS_BLOCK\n");
        sb.push_str("TAG_DRAW_ATRRS_BLOCK\n");
        // Java: sb.append("mPenWidth: " + this.mPenWidth);
        sb.push_str(&format!("mPenWidth: {}", self.mPenWidth));
        // Java: sb.append(' ');
        sb.push(' ');
        // Java: sb.append("mPenHeight: " + this.mPenHeight);
        sb.push_str(&format!("mPenHeight: {}", self.mPenHeight));
        // Java: sb.append(' ');
        sb.push(' ');
        // Java: sb.append("mPenColor: " + this.mPenColor);
        sb.push_str(&format!("mPenColor: {}", self.mPenColor));
        // Java: sb.append(' ');
        sb.push(' ');
        // Java: sb.append("mPenTip: " + this.mPenTip);
        sb.push_str(&format!("mPenTip: {}", self.mPenTip));
        // Java: sb.append(' ');
        sb.push(' ');
        // Java: sb.append("mPenTransparency: " + this.mPenTransparency);
        sb.push_str(&format!("mPenTransparency: {}", self.mPenTransparency));
        // Java: sb.append(' ');
        sb.push(' ');
        // Java: sb.append("mIsHighlighter: " + this.mIsHighlighter);
        sb.push_str(&format!("mIsHighlighter: {}", self.mIsHighlighter));
        // Java: sb.append('\n');
        sb.push('\n');
        // Java: return sb.toString();
        sb
    }
}
