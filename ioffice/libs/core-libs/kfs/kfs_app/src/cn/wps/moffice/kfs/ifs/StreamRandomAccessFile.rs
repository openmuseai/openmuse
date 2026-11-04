#![allow(non_snake_case, dead_code, unused_variables, unused_imports, unused_mut, unused_assignments)]

// Java: import cn.wps.moffice.kfs.File
use crate::kfs::File::File;
// Java: import cn.wps.moffice.kfs.FileInputStream
use crate::kfs::FileInputStream::FileInputStream;
// Java: import cn.wps.moffice.kfs.FileOutputStream
use crate::kfs::FileOutputStream::FileOutputStream;
// Java: import cn.wps.moffice.kfs.KfsFileUtil
use crate::kfs::KfsFileUtil::KfsFileUtil;
// Java: import cn.wps.moffice.kfs.UTFDataFormatException
use crate::kfs::UTFDataFormatException::UTFDataFormatException;
// Java: import cn.wps.moffice.util.PlatformAdapter
use crate::util::PlatformAdapter::PlatformAdapter;
use crate::kfs::Memory::{Memory, ByteOrder};

// Java field:    private static final int BLOCK_SIZE = 1048576;
const BLOCK_SIZE: usize = 1048576;

// Java: public class StreamRandomAccessFile
pub struct StreamRandomAccessFile {
    // Java field:    private final byte[] mBuff;
    mBuff: Vec<u8>,
    // Java field:    private final byte[] mScratch;
    mScratch: Vec<u8>,
    // Java field:    private InputStream mIs;
    mIs: Option<Box<dyn std::io::Read>>,
    // Java field:    private long mLength;
    mLength: i64,
    // Java field:    private long mFilePointer;
    mFilePointer: i64,
    // Java field:    private final List<String> mBlockPaths;
    mBlockPaths: Vec<String>,
}

// Java: public class StreamRandomAccessFile
impl StreamRandomAccessFile {
    // Java: public StreamRandomAccessFile(InputStream is) throws FileNotFoundException
    pub fn new(is: Box<dyn std::io::Read>) -> StreamRandomAccessFile {
        StreamRandomAccessFile {
            mBuff: vec![0u8; BLOCK_SIZE],
            mScratch: vec![0u8; 8],
            mIs: Some(is),
            mLength: 0,
            mFilePointer: 0,
            mBlockPaths: Vec::new(),
        }
    }

    // Java: public long getFilePointer()
    pub fn getFilePointer(&self) -> i64 {
        // Java: return this.mFilePointer;
        self.mFilePointer
    }

    // Java: public String readUTF() throws IOException
    pub fn readUTF(&mut self) -> String {
        // Java: int utfLen = this.readUnsignedShort();
        let utfLen = self.readUnsignedShort() as usize;
        // Java: byte[] byteArray = new byte[utfLen];
        let mut byteArray = vec![0u8; utfLen];
        // Java: char[] charArray = new char[utfLen];
        let mut charArray = vec![0u16; utfLen];
        let mut count: usize = 0;
        let mut charArrayCount: usize = 0;
        // Java: this.readFully(byteArray, 0, utfLen);
        self.readFully(&mut byteArray, 0, utfLen as i32);

        // Java: while(count < utfLen) { int c = byteArray[count] & 255; if (c > 127) break; ...
        while count < utfLen {
            let c = (byteArray[count] as i32) & 255;
            if c > 127 { break; }
            count += 1;
            charArray[charArrayCount] = c as u16;
            charArrayCount += 1;
        }

        // Java: while(count < utfLen) { ... switch (c >> 4) ... }
        while count < utfLen {
            let c = (byteArray[count] as i32) & 255;
            match c >> 4 {
                0..=7 => {
                    count += 1;
                    charArray[charArrayCount] = c as u16;
                    charArrayCount += 1;
                }
                12 | 13 => {
                    count += 2;
                    if count > utfLen {
                        panic!("UTFDataFormatException: malformed input: partial character at end");
                    }
                    let char2 = byteArray[count - 1] as i32;
                    if (char2 & 192) != 128 {
                        panic!("UTFDataFormatException: malformed input around byte {}", count);
                    }
                    charArray[charArrayCount] = (((c & 31) << 6) | (char2 & 63)) as u16;
                    charArrayCount += 1;
                }
                14 => {
                    count += 3;
                    if count > utfLen {
                        panic!("UTFDataFormatException: malformed input: partial character at end");
                    }
                    let char2 = byteArray[count - 2] as i32;
                    let char3 = byteArray[count - 1] as i32;
                    if (char2 & 192) != 128 || (char3 & 192) != 128 {
                        panic!("UTFDataFormatException: malformed input around byte {}", count - 1);
                    }
                    charArray[charArrayCount] = (((c & 15) << 12) | ((char2 & 63) << 6) | (char3 & 63)) as u16;
                    charArrayCount += 1;
                }
                _ => {
                    panic!("UTFDataFormatException: malformed input around byte {}", count);
                }
            }
        }
        // Java: return new String(charArray, 0, charArrayCount);
        String::from_utf16_lossy(&charArray[..charArrayCount])
    }

