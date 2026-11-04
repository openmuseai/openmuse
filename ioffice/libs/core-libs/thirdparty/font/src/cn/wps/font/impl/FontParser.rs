// Java: import java.io.File
use std::path::Path;
// Java: import java.io.IOException
use std::io::{self, Read, Seek, SeekFrom};
// Java: import java.io.RandomAccessFile
use std::fs::File;
use std::sync::{Mutex, OnceLock};

// Java field: String SUFFIX_SEP
pub(crate) const SUFFIX_SEP: &str = ".";
// Java field: String[] FONT_FILE_SUFFIXES
static FONT_FILE_SUFFIXES: OnceLock<Vec<String>> = OnceLock::new();
// Java field: byte[] sNameBuf
static sNameBuf: OnceLock<Mutex<Option<Vec<u8>>>> = OnceLock::new();
// Java field: int CLASSSIZE
pub(crate) const CLASSSIZE_OFFSET_TABLE: i32 = 12;
// Java field: int CLASSSIZE
pub(crate) const CLASSSIZE_TABLE_DIRECTORY: i32 = 16;
// Java field: int CLASSSIZE
pub(crate) const CLASSSIZE_NAME_TABLE_HEADER: i32 = 6;
// Java field: int CLASSSIZE
pub(crate) const CLASSSIZE_NAME_RECORD: i32 = 12;

fn get_FONT_FILE_SUFFIXES() -> &'static Vec<String> {
    FONT_FILE_SUFFIXES.get_or_init(|| vec![
        "ttf".to_string(), "ttc".to_string(), "otf".to_string(),
    ])
}

fn get_sNameBuf() -> &'static Mutex<Option<Vec<u8>>> {
    sNameBuf.get_or_init(|| Mutex::new(None))
}

// Java: public class FontParser
pub struct FontParser {
}

// Java: public class TTFOffsetTable
pub struct TTFOffsetTable {
    // Java field: int uMajorVersion
    pub(crate) uMajorVersion: i32,
    // Java field: int uMinorVersion
    pub(crate) uMinorVersion: i32,
    // Java field: int uNumOfTables
    pub(crate) uNumOfTables: i32,
    // Java field: int uSearchRange
    pub(crate) uSearchRange: i32,
    // Java field: int uEntrySelector
    pub(crate) uEntrySelector: i32,
    // Java field: int uRangeShift
    pub(crate) uRangeShift: i32,
}

// Java: public class TTFTableDirectory
pub struct TTFTableDirectory {
    // Java field: byte[] szTag
    pub(crate) szTag: Vec<u8>,
    // Java field: long uCheckSum
    pub(crate) uCheckSum: i64,
    // Java field: long uOffset
    pub(crate) uOffset: i64,
    // Java field: long uLength
    pub(crate) uLength: i64,
}

// Java: public class TTFNameTableHeader
pub struct TTFNameTableHeader {
    // Java field: int uFSelector
    pub(crate) uFSelector: i32,
    // Java field: int uNRCount
    pub(crate) uNRCount: i32,
    // Java field: int uStorageOffset
    pub(crate) uStorageOffset: i32,
}

// Java: public class TTFNameRecord
pub struct TTFNameRecord {
    // Java field: int uPlatformID
    pub(crate) uPlatformID: i32,
    // Java field: int uEncodingID
    pub(crate) uEncodingID: i32,
    // Java field: int uLanguageID
    pub(crate) uLanguageID: i32,
    // Java field: int uNameID
    pub(crate) uNameID: i32,
    // Java field: int uStringLength
    pub(crate) uStringLength: i32,
    // Java field: int uStringOffset
    pub(crate) uStringOffset: i32,
}

// Java: public class FontParser
impl FontParser {
    // Java: FontParser()
    pub fn new() -> Self {
        // Java: super();
        FontParser {}
    }

    // Java: static short uByte(byte b)
    pub fn uByte(b: u8) -> i16 {
        // Java: return (short)(255 & b);
        (255 & b) as i16
    }

    // Java: static int uShort(byte[] bytes, int off)
    pub fn uShort(bytes: &[u8], off: usize) -> i32 {
        // Java: return (uByte(bytes[off++]) << 8) + uByte(bytes[off]);
        ((Self::uByte(bytes[off]) as i32) << 8) + Self::uByte(bytes[off + 1]) as i32
    }

