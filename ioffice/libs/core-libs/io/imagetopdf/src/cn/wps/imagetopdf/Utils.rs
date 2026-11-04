// Java: import cn.wps.imagetopdf.obj.DictObj
use crate::cn::wps::imagetopdf::obj::DictObj::DictObj;
// Java: import java.io.IOException
use std::io::Read;
// Java: import java.io.InputStream
// Java: import java.util.TreeMap
use std::collections::BTreeMap;

use crate::cn::wps::imagetopdf::OutputStreamCounter::OutputStreamCounter;

// Java: public class Utils
pub struct Utils {
}

// Java: public class Utils
impl Utils {
    // Java: Utils()
    pub fn new() -> Self {
        // Java: super();
        Utils {}
    }

    // Java: static int getInt(InputStream is)
    pub fn getInt(is: &mut dyn Read) -> i32 {
        // Java: return (is.read() << 24) + (is.read() << 16) + (is.read() << 8) + is.read();
        let mut buf = [0u8; 1];
        let b0 = if is.read(&mut buf).unwrap_or(0) > 0 { buf[0] as i32 } else { 0 };
        let b1 = if is.read(&mut buf).unwrap_or(0) > 0 { buf[0] as i32 } else { 0 };
        let b2 = if is.read(&mut buf).unwrap_or(0) > 0 { buf[0] as i32 } else { 0 };
        let b3 = if is.read(&mut buf).unwrap_or(0) > 0 { buf[0] as i32 } else { 0 };
        (b0 << 24) + (b1 << 16) + (b2 << 8) + b3
    }

    // Java: static String getString(InputStream is)
    pub fn getString(is: &mut dyn Read) -> String {
        // Java: StringBuffer buf = new StringBuffer();
        let mut buf = String::new();
        // Java: for(int i = 0; i < 4; ++i)
        let mut byte_buf = [0u8; 1];
        for _i in 0..4 {
            // Java: buf.append((char)is.read());
            if is.read(&mut byte_buf).unwrap_or(0) > 0 {
                buf.push(byte_buf[0] as char);
            }
        }
        // Java: return buf.toString();
        buf
    }

    // Java: static boolean checkMarker(String s)
    pub fn checkMarker(s: &str) -> bool {
        // Java: if (s.length() != 4)
        if s.len() != 4 {
            // Java: return false;
            return false;
        // Java: else
        } else {
            // Java: for(int k = 0; k < 4; ++k)
            for k in 0..4 {
                // Java: char c = s.charAt(k);
                let c = s.as_bytes()[k] as char;
                // Java: if ((c < 'a' || c > 'z') && (c < 'A' || c > 'Z'))
                if (c < 'a' || c > 'z') && (c < 'A' || c > 'Z') {
                    // Java: return false;
                    return false;
                }
            }
            // Java: return true;
            true
        }
    }

    // Java: static int getWord(InputStream is)
    pub fn getWord(is: &mut dyn Read) -> i32 {
        // Java: return (is.read() << 8) + is.read();
        let mut buf = [0u8; 1];
        let b0 = if is.read(&mut buf).unwrap_or(0) > 0 { buf[0] as i32 } else { 0 };
        let b1 = if is.read(&mut buf).unwrap_or(0) > 0 { buf[0] as i32 } else { 0 };
        (b0 << 8) + b1
    }

    // Java: static void skip(InputStream is, int size)
    pub fn skip(is: &mut dyn Read, size: i32) {
        // Java: while(true)
        let mut size = size;
        loop {
            // Java: if (size > 0)
            if size > 0 {
                // Java: long n = is.skip((long)size);
                let mut discard = vec![0u8; size as usize];
                let n = is.read(&mut discard).unwrap_or(0) as i64;
                // Java: if (n > 0L)
                if n > 0 {
                    // Java: size = (int)((long)size - n);
                    size = (size as i64 - n) as i32;
                    // Java: continue;
                    continue;
                }
            }
            // Java: return;
            return;
        }
    }

    // Java: static long writeBytes(OutputStreamCounter out, TreeMap<Integer, Long> offsets, DictObj obj)
    pub fn writeBytes_obj(out: &mut OutputStreamCounter, offsets: &mut BTreeMap<i32, i64>, obj: &DictObj) -> i64 {
        // Java: return writeBytes(out, offsets, obj.getObjNum(), obj.getBytes());
        Self::writeBytes(out, offsets, obj.getObjNum(), &obj.getBytes())
    }

    // Java: static long writeBytes(OutputStreamCounter out, TreeMap<Integer, Long> offsets, byte[] bytes)
    pub fn writeBytes_bytes(out: &mut OutputStreamCounter, offsets: &mut BTreeMap<i32, i64>, bytes: &[u8]) -> i64 {
        // Java: return writeBytes(out, offsets, offsets.size(), bytes);
        let index = offsets.len() as i32;
        Self::writeBytes(out, offsets, index, bytes)
    }

    // Java: static long writeBytes(OutputStreamCounter out, TreeMap<Integer, Long> offsets, int index, byte[] bytes)
    pub fn writeBytes(out: &mut OutputStreamCounter, offsets: &mut BTreeMap<i32, i64>, index: i32, bytes: &[u8]) -> i64 {
        // Java: long offset = out.getPosition();
        let offset = out.getPosition();
        // Java: offsets.put(index, offset);
        offsets.insert(index, offset);
        // Java: try
        // Java: out.write(bytes);
        let _ = out.write_buf(bytes);
        // Java: catch (IOException e)
        // Java: e.printStackTrace();
        // Java: return offset;
        offset
    }
}
