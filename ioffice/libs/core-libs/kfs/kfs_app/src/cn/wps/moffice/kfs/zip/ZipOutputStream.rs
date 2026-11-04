#![allow(
    non_snake_case,
    dead_code,
    unused_variables,
    unused_imports,
    unused_mut,
    unused_assignments,
    unused_parens
)]
// Java: import cn.wps.moffice.kfs.raw.EntryBase
use crate::kfs::raw::EntryBase::EntryBase;
// Java: import cn.wps.moffice.kfs.zip.Deflater
use crate::kfs::zip::Deflater::DeflateDataCallBack;
use crate::kfs::zip::Deflater::Deflater;
// Java: import cn.wps.moffice.kfs.zip.ZipEntry
use crate::kfs::zip::ZipEntry::ZipEntry;
// Rust-native: ZipErrors —— IOException 谱系错误的 Java 名字还原（见 ZipErrors.rs）
use crate::kfs::zip::ZipErrors;
// Java: import java.util.zip.CRC32
use flate2::Crc;
use std::collections::HashSet;
use std::io::Write;

// Java field:    public static final int DEFLATED = 8;
pub const DEFLATED: i32 = 8;
// Java field:    public static final int STORED = 0;
pub const STORED: i32 = 0;
// Java field:    protected static final int ZIPLocalHeaderVersionNeeded = 20;
pub const ZIPLocalHeaderVersionNeeded: i32 = 20;
// Java field:    protected static final int GPBF_DATA_DESCRIPTOR_FLAG = 8;
pub const GPBF_DATA_DESCRIPTOR_FLAG: i32 = 8;
// Java field:    protected static final int GPBF_UTF8_FLAG = 2048;
pub const GPBF_UTF8_FLAG: i32 = 2048;
// Java field:    private static final String TAG = null;
pub const TAG: Option<&str> = None;
// Java field:    public static final Charset UTF_8 = Charset.forName("UTF-8");
// Rust: 名字一律按 UTF-8 字节写（nameBytes = getName().as_bytes()），不再持有 Charset 对象
pub const UTF_8: &str = "UTF-8";

pub trait IListener {
    fn onUpdateEntry(&mut self, entry: &ZipEntry);
}

// Java: public class ZipOutputStream extends DeflaterOutputStream implements ZipConstants
pub struct ZipOutputStream {
    // Java field:    private String comment;
    comment: Option<String>,
    // Java field:    private final Set<String> entries = new HashSet();
    entries: HashSet<String>,
    // Java field:    private int compressMethod = 8;
    compressMethod: i32,
    // Java field:    private int compressLevel = -1;
    compressLevel: i32,
    // Java field:    protected ByteArrayOutputStream cDir = new ByteArrayOutputStream();
    pub cDir: Vec<u8>,
    // Java field:    protected ZipEntry currentEntry;
    pub currentEntry: Option<ZipEntry>,
    // Java field:    protected final CRC32 crc = new CRC32();
    crc: Crc,
    // Java field:    protected int offset = 0;
    pub offset: i32,
    // Java field:    protected int curOffset = 0;
    pub curOffset: i32,
    // Java field:    protected int nameLength;
    pub nameLength: i32,
    // Java field:    protected byte[] nameBytes;
    pub nameBytes: Option<Vec<u8>>,
    // Java field:    private IListener listener = null;
    listener: Option<Box<dyn IListener>>,
    // Java field:    protected long tbytes = 0L;
    pub tbytes: i64,
    // Java field:    protected byte[] buf;  ← 继承自 DeflaterOutputStream（deflate 的输出缓冲，默认 512）
    pub buf: Vec<u8>,
    // Java field:    private boolean done;  ← 继承自 DeflaterOutputStream 的“已 finish”闩锁
    //                ZipOutputStream.closeEntry() 通过反射 setDone(false) 复位它，让下一个 entry 能继续写。
    done: bool,
    out: Option<Box<dyn Write>>,
    def: Deflater,
    cDir_active: bool,
}