    // Java: static long uInt(byte[] bytes, int off)
    pub fn uInt(bytes: &[u8], off: usize) -> i64 {
        // Java: return (long)((uByte(bytes[off++]) << 24) + (uByte(bytes[off++]) << 16) + (uByte(bytes[off++]) << 8) + uByte(bytes[off]));
        ((Self::uByte(bytes[off]) as i64) << 24)
            + ((Self::uByte(bytes[off + 1]) as i64) << 16)
            + ((Self::uByte(bytes[off + 2]) as i64) << 8)
            + Self::uByte(bytes[off + 3]) as i64
    }

    // Java: static void dispose()
    pub fn dispose() {
        // Java: sNameBuf = null;
        let mut buf = get_sNameBuf().lock().unwrap();
        *buf = None;
    }

    // Java: static byte[] getNameBuf()
    pub fn getNameBuf() -> Vec<u8> {
        let mut buf = get_sNameBuf().lock().unwrap();
        // Java: if (null == sNameBuf)
        if buf.is_none() {
            // Java: sNameBuf = new byte[512];
            *buf = Some(vec![0u8; 512]);
        }
        // Java: return sNameBuf;
        buf.as_ref().unwrap().clone()
    }

    // Java: static String[] doGetFontName(int numOfTables, RandomAccessFile file, byte[] flagBuf)
    pub fn doGetFontName(numOfTables: i32, file: &mut File, flagBuf: &mut [u8]) -> Option<Vec<String>> {
        // Java: TTFTableDirectory tblDir = new TTFTableDirectory();
        let mut tblDir = TTFTableDirectory::new();
        // Java: boolean bFound = false;
        let mut bFound = false;
        // Java: for(int i = 0; i < numOfTables; ++i)
        for _i in 0..numOfTables {
            // Java: file.read(flagBuf, 0, 16);
            if file.read(&mut flagBuf[0..16]).unwrap_or(0) != 16 { break; }
            // Java: tblDir.uncompress(flagBuf, 0);
            tblDir.uncompress(flagBuf, 0);
            // Java: if (tblDir.szTag[0] == 110 && tblDir.szTag[1] == 97 && tblDir.szTag[2] == 109 && tblDir.szTag[3] == 101)
            if tblDir.szTag[0] == 110 && tblDir.szTag[1] == 97 && tblDir.szTag[2] == 109 && tblDir.szTag[3] == 101 {
                // Java: bFound = true;
                bFound = true;
                // Java: break;
                break;
            }
        }
        // Java: if (bFound)
        if bFound {
            // Java: file.seek(tblDir.uOffset);
            file.seek(SeekFrom::Start(tblDir.uOffset as u64)).ok()?;
            // Java: TTFNameTableHeader ttNTHeader = new TTFNameTableHeader();
            let mut ttNTHeader = TTFNameTableHeader::new();
            // Java: file.read(flagBuf, 0, 6);
            file.read(&mut flagBuf[0..6]).ok()?;
            // Java: ttNTHeader.uncompress(flagBuf, 0);
            ttNTHeader.uncompress(flagBuf, 0);
            // Java: String familyName = getName(ttNTHeader, file, flagBuf, tblDir.uOffset, 1);
            let mut familyName = Self::getName(&ttNTHeader, file, flagBuf, tblDir.uOffset, 1);
            // Java: String fontName = getName(ttNTHeader, file, flagBuf, tblDir.uOffset, 4);
            let fontName = Self::getName(&ttNTHeader, file, flagBuf, tblDir.uOffset, 4);
            // Java: if (null != fontName)
            if let Some(ref font_name) = fontName {
                // Java: if (null == familyName)
                if familyName.is_none() {
                    // Java: familyName = fontName;
                    familyName = fontName.clone();
                }
                // Java: return new String[]{fontName, familyName};
                return Some(vec![font_name.clone(), familyName.unwrap()]);
            }
        }
        // Java: return null;
        None
    }

