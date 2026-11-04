#![allow(non_snake_case, dead_code, unused_variables, non_upper_case_globals)]

// Java: import cn.wps.base.assertion.Assert
use crate::cn::wps::base::assertion::Assert::Assert;
// Java: import java.util.HashMap
use std::collections::HashMap;
use std::sync::OnceLock;
use std::sync::Mutex;

// Java field: String DOC
pub const DOC: &str = "doc";
// Java field: String DOT
pub const DOT: &str = "dot";
// Java field: String WPS
pub const WPS: &str = "wps";
// Java field: String WPT
pub const WPT: &str = "wpt";
// Java field: String DOCX
pub const DOCX: &str = "docx";
// Java field: String DOTX
pub const DOTX: &str = "dotx";
// Java field: String DOCM
pub const DOCM: &str = "docm";
// Java field: String DOTM
pub const DOTM: &str = "dotm";
// Java field: String TXT
pub const TXT: &str = "txt";
// Java field: String PDF
pub const PDF: &str = "pdf";
// Java field: String RTF
pub const RTF: &str = "rtf";
// Java field: String XML07
pub const XML07: &str = "xml07";
// Java field: String XML03
pub const XML03: &str = "xml03";
// Java field: String UOT3
pub const UOT3: &str = "uot3";
// Java field: String EPUB
pub const EPUB: &str = "epub";
// Java field: HashMap<String, FileFormat> sMap
static S_MAP: OnceLock<HashMap<String, FileFormat>> = OnceLock::new();

fn get_map() -> &'static HashMap<String, FileFormat> {
    S_MAP.get_or_init(|| {
        let mut map = HashMap::new();
        map.insert("doc".to_string(), FF_DOC);
        map.insert("dot".to_string(), FF_DOC);
        map.insert("wps".to_string(), FF_DOC);
        map.insert("wpt".to_string(), FF_DOC);
        map.insert("docx".to_string(), FF_DOCX);
        map.insert("dotx".to_string(), FF_DOTX);
        map.insert("txt".to_string(), FF_TXT);
        map.insert("pdf".to_string(), FF_PDF);
        map.insert("rtf".to_string(), FF_RTF);
        map.insert("xml07".to_string(), FF_XML07);
        map.insert("xml03".to_string(), FF_XML03);
        map.insert("epub".to_string(), FF_EPUB);
        map.insert("uot3".to_string(), FF_UOT3);
        map
    })
}

// Java: public enum FileFormat
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum FileFormat {
    FF_UNKNOWN,
    FF_DOC,
    FF_DOCX,
    FF_DOTX,
    FF_TXT,
    FF_EPUB,
    FF_HTML,
    FF_PDF,
    FF_RTF,
    FF_XML07,
    FF_MHT,
    FF_XML03,
    FF_UOT3,
}

pub const FF_UNKNOWN: FileFormat = FileFormat::FF_UNKNOWN;
pub const FF_DOC: FileFormat = FileFormat::FF_DOC;
pub const FF_DOCX: FileFormat = FileFormat::FF_DOCX;
pub const FF_DOTX: FileFormat = FileFormat::FF_DOTX;
pub const FF_TXT: FileFormat = FileFormat::FF_TXT;
pub const FF_EPUB: FileFormat = FileFormat::FF_EPUB;
pub const FF_HTML: FileFormat = FileFormat::FF_HTML;
pub const FF_PDF: FileFormat = FileFormat::FF_PDF;
pub const FF_RTF: FileFormat = FileFormat::FF_RTF;
pub const FF_XML07: FileFormat = FileFormat::FF_XML07;
pub const FF_MHT: FileFormat = FileFormat::FF_MHT;
pub const FF_XML03: FileFormat = FileFormat::FF_XML03;
pub const FF_UOT3: FileFormat = FileFormat::FF_UOT3;

// Java: public enum FileFormat
impl FileFormat {
    // Java: FileFormat()
    pub fn new() -> Self {
        FileFormat::FF_UNKNOWN
    }

    fn name(&self) -> &str {
        match self {
            FileFormat::FF_UNKNOWN => "FF_UNKNOWN",
            FileFormat::FF_DOC => "FF_DOC",
            FileFormat::FF_DOCX => "FF_DOCX",
            FileFormat::FF_DOTX => "FF_DOTX",
            FileFormat::FF_TXT => "FF_TXT",
            FileFormat::FF_EPUB => "FF_EPUB",
            FileFormat::FF_HTML => "FF_HTML",
            FileFormat::FF_PDF => "FF_PDF",
            FileFormat::FF_RTF => "FF_RTF",
            FileFormat::FF_XML07 => "FF_XML07",
            FileFormat::FF_MHT => "FF_MHT",
            FileFormat::FF_XML03 => "FF_XML03",
            FileFormat::FF_UOT3 => "FF_UOT3",
        }
    }

    // Java: static FileFormat fromExtension(String ext)
    pub fn fromExtension(ext: &str) -> FileFormat {
        // Java: Assert.assertNotNull("ext should not be null.", ext);
        let _ext_opt: Option<&str> = Some(ext);
        Assert::assertNotNull_msg("ext should not be null.", &_ext_opt);
        // Java: ext = ext.trim();
        let ext = ext.trim();
        // Java: ext = ext.toLowerCase();
        let ext_lower = ext.to_lowercase();
        // Java: FileFormat format = (FileFormat)sMap.get(ext);
        let map = get_map();
        let mut format = map.get(&ext_lower).copied();
        // Java: if (null == format && ext.equals("xml"))
        if format.is_none() && ext_lower == "xml" {
            // Java: format = (FileFormat)sMap.get("xml07");
            format = map.get("xml07").copied();
        }
        // Java: return null != format ? format : FF_UNKNOWN;
        format.unwrap_or(FF_UNKNOWN)
    }

    // Java: static String getFormatStr(FileFormat fileFormat)
    pub fn getFormatStr(file_format: FileFormat) -> String {
        // Java: String str = fileFormat.name();
        let str_val = file_format.name();
        // Java: if (str.startsWith("FF_"))
        let result = if str_val.starts_with("FF_") {
            // Java: str = str.substring(3);
            &str_val[3..]
        } else {
            str_val
        };
        // Java: return str.toLowerCase();
        result.to_lowercase()
    }
}