// Java: public class ZipOutputStream extends DeflaterOutputStream implements ZipConstants
impl ZipOutputStream {
    // Java: inherited FilterOutputStream.flush() delegates to the same out stream.
    pub fn try_flush(&mut self)->std::io::Result<()>{
        self.out.as_mut().ok_or_else(||ZipErrors::ioException("Stream is closed"))?.flush()
    }
    // Java: public ZipOutputStream(OutputStream p1)
    pub fn new(out: Box<dyn Write>) -> ZipOutputStream {
        Self {
            comment: None,
            entries: HashSet::new(),
            compressMethod: 8,
            compressLevel: -1,
            cDir: Vec::new(),
            currentEntry: None,
            crc: Crc::new(),
            offset: 0,
            curOffset: 0,
            nameLength: 0,
            nameBytes: None,
            listener: None,
            tbytes: 0,
            buf: vec![0u8; 512],
            done: false,
            out: Some(out),
            def: Deflater::new(-1, true),
            cDir_active: true,
        }
    }

    // Rust: try_ 核心 —— close 里的 finish() 会写 EOCD，写失败可以恢复
    pub fn try_close(&mut self) -> std::io::Result<()> {
        // Java: if (this.out != null)
        if self.out.is_some() {
            // Java: this.finish();
            self.try_finish()?;
            // Java: this.def.end();
            self.def.end();
            // Java: this.out.close();  → Rust: 释放 Box<dyn Write>（File 会随之关闭 fd）
            self.out = None;
        }
        Ok(())
    }

    // Java: public void close() throws IOException
    pub fn close(&mut self) {
        self.try_close()
            .unwrap_or_else(|e| ZipErrors::panicLikeJava(&e));
    }