    // Java: static String getName(TTFNameTableHeader ttNTHeader, RandomAccessFile file, byte[] flagBuf, long tabDirOffset, int uNameId)
    pub fn getName(ttNTHeader: &TTFNameTableHeader, file: &mut File, flagBuf: &mut [u8], tabDirOffset: i64, uNameId: i32) -> Option<String> {
        // Java: String fontName = null;
        let mut fontName: Option<String> = None;
        // Java: String utf16FontName = null;
        let mut utf16FontName: Option<String> = None;
        // Java: byte[] nameBuf = getNameBuf();
        let mut nameBuf = Self::getNameBuf();
        // Java: TTFNameRecord ttRecord = new TTFNameRecord();
        let mut ttRecord = TTFNameRecord::new();
        // Java: for(int i = 0; i < ttNTHeader.uNRCount; ++i)
        for i in 0..ttNTHeader.uNRCount {
            // Java: file.seek(tabDirOffset + 6L + (long)(i * 12));
            file.seek(SeekFrom::Start((tabDirOffset + 6 + (i as i64 * 12)) as u64)).ok()?;
            // Java: int readLen = file.read(flagBuf, 0, 12);
            let readLen = file.read(&mut flagBuf[0..12]).unwrap_or(0);
            // Java: if (readLen != 12)
            if readLen != 12 {
                // Java: break;
                break;
            }
            // Java: ttRecord.uncompress(flagBuf, 0);
            ttRecord.uncompress(flagBuf, 0);
            // Java: if ((ttRecord.uLanguageID == 2052 || primaryLangId(ttRecord.uLanguageID) == 9) && ttRecord.uNameID == uNameId)
            if (ttRecord.uLanguageID == 2052 || Self::primaryLangId(ttRecord.uLanguageID) == 9) && ttRecord.uNameID == uNameId {
                // Java: file.seek(tabDirOffset + (long)ttNTHeader.uStorageOffset + (long)ttRecord.uStringOffset);
                file.seek(SeekFrom::Start((tabDirOffset + ttNTHeader.uStorageOffset as i64 + ttRecord.uStringOffset as i64) as u64)).ok()?;
                // Java: readLen = file.read(nameBuf, 0, ttRecord.uStringLength);
                let read_len = file.read(&mut nameBuf[0..ttRecord.uStringLength as usize]).unwrap_or(0);
                // Java: if (readLen != ttRecord.uStringLength)
                if read_len != ttRecord.uStringLength as usize {
                    // Java: break;
                    break;
                }
                // Java: if (ttRecord.uPlatformID != 3)
                if ttRecord.uPlatformID != 3 {
                    // Java: fontName = new String(nameBuf, 0, ttRecord.uStringLength);
                    fontName = Some(String::from_utf8_lossy(&nameBuf[0..ttRecord.uStringLength as usize]).to_string());
                }
                // Java: else if (ttRecord.uEncodingID != 3)
                else if ttRecord.uEncodingID != 3 {
                    // Java: if (ttRecord.uEncodingID != 1 && ttRecord.uEncodingID != 0)
                    if ttRecord.uEncodingID != 1 && ttRecord.uEncodingID != 0 {
                        // Java: fontName = new String(nameBuf, 0, ttRecord.uStringLength);
                        fontName = Some(String::from_utf8_lossy(&nameBuf[0..ttRecord.uStringLength as usize]).to_string());
                    }
                    // Java: else
                    else {
                        // Java: fontName = new String(nameBuf, 0, ttRecord.uStringLength, "utf-16");
                        let u16_data: Vec<u16> = nameBuf[0..ttRecord.uStringLength as usize]
                            .chunks(2)
                            .map(|c| u16::from_be_bytes([c[0], if c.len() > 1 { c[1] } else { 0 }]))
                            .collect();
                        fontName = Some(String::from_utf16_lossy(&u16_data));
                        // Java: utf16FontName = fontName;
                        utf16FontName = fontName.clone();
                    }
                }
                // Java: else
                else {
                    // Java: byte[] str = new byte[ttRecord.uStringLength / 2];
                    let str_len = ttRecord.uStringLength as usize / 2;
                    let mut str_buf = vec![0u8; str_len];
                    // Java: for(int j = 0; j < str.length; ++j)
                    for j in 0..str_len {
                        // Java: str[j] = nameBuf[j * 2 + 1];
                        str_buf[j] = nameBuf[j * 2 + 1];
                    }
                    // Java: fontName = new String(str, "GBK");
                    fontName = Some(String::from_utf8_lossy(&str_buf).to_string());
                }
                // Java: if (fontName.startsWith("SamsungKorean"))
                if let Some(ref name) = fontName {
                    if name.starts_with("SamsungKorean") {
                        // Java: return fontName;
                        return fontName;
                    }
                }
                // Java: if (ttRecord.uLanguageID == 2052)
                if ttRecord.uLanguageID == 2052 {
                    // Java: return fontName;
                    return fontName;
                }
            }
        }
        // Java: return null != utf16FontName ? utf16FontName : fontName;
        if utf16FontName.is_some() { utf16FontName } else { fontName }
    }

