#![allow(unreachable_code)]
#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

// Java: import cn.wps.base.log.IOLog
// Java: import cn.wps.io.dom.Helper
// Java: import cn.wps.io.xwpf.XWPFPackage
// Java: import cn.wps.moffice.crash.NoSpaceLeftException
// Java: import cn.wps.moffice.kfs.zip.ZipEntry
// Java: import cn.wps.moffice.kfs.zip.ZipInputStream
// Java: import java.io.ByteArrayInputStream
// Java: import java.io.ByteArrayOutputStream
// Java: import java.io.IOException
// Java: import java.io.InputStream
// Java: import java.util.ArrayList
// Java: import java.util.Enumeration
// Java: import java.util.Iterator
// Java: import java.util.concurrent.ExecutionException
// Java: import java.util.concurrent.ExecutorService
// Java: import java.util.concurrent.Executors
// Java: import java.util.concurrent.FutureTask
// Java: import java.util.concurrent.TimeUnit
// Java: import java.util.concurrent.TimeoutException

use super::ZipEntryFutureTask::ZipEntryFutureTask;
use super::ZipEntrySource::ZipEntrySource;
use super::ZipEntrySource::ZipEntryRef;
use crate::cn::wps::io::dom::Helper::Helper;
use crate::cn::wps::io::xwpf::XWPFPackage::XWPFPackage;
use core_libs_kfs::kfs::zip::ZipInputStream::ZipInputStream;
use std::io::{Cursor, Read};
use std::time::{Duration, Instant};

pub use super::ZipEntrySource::FakeZipEntry;

// Java field: String TAG
const TAG: Option<&str> = None;

// Rust-only: Java 的 `IOLog.e(TAG, "IOException", e)` 在本仓无日志实现（同 crate 其余文件同样只留注释）。
fn logIOException(where_: &str, detail: &dyn std::fmt::Display) {
    // Java: IOLog.e(TAG, "IOException", e);
    let _ = (TAG, where_, detail);
}

// Rust-only: 构造期两类失败。Java 侧第一类是 checked（IOException，被构造器的 catch 吞掉后
// 返回**已收集的** entries）；第二类是 unchecked（FileDamagedException，直接向上抛）。
enum ZipSourceFailure {
    // Java: catch (IOException e) —— 例如非 zip 流的 ZipException("unknown format")
    NotAZip,
    // Java: XWPFPackage.checkEntryNameLegal 抛 FileDamagedException
    IllegalEntryName,
}

impl ZipSourceFailure {
    fn message(&self) -> String {
        match self {
            ZipSourceFailure::NotAZip => "not a zip stream".to_string(),
            ZipSourceFailure::IllegalEntryName => "FileDamagedException".to_string(),
        }
    }
}

// Java: public class ZipInputStreamZipEntrySource implements ZipEntrySource
pub struct ZipInputStreamZipEntrySource {
    // Java field: ArrayList<FakeZipEntry> zipEntries
    pub(crate) zipEntries: Option<Vec<FakeZipEntry>>,
}

