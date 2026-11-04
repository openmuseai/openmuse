#![allow(non_snake_case, dead_code, unused_variables, unused_imports, unused_mut, unused_assignments, unused_parens)]

//! Java `cn.wps.moffice.kfs.zip.ZipInputStream` 的 Rust 侧。
//!
//! 与 Java 对齐的三条 IO 机制（Rust 侧的平台映射）：
//!   1. `new PushbackInputStream(stream, 512)`            → `PushbackInputStream`（本地实现）
//!      —— `closeEntry()` 回推“读多了的尾巴”必须靠它，否则下一个 entry 头错位；
//!   2. `InflaterInputStream` + `new Inflater(true)`      → 拍平的 `inf`/`buf`/`len` 字段 +
//!      `Inflater`（raw deflate），`read()` 的 method=8 分支真正解压；
//!   3. `java.util.zip.CRC32 crc`                          → `flate2::Crc`，`read()` 里逐块 update，
//!      `readAndVerifyDataDescriptor()` 里参与校验。
//!
//! 两层结构（见 `ZipErrors.rs`）：`try_xxx` 核心返回 `std::io::Result`；与 Java 同签名的
//! `xxx` 是薄外壳，遇 IOException 谱系的错误 panic，保持 Java 抛出语义与 1:1 可对照性。
//!
//! 仍未进入 Result 通道的 Java 异常（会在核心路径上 panic）：
//!   * `ModifiedUtf8.decode` 的 `UTFDataFormatException`（坏文件名编码）——
//!     `ModifiedUtf8::try_decode` 已提供，此处接上即可；
//!   * `Zip64.parseZip64ExtendedInfo` 的 `ZipException`；
//!   * `ZipEntry.validateStringLength/setCrc/setSize/setMethod` 的 `IllegalArgumentException`。

use flate2::Crc;
use std::io::{self, Read};

use crate::kfs::mfs::Memory::ByteOrder;
use crate::kfs::mfs::Memory::Memory;
use crate::kfs::zip::Inflater::Inflater;
use crate::kfs::zip::InflaterInputStream::BUF_SIZE;
use crate::kfs::zip::ModifiedUtf8::ModifiedUtf8;
use crate::kfs::zip::PushbackInputStream::PushbackInputStream;
use crate::kfs::zip::Streams::Streams;
use crate::kfs::zip::Zip64::Zip64;
use crate::kfs::zip::ZipEntry::ZipEntry;
use crate::kfs::zip::ZipErrors;

// Java field:    private static final int ZIPLocalHeaderVersionNeeded = 20;
const ZIPLocalHeaderVersionNeeded: i32 = 20;

// Java: public class ZipInputStream extends InflaterInputStream implements ZipConstants
//
// Rust: Java 的继承在这里“拍平”——下面 5 个字段来自 InflaterInputStream（Java 里 ZipInputStream
// 也是直接以 this.inf / this.buf / this.len / this.closed / this.eof 访问它们），字段名保持一致。
// 之所以不写成 `super_: InflaterInputStream`：Java 的 ZipInputStream.read() 完全覆盖了父类实现，
// 并且要直接驱动父类的 inf/buf/len；用一个被拆开的组合结构反而会引入“该用哪份 input”的歧义。
pub struct ZipInputStream {
    // Java field (InflaterInputStream):    protected Inflater inf;
    inf: Inflater,
    // Java field (InflaterInputStream):    protected byte[] buf;
    buf: Vec<u8>,
    // Java field (InflaterInputStream):    protected int len;
    len: i32,
    // Java field (InflaterInputStream):    boolean closed;
    closed: bool,
    // Java field (InflaterInputStream):    boolean eof;
    // Java 的 ZipInputStream.read() 也只在 method=0 分支写它，不参与判断（保持一致，勿当成漏用）
    eof: bool,
    // Java field: FilterInputStream.in —— Java 侧是 super(new PushbackInputStream(stream, 512), ...)
    input: PushbackInputStream,

