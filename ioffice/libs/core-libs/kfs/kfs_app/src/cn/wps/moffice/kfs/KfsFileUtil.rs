#![allow(non_snake_case, dead_code, unused_variables, unused_imports)]

use crate::kfs::mfs::File::File;

// Java: public class KfsFileUtil
#[derive(Default)]
pub struct KfsFileUtil {}

// Java: public class KfsFileUtil
impl KfsFileUtil {
    // Java: public KfsFileUtil()
    pub fn new() -> KfsFileUtil { Self {} }

    // Java: public static File convertOrgToKfsFile(java.io.File orgFile)
    pub fn convertOrgToKfsFile(orgFile: &File) -> File {
        // Java: return null == orgFile ? null : new File(orgFile);
        File::new__file(orgFile)
    }
}
