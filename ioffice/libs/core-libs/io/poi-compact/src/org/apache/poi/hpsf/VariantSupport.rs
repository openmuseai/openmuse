// Java: import java.io.IOException
use stubs::java::io::IOException::*;
// Java: import java.io.OutputStream
use stubs::java::io::OutputStream::*;
// Java: import java.io.UnsupportedEncodingException
use stubs::java::io::UnsupportedEncodingException::*;
// Java: import java.nio.charset.StandardCharsets
use stubs::java::nio::charset::StandardCharsets::*;
// Java: import java.util.Date
use stubs::java::util::Date::*;
// Java: import java.util.LinkedList
use stubs::java::util::LinkedList::*;
// Java: import java.util.List
use stubs::java::util::List::*;
// Java: import org.apache.poi.util.LittleEndian
use util::poi_util::LittleEndian::*;
// Java: import org.apache.poi.hpsf.Variant
use crate::org::apache::poi::hpsf::Variant::*;
// Java: import org.apache.poi.hpsf.UnsupportedVariantTypeException
use crate::org::apache::poi::hpsf::UnsupportedVariantTypeException::*;
// Java: import org.apache.poi.hpsf.ReadingNotSupportedException
use crate::org::apache::poi::hpsf::ReadingNotSupportedException::*;
// Java: import org.apache.poi.hpsf.WritingNotSupportedException
use crate::org::apache::poi::hpsf::WritingNotSupportedException::*;

// Java field:    private static boolean logUnsupportedTypes = false;
static logUnsupportedTypes: std::sync::atomic::AtomicBool = false;
// Java field:    protected static List unsupportedMessage;
pub static unsupportedMessage: std::sync::OnceLock<List> =
                std::sync::OnceLock::new();
// Java field:    public static final int[] SUPPORTED_TYPES = new int[]{0, 2, 3, 20, 5, 64, 30, 31, 71, 11};
pub static SUPPORTED_TYPES: std::sync::LazyLock<Vec<i32>> =
                    std::sync::LazyLock::new(|| &[0, 2, 3, 20, 5, 64, 30, 31, 71, 11]);

// Java: public class VariantSupport extends Variant 
#[derive(Default)]
pub struct VariantSupport {
    // Java field: Variant super_;
    pub super_: Variant,
}

// Java: public class VariantSupport extends Variant 
impl VariantSupport {
    // Java: public VariantSupport() 
    pub fn new() -> VariantSupport {
      Self { super_: Variant::new() }
    }

    // Java: public static void setLogUnsupportedTypes(boolean logUnsupportedTypes) 
    pub fn setLogUnsupportedTypes(logUnsupportedTypes: bool) {
      // Java: VariantSupport.logUnsupportedTypes = logUnsupportedTypes;
      VariantSupport::logUnsupportedTypes = logUnsupportedTypes;
    }

    // Java: public static boolean isLogUnsupportedTypes() 
    pub fn isLogUnsupportedTypes() -> bool {
      // Java: return logUnsupportedTypes;
      return logUnsupportedTypes;
    }

    // Java: protected static void writeUnsupportedTypeMessage(UnsupportedVariantTypeException ex) 
    pub fn writeUnsupportedTypeMessage(ex: UnsupportedVariantTypeException) {
      // Java: if (isLogUnsupportedTypes())
      if Self::isLogUnsupportedTypes() {
         // Java: if (unsupportedMessage == null)
         if unsupportedMessage.is_none() {
            // Java: unsupportedMessage = new LinkedList();
            unsupportedMessage = VecDeque::new();
         }
         // Java: Long vt = ex.getVariantType();
         let vt: i64 = ex.getVariantType();
         // Java: if (!unsupportedMessage.contains(vt))
         if !unsupportedMessage.contains(vt) {
            // Java: System.err.println(ex.getMessage());
            System::err.println(ex.getMessage());
            // Java: unsupportedMessage.add(vt);
            unsupportedMessage.add(vt);
         }
      }
    }