    // Java field:    private boolean entriesEnd = false;
    entriesEnd: bool,
    // Java field:    private boolean hasDD = false;
    hasDD: bool,
    // Java field:    private int entryIn = 0;
    entryIn: i32,
    // Java field:    private int inRead;
    inRead: i32,
    // Java field:    private int lastRead = 0;
    lastRead: i32,
    // Java field:    private ZipEntry currentEntry;
    currentEntry: Option<ZipEntry>,
    // Java field:    private boolean currentEntryIsZip64;
    currentEntryIsZip64: bool,
    // Java field:    private final byte[] hdrBuf = new byte[34];
    hdrBuf: Vec<u8>,
    // Java field:    private final CRC32 crc = new CRC32();
    // Rust: flate2::Crc 提供与 java.util.zip.CRC32 相同的 CRC-32 与 update/sum 语义
    crc: Crc,
    // Java field:    private byte[] stringBytesBuf = new byte[256];
    stringBytesBuf: Vec<u8>,
    // Java field:    private char[] stringCharBuf = new char[256];
    stringCharBuf: Vec<u16>,
}

// Java: public class ZipInputStream extends InflaterInputStream implements ZipConstants
impl ZipInputStream {
    // Java: public ZipInputStream(InputStream stream)
    pub fn new(stream: Box<dyn Read>) -> ZipInputStream {
        // Java: super(new PushbackInputStream(stream, 512), new Inflater(true));
        // Java: if (stream == null) throw new NullPointerException("stream == null");
        // Rust: Box<dyn Read> 非空，null 由类型系统排除。
        // Rust: PushbackInputStream 的容量必须 >= InflaterInputStream 的读缓冲（都是 512），
        //       这样 closeEntry() 的 unread(diff) 才永远装得下。
        let input = PushbackInputStream::new__inputstream_int(stream, BUF_SIZE);
        Self {
            // Java: new Inflater(true) —— nowrap == true == raw deflate
            inf: Inflater::new(true),
            buf: vec![0u8; BUF_SIZE as usize],
            len: 0,
            closed: false,
            eof: false,
            input,
            entriesEnd: false,
            hasDD: false,
            entryIn: 0,
            inRead: 0,
            lastRead: 0,
            currentEntry: None,
            currentEntryIsZip64: false,
            hdrBuf: vec![0u8; 34],
            crc: Crc::new(),
            stringBytesBuf: vec![0u8; 256],
            stringCharBuf: vec![0u16; 256],
        }
    }

    // Java: public void close() throws IOException
    pub fn close(&mut self) {
        self.try_close().unwrap_or_else(|e| ZipErrors::panicLikeJava(&e));
    }

    // Rust-native: close 的可恢复版本
    pub fn try_close(&mut self) -> io::Result<()> {
        // Java: if (!this.closed)
        if !self.closed {
            // Java: this.closeEntry();
            self.try_closeEntry()?;
            // Java: super.close();
            //   → InflaterInputStream.close(): this.inf.end(); this.closed = true; this.eof = true;
            //   → FilterInputStream.close(): this.in.close()
            self.inf.end();
            self.closed = true;
            self.eof = true;
            // Rust: std::io::Read 没有 close()，这里释放底层流的所有权
            self.input.close()?;
        }
        Ok(())
    }

    // Java: public void closeEntry() throws IOException
    pub fn closeEntry(&mut self) {
        self.try_closeEntry().unwrap_or_else(|e| ZipErrors::panicLikeJava(&e));
    }

