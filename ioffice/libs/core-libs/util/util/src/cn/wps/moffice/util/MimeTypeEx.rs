// Java: import java.util.HashMap
use std::collections::HashMap;
use std::sync::OnceLock;

// Java field: HashMap<String, String> sMimeMap
static S_MIME_MAP: OnceLock<HashMap<String, String>> = OnceLock::new();

fn get_mime_map() -> &'static HashMap<String, String> {
    S_MIME_MAP.get_or_init(|| {
        let mut map = HashMap::new();
        // Java: static
        // Java: sMimeMap.put("docm", "application/vnd.ms-word.document.macroEnabled.12");
        map.insert("docm".to_string(), "application/vnd.ms-word.document.macroEnabled.12".to_string());
        // Java: sMimeMap.put("dotm", "application/vnd.ms-word.template.macroEnabled.12");
        map.insert("dotm".to_string(), "application/vnd.ms-word.template.macroEnabled.12".to_string());
        // Java: sMimeMap.put("xlsm", "application/vnd.ms-excel.sheet.macroEnabled.12");
        map.insert("xlsm".to_string(), "application/vnd.ms-excel.sheet.macroEnabled.12".to_string());
        // Java: sMimeMap.put("xltm", "application/vnd.ms-excel.template.macroEnabled.12");
        map.insert("xltm".to_string(), "application/vnd.ms-excel.template.macroEnabled.12".to_string());
        // Java: sMimeMap.put("xlam", "application/vnd.ms-excel.addin.macroEnabled.12");
        map.insert("xlam".to_string(), "application/vnd.ms-excel.addin.macroEnabled.12".to_string());
        // Java: sMimeMap.put("xlsb", "application/vnd.ms-excel.sheet.binary.macroEnabled.12");
        map.insert("xlsb".to_string(), "application/vnd.ms-excel.sheet.binary.macroEnabled.12".to_string());
        // Java: sMimeMap.put("ppam", "application/vnd.ms-powerpoint.addin.macroEnabled.12");
        map.insert("ppam".to_string(), "application/vnd.ms-powerpoint.addin.macroEnabled.12".to_string());
        // Java: sMimeMap.put("pptm", "application/vnd.ms-powerpoint.presentation.macroEnabled.12");
        map.insert("pptm".to_string(), "application/vnd.ms-powerpoint.presentation.macroEnabled.12".to_string());
        // Java: sMimeMap.put("potm", "application/vnd.ms-powerpoint.template.macroEnabled.12");
        map.insert("potm".to_string(), "application/vnd.ms-powerpoint.template.macroEnabled.12".to_string());
        // Java: sMimeMap.put("ppsm", "application/vnd.ms-powerpoint.slideshow.macroEnabled.12");
        map.insert("ppsm".to_string(), "application/vnd.ms-powerpoint.slideshow.macroEnabled.12".to_string());
        map
    })
}

// Java: public class MimeTypeEx
pub struct MimeTypeEx;

// Java: public class MimeTypeEx
impl MimeTypeEx {
    // Java: MimeTypeEx()
    pub fn new() -> Self {
        // Java: super();
        MimeTypeEx
    }

    // Java: static String getMimeType(String extension)
    pub fn get_mime_type(extension: &str) -> Option<&'static str> {
        // Java: return (String)sMimeMap.get(extension);
        get_mime_map().get(extension).map(|s| s.as_str())
    }
}