    // Java: public void closeEntry() throws IOException
    pub fn try_closeEntry(&mut self) -> std::io::Result<()> {
        // Java: this.checkClosed();
        self.try_checkClosed()?;
        // Java: if (this.currentEntry != null)
        if self.currentEntry.is_none() {
            return Ok(());
        }

        let entry = self.currentEntry.as_ref().unwrap().clone();

        // Java: if (this.currentEntry.getMethod() == 8 && this.currentEntry.getMode() != 1) this.super.finish();
        // Rust: 注意 Java 这里调的是 **super**.finish()（DeflaterOutputStream.finish：把 deflate 尾巴刷出去），
        //       不是同名重载 ZipOutputStream.finish()（写 EOCD）。两个 finish 必须分开实现，见 finishDeflater()。
        if entry.getMethod() == 8 && entry.getMode() != 1 {
            self.try_finishDeflater()?;
        }

        // Java: if (this.currentEntry.getMethod() == 0 && this.currentEntry.getSize() != this.tbytes) throw new ZipException(...)
        if entry.getMethod() == 0 && entry.getSize() != self.tbytes {
            return Err(ZipErrors::zipException(format!(
                "{} size mismatch EntrySize is {}, tbytes is {}",
                entry.getName(),
                entry.getSize(),
                self.tbytes
            )));
        }

        // Java: this.curOffset = 30;
        self.curOffset = 30;
        // Java: this.curOffset += 16;
        self.curOffset += 16;

        // Java: writeLong(this.out, 134695760L);
        if let Some(ref mut out) = self.out {
            Self::try_writeLong_to(out.as_mut(), 134695760)?;
        }

        // Java: long lCrc = this.currentEntry.getMode() == 1 ? this.currentEntry.getCrc() : this.crc.getValue();
        let lCrc: i64 = if entry.getMode() == 1 {
            entry.getCrc()
        } else {
            // Java: this.crc.getValue()
            self.crc.sum() as i64
        };
        let lCrc = std::cmp::max(0, lCrc);

        {
            let ce = self.currentEntry.as_mut().unwrap();
            ce.setCrc(lCrc);
        }

        if let Some(ref mut out) = self.out {
            Self::try_writeLong_to(out.as_mut(), self.currentEntry.as_ref().unwrap().getCrc())?;
        }

        if entry.getMode() != 1 {
            let ce = self.currentEntry.as_mut().unwrap();
            if ce.getMethod() != 0 {
                ce.setCompressedSize(self.def.getTotalOut() as i64);
                ce.setSize(self.def.getTotalIn() as i64);
            } else {
                ce.setCompressedSize(self.tbytes);
                ce.setSize(self.tbytes);
            }
        }

        let compressed_size = self.currentEntry.as_ref().unwrap().getCompressedSize();
        let entry_size = self.currentEntry.as_ref().unwrap().getSize();
        if let Some(ref mut out) = self.out {
            Self::try_writeLong_to(out.as_mut(), compressed_size)?;
            Self::try_writeLong_to(out.as_mut(), entry_size)?;
        }

        let flags = self._flags();

        Self::writeLong_vec(&mut self.cDir, 33639248);
        Self::writeShort_vec(&mut self.cDir, 20);
        Self::writeShort_vec(&mut self.cDir, 20);
        Self::writeShort_vec(&mut self.cDir, flags);
        Self::writeShort_vec(
            &mut self.cDir,
            self.currentEntry.as_ref().unwrap().getMethod(),
        );
        Self::writeShort_vec(&mut self.cDir, self.currentEntry.as_ref().unwrap().time);
        Self::writeShort_vec(&mut self.cDir, self.currentEntry.as_ref().unwrap().modDate);
        Self::writeLong_vec(&mut self.cDir, self.currentEntry.as_ref().unwrap().getCrc());

        if self.currentEntry.as_ref().unwrap().getMethod() == 8 {
            self.curOffset = (self.curOffset as i64 + compressed_size) as i32;
            Self::writeLong_vec(&mut self.cDir, compressed_size);
            Self::writeLong_vec(&mut self.cDir, entry_size);
        } else {
            self.curOffset = (self.curOffset as i64 + self.tbytes) as i32;
            Self::writeLong_vec(&mut self.cDir, self.tbytes);
            Self::writeLong_vec(&mut self.cDir, self.tbytes);
        }

        self.curOffset += self.nameLength;
        Self::writeShort_vec(&mut self.cDir, self.nameLength);

        // Java: byte[] extra = this.currentEntry.getExtra();
        let extra = self.currentEntry.as_ref().unwrap().getExtra().cloned();
        if let Some(ref ex) = extra {
            self.curOffset += ex.len() as i32;
            Self::writeShort_vec(&mut self.cDir, ex.len() as i32);
        } else {
            Self::writeShort_vec(&mut self.cDir, 0);
        }

        let comment = self
            .currentEntry
            .as_ref()
            .unwrap()
            .getComment()
            .map(str::to_owned);
        if let Some(ref c) = comment {
            Self::writeShort_vec(&mut self.cDir, c.len() as i32);
        } else {
            Self::writeShort_vec(&mut self.cDir, 0);
        }

        Self::writeShort_vec(&mut self.cDir, 0);
        Self::writeShort_vec(&mut self.cDir, 0);
        Self::writeLong_vec(&mut self.cDir, 0);

        {
            let ce = self.currentEntry.as_mut().unwrap();
            ce.localHeaderRelOffset = self.offset as i64;
        }
        Self::writeLong_vec(
            &mut self.cDir,
            self.currentEntry.as_ref().unwrap().localHeaderRelOffset,
        );

        if let Some(ref nb) = self.nameBytes {
            self.cDir.extend_from_slice(nb);
        }
        self.nameBytes = None;

        if let Some(ref ex) = extra {
            self.cDir.extend_from_slice(ex);
        }

        self.offset += self.curOffset;

        if let Some(ref c) = comment {
            self.cDir.extend_from_slice(c.as_bytes());
        }

        if let Some(ref mut listener) = self.listener {
            listener.onUpdateEntry(self.currentEntry.as_ref().unwrap());
        }
        self.listener = None;
        self.currentEntry = None;
        // Java: this.clearDeflateDataCallBack();
        self.clearDeflateDataCallBack();
        // Java: this.crc.reset();
        self.crc = Crc::new();
        // Java: this.tbytes = 0L;
        self.tbytes = 0;
        // Java: this.def.reset();
        self.def.reset();
        // Java: this.setDone(false);  ← 复位 DeflaterOutputStream.done，让下一个 entry 还能写
        self.setDone(false);

        Ok(())
    }

