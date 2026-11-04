// Java: public class BuildConfig
pub struct BuildConfig {
    // Java field: boolean DEBUG
    pub(crate) DEBUG: bool,
    // Java field: String LIBRARY_PACKAGE_NAME
    pub(crate) LIBRARY_PACKAGE_NAME: &'static str,
    // Java field: String BUILD_TYPE
    pub(crate) BUILD_TYPE: &'static str,
}

// Java: public class BuildConfig
impl BuildConfig {
    // Java: BuildConfig()
    pub fn new() -> Self {
        // Java: super();
        BuildConfig {
            DEBUG: false,
            LIBRARY_PACKAGE_NAME: "cn.wps.imagetopdf",
            BUILD_TYPE: "release",
        }
    }

    pub const DEBUG: bool = false;
    pub const LIBRARY_PACKAGE_NAME: &'static str = "cn.wps.imagetopdf";
    pub const BUILD_TYPE: &'static str = "release";
}
