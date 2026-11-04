#![allow(non_snake_case, dead_code, unused_variables, unused_imports)]

// Java: import cn.wps.moffice.kfs.FileOutputStream
use crate::kfs::FileOutputStream::FileOutputStream;
// Java: import cn.wps.moffice.kfs.KfsContext
use crate::kfs::KfsContext::KfsContext;
// Java: import cn.wps.moffice.kfs.RandomAccessFile
use crate::kfs::RandomAccessFile::RandomAccessFile;
// Java: import cn.wps.moffice.kfs.log.Log
use crate::kfs::log::Log::Log;

use super::KFSFileLogAdapter::KFSFileLogAdapter;

// Java field:    static final String TAG = "FileCloseUtil";
const TAG: &str = "FileCloseUtil";

// Java: public class FileCloseUtil
#[derive(Default)]
pub struct FileCloseUtil {}

// Java: public class FileCloseUtil
impl FileCloseUtil {
    // Java: public FileCloseUtil()
    pub fn new() -> Self { Self {} }

    // Java: public static void closeQuietly(RandomAccessFile out)
    pub fn closeQuietly_raf(out: &mut Option<RandomAccessFile>) {
        // Java: if (null != out)
        if let Some(ref mut raf) = out {
            // Java: try { out.close(); }
            raf.close();
        }
    }

    // Java: public static void closeQuietly(FileOutputStream out)
    pub fn closeQuietly_fos(out: &mut Option<FileOutputStream>) {
        // Java: if (null != out)
        if let Some(ref mut fos) = out {
            // Java: try { out.close(); }
            fos.close();
        }
    }
}
