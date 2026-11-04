// Java: import java.io.File
use std::path::Path;
// Java: import java.io.IOException
use std::io::Read;
// Java: import java.io.RandomAccessFile
use std::fs::File;
use super::FontParser::{FontParser, TTFOffsetTable};

// Java field: int MAJOR_TT_OUTLINES
const MAJOR_TT_OUTLINES: i32 = 1;
// Java field: int MINOR_TT_OUTLINES
const MINOR_TT_OUTLINES: i32 = 0;
// Java field: int MAJOR_CFF_OUTLINES
const MAJOR_CFF_OUTLINES: i32 = 20308;
// Java field: int MINOR_CFF_OUTLINES
const MINOR_CFF_OUTLINES: i32 = 21583;

// Java: public class TTFParser extends FontParser
pub struct TTFParser {
}

// Java: public class TTFParser extends FontParser
impl TTFParser {
    // Java: TTFParser()
    pub fn new() -> Self {
        // Java: super();
        TTFParser {}
    }

    // Java: static String[] getFontNameFromFile(File fontFile)
    pub fn getFontNameFromFile(font_file: &Path) -> Option<Vec<String>> {
        // Java: RandomAccessFile file = new RandomAccessFile(fontFile, "r");
        let mut file = File::open(font_file).ok()?;
        // Java: byte[] flagBuf = new byte[16];
        let mut flag_buf = [0u8; 16];
        // Java: FontParser.TTFOffsetTable ttOffsetTable = new FontParser.TTFOffsetTable();
        let mut tt_offset_table = TTFOffsetTable::new();
        // Java: file.read(flagBuf, 0, 12);
        file.read_exact(&mut flag_buf[0..12]).ok()?;
        // Java: ttOffsetTable.uncompress(flagBuf, 0);
        tt_offset_table.uncompress(&flag_buf, 0);
        // Java: boolean isValidVersion = false;
        let mut is_valid_version = false;
        // Java: int major = ttOffsetTable.uMajorVersion;
        let major = tt_offset_table.uMajorVersion;
        // Java: int minor = ttOffsetTable.uMinorVersion;
        let minor = tt_offset_table.uMinorVersion;
        // Java: if (1 == major && 0 == minor)
        if major == MAJOR_TT_OUTLINES && minor == MINOR_TT_OUTLINES {
            // Java: isValidVersion = true;
            is_valid_version = true;
        }
        // Java: else if (20308 == major && 21583 == minor)
        else if major == MAJOR_CFF_OUTLINES && minor == MINOR_CFF_OUTLINES {
            // Java: isValidVersion = true;
            is_valid_version = true;
        }
        // Java: String[] fontName = isValidVersion ? doGetFontName(ttOffsetTable.uNumOfTables, file, flagBuf) : null;
        let font_name = if is_valid_version {
            FontParser::doGetFontName(tt_offset_table.uNumOfTables, &mut file, &mut flag_buf)
        } else {
            None
        };
        // Java: file.close();
        // Java: return fontName;
        font_name
    }
}
