// Java: import cn.wps.moffice.kfs.File
// Java: import cn.wps.moffice.kfs.FileInputStream
// Java: import java.io.ByteArrayInputStream
// Java: import java.io.IOException
// Java: import java.io.InputStream
// Java: import java.util.ArrayList
// Java: import java.util.List
use std::io::{self, Read, Cursor};

// Java: public class HexRead
pub struct HexRead;

// Java: public class HexRead
impl HexRead {
    // Java: HexRead()
    // Java: super();

    // Java: static byte[] readData(String filename)
    pub fn read_data_file(filename: &str) -> io::Result<Vec<u8>> {
        // Java: File file = new File(filename);
        // Java: FileInputStream stream = new FileInputStream(file);
        let data = std::fs::read(filename)?;
        let mut stream = Cursor::new(data);
        // Java: byte[] var3;
        // Java: try
        {
            // Java: var3 = readData(stream, -1);
            let var3 = Self::read_data_stream(&mut stream, -1)?;
            // Java: finally
            // Java: stream.close();
            // Java: return var3;
            Ok(var3)
        }
    }

    // Java: static byte[] readData(InputStream stream, String section)
    pub fn read_data_section(stream: &mut dyn Read, section: &str) -> io::Result<Vec<u8>> {
        // Java: try
        {
            // Java: StringBuffer sectionText = new StringBuffer();
            let mut section_text = String::new();
            // Java: boolean inSection = false;
            let mut in_section = false;
            // Java: for(int c = stream.read(); c != -1; c = stream.read())
            loop {
                let mut buf = [0u8; 1];
                let c = match stream.read(&mut buf) {
                    Ok(0) => -1i32,
                    Ok(_) => buf[0] as i32,
                    Err(_) => -1i32,
                };
                if c == -1 {
                    break;
                }
                // Java: switch (c)
                match c {
                    // Java: case 10:
                    // Java: case 13:
                    10 | 13 => {
                        // Java: inSection = false;
                        in_section = false;
                        // Java: sectionText = new StringBuffer();
                        section_text.clear();
                        // Java: break;
                    }
                    // Java: case 91:
                    91 => {
                        // Java: inSection = true;
                        in_section = true;
                        // Java: break;
                    }
                    // Java: case 93:
                    93 => {
                        // Java: inSection = false;
                        in_section = false;
                        // Java: if (sectionText.toString().equals(section))
                        if section_text == section {
                            // Java: byte[] var5 = readData(stream, 91);
                            let var5 = Self::read_data_stream(stream, 91)?;
                            // Java: return var5;
                            return Ok(var5);
                        }
                        // Java: sectionText = new StringBuffer();
                        section_text.clear();
                        // Java: break;
                    }
                    // Java: default:
                    _ => {
                        // Java: if (inSection)
                        if in_section {
                            // Java: sectionText.append((char)c);
                            section_text.push(c as u8 as char);
                        }
                    }
                }
            }
            // Java: throw new IOException("Section '" + section + "' not found");
            Err(io::Error::new(io::ErrorKind::NotFound, format!("Section '{}' not found", section)))
        }
        // Java: finally
        // Java: stream.close();
    }

    // Java: static byte[] readData(String filename, String section)
    pub fn read_data_file_section(filename: &str, section: &str) -> io::Result<Vec<u8>> {
        // Java: File file = new File(filename);
        // Java: FileInputStream stream = new FileInputStream(file);
        let data = std::fs::read(filename)?;
        let mut stream = Cursor::new(data);
        // Java: return readData((InputStream)stream, section);
        Self::read_data_section(&mut stream, section)
    }

