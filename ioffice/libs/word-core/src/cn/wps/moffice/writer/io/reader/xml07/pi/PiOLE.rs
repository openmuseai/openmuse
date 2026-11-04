// Java: import cn.wps.moffice.writer.io.reader.xml07.pi.IProcInfo
use crate::cn::wps::moffice::writer::io::reader::xml07::pi::IProcInfo::*;

// java public class PiOLE implements IProcInfo
#[derive(Default)]
pub struct PiOLE {
    // java public String mRawData = null;
    pub(crate) mRawData: Option<String>,
}

// java public interface IProcInfo
impl IProcInfo for PiOLE {
}

// java public class PiOLE implements IProcInfo
impl PiOLE {
}
