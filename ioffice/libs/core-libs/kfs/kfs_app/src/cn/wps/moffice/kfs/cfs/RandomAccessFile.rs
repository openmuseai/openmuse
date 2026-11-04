#![allow(non_snake_case, dead_code, unused_variables, unused_imports, unused_mut, unused_assignments)]

use crate::kfs::tools::Tools::Tools;
use crate::kfs::Memory::{Memory, ByteOrder};
use crate::kfs::ModifiedUtf8::ModifiedUtf8;
use super::CfsCore::*;
use super::File::File;

// Java: public class RandomAccessFile implements DataInput, DataOutput, Closeable
pub struct RandomAccessFile {
    // Java field:    private int fd;
    fd: i32,
    // Java field:    private boolean syncMetadata;
    syncMetadata: bool,
    // Java field:    private int mode;
    mode: i32,
    // Java field:    private final byte[] scratch;
    scratch: Vec<u8>,
}

impl Default for RandomAccessFile {
    fn default() -> Self { Self { fd: -1, syncMetadata: false, mode: 0, scratch: vec![0u8; 8] } }
}

// Java: public class RandomAccessFile implements DataInput, DataOutput, Closeable
impl RandomAccessFile {
    // Java: public RandomAccessFile(File file, String mode) throws FileNotFoundException
    pub fn new(file: &File, mode: &str) -> RandomAccessFile {
        // Java: int flags = ...
        let mut flags = 0i32;
        let mode_int;
        if mode == "r" {
            flags = O_RDONLY;
            mode_int = 0;
        } else if mode == "rw" || mode == "rws" || mode == "rwd" {
            flags = O_RDWR | O_CREAT;
            mode_int = if mode == "rws" { 2 } else if mode == "rwd" { 1 } else { 0 };
        } else {
            panic!("IllegalArgumentException: Invalid mode: {}", mode);
        }
        // Java: this.fd = CfsCore.nativeCfsOpen(Tools.string2bytes(file.getPath()), flags);
        let fd = CfsCore::nativeCfsOpen(&Tools::string2bytes(file.getPath()), flags);
        if fd < 0 { panic!("FileNotFoundException"); }
        RandomAccessFile { fd, syncMetadata: mode == "rws", mode: mode_int, scratch: vec![0u8; 8] }
    }

    // Java: public RandomAccessFile(String fileName, String mode) throws FileNotFoundException
    pub fn new__string_string(fileName: String, mode: &str) -> RandomAccessFile {
        let file = File::new__string(fileName);
        Self::new(&file, mode)
    }

    // Java: public void close() throws IOException
    pub fn close(&mut self) {
        if self.fd >= 0 {
            CfsCore::nativeCfsClose(self.fd);
            self.fd = -1;
        }
    }

    // Java: public long getFilePointer() throws IOException
    pub fn getFilePointer(&mut self) -> i64 {
        CfsCore::nativeCfsSeek(self.fd, 0, SEEK_CUR) as i64
    }

    // Java: public long length() throws IOException
    pub fn length(&mut self) -> i64 {
        CfsCore::nativeCfsLength(self.fd)
    }

    // Java: public int read() throws IOException
    pub fn read(&mut self) -> i32 {
        let mut buf = [0u8; 1];
        if self.read__bytearr_int_int(&mut buf, 0, 1) != -1 { (buf[0] as i32) & 255 } else { -1 }
    }

    // Java: public int read(byte[] buffer) throws IOException
    pub fn read__bytearr(&mut self, buffer: &mut [u8]) -> i32 {
        let len = buffer.len() as i32;
        self.read__bytearr_int_int(buffer, 0, len)
    }

    // Java: public int read(byte[] buffer, int byteOffset, int byteCount) throws IOException
    pub fn read__bytearr_int_int(&mut self, buffer: &mut [u8], byteOffset: i32, byteCount: i32) -> i32 {
        CfsCore::read(self.fd, buffer, byteOffset, byteCount)
    }

    // Java: public final boolean readBoolean() throws IOException
    pub fn readBoolean(&mut self) -> bool {
        let t = self.read();
        if t < 0 { panic!("EOFException"); }
        t != 0
    }

