// Java: import cn.wps.moffice.kfs.raw.EntryBase
use crate::cn::wps::moffice::kfs::raw::EntryBase::*;
// Java: import cn.wps.moffice.util.LittleEndian
use core_libs_util::moffice::util::LittleEndian::LittleEndian;
// Java: import java.io.IOException
// Rust uses std::io::Error for Java IOException paths.
// Java: import java.io.InputStream
use std::io::Read;
// Java: import java.nio.charset.Charset
// Rust represents the selected ZIP charset by its name.
// Java: import java.nio.charset.StandardCharsets
// UTF-8 decoding is provided by String::from_utf8_lossy.
// Java: import java.util.Arrays
// Rust byte slices implement Debug, matching Arrays.toString for diagnostics.
// Java: import java.util.Date
// chrono timestamps replace java.util.Date.
// Java: import java.util.GregorianCalendar
use chrono::{Datelike, Local, TimeZone, Timelike};
// Java: import java.util.zip.ZipException
// ZipException is represented by a panic at this infallible Java-compatible API boundary.
// Java: import cn.wps.moffice.kfs.zip.ZipConstants
// ZipConstants is a Java marker interface; its constants are defined in this module.
// Java: import java.lang.Cloneable
// Rust Clone implements Java Cloneable semantics.
// Java: import java.lang.Object
// Rust values replace java.lang.Object.
use crate::cn::wps::moffice::kfs::zip::Zip64::Zip64;

// java public static final int DEFLATED = 8;
pub const DEFLATED: i32 = 8;
// java public static final int STORED = 0;
pub const STORED: i32 = 0;

// java public class ZipEntry extends EntryBase implements ZipConstants, Cloneable
#[derive(Default, Clone, Debug)]
pub struct ZipEntry {
    // java EntryBase super_
    pub(crate) super_: EntryBase,
    // java String name;
    pub name: Option<String>,
    // java String comment;
    pub comment: Option<String>,
    // java long crc = -1L;
    pub crc: i64,
    // java long compressedSize = -1L;
    pub compressedSize: i64,
    // java long size = -1L;
    pub size: i64,
    // java int compressionMethod = -1;
    pub compressionMethod: i32,
    // java int time = -1;
    pub time: i32,
    // java int modDate = -1;
    pub modDate: i32,
    // java byte[] extra;
    pub extra: Option<Vec<u8>>,
    // java long localHeaderRelOffset = -1L;
    pub localHeaderRelOffset: i64,
    // java long dataOffset = -1L;
    pub dataOffset: i64,
    // java private int gpbf;
    gpbf: i32,
}

// java public class ZipEntry extends EntryBase implements ZipConstants, Cloneable
impl ZipEntry {
    // java public  ZipEntry(String name, String comment, long crc, long compressedSize, long size, int compressionMethod, int time, int modDate, byte[] extra, long localHeaderRelOffset, long dataOffset)
    pub fn new__string_string_long_long_long_int_int_int_bytearr_long_long(
        name: Option<String>,
        comment: Option<String>,
        crc: i64,
        compressedSize: i64,
        size: i64,
        compressionMethod: i32,
        time: i32,
        modDate: i32,
        extra: Option<Vec<u8>>,
        localHeaderRelOffset: i64,
        dataOffset: i64,
    ) -> Self {
        // java super();
        // java this.name = name;
        Self {
            super_: EntryBase::new(),
            name,
            comment,
            crc,
            compressedSize,
            size,
            compressionMethod,
            time,
            modDate,
            extra,
            localHeaderRelOffset,
            dataOffset,
            gpbf: 0,
        }
        // java this.comment = comment;
        // java this.crc = crc;
        // java this.compressedSize = compressedSize;
        // java this.size = size;
        // java this.compressionMethod = compressionMethod;
        // java this.time = time;
        // java this.modDate = modDate;
        // java this.extra = extra;
        // java this.localHeaderRelOffset = localHeaderRelOffset;
        // java this.dataOffset = dataOffset;
    }

