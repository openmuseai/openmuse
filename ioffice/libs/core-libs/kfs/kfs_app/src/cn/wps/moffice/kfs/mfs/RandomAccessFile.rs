#![allow(non_snake_case, dead_code, unused_variables, unused_imports, unused_mut, unused_assignments, unused_parens)]

// Java: import cn.wps.moffice.kfs.mfs.core.iface.IMfsFile
use crate::kfs::mfs::core::iface::IMfsFile::*;
// Java: import cn.wps.moffice.kfs.mfs.File
use crate::kfs::mfs::File::File;
use crate::kfs::mfs::Memory::{Memory, ByteOrder};
use crate::kfs::mfs::ModifiedUtf8::ModifiedUtf8;

// Java: public class RandomAccessFile implements DataInput, DataOutput, Closeable
pub struct RandomAccessFile {
    // Java field:    private int mode;
    mode: i32,
    // Java field:    private final byte[] scratch;
    scratch: Vec<u8>,
    // Java field:    private long mFilePointer;
    mFilePointer: i64,
    // Java field:    private IMfsFile mMfsFile;
    mMfsFile: Option<Box<dyn IMfsFile>>,
    // Java field:    private java.io.RandomAccessFile mJdkRaf;
    mJdkRaf: Option<std::fs::File>,
}

impl Default for RandomAccessFile {
    fn default() -> Self {
        Self { mode: 0, scratch: vec![0u8; 8], mFilePointer: 0, mMfsFile: None, mJdkRaf: None }
    }
}

// Java: public class RandomAccessFile implements DataInput, DataOutput, Closeable
impl RandomAccessFile {
    // Java: public RandomAccessFile(File file, String mode) throws FileNotFoundException
    pub fn new(file: &mut File, mode: &str) -> RandomAccessFile {
        let mode_int = if mode.contains('w') { 1 } else { 0 };
        // Java: if (file.exists0())
        if file.exists0() {
            let f = std::fs::OpenOptions::new()
                .read(true)
                .write(mode.contains('w'))
                .open(file.getPath())
                .ok();
            RandomAccessFile { mode: mode_int, scratch: vec![0u8; 8], mFilePointer: 0, mMfsFile: None, mJdkRaf: f }
        } else {
            // Java: this.mMfsFile = file.getMfsFile();
            let mfs = file.getMfsFile();
            if mfs.is_none() {
                panic!("FileNotFoundException");
            }
            RandomAccessFile { mode: mode_int, scratch: vec![0u8; 8], mFilePointer: 0, mMfsFile: mfs, mJdkRaf: None }
        }
    }

    // Java: public RandomAccessFile(String fileName, String mode) throws FileNotFoundException
    pub fn new__string_string(fileName: String, mode: &str) -> RandomAccessFile {
        let mut file = File::new__string(fileName);
        Self::new(&mut file, mode)
    }

    // Java: public void close() throws IOException
    pub fn close(&mut self) {
        self.mJdkRaf = None;
        self.mMfsFile = None;
    }

    // Java: public long getFilePointer() throws IOException
    pub fn getFilePointer(&mut self) -> i64 {
        // Java: return null != this.mJdkRaf ? this.mJdkRaf.getFilePointer() : this.mFilePointer;
        if let Some(ref mut f) = self.mJdkRaf {
            use std::io::Seek;
            f.stream_position().unwrap_or(0) as i64
        } else {
            self.mFilePointer
        }
    }

    // Java: public long length() throws IOException
    pub fn length(&mut self) -> i64 {
        // Java: return this.mJdkRaf != null ? this.mJdkRaf.length() : this.mMfsFile.length();
        if let Some(ref f) = self.mJdkRaf {
            f.metadata().map(|m| m.len() as i64).unwrap_or(0)
        } else if let Some(ref mut mfs) = self.mMfsFile {
            mfs.length()
        } else {
            0
        }
    }

    // Java: public int read() throws IOException
    pub fn read(&mut self) -> i32 {
        // Java: return this.read(this.scratch, 0, 1) != -1 ? this.scratch[0] & 255 : -1;
        let mut buf = vec![0u8; 1];
        if self.read__bytearr_int_int(&mut buf, 0, 1) != -1 {
            (buf[0] as i32) & 255
        } else {
            -1
        }
    }

    // Java: public int read(byte[] buffer) throws IOException
    pub fn read__bytearr(&mut self, buffer: &mut [u8]) -> i32 {
        // Java: return this.read(buffer, 0, buffer.length);
        let len = buffer.len() as i32;
        self.read__bytearr_int_int(buffer, 0, len)
    }

