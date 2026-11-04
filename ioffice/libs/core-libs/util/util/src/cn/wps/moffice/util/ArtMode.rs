// Java: public class ArtMode
pub struct ArtMode {
    // Java field: String TAG
    // Java field: boolean IS_VM_MULTIDEX_CAPABLE
    // Java field: String TEST_CLASS_NAME_WR
    // Java field: String TEST_CLASS_NAME_SS
    // Java field: String TEST_CLASS_NAME_PR
    // Java field: String TEST_CLASS_NAME_PDF
    // Java field: String TEST_CLASS_NAME_GCM
    // Java field: String TEST_CLASS_NAME_EXTLIBS
}

// Java: public class ArtMode
impl ArtMode {
    pub const IS_VM_MULTIDEX_CAPABLE: bool = false;

    // Java: ArtMode()
    pub fn new() -> Self {
        // Java: super();
        Self {}
    }

    // Java: static boolean doLoadArtModeTestClass(String className)
    fn doLoadArtModeTestClass(_class_name: &str) -> bool {
        // Java: try
        // Java: Class.forName(className);
        // Java: return true;
        // Java: catch (ClassNotFoundException var2)
        // Java: catch (Throwable var3)
        // Java: return false;
        false
    }

    // Java: static boolean isVMMultidexCapable()
    fn isVMMultidexCapable() -> bool {
        // Java: boolean ret = false;
        let mut ret = false;
        // Java: ret = ret || doLoadArtModeTestClass("cn.wps.moffice.writer.ArtModeTestClass");
        ret = ret || Self::doLoadArtModeTestClass("cn.wps.moffice.writer.ArtModeTestClass");
        // Java: ret = ret || doLoadArtModeTestClass("cn.wps.moffice.spreadsheet.ArtModeTestClass");
        ret = ret || Self::doLoadArtModeTestClass("cn.wps.moffice.spreadsheet.ArtModeTestClass");
        // Java: ret = ret || doLoadArtModeTestClass("cn.wps.moffice.presentation.ArtModeTestClass");
        ret = ret || Self::doLoadArtModeTestClass("cn.wps.moffice.presentation.ArtModeTestClass");
        // Java: ret = ret || doLoadArtModeTestClass("cn.wps.moffice.pdf.ArtModeTestClass");
        ret = ret || Self::doLoadArtModeTestClass("cn.wps.moffice.pdf.ArtModeTestClass");
        // Java: ret = ret || doLoadArtModeTestClass("cn.wps.moffice.main.gcm.ArtModeTestClass");
        ret = ret || Self::doLoadArtModeTestClass("cn.wps.moffice.main.gcm.ArtModeTestClass");
        // Java: ret = ret || doLoadArtModeTestClass("cn.wps.moffice.extlibs.ArtModeTestClass");
        ret = ret || Self::doLoadArtModeTestClass("cn.wps.moffice.extlibs.ArtModeTestClass");
        // Java: return ret;
        ret
    }
}
