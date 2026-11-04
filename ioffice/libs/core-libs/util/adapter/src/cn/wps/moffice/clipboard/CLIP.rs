// Java: import cn.wps.core.runtime.Platform
use crate::cn::wps::core::runtime::Platform;
// Java: import java.util.HashMap
use std::collections::HashMap;

// Java field: String SIGNATURE_CLIP
pub(crate) const SIGNATURE_CLIP: &str = "Clip Data of Android WPS";
// Java field: String CLIP_FOLDER
static CLIP_FOLDER: std::sync::OnceLock<std::sync::Mutex<String>> = std::sync::OnceLock::new();
// Java field: HashMap<String, String> sMimeType2Path
static S_MIME_TYPE2_PATH: std::sync::OnceLock<std::sync::Mutex<HashMap<String, String>>> = std::sync::OnceLock::new();
// Java field: int PASTE_SUCCESS
pub(crate) const PASTE_SUCCESS: i32 = 1;
// Java field: int PASTE_FAIL
pub(crate) const PASTE_FAIL: i32 = 2;
// Java field: int PASTE_WRONGFUL_REGION
pub(crate) const PASTE_WRONGFUL_REGION: i32 = 3;
// Java field: int PASTE_TABLE_OUT_OF_RANGE
pub(crate) const PASTE_TABLE_OUT_OF_RANGE: i32 = 4;
// Java field: String ALL
pub(crate) const ALL: &str = "*/*";
// Java field: String HTML
pub(crate) const HTML: &str = "text/html";
// Java field: String TEXT
pub(crate) const TEXT: &str = "text/plain";
// Java field: String DOCX
pub(crate) const DOCX: &str = "application/vnd.openxmlformats-officedocument.wordprocessingml.document";
// Java field: String GVML
pub(crate) const GVML: &str = "application/vnd.openxmlformats-officedocument.clipboard.drawing";
// Java field: String PPT14_SLIDES
pub(crate) const PPT14_SLIDES: &str = "application/vnd.openxmlformats-officedocument.clipboard.ppt14slides";
// Java field: String ART_TABLE
pub(crate) const ART_TABLE: &str = "application/vnd.openxmlformats-officedocument.clipboard.arttable";
// Java field: String SXML
pub(crate) const SXML: &str = "XML Spreadsheet";
// Java field: String SS_HTML
pub(crate) const SS_HTML: &str = "application/vnd.moffice.spreadsheet.html";
// Java field: String SS_INERNAL
pub(crate) const SS_INERNAL: &str = "application/vnd.moffice.spreadsheet.internal";
// Java field: String PP_INERNAL
pub(crate) const PP_INERNAL: &str = "application/vnd.moffice.pownerpoint.internal";
// Java field: String SS_CUSTOM_CLIPBOARD
pub(crate) const SS_CUSTOM_CLIPBOARD: &str = "custom/clipboard/spreadsheet";
// Java field: String SS_COMPLETE_FORMULA
pub(crate) const SS_COMPLETE_FORMULA: &str = "custom/spreadsheet.complete.formula";
// Java field: String SS_INCOMPLETE_FORMULA
pub(crate) const SS_INCOMPLETE_FORMULA: &str = "custom/spreadsheet.incomplete.formula";
// Java field: String SUFFIX_GVML
pub(crate) const SUFFIX_GVML: &str = ".gvml";
// Java field: String SUFFIX_HTML
pub(crate) const SUFFIX_HTML: &str = ".html";
// Java field: String SUFFIX_TEXT
pub(crate) const SUFFIX_TEXT: &str = ".txt";
// Java field: String SUFFIX_EMBED_SRC
pub(crate) const SUFFIX_EMBED_SRC: &str = ".embedsource";
// Java field: String SUFFIX_SS_HTML
pub(crate) const SUFFIX_SS_HTML: &str = ".sshtml";
// Java field: String SUFFIX_PPT14_SLIDES
pub(crate) const SUFFIX_PPT14_SLIDES: &str = ".ppt14slides";
// Java field: String SUFFIX_ART_TABLE
pub(crate) const SUFFIX_ART_TABLE: &str = ".arttable";
// Java field: String SUFFIX_SXML
pub(crate) const SUFFIX_SXML: &str = ".xml";
// Java field: String GVML
static PATH_GVML: std::sync::OnceLock<std::sync::Mutex<String>> = std::sync::OnceLock::new();
// Java field: String HTML
static PATH_HTML: std::sync::OnceLock<std::sync::Mutex<String>> = std::sync::OnceLock::new();
// Java field: String TEXT
static PATH_TEXT: std::sync::OnceLock<std::sync::Mutex<String>> = std::sync::OnceLock::new();
// Java field: String EMBED_SRC
static PATH_EMBED_SRC: std::sync::OnceLock<std::sync::Mutex<String>> = std::sync::OnceLock::new();
// Java field: String SS_HTML
static PATH_SS_HTML: std::sync::OnceLock<std::sync::Mutex<String>> = std::sync::OnceLock::new();
// Java field: String PPT14_SLIDES
static PATH_PPT14_SLIDES: std::sync::OnceLock<std::sync::Mutex<String>> = std::sync::OnceLock::new();
// Java field: String ART_TABLE
static PATH_ART_TABLE: std::sync::OnceLock<std::sync::Mutex<String>> = std::sync::OnceLock::new();
// Java field: String SXML
static PATH_SXML: std::sync::OnceLock<std::sync::Mutex<String>> = std::sync::OnceLock::new();

