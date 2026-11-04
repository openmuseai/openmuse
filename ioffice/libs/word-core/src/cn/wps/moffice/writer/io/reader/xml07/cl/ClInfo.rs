// Java: cn.wps.moffice.writer.io.reader.xml07.cl.ClInfo
//
// ★M-STAGE1 P2-12 —— 全仓原缺失，按 Java 原文（19 行）新建。

// Java: import cn.wps.moffice.writer.io.reader.xml07.co.ICoreObject
use crate::cn::wps::moffice::writer::io::reader::xml07::co::ICoreObject::ICoreObject;

// java public class ClInfo
#[derive(Default)]
pub struct ClInfo {
    // java public int mClType;
    pub(crate) mClType: i32,
    // java public String mRid;
    pub(crate) mRid: Option<String>,
    // java public ICoreObject mCo;
    // rust: ★Java 是接口引用；Rust 用 `Box<dyn ICoreObject>`（唯一所有权）
    pub(crate) mCo: Option<Box<dyn ICoreObject>>,
    // java public String mPartName;
    pub(crate) mPartName: Option<String>,
    // java public String mRawData;
    pub(crate) mRawData: Option<String>,
}

// java public class ClInfo
impl ClInfo {
    // java public ClInfo(int clType)
    pub fn ClInfo(clType: i32) -> Self {
      // java this.mClType = clType;
      let mClType: i32 = clType;
      Self {
        mClType,
        // java this.mRid = null;
        // java this.mCo = null;
        // java this.mPartName = null;
        // java this.mRawData = null;
        ..Default::default()
      }
    }
}