    // Java: public final byte readByte() throws IOException
    pub fn readByte(&mut self) -> u8 {
        let t = self.read();
        if t < 0 { panic!("EOFException"); }
        t as u8
    }

    // Java: public final char readChar() throws IOException
    pub fn readChar(&mut self) -> u16 { self.readShort() as u16 }

    // Java: public final double readDouble() throws IOException
    pub fn readDouble(&mut self) -> f64 { f64::from_bits(self.readLong() as u64) }

    // Java: public final float readFloat() throws IOException
    pub fn readFloat(&mut self) -> f32 { f32::from_bits(self.readInt() as u32) }

    // Java: public final void readFully(byte[] dst) throws IOException
    pub fn readFully(&mut self, dst: &mut [u8]) {
        let len = dst.len() as i32;
        self.readFully__bytearr_int_int(dst, 0, len);
    }

    // Java: public final void readFully(byte[] dst, int offset, int byteCount) throws IOException
    pub fn readFully__bytearr_int_int(&mut self, dst: &mut [u8], mut offset: i32, mut byteCount: i32) {
        CfsCore::checkOffsetAndCount(dst.len() as i32, offset, byteCount);
        while byteCount > 0 {
            let result = self.read__bytearr_int_int(dst, offset, byteCount);
            if result < 0 { panic!("EOFException"); }
            offset += result;
            byteCount -= result;
        }
    }

    // Java: public final int readInt() throws IOException
    pub fn readInt(&mut self) -> i32 {
        let mut scratch = vec![0u8; 8];
        self.readFully__bytearr_int_int(&mut scratch, 0, 4);
        Memory::peekInt(&scratch, 0, ByteOrder::BIG_ENDIAN)
    }

    // Java: public final long readLong() throws IOException
    pub fn readLong(&mut self) -> i64 {
        let mut scratch = vec![0u8; 8];
        self.readFully__bytearr_int_int(&mut scratch, 0, 8);
        Memory::peekLong(&scratch, 0, ByteOrder::BIG_ENDIAN)
    }

    // Java: public final short readShort() throws IOException
    pub fn readShort(&mut self) -> i16 {
        let mut scratch = vec![0u8; 8];
        self.readFully__bytearr_int_int(&mut scratch, 0, 2);
        Memory::peekShort(&scratch, 0, ByteOrder::BIG_ENDIAN)
    }

    // Java: public final int readUnsignedByte() throws IOException
    pub fn readUnsignedByte(&mut self) -> i32 {
        let t = self.read();
        if t < 0 { panic!("EOFException"); }
        t
    }

    // Java: public final int readUnsignedShort() throws IOException
    pub fn readUnsignedShort(&mut self) -> i32 { (self.readShort() as i32) & 0xFFFF }

    // Java: public final String readUTF() throws IOException
    pub fn readUTF(&mut self) -> String {
        let utfSize = self.readUnsignedShort();
        if utfSize == 0 { return String::new(); }
        let mut buf = vec![0u8; utfSize as usize];
        let buf_len = buf.len() as i32;
        if self.read__bytearr_int_int(&mut buf, 0, buf_len) != buf_len {
            panic!("EOFException");
        }
        let mut out = vec![0u16; utfSize as usize];
        ModifiedUtf8::decode(&buf, &mut out, 0, utfSize)
    }

    // Java: public void seek(long offset) throws IOException
    pub fn seek(&mut self, offset: i64) {
        if offset < 0 { panic!("IOException: offset < 0: {}", offset); }
        CfsCore::nativeCfsSeek(self.fd, offset, SEEK_SET);
    }

    // Java: public void setLength(long newLength) throws IOException
    pub fn setLength(&mut self, newLength: i64) {
        CfsCore::nativeCfsTruncate(self.fd, newLength);
    }

    // Java: public int skipBytes(int count) throws IOException
    pub fn skipBytes(&mut self, count: i32) -> i32 {
        if count > 0 {
            CfsCore::nativeCfsSeek(self.fd, count as i64, SEEK_CUR);
            count
        } else {
            0
        }
    }