    // Rust-native: closeEntry 的可恢复版本
    pub fn try_closeEntry(&mut self) -> io::Result<()> {
        // Java: this.checkClosed();
        self.try_checkClosed()?;
        // Java: if (this.currentEntry != null)
        if self.currentEntry.is_some() {
            // Java: Exception failure = null;
            let mut failure: Option<io::Error> = None;

            // Java: try { Streams.skipAll(this); } catch (Exception e) { failure = e; }
            if let Err(e) = self.try_skipAllCurrentEntry() {
                failure = Some(e);
            }

            // Java: int inB; int out;
            let (inB, out);
            // Java: if (this.currentEntry.compressionMethod == 8)
            if self.currentEntry.as_ref().unwrap().compressionMethod == 8 {
                // Java: inB = this.inf.getTotalIn(); out = this.inf.getTotalOut();
                inB = self.inf.getTotalIn();
                out = self.inf.getTotalOut();
            } else {
                // Java: inB = this.inRead; out = this.inRead;
                inB = self.inRead;
                out = self.inRead;
            }

            // Java: int diff = this.entryIn - inB;
            let diff = self.entryIn - inB;
            // Java: if (diff != 0) ((PushbackInputStream)this.in).unread(this.buf, this.len - diff, diff);
            if diff != 0 {
                if diff < 0 || diff > self.len {
                    // Java: PushbackInputStream.unread 在这种输入下会抛 IOException（回推缓冲越界）
                    if failure.is_none() {
                        failure = Some(ZipErrors::ioException(format!(
                            "Push back buffer is full: diff={}, len={}",
                            diff, self.len
                        )));
                    }
                } else {
                    let start = (self.len - diff) as usize;
                    let end = self.len as usize;
                    // Rust: 借用拆分——input 与 buf 是不同字段
                    if let Err(e) = self
                        .input
                        .unread__bytearr_int_int(&self.buf[start..end], 0, diff)
                    {
                        if failure.is_none() {
                            failure = Some(e);
                        }
                    }
                }
            }

            // Java: try { this.readAndVerifyDataDescriptor((long)inB, (long)out, this.currentEntryIsZip64); }
            //       catch (Exception e) { if (failure == null) failure = e; }
            if let Err(e) = self.try_readAndVerifyDataDescriptor(
                inB as i64,
                out as i64,
                self.currentEntryIsZip64,
            ) {
                if failure.is_none() {
                    failure = Some(e);
                }
            }

            // Java: this.inf.reset();
            self.inf.reset();
            // Java: this.lastRead = this.inRead = this.entryIn = this.len = 0;
            self.lastRead = 0;
            self.inRead = 0;
            self.entryIn = 0;
            self.len = 0;
            // Java: this.crc.reset();
            self.crc = Crc::new();
            // Java: this.currentEntry = null;
            self.currentEntry = None;
            // Java: if (failure != null) { if (failure instanceof IOException) throw (IOException)failure;
            //        else if (failure instanceof RuntimeException) throw (RuntimeException)failure;
            //        else { AssertionError error = new AssertionError(); error.initCause(failure); throw error; } }
            // Rust: 本实现里非 IOException 谱系的失败都以 panic 表达（见 ZipErrors.rs 的约定），
            //       所以这里只需要把第一个 IOException 谱系的失败原样抛出。
            if let Some(e) = failure {
                return Err(e);
            }
        }
        Ok(())
    }

    // Rust-native: Java `Streams.skipAll(this)` 的等价实现。
    // 注意对象是 **this**（ZipInputStream）而不是底层流：skipAll 的落点是“当前 entry 数据读完”，
    // 读穿底层流会把后续 entry 和中央目录一起吃掉（旧实现在这里就是错的）。
    fn try_skipAllCurrentEntry(&mut self) -> io::Result<()> {
        // Java: do { in.skip(Long.MAX_VALUE); } while (in.read() != -1);
        loop {
            // Java: in.skip(Long.MAX_VALUE) → InflaterInputStream.skip → Streams.skipByReading(this, Long.MAX_VALUE)
            Streams::try_skipByReading(self, i64::MAX)?;
            // Java: ... while (in.read() != -1)
            let mut one = [0u8; 1];
            // Rust: Java 用 `!= -1`；read 返回 0 时 Java 会活锁，这里把 0 也当结束
            if self.try_read(&mut one, 0, 1)? <= 0 {
                break;
            }
        }
        Ok(())
    }

    // Java: private void readAndVerifyDataDescriptor(long inB, long out, boolean isZip64) throws IOException
    fn readAndVerifyDataDescriptor(&mut self, inB: i64, out: i64, isZip64: bool) {
        self.try_readAndVerifyDataDescriptor(inB, out, isZip64)
            .unwrap_or_else(|e| ZipErrors::panicLikeJava(&e));
    }