// Java: public class ZipInputStreamZipEntrySource implements ZipEntrySource
impl ZipInputStreamZipEntrySource {
    // Java: ZipInputStreamZipEntrySource(ZipInputStream inp)
    pub fn new(inp: &mut dyn std::io::Read) -> Self {
        // ★Java 签名外壳：Java 只有一个 ctor（`ZipInputStreamZipEntrySource.java:27-51`），
        //   其方法体逐语句对应在下方 `try_from_zip_input_stream`（本文件 `:132` 起）。
        //   Rust 这里拆成两个入口是为了同时保留「吞错版」与「返错版」（见 classes.csv 的
        //   「① try_new 与 ZipInputStreamZipEntrySource(...) 必须同时保留」约定）。
        // Java: super();
        // Java: while(true)
        // Java: ZipEntry zipEntry = inp.getNextEntry();
        // Java: if (null != zipEntry)
        // Java: String entryName = zipEntry.getName();
        // Java: XWPFPackage.checkEntryNameLegal(entryName);
        // Java: FakeZipEntry entry = new FakeZipEntry(entryName, inp);
        let entry = ();
        let _ = &entry;
        // Java: if (!this.closeEntry(inp))
        // Java: this.zipEntries.add(entry);
        // Java: continue
        // Java: catch (IOException e) { IOLog.e(TAG, "IOException", e); }
        // Java: Helper.close(inp);
        // Java: return;
        match Self::open(inp) {
            Ok(source) => source,
            // Java: catch (IOException e) { IOLog.e(TAG, "IOException", e); } → Helper.close(inp); return;
            // （Java 此处返回的是**已收集的** entries；本入口还没有 entries，故为空列表）
            Err(ZipSourceFailure::NotAZip) => {
                logIOException("ZipInputStreamZipEntrySource", &"not a zip stream");
                ZipInputStreamZipEntrySource {
                    zipEntries: Some(Vec::new()),
                }
            }
            // Java: FileDamagedException（unchecked）不被 catch 捕获 → 直接向上抛
            Err(failure) => panic!("{}", failure.message()),
        }
    }

    pub fn ZipInputStreamZipEntrySource(inp: &mut dyn std::io::Read) -> Self {
        Self::new(inp)
    }

    // Java: ZipInputStreamZipEntrySource(ZipInputStream inp) —— Java 签名版本：
    // 调用方已持有 kfs `ZipInputStream`，本入口**不额外缓冲**，是真正的流式路径。
    pub fn from_zip_input_stream(inp: &mut ZipInputStream) -> Self {
        match Self::try_from_zip_input_stream(inp) {
            Ok(source) => source,
            // Java: FileDamagedException 直接向上抛
            Err(failure) => panic!("{}", failure.message()),
        }
    }

    /// Fallible Rust boundary used by function-level tests and callers that must distinguish
    /// malformed ZIP input from a valid empty archive.
    ///
    /// Rust-only：Java 的流式后端遇到「不是 zip」会静默得到 0 entry（`ZipException("unknown format")`
    /// 被构造器的 catch 吞掉），Java 侧可判定的 not_zip 信号来自 ZipFile 后端（`new ZipFile(f)` 抛 IOException）。
    /// 这里保留同名的可判定通道，供 s01 层映射 `error.kind`。
    pub fn try_new(inp: &mut dyn std::io::Read) -> Result<Self, String> {
        Self::open(inp).map_err(|failure| failure.message())
    }

    // Rust-only: `&mut dyn Read` → kfs `ZipInputStream` 的适配层。
    // `ZipInputStream::new` 需要 `Box<dyn Read + 'static>`，所以本入口先把输入读进内存；
    // 需要真流式的调用方请用 `from_zip_input_stream`。
    fn open(inp: &mut dyn std::io::Read) -> Result<Self, ZipSourceFailure> {
        let mut bytes = Vec::new();
        inp.read_to_end(&mut bytes)
            .map_err(|_| ZipSourceFailure::NotAZip)?;
        // Java: Streams.readFully(this.in, this.hdrBuf, 0, 4) → hdr 既不是 LOCSIG(67324752)
        //       也不是 CENSIG(33639248) → ZipException("unknown format")
        // （空归档以 ENDSIG(101010256) 开头，Java 同样落到 "unknown format" → 0 entry，这里一并放行）
        let signature = &bytes[..bytes.len().min(4)];
        if signature != b"PK\x03\x04" && signature != b"PK\x01\x02" && signature != b"PK\x05\x06" {
            return Err(ZipSourceFailure::NotAZip);
        }
        let mut zis = ZipInputStream::new(Box::new(Cursor::new(bytes)));
        Self::try_from_zip_input_stream(&mut zis)
    }