    // Java: public void closeEntry() throws IOException
    pub fn closeEntry(&mut self) {
        self.try_closeEntry()
            .unwrap_or_else(|e| ZipErrors::panicLikeJava(&e));
    }

    // Java: public void finish() throws IOException
    pub fn try_finish(&mut self) -> std::io::Result<()> {
        if self.out.is_none() {
            return Err(ZipErrors::ioException("Stream is closed"));
        }
        if !self.cDir_active {
            return Ok(());
        }
        if self.entries.is_empty() {
            return Err(ZipErrors::zipException("No entries"));
        }

        if self.currentEntry.is_some() {
            self.try_closeEntry()?;
        }

        // Java: int cdirSize = this.cDir.size();
        let cdirSize = self.cDir.len() as i32;
        Self::writeLong_vec(&mut self.cDir, 101010256);
        Self::writeShort_vec(&mut self.cDir, 0);
        Self::writeShort_vec(&mut self.cDir, 0);
        Self::writeShort_vec(&mut self.cDir, self.entries.len() as i32);
        Self::writeShort_vec(&mut self.cDir, self.entries.len() as i32);
        Self::writeLong_vec(&mut self.cDir, cdirSize as i64);
        Self::writeLong_vec(&mut self.cDir, self.offset as i64);

        if let Some(ref comment) = self.comment {
            Self::writeShort_vec(&mut self.cDir, comment.len() as i32);
            self.cDir.extend_from_slice(comment.as_bytes());
        } else {
            Self::writeShort_vec(&mut self.cDir, 0);
        }

        if let Some(ref mut out) = self.out {
            // Java: this.out.write(this.cDir.toByteArray());
            out.write_all(&self.cDir)?;
        }
        self.cDir_active = false;

        Ok(())
    }

    // Java: public void finish() throws IOException
    pub fn finish(&mut self) {
        self.try_finish()
            .unwrap_or_else(|e| ZipErrors::panicLikeJava(&e));
    }

    // Java: public void putNextEntry(ZipEntry ze) throws IOException
    pub fn putNextEntry(&mut self, ze: ZipEntry) {
        self.putNextEntry__zipentry_listener_callback(ze, None, None);
    }

    /// Compatibility alias for generated call sites.
    pub fn putNextEntry_simple(&mut self, ze: ZipEntry) -> std::io::Result<()> {
        // Rust: 这里原本是“panic 外壳 + Ok(())”，等于把错误吞掉；现在直接走 try_ 核心
        self.try_putNextEntry__zipentry_listener_callback(ze, None, None)
    }

