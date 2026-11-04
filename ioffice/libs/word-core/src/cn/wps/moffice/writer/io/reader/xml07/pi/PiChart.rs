// Java: cn.wps.moffice.writer.io.reader.xml07.pi.PiChart
//
// ★M-STAGE1 P2-12 —— 全仓原缺失，按 Java 原文（5 行）新建。

// Java: import cn.wps.moffice.writer.io.reader.xml07.pi.IProcInfo
use crate::cn::wps::moffice::writer::io::reader::xml07::pi::IProcInfo::IProcInfo;

// java public class PiChart implements IProcInfo
#[derive(Default)]
pub struct PiChart {
    // java public String mRid = null;
    pub(crate) mRid: Option<String>,
}

// java public class PiChart implements IProcInfo
impl PiChart {
    // rust: 原文无显式构造器（走默认构造）⇒ `new()` 为 Rust 侧便利方法。
    pub fn new() -> Self {
        Self::default()
    }
}

// java public class PiChart implements IProcInfo
impl IProcInfo for PiChart {
}