    // Rust-only: 下面是 Java 构造器 `ZipInputStreamZipEntrySource(ZipInputStream inp)` 的方法体（逐语句对应）
    fn try_from_zip_input_stream(inp: &mut ZipInputStream) -> Result<Self, ZipSourceFailure> {
        // Java: super();
        // Java: ArrayList<FakeZipEntry> zipEntries = new ArrayList();
        let mut zipEntries: Vec<FakeZipEntry> = Vec::new();
        // Java: while(true)
        loop {
            // Java: try
            // Java: ZipEntry zipEntry = inp.getNextEntry();
            match inp.try_getNextEntry() {
                // Java: if (null != zipEntry)
                Ok(Some(zipEntry)) => {
                    // Java: String entryName = zipEntry.getName();
                    let entryName = zipEntry.getName().to_string();
                    // Java: XWPFPackage.checkEntryNameLegal(entryName);
                    // Rust: 复用与 Java **同一判据**（不再内联复制）；FileDamagedException 是 unchecked，
                    //       两层约定下走 try_ 通道，由调用方决定 panic 还是 Err。
                    XWPFPackage::try_checkEntryNameLegal(&entryName)
                        .map_err(|_| ZipSourceFailure::IllegalEntryName)?;
                    // Java: FakeZipEntry entry = new FakeZipEntry(entryName, inp);
                    let entry = FakeZipEntry::FakeZipEntry(&entryName, inp);
                    // Java: if (!this.closeEntry(inp)) { return; }
                    if !Self::closeEntry(inp) {
                        // Java: return;（★这条路径**不**调用 Helper.close(inp)，与 Java 一致）
                        return Ok(ZipInputStreamZipEntrySource {
                            zipEntries: Some(zipEntries),
                        });
                    }
                    // Java: this.zipEntries.add(entry);
                    zipEntries.push(entry);
                    // Java: continue;
                    continue;
                }
                // Java: zipEntry == null → 跳出 if，落到下面的 Helper.close(inp); return;
                Ok(None) => {}
                // Java: catch (IOException e) { IOLog.e(TAG, "IOException", e); }
                // Rust: 吞掉并**保留已收集的 entries**（旧实现整体返回 Err → 部分结果丢失）
                Err(error) => logIOException("getNextEntry", &error),
            }
            // Java: Helper.close(inp);
            Helper::close(inp);
            // Java: return;
            return Ok(ZipInputStreamZipEntrySource {
                zipEntries: Some(zipEntries),
            });
        }
    }

    pub fn from_entries(entries: Vec<FakeZipEntry>) -> Self {
        ZipInputStreamZipEntrySource {
            zipEntries: Some(entries),
        }
    }

    // Java: boolean closeEntry(ZipInputStream inp)
    fn closeEntry(inp: &mut ZipInputStream) -> bool {
        // Java: ExecutorService executor = Executors.newSingleThreadExecutor();
        // Java: FutureTask<Void> futureTask = new ZipEntryFutureTask(new ZipEntryCallable(inp));
        // Rust: Java 的「任务」= `inp.closeEntry()`；这里把同一份 5000ms 预算（ZipEntryFutureTask.TIMEOUT）
        //       作为 futureTask 的对应物。
        let futureTask_timeout_ms = ZipEntryFutureTask::TIMEOUT as u64;
        // Java: executor.execute(futureTask);
        // Rust: ZipEntryCallable.call() 的全部内容就是 `inp.closeEntry()`；Java 把它丢到单线程
        //       executor 上，用 futureTask.get(5000L, MILLISECONDS) 做超时保护。
        //       kfs `ZipInputStream` 不是 Send（内部持有 `Box<dyn Read>`），Rust 无法把它搬到另一个
        //       线程再 interrupt，因此这里用**同一个预算 + 分块检查**实现同样的可观测语义：
        //       超时 → 放弃（等价 TimeoutException → cancel(true)）并返回 false。
        let deadline = Instant::now() + Duration::from_millis(futureTask_timeout_ms);
        // Java: boolean var16;
        let mut var16: bool = false;
        // Java: byte[] buffer = new byte[4096];
        let mut buffer = [0u8; 4096];
        let buffer_len = buffer.len();
        // Java: try
        loop {
            // Java: futureTask.get(5000L, TimeUnit.MILLISECONDS);
            if Instant::now() >= deadline {
                // Java: catch (TimeoutException var13) { futureTask.cancel(true); var16 = false; return var16; }
                return false;
            }
            // Java: this.mInp.closeEntry(); —— 丢弃当前 entry 的剩余字节
            match inp.try_read(&mut buffer, 0, buffer_len) {
                // Java: read() > 0 → 还有剩余
                Ok(read) if read > 0 => continue,
                // Java: read() <= 0 → 当前 entry 已耗尽
                Ok(_) => break,
                // Java: catch (ExecutionException var12) { futureTask.cancel(true); var16 = false; return var16; }
                Err(error) => {
                    logIOException("closeEntry", &error);
                    return false;
                }
            }
        }
        // Java: this.mInp.closeEntry() 的收尾（skipAll 已无剩余 + 回推修正 + CRC/尺寸校验）
        if let Err(error) = inp.try_closeEntry() {
            // Java: catch (ExecutionException var12) { futureTask.cancel(true); var16 = false; return var16; }
            logIOException("closeEntry", &error);
            return false;
        }
        // Java: catch (InterruptedException var11) { futureTask.cancel(true); var16 = false; }
        // rust: no-stmt（协作式截止检查不抛 InterruptedException；超时已由上方 TimeoutException 分支返回 false）
        // Java: finally { executor.shutdown(); }
        // Java: return true;
        let _ = &mut var16;
        true
    }

