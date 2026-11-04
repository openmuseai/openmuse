#![allow(non_snake_case, dead_code, unused_variables, non_upper_case_globals)]

// Java: import cn.wps.moffice.kfs.File
// use kfs_kfs_app::cn::wps::moffice::kfs::mfs::File;
// Java: import cn.wps.moffice.kfs.FileInputStream
// use kfs_kfs_app::cn::wps::moffice::kfs::mfs::FileInputStream;
// Java: import java.io.IOException
// Java: import java.io.InputStream
// Java: import java.security.MessageDigest

use std::io::{self, Read};

// Java field: _hexDigits: &[char]
const HEX_DIGITS: &[char] = &['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f'];

// Java: public class MD5Util
pub struct MD5Util;

// Java: public class MD5Util
impl MD5Util {
    // Java: MD5Util()
    pub fn new() -> Self {
        // Java: super();
        Self
    }

    // Java: static String getMD5(String source)
    pub fn getMD5_from_string(source: Option<&str>) -> Option<String> {
        // Java: return source == null ? null : getMD5(source.getBytes());
        source.map(|s| Self::getMD5_from_bytes(s.as_bytes()))
    }

    // Java: static String getMD5(_source: &[u8])
    pub fn getMD5_from_bytes(source: &[u8]) -> String {
        // Java: String s = null;
        let mut s: Option<String> = None;
        // Java: try
        {
            // Java: MD5 md = new MD5();
            // Java: md.update(source);
            // Java: _tmp: &[u8] = new byte[16];
            // Java: md.md5final(tmp);
            let tmp: [u8; 16] = todo!();
            // Java: _str: &[char] = new char[32];
            let mut str_chars = vec!['\0'; 32];
            // Java: int k = 0;
            let mut k = 0usize;
            // Java: for(int i = 0; i < 16; ++i)
            for i in 0..16
            {
                // Java: byte byte0 = tmp[i];
                let byte0 = tmp[i] as i32;
                // Java: str[k++] = hexDigits[byte0 >>> 4 & 15];
                str_chars[k] = HEX_DIGITS[((byte0 >> 4) & 15) as usize];
                k += 1;
                // Java: str[k++] = hexDigits[byte0 & 15];
                str_chars[k] = HEX_DIGITS[(byte0 & 15) as usize];
                k += 1;
            }
            // Java: s = new String(str);
            s = Some(str_chars.iter().collect());
        }
        // Java: catch (Exception e)
        {
            // Java: e.printStackTrace();
        }
        // Java: return s;
        s.unwrap_or_default()
    }

    // Java: static String getMD5(InputStream in)
    pub fn getMD5_from_stream(input: &mut dyn Read) -> Option<String> {
        // Java: try
        {
            // Java: MD5 md = new MD5();
            // Java: _buffer: &[u8] = new byte[8192];
            let mut buffer = [0u8; 8192];
            // Java: int len;
            // Java: while((len = in.read(buffer)) != -1)
            loop {
                let len = match input.read(&mut buffer) {
                    Ok(0) => break,
                    Ok(n) => n,
                    Err(_) => break,
                };
                {
                    // Java: md.update(buffer, len);
                }
            }
            // Java: _tmp: &[u8] = new byte[16];
            // Java: md.md5final(tmp);
            let tmp: [u8; 16] = todo!();
            // Java: _str: &[char] = new char[32];
            let mut str_chars = vec!['\0'; 32];
            // Java: int k = 0;
            let mut k = 0usize;
            // Java: for(int i = 0; i < 16; ++i)
            for i in 0..16
            {
                // Java: byte byte0 = tmp[i];
                let byte0 = tmp[i] as i32;
                // Java: str[k++] = hexDigits[byte0 >>> 4 & 15];
                str_chars[k] = HEX_DIGITS[((byte0 >> 4) & 15) as usize];
                k += 1;
                // Java: str[k++] = hexDigits[byte0 & 15];
                str_chars[k] = HEX_DIGITS[(byte0 & 15) as usize];
                k += 1;
            }
            // Java: return new String(str);
            return Some(str_chars.iter().collect());
        }
        // Java: catch (IOException e)
        {
            // Java: e.printStackTrace();
            // Java: return null;
            None
        }
    }

    // Java: static String getMD5(File file)
    pub fn getMD5_from_file(file_path: &str) -> Option<String> {
        // Java: String s = null;
        let mut s: Option<String> = None;
        // Java: if (file != null && file.isFile())
        let path = std::path::Path::new(file_path);
        if path.is_file()
        {
            // Java: try
            if let Ok(mut fin) = std::fs::File::open(file_path) {
                // Java: FileInputStream fin = new FileInputStream(file);
                // Java: s = getMD5(fin, file.getName());
                let file_name = path.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default();
                s = Self::getMD5_from_stream_with_name(&mut fin, &file_name);
                // Java: fin.close();
                drop(fin);
            }
            // Java: catch (Exception var3)
            {
            }
            // Java: return s;
            return s;
        }
        // Java: else
        {
            // Java: return s;
            s
        }
    }

