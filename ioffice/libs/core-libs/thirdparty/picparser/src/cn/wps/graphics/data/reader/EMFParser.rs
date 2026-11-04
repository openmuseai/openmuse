#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
// Java: public class EMFParser extends MetaFileParser
pub struct EMFParser {
    // Java field: String TAG
    _private: (),
}

pub(crate) const TAG: &str = "EMFParser";

impl EMFParser {
    // Java: EMFParser(InputStream is)
    pub fn new() -> Self {
        // Java: super(is);
        EMFParser { _private: () }
    }

    // Java: EMFParser(InputStream is, int count)
    pub fn new_2() -> Self {
        // Java: super(is, count);
        EMFParser { _private: () }
    }

    // Java: EMFParser(byte[] bytes)
    pub fn new_3() -> Self {
        // Java: super(bytes);
        EMFParser { _private: () }
    }

    // Java: public MetaFileHeader readMetaFileHeader()
    pub fn readMetaFileHeader(&self) -> i32 {
        // Java: EMFHeader header = new EMFHeader(this);
        // Java: return header;
        0
    }

    // Java: public MetaFileRecord readRecord(boolean bEscape)
    pub fn readRecord(&self, _bEscape: i32) -> i32 {
        // Java: int recordID = this.readUnsignedInt();
        // Java: int length = this.readUnsignedInt() - 8;
        // Java: Map<Integer, MetaFileRecord> recordMap = RecordMaps.getEmfRecordMap();
        // Java: MetaFileRecord record = (MetaFileRecord)recordMap.get(recordID);
        // Java: if (record == null)
        {
            // Java: record = this.defaultRecord;
        }
        // Java: record = record.parser(this, length);
        // Java: return record;
        0
    }

    // Java: public Rectangle readRect()
    pub fn readRect(&self) -> i32 {
        // Java: int left = this.readInt();
        // Java: int top = this.readInt();
        // Java: int right = this.readInt();
        // Java: int bottom = this.readInt();
        // Java: Rectangle rect = new Rectangle(left, top, right - left, bottom - top);
        // Java: return rect;
        0
    }

    // Java: public PointF readIntPoint()
    pub fn readIntPoint(&self) -> i32 {
        // Java: return new PointF((float)this.readInt(), (float)this.readInt());
        0
    }

    // Java: public Matrix readMatrix()
    pub fn readMatrix(&self) -> i32 {
        // Java: return GdiTool.createMatrix(this.readFloat(), this.readFloat(), this.readFloat(), this.readFloat(), this.readFloat(), this.readFloat());
        0
    }

    // Java: public Size getSize()
    pub fn getSize(&self) -> i32 {
        // Java: try
        {
            // Java: this.reset();
            // Java: EMFHeader header = (EMFHeader)this.readMetaFileHeader();
            // Java: return new Size(header.getWidth(), header.getHeight());
        }
        // Java: catch (Exception var2)
        {
            // Java: return null;
        }
        0
    }

    // Java: public static Size getSize(InputStream is)
    pub fn getSize_static(_is: i32) -> i32 {
        // Java: EMFParser parser = null;
        // Java: Size size = null;
        // Java: try
        {
            // Java: is.reset();
            // Java: parser = new EMFParser(is, 152);
            // Java: EMFHeader header = new EMFHeader(parser);
            // Java: size = new Size(header.getWidth(), header.getHeight());
        }
        // Java: catch (Exception var7)
        {
        }
        // Java: finally
        {
            // Java: if (null != parser)
            {
                // Java: parser.close();
                // Java: EMFParser var9 = null;
            }
        }
        // Java: return size;
        0
    }

    // Java: public static EMFHeader getHeader(InputStream is)
    pub fn getHeader(_is: i32) -> i32 {
        // Java: EMFParser parser = null;
        // Java: EMFHeader header = null;
        // Java: try
        {
            // Java: is.reset();
            // Java: parser = new EMFParser(is, 152);
            // Java: header = new EMFHeader(parser);
        }
        // Java: catch (Exception var7)
        {
        }
        // Java: finally
        {
            // Java: if (null != parser)
            {
                // Java: parser.close();
                // Java: EMFParser var9 = null;
            }
        }
        // Java: return header;
        0
    }
}
