// Java: cn.wps.moffice.writer.io.reader.xml07.pi.PiSectPr
//
// ★M-STAGE1 P2-12 —— 全仓原缺失，按 Java 原文（5 行）新建。

// Java: import cn.wps.moffice.writer.io.reader.xml07.pi.IProcInfo
use crate::cn::wps::moffice::writer::io::reader::xml07::pi::IProcInfo::IProcInfo;

// java public class PiSectPr implements IProcInfo
#[derive(Default)]
pub struct PiSectPr {
    // java public String[] mHdrFdrs = new String[6];
    pub(crate) mHdrFdrs: [Option<String>; 6],
}

// java public class PiSectPr implements IProcInfo
impl PiSectPr {
    // rust: 原文无显式构造器（走默认构造 + 字段初始化器 `new String[6]`）⇒
    //   `new()` 为 Rust 侧便利方法。
    pub fn new() -> Self {
        Self::default()
    }
}

// java public class PiSectPr implements IProcInfo
impl IProcInfo for PiSectPr {
}