    // Java: Enumeration<? extends ZipEntry> getEntries()
    pub fn getEntries(&self) -> &Option<Vec<FakeZipEntry>> {
        // Java: return new EntryEnumerator();
        &self.zipEntries
    }

    // Java: int size()
    pub fn size(&self) -> usize {
        // Java: return this.zipEntries.size();
        self.zipEntries.as_ref().map_or(0, |v| v.len())
    }

    // Java: InputStream getInputStream(ZipEntry entry, boolean isOriginInput)
    //   ★形参声明类型 = Java 基类 `ZipEntry`（Rust 等价 `ZipEntryRef`），**不再**收窄成 `FakeZipEntry`
    pub fn getInputStream_with_origin(&self, entry: &ZipEntryRef, _is_origin_input: bool) -> Cursor<Vec<u8>> {
        // Java: return this.getInputStream(entry);
        self.getInputStream(entry)
    }

    // Java: InputStream getInputStream(ZipEntry zipEntry)
    pub fn getInputStream(&self, zip_entry: &ZipEntryRef) -> Cursor<Vec<u8>> {
        // Java: FakeZipEntry entry = (FakeZipEntry)zipEntry;
        // ★Java 这一步是**显式强转**：非 `FakeZipEntry` 抛 `ClassCastException`。
        //   真实类型下它变成一次穷尽 match（`as_fake()`），不再需要 `dyn Any` 探测；
        //   失败仍然 panic（对应 ClassCastException），**不**静默返回空流。
        let entry = zip_entry.as_fake();
        // Java: return entry.getInputStream();
        entry.getInputStream()
    }

    // Java: void close()
    pub fn close(&mut self) {
        // Java: this.zipEntries.clear();
        // Java: this.zipEntries = null;
        self.zipEntries = None;
    }
}

// Java: private class EntryEnumerator implements Enumeration<ZipEntry>
pub struct EntryEnumerator {
    // Java field: Iterator<? extends ZipEntry> iterator
    iterator: Vec<FakeZipEntry>,
    cursor: usize,
}

// Java: private class EntryEnumerator implements Enumeration<ZipEntry>
impl EntryEnumerator {
    // Java: EntryEnumerator()
    pub fn EntryEnumerator(source: &ZipInputStreamZipEntrySource) -> Self {
        // Java: super();
        // Java: this.iterator = ZipInputStreamZipEntrySource.this.zipEntries.iterator();
        let iterator = source.zipEntries.clone().expect("zipEntries");
        EntryEnumerator {
            iterator,
            cursor: 0,
        }
    }

    // Java: boolean hasMoreElements()
    pub fn hasMoreElements(&self) -> bool {
        // Java: return this.iterator.hasNext();
        self.cursor < self.iterator.len()
    }

