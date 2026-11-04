#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
// Java: public class MetaFileParser extends BufferReader
pub struct MetaFileParser {
    // Java field: String TAG
    // Java field: int TYPE_UNKNOWN
    // Java field: int TYPE_EMF
    // Java field: int TYPE_WMF
    // Java field: List<MetaFileRecord> lstRecord
    lstRecord: Option<i32>,
    // Java field: MetaFileHeader header
    header: Option<i32>,
    // Java field: MetaFileRecord defaultRecord
    defaultRecord: Option<i32>,
}

pub(crate) const TAG: &str = "MetaFileParser";
pub(crate) const TYPE_UNKNOWN: i32 = 0;
pub(crate) const TYPE_EMF: i32 = 1;
pub(crate) const TYPE_WMF: i32 = 2;

impl MetaFileParser {
    // Java: MetaFileParser(InputStream is)
    pub fn new() -> Self {
        // Java: super(is);
        MetaFileParser {
            lstRecord: None,
            header: None,
            defaultRecord: None,
        }
    }

    // Java: MetaFileParser(InputStream is, int count)
    pub fn new_2() -> Self {
        // Java: super(is, count);
        MetaFileParser {
            lstRecord: None,
            header: None,
            defaultRecord: None,
        }
    }

    // Java: MetaFileParser(byte[] bytes)
    pub fn new_3() -> Self {
        // Java: super(bytes);
        MetaFileParser {
            lstRecord: None,
            header: None,
            defaultRecord: None,
        }
    }

    // Java: public void run()
    pub fn run(&self) {
        // Java: this.reset();
        // Java: this.header = this.readMetaFileHeader();
        // Java: if (!this.header.isValid())
        {
            // Java: KSLog.d(TAG, "MetaFileHeadInValid --> Close stream parser");
        }
        // Java: else
        {
            // Java: MetaFileRecord lastRecord = null;
            // Java: MetaFileRecord record = null;
            // Java: while(null != (record = this.readRecord(false)))
            {
                // Java: if (record instanceof WMF_ExcludeClipRect)
                {
                    // Java: WMF_ExcludeClipRect clipRecord = (WMF_ExcludeClipRect)record;
                    // Java: if (lastRecord != null)
                    {
                        // Java: ((WMF_ExcludeClipRect)lastRecord).add(clipRecord);
                    }
                    // Java: else
                    {
                        // Java: this.lstRecord.add(record);
                        // Java: lastRecord = clipRecord;
                    }
                }
                // Java: else if (record instanceof ExcludeClipRect)
                {
                    // Java: ExcludeClipRect clipRecord = (ExcludeClipRect)record;
                    // Java: if (lastRecord != null)
                    {
                        // Java: ((ExcludeClipRect)lastRecord).add(clipRecord);
                    }
                    // Java: else
                    {
                        // Java: this.lstRecord.add(record);
                        // Java: lastRecord = clipRecord;
                    }
                }
                // Java: else
                {
                    // Java: this.lstRecord.add(record);
                    // Java: lastRecord = null;
                }
            }
        }
    }

    // Java: public String runEmbed()
    pub fn runEmbed(&self) -> String {
        // Java: this.reset();
        // Java: this.header = this.readMetaFileHeader();
        // Java: if (!this.header.isValid())
        {
            // Java: KSLog.d(TAG, "MetaFileHeadInValid --> Close stream parser");
            // Java: return null;
        }
        // Java: else
        {
            // Java: MetaFileRecord escape = (MetaFileRecord)RecordMaps.getWmfRecordMap().get(1574);
            // Java: if (escape != null && escape instanceof WMF_Escape)
            {
                // Java: WMF_Escape wmf_escape = (WMF_Escape)escape;
                // Java: wmf_escape.reset();
            }
            // Java: MetaFileRecord lastRecord = null;
            // Java: MetaFileRecord record = null;
            // Java: while(null != (record = this.readRecord(true)))
            {
                // Java: if (record instanceof WMF_ExcludeClipRect)
                {
                    // Java: WMF_ExcludeClipRect clipRecord = (WMF_ExcludeClipRect)record;
                    // Java: if (lastRecord != null)
                    {
                        // Java: ((WMF_ExcludeClipRect)lastRecord).add(clipRecord);
                    }
                    // Java: else
                    {
                        // Java: this.lstRecord.add(record);
                        // Java: lastRecord = clipRecord;
                    }
                }
                // Java: else if (record instanceof ExcludeClipRect)
                {
                    // Java: ExcludeClipRect clipRecord = (ExcludeClipRect)record;
                    // Java: if (lastRecord != null)
                    {
                        // Java: ((ExcludeClipRect)lastRecord).add(clipRecord);
                    }
                    // Java: else
                    {
                        // Java: this.lstRecord.add(record);
                        // Java: lastRecord = clipRecord;
                    }
                }
                // Java: else
                {
                    // Java: if (record instanceof WMF_Escape)
                    {
                        // Java: WMF_Escape wmfEscape = (WMF_Escape)record;
                        // Java: WMF_Escape_Enhanced_MetaFile metafile = wmfEscape.getEnhancedMetaFile();
                        // Java: if (metafile != null && metafile.isValid())
                        {
                            // Java: String path = metafile.getPath();
                            // Java: if (path != null)
                            {
                                // Java: return path;
                            }
                        }
                    }
                    // Java: else
                    {
                        // Java: this.lstRecord.add(record);
                    }
                    // Java: lastRecord = null;
                }
            }
            // Java: return null;
        }
        String::new()
    }

    // Java: public MetaFileHeader getHeader()
    pub fn getHeader(&self) -> i32 {
        // Java: return this.header;
        0
    }

    // Java: public List<MetaFileRecord> getRecords()
    pub fn getRecords(&self) -> i32 {
        // Java: return this.lstRecord;
        0
    }

    // Java: public abstract MetaFileHeader readMetaFileHeader()
    pub fn readMetaFileHeader(&self) -> i32 {
        0
    }

    // Java: public abstract MetaFileRecord readRecord(boolean var1)
    pub fn readRecord(&self, _var1: bool) -> i32 {
        0
    }

    // Java: public abstract Rectangle readRect()
    pub fn readRect(&self) -> i32 {
        0
    }

    // Java: public abstract Size getSize()
    pub fn getSize(&self) -> i32 {
        0
    }
}