    // Java: public void setLength(long newLength) throws IOException
    pub fn setLength(&mut self, _newLength: i64) {
        panic!("UnsupportedOperationException");
    }

    // Java: public long length() throws IOException
    pub fn length(&mut self) -> i64 {
        // Java: while(this.nextBlock()) {}
        while self.nextBlock() {}
        // Java: return this.mLength;
        self.mLength
    }

    // Java: public int read() throws IOException
    pub fn read(&mut self) -> i32 {
        // Java: return this.read(this.mScratch, 0, 1) != -1 ? this.mScratch[0] & 255 : -1;
        let mut buf = vec![0u8; 1];
        if self.read__bytearr_int_int(&mut buf, 0, 1) != -1 {
            (buf[0] as i32) & 255
        } else {
            -1
        }
    }

    // Java: public int read(byte[] dst, int offset, int byteCount) throws IOException
    pub fn read__bytearr_int_int(&mut self, dst: &mut [u8], mut offset: i32, byteCount: i32) -> i32 {
        // Java: ArrayUtils.checkOffsetAndCount(dst.length, offset, byteCount);
        if offset < 0 || byteCount < 0 || (dst.len() as i32) - offset < byteCount {
            panic!("ArrayIndexOutOfBoundsException");
        }
        // Java: if (0 == byteCount) return 0;
        if byteCount == 0 { return 0; }

        let mut remaining = byteCount;
        let mut bytesHaveRead: i32 = 0;

        // Java: while(true)
        while remaining > 0 {
            let currentPos = self.getFilePointer();
            let blockIndex = (currentPos / BLOCK_SIZE as i64) as usize;
            let relativePos = (currentPos % BLOCK_SIZE as i64) as i32;

            if blockIndex < self.mBlockPaths.len() || self.nextBlock() {
                let blockPath = self.mBlockPaths[blockIndex].clone();
                let mut file = File::new__string(blockPath);
                let file_len = file.length();
                let mut bytesWillRead = (file_len - relativePos as i64) as i32;
                if bytesWillRead != 0 {
                    bytesWillRead = remaining.min(bytesWillRead);
                    let mut fis = FileInputStream::new(file.getPath());
                    fis.skip(relativePos as i64);
                    let nRead = fis.read__bytearr_int_int(dst, offset, bytesWillRead);
                    fis.close();
                    if nRead > 0 && nRead == bytesWillRead {
                        bytesHaveRead += bytesWillRead;
                        offset += bytesWillRead;
                        remaining -= bytesWillRead;
                        self.seek(currentPos + bytesWillRead as i64);
                        continue;
                    }
                    panic!("IOException: StreamRandomAccessFile read bytes error!");
                }
            }
            break;
        }

        if remaining == byteCount { -1 } else { bytesHaveRead }
    }

    // Java: public void readFully(byte[] dst, int offset, int byteCount) throws IOException
    pub fn readFully(&mut self, dst: &mut [u8], mut offset: i32, mut byteCount: i32) {
        if offset < 0 || byteCount < 0 || (dst.len() as i32) - offset < byteCount {
            panic!("ArrayIndexOutOfBoundsException");
        }
        let mut n = 0;
        // Java: do { int count = this.read(dst, offset + n, byteCount - n); ... } while(n < byteCount);
        loop {
            let count = self.read__bytearr_int_int(dst, offset + n, byteCount - n);
            if count < 0 { panic!("EOFException"); }
            n += count;
            if n >= byteCount { break; }
        }
    }

    // Java: public int skipBytes(int count) throws IOException
    pub fn skipBytes(&mut self, count: i32) -> i32 {
        if count > 0 {
            let currentPos = self.getFilePointer();
            let eof = self.length();
            let newCount = if currentPos + count as i64 > eof { (eof - currentPos) as i32 } else { count };
            self.seek(currentPos + newCount as i64);
            newCount
        } else {
            0
        }
    }