    // Java: public void putNextEntry(ZipEntry ze, IListener listener, DeflateDataCallBack deflateDataCallBack) throws IOException
    pub fn try_putNextEntry__zipentry_listener_callback(
        &mut self,
        mut ze: ZipEntry,
        listener: Option<Box<dyn IListener>>,
        deflateDataCallBack: Option<Box<dyn DeflateDataCallBack>>,
    ) -> std::io::Result<()> {
        if self.currentEntry.is_some() {
            self.try_closeEntry()?;
        }

        if ze.getMethod() == 0 || (self.compressMethod == 0 && ze.getMethod() == -1) {
            if ze.getCrc() == -1 {
                return Err(ZipErrors::zipException("CRC mismatch"));
            }
            if ze.getSize() == -1 && ze.getCompressedSize() == -1 {
                return Err(ZipErrors::zipException("Size mismatch"));
            }
            // Java: if (ze.getSize() != ze.getCompressedSize() && ze.getCompressedSize() != -1L && ze.getSize() != -1L)
            //         throw new ZipException("Size mismatch");
            if ze.getSize() != ze.getCompressedSize()
                && ze.getCompressedSize() != -1
                && ze.getSize() != -1
            {
                return Err(ZipErrors::zipException("Size mismatch"));
            }
        }

        self.try_checkClosed()?;

        if self.entries.contains(ze.getName()) {
            return Err(ZipErrors::zipException(format!(
                "Entry already exists: {}",
                ze.getName()
            )));
        }

        let name_bytes = ze.getName().as_bytes().to_vec();
        self.nameLength = name_bytes.len() as i32;
        self.nameBytes = Some(name_bytes);

        if self.nameLength > 65535 {
            panic!(
                "IllegalArgumentException: Name too long: {} UTF-8 bytes",
                self.nameLength
            );
        }

        self.def.setLevel(self.compressLevel);

        if ze.getMethod() == -1 {
            ze.setMethod(self.compressMethod);
        }

        // Java: this.currentEntry = ze; this.listener = listener; this.setDeflateDataCallBack(deflateDataCallBack);
        //       this.entries.add(this.currentEntry.getName());
        // Rust: 与 Java 同序 —— 先把 currentEntry/listener/回调装好，再算 flags 并写局部头
        self.entries.insert(ze.getName().to_string());
        self.listener = listener;
        // Java: this.setDeflateDataCallBack(deflateDataCallBack);
        self.setDeflateDataCallBack(deflateDataCallBack);
        self.currentEntry = Some(ze);

        // Java: int flags = this._flags();
        let flags = self._flags();

        // Java: if (this.currentEntry.getTime() == -1L) this.currentEntry.setTime(System.currentTimeMillis());
        {
            let ce = self.currentEntry.as_mut().unwrap();
            if ce.getTime() == -1 {
                ce.setTime(chrono::Local::now().timestamp_millis());
            }
        }

        // Java: if (method == 0) { size/csize 互为兜底 }，随后把局部头需要的字段一次取出
        let (method, crc, entry_size, time, modDate, extra) = {
            let ce = self.currentEntry.as_mut().unwrap();
            if ce.getMethod() == 0 {
                if ce.getSize() == -1 {
                    ce.setSize(ce.getCompressedSize());
                } else if ce.getCompressedSize() == -1 {
                    ce.setCompressedSize(ce.getSize());
                }
            }
            (
                ce.getMethod(),
                ce.getCrc(),
                ce.getSize(),
                ce.time,
                ce.modDate,
                ce.getExtra().cloned(),
            )
        };
        let name_bytes = self.nameBytes.clone();

        if let Some(ref mut out) = self.out {
            // Java: writeLong(this.out, 67324752L) —— 局部文件头签名
            Self::try_writeLong_to(out.as_mut(), 67324752)?;
            Self::try_writeShort_to(out.as_mut(), 20)?;
            Self::try_writeShort_to(out.as_mut(), flags)?;
            Self::try_writeShort_to(out.as_mut(), method)?;
            // Java: Pair dateTimepair = getTime(...); writeShort(out, pair.second /*time*/); writeShort(out, pair.first /*modDate*/);
            Self::try_writeShort_to(out.as_mut(), time)?;
            Self::try_writeShort_to(out.as_mut(), modDate)?;

            if method == 0 {
                // Java: writeLong(out, getCrc()); writeLong(out, getSize()); writeLong(out, getSize());
                Self::try_writeLong_to(out.as_mut(), crc)?;
                Self::try_writeLong_to(out.as_mut(), entry_size)?;
                Self::try_writeLong_to(out.as_mut(), entry_size)?;
            } else {
                // 压缩条目走数据描述符，局部头里这三个字段先填 0
                Self::try_writeLong_to(out.as_mut(), 0)?;
                Self::try_writeLong_to(out.as_mut(), 0)?;
                Self::try_writeLong_to(out.as_mut(), 0)?;
            }

            Self::try_writeShort_to(out.as_mut(), self.nameLength)?;

            if let Some(ref ex) = extra {
                Self::try_writeShort_to(out.as_mut(), ex.len() as i32)?;
            } else {
                Self::try_writeShort_to(out.as_mut(), 0)?;
            }

            if let Some(ref nb) = name_bytes {
                out.write_all(nb)?;
            }
            if let Some(ref ex) = extra {
                out.write_all(ex)?;
            }
        }


        Ok(())
    }