    // java public  ZipEntry(String name)
    pub fn new__string(name: String) -> Self {
        // java super();
        // java throw new NullPointerException("name == null");
        // Rust String is non-null, so the Java null guard is enforced by the type.
        // java validateStringLength("Name", name); this.name = name;
        Self::validateStringLength("Name", &name);
        let mut result = Self::new();
        result.name = Some(name);
        result
    }

    // java public  ZipEntry()
    pub fn new() -> Self {
        // java super();
        Self {
            super_: EntryBase::new(),
            name: None,
            comment: None,
            crc: -1,
            compressedSize: -1,
            size: -1,
            compressionMethod: -1,
            time: -1,
            modDate: -1,
            extra: None,
            localHeaderRelOffset: -1,
            dataOffset: -1,
            gpbf: 0,
        }
    }

    // java public String getComment()
    pub fn getComment(&self) -> Option<&str> {
        // java return this.comment;
        self.comment.as_deref()
    }

    // java public long getCompressedSize()
    pub fn getCompressedSize(&self) -> i64 {
        // java return this.compressedSize;
        return self.compressedSize;
    }

    // java public long getCrc()
    pub fn getCrc(&self) -> i64 {
        // java return this.crc;
        return self.crc;
    }

    // java public byte[] getExtra()
    pub fn getExtra(&self) -> Option<&Vec<u8>> {
        // java return this.extra;
        self.extra.as_ref()
    }

    // java public int getMethod()
    pub fn getMethod(&self) -> i32 {
        // java return this.compressionMethod;
        return self.compressionMethod;
    }

    // java public String getName()
    pub fn getName(&self) -> &str {
        // java return this.name;
        self.name.as_deref().expect("name == null")
    }

    // java public void setName(String name)
    pub fn setName(&mut self, name: Option<String>) {
        // java this.name = name;
        self.name = name;
    }

    // java public int getGpbf()
    pub fn getGpbf(&self) -> i32 {
        // java return this.gpbf;
        return self.gpbf;
    }

    // java public long getSize()
    pub fn getSize(&self) -> i64 {
        // java return this.size;
        return self.size;
    }

    // java public long getTime()
    pub fn getTime(&self) -> i64 {
        // java if (this.time != -1)
        if self.time != -1 {
            // java GregorianCalendar cal = new GregorianCalendar();
            let year = 1980 + ((self.modDate >> 9) & 127);
            // java cal.set(14, 0);
            let month = ((self.modDate >> 5) & 15) as u32;
            // java cal.set(1980 + (this.modDate >> 9 & 127), (this.modDate >> 5 & 15) - 1, this.modDate & 31, this.time >> 11 & 31, this.time >> 5 & 63, (this.time & 31) << 1);
            let local = Local
                .with_ymd_and_hms(
                    year,
                    month,
                    (self.modDate & 31) as u32,
                    ((self.time >> 11) & 31) as u32,
                    ((self.time >> 5) & 63) as u32,
                    ((self.time & 31) << 1) as u32,
                )
                .earliest();
            // java return cal.getTime().getTime();
            return local.map(|value| value.timestamp_millis()).unwrap_or(-1);
        }
        // java else
        else {
            // java return -1L;
            return -1;
        }
    }

    // java public boolean isDirectory()
    pub fn isDirectory(&self) -> bool {
        // java return this.name.charAt(this.name.length() - 1) == '/';
        return self.getName().chars().last().expect("empty ZIP entry name") == '/';
    }

    // java public void setComment(String comment)
    pub fn setComment(&mut self, comment: Option<String>) {
        // java if (comment == null)
        if comment.is_none() {
            // java this.comment = null;
            self.comment = None;
        }
        // java else
        else {
            // java validateStringLength("Comment", comment);
            Self::validateStringLength("Comment", comment.as_deref().unwrap());
            // java this.comment = comment;
            self.comment = comment;
        }
    }

    // java public void setCompressedSize(long value)
    pub fn setCompressedSize(&mut self, value: i64) {
        // java this.compressedSize = value;
        self.compressedSize = value;
    }

    // java public void setCrc(long value)
    pub fn setCrc(&mut self, value: i64) {
        // java if (value >= 0L && value <= 4294967295L)
        if value >= 0 && value <= 4294967295 {
            // java this.crc = value;
            self.crc = value;
        }
        // java else
        else {
            // java throw new IllegalArgumentException("Bad CRC32: " + value);
            panic!("Bad CRC32: {}", value);
        }
    }