    // Java: static byte[] readData(InputStream stream, int eofChar)
    pub fn read_data_stream(stream: &mut dyn Read, eof_char: i32) -> io::Result<Vec<u8>> {
        // Java: int characterCount = 0;
        let mut character_count = 0;
        // Java: byte b = 0;
        let mut b: u8 = 0;
        // Java: List bytes = new ArrayList();
        let mut bytes: Vec<u8> = Vec::new();
        // Java: boolean done = false;
        let mut done = false;
        // Java: while(!done)
        while !done {
            // Java: int count = stream.read();
            let mut buf = [0u8; 1];
            let count: i32 = match stream.read(&mut buf) {
                Ok(0) => -1,
                Ok(_) => buf[0] as i32,
                Err(_) => -1,
            };
            // Java: char baseChar = 'a';
            let mut base_char = b'a' as i32;
            // Java: if (count == eofChar)
            if count == eof_char {
                // Java: break;
                break;
            }
            // Java: switch (count)
            match count {
                // Java: case -1:
                -1 => {
                    // Java: done = true;
                    done = true;
                }
                // Java: case 0:
                // Java: case 1:
                // Java: case 2:
                // Java: case 3:
                // Java: case 4:
                // Java: case 5:
                // Java: case 6:
                // Java: case 7:
                // Java: case 8:
                // Java: case 9:
                // Java: case 10:
                // Java: case 11:
                // Java: case 12:
                // Java: case 13:
                // Java: case 14:
                // Java: case 15:
                // Java: case 16:
                // Java: case 17:
                // Java: case 18:
                // Java: case 19:
                // Java: case 20:
                // Java: case 21:
                // Java: case 22:
                // Java: case 23:
                // Java: case 24:
                // Java: case 25:
                // Java: case 26:
                // Java: case 27:
                // Java: case 28:
                // Java: case 29:
                // Java: case 30:
                // Java: case 31:
                // Java: case 32:
                // Java: case 33:
                // Java: case 34:
                // Java: case 36:
                // Java: case 37:
                // Java: case 38:
                // Java: case 39:
                // Java: case 40:
                // Java: case 41:
                // Java: case 42:
                // Java: case 43:
                // Java: case 44:
                // Java: case 45:
                // Java: case 46:
                // Java: case 47:
                // Java: case 58:
                // Java: case 59:
                // Java: case 60:
                // Java: case 61:
                // Java: case 62:
                // Java: case 63:
                // Java: case 64:
                // Java: case 71:
                // Java: case 72:
                // Java: case 73:
                // Java: case 74:
                // Java: case 75:
                // Java: case 76:
                // Java: case 77:
                // Java: case 78:
                // Java: case 79:
                // Java: case 80:
                // Java: case 81:
                // Java: case 82:
                // Java: case 83:
                // Java: case 84:
                // Java: case 85:
                // Java: case 86:
                // Java: case 87:
                // Java: case 88:
                // Java: case 89:
                // Java: case 90:
                // Java: case 91:
                // Java: case 92:
                // Java: case 93:
                // Java: case 94:
                // Java: case 95:
                // Java: case 96:
                // Java: default:
                0..=34 | 36..=47 | 58..=64 | 71..=96 => {
                    // Java: break;
                }
                // Java: case 35:
                35 => {
                    // Java: readToEOL(stream);
                    Self::read_to_eol(stream)?;
                    // Java: break;
                }
                // Java: case 48:
                // Java: case 49:
                // Java: case 50:
                // Java: case 51:
                // Java: case 52:
                // Java: case 53:
                // Java: case 54:
                // Java: case 55:
                // Java: case 56:
                // Java: case 57:
                48..=57 => {
                    // Java: b = (byte)(b << 4);
                    b = b << 4;
                    // Java: b = (byte)(b + (byte)(count - 48));
                    b = b.wrapping_add((count - 48) as u8);
                    // Java: ++characterCount;
                    character_count += 1;
                    // Java: if (characterCount == 2)
                    if character_count == 2 {
                        // Java: bytes.add(b);
                        bytes.push(b);
                        // Java: characterCount = 0;
                        character_count = 0;
                        // Java: b = 0;
                        b = 0;
                    }
                    // Java: break;
                }
                // Java: case 65:
                // Java: case 66:
                // Java: case 67:
                // Java: case 68:
                // Java: case 69:
                // Java: case 70:
                65..=70 => {
                    // Java: baseChar = 'A';
                    base_char = b'A' as i32;
                    // Java: b = (byte)(b << 4);
                    b = b << 4;
                    // Java: b = (byte)(b + (byte)(count + 10 - baseChar));
                    b = b.wrapping_add((count + 10 - base_char) as u8);
                    // Java: ++characterCount;
                    character_count += 1;
                    // Java: if (characterCount == 2)
                    if character_count == 2 {
                        // Java: bytes.add(b);
                        bytes.push(b);
                        // Java: characterCount = 0;
                        character_count = 0;
                        // Java: b = 0;
                        b = 0;
                    }
                }
                // Java: case 97:
                // Java: case 98:
                // Java: case 99:
                // Java: case 100:
                // Java: case 101:
                // Java: case 102:
                97..=102 => {
                    // Java: b = (byte)(b << 4);
                    b = b << 4;
                    // Java: b = (byte)(b + (byte)(count + 10 - baseChar));
                    b = b.wrapping_add((count + 10 - base_char) as u8);
                    // Java: ++characterCount;
                    character_count += 1;
                    // Java: if (characterCount == 2)
                    if character_count == 2 {
                        // Java: bytes.add(b);
                        bytes.push(b);
                        // Java: characterCount = 0;
                        character_count = 0;
                        // Java: b = 0;
                        b = 0;
                    }
                }
                _ => {
                    // Java: break;
                }
            }
        }
        // Java: Byte[] polished = (Byte[])bytes.toArray(new Byte[0]);
        // Java: byte[] rval = new byte[polished.length];
        // Java: for(int j = 0; j < polished.length; ++j)
        // Java: rval[j] = polished[j];
        // Java: return rval;
        Ok(bytes)
    }

    // Java: static byte[] readFromString(String data)
    pub fn read_from_string(data: &str) -> Vec<u8> {
        // Java: try
        {
            // Java: return readData(new ByteArrayInputStream(data.getBytes()), -1);
            let mut cursor = Cursor::new(data.as_bytes().to_vec());
            Self::read_data_stream(&mut cursor, -1).unwrap()
        }
        // Java: catch (IOException e)
        // Java: throw new RuntimeException(e);
    }

    // Java: static void readToEOL(InputStream stream)
    fn read_to_eol(stream: &mut dyn Read) -> io::Result<()> {
        // Java: for(int c = stream.read(); c != -1 && c != 10 && c != 13; c = stream.read())
        loop {
            let mut buf = [0u8; 1];
            let c = match stream.read(&mut buf) {
                Ok(0) => -1i32,
                Ok(_) => buf[0] as i32,
                Err(_) => -1i32,
            };
            if c == -1 || c == 10 || c == 13 {
                break;
            }
        }
        Ok(())
    }
}