    // Java: public void putNextEntry(ZipEntry ze, IListener listener, DeflateDataCallBack deflateDataCallBack) throws IOException
    pub fn putNextEntry__zipentry_listener_callback(
        &mut self,
        ze: ZipEntry,
        listener: Option<Box<dyn IListener>>,
        deflateDataCallBack: Option<Box<dyn DeflateDataCallBack>>,
    ) {
        self.try_putNextEntry__zipentry_listener_callback(ze, listener, deflateDataCallBack)
            .unwrap_or_else(|e| ZipErrors::panicLikeJava(&e));
    }


    fn _flags(&self) -> i32 {
        let mut flags: i32 = 8;
        if let Some(ref ce) = self.currentEntry {
            if ce.getMode() == 1 {
                flags |= ce.getGpbf();
            }
        }
        flags |= 2048;
        flags
    }

    // Java: public void setComment(String comment)
    pub fn setComment(&mut self, comment: String) {
        if comment.len() > 65535 {
            panic!(
                "IllegalArgumentException: Comment too long: {} characters",
                comment.len()
            );
        }
        self.comment = Some(comment);
    }

    // Java: public void setLevel(int level)
    pub fn setLevel(&mut self, level: i32) {
        if level >= -1 && level <= 9 {
            self.compressLevel = level;
        } else {
            panic!("IllegalArgumentException");
        }
    }

    // Java: public void setMethod(int method)
    pub fn setMethod(&mut self, method: i32) {
        if method != 0 && method != 8 {
            panic!("IllegalArgumentException");
        }
        self.compressMethod = method;
    }

    // Rust: try_ 核心 —— 写失败（磁盘满/管道断）与“没有 active entry”都可以被 Rust 调用方恢复
    pub fn try_write_buf(
        &mut self,
        buffer: &[u8],
        offset: usize,
        byteCount: usize,
    ) -> std::io::Result<()> {
        // Java: checkOffsetAndCount(buffer.length, offset, byteCount);
        Self::checkOffsetAndCount(buffer.len() as i32, offset as i32, byteCount as i32);

        // Java: if (this.currentEntry == null) throw new ZipException("No active entry");
        let (mode, method) = match self.currentEntry {
            Some(ref ce) => (ce.getMode(), ce.getMethod()),
            None => return Err(ZipErrors::zipException("No active entry")),
        };

        if mode == 1 {
            // Java: this.out.write(buffer, offset, byteCount);
            // mode == 1 表示数据已经是压缩态（例如从别的 zip 直接搬运），透传且不动 CRC
            self.try_writeToOut(&buffer[offset..offset + byteCount])?;
        } else if method == 0 {
            // Java: this.out.write(buffer, offset, byteCount);
            self.try_writeToOut(&buffer[offset..offset + byteCount])?;
            // Java: this.crc.update(buffer, offset, byteCount);
            self.crc.update(&buffer[offset..offset + byteCount]);
        } else {
            // Java: super.write(buffer, offset, byteCount);  ← DeflaterOutputStream.write：喂给 deflater 并溢出压缩数据
            self.try_deflateWrite(buffer, offset, byteCount)?;
            // Java: this.crc.update(buffer, offset, byteCount);
            // CRC 始终算在 **未压缩** 数据上（旧实现从不更新 crc，导致 DD 与中央目录里的 CRC 恒为 0）
            self.crc.update(&buffer[offset..offset + byteCount]);
        }

        // Java: this.tbytes += (long)byteCount;
        self.tbytes += byteCount as i64;
        Ok(())
    }

    // Java: public void write(byte[] buffer, int offset, int byteCount) throws IOException
    pub fn write_buf(&mut self, buffer: &[u8], offset: usize, byteCount: usize) {
        self.try_write_buf(buffer, offset, byteCount)
            .unwrap_or_else(|e| ZipErrors::panicLikeJava(&e));
    }