    // Java: static int primaryLangId(int id)
    pub fn primaryLangId(id: i32) -> i32 {
        // Java: return id & 1023;
        id & 1023
    }

    // Java: static String[][] getFontNames(File file)
    pub fn getFontNames(file: &Path) -> Option<Vec<Vec<String>>> {
        // Java: String[][] fontNames = TTCParser.getFontNamesFromFile(file);
        let fontNames = TTCParser::getFontNamesFromFile(file);
        // Java: if (null != fontNames && 0 != fontNames.length)
        if let Some(ref names) = fontNames {
            if !names.is_empty() {
                // Java: return fontNames;
                return fontNames;
            }
        }
        // Java: else
        // Java: String[] fontName = TTFParser.getFontNameFromFile(file);
        let fontName = TTFParser::getFontNameFromFile(file);
        // Java: return null != fontName && fontName.length > 0 ? new String[][]{fontName} : null;
        if let Some(name) = fontName {
            if !name.is_empty() {
                return Some(vec![name]);
            }
        }
        None
    }

    // Java: static boolean hasFontFileSuffix(String fontName)
    pub fn hasFontFileSuffix(fontName: &str) -> bool {
        // Java: int sepIdx = fontName.lastIndexOf(".");
        let sepIdx = fontName.rfind('.');
        // Java: if (sepIdx < 0)
        if sepIdx.is_none() {
            // Java: return false;
            return false;
        }
        // Java: else
        // Java: int suffixStartIdx = sepIdx + ".".length();
        let suffixStartIdx = sepIdx.unwrap() + 1;
        // Java: String suffix = fontName.substring(suffixStartIdx);
        let suffix = &fontName[suffixStartIdx..];
        // Java: int i = 0;
        let suffixes = get_FONT_FILE_SUFFIXES();
        // Java: for(int n = FONT_FILE_SUFFIXES.length; i < n; ++i)
        for s in suffixes.iter() {
            // Java: if (FONT_FILE_SUFFIXES[i].equalsIgnoreCase(suffix))
            if s.eq_ignore_ascii_case(suffix) {
                // Java: return true;
                return true;
            }
        }
        // Java: return false;
        false
    }
}

// Java: public class TTFOffsetTable
impl TTFOffsetTable {
    pub fn new() -> Self {
        TTFOffsetTable {
            uMajorVersion: 0, uMinorVersion: 0, uNumOfTables: 0,
            uSearchRange: 0, uEntrySelector: 0, uRangeShift: 0,
        }
    }

    // Java: void uncompress(byte[] bytes, int offset)
    pub fn uncompress(&mut self, bytes: &[u8], offset: usize) {
        let mut offset = offset;
        // Java: this.uMajorVersion = FontParser.uShort(bytes, offset);
        self.uMajorVersion = FontParser::uShort(bytes, offset);
        // Java: offset += 2;
        offset += 2;
        // Java: this.uMinorVersion = FontParser.uShort(bytes, offset);
        self.uMinorVersion = FontParser::uShort(bytes, offset);
        // Java: offset += 2;
        offset += 2;
        // Java: this.uNumOfTables = FontParser.uShort(bytes, offset);
        self.uNumOfTables = FontParser::uShort(bytes, offset);
        // Java: offset += 2;
        offset += 2;
        // Java: this.uSearchRange = FontParser.uShort(bytes, offset);
        self.uSearchRange = FontParser::uShort(bytes, offset);
        // Java: offset += 2;
        offset += 2;
        // Java: this.uEntrySelector = FontParser.uShort(bytes, offset);
        self.uEntrySelector = FontParser::uShort(bytes, offset);
        // Java: offset += 2;
        offset += 2;
        // Java: this.uRangeShift = FontParser.uShort(bytes, offset);
        self.uRangeShift = FontParser::uShort(bytes, offset);
    }
}