    // java public void setExtra(byte[] data)
    pub fn setExtra(&mut self, data: Option<Vec<u8>>) {
        // java if (data != null && data.length > 65535)
        if let Some(ref bytes) = data {
            if bytes.len() > 65535 {
                // java throw new IllegalArgumentException("Extra data too long: " + data.length);
                panic!("Extra data too long: {}", bytes.len());
            }
        }
        // java else
        {
            // java this.extra = data;
            self.extra = data;
        }
    }

    // java public void setMethod(int value)
    pub fn setMethod(&mut self, value: i32) {
        // java if (value != 0 && value != 8)
        if value != 0 && value != 8 {
            // java throw new IllegalArgumentException("Bad method: " + value);
            panic!("Bad method: {}", value);
        }
        // java else
        else {
            // java this.compressionMethod = value;
            self.compressionMethod = value;
        }
    }

    // java public void setSize(long value)
    pub fn setSize(&mut self, value: i64) {
        // java if (value < 0L)
        if value < 0 {
            // java throw new IllegalArgumentException("Bad size: " + value);
            panic!("Bad size: {}", value);
        }
        // java else
        else {
            // java this.size = value;
            self.size = value;
        }
    }

    // java public void setTime(long value)
    pub fn setTime(&mut self, value: i64) {
        // java GregorianCalendar cal = new GregorianCalendar();
        let cal = Local
            .timestamp_millis_opt(value)
            .single()
            .or_else(|| Local.timestamp_millis_opt(value).earliest())
            .expect("timestamp is outside chrono's supported range");
        // java cal.setTime(new Date(value));
        // The chrono value already represents Date(value) in the local timezone.
        // java int year = cal.get(1);
        let year: i32 = cal.year();
        // java if (year < 1980)
        if year < 1980 {
            // java this.modDate = 33;
            self.modDate = 33;
            // java this.time = 0;
            self.time = 0;
        }
        // java else
        else {
            // java this.modDate = cal.get(5);
            self.modDate = cal.day() as i32;
            // java this.modDate |= cal.get(2) + 1 << 5;
            self.modDate |= (cal.month() as i32) << 5;
            // java this.modDate |= cal.get(1) - 1980 << 9;
            self.modDate |= (year - 1980) << 9;
            // java this.time = cal.get(13) >> 1;
            self.time = (cal.second() as i32) >> 1;
            // java this.time |= cal.get(12) << 5;
            self.time |= (cal.minute() as i32) << 5;
            // java this.time |= cal.get(11) << 11;
            self.time |= (cal.hour() as i32) << 11;
        }
    }

    // java public void setDataOffset(long value)
    pub fn setDataOffset(&mut self, value: i64) {
        // java this.dataOffset = value;
        self.dataOffset = value;
    }

    // java public long getDataOffset()
    pub fn getDataOffset(&self) -> i64 {
        // java return this.dataOffset;
        return self.dataOffset;
    }

    // java public String toString()
    pub fn toString(&self) -> String {
        // java StringBuilder sb = new StringBuilder();
        let mut sb = String::new();
        // java sb.append("[zip entry] ");
        sb.push_str("[zip entry] ");
        // java sb.append("name: ");
        sb.push_str("name: ");
        // java sb.append(this.name);
        sb.push_str(self.name.as_deref().unwrap_or("null"));
        // java sb.append("; entry type: ");
        sb.push_str("; entry type: ");
        // java sb.append(this.type());
        sb.push_str(&self.type_().to_string());
        // java return sb.toString();
        return sb;
    }