    // Rust-native: readAndVerifyDataDescriptor 的可恢复版本
    fn try_readAndVerifyDataDescriptor(
        &mut self,
        inB: i64,
        out: i64,
        isZip64: bool,
    ) -> io::Result<()> {
        // Java: if (this.hasDD)
        if self.hasDD {
            // Java: if (isZip64) { Streams.readFully(this.in, this.hdrBuf, 0, 24); }
            // Java: else { Streams.readFully(this.in, this.hdrBuf, 0, 16); }
            if isZip64 {
                Streams::try_readFully(&mut self.input, &mut self.hdrBuf, 0, 24)?;
            } else {
                Streams::try_readFully(&mut self.input, &mut self.hdrBuf, 0, 16)?;
            }

            // Java: int sig = Memory.peekInt(this.hdrBuf, 0, ByteOrder.LITTLE_ENDIAN);
            let sig = Memory::peekInt(&self.hdrBuf, 0, ByteOrder::LittleEndian);
            // Java: if (sig != 134695760) throw new ZipException(String.format("unknown format (EXTSIG=%x)", sig));
            if sig != 134695760 {
                return Err(ZipErrors::zipException(format!(
                    "unknown format (EXTSIG={:x})",
                    sig
                )));
            }

            // Java: this.currentEntry.crc = (long)Memory.peekInt(this.hdrBuf, 4, ByteOrder.LITTLE_ENDIAN) & 4294967295L;
            // Java 直接解引用 currentEntry（null 会 NPE）——这里保持同样的前置条件，不做静默跳过
            let ce = self
                .currentEntry
                .as_mut()
                .expect("NullPointerException: currentEntry == null");
            ce.crc = (Memory::peekInt(&self.hdrBuf, 4, ByteOrder::LittleEndian) as i64) & 4294967295;
            // Java: if (isZip64)
            if isZip64 {
                // Java: this.currentEntry.compressedSize = Memory.peekLong(this.hdrBuf, 8, ByteOrder.LITTLE_ENDIAN);
                ce.compressedSize = Memory::peekLong(&self.hdrBuf, 8, ByteOrder::LittleEndian);
                // Java: this.currentEntry.size = Memory.peekLong(this.hdrBuf, 16, ByteOrder.LITTLE_ENDIAN);
                ce.size = Memory::peekLong(&self.hdrBuf, 16, ByteOrder::LittleEndian);
            } else {
                // Java: this.currentEntry.compressedSize = (long)Memory.peekInt(this.hdrBuf, 8, ByteOrder.LITTLE_ENDIAN) & 4294967295L;
                ce.compressedSize =
                    (Memory::peekInt(&self.hdrBuf, 8, ByteOrder::LittleEndian) as i64) & 4294967295;
                // Java: this.currentEntry.size = (long)Memory.peekInt(this.hdrBuf, 12, ByteOrder.LITTLE_ENDIAN) & 4294967295L;
                ce.size =
                    (Memory::peekInt(&self.hdrBuf, 12, ByteOrder::LittleEndian) as i64) & 4294967295;
            }
        }

        let ce = self
            .currentEntry
            .as_ref()
            .expect("NullPointerException: currentEntry == null");
        // Java: if (this.currentEntry.crc != this.crc.getValue())
        if ce.crc != (self.crc.sum() as i64) {
            // Java: throw new ZipException("CRC mismatch");
            return Err(ZipErrors::zipException("CRC mismatch"));
        }
        // Java: else if (this.currentEntry.compressedSize != inB || this.currentEntry.size != out)
        else if ce.compressedSize != inB || ce.size != out {
            // Java: throw new ZipException("Size mismatch");
            // 旧实现把这里的校验注释掉了（"don't panic in all cases"），于是损坏的归档会静默通过。
            return Err(ZipErrors::zipException("Size mismatch"));
        }
        Ok(())
    }

    // Java: public ZipEntry getNextEntry() throws IOException
    pub fn getNextEntry(&mut self) -> Option<ZipEntry> {
        self.try_getNextEntry()
            .unwrap_or_else(|e| ZipErrors::panicLikeJava(&e))
    }