    // Java: ZipEntry nextElement()
    pub fn nextElement(&mut self) -> FakeZipEntry {
        // Java: return (ZipEntry)this.iterator.next();
        let entry = self.iterator[self.cursor].clone();
        self.cursor += 1;
        entry
    }
}

// Java: public static class FakeZipEntry extends ZipEntry
impl FakeZipEntry {
    // Java: FakeZipEntry(String entryName, ZipInputStream inp)
    pub fn FakeZipEntry(entry_name: &str, inp: &mut ZipInputStream) -> Self {
        // Java: super(entryName);
        // Java: ByteArrayOutputStream baos = new ByteArrayOutputStream();
        let mut baos: Vec<u8> = Vec::new();
        // Java: byte[] buffer = new byte[4096];
        let mut buffer = [0u8; 4096];
        let capacity = buffer.len();
        // Java: int read = 0;
        let mut read: i32 = 0;
        // Java: try
        // Java: while(true)
        loop {
            // Java: read = inp.read(buffer);
            match inp.try_read(&mut buffer, 0, capacity) {
                Ok(value) => read = value,
                // Java: catch (IOException e) {
                // Java: IOLog.e(ZipInputStreamZipEntrySource.TAG, "IOException", e);
                // Java: if (NoSpaceLeftException.isNoSpaceLeftException(e)) {
                // Java: throw new RuntimeException(e);
                // Java: }
                // Rust: 与 Java 一致 —— 吞掉 IOException 并**保留已读到的部分字节**；
                //       NoSpaceLeftException 在本仓无对应物（见 §1.7），故不做 RuntimeException 重抛。
                Err(error) => {
                    logIOException("FakeZipEntry", &error);
                    break;
                }
            }
            // Java: if (read <= 0) { break; }
            if read <= 0 {
                break;
            }
            // Java: baos.write(buffer, 0, read);
            baos.extend_from_slice(&buffer[..read as usize]);
        }
        // Java: this.data = baos.toByteArray();
        FakeZipEntry::new(entry_name, baos)
    }

    // Java: InputStream getInputStream()
    pub fn getInputStream(&self) -> Cursor<Vec<u8>> {
        // Java: return new ByteArrayInputStream(this.data);
        Cursor::new(self.data.clone())
    }
}

impl ZipEntrySource for ZipInputStreamZipEntrySource {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn getEntries(&self) -> Option<&[FakeZipEntry]> {
        self.zipEntries.as_deref()
    }

    fn size(&self) -> i32 {
        ZipInputStreamZipEntrySource::size(self) as i32
    }

    // Java: getInputStream(ZipEntry var1, boolean var2) —— 形参的声明类型是**基类 `ZipEntry`**
    fn getInputStream_with_flag(
        &self,
        var1: &ZipEntryRef,
        var2: bool,
    ) -> Result<Box<dyn Read>, String> {
        // ★Java 体是 `return this.getInputStream(entry);` —— 2 参重载**原样转发**，不做强转
        //   （`ZipInputStreamZipEntrySource.java:88-90`）⇒ 这里也原样转发。
        Ok(Box::new(ZipInputStreamZipEntrySource::getInputStream_with_origin(
            self, var1, var2,
        )))
    }

    // Java: getInputStream(ZipEntry zipEntry)
    fn getInputStream(&self, var1: &ZipEntryRef) -> Result<Box<dyn Read>, String> {
        // ★Java 体是 `FakeZipEntry entry = (FakeZipEntry)zipEntry;`（`:93` 显式强转）⇒ 落到 1 参
        //   固有方法时按同一语义取 `FakeZipEntry`：`as_fake()` 在 `Base` 分支 panic，
        //   对应 Java 的 `ClassCastException`（同一失败类别，**不**静默返回 None）。
        Ok(Box::new(ZipInputStreamZipEntrySource::getInputStream(
            self, var1,
        )))
    }

    fn close(&mut self) {
        ZipInputStreamZipEntrySource::close(self);
    }
}
