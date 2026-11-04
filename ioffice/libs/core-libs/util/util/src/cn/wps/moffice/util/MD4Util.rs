#![allow(non_snake_case, dead_code, unused_variables, non_upper_case_globals)]

// Java: import cn.wps.moffice.kfs.File
// use kfs_kfs_app::cn::wps::moffice::kfs::mfs::File;
// Java: import cn.wps.moffice.kfs.FileInputStream
// use kfs_kfs_app::cn::wps::moffice::kfs::mfs::FileInputStream;
// Java: import java.io.EOFException
// Java: import java.io.IOException
// Java: import java.io.InputStream

use std::io::{self, Read};

// Java: public class MD4Util
pub struct MD4Util;

// Java: public class MD4Util
impl MD4Util {
    // Java: MD4Util()
    pub fn new() -> Self {
        // Java: super();
        MD4Util
    }

    // Java: static _getMD4: &[u8](_content: &[u8])
    pub fn getMD4_bytes(content: &[i8]) -> Vec<i8> {
        // Java: return getMD4(content, 0, content.length);
        Self::getMD4_bytes_range(content, 0, content.len() as i32)
    }

    // Java: static _getMD4: &[u8](_content: &[u8], int offset, int length)
    pub fn getMD4_bytes_range(content: &[i8], offset: i32, length: i32) -> Vec<i8> {
        // Java: MD4 md = new MD4();
        // Java: md.update(content, offset, length);
        // Java: return md.digest();
        todo!()
    }

    // Java: static _getMD4: &[u8](InputStream input)
    pub fn getMD4_stream(input: &mut dyn Read) -> io::Result<Vec<i8>> {
        // Java: _md4: &[u8] = null;
        let mut md4: Option<Vec<i8>> = None;
        // Java: MD4 md = new MD4();
        // Java: _arr: &[u8] = new byte[4096];
        let mut arr = vec![0u8; 4096];
        // Java: int len = -1;
        let mut len: i32;
        // Java: while((len = input.read(arr)) >= 0)
        loop {
            let n = input.read(&mut arr)?;
            len = n as i32;
            if len <= 0 { break; }
            {
                // Java: md.update(arr, 0, len);
            }
        }
        // Java: md4 = md.digest();
        // Java: return md4;
        todo!()
    }

    // Java: static _getMD4: &[u8](InputStream input, int size)
    pub fn getMD4_stream_sized(input: &mut dyn Read, size: i32) -> io::Result<Vec<i8>> {
        // Java: MD4 md4 = new MD4();
        // Java: int nRead;
        let mut n_read: i32;
        // Java: for(_buf: &[u8] = new byte[4096]; size > 0; size -= nRead)
        let mut buf = vec![0u8; 4096];
        while size > 0 {
            // Java: int nToRead = Math.min(buf.length, size);
            let n_to_read = std::cmp::min(buf.len() as i32, size);
            // Java: nRead = input.read(buf, 0, nToRead);
            let n = input.read(&mut buf[..n_to_read as usize])?;
            n_read = n as i32;
            // Java: if (nRead <= 0)
            if n_read <= 0 {
                // Java: throw new EOFException("");
                return Err(io::Error::new(io::ErrorKind::UnexpectedEof, ""));
            }
            // Java: md4.update(buf, 0, nRead);
            size -= n_read;
        }
        // Java: return md4.digest();
        todo!()
    }

    // Java: static _getMD4: &[u8](File file)
    pub fn getMD4_file(filepath: &str) -> io::Result<Vec<i8>> {
        // Java: return getMD4((InputStream)(new FileInputStream(file)));
        let mut f = std::fs::File::open(file_path)?;
        Self::getMD4_stream(&mut f)
    }

    // Java: static _getMD4: &[u8](String filePath)
    pub fn getMD4_filepath(filepath: &str) -> io::Result<Vec<i8>> {
        // Java: return getMD4((InputStream)(new FileInputStream(filePath)));
        let mut f = std::fs::File::open(file_path)?;
        Self::getMD4_stream(&mut f)
    }
}
