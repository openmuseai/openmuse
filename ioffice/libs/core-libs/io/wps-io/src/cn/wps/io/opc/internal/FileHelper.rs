// Java: import cn.wps.moffice.kfs.File
use std::path::{Path, MAIN_SEPARATOR};
// Java: import cn.wps.moffice.kfs.FileInputStream
// Java: import cn.wps.moffice.kfs.FileOutputStream
// Java: import java.io.IOException
use std::io;
// Java: import java.nio.channels.FileChannel

// Java: public class FileHelper
pub struct FileHelper {
}

// Java: public class FileHelper
impl FileHelper {
    // Java: FileHelper()
    pub fn new() -> Self {
        // Java: super();
        FileHelper {}
    }

    // Java: static File getDirectory(File f)
    pub fn getDirectory(f: Option<&Path>) -> Option<String> {
        // Java: if (f != null)
        if let Some(f) = f {
            // Java: String path = f.getPath();
            let path = f.to_string_lossy().to_string();
            // Java: int len = path.length();
            let len = path.len();
            // Java: int num2 = len;
            let mut num2 = len as i32;
            // Java: while(true)
            loop {
                // Java: --num2;
                num2 -= 1;
                // Java: if (num2 < 0)
                if num2 < 0 {
                    // Java: break;
                    break;
                }
                // Java: char ch1 = path.charAt(num2);
                let ch1 = path.as_bytes()[num2 as usize] as char;
                // Java: if (ch1 == File.separatorChar)
                if ch1 == MAIN_SEPARATOR {
                    // Java: return new File(path.substring(0, num2));
                    return Some(path[..num2 as usize].to_string());
                }
            }
        }
        // Java: return null;
        None
    }

    // Java: static void copyFile(File in, File out)
    pub fn copyFile(input: &Path, output: &Path) -> io::Result<()> {
        // Java: FileChannel sourceChannel = (new FileInputStream(in)).getChannel();
        // Java: FileChannel destinationChannel = (new FileOutputStream(out)).getChannel();
        // Java: sourceChannel.transferTo(0L, sourceChannel.size(), destinationChannel);
        std::fs::copy(input, output)?;
        // Java: sourceChannel.close();
        // Java: destinationChannel.close();
        Ok(())
    }

    // Java: static String getFilename(File file)
    pub fn getFilename(file: Option<&Path>) -> String {
        // Java: if (file != null)
        if let Some(file) = file {
            // Java: String path = file.getPath();
            let path = file.to_string_lossy().to_string();
            // Java: int len = path.length();
            let len = path.len();
            // Java: int num2 = len;
            let mut num2 = len as i32;
            // Java: while(true)
            loop {
                // Java: --num2;
                num2 -= 1;
                // Java: if (num2 < 0)
                if num2 < 0 {
                    // Java: break;
                    break;
                }
                // Java: char ch1 = path.charAt(num2);
                let ch1 = path.as_bytes()[num2 as usize] as char;
                // Java: if (ch1 == File.separatorChar)
                if ch1 == MAIN_SEPARATOR {
                    // Java: return path.substring(num2 + 1, len);
                    return path[(num2 as usize + 1)..len].to_string();
                }
            }
        }
        // Java: return "";
        String::new()
    }
}
