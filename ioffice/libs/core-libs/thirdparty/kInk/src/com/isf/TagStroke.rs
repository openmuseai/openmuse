// Java: import cn.wps.base.assertion.KSAssert
// Java: import com.isf.utils.BytesReader
// Java: import com.isf.utils.Multibytecoding
// Java: import com.isf.utils.PacketDataDecompression
use super::utils::BytesReader::BytesReader;
use super::utils::Multibytecoding::Multibytecoding;
use super::utils::PacketDataDecompression::PacketDataDecompression;
use super::TaggedStructure::TaggedStructure;
use super::IsfReaderContext::IsfReaderContext;
use super::StrokeInfo::StrokeInfo;

// Java: public class TagStroke extends TaggedStructure
pub struct TagStroke {
    // Java field: int mCountPoints
    mCountPoints: i32,
    // Java field: int[] mX
    mX: Vec<i32>,
    // Java field: int[] mY
    mY: Vec<i32>,
    // Java field: int[] mButton
    mButton: Option<Vec<i32>>,
    pub(crate) base: TaggedStructure,
}

// Java: public class TagStroke extends TaggedStructure
impl TagStroke {
    // Java: TagStroke(BytesReader reader, int tagName, IsfReaderContext context)
    pub fn TagStroke(reader: &mut BytesReader, tagName: i32, context: &mut IsfReaderContext) -> Self {
        // Java: super(reader, tagName, context);
        let base = TaggedStructure::TaggedStructure(reader, tagName);
        // Java: KSAssert.assertNotNull("mReader should not be null!", this.mReader);
        // Java: this.mCountPoints = (int)Multibytecoding.decodeUInt(this.mReader);
        let mCountPoints = Multibytecoding::decodeUInt(reader) as i32;
        let mut s = Self {
            mCountPoints,
            mX: vec![0; mCountPoints as usize],
            mY: vec![0; mCountPoints as usize],
            mButton: None,
            base,
        };
        // Java: this.decompressXdata();
        s.decompressXdata(reader);
        // Java: this.decompressYdata();
        s.decompressYdata(reader);
        // Java: this.decompressButtonsData();
        s.decompressButtonsData(reader, context);
        // Java: this.processRemaining();
        s.base.processRemaining(reader);
        s
    }

    // Java: void decompressButtonsData()
    fn decompressButtonsData(&mut self, reader: &mut BytesReader, context: &mut IsfReaderContext) {
        // Java: KSAssert.assertNotNull("mContext should not be null!", this.mContext);
        // Java: if (null != this.mContext.mCurrentStrokeInfo && this.mContext.mCurrentStrokeInfo.mHasPressureData)
        let has_pressure = match &context.mCurrentStrokeInfo {
            Some(si) => si.mHasPressureData,
            None => false,
        };
        if has_pressure {
            // Java: KSAssert.assertNotNull("mReader should not be null!", this.mReader);
            // Java: this.mButton = new int[this.mCountPoints];
            let mut button = vec![0i32; self.mCountPoints as usize];
            // Java: PacketDataDecompression decompress = new PacketDataDecompression(this.mReader, this.mButton, this.mSize - 1L);
            let mut decompress = PacketDataDecompression::PacketDataDecompression(reader, &mut button, self.base.mSize - 1);
            // Java: decompress.inflate();
            decompress.inflate(reader);
            self.mButton = Some(button);
        }
    }

    // Java: void decompressXdata()
    fn decompressXdata(&mut self, reader: &mut BytesReader) {
        // Java: KSAssert.assertNotNull("mReader should not be null!", this.mReader);
        // Java: this.mX = new int[this.mCountPoints];
        // Java: PacketDataDecompression decompress = new PacketDataDecompression(this.mReader, this.mX, this.mSize - 1L);
        let mut decompress = PacketDataDecompression::PacketDataDecompression(reader, &mut self.mX, self.base.mSize - 1);
        // Java: decompress.inflate();
        decompress.inflate(reader);
    }

    // Java: void decompressYdata()
    fn decompressYdata(&mut self, reader: &mut BytesReader) {
        // Java: KSAssert.assertNotNull("mReader should not be null!", this.mReader);
        // Java: this.mY = new int[this.mCountPoints];
        // Java: PacketDataDecompression decompress = new PacketDataDecompression(this.mReader, this.mY, this.mSize - 1L);
        let mut decompress = PacketDataDecompression::PacketDataDecompression(reader, &mut self.mY, self.base.mSize - 1);
        // Java: decompress.inflate();
        decompress.inflate(reader);
    }

    // Java: int count()
    pub fn count(&self) -> i32 {
        // Java: return this.mCountPoints;
        self.mCountPoints
    }

    // Java: int item_x(int index)
    pub fn item_x(&self, index: i32) -> i32 {
        // Java: return this.mX[index];
        self.mX[index as usize]
    }

    // Java: int item_y(int index)
    pub fn item_y(&self, index: i32) -> i32 {
        // Java: return this.mY[index];
        self.mY[index as usize]
    }

    // Java: boolean hasPressureData()
    pub fn hasPressureData(&self) -> bool {
        // Java: return this.mButton != null;
        self.mButton.is_some()
    }

    // Java: int item_f(int index)
    pub fn item_f(&self, index: i32) -> i32 {
        // Java: return this.mButton[index];
        self.mButton.as_ref().map(|b| b[index as usize]).unwrap_or(0)
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
        // Java: StringBuilder sb = new StringBuilder();
        let mut sb = String::new();
        // Java: sb.append("TAG_STROKE\n");
        sb.push_str("TAG_STROKE\n");
        // Java: for(int i = 0; i < this.mCountPoints; ++i)
        for i in 0..(self.mCountPoints as usize) {
            // Java: sb.append("(");
            sb.push('(');
            // Java: sb.append(this.mX[i]);
            sb.push_str(&self.mX[i].to_string());
            // Java: sb.append(", ");
            sb.push_str(", ");
            // Java: sb.append(this.mY[i]);
            sb.push_str(&self.mY[i].to_string());
            // Java: sb.append("), ");
            sb.push_str("), ");
        }
        // Java: return sb.toString();
        sb
    }

    // Java: int type()
    pub fn r#type(&self) -> i32 {
        // Java: return 10;
        10
    }
}