    // Rust-native: getNextEntry 的可恢复版本
    pub fn try_getNextEntry(&mut self) -> io::Result<Option<ZipEntry>> {
        // Java: this.closeEntry();
        self.try_closeEntry()?;
        // Java: if (this.entriesEnd) { return null; }
        if self.entriesEnd {
            return Ok(None);
        }

        // Java: Streams.readFully(this.in, this.hdrBuf, 0, 4);
        Streams::try_readFully(&mut self.input, &mut self.hdrBuf, 0, 4)?;
        // Java: int hdr = Memory.peekInt(this.hdrBuf, 0, ByteOrder.LITTLE_ENDIAN);
        let hdr = Memory::peekInt(&self.hdrBuf, 0, ByteOrder::LittleEndian);
        // Java: if ((long)hdr == 33639248L) { this.entriesEnd = true; return null; }
        if (hdr as i64) == 33639248 {
            self.entriesEnd = true;
            return Ok(None);
        }
        // Java: else if ((long)hdr != 67324752L) { return null; }
        if (hdr as i64) != 67324752 {
            return Ok(None);
        }

        // Java: Streams.readFully(this.in, this.hdrBuf, 0, 26);
        Streams::try_readFully(&mut self.input, &mut self.hdrBuf, 0, 26)?;
        // Java: int version = this.peekShort(0) & 255;
        let version = self.peekShort(0) & 255;
        // Java: if (version > 20) throw new ZipException("Cannot read local header version " + version);
        if version > ZIPLocalHeaderVersionNeeded {
            return Err(ZipErrors::zipException(format!(
                "Cannot read local header version {}",
                version
            )));
        }

        // Java: int flags = this.peekShort(2);
        let flags = self.peekShort(2);
        // Java: if ((flags & 1) != 0) throw new ZipException("Invalid General Purpose Bit Flag: " + flags);
        if (flags & 1) != 0 {
            return Err(ZipErrors::zipException(format!(
                "Invalid General Purpose Bit Flag: {}",
                flags
            )));
        }

        // Java: this.hasDD = (flags & 8) != 0;
        self.hasDD = (flags & 8) != 0;
        // Java: int ceLastModifiedTime = this.peekShort(6);
        let ceLastModifiedTime = self.peekShort(6);
        // Java: int ceLastModifiedDate = this.peekShort(8);
        let ceLastModifiedDate = self.peekShort(8);
        // Java: int ceCompressionMethod = this.peekShort(4);
        let ceCompressionMethod = self.peekShort(4);
        // Java: long ceCrc = 0L; long ceCompressedSize = 0L; long ceSize = -1L;
        // Java: ceCrc = (long)Memory.peekInt(this.hdrBuf, 10, ByteOrder.LITTLE_ENDIAN) & 4294967295L;
        let ceCrc: i64 =
            (Memory::peekInt(&self.hdrBuf, 10, ByteOrder::LittleEndian) as i64) & 4294967295;
        // Java: ceCompressedSize = (long)Memory.peekInt(this.hdrBuf, 14, ByteOrder.LITTLE_ENDIAN) & 4294967295L;
        let ceCompressedSize: i64 =
            (Memory::peekInt(&self.hdrBuf, 14, ByteOrder::LittleEndian) as i64) & 4294967295;
        // Java: ceSize = (long)Memory.peekInt(this.hdrBuf, 18, ByteOrder.LITTLE_ENDIAN) & 4294967295L;
        let ceSize: i64 =
            (Memory::peekInt(&self.hdrBuf, 18, ByteOrder::LittleEndian) as i64) & 4294967295;

        // Java: int nameLength = this.peekShort(22);
        let nameLength = self.peekShort(22);
        // Java: if (nameLength == 0) throw new ZipException("Entry is not named");
        if nameLength == 0 {
            return Err(ZipErrors::zipException("Entry is not named"));
        }

        // Java: int extraLength = this.peekShort(24);
        let extraLength = self.peekShort(24) as usize;
        // Java: String name = this.readString(nameLength);
        let name = self.try_readString(nameLength as usize)?;
        // Java: this.currentEntry = this.createZipEntry(name);
        let mut entry = self.createZipEntry(name);
        // Java: this.currentEntry.time = ceLastModifiedTime;
        entry.time = ceLastModifiedTime;
        // Java: this.currentEntry.modDate = ceLastModifiedDate;
        entry.modDate = ceLastModifiedDate;
        // Java: this.currentEntry.setMethod(ceCompressionMethod);
        entry.setMethod(ceCompressionMethod);
        // Java: if (ceSize != -1L)
        // Rust: Java 里 ceSize 已经 & 4294967295L，所以这个条件恒为真（旧实现写成
        //       `ceSize != 0xFFFFFFFF`，导致 ZIP64 本地头（字段就是 0xFFFFFFFF）时整个赋值被跳过）。
        if ceSize != -1 {
            // Java: this.currentEntry.setCrc(ceCrc);
            entry.setCrc(ceCrc);
            // Java: this.currentEntry.setSize(ceSize);
            entry.setSize(ceSize);
            // Java: this.currentEntry.setCompressedSize(ceCompressedSize);
            entry.setCompressedSize(ceCompressedSize);
        }

        // Java: if (extraLength > 0)
        if extraLength > 0 {
            // Java: byte[] extraData = new byte[extraLength];
            let mut extraData = vec![0u8; extraLength];
            // Java: Streams.readFully(this.in, extraData, 0, extraLength);
            Streams::try_readFully(&mut self.input, &mut extraData, 0, extraLength)?;
            // Java: this.currentEntry.setExtra(extraData);
            entry.setExtra(Some(extraData));
            // Java: this.currentEntryIsZip64 = Zip64.parseZip64ExtendedInfo(this.currentEntry, false);
            // Rust: 走 Zip64 的 try_ 核心 —— 「本地头声称有 zip64 记录、实际没有」时
            //       由 try_getNextEntry 返回 Err，而不是在内部先 panic。
            self.currentEntryIsZip64 = Zip64::try_parseZip64ExtendedInfo(&mut entry, false)?;
        } else {
            // Java: this.currentEntryIsZip64 = false;
            self.currentEntryIsZip64 = false;
        }

        // Java: return this.currentEntry;
        // Rust: Java 返回的是同一个 ZipEntry 对象（流式 entry 的 size/crc 会在 closeEntry 时被
        //       数据描述符回填，持引用的一方能看到）；Rust 无法共享可变所有权，这里返回副本并让
        //       currentEntry 自己演进。当前唯一调用方（wps-io 的 ZipInputStreamZipEntrySource）
        //       只取 name+数据，不依赖这个身份，若要严格对齐需改成 Rc<RefCell<ZipEntry>>。
        self.currentEntry = Some(entry.clone());
        Ok(Some(entry))
    }