// Java: public class CLIP
pub struct CLIP {
}

// Java: public class PasteResult
pub struct PasteResult {
}

// Java: public class MimeType
pub struct MimeType {
}

// Java: public class Path
pub struct CLIPPath {
}

// Java: public class CLIP
impl CLIP {
    // Java: CLIP()
    pub fn new() -> Self {
        // Java: super();
        CLIP {}
    }

    // Java: static String getClipPath(String mimeType)
    pub fn getClipPath(_mimeType: &str) -> Option<String> {
        // Java: return null == sMimeType2Path ? null : (String)sMimeType2Path.get(mimeType);
        todo!()
    }

    // Java: static String getUri()
    pub fn getUri() -> String {
        // Java: String authority = getAuthority();
        let authority = Self::getAuthority();
        // Java: return "content://" + authority + "/copy";
        format!("content://{}/copy", authority)
    }

    // Java: static String getAuthority()
    pub fn getAuthority() -> String {
        // Java: String authority = Platform.getClipAuthority();
        let mut authority = Platform::Platform::getClipAuthority();
        // Java: if (null == authority)
        if authority.is_none() {
            // Java: authority = "cn.wps.moffice.provider";
            authority = Some("cn.wps.moffice.provider".to_string());
        }
        // Java: return authority;
        authority.unwrap_or_default()
    }

    // Java: static void initPath()
    pub fn initPath() {
        // Java: CLIP_FOLDER = Platform.getClipboardDirectory();
        // Java: CLIP.Path.GVML = CLIP_FOLDER + "clip" + ".gvml";
        // Java: CLIP.Path.HTML = CLIP_FOLDER + "clip" + ".html";
        // Java: CLIP.Path.TEXT = CLIP_FOLDER + "clip" + ".txt";
        // Java: CLIP.Path.EMBED_SRC = CLIP_FOLDER + "clip" + ".embedsource";
        // Java: CLIP.Path.SS_HTML = CLIP_FOLDER + "clip" + ".sshtml";
        // Java: CLIP.Path.PPT14_SLIDES = CLIP_FOLDER + "clip" + ".ppt14slides";
        // Java: CLIP.Path.ART_TABLE = CLIP_FOLDER + "clip" + ".arttable";
        // Java: CLIP.Path.SXML = CLIP_FOLDER + "clip" + ".xml";
        // Java: sMimeType2Path = new HashMap();
        // Java: sMimeType2Path.put("text/html", CLIP.Path.HTML);
        // Java: sMimeType2Path.put("application/vnd.openxmlformats-officedocument.clipboard.drawing", CLIP.Path.GVML);
        // Java: sMimeType2Path.put("text/plain", CLIP.Path.TEXT);
        // Java: sMimeType2Path.put("application/vnd.moffice.spreadsheet.html", CLIP.Path.SS_HTML);
        // Java: sMimeType2Path.put("application/vnd.openxmlformats-officedocument.clipboard.ppt14slides", CLIP.Path.PPT14_SLIDES);
        // Java: sMimeType2Path.put("application/vnd.openxmlformats-officedocument.clipboard.arttable", CLIP.Path.ART_TABLE);
        // Java: sMimeType2Path.put("XML Spreadsheet", CLIP.Path.SXML);
        todo!()
    }

}

// Java: public class PasteResult
impl PasteResult {
    // Java: PasteResult()
    pub fn new() -> Self {
        // Java: super();
        PasteResult {}
    }

}

// Java: public class MimeType
impl MimeType {
    // Java: MimeType()
    pub fn new() -> Self {
        // Java: super();
        MimeType {}
    }

}

// Java: public class Path
impl CLIPPath {
    // Java: Path()
    pub fn new() -> Self {
        // Java: super();
        CLIPPath {}
    }

}
