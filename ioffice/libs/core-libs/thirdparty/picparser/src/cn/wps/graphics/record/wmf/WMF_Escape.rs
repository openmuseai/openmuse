#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
// Java: public class WMF_Escape extends WMFRecord
pub struct WMF_Escape {
    // Java field: int mEscapeFunction
    mEscapeFunction: i32,
    // Java field: WMF_Escape_Enhanced_MetaFile mMetaFile
    mMetaFile: Option<i32>,
}

impl WMF_Escape {
    // Java: WMF_Escape()
    pub fn new() -> Self {
        // Java: super();
        WMF_Escape {
            mEscapeFunction: 0,
            mMetaFile: None,
        }
    }

    // Java: public WMFRecord read(WMFParser wmf, int size)
    pub fn read(&self, _wmf: i32, _size: i32) -> i32 {
        // Java: WMF_Escape tag = new WMF_Escape();
        // Java: tag.mEscapeFunction = wmf.readUnsignedShort();
        // Java: wmf.skip(2);
        // Java: if (tag.mEscapeFunction == 15)
        {
        // Java: Log.i("test", "META_ESCAPE_ENHANCED_METAFILE");
        // Java: if (this.mMetaFile == null)
        {
        // Java: this.mMetaFile = new WMF_Escape_Enhanced_MetaFile();
        }
        // Java: if (this.mMetaFile.read(wmf, size) != null)
        {
        // Java: tag.mMetaFile = this.mMetaFile;
        // Java: this.mMetaFile = null;
        }
        }
        // Java: return tag;
        0
    }

    // Java: public void render(MetaFilePainter renderer)
    pub fn render(&self, _renderer: i32) {
    }

    // Java: public WMF_Escape_Enhanced_MetaFile getEnhancedMetaFile()
    pub fn getEnhancedMetaFile(&self) -> i32 {
        // Java: return this.mMetaFile;
        0
    }

    // Java: public void reset()
    pub fn reset(&self) {
        // Java: this.mMetaFile = null;
    }

}
