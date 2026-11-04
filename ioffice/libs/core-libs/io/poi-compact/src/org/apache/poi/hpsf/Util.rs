// Java: import java.io.IOException
// Java: import java.io.PrintWriter
// Java: import java.io.StringWriter
// Java: import java.util.Collection
// Java: import java.util.Date

// Java field: long EPOCH_DIFF
pub const EPOCH_DIFF: i64 = 11644473600000i64;

// Java: public class Util
pub struct Util;

// Java: public class Util
impl Util {
    // Java: Util()
    pub fn new() -> Self {
        // Java: super();
        Util
    }

    // Java: boolean equal(byte[] a, byte[] b)
    pub fn equal(a: &[u8], b: &[u8]) -> bool {
        // Java: if (a.length != b.length)
        if a.len() != b.len()
        {
            // Java: return false;
            return false;
        }
        // Java: else
        else
        {
            // Java: for(int i = 0; i < a.length; ++i)
            for i in 0..a.len()
            {
                // Java: if (a[i] != b[i])
                if a[i] != b[i]
                {
                    // Java: return false;
                    return false;
                }
            }
            // Java: return true;
            return true;
        }
    }

    // Java: void copy(byte[] src, int srcOffset, int length, byte[] dst, int dstOffset)
    pub fn copy_to(src: &[u8], src_offset: i32, length: i32, dst: &mut [u8], dst_offset: i32) {
        // Java: for(int i = 0; i < length; ++i)
        for i in 0..length as usize
        {
            // Java: dst[dstOffset + i] = src[srcOffset + i];
            dst[dst_offset as usize + i] = src[src_offset as usize + i];
        }
    }

    // Java: byte[] cat(byte[][] byteArrays)
    pub fn cat(byte_arrays: &[&[u8]]) -> Vec<u8> {
        // Java: int capacity = 0;
        let mut capacity: usize = 0;
        // Java: for(int i = 0; i < byteArrays.length; ++i)
        for i in 0..byte_arrays.len()
        {
            // Java: capacity += byteArrays[i].length;
            capacity += byte_arrays[i].len();
        }
        // Java: byte[] result = new byte[capacity];
        let mut result = Vec::with_capacity(capacity);
        // Java: int r = 0;
        let mut _r: usize = 0;
        // Java: for(int i = 0; i < byteArrays.length; ++i)
        for i in 0..byte_arrays.len()
        {
            // Java: for(int j = 0; j < byteArrays[i].length; ++j)
            for j in 0..byte_arrays[i].len()
            {
                // Java: result[r++] = byteArrays[i][j];
                result.push(byte_arrays[i][j]);
            }
        }
        // Java: return result;
        result
    }

    // Java: byte[] copy(byte[] src, int offset, int length)
    pub fn copy(src: &[u8], offset: i32, length: i32) -> Vec<u8> {
        // Java: byte[] result = new byte[length];
        let mut result = vec![0u8; length as usize];
        // Java: copy(src, offset, length, result, 0);
        Self::copy_to(src, offset, length, &mut result, 0);
        // Java: return result;
        result
    }

    // Java: Date filetimeToDate(int high, int low)
    pub fn filetimeToDate_ints(high: i32, low: i32) -> i64 {
        // Java: long filetime = (long)high << 32 | (long)low & 4294967295L;
        let filetime: i64 = ((high as i64) << 32) | (low as i64 & 0xFFFFFFFF);
        // Java: return filetimeToDate(filetime);
        Self::filetimeToDate(filetime)
    }

    // Java: Date filetimeToDate(long filetime)
    pub fn filetimeToDate(filetime: i64) -> i64 {
        // Java: long ms_since_16010101 = filetime / 10000L;
        let ms_since_16010101 = filetime / 10000i64;
        // Java: long ms_since_19700101 = ms_since_16010101 - 11644473600000L;
        let ms_since_19700101 = ms_since_16010101 - 11644473600000i64;
        // Java: return new Date(ms_since_19700101);
        ms_since_19700101
    }

    // Java: long dateToFileTime(Date date)
    pub fn dateToFileTime(date_ms: i64) -> i64 {
        // Java: long ms_since_19700101 = date.getTime();
        let ms_since_19700101 = date_ms;
        // Java: long ms_since_16010101 = ms_since_19700101 + 11644473600000L;
        let ms_since_16010101 = ms_since_19700101 + 11644473600000i64;
        // Java: return ms_since_16010101 * 10000L;
        ms_since_16010101 * 10000i64
    }

    // Java: boolean equals(Collection<?> c1, Collection<?> c2)
    pub fn equals_collections(c1: &[String], c2: &[String]) -> bool {
        // Java: Object[] o1 = c1.toArray();
        let o1 = c1;
        // Java: Object[] o2 = c2.toArray();
        let o2 = c2;
        // Java: return internalEquals(o1, o2);
        Self::internalEquals(o1, o2)
    }