    // Java: public int read(byte[] buffer, int byteOffset, int byteCount) throws IOException
    pub fn read__bytearr_int_int(&mut self, buffer: &mut [u8], byteOffset: i32, byteCount: i32) -> i32 {
        let off = byteOffset as usize;
        let cnt = byteCount as usize;
        // Java: if (this.mJdkRaf != null)
        if let Some(ref mut f) = self.mJdkRaf {
            use std::io::Read;
            match f.read(&mut buffer[off..off + cnt]) {
                Ok(0) => -1,
                Ok(n) => n as i32,
                Err(_) => -1,
            }
        }
        // Java: else if (null != this.mMfsFile)
        else if let Some(ref mut mfs) = self.mMfsFile {
            // Java: int count = this.mMfsFile.read(this.mFilePointer, buffer, byteOffset, byteCount);
            let count = mfs.read(self.mFilePointer, buffer[off..off + cnt].to_vec(), byteOffset, byteCount);
            // Java: this.mFilePointer += (long)(count > 0 ? count : 0);
            self.mFilePointer += (if count > 0 { count } else { 0 }) as i64;
            count
        }
        // Java: else
        else {
            -1
        }
    }

    // Java: public final boolean readBoolean() throws IOException
    pub fn readBoolean(&mut self) -> bool {
        let temp = self.read();
        if temp < 0 { panic!("EOFException"); }
        temp != 0
    }

    // Java: public final byte readByte() throws IOException
    pub fn readByte(&mut self) -> u8 {
        let temp = self.read();
        if temp < 0 { panic!("EOFException"); }
        temp as u8
    }

    // Java: public final char readChar() throws IOException
    pub fn readChar(&mut self) -> u16 {
        self.readShort() as u16
    }

    // Java: public final double readDouble() throws IOException
    pub fn readDouble(&mut self) -> f64 {
        // Java: return Double.longBitsToDouble(this.readLong());
        f64::from_bits(self.readLong() as u64)
    }

    // Java: public final float readFloat() throws IOException
    pub fn readFloat(&mut self) -> f32 {
        // Java: return Float.intBitsToFloat(this.readInt());
        f32::from_bits(self.readInt() as u32)
    }

    // Java: public final void readFully(byte[] dst) throws IOException
    pub fn readFully(&mut self, dst: &mut [u8]) {
        let len = dst.len() as i32;
        self.readFully__bytearr_int_int(dst, 0, len);
    }

    // Java: public final void readFully(byte[] dst, int offset, int byteCount) throws IOException
    pub fn readFully__bytearr_int_int(&mut self, dst: &mut [u8], mut offset: i32, mut byteCount: i32) {
        // Java: checkOffsetAndCount(dst.length, offset, byteCount);
        Self::checkOffsetAndCount(dst.len() as i32, offset, byteCount);
        // Java: while(byteCount > 0)
        while byteCount > 0 {
            // Java: int result = this.read(dst, offset, byteCount);
            let result = self.read__bytearr_int_int(dst, offset, byteCount);
            // Java: if (result < 0) throw new EOFException();
            if result < 0 { panic!("EOFException"); }
            offset += result;
            byteCount -= result;
        }
    }

    // Java: private static void checkOffsetAndCount(int arrayLength, int offset, int count)
    fn checkOffsetAndCount(arrayLength: i32, offset: i32, count: i32) {
        if (offset | count) < 0 || offset > arrayLength || arrayLength - offset < count {
            panic!("ArrayIndexOutOfBoundsException");
        }
    }

    // Java: public final int readInt() throws IOException
    pub fn readInt(&mut self) -> i32 {
        // Java: this.readFully(this.scratch, 0, 4);
        let mut scratch = vec![0u8; 8];
        self.readFully__bytearr_int_int(&mut scratch, 0, 4);
        self.scratch = scratch.clone();
        // Java: return Memory.peekInt(this.scratch, 0, ByteOrder.BIG_ENDIAN);
        Memory::peekInt(&self.scratch, 0, ByteOrder::BIG_ENDIAN)
    }

    // Java: public final String readLine() throws IOException
    pub fn readLine(&mut self) -> Option<String> {
        // Java: StringBuilder line = new StringBuilder(80);
        let mut line = String::with_capacity(80);
        // Java: boolean foundTerminator = false;
        let mut foundTerminator = false;
        // Java: long unreadPosition = 0L;
        let mut unreadPosition: i64 = 0;
        // Java: while(true)
        loop {
            // Java: int nextByte = this.read();
            let nextByte = self.read();
            // Java: switch (nextByte)
            match nextByte {
                -1 => {
                    // Java: return line.length() != 0 ? line.toString() : null;
                    return if !line.is_empty() { Some(line) } else { None };
                }
                10 => { return Some(line); }
                13 => {
                    if foundTerminator {
                        self.seek(unreadPosition);
                        return Some(line);
                    }
                    foundTerminator = true;
                    unreadPosition = self.getFilePointer();
                }
                _ => {
                    if foundTerminator {
                        self.seek(unreadPosition);
                        return Some(line);
                    }
                    line.push(nextByte as u8 as char);
                }
            }
        }
    }

