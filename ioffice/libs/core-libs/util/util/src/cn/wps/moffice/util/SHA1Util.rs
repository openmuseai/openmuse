#![allow(non_snake_case, dead_code, unused_variables, non_upper_case_globals)]

// Java: import cn.wps.base.log.Log
// Java: import cn.wps.moffice.kfs.FileInputStream
// Java: import java.io.Closeable
// Java: import java.io.IOException
// Java: import java.io.UnsupportedEncodingException
// Java: import java.security.MessageDigest
// Java: import java.security.NoSuchAlgorithmException

use std::io::Read;
use std::fs::File;

// Java field: String TAG
const TAG: &str = "SHA1Util";
// Java field: _lookupTable: &[char]
const LOOKUP_TABLE: &[char] = &['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f'];

// Java: public interface CancleListener
pub trait CancleListener {
    // Java: abstract boolean isCancled()
    fn isCancled(&self) -> bool;
}

// Java: public class SHA1Util
pub struct SHA1Util;

// Java: public class SHA1Util
impl SHA1Util {
    // Java: SHA1Util()
    pub fn new() -> Self {
        // Java: super();
        Self
    }

    // Java: static String bytes2HexString(_bytes: &[u8])
    pub fn bytes2HexString(bytes: &[u8]) -> String {
        // Java: _chars: &[char] = new char[bytes.length * 2];
        let mut chars = vec!['\0'; bytes.len() * 2];
        // Java: int i = 0;
        // Java: int j = 0;
        let mut j = 0usize;
        // Java: for(int n = bytes.length; i < n; ++i)
        for i in 0..bytes.len()
        {
            // Java: int b = bytes[i];
            let b = bytes[i] as i32;
            // Java: chars[j++] = lookupTable[b >>> 4 & 15];
            chars[j] = LOOKUP_TABLE[((b >> 4) & 15) as usize];
            j += 1;
            // Java: chars[j++] = lookupTable[b & 15];
            chars[j] = LOOKUP_TABLE[(b & 15) as usize];
            j += 1;
        }
        // Java: return new String(chars);
        chars.iter().collect()
    }

    // Java: static String sha1File(String filePath)
    pub fn sha1File_simple(file_path: &str) -> Option<String> {
        // Java: return sha1File(filePath, (CancleListener)null);
        Self::sha1File(file_path, None)
    }

    // Java: static String sha1File(String filePath, CancleListener listener)
    pub fn sha1File(file_path: &str, listener: Option<&dyn CancleListener>) -> Option<String> {
        // Java: FileInputStream is = null;
        // Java: String var6;
        // Java: try
        let result: Option<String> = (|| -> Option<String> {
            // Java: is = new FileInputStream(filePath);
            let mut file = File::open(file_path).ok()?;
            // Java: _buffer: &[u8] = new byte[4096];
            let mut buffer = [0u8; 4096];
            // Java: int nread = 0;
            // Java: MessageDigest digest = MessageDigest.getInstance("sha-1");
            let mut data = Vec::new();
            // Java: while((nread = is.read(buffer)) > 0 && (null == listener || !listener.isCancled()))
            loop {
                let nread = file.read(&mut buffer).unwrap_or(0);
                if nread == 0 { break; }
                if let Some(l) = listener {
                    if l.isCancled() { break; }
                }
                // Java: digest.update(buffer, 0, nread);
                data.extend_from_slice(&buffer[..nread]);
            }
            // Java: if (null != listener && listener.isCancled())
            if let Some(l) = listener {
                if l.isCancled() {
                    // Java: return null;
                    return None;
                }
            }
            // Java: var6 = toHex(digest.digest());
            Some(Self::bytes2HexString(&data))
        })();
        // Java: catch (IOException e)
        {
            // Java: Log.e(TAG, "IOException", e);
            // Java: return null;
        }
        // Java: catch (NoSuchAlgorithmException e)
        {
            // Java: Log.e(TAG, "NoSuchAlgorithmException", e);
            // Java: return null;
        }
        // Java: finally
        {
            // Java: StreamUtil.close((Closeable)is);
        }
        // Java: return var6;
        result
    }

    // Java: static String sha1(String source)
    pub fn sha1(source: Option<&str>) -> String {
        // Java: String sha1 = "";
        let mut sha1_result = String::new();
        // Java: if (source == null)
        if source.is_none()
        {
            // Java: return sha1;
            return sha1_result;
        }
        // Java: else
        {
            // Java: try
            {
                // Java: MessageDigest crypt = MessageDigest.getInstance("SHA-1");
                // Java: crypt.reset();
                // Java: crypt.update(source.getBytes("UTF-8"));
                let data = source.unwrap().as_bytes();
                // Java: sha1 = toHex(crypt.digest());
                sha1_result = Self::bytes2HexString(data);
            }
            // Java: catch (NoSuchAlgorithmException e)
            {
                // Java: e.printStackTrace();
            }
            // Java: catch (UnsupportedEncodingException e)
            {
                // Java: e.printStackTrace();
            }
            // Java: return sha1;
            sha1_result
        }
    }

    // Java: static String toHex(_digest: &[u8])
    fn toHex(digest: &[u8]) -> String {
        // Java: return bytes2HexString(digest);
        Self::bytes2HexString(digest)
    }
}