    // Java: static String getMD5(InputStream fin, String fileName)
    pub fn getMD5_from_stream_with_name(fin: &mut dyn Read, fileName: &str) -> Option<String> {
        // Java: if (fin == null)
        // Java: return "";
        // Java: else
        {
            // Java: try
            {
                // Java: MD5 md = new MD5();
                // Java: md.update(fileName.getBytes());
                // Java: _arr: &[u8] = new byte[1024];
                let mut arr = [0u8; 1024];
                // Java: int len = -1;
                // Java: while((len = fin.read(arr)) >= 0)
                loop {
                    let len = match fin.read(&mut arr) {
                        Ok(0) => break,
                        Ok(n) => n,
                        Err(_) => break,
                    };
                    {
                        // Java: md.update(arr, 0, len);
                    }
                }
                // Java: _tmp: &[u8] = new byte[16];
                // Java: md.md5final(tmp);
                let tmp: [u8; 16] = todo!();
                // Java: _str: &[char] = new char[32];
                let mut str_chars = vec!['\0'; 32];
                // Java: int k = 0;
                let mut k = 0usize;
                // Java: for(int i = 0; i < 16; ++i)
                for i in 0..16
                {
                    // Java: byte byte0 = tmp[i];
                    let byte0 = tmp[i] as i32;
                    // Java: str[k++] = hexDigits[byte0 >>> 4 & 15];
                    str_chars[k] = HEX_DIGITS[((byte0 >> 4) & 15) as usize];
                    k += 1;
                    // Java: str[k++] = hexDigits[byte0 & 15];
                    str_chars[k] = HEX_DIGITS[(byte0 & 15) as usize];
                    k += 1;
                }
                // Java: return new String(str);
                return Some(str_chars.iter().collect());
            }
            // Java: catch (Exception e)
            {
                // Java: e.printStackTrace();
                // Java: return null;
                None
            }
        }
    }

    // Java: static String getMD5(String source, int place)
    pub fn getMD5_from_string_with_place(source: Option<&str>, place: i32) -> Option<String> {
        // Java: return source == null ? null : getMD5(source.getBytes());
        source.map(|s| Self::getMD5_from_bytes(s.as_bytes()))
    }

    // Java: static String getMD5(_source: &[u8], int place)
    pub fn getMD5_from_bytes_with_place(source: &[u8], place: i32) -> String {
        // Java: String s = null;
        let mut s: Option<String> = None;
        // Java: try
        {
            // Java: MessageDigest md = MessageDigest.getInstance("MD5");
            // Java: md.update(source);
            // Java: _tmp: &[u8] = md.digest();
            let tmp: Vec<u8> = todo!();
            // Java: _str: &[char] = new char[place * 2];
            let mut str_chars = vec!['\0'; (place * 2) as usize];
            // Java: int k = 0;
            let mut k = 0usize;
            // Java: for(int i = 0; i < place; ++i)
            for i in 0..(place as usize)
            {
                // Java: byte byte0 = tmp[i];
                let byte0 = tmp[i] as i32;
                // Java: str[k++] = hexDigits[byte0 >>> 4 & 15];
                str_chars[k] = HEX_DIGITS[((byte0 >> 4) & 15) as usize];
                k += 1;
                // Java: str[k++] = hexDigits[byte0 & 15];
                str_chars[k] = HEX_DIGITS[(byte0 & 15) as usize];
                k += 1;
            }
            // Java: s = new String(str);
            s = Some(str_chars.iter().collect());
        }
        // Java: catch (Exception e)
        {
            // Java: e.printStackTrace();
        }
        // Java: return s;
        s.unwrap_or_default()
    }

    // Java: static _getMD5Bytes: &[u8](String msg)
    pub fn getMD5Bytes(msg: &str) -> Option<Vec<u8>> {
        // Java: try
        {
            // Java: MessageDigest md = MessageDigest.getInstance("MD5");
            // Java: return md.digest(msg.getBytes());
            todo!()
        }
        // Java: catch (Exception ex)
        {
            // Java: ex.printStackTrace();
            // Java: return null;
            None
        }
    }

    // Java: static void main(_args: &[String])
    pub fn main_test() {
        // Java: String md5 = getMD5("/mnt/sdcard/KingsoftOffice/file/说明.doc");
        let md5 = Self::getMD5_from_string(Some("/mnt/sdcard/KingsoftOffice/file/说明.doc"));
        // Java: System.out.println(md5);
        println!("{:?}", md5);
    }
}