    // Java: public final long readLong() throws IOException
    pub fn readLong(&mut self) -> i64 {
        let mut scratch = vec![0u8; 8];
        self.readFully__bytearr_int_int(&mut scratch, 0, 8);
        self.scratch = scratch.clone();
        // Java: return Memory.peekLong(this.scratch, 0, ByteOrder.BIG_ENDIAN);
        Memory::peekLong(&self.scratch, 0, ByteOrder::BIG_ENDIAN)
    }

    // Java: public final short readShort() throws IOException
    pub fn readShort(&mut self) -> i16 {
        let mut scratch = vec![0u8; 8];
        self.readFully__bytearr_int_int(&mut scratch, 0, 2);
        self.scratch = scratch.clone();
        // Java: return Memory.peekShort(this.scratch, 0, ByteOrder.BIG_ENDIAN);
        Memory::peekShort(&self.scratch, 0, ByteOrder::BIG_ENDIAN)
    }

    // Java: public final int readUnsignedByte() throws IOException
    pub fn readUnsignedByte(&mut self) -> i32 {
        let temp = self.read();
        if temp < 0 { panic!("EOFException"); }
        temp
    }

    // Java: public final int readUnsignedShort() throws IOException
    pub fn readUnsignedShort(&mut self) -> i32 {
        // Java: return this.readShort() & '\uffff';
        (self.readShort() as i32) & 0xFFFF
    }

    // Java: public final String readUTF() throws IOException
    pub fn readUTF(&mut self) -> String {
        // Java: int utfSize = this.readUnsignedShort();
        let utfSize = self.readUnsignedShort();
        // Java: if (utfSize == 0) return "";
        if utfSize == 0 { return String::new(); }
        // Java: byte[] buf = new byte[utfSize];
        let mut buf = vec![0u8; utfSize as usize];
        let buf_len = buf.len() as i32;
        // Java: if (this.read(buf, 0, buf.length) != buf.length)
        if self.read__bytearr_int_int(&mut buf, 0, buf_len) != buf_len {
            panic!("EOFException");
        }
        // Java: return ModifiedUtf8.decode(buf, new char[utfSize], 0, utfSize);
        let mut out = vec![0u16; utfSize as usize];
        ModifiedUtf8::decode(&buf, &mut out, 0, utfSize)
    }

    // Java: public void seek(long offset) throws IOException
    pub fn seek(&mut self, offset: i64) {
        // Java: if (offset < 0L) throw new IOException("offset < 0: " + offset);
        if offset < 0 {
            panic!("IOException: offset < 0: {}", offset);
        }
        // Java: if (this.mJdkRaf != null)
        if let Some(ref mut f) = self.mJdkRaf {
            use std::io::Seek;
            let _ = f.seek(std::io::SeekFrom::Start(offset as u64));
        }
        // Java: else
        else {
            // Java: this.mFilePointer = offset;
            self.mFilePointer = offset;
        }
    }

    // Java: public void setLength(long newLength) throws IOException
    pub fn setLength(&mut self, newLength: i64) {
        // Java: if (this.mJdkRaf != null)
        if let Some(ref f) = self.mJdkRaf {
            let _ = f.set_len(newLength as u64);
        }
        // Java: else
        else if let Some(ref mut mfs) = self.mMfsFile {
            // Java: this.mMfsFile.truncate(newLength);
            mfs.truncate(newLength);
        }
    }

    // Java: public int skipBytes(int count) throws IOException
    pub fn skipBytes(&mut self, count: i32) -> i32 {
        // Java: if (this.mJdkRaf != null)
        if self.mJdkRaf.is_some() {
            use std::io::Seek;
            if let Some(ref mut f) = self.mJdkRaf {
                let _ = f.seek(std::io::SeekFrom::Current(count as i64));
            }
            return count;
        }
        // Java: else if (count > 0)
        if count > 0 {
            let currentPos = self.getFilePointer();
            let eof = self.length();
            let newCount = if currentPos + (count as i64) > eof { (eof - currentPos) as i32 } else { count };
            self.seek(currentPos + (newCount as i64));
            return newCount;
        }
        0
    }