    // java public  ZipEntry(ZipEntry ze)
    pub fn new__zipentry(ze: &ZipEntry) -> Self {
        // java super();
        // java this.name = ze.name;
        Self {
            super_: EntryBase::new(),
            name: ze.name.clone(),
            comment: ze.comment.clone(),
            crc: ze.crc,
            compressedSize: ze.compressedSize,
            size: ze.size,
            compressionMethod: ze.compressionMethod,
            time: ze.time,
            modDate: ze.modDate,
            extra: ze.extra.clone(),
            localHeaderRelOffset: ze.localHeaderRelOffset,
            dataOffset: ze.dataOffset,
            gpbf: ze.gpbf,
        }
        // java this.comment = ze.comment;
        // java this.time = ze.time;
        // java this.size = ze.size;
        // java this.compressedSize = ze.compressedSize;
        // java this.crc = ze.crc;
        // java this.compressionMethod = ze.compressionMethod;
        // java this.modDate = ze.modDate;
        // java this.extra = ze.extra;
        // java this.localHeaderRelOffset = ze.localHeaderRelOffset;
        // java this.dataOffset = ze.dataOffset;
    }

    // java public Object clone()
    pub fn clone(&self) -> Self {
        // java ZipEntry result = (ZipEntry)super.clone();
        let mut result = Clone::clone(self);
        // java result.extra = this.extra != null ? (byte[])this.extra.clone() : null;
        result.extra = self.extra.clone();
        // java return result;
        return result;
    }

    // java public int hashCode()
    pub fn hashCode(&self) -> i32 {
        // java return this.name.hashCode();
        return self.getName().encode_utf16().fold(0i32, |hash, unit| {
            hash.wrapping_mul(31).wrapping_add(unit as i32)
        });
    }

    // java public int type()
    pub fn type_(&self) -> i32 {
        // java return 1;
        return 1;
    }

