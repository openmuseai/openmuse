// Java: import cn.wps.base.assertion.Assert
// use util::cn::wps::core::runtime::Assert;
// Java: import java.util.HashMap
use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

// Java field: String DOC
pub(crate) const DOC: &str = "doc";
// Java field: String DOT
pub(crate) const DOT: &str = "dot";
// Java field: String DOCX
pub(crate) const DOCX: &str = "docx";
// Java field: String DOTX
pub(crate) const DOTX: &str = "dotx";
// Java field: String DOCM
pub(crate) const DOCM: &str = "docm";
// Java field: String DOTM
pub(crate) const DOTM: &str = "dotm";
// Java field: String WPS
pub(crate) const WPS: &str = "wps";
// Java field: String WPT
pub(crate) const WPT: &str = "wpt";
// Java field: String RTF
pub(crate) const RTF: &str = "rtf";
// Java field: String XLS
pub(crate) const XLS: &str = "xls";
// Java field: String XLT
pub(crate) const XLT: &str = "xlt";
// Java field: String XLSX
pub(crate) const XLSX: &str = "xlsx";
// Java field: String XLSM
pub(crate) const XLSM: &str = "xlsm";
// Java field: String XLTX
pub(crate) const XLTX: &str = "xltx";
// Java field: String XLTM
pub(crate) const XLTM: &str = "xltm";
// Java field: String ET
pub(crate) const ET: &str = "et";
// Java field: String ETT
pub(crate) const ETT: &str = "ett";
// Java field: String XLSB
pub(crate) const XLSB: &str = "xlsb";
// Java field: String PPT
pub(crate) const PPT: &str = "ppt";
// Java field: String POT
pub(crate) const POT: &str = "pot";
// Java field: String PPTX
pub(crate) const PPTX: &str = "pptx";
// Java field: String POTX
pub(crate) const POTX: &str = "potx";
// Java field: String PPS
pub(crate) const PPS: &str = "pps";
// Java field: String PPSX
pub(crate) const PPSX: &str = "ppsx";
// Java field: String DPS
pub(crate) const DPS: &str = "dps";
// Java field: String DPT
pub(crate) const DPT: &str = "dpt";
// Java field: String TXT
pub(crate) const TXT: &str = "txt";
// Java field: String MHT
pub(crate) const MHT: &str = "mht";
// Java field: String MHTM
pub(crate) const MHTM: &str = "mhtm";
// Java field: String MHTML
pub(crate) const MHTML: &str = "mhtml";
// Java field: String HTML
pub(crate) const HTML: &str = "html";
// Java field: String HTM
pub(crate) const HTM: &str = "htm";
// Java field: String PDF
pub(crate) const PDF: &str = "pdf";
// Java field: String CSV
pub(crate) const CSV: &str = "csv";
// Java field: String XML
pub(crate) const XML: &str = "xml";
// Java field: String XML07
pub(crate) const XML07: &str = "xml07";
// Java field: String XML03
pub(crate) const XML03: &str = "xml03";
// Java field: String OFD
pub(crate) const OFD: &str = "ofd";
// Java field: String EPUB
pub(crate) const EPUB: &str = "epub";
// Java field: String UOT3
pub(crate) const UOT3: &str = "uot3";
// Java field: String UOS3
pub(crate) const UOS3: &str = "uos3";
// Java field: String UOP3
pub(crate) const UOP3: &str = "uop3";
// Java field: String XLAM
pub(crate) const XLAM: &str = "xlam";
// Java field: String POTM
pub(crate) const POTM: &str = "potm";
// Java field: HashMap<String, FileFormatEnum> sExt2FileFormatEnum
static sExt2FileFormatEnum: OnceLock<Mutex<HashMap<String, FileFormatEnum>>> = OnceLock::new();

// Java: public enum FileFormatEnum
pub struct FileFormatEnum {
    // Java field: String mExt
    pub(crate) mExt: String,
}

// Java: public class EXT
pub struct EXT {
}

// Java: public enum FileFormatEnum
impl FileFormatEnum {
    // Java: FileFormatEnum(String ext)
    pub fn new(ext: &str) -> Self {
      // Java: Assert.assertNotNull("ext should not be null.", ext);
      // Java: this.mExt = ext;
      // Java: FileFormatEnum.EXT.sExt2FileFormatEnum.put(ext, this);
      FileFormatEnum { mExt: ext.to_string() }
    }

    // Java: String getExt()
    pub fn getExt(&self) -> &str {
      // Java: return this.mExt;
      &self.mExt
    }

    // Java: static FileFormatEnum getFileFormatEnum(String ext)
    pub fn getFileFormatEnum(ext: &str) -> Option<FileFormatEnum> {
      // Java: Assert.assertNotNull("ext should not be null.", ext);
      // Java: return (FileFormatEnum)FileFormatEnum.EXT.sExt2FileFormatEnum.get(ext.toLowerCase());
      let pool = sExt2FileFormatEnum.get_or_init(|| Mutex::new(HashMap::new()));
      let map = pool.lock().unwrap();
      map.get(&ext.to_lowercase()).map(|f| FileFormatEnum { mExt: f.mExt.clone() })
    }

}

// Java: public class EXT
impl EXT {
    // Java: EXT()
    pub fn new() -> Self {
         // Java: super();
         EXT {}
    }

}