    // Java: public void seek(long offset) throws IOException
    pub fn seek(&mut self, offset: i64) {
        // Java: this.mFilePointer = offset;
        self.mFilePointer = offset;
    }

    // Java: public void close() throws IOException
    pub fn close(&mut self) {
        self.closeInputStream();
    }

    // Java: public boolean readBoolean() throws IOException
    pub fn readBoolean(&mut self) -> bool {
        let temp = self.read();
        if temp < 0 { panic!("EOFException"); }
        temp != 0
    }

    // Java: public byte readByte() throws IOException
    pub fn readByte(&mut self) -> u8 {
        let temp = self.read();
        if temp < 0 { panic!("EOFException"); }
        temp as u8
    }

    // Java: public int readUnsignedByte() throws IOException
    pub fn readUnsignedByte(&mut self) -> i32 {
        let temp = self.read();
        if temp < 0 { panic!("EOFException"); }
        temp
    }

    // Java: public short readShort() throws IOException
    pub fn readShort(&mut self) -> i16 {
        let mut scratch = vec![0u8; 8];
        self.readFully(&mut scratch, 0, 2);
        // Java: return Memory.peekShort(this.mScratch, 0, ByteOrder.BIG_ENDIAN);
        Memory::peekShort(&scratch, 0, ByteOrder::BIG_ENDIAN)
    }

    // Java: public int readUnsignedShort() throws IOException
    pub fn readUnsignedShort(&mut self) -> i32 {
        // Java: return this.readShort() & '\uffff';
        (self.readShort() as i32) & 0xFFFF
    }

    // Java: public char readChar() throws IOException
    pub fn readChar(&mut self) -> u16 { self.readShort() as u16 }

    // Java: public int readInt() throws IOException
    pub fn readInt(&mut self) -> i32 {
        let mut scratch = vec![0u8; 8];
        self.readFully(&mut scratch, 0, 4);
        Memory::peekInt(&scratch, 0, ByteOrder::BIG_ENDIAN)
    }

    // Java: public long readLong() throws IOException
    pub fn readLong(&mut self) -> i64 {
        let mut scratch = vec![0u8; 8];
        self.readFully(&mut scratch, 0, 8);
        Memory::peekLong(&scratch, 0, ByteOrder::BIG_ENDIAN)
    }

    // Java: public float readFloat() throws IOException
    pub fn readFloat(&mut self) -> f32 { f32::from_bits(self.readInt() as u32) }

    // Java: public double readDouble() throws IOException
    pub fn readDouble(&mut self) -> f64 { f64::from_bits(self.readLong() as u64) }

    // Java: public String readLine() throws IOException
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

    // Java: public void write(byte[] buffer, int byteOffset, int byteCount) throws IOException
    pub fn write(&mut self, _buffer: &[u8], _byteOffset: i32, _byteCount: i32) {
        panic!("UnsupportedOperationException");
    }

    // Java: public void write(int oneByte) throws IOException
    pub fn write__int(&mut self, _oneByte: i32) {
        panic!("UnsupportedOperationException");
    }

    // Java: private boolean nextBlock() throws IOException
    fn nextBlock(&mut self) -> bool {
        // Java: if (null == this.mIs) return false;
        if self.mIs.is_none() { return false; }
        if let Some(ref mut is) = self.mIs {
            let mut buf = vec![0u8; BLOCK_SIZE];
            match is.read(&mut buf) {
                Ok(0) | Err(_) => {
                    self.closeInputStream();
                    return false;
                }
                Ok(count) => {
                    // Java: File file = KfsFileUtil.convertOrgToKfsFile(PlatformAdapter.createTempFile("block", ".tmp"));
                    if let Some(file) = PlatformAdapter::createTempFile("block", ".tmp") {
                        let mut fos = FileOutputStream::new(file.getPath());
                        self.mLength += count as i64;
                        // Java: fos.write(this.mBuff, 0, count);
                        fos.write__bytearr_int_int(&buf, 0, count as i32);
                        // Java: this.mBlockPaths.add(file.getAbsolutePath());
                        self.mBlockPaths.push(file.getAbsolutePath());
                        fos.close();
                    }
                    return true;
                }
            }
        }
        false
    }

    // Java: private void closeInputStream()
    fn closeInputStream(&mut self) {
        // Java: this.mIs = null;
        self.mIs = None;
    }
}