    // Java: public void write(byte[] buffer) throws IOException
    pub fn write(&mut self, buffer: &[u8]) {
        let len = buffer.len() as i32;
        self.write__bytearr_int_int(buffer, 0, len);
    }

    // Java: public void write(byte[] buffer, int byteOffset, int byteCount) throws IOException
    pub fn write__bytearr_int_int(&mut self, buffer: &[u8], byteOffset: i32, byteCount: i32) {
        CfsCore::write(self.fd, buffer, byteOffset, byteCount);
        if self.syncMetadata { CfsCore::nativeCfsSync(self.fd); }
    }

    // Java: public void write(int oneByte) throws IOException
    pub fn write__int(&mut self, oneByte: i32) {
        let buf = [(oneByte & 255) as u8];
        self.write(&buf);
    }

    // Java: public final void writeBoolean(boolean val) throws IOException
    pub fn writeBoolean(&mut self, val: bool) { self.write__int(if val { 1 } else { 0 }); }
    // Java: public final void writeByte(int val) throws IOException
    pub fn writeByte(&mut self, val: i32) { self.write__int(val & 255); }

    // Java: public final void writeBytes(String str) throws IOException
    pub fn writeBytes(&mut self, s: &str) {
        let bytes: Vec<u8> = s.bytes().map(|b| b & 255).collect();
        self.write(&bytes);
    }

    // Java: public final void writeChar(int val) throws IOException
    pub fn writeChar(&mut self, val: i32) { self.writeShort(val); }

    // Java: public final void writeChars(String str) throws IOException
    pub fn writeChars(&mut self, s: &str) {
        let utf16: Vec<u8> = s.encode_utf16().flat_map(|c| c.to_be_bytes()).collect();
        self.write(&utf16);
    }

    // Java: public final void writeDouble(double val) throws IOException
    pub fn writeDouble(&mut self, val: f64) { self.writeLong(val.to_bits() as i64); }
    // Java: public final void writeFloat(float val) throws IOException
    pub fn writeFloat(&mut self, val: f32) { self.writeInt(val.to_bits() as i32); }

    // Java: public final void writeInt(int val) throws IOException
    pub fn writeInt(&mut self, val: i32) {
        let mut scratch = vec![0u8; 8];
        Memory::pokeInt(&mut scratch, 0, val, ByteOrder::BIG_ENDIAN);
        self.write__bytearr_int_int(&scratch, 0, 4);
    }

    // Java: public final void writeLong(long val) throws IOException
    pub fn writeLong(&mut self, val: i64) {
        let mut scratch = vec![0u8; 8];
        Memory::pokeLong(&mut scratch, 0, val, ByteOrder::BIG_ENDIAN);
        self.write__bytearr_int_int(&scratch, 0, 8);
    }

    // Java: public final void writeShort(int val) throws IOException
    pub fn writeShort(&mut self, val: i32) {
        let mut scratch = vec![0u8; 8];
        Memory::pokeShort(&mut scratch, 0, val as i16, ByteOrder::BIG_ENDIAN);
        self.write__bytearr_int_int(&scratch, 0, 2);
    }

    // Java: public final void writeUTF(String str) throws IOException
    pub fn writeUTF(&mut self, s: &str) {
        let encoded = ModifiedUtf8::encode__string(s);
        self.write(&encoded);
    }

    // Java: public final String readLine() throws IOException
    pub fn readLine(&mut self) -> Option<String> {
        let mut line = String::with_capacity(80);
        let mut foundTerminator = false;
        let mut unreadPosition: i64 = 0;
        loop {
            let nextByte = self.read();
            match nextByte {
                -1 => return if !line.is_empty() { Some(line) } else { None },
                10 => return Some(line),
                13 => {
                    if foundTerminator { self.seek(unreadPosition); return Some(line); }
                    foundTerminator = true;
                    unreadPosition = self.getFilePointer();
                }
                _ => {
                    if foundTerminator { self.seek(unreadPosition); return Some(line); }
                    line.push(nextByte as u8 as char);
                }
            }
        }
    }
}
