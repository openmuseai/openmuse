#![allow(non_snake_case, dead_code, unused_variables, unused_imports)]

use super::File::File;
use super::FileInputStream::FileInputStream;

// Java: public class CfsUtil
#[derive(Default)]
pub struct CfsUtil {}

// Java: public class CfsUtil
impl CfsUtil {
    pub fn new() -> CfsUtil { Self {} }

    // Java: public static boolean export(File file, String desPath)
    pub fn export(file: &File, desPath: &str) -> bool {
        // Java: if (null != file && file.exists() && null != desPath)
        if !file.exists() || desPath.is_empty() {
            return false;
        }
        // Java: java.io.FileOutputStream fos = null;
        let fos = std::fs::File::create(desPath);
        let mut fos = match fos {
            Ok(f) => f,
            Err(_) => return false,
        };
        // Java: FileInputStream fis = new FileInputStream(file);
        let mut fis = FileInputStream::new(file);
        // Java: byte[] buff = new byte[4096];
        let mut buff = vec![0u8; 4096];
        // Java: while(true) { int n = fis.read(buff, 0, buff.length); if (n <= 0) return true; fos.write(buff, 0, n); }
        loop {
            let n = fis.read__bytearr_int_int(&mut buff, 0, 4096);
            if n <= 0 { return true; }
            use std::io::Write;
            if fos.write_all(&buff[..n as usize]).is_err() {
                return false;
            }
        }
    }
}
