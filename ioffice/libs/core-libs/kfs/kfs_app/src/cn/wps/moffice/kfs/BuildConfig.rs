// Java field:    public static final boolean DEBUG = false;
pub const DEBUG: bool = false;
// Java field:    public static final boolean RELEASE = true;
pub const RELEASE: bool = true;

// Java: public class BuildConfig 
#[derive(Default)]
pub struct BuildConfig {
}

// Java: public class BuildConfig 
impl BuildConfig {
    // Java: public BuildConfig() 
    pub fn new() -> BuildConfig {
      Self {}
    }

}