    // Java: private String readString(int byteLength) throws IOException
    fn readString(&mut self, byteLength: usize) -> String {
        self.try_readString(byteLength)
            .unwrap_or_else(|e| ZipErrors::panicLikeJava(&e))
    }

    // Rust-native: readString 的可恢复版本
    fn try_readString(&mut self, byteLength: usize) -> io::Result<String> {
        // Java: if (byteLength > this.stringBytesBuf.length)
        if byteLength > self.stringBytesBuf.len() {
            // Java: this.stringBytesBuf = new byte[byteLength];
            self.stringBytesBuf = vec![0u8; byteLength];
        }

        // Java: Streams.readFully(this.in, this.stringBytesBuf, 0, byteLength);
        Streams::try_readFully(&mut self.input, &mut self.stringBytesBuf, 0, byteLength)?;
        // Java: if (byteLength > this.stringCharBuf.length)
        if byteLength > self.stringCharBuf.len() {
            // Java: this.stringCharBuf = new char[byteLength];
            self.stringCharBuf = vec![0u16; byteLength];
        }

        // Java: return ModifiedUtf8.decode(this.stringBytesBuf, this.stringCharBuf, 0, byteLength);
        // Rust: try_decode 把 UTFDataFormatException 变成 io::Error（旧实现直接 panic）
        ModifiedUtf8::try_decode(&self.stringBytesBuf, &mut self.stringCharBuf, 0, byteLength)
    }

    // Java: private int peekShort(int offset)
    fn peekShort(&self, offset: i32) -> i32 {
        // Java: return Memory.peekShort(this.hdrBuf, offset, ByteOrder.LITTLE_ENDIAN) & '\uffff';
        (Memory::peekShort(&self.hdrBuf, offset, ByteOrder::LittleEndian) as i32) & 0xFFFF
    }