    // Java: protected void deflate() throws IOException  ← DeflaterOutputStream.deflate()
    fn try_deflate(&mut self) -> std::io::Result<()> {
        // Java: int len = def.deflate(buf, 0, buf.length);
        // Rust: Java 风格的 (buf, off, len) 签名要先把长度取出来，否则 &mut self.buf 与 self.buf.len() 借用冲突
        let buf_len = self.buf.len() as i32;
        let produced = self.def.deflate__bytearr_int_int(&mut self.buf, 0, buf_len);
        // Java: if (len > 0) out.write(buf, 0, len);
        if produced > 0 {
            // Rust: 借用拆分（out 与 buf 是不同字段），避免 &mut self 与 &self.buf 冲突
            let Self { out, buf, .. } = self;
            if let Some(o) = out.as_mut() {
                o.write_all(&buf[..produced as usize])?;
            }
        }
        Ok(())
    }

    // Java: public void write(byte[] b, int off, int len) throws IOException  ← DeflaterOutputStream.write()
    fn try_deflateWrite(&mut self, b: &[u8], off: usize, len: usize) -> std::io::Result<()> {
        // Java: if (def.finished()) throw new IOException("write beyond end of stream");
        // Rust: JDK 里这道门禁来自 DeflaterOutputStream 的 done 闩锁 + def.finished()，两者都保留
        if self.def.finished() || self.done {
            return Err(ZipErrors::ioException("write beyond end of stream"));
        }
        // Java: if (len == 0) return;
        if len == 0 {
            return Ok(());
        }
        // Java: def.setInput(b, off, len);
        self.def.setInput(b, off as i32, len as i32);
        // Java: while (!def.needsInput()) { int len2 = def.deflate(buf, 0, buf.length); out.write(buf, 0, len2); }
        while !self.def.needsInput() {
            let buf_len = self.buf.len() as i32;
            let produced = self.def.deflate__bytearr_int_int(&mut self.buf, 0, buf_len);
            if produced > 0 {
                let Self { out, buf, .. } = self;
                if let Some(o) = out.as_mut() {
                    o.write_all(&buf[..produced as usize])?;
                }
            }
        }
        Ok(())
    }

    // Java: public void finish() throws IOException  ← DeflaterOutputStream.finish()
    // Rust: 与 ZipOutputStream.finish()（写 EOCD）**同名但不同义**。Java 靠 super.finish() 区分；
    //       Rust 里必须拆成两个方法，否则 closeEntry 会误写 EOCD。
    fn try_finishDeflater(&mut self) -> std::io::Result<()> {
        // Java: if (!def.finished())
        if self.def.finished() {
            return Ok(());
        }
        // Java: def.finish();
        self.def.finish();
        // Java: while (!def.finished()) { deflate(); }
        while !self.def.finished() {
            let before = self.def.getTotalOut();
            self.try_deflate()?;
            // Rust: JDK 侧由 deflate() 返回 0 兜底，这里显式防死循环
            if self.def.getTotalOut() == before && !self.def.finished() {
                break;
            }
        }
        // Java: done = true;
        self.done = true;
        Ok(())
    }

    // Java: private void setDeflateDataCallBack(DeflateDataCallBack deflateDataCallBack)
    fn setDeflateDataCallBack(&mut self, deflateDataCallBack: Option<Box<dyn DeflateDataCallBack>>) {
        // Java: if (this.def instanceof Deflater) { ((Deflater)this.def).setDeflateDataCallBack(deflateDataCallBack); }
        // Rust: self.def 已经是 kfs 的 Deflater（flate2 引擎 + 回调），不需要 instanceof 判断
        self.def.setDeflateDataCallBack(deflateDataCallBack);
    }

    // Java: public void clearDeflateDataCallBack()
    pub fn clearDeflateDataCallBack(&mut self) {
        // Java: this.setDeflateDataCallBack((DeflateDataCallBack)null);
        self.setDeflateDataCallBack(None);
    }