    // Java: public boolean isSupportedType(int variantType) 
    pub fn isSupportedType(&mut self, variantType: i32) -> bool {
      let i: i32 = 0;
      // Java: for(int i = 0; i < SUPPORTED_TYPES.length; ++i)
      while i < SUPPORTED_TYPES::length {
         // Java: if (variantType == SUPPORTED_TYPES[i])
         if variantType == SUPPORTED_TYPES[i] {
            // Java: return true;
            return true;
         }
         i += 1;
      }
      // Java: return false;
      return false;
    }

    // Java: public static Object read(byte[] src, int offset, int length, long type, int codepage) throws ReadingNotSupportedException, UnsupportedEncodingException 
    pub fn read(src: Vec<u8>, offset: i32, length: i32, r#type: i64, codepage: i32) -> Box<dyn std::any::Any> {
      // Java: Object value = null;
      let value: Box<dyn std::any::Any> = None;
      // Java: int o1 = offset;
      let o1: i32 = offset;
      // Java: int l1 = length - 4;
      let l1: i32 = length - 4;
      // Java: long lType = type;
      let lType: i64 = r#type;
      // Java: if (codepage == 1200 && type == 30L)
      if codepage == 1200 && r#type == 30 {
         // Java: lType = 31L;
         lType = 31;
      }
      // Java: switch ((int)lType)
      match ((lType) as i32) {
         // case 0
         0 => { value = None; }
         // case 2
         2 => { value = Integer::valueOf(LittleEndian::getShort(src, offset)); }
         // case 3
         3 => { value = LittleEndian::getInt(src, offset); }
         // case 5
         5 => { value = Double::new(LittleEndian::getDouble(src, offset)); }
         // case 11
         11 => {
    let bool: i64 = LittleEndian::getUInt(src, offset);
    if bool != 0 {
    value = Boolean::TRUE;
    }
    else {
    value = Boolean::FALSE;
    }
}
         // case 20
         20 => { value = LittleEndian::getLong(src, offset); }
         // case 30
         30 => {
    let first: i32 = offset + 4;
    let last: i64 = ((first) as i64) + LittleEndian::getUInt(src, offset) - 1;
    while last < ((src.length) as i64) && src[((last) as i32)] == 0 && ((first) as i64) <= last {
    last -= 1;
    }
    let delta: i64 = last - ((src.length) as i64);
    last = (if delta >= 0 { last - delta - 1 } else { last });
    let l: i32 = ((last - ((first) as i64) + 1) as i32);
    value = (if codepage != -1 { String::new(src, first, l, Self::codepageToEncoding(codepage)) } else { String::new(src, first, l, StandardCharsets::UTF_8) });
}
         // case 31
         31 => {
    first = offset + 4;
    last = ((first) as i64) + LittleEndian::getUInt(src, offset) - 1;
    l = ((last - ((first) as i64)) as i32);
    let o1_31: i32 = offset + 4;
    let b: StringBuffer = StringBuffer::new(((last - ((first) as i64)) as i32));
    while ((i) as i64) <= l {
    let i1: i32 = o1_31 + i * 2;
    let i2: i32 = i1 + 1;
    if i2 >= src.length {
    break;
    }
    let high: i32 = src[i2] << 8;
    let low: i32 = src[i1] & 255;
    let c: u16 = ((high | low) as u16);
    b.append(c);
    i += 1;
    }
    while b.length() > 0 && b.charAt(b.length() - 1) == 0 {
    b.setLength(b.length() - 1);
    }
    value = b.toString();
}
         // case 64
         64 => {
    let low64: i64 = LittleEndian::getUInt(src, offset);
    let o1_64: i32 = offset + 4;
    let high64: i64 = LittleEndian::getUInt(src, o1_64);
    value = Util::filetimeToDate(((high64) as i32), ((low64) as i32));
}
         // case 71
         71 => {
    if l1 < 0 {
    l1 = LittleEndian::getInt(src, offset);
    o1 = offset + 4;
    }
    let v: Vec<u8> = vec![Default::default(); l1];
    if l1 <= src.length - o1 {
    System::arraycopy(src, o1, v, 0, v.length);
    value = v;
    }
}
         // case _
         _ => {
    let size: i32 = Math::min(src.length - offset - 1, l1);
    v = vec![Default::default(); size];
    while i < size {
    v[i] = src[o1 + i];
    i += 1;
    }
    panic!("{}", ReadingNotSupportedException::new(r#type, v));
}
      }
      // Java: return value;
      return value;
    }

    // Java: public static String codepageToEncoding(int codepage) throws UnsupportedEncodingException 
    pub fn codepageToEncoding(codepage: i32) -> String {
      // Java: if (codepage <= 0)
      if codepage <= 0 {
         // Java: throw new UnsupportedEncodingException("Codepage number may not be " + codepage);
         panic!("{}", UnsupportedEncodingException::new("Codepage number may not be " + codepage));
      }
      // Java: else
      else {
         // Java: switch (codepage)
         match codepage {
            // case 37
            37 => { return "cp037"; }
            // case 932
            932 => { return "SJIS"; }
            // case 936
            936 => { return "GBK"; }
            // case 949
            949 => { return "ms949"; }
            // case 1200
            1200 => { return "UTF-16"; }
            // case 1201
            1201 => { return "UTF-16BE"; }
            // case 1250
            1250 => { return "windows-1250"; }
            // case 1251
            1251 => { return "windows-1251"; }
            // case 1252
            1252 => { return "windows-1252"; }
            // case 1253
            1253 => { return "windows-1253"; }
            // case 1254
            1254 => { return "windows-1254"; }
            // case 1255
            1255 => { return "windows-1255"; }
            // case 1256
            1256 => { return "windows-1256"; }
            // case 1257
            1257 => { return "windows-1257"; }
            // case 1258
            1258 => { return "windows-1258"; }
            // case 1361
            1361 => { return "johab"; }
            // case 10000
            10000 => { return "MacRoman"; }
            // case 10001
            10001 => { return "SJIS"; }
            // case 10002
            10002 => { return "Big5"; }
            // case 10003
            10003 => { return "EUC-KR"; }
            // case 10004
            10004 => { return "MacArabic"; }
            // case 10005
            10005 => { return "MacHebrew"; }
            // case 10006
            10006 => { return "MacGreek"; }
            // case 10007
            10007 => { return "MacCyrillic"; }
            // case 10008
            10008 => { return "EUC_CN"; }
            // case 10010
            10010 => { return "MacRomania"; }
            // case 10017
            10017 => { return "MacUkraine"; }
            // case 10021
            10021 => { return "MacThai"; }
            // case 10029
            10029 => { return "MacCentralEurope"; }
            // case 10079
            10079 => { return "MacIceland"; }
            // case 10081
            10081 => { return "MacTurkish"; }
            // case 10082
            10082 => { return "MacCroatian"; }
            // case 20127, 65000
            20127 | 65000 => { return "US-ASCII"; }
            // case 20866
            20866 => { return "KOI8-R"; }
            // case 28591
            28591 => { return "ISO-8859-1"; }
            // case 28592
            28592 => { return "ISO-8859-2"; }
            // case 28593
            28593 => { return "ISO-8859-3"; }
            // case 28594
            28594 => { return "ISO-8859-4"; }
            // case 28595
            28595 => { return "ISO-8859-5"; }
            // case 28596
            28596 => { return "ISO-8859-6"; }
            // case 28597
            28597 => { return "ISO-8859-7"; }
            // case 28598
            28598 => { return "ISO-8859-8"; }
            // case 28599
            28599 => { return "ISO-8859-9"; }
            // case 50220, 50221, 50222
            50220 | 50221 | 50222 => { return "ISO-2022-JP"; }
            // case 50225
            50225 => { return "ISO-2022-KR"; }
            // case 51932
            51932 => { return "EUC-JP"; }
            // case 51949
            51949 => { return "EUC-KR"; }
            // case 52936
            52936 => { return "GB2312"; }
            // case 54936
            54936 => { return "GB18030"; }
            // case 65001
            65001 => { return "UTF-8"; }
            // case _
            _ => { return "cp" + codepage; }
         }
      }
    }

    // Java: public static int write(OutputStream out, long type, Object value, int codepage) throws IOException, WritingNotSupportedException 
    pub fn write(out: Box<dyn std::io::Write>, r#type: i64, value: Box<dyn std::any::Any>, codepage: i32) -> i32 {
      // Java: int length = 0;
      let length: i32 = 0;
      // Java: switch ((int)type)
      match ((r#type) as i32) {
         // case 0
         0 => {
    TypeWriter::writeUIntToStream(out, 0);
    length = 4;
}
         // case 2
         2 => {
    TypeWriter::writeToStream(out, ((value) as i32).shortValue());
    length = 2;
}
         // case 3
         3 => {
    if !value.downcast_ref::<i32>().is_some() {
    panic!("{}", ClassCastException::new("Could not cast an object to " + todo!().toString() + ": " + value.getClass().toString() + ", " + value.toString()));
    }
    length = length + TypeWriter::writeToStream(out, ((value) as i32));
}
         // case 5
         5 => { length = length + TypeWriter::writeToStream(out, ((value) as f64)); }
         // case 11
         11 => {
    let trueOrFalse: i32;
    if ((value) as bool) {
    trueOrFalse = 1;
    }
    else {
    trueOrFalse = 0;
    }
    length = TypeWriter::writeUIntToStream(out, ((trueOrFalse) as i64));
}
         // case 20
         20 => {
    TypeWriter::writeToStream(out, ((value) as i64));
    length = 8;
}
         // case 30
         30 => {
    let bytes: Vec<u8> = (if codepage == -1 { ((value) as String).getBytes(StandardCharsets::UTF_8) } else { ((value) as String).getBytes(Self::codepageToEncoding(codepage)) });
    length = TypeWriter::writeUIntToStream(out, ((bytes.length + 1) as i64));
    let b: Vec<u8> = vec![Default::default(); bytes.length + 1];
    System::arraycopy(bytes, 0, b, 0, bytes.length);
    b[b.length - 1] = 0;
    out.write(b);
    length = length + b.length;
}
         // case 31
         31 => {
    let nrOfChars: i32 = ((value) as String).length() + 1;
    length = length + TypeWriter::writeUIntToStream(out, ((nrOfChars) as i64));
    let s: Vec<u16> = Util::pad4(((value) as String));
    while i < s.length {
    let high: i32 = s[i] & 0xFF00u16 >> 8;
    let low: i32 = s[i] & 255;
    let highb: u8 = ((high) as u8);
    let lowb: u8 = ((low) as u8);
    out.write(lowb);
    out.write(highb);
    length = length + 2;
    i += 1;
    }
    out.write(0);
    out.write(0);
    length = length + 2;
}
         // case 64
         64 => {
    let filetime: i64 = Util::dateToFileTime(((value) as Date));
    let high64w: i32 = ((filetime >> 32 & 4294967295) as i32);
    let low64w: i32 = ((filetime & 4294967295) as i32);
    length = length + TypeWriter::writeUIntToStream(out, 4294967295 & ((low64w) as i64));
    length = length + TypeWriter::writeUIntToStream(out, 4294967295 & ((high64w) as i64));
}
         // case 71
         71 => {
    let b71: Vec<u8> = ((value) as Vec<u8>);
    out.write(b71);
    length = b71.length;
}
         // case _
         _ => {
    if !value.downcast_ref::<Vec<u8>>().is_some() {
    panic!("{}", WritingNotSupportedException::new(r#type, value));
    }
    b = ((value) as Vec<u8>);
    out.write(b);
    length = b.length;
    Self::writeUnsupportedTypeMessage(WritingNotSupportedException::new(r#type, value));
}
      }
      // Java: return length;
      return length;
    }

}

impl std::ops::Deref for VariantSupport {
    type Target = Variant;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}
