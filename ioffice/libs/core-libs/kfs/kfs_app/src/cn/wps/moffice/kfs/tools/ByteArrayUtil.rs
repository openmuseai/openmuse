#![allow(non_snake_case, dead_code, unused_variables, unused_imports, unused_mut, unused_assignments)]

// Java field:       static final int BLOCK_SIZE = 4096;
pub const BLOCK_SIZE: i32 = 4096;

// Java: public class ByteArrayUtil
#[derive(Default)]
pub struct ByteArrayUtil {}

// Java: private static class ByteArray
#[derive(Default)]
pub struct ByteArray {
    // Java field:       ArrayList<byte[]> blocks;
    pub blocks: Vec<Vec<u8>>,
    // Java field:       int count;
    pub count: i32,
}

// Java: public class ByteArrayUtil
impl ByteArrayUtil {
    // Java: public ByteArrayUtil()
    pub fn new() -> ByteArrayUtil { Self {} }

    // Java: public static byte[] parse(String bytes)
    pub fn parse(bytes: String) -> Vec<u8> {
        // Java: return parse_binary_text(bytes);
        Self::parse_binary_text(bytes)
    }

    // Java: private static byte[] parse_binary_text(String text)
    fn parse_binary_text(text: String) -> Vec<u8> {
        // Java: char[] chars = new char[text.length()];
        let chars: Vec<u16> = text.encode_utf16().collect();
        // Java: ByteArray bytes = new ByteArray();
        let mut bytes = ByteArray::new();
        // Java: int ichar = 0;
        let mut ichar: usize = 0;
        // Java: while(ichar < chars.length)
        while ichar < chars.len() {
            // Java: char c = chars[ichar];
            let c = chars[ichar];
            // Java: if (isxdigit(c))
            if Self::isxdigit(c) {
                // Java: if (ichar == chars.length - 1)
                if ichar == chars.len() - 1 {
                    // Java: throw new RuntimeException("invalid data");
                    panic!("invalid data");
                }
                // Java: char c2 = chars[ichar + 1];
                let c2 = chars[ichar + 1];
                // Java: if (!isxdigit(c2))
                if !Self::isxdigit(c2) {
                    // Java: throw new RuntimeException("invalid data");
                    panic!("invalid data");
                }
                // Java: int intVal = xdigit_index(c) * 16 + xdigit_index(c2);
                let int_val = Self::xdigit_index(c) * 16 + Self::xdigit_index(c2);
                // Java: bytes.append((byte)intVal);
                bytes.append(int_val as u8);
                // Java: ichar += 2;
                ichar += 2;
            } else {
                // Java: ++ichar;
                ichar += 1;
            }
        }
        // Java: return bytes.toArray();
        bytes.toArray()
    }

    // Java: private static boolean isxdigit(char c)
    fn isxdigit(c: u16) -> bool {
        // Java: return c >= '0' && c <= '9' || c >= 'a' && c <= 'f' || c >= 'A' && c <= 'F';
        (c >= b'0' as u16 && c <= b'9' as u16)
            || (c >= b'a' as u16 && c <= b'f' as u16)
            || (c >= b'A' as u16 && c <= b'F' as u16)
    }

    // Java: private static int xdigit_index(char c)
    fn xdigit_index(c: u16) -> i32 {
        // Java: if (c >= '0' && c <= '9')
        if c >= b'0' as u16 && c <= b'9' as u16 {
            // Java: return c - 48;
            (c - 48) as i32
        } else if c >= b'a' as u16 && c <= b'f' as u16 {
            // Java: return c >= 'a' && c <= 'f' ? c - 97 + 10 : c - 65 + 10;
            (c - 97 + 10) as i32
        } else {
            (c - 65 + 10) as i32
        }
    }
}

// Java: private static class ByteArray
impl ByteArray {
    // Java: private ByteArray()
    pub fn new() -> ByteArray {
        Self { blocks: Vec::new(), count: 0 }
    }

    // Java: void append(byte b)
    pub fn append(&mut self, b: u8) {
        // Java: int block_index = this.count / 4096;
        let block_index = (self.count / 4096) as usize;
        // Java: int block_offset = this.count % 4096;
        let block_offset = (self.count % 4096) as usize;
        // Java: if (block_index >= this.blocks.size())
        if block_index >= self.blocks.len() {
            // Java: byte[] block = new byte[4096];
            // Java: this.blocks.add(block);
            self.blocks.push(vec![0u8; 4096]);
        }
        // Java: byte[] block = (byte[])this.blocks.get(block_index);
        // Java: block[block_offset] = b;
        self.blocks[block_index][block_offset] = b;
        // Java: ++this.count;
        self.count += 1;
    }

    // Java: int length()
    pub fn length(&self) -> i32 {
        // Java: return this.count;
        self.count
    }

    // Java: byte[] toArray()
    pub fn toArray(&self) -> Vec<u8> {
        // Java: byte[] buf = new byte[this.length()];
        let len = self.count as usize;
        let mut buf = vec![0u8; len];
        // Java: for(int i = 0; i < this.blocks.size(); ++i)
        for i in 0..self.blocks.len() {
            // Java: int n = 4096;
            let n = if i == self.blocks.len() - 1 {
                // Java: n = this.count % 4096;
                (self.count % 4096) as usize
            } else {
                4096
            };
            // Java: System.arraycopy(this.blocks.get(i), 0, buf, i * 4096, n);
            let offset = i * 4096;
            buf[offset..offset + n].copy_from_slice(&self.blocks[i][..n]);
        }
        // Java: return buf;
        buf
    }
}
