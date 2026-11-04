// Java: import java.io.File
use std::path::Path;
// Java: import java.io.IOException
use std::io::{Read, Seek, SeekFrom};
// Java: import java.io.RandomAccessFile
use std::fs::File;
use super::FontParser::{FontParser, TTFOffsetTable};

// Java field: int TAG_SIZE
const TAG_SIZE: i32 = 4;

// Java: public class TTCParser extends FontParser
pub struct TTCParser {
}

// Java: public class TTCParser extends FontParser
impl TTCParser {
    // Java: TTCParser()
    pub fn new() -> Self {
        // Java: super();
        TTCParser {}
    }

    // Java: static String[][] getFontNamesFromFile(File fontFile)
    pub fn getFontNamesFromFile(font_file: &Path) -> Option<Vec<Option<Vec<String>>>> {
        // Java: RandomAccessFile file = new RandomAccessFile(fontFile, "r");
        let mut file = File::open(font_file).ok()?;
        // Java: byte[] flagBuf = new byte[16];
        let mut flag_buf = [0u8; 16];
        // Java: String[] fontName = null;
        // Java: file.read(flagBuf, 0, 4);
        file.read_exact(&mut flag_buf[0..4]).ok()?;
        // Java: if ((char)flagBuf[0] == 't' && (char)flagBuf[1] == 't' && (char)flagBuf[2] == 'c' && (char)flagBuf[3] == 'f')
        if flag_buf[0] == b't' && flag_buf[1] == b't' && flag_buf[2] == b'c' && flag_buf[3] == b'f' {
            // Java: file.read(flagBuf, 0, 4);
            file.read_exact(&mut flag_buf[0..4]).ok()?;
            // Java: file.read(flagBuf, 0, 4);
            file.read_exact(&mut flag_buf[0..4]).ok()?;
            // Java: long num = uInt(flagBuf, 0);
            let num = FontParser::uInt(&flag_buf, 0) as usize;
            // Java: long[] offsets = new long[(int)num];
            let mut offsets = vec![0i64; num];
            // Java: for(int i = 0; (long)i < num; ++i)
            for i in 0..num {
                // Java: file.read(flagBuf, 0, 4);
                file.read_exact(&mut flag_buf[0..4]).ok()?;
                // Java: offsets[i] = uInt(flagBuf, 0);
                offsets[i] = FontParser::uInt(&flag_buf, 0);
            }
            // Java: String[][] result = new String[(int)num][];
            let mut result: Vec<Option<Vec<String>>> = Vec::with_capacity(num);
            // Java: for(int n = 0; (long)n < num; ++n)
            for n in 0..num {
                // Java: file.seek(offsets[n]);
                file.seek(SeekFrom::Start(offsets[n] as u64)).ok()?;
                // Java: FontParser.TTFOffsetTable ttOffsetTable = new FontParser.TTFOffsetTable();
                let mut tt_offset_table = TTFOffsetTable::new();
                // Java: file.read(flagBuf, 0, 12);
                file.read_exact(&mut flag_buf[0..12]).ok()?;
                // Java: ttOffsetTable.uncompress(flagBuf, 0);
                tt_offset_table.uncompress(&flag_buf, 0);
                // Java: fontName = doGetFontName(ttOffsetTable.uNumOfTables, file, flagBuf);
                let font_name = FontParser::doGetFontName(tt_offset_table.uNumOfTables, &mut file, &mut flag_buf);
                // Java: result[n] = fontName;
                result.push(font_name);
            }
            // Java: file.close();
            // Java: return result;
            Some(result)
        }
        // Java: else
        else {
            // Java: file.close();
            // Java: return null;
            None
        }
    }
}
