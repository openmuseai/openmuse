// Java: import cn.wps.core.runtime.Platform
use crate::cn::wps::core::runtime::Platform;

// Java field: String DRAG_FOLDER
static DRAG_FOLDER: std::sync::OnceLock<std::sync::Mutex<String>> = std::sync::OnceLock::new();
// Java field: String SIGNATURE_DRAG
pub(crate) const SIGNATURE_DRAG: &str = "Drag Data of Android WPS";
// Java field: String HTML
static DRAG_PATH_HTML: std::sync::OnceLock<std::sync::Mutex<String>> = std::sync::OnceLock::new();

// Java: public class DRAG
pub struct DRAG {
}

// Java: public class Path
pub struct DRAGPath {
}

// Java: public class DRAG
impl DRAG {
    // Java: DRAG()
    pub fn new() -> Self {
        // Java: super();
        DRAG {}
    }

    // Java: static void initPath()
    pub fn initPath() {
        // Java: DRAG_FOLDER = Platform.getDragDirectory();
        // Java: DRAG.Path.HTML = DRAG_FOLDER + "drag.html";
        todo!()
    }

    // Java: static String getUri()
    pub fn getUri() -> String {
        // Java: String authority = CLIP.getAuthority();
        let authority = super::CLIP::CLIP::getAuthority();
        // Java: return "content://" + authority + "/drag";
        format!("content://{}/drag", authority)
    }

}

// Java: public class Path
impl DRAGPath {
    // Java: Path()
    pub fn new() -> Self {
        // Java: super();
        DRAGPath {}
    }

}