    // Java: public int read(byte[] buffer, int byteOffset, int byteCount) throws IOException
    pub fn read(&mut self, buffer: &mut [u8], byteOffset: usize, byteCount: usize) -> i32 {
        self.try_read(buffer, byteOffset, byteCount)
            .unwrap_or_else(|e| ZipErrors::panicLikeJava(&e))
    }

    // Rust-native: read 的可恢复版本
    pub fn try_read(
        &mut self,
        buffer: &mut [u8],
        byteOffset: usize,
        byteCount: usize,
    ) -> io::Result<i32> {
        // Java: this.checkClosed();
        self.try_checkClosed()?;
        // Java: ArrayUtils.checkOffsetAndCount(buffer.length, byteOffset, byteCount);
        // Java: ArrayIndexOutOfBoundsException（RuntimeException）→ panic
        if byteOffset > buffer.len() || buffer.len() - byteOffset < byteCount {
            panic!("ArrayIndexOutOfBoundsException");
        }
        // Java: if (!this.inf.finished() && this.currentEntry != null)
        if !self.inf.finished() && self.currentEntry.is_some() {
            // Java: if (this.currentEntry.compressionMethod == 0)
            if self.currentEntry.as_ref().unwrap().compressionMethod == 0 {
                // Java: int csize = (int)this.currentEntry.size;
                let csize = self.currentEntry.as_ref().unwrap().size as i32;
                // Java: if (this.inRead >= csize) { return -1; }
                if self.inRead >= csize {
                    return Ok(-1);
                }

                // Java: if (this.lastRead >= this.len)
                if self.lastRead >= self.len {
                    // Java: this.lastRead = 0;
                    self.lastRead = 0;
                    // Java: if ((this.len = this.in.read(this.buf)) == -1) { this.eof = true; return -1; }
                    let read = self.input.read__bytearr(&mut self.buf)?;
                    self.len = read;
                    if self.len == -1 {
                        self.eof = true;
                        return Ok(-1);
                    }
                    // Java: this.entryIn += this.len;
                    self.entryIn += self.len;
                }

                // Java: int toRead = byteCount > this.len - this.lastRead ? this.len - this.lastRead : byteCount;
                let mut toRead = if (byteCount as i32) > self.len - self.lastRead {
                    (self.len - self.lastRead) as usize
                } else {
                    byteCount
                };
                // Java: if (csize - this.inRead < toRead) { toRead = csize - this.inRead; }
                if csize - self.inRead < toRead as i32 {
                    toRead = (csize - self.inRead) as usize;
                }

                // Java: System.arraycopy(this.buf, this.lastRead, buffer, byteOffset, toRead);
                buffer[byteOffset..byteOffset + toRead].copy_from_slice(
                    &self.buf[self.lastRead as usize..self.lastRead as usize + toRead],
                );
                // Java: this.lastRead += toRead;
                self.lastRead += toRead as i32;
                // Java: this.inRead += toRead;
                self.inRead += toRead as i32;
                // Java: this.crc.update(buffer, byteOffset, toRead);
                // Rust: 旧实现漏了 CRC 更新，导致 closeEntry 的 CRC 校验永远不可能通过
                self.crc.update(&buffer[byteOffset..byteOffset + toRead]);
                // Java: return toRead;
                return Ok(toRead as i32);
            } else {
                // Java: if (this.inf.needsInput()) { this.fill(); if (this.len > 0) { this.entryIn += this.len; } }
                if self.inf.needsInput() {
                    self.try_fill()?;
                    if self.len > 0 {
                        self.entryIn += self.len;
                    }
                }

                // Java: read = this.inf.inflate(buffer, byteOffset, byteCount);
                // Java: catch (DataFormatException e) { throw new ZipException(e.getMessage()); }
                let read = self.inf.inflate(buffer, byteOffset, byteCount)?;
                // Java: if (read == 0 && this.inf.finished()) { return -1; }
                // Rust: Java 只调用一次 inflate；若它返回 0，Java 会把这个 0 交给调用方
                //       （调用方的 while (read > 0) 会提前结束、静默截断 entry），这里在
                //       “本轮没产出但压缩流还没结束”时补一次尝试，避免这个数据丢失。
                let read = if read == 0 && !self.inf.finished() && self.inf.needsInput() {
                    self.try_fill()?;
                    if self.len > 0 {
                        self.entryIn += self.len;
                    }
                    self.inf.inflate(buffer, byteOffset, byteCount)?
                } else {
                    read
                };
                if read == 0 && self.inf.finished() {
                    return Ok(-1);
                }
                // Java: this.crc.update(buffer, byteOffset, read);
                self.crc
                    .update(&buffer[byteOffset..byteOffset + read as usize]);
                // Java: return read;
                return Ok(read);
            }
        } else {
            // Java: return -1;
            return Ok(-1);
        }
    }