    // Java: public void write(byte[] buffer) throws IOException
    pub fn write(&mut self, buffer: &[u8]) {
        let len = buffer.len() as i32;
        self.write__bytearr_int_int(buffer, 0, len);
    }

    // Java: public void write(byte[] buffer, int byteOffset, int byteCount) throws IOException
    pub fn write__bytearr_int_int(&mut self, buffer: &[u8], byteOffset: i32, byteCount: i32) {
        let off = byteOffset as usize;
        let cnt = byteCount as usize;
        // Java: if (null != this.mJdkRaf)
        if let Some(ref mut f) = self.mJdkRaf {
            use std::io::Write;
            let _ = f.write_all(&buffer[off..off + cnt]);
        }
        // Java: else if (null != this.mMfsFile)
        else if let Some(ref mut mfs) = self.mMfsFile {
            // Java: this.mMfsFile.write(this.mFilePointer, buffer, byteOffset, byteCount);
            mfs.write(self.mFilePointer, buffer[off..off + cnt].to_vec(), byteOffset, byteCount);
            // Java: this.mFilePointer += (long)byteCount;
            self.mFilePointer += byteCount as i64;
        }
    }

    // Java: public void write(int oneByte) throws IOException
    pub fn write__int(&mut self, oneByte: i32) {
        // Java: this.scratch[0] = (byte)(oneByte & 255);
        let buf = [(oneByte & 255) as u8];
        self.write(&buf);
    }

    // Java: public final void writeBoolean(boolean val) throws IOException
    pub fn writeBoolean(&mut self, val: bool) {
        self.write__int(if val { 1 } else { 0 });
    }

    // Java: public final void writeByte(int val) throws IOException
    pub fn writeByte(&mut self, val: i32) {
        self.write__int(val & 255);
    }

    // Java: public final void writeBytes(String str) throws IOException
    pub fn writeBytes(&mut self, s: &str) {
        // Java: byte[] bytes = new byte[str.length()];
        let bytes: Vec<u8> = s.bytes().map(|b| b & 255).collect();
        self.write(&bytes);
    }

    // Java: public final void writeChar(int val) throws IOException
    pub fn writeChar(&mut self, val: i32) {
        self.writeShort(val);
    }

    // Java: public final void writeChars(String str) throws IOException
    pub fn writeChars(&mut self, s: &str) {
        // Java: this.write(str.getBytes("UTF-16BE"));
        let utf16: Vec<u8> = s.encode_utf16()
            .flat_map(|c| c.to_be_bytes())
            .collect();
        self.write(&utf16);
    }

    // Java: public final void writeDouble(double val) throws IOException
    pub fn writeDouble(&mut self, val: f64) {
        // Java: this.writeLong(Double.doubleToLongBits(val));
        self.writeLong(val.to_bits() as i64);
    }

    // Java: public final void writeFloat(float val) throws IOException
    pub fn writeFloat(&mut self, val: f32) {
        // Java: this.writeInt(Float.floatToIntBits(val));
        self.writeInt(val.to_bits() as i32);
    }

    // Java: public final void writeInt(int val) throws IOException
    pub fn writeInt(&mut self, val: i32) {
        // Java: Memory.pokeInt(this.scratch, 0, val, ByteOrder.BIG_ENDIAN);
        Memory::pokeInt(&mut self.scratch, 0, val, ByteOrder::BIG_ENDIAN);
        let scratch = self.scratch.clone();
        self.write__bytearr_int_int(&scratch, 0, 4);
    }

    // Java: public final void writeLong(long val) throws IOException
    pub fn writeLong(&mut self, val: i64) {
        // Java: Memory.pokeLong(this.scratch, 0, val, ByteOrder.BIG_ENDIAN);
        Memory::pokeLong(&mut self.scratch, 0, val, ByteOrder::BIG_ENDIAN);
        let scratch = self.scratch.clone();
        self.write__bytearr_int_int(&scratch, 0, 8);
    }

    // Java: public final void writeShort(int val) throws IOException
    pub fn writeShort(&mut self, val: i32) {
        // Java: Memory.pokeShort(this.scratch, 0, (short)val, ByteOrder.BIG_ENDIAN);
        Memory::pokeShort(&mut self.scratch, 0, val as i16, ByteOrder::BIG_ENDIAN);
        let scratch = self.scratch.clone();
        self.write__bytearr_int_int(&scratch, 0, 2);
    }

    // Java: public final void writeUTF(String str) throws IOException
    pub fn writeUTF(&mut self, s: &str) {
        // Java: this.write(ModifiedUtf8.encode(str));
        let encoded = ModifiedUtf8::encode__string(s);
        self.write(&encoded);
    }
}
