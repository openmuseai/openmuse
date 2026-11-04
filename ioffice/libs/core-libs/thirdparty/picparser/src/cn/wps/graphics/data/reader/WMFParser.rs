#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
// Java: public class WMFParser extends MetaFileParser
pub struct WMFParser {
    // Java field: String TAG
    // Java field: byte[] tempBytes
    _private: (),
}

pub(crate) const TAG: &str = "WMFParser";

impl WMFParser {
    // Java: WMFParser(InputStream is)
    pub fn new() -> Self {
        // Java: super(is);
        WMFParser { _private: () }
    }

    // Java: WMFParser(InputStream is, int count)
    pub fn new_2() -> Self {
        // Java: super(is, count);
        WMFParser { _private: () }
    }

    // Java: WMFParser(byte[] bytes)
    pub fn new_3() -> Self {
        // Java: super(bytes);
        WMFParser { _private: () }
    }

    // Java: MetaFileHeader readMetaFileHeader()
    pub fn readMetaFileHeader(&self) -> i32 {
        // Java: WMFHeader header = new WMFHeader(this);
        // Java: return header;
        0
    }

    // Java: MetaFileRecord readRecord(boolean bEscape)
    pub fn readRecord(&self, _bEscape: bool) -> i32 {
        // Java: int length = this.readUnsignedInt() * 2 - 6;
        // Java: int recordID = this.readUnsignedShort();
        // Java: MetaFileRecord record = null;
        // Java: if (!bEscape && recordID == 1574)
        {
            // Java: record = this.defaultRecord;
        }
        // Java: else
        {
            // Java: Map<Integer, MetaFileRecord> recordMap = RecordMaps.getWmfRecordMap();
            // Java: record = (MetaFileRecord)recordMap.get(recordID);
            // Java: if (record == null)
            {
                // Java: record = this.defaultRecord;
            }
        }
        // Java: record = record.parser(this, length);
        // Java: return record;
        0
    }

    // Java: Rectangle readRect()
    pub fn readRect(&self) -> i32 {
        // Java: int bottom = this.readShort();
        // Java: int right = this.readShort();
        // Java: int top = this.readShort();
        // Java: int left = this.readShort();
        // Java: Rectangle rect = new Rectangle(left, top, right - left, bottom - top);
        // Java: return rect;
        0
    }

    // Java: Size getSize()
    pub fn getSize(&self) -> i32 {
        // Java: ArrayList<MetaFileRecord> records = new ArrayList();
        // Java: Size size = null;
        // Java: boolean var12 = false;
        // Java: label177:
        {
            // Java: try
            {
                // Java: var12 = true;
                // Java: this.reset();
                // Java: this.readMetaFileHeader();
                // Java: while(true)
                {
                    // Java: int length = this.readUnsignedInt() * 2 - 6;
                    // Java: int recordID = this.readUnsignedShort();
                    // Java: if (recordID == 0)
                    {
                        // Java: var12 = false;
                        // Java: break label177;
                    }
                    // Java: if (recordID != 524 && recordID != 526)
                    {
                        // Java: this.skip(length);
                    }
                    // Java: else
                    {
                        // Java: Map<Integer, MetaFileRecord> recordMap = RecordMaps.getWmfRecordMap();
                        // Java: MetaFileRecord record = (MetaFileRecord)recordMap.get(recordID);
                        // Java: record = record.read(this, length);
                        // Java: records.add(record);
                    }
                }
            }
            // Java: catch (Exception var13)
            {
                // Java: var12 = false;
            }
            // Java: finally
            {
                // Java: if (var12)
                {
                    // Java: int i = 0;
                    // Java: while(true)
                    {
                        // Java: if (i >= records.size())
                        {
                            // Java: ;
                        }
                        // Java: else
                        {
                            // Java: MetaFileRecord record = (MetaFileRecord)records.get(i);
                            // Java: if (record instanceof WMF_SetViewportExt)
                            {
                                // Java: size = ((WMF_SetViewportExt)record).getSize();
                            }
                            // Java: else if (record instanceof WMF_SetWindowExt)
                            {
                                // Java: size = ((WMF_SetWindowExt)record).getSize();
                            }
                            // Java: ++i;
                        }
                    }
                }
            }
            // Java: for(int var19 = 0; var19 < records.size(); ++var19)
            {
                // Java: MetaFileRecord record = (MetaFileRecord)records.get(var19);
                // Java: if (record instanceof WMF_SetViewportExt)
                {
                    // Java: size = ((WMF_SetViewportExt)record).getSize();
                }
                // Java: else if (record instanceof WMF_SetWindowExt)
                {
                    // Java: size = ((WMF_SetWindowExt)record).getSize();
                }
            }
            // Java: return size;
        }
        // Java: for(int i = 0; i < records.size(); ++i)
        {
            // Java: MetaFileRecord record = (MetaFileRecord)records.get(i);
            // Java: if (record instanceof WMF_SetViewportExt)
            {
                // Java: size = ((WMF_SetViewportExt)record).getSize();
            }
            // Java: else if (record instanceof WMF_SetWindowExt)
            {
                // Java: size = ((WMF_SetWindowExt)record).getSize();
            }
        }
        // Java: return size;
        0
    }

    // Java: static Size getSize(InputStream is)
    pub fn getSize_static(_is: i32) -> i32 {
        // Java: Size size = null;
        // Java: try
        {
            // Java: size = WMFHeader.getSize(is);
            // Java: if (size != null)
            {
                // Java: return size;
            }
            // Java: while(true)
            {
                // Java: int nRead = is.read(tempBytes);
                // Java: if (nRead <= 0)
                {
                    // Java: break;
                }
                // Java: int length = tempBytes[0] & 255 | (tempBytes[1] & 255) << 8 | (tempBytes[2] & 255) << 16 | (tempBytes[3] & 255) << 24;
                // Java: length = length * 2 - 6;
                // Java: int recordID = tempBytes[4] & 255 | (tempBytes[5] & 255) << 8;
                // Java: if (recordID == 0)
                {
                    // Java: break;
                }
                // Java: if (recordID != 524 && recordID != 526)
                {
                    // Java: is.skip((long)length);
                }
                // Java: else
                {
                    // Java: is.read(tempBytes, 0, 4);
                    // Java: float h = (float)((short)(tempBytes[0] & 255 | (tempBytes[1] & 255) << 8));
                    // Java: float w = (float)((short)(tempBytes[2] & 255 | (tempBytes[3] & 255) << 8));
                    // Java: size = new Size(Math.abs(w), Math.abs(h));
                    // Java: if (size.getWidth() > 1.0F && size.getHeight() > 1.0F)
                    {
                        // Java: break;
                    }
                }
            }
        }
        // Java: catch (Exception var7)
        {
        }
        // Java: return size;
        0
    }
}