    // Rust-native: InflaterInputStream.fill() 的等价实现。
    // Java 的 ZipInputStream 继承父类 fill()，而父类 fill() 读的是 `this.in`（同一个
    // PushbackInputStream）。Rust 里 pushback 由本结构持有，所以在这里就地实现同样的逻辑
    // （否则会出现“两份 input，inflater 吃到哪一份”的歧义）。
    fn try_fill(&mut self) -> io::Result<()> {
        // Java: this.checkClosed();
        self.try_checkClosed()?;
        // Java: if ((this.len = this.in.read(this.buf)) > 0) this.inf.setInput(this.buf, 0, this.len);
        let read = self.input.read__bytearr(&mut self.buf)?;
        self.len = read;
        if self.len > 0 {
            self.inf.setInput(&self.buf, 0, self.len);
        }
        Ok(())
    }

    // Java: public long skip(long byteCount) throws IOException
    // Java 的 ZipInputStream 从 InflaterInputStream 继承 skip，落点是 Streams.skipByReading(this, byteCount)，
    // 即“通过本流的 read 跳过”（会正常更新 inRead/entryIn/CRC），这里保持一致。
    // 注意：Rust 里 `stream.skip(n)` 会解析到这个 i64 版本；要用 std 的 Read::skip 需写 `Read::skip(&mut s, n)`。
    pub fn skip(&mut self, byteCount: i64) -> i64 {
        // Java: if (byteCount < 0L) throw new IllegalArgumentException("byteCount < 0");
        if byteCount < 0 {
            panic!("IllegalArgumentException: byteCount < 0");
        }
        Streams::skipByReading(self, byteCount)
    }

    // Java: public int available() throws IOException
    pub fn available(&self) -> i32 {
        // Java: this.checkClosed();
        self.checkClosed();
        // Java: return this.currentEntry != null && (long)this.inRead >= this.currentEntry.size ? 0 : 1;
        // Rust: 旧实现在 currentEntry == null 时返回 0，Java 是返回 1（条件整体为假）
        if self.currentEntry.is_some()
            && (self.inRead as i64) >= self.currentEntry.as_ref().unwrap().size
        {
            0
        } else {
            1
        }
    }

    // Java: protected ZipEntry createZipEntry(String name)
    pub fn createZipEntry(&self, name: String) -> ZipEntry {
        // Java: return new ZipEntry(name);
        ZipEntry::new__string(name)
    }

    // Java: private void checkClosed() throws IOException
    fn checkClosed(&self) {
        self.try_checkClosed()
            .unwrap_or_else(|e| ZipErrors::panicLikeJava(&e));
    }

    // Rust-native: checkClosed 的可恢复版本
    fn try_checkClosed(&self) -> io::Result<()> {
        // Java: if (this.closed) throw new IOException("Stream is closed");
        if self.closed {
            return Err(ZipErrors::ioException("Stream is closed"));
        }
        Ok(())
    }
}

// Rust-native: 让 ZipInputStream 能直接当 `Box<dyn Read>` / `&mut dyn Read` 用
// （`try_read` 的错误以 std::io::Error 形式抛出，Java 的 -1 映射成 Ok(0)）
impl Read for ZipInputStream {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        if buf.is_empty() {
            return Ok(0);
        }
        let ret = self.try_read(buf, 0, buf.len())?;
        if ret < 0 {
            Ok(0)
        } else {
            Ok(ret as usize)
        }
    }
}