// Java: public class TTFTableDirectory
impl TTFTableDirectory {
    pub fn new() -> Self {
        TTFTableDirectory {
            szTag: vec![0u8; 4], uCheckSum: 0, uOffset: 0, uLength: 0,
        }
    }

    // Java: void uncompress(byte[] bytes, int offset)
    pub fn uncompress(&mut self, bytes: &[u8], offset: usize) {
        let mut offset = offset;
        // Java: System.arraycopy(bytes, offset, this.szTag, 0, 4);
        self.szTag[0..4].copy_from_slice(&bytes[offset..offset + 4]);
        // Java: offset += 4;
        offset += 4;
        // Java: this.uCheckSum = FontParser.uInt(bytes, offset);
        self.uCheckSum = FontParser::uInt(bytes, offset);
        // Java: offset += 4;
        offset += 4;
        // Java: this.uOffset = FontParser.uInt(bytes, offset);
        self.uOffset = FontParser::uInt(bytes, offset);
        // Java: offset += 4;
        offset += 4;
        // Java: this.uLength = FontParser.uInt(bytes, offset);
        self.uLength = FontParser::uInt(bytes, offset);
    }
}

// Java: public class TTFNameTableHeader
impl TTFNameTableHeader {
    pub fn new() -> Self {
        TTFNameTableHeader { uFSelector: 0, uNRCount: 0, uStorageOffset: 0 }
    }

    // Java: void uncompress(byte[] bytes, int offset)
    pub fn uncompress(&mut self, bytes: &[u8], offset: usize) {
        let mut offset = offset;
        // Java: this.uFSelector = FontParser.uShort(bytes, offset);
        self.uFSelector = FontParser::uShort(bytes, offset);
        // Java: offset += 2;
        offset += 2;
        // Java: this.uNRCount = FontParser.uShort(bytes, offset);
        self.uNRCount = FontParser::uShort(bytes, offset);
        // Java: offset += 2;
        offset += 2;
        // Java: this.uStorageOffset = FontParser.uShort(bytes, offset);
        self.uStorageOffset = FontParser::uShort(bytes, offset);
    }
}

// Java: public class TTFNameRecord
impl TTFNameRecord {
    pub fn new() -> Self {
        TTFNameRecord {
            uPlatformID: 0, uEncodingID: 0, uLanguageID: 0,
            uNameID: 0, uStringLength: 0, uStringOffset: 0,
        }
    }

    // Java: void uncompress(byte[] bytes, int offset)
    pub fn uncompress(&mut self, bytes: &[u8], offset: usize) {
        let mut offset = offset;
        // Java: this.uPlatformID = FontParser.uShort(bytes, offset);
        self.uPlatformID = FontParser::uShort(bytes, offset);
        // Java: offset += 2;
        offset += 2;
        // Java: this.uEncodingID = FontParser.uShort(bytes, offset);
        self.uEncodingID = FontParser::uShort(bytes, offset);
        // Java: offset += 2;
        offset += 2;
        // Java: this.uLanguageID = FontParser.uShort(bytes, offset);
        self.uLanguageID = FontParser::uShort(bytes, offset);
        // Java: offset += 2;
        offset += 2;
        // Java: this.uNameID = FontParser.uShort(bytes, offset);
        self.uNameID = FontParser::uShort(bytes, offset);
        // Java: offset += 2;
        offset += 2;
        // Java: this.uStringLength = FontParser.uShort(bytes, offset);
        self.uStringLength = FontParser::uShort(bytes, offset);
        // Java: offset += 2;
        offset += 2;
        // Java: this.uStringOffset = FontParser.uShort(bytes, offset);
        self.uStringOffset = FontParser::uShort(bytes, offset);
    }
}

struct TTCParser;
impl TTCParser {
    fn getFontNamesFromFile(_file: &Path) -> Option<Vec<Vec<String>>> {
        None
    }
}

struct TTFParser;
impl TTFParser {
    fn getFontNameFromFile(_file: &Path) -> Option<Vec<String>> {
        None
    }
}