    // Java: protected final void setDone(boolean val)
    fn setDone(&mut self, val: bool) {
        // Java: Class<?> clazz = this.getClass().getSuperclass(); this.mDoneField = getField(clazz, "done");
        //       …也就是用反射去写 DeflaterOutputStream 的私有 done 字段。
        // Rust: 该字段已拍平为 ZipOutputStream.done（继承拍平，见文件头说明），不需要反射；
        //       Java 的 getField/mDoneField 只服务于这条反射路径，属于平台实现细节，不再移植。
        self.done = val;
    }

    // Rust-native: this.out.write(...) 的统一出口（Java 是 out.write，抛 IOException）
    fn try_writeToOut(&mut self, bytes: &[u8]) -> std::io::Result<()> {
        if let Some(ref mut out) = self.out {
            out.write_all(bytes)?;
        }
        Ok(())
    }

    // Java: private void checkClosed() throws IOException
    fn try_checkClosed(&self) -> std::io::Result<()> {
        if !self.cDir_active {
            return Err(ZipErrors::ioException("Stream is closed"));
        }
        Ok(())
    }

    // Java 签名外壳
    fn checkClosed(&self) {
        self.try_checkClosed()
            .unwrap_or_else(|e| ZipErrors::panicLikeJava(&e));
    }

    // Rust: try_ 核心 —— Java 的 `os.write(...)` 会抛 IOException，Rust 调用方可以真正恢复
    pub fn try_writeLong_to(out: &mut dyn Write, i: i64) -> std::io::Result<i64> {
        let bytes = [
            (i & 255) as u8,
            ((i >> 8) & 255) as u8,
            ((i >> 16) & 255) as u8,
            ((i >> 24) & 255) as u8,
        ];
        // Java: os.write(...) throws IOException —— 旧实现用 unwrap_or(()) 静默吞掉写失败，
        //        磁盘满/管道断时会产生“看起来写成功”的截断 zip；这里按 Java 语义返回 Err。
        out.write_all(&bytes)?;
        Ok(i)
    }

    pub fn try_writeShort_to(out: &mut dyn Write, i: i32) -> std::io::Result<i32> {
        let bytes = [(i & 255) as u8, ((i >> 8) & 255) as u8];
        out.write_all(&bytes)?;
        Ok(i)
    }

    // Java 签名外壳（protected static long writeLong / int writeShort）：抛 IOException
    pub fn writeLong_to(out: &mut dyn Write, i: i64) -> i64 {
        Self::try_writeLong_to(out, i).unwrap_or_else(|e| ZipErrors::panicLikeJava(&e))
    }

    pub fn writeShort_to(out: &mut dyn Write, i: i32) -> i32 {
        Self::try_writeShort_to(out, i).unwrap_or_else(|e| ZipErrors::panicLikeJava(&e))
    }

    fn writeLong_vec(v: &mut Vec<u8>, i: i64) {
        v.push((i & 255) as u8);
        v.push(((i >> 8) & 255) as u8);
        v.push(((i >> 16) & 255) as u8);
        v.push(((i >> 24) & 255) as u8);
    }

    fn writeShort_vec(v: &mut Vec<u8>, i: i32) {
        v.push((i & 255) as u8);
        v.push(((i >> 8) & 255) as u8);
    }

    // Java: static int utf8Count(String value)
    pub fn utf8Count(value: &str) -> i32 {
        value.as_bytes().len() as i32
    }

    // Java: static byte[] toUTF8Bytes(String value, int length)
    pub fn toUTF8Bytes(value: &str, _length: i32) -> Vec<u8> {
        value.as_bytes().to_vec()
    }

    // Java: protected static void checkOffsetAndCount(int arrayLength, int offset, int count)
    pub fn checkOffsetAndCount(arrayLength: i32, offset: i32, count: i32) {
        if (offset | count) < 0 || offset > arrayLength || arrayLength - offset < count {
            panic!(
                "ArrayIndexOutOfBoundsException: length={}; regionStart={}; regionLength={}",
                arrayLength, offset, count
            );
        }
    }
}