    // Java: boolean equals(Object[] c1, Object[] c2)
    pub fn equals_arrays(c1: &[String], c2: &[String]) -> bool {
        // Java: Object[] o1 = c1.clone();
        let o1 = c1.to_vec();
        // Java: Object[] o2 = c2.clone();
        let o2 = c2.to_vec();
        // Java: return internalEquals(o1, o2);
        Self::internalEquals(&o1, &o2)
    }

    // Java: boolean internalEquals(Object[] o1, Object[] o2)
    fn internalEquals(o1: &[String], o2: &[String]) -> bool {
        let mut o2_copy: Vec<Option<&str>> = o2.iter().map(|s| Some(s.as_str())).collect();
        // Java: for(int i1 = 0; i1 < o1.length; ++i1)
        for i1 in 0..o1.len()
        {
            // Java: Object obj1 = o1[i1];
            let obj1 = &o1[i1];
            // Java: boolean matchFound = false;
            let mut match_found = false;
            // Java: for(int i2 = 0; !matchFound && i2 < o1.length; ++i2)
            for i2 in 0..o2_copy.len()
            {
                if match_found { break; }
                // Java: Object obj2 = o2[i2];
                let _obj2 = o2_copy[i2];
                // Java: if (obj1.equals(obj2))
                if let Some(obj2) = o2_copy[i2] {
                    if obj1.as_str() == obj2 {
                        // Java: matchFound = true;
                        match_found = true;
                        // Java: o2[i2] = null;
                        o2_copy[i2] = None;
                    }
                }
            }
            // Java: if (!matchFound)
            if !match_found
            {
                // Java: return false;
                return false;
            }
        }
        // Java: return true;
        true
    }

    // Java: byte[] pad4(byte[] ba)
    pub fn pad4_bytes(ba: &[u8]) -> Vec<u8> {
        // Java: int PAD = 4;
        let _pad_const = 4;
        // Java: int l = ba.length % 4;
        let l = ba.len() % 4;
        // Java: byte[] result;
        let result: Vec<u8>;
        // Java: if (l == 0)
        if l == 0
        {
            // Java: result = ba;
            result = ba.to_vec();
        }
        // Java: else
        else
        {
            // Java: l = 4 - l;
            let pad = 4 - l;
            // Java: result = new byte[ba.length + l];
            let mut padded = vec![0u8; ba.len() + pad];
            // Java: System.arraycopy(ba, 0, result, 0, ba.length);
            padded[..ba.len()].copy_from_slice(ba);
            // Java: return result;
            result = padded;
        }
        result
    }

    // Java: char[] pad4(char[] ca)
    pub fn pad4_chars(ca: &[u16]) -> Vec<u16> {
        // Java: int PAD = 4;
        let _pad_const = 4;
        // Java: int l = ca.length % 4;
        let l = ca.len() % 4;
        // Java: char[] result;
        let result: Vec<u16>;
        // Java: if (l == 0)
        if l == 0
        {
            // Java: result = ca;
            result = ca.to_vec();
        }
        // Java: else
        else
        {
            // Java: l = 4 - l;
            let pad = 4 - l;
            // Java: result = new char[ca.length + l];
            let mut padded = vec![0u16; ca.len() + pad];
            // Java: System.arraycopy(ca, 0, result, 0, ca.length);
            padded[..ca.len()].copy_from_slice(ca);
            // Java: return result;
            result = padded;
        }
        result
    }

    // Java: char[] pad4(String s)
    pub fn pad4_string(s: &str) -> Vec<u16> {
        // Java: return pad4(s.toCharArray());
        let chars: Vec<u16> = s.encode_utf16().collect();
        Self::pad4_chars(&chars)
    }

    // Java: String toString(Throwable t)
    pub fn toString_error(t: &dyn std::error::Error) -> String {
        // Java: StringWriter sw = new StringWriter();
        let mut sw = String::new();
        // Java: PrintWriter pw = new PrintWriter(sw);
        let pw = &mut sw;
        // Java: t.printStackTrace(pw);
        pw.push_str(&format!("{}", t));
        // Java: pw.close();
        drop(pw);
        // Java: try
        let result: Result<String, std::io::Error> = (|| {
            // Java: sw.close();
            let output = sw.clone();
            // Java: return sw.toString();
            Ok(output)
        })();
        // Java: catch (IOException e)
        match result {
            Ok(s) => s,
            Err(e) => {
                // Java: StringBuffer b = new StringBuffer(t.getMessage());
                let mut b = String::from(t.to_string());
                // Java: b.append("\n");
                b.push('\n');
                // Java: b.append("Could not create a stacktrace. Reason: ");
                b.push_str("Could not create a stacktrace. Reason: ");
                // Java: b.append(e.getMessage());
                b.push_str(&e.to_string());
                // Java: return b.toString();
                b
            }
        }
    }
}