    // java   ZipEntry(byte[] cdeHdrBuf, InputStream cdStream, Charset defaultCharset, boolean isZip64)
    pub fn new__bytearr_inputstream_charset_boolean(
        cdeHdrBuf: &mut [u8],
        cdStream: &mut dyn Read,
        defaultCharset: &str,
        _isZip64: bool,
    ) -> Self {
        // java super();
        // java cdStream.read(cdeHdrBuf, 0, cdeHdrBuf.length);
        cdStream
            .read_exact(cdeHdrBuf)
            .expect("failed to read central directory entry");
        // java int offset = 0;
        let mut offset: usize = 0;
        // java int sig = LittleEndian.getInt(cdeHdrBuf, offset);
        let sig = LittleEndian::getInt(cdeHdrBuf, offset);
        // java offset += 4;
        offset += 4;
        // java ZipFile.throwZipException("Central Directory Entry", sig);
        if sig != 0x02014b50_u32 as i32 {
            panic!("ZipException: Central Directory Entry signature: {sig:#x}");
        }
        // java offset = 8;
        offset = 8;
        // java this.gpbf = LittleEndian.getShort(cdeHdrBuf, offset) & '\uffff';
        let mut this = Self::new();
        this.gpbf = (LittleEndian::getShort(cdeHdrBuf, offset) as u16) as i32;
        // java offset += 2;
        offset += 2;
        // java throw new ZipException("Invalid General Purpose Bit Flag: " + this.gpbf);
        if (this.gpbf & 1) != 0 {
            panic!(
                "ZipException: Invalid General Purpose Bit Flag: {}",
                this.gpbf
            );
        }
        // java Charset charset = defaultCharset; if ((this.gpbf & 2048) != 0) { charset = StandardCharsets.UTF_8; } this.compressionMethod = LittleEndian.getShort(cdeHdrBuf, offset) & '\uffff'; offset += 2; this.time = LittleEndian.getShort(cdeHdrBuf, offset) & '\uffff'; offset += 2; this.modDate = LittleEndian.getShort(cdeHdrBuf, offset) & '\uffff'; offset += 2; this.crc = (long)LittleEndian.getInt(cdeHdrBuf, offset) & 4294967295L; offset += 4; this.compressedSize = (long)LittleEndian.getInt(cdeHdrBuf, offset) & 4294967295L; offset += 4; this.size = (long)LittleEndian.getInt(cdeHdrBuf, offset) & 4294967295L; offset += 4; int nameLength = LittleEndian.getShort(cdeHdrBuf, offset) & '\uffff'; offset += 2; int extraLength = LittleEndian.getShort(cdeHdrBuf, offset) & '\uffff'; offset += 2; int commentByteCount = LittleEndian.getShort(cdeHdrBuf, offset) & '\uffff'; offset += 2; offset = 42; this.localHeaderRelOffset = (long)LittleEndian.getInt(cdeHdrBuf, offset) & 4294967295L; offset += 4; byte[] nameBytes = new byte[nameLength]; cdStream.read(nameBytes, 0, nameBytes.length); if (containsNulByte(nameBytes)) { throw new ZipException("Filename contains NUL byte: " + Arrays.toString(nameBytes)); } else { this.name = new String(nameBytes, 0, nameBytes.length, charset); if (extraLength > 0) { this.extra = new byte[extraLength]; cdStream.read(this.extra, 0, extraLength); } if (commentByteCount > 0) { byte[] commentBytes = new byte[commentByteCount]; cdStream.read(commentBytes, 0, commentByteCount); this.comment = new String(commentBytes, 0, commentBytes.length, charset); } Zip64.parseZip64ExtendedInfo(this, true); }
        let _charset = if (this.gpbf & 2048) != 0 {
            "UTF-8"
        } else {
            defaultCharset
        };
        this.compressionMethod = (LittleEndian::getShort(cdeHdrBuf, offset) as u16) as i32;
        offset += 2;
        this.time = (LittleEndian::getShort(cdeHdrBuf, offset) as u16) as i32;
        offset += 2;
        this.modDate = (LittleEndian::getShort(cdeHdrBuf, offset) as u16) as i32;
        offset += 2;
        this.crc = (LittleEndian::getInt(cdeHdrBuf, offset) as u32) as i64;
        offset += 4;
        this.compressedSize = (LittleEndian::getInt(cdeHdrBuf, offset) as u32) as i64;
        offset += 4;
        this.size = (LittleEndian::getInt(cdeHdrBuf, offset) as u32) as i64;
        offset += 4;
        let name_length = (LittleEndian::getShort(cdeHdrBuf, offset) as u16) as usize;
        offset += 2;
        let extra_length = (LittleEndian::getShort(cdeHdrBuf, offset) as u16) as usize;
        offset += 2;
        let comment_length = (LittleEndian::getShort(cdeHdrBuf, offset) as u16) as usize;
        offset = 42;
        this.localHeaderRelOffset = (LittleEndian::getInt(cdeHdrBuf, offset) as u32) as i64;
        let mut name_bytes = vec![0; name_length];
        cdStream
            .read_exact(&mut name_bytes)
            .expect("failed to read ZIP entry name");
        if Self::containsNulByte(&name_bytes) {
            panic!("ZipException: Filename contains NUL byte: {:?}", name_bytes);
        }
        this.name = Some(String::from_utf8_lossy(&name_bytes).into_owned());
        if extra_length > 0 {
            let mut bytes = vec![0; extra_length];
            cdStream
                .read_exact(&mut bytes)
                .expect("failed to read ZIP extra data");
            this.extra = Some(bytes);
        }
        if comment_length > 0 {
            let mut bytes = vec![0; comment_length];
            cdStream
                .read_exact(&mut bytes)
                .expect("failed to read ZIP comment");
            this.comment = Some(String::from_utf8_lossy(&bytes).into_owned());
        }
        Zip64::parseZip64ExtendedInfo(&mut this, true);
        this
    }

    // java private static boolean containsNulByte(byte[] bytes)
    pub fn containsNulByte(bytes: &[u8]) -> bool {
        // java for (byte b : bytes)
        for b in bytes.iter() {
            // java if (b == 0)
            if *b == 0 {
                // java return true;
                return true;
            }
        }
        // java return false;
        return false;
    }

    // java private static void validateStringLength(String argument, String string)
    pub fn validateStringLength(argument: &str, string: &str) {
        // java byte[] bytes = string.getBytes(StandardCharsets.UTF_8);
        let bytes = string.as_bytes();
        // java if (bytes.length > 65535)
        if (bytes.len() as i32) > 65535 {
            // java throw new IllegalArgumentException(argument + " too long: " + bytes.length);
            panic!("{} too long: {}", argument, bytes.len());
        }
    }
}

impl std::ops::Deref for ZipEntry {
    type Target = EntryBase;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for ZipEntry {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}
