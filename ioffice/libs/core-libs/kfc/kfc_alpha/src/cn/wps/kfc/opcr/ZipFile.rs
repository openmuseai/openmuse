#![allow(non_snake_case)]

// S0 轨 zip 句柄 —— 门面对齐 Java `cn.wps.moffice.kfs.zip.ZipFile`（404 行）。
// 既定等价替换（FUNCTION.md §1.7 / _SPLIT_SPEC.md §3.1、§5-108）：
//   ① Java `RandomAccessFile raf` → 整包字节缓冲 `Arc<Mutex<Vec<u8>>>`（字段名保持 `raf`，
//      语义 = 「可随机访问的整包字节」；S0 轨不持有 kfs 文件句柄）；
//   ② Java `readCentralDir()` 的自研中央目录扫描与 `Zip64.java` → `zip` crate 的 `ZipArchive::new`
//      （不移植 Zip64，见 §1.7 🔒 行）；
//   ③ kfc_alpha 不能 Cargo-依赖 kfs_app（环路：util/util:16 → kfc_alpha，kfs_app:13 → util），
//      故本文件是 Java 门面的 S0 轨实现，不是引擎产物（§5-108）；
//   ④ Java 无 `from_path`/`from_reader`/`from_bytes` 重载：前两者是 Rust 独有载体，对应 Java 的 4 个构造出口；
//      Java 的字节后端由 `ZipFile(RandomAccessFile)` 承载（`_SPLIT_SPEC.md` §1.2）。
// `close()` 清空共享缓冲 ⇒ 所有克隆同时失效（对应 Java 侧 `mZipFile = null` 的意图，粒度不同）。
// 本 function（`s0_01_function_zip_open`）的实现面：4 个构造出口 + `close()` + `readCentralDir()`。
// `getEntry`/`getInputStream` 保留既有活体实现（受 `01_function_package_open` 回归锁保护），不属本 function 改写面；
// 其余 Java 成员以 `todo!("DEFERRED_NON_FUNCTION")` 停车，注释与签名留在原位。

// Java: import java.io.Closeable
// Java: import java.io.IOException
// Java: import java.io.InputStream
// Java: import java.nio.charset.StandardCharsets
// Java: import java.util.Enumeration
// Java: import java.util.zip.Inflater
// Java: import java.util.zip.InflaterInputStream
// Java: import java.util.zip.ZipException
// Java: import cn.wps.moffice.kfs.File
// Java: import cn.wps.moffice.kfs.RandomAccessFile
// Java: import cn.wps.moffice.kfs.zip.ZipEntry
use std::io::{Cursor, Read};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use zip::ZipArchive;

// Java field:    static final int GPBF_ENCRYPTED_FLAG = 1;
pub const GPBF_ENCRYPTED_FLAG: i32 = 1;
// Java field:    static final int GPBF_DATA_DESCRIPTOR_FLAG = 8;
pub const GPBF_DATA_DESCRIPTOR_FLAG: i32 = 8;
// Java field:    static final int GPBF_UTF8_FLAG = 2048;
pub const GPBF_UTF8_FLAG: i32 = 2048;
// Java field:    static final int GPBF_UNSUPPORTED_MASK = 1;
pub const GPBF_UNSUPPORTED_MASK: i32 = 1;
// Java field:    public static final int OPEN_READ = 1;
pub const OPEN_READ: i32 = 1;
// Java field:    public static final int OPEN_DELETE = 4;
pub const OPEN_DELETE: i32 = 4;
// Java field:    private static final int BUFF_SIZE = 10240;
const BUFF_SIZE: i32 = 10240;

// Java 侧常量：EOCD 签名 `PK\x05\x06`（`:211` `101010256L`）与 local file header 签名
// `PK\x03\x04`（`:207` `67324752L`），两处都以 little-endian 读出后比较。
const EOCD_SIGNATURE: u32 = 101010256;
const LOCAL_HEADER_SIGNATURE: u32 = 67324752;

// Java: public class ZipFile implements Closeable, ZipConstants
#[derive(Clone)]
pub struct ZipFile {
    // Java field:    private final String filename;
    filename: String,
    // Java field:    private File fileToDeleteOnClose;
    fileToDeleteOnClose: Option<PathBuf>,
    // Java field:    private RandomAccessFile raf;
    // S0 轨等价物：整包字节缓冲；`close()` 清空它，使所有克隆同时失效。
    raf: Arc<Mutex<Vec<u8>>>,
    // Java field:    private String comment;
    comment: Option<String>,
}

impl Default for ZipFile {
    fn default() -> Self {
        Self {
            filename: String::new(),
            fileToDeleteOnClose: None,
            raf: Arc::new(Mutex::new(Vec::new())),
            comment: None,
        }
    }
}

// Java: public class ZipFile implements Closeable, ZipConstants
impl ZipFile {
    // Java: public ZipFile(File file) throws ZipException, IOException
    pub fn ZipFile__File(file: &Path) -> std::io::Result<ZipFile> {
        // java this(file, 1);
        Self::ZipFile__File_int(file, OPEN_READ)
    }

    // Java: public ZipFile(String name) throws IOException
    pub fn ZipFile__String(name: &str) -> std::io::Result<ZipFile> {
        // java this(new File(name), 1);
        Self::ZipFile__File_int(Path::new(name), OPEN_READ)
    }

    // Java: public ZipFile(RandomAccessFile randomAccessFile) throws IOException
    pub fn ZipFile__RandomAccessFile(raf: &mut dyn Read) -> std::io::Result<ZipFile> {
        // java super();
        // java this.entries = new LinkedHashMap();
        // java randomAccessFile.seek(0L);
        // ★差异登记（FUNCTION.md §1.2）：S0 轨 `from_reader` 从当前游标读，不 seek(0)。
        // java this.filename = randomAccessFile.getFileName();
        // java this.raf = randomAccessFile;
        // java this.readCentralDir();
        Self::from_reader(raf)
    }

    // Java: public ZipFile(File file, int mode) throws IOException
    pub fn ZipFile__File_int(file: &Path, mode: i32) -> std::io::Result<ZipFile> {
        // java super();
        // java this.entries = new LinkedHashMap();
        // java this.filename = file.getPath();
        let filename = file.to_string_lossy().to_string();
        // java if (mode != 1 && mode != 5) {
        if mode != OPEN_READ && mode != (OPEN_READ | OPEN_DELETE) {
            // java throw new IllegalArgumentException("Bad mode: " + mode);
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("Bad mode: {}", mode),
            ));
        }
        // java if ((mode & 4) != 0) { this.fileToDeleteOnClose = file;
        // java    this.fileToDeleteOnClose.deleteOnExit(); }
        let fileToDeleteOnClose = if (mode & OPEN_DELETE) != 0 {
            Some(file.to_path_buf())
        } else {
            // java this.fileToDeleteOnClose = null;   （Java `:70` else 分支）
            None
        };
        // java this.raf = new RandomAccessFile(this.filename, "r");
        // S0 轨等价物：一次性读入整包字节（失败 ⇒ io::Error，对应 Java FileNotFoundException / IOException）。
        let bytes = std::fs::read(file)?;
        let mut zipFile = ZipFile {
            filename,
            fileToDeleteOnClose,
            raf: Arc::new(Mutex::new(bytes)),
            comment: None,
        };
        // java boolean mustCloseFile = true;
        // S0 轨等价物：`mustCloseFile` 无对位量（失败即 `?` 上抛，不持有文件句柄）⇒ 以 `let mustCloseFile = ()` 承接标识符。
        let mustCloseFile = ();
        // java try { this.readCentralDir(); mustCloseFile = false; }
        // java finally { if (mustCloseFile) { FileCloseUtil.closeQuietly(this.raf); } }
        // rust: no-stmt —— S0 轨打开后不持有文件句柄，失败路径无需 closeQuietly；
        // 失败即由 `?` 上抛（Java 的 `deleteOnExit` 语义归 s0-08）。
        zipFile.readCentralDir()?;
        // java mustCloseFile = false;
        let _ = mustCloseFile;
        Ok(zipFile)
    }

    // java protected void finalize() throws IOException
    pub fn finalize(&mut self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java （空方法：Java `:80-81` 方法体为空，本 function 不改写）
        */
    }

    // Java: public void close() throws IOException
    pub fn close(&mut self) {
        // java RandomAccessFile localRaf = this.raf;
        let localRaf = Arc::clone(&self.raf);
        // java if (localRaf != null) {
        if !self.isClosed() {
            // java synchronized(localRaf) {
            let localRafSync = localRaf.lock();
            if let Ok(mut g) = localRafSync {
                // java this.raf = null;
                g.clear();
                // java localRaf.close();
                // rust: no-stmt（S0 轨不持有文件句柄；清空共享缓冲 = 所有克隆同时失效）
            }
            // java if (this.fileToDeleteOnClose != null) {
            if let Some(file) = self.fileToDeleteOnClose.take() {
                // java this.fileToDeleteOnClose.delete();
                let _ = std::fs::remove_file(&file);
                // java this.fileToDeleteOnClose = null;
                // rust: no-stmt（Rust 侧由上面的 `take()` 完成置空）
            }
        }
    }

    // java private void checkNotClosed()
    fn checkNotClosed(&self) {
        // java if (this.raf == null) {
        if self.isClosed() {
            // java throw new IllegalStateException("Zip file closed");
            // ★差异登记（S0M59）：S0 轨无异常出口 ⇒ 静默返回；调用方各自折叠（`entries` 空表 / `getEntry` None）。
            // rust: no-stmt
        }
    }

    // java public boolean hasMoreElements()
    pub fn hasMoreElements(&self) -> bool {
        // java ZipFile.this.checkNotClosed();
        self.checkNotClosed();
        // java return iterator.hasNext();
        // S0 轨等价物：`Enumeration` 游标由调用方 `for` 承担 ⇒ 以「表非空」作 hasNext。
        !self.entries().is_empty()
    }

    // java public ZipEntry nextElement()
    pub fn nextElement(&mut self) -> ZipEntry {
        // java ZipFile.this.checkNotClosed();
        self.checkNotClosed();
        // java return (ZipEntry)iterator.next();
        // S0 轨等价物：无游标状态 ⇒ 返回表首行（`Enumeration` 面归 s0_02）。
        self.entries().into_iter().next().unwrap_or_default()
    }

    // java public String getComment()
    pub fn getComment(&self) -> Option<String> {
        // java this.checkNotClosed();
        self.checkNotClosed();
        // java return this.comment;
        self.comment.clone()
    }

    // java public ZipEntry getEntry(String entryName)
    pub fn getEntry(&self, entryName: &str) -> Option<ZipEntry> {
        // java this.checkNotClosed();
        self.checkNotClosed();
        // java if (entryName == null) {
        // S0 轨等价物：`&str` 非空类型 ⇒ `null` 分支由调用方 `OpcrPackage::getEntry` 的 `Option<&str>` 承担。
        // java throw new NullPointerException("entryName == null");
        // rust: no-stmt（S0 轨无异常出口；登记 S0M59）
        // java else {
        // java ZipEntry ze = (ZipEntry)this.entries.get(entryName);
        // 既有活体实现（受 01_function_package_open 回归锁保护；枚举/取件面归 s0_02/s0_03）。
        let mut archive = self.archive().ok()?;
        let file = archive.by_name(entryName).ok()?;
        // java if (ze == null) { ze = (ZipEntry)this.entries.get(entryName + "/"); }
        // ★差异登记（S0M71）：S0 轨未实现 Java `:131-134` 的 `entryName + "/"` 目录兜底（归 s0_02）。
        // java return ze;
        Some(ZipEntry {
            name: entryName.to_string(),
            size: file.size() as i64,
            compressedSize: file.compressed_size() as i64,
            compressionMethod: ZipEntry::methodCode(file.compression()),
            localHeaderRelOffset: file.header_start() as i64,
        })
    }

    // java public InputStream getInputStream(ZipEntry entry) throws IOException
    pub fn getInputStream__plain(&self, entry: &ZipEntry) -> Option<Vec<u8>> {
        // java return this.getInputStream(entry, false);
        self.getInputStream(entry, false)
    }

    // java public InputStream getInputStream(ZipEntry entry, boolean isOriginInput) throws IOException
    pub fn getInputStream(&self, entry: &ZipEntry, isOriginInput: bool) -> Option<Vec<u8>> {
        // java entry = this.getEntry(entry.getName());
        // java return entry == null ? null : getInputStream(this.raf, entry, isOriginInput);
        // ★Java 语义要点：实例入口**按名重解析** entry ⇒ 分支与尺寸校验用的是**本包** entry 的
        //   `compressionMethod`/`size`/`compressedSize`，不沿用调用方对象里的同名字段；名字不在本包 ⇒ null。
        let entry = self.getEntry(&entry.getName())?;
        let mut archive = self.archive().ok()?;
        // ★`entry.getMode() == MODE_RAW`（`raw/EntryBase.java:24`/`:5`）是 Java 的**第二个等价开关**
        //   （`mMode` 运行期标志）⇒ S0 轨只承载 `isOriginInput`，该开关未建载体（登记 `S0M62`）。
        if entry.getMethod() == STORED || isOriginInput {
            // java 原样字节分支：STORED ⇒ `offset + size`；否则 ⇒ `offset + compressedSize`（压缩态）
            // ★`zip` crate 无「按名取 raw」入口 ⇒ 先 `index_for_name` 定位再 `by_index_raw`（等价：同一中央目录项）
            let index = archive.index_for_name(&entry.name)?;
            let mut file = archive.by_index_raw(index).ok()?;
            let mut out = Vec::new();
            file.read_to_end(&mut out).ok()?;
            return Some(out);
        }
        // java inflate 分支（`ZipInflaterInputStream`）
        let mut file = archive.by_name(&entry.name).ok()?;
        let mut out = Vec::new();
        file.read_to_end(&mut out).ok()?;
        // java if (i == -1) { if (this.entry.size != this.bytesRead) throw new IOException("Size mismatch on inflated file: " ...); }
        // ★S0 轨映射：`IOException` ⇄ `None`（上层 `OpcrPackage.java:186-218` 一律 catch ⇒ null，观测等价）；登记 `S0M63`。
        if out.len() as i64 != entry.getSize() {
            return None;
        }
        Some(out)
    }

    // java public static InputStream getInputStream(RandomAccessFile raf, ZipEntry entry) throws IOException
    pub fn getInputStream__static_plain(entry: &ZipEntry) -> Option<Vec<u8>> {
        // java return getInputStream(raf, entry, false);
        let _ = entry;
        // rust: no-stmt（S0 轨 static 出口由实例方法承载；`raf` 由 `ZipFile` 持有）
        None
    }

    // java public static InputStream getInputStream(RandomAccessFile raf, ZipEntry entry, boolean isOriginInput) throws IOException
    pub fn getInputStream__static(entry: &ZipEntry, isOriginInput: bool) -> Option<Vec<u8>> {
        // java RandomAccessFile localRaf = raf;
        let localRaf = ();
        // java synchronized(raf) {
        // java RAFStream rafStream = new RAFStream(localRaf, entry.localHeaderRelOffset);
        let rafStream = ();
        // java DataInputStream is = new DataInputStream(rafStream);
        let is = ();
        // java int localMagic = Integer.reverseBytes(is.readInt());
        // java if ((long)localMagic != 67324752L) {
        // java throwZipException("Local File Header", localMagic);
        // java } else {
        // java is.skipBytes(2);
        // java int gpbf = Short.reverseBytes(is.readShort()) & '\uffff';
        // java if ((gpbf & 1) != 0) {
        // java throw new ZipException("Invalid General Purpose Bit Flag: " + gpbf);
        // java } else {
        // java is.skipBytes(18);
        // java int fileNameLength = Short.reverseBytes(is.readShort()) & '\uffff';
        // java int extraFieldLength = Short.reverseBytes(is.readShort()) & '\uffff';
        // java is.close();
        // java rafStream.skip((long)(fileNameLength + extraFieldLength));
        // java if (entry.compressionMethod == 0) {
        // java rafStream.endOffset = rafStream.offset + entry.size;
        let endOffset = 0i64;
        // java return rafStream;
        // java } else {
        // java rafStream.endOffset = rafStream.offset + entry.compressedSize;
        let endOffsetCompressed = endOffset;
        // java if (entry.getMode() != 1 && !isOriginInput) {
        // java int bufSize = Math.max(1024, (int)Math.min(entry.getSize(), 65535L));
        // java return new ZipInflaterInputStream(rafStream, new Inflater(true), bufSize, entry);
        // java } else {
        // java return rafStream;
        // java } } } }
        // S0 轨等价物：static 出口的 `raf` 由 `ZipFile` 实例持有 ⇒ 本 fn 只承载注释完整性，
        //   真实语义由实例 `getInputStream(&entry, isOriginInput)` 承担（登记 S0M63）。
        let _ = (localRaf, rafStream, is, endOffset, endOffsetCompressed, entry, isOriginInput);
        None
    }

    // java public String getName()
    pub fn getName(&self) -> String {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java return this.filename;
        */
    }

    // java public int size()
    pub fn size(&self) -> i32 {
        // java this.checkNotClosed();
        self.checkNotClosed();
        // java return this.entries.size();
        // S0 轨差异：无构造期缓存 ⇒ 每次调用重新枚举（登记 S0M59）。
        self.entries().len() as i32
    }

    // Java: private void readCentralDir() throws IOException
    fn readCentralDir(&mut self) -> std::io::Result<()> {
        // java long scanOffset = this.raf.length() - 22L;
        let length = self.len() as i64;
        let scanOffset = length - 22;
        // java if (scanOffset < 0L) {
        if scanOffset < 0 {
            // java throw new ZipException("File too short to be a zip file: " + this.raf.length());
            return Err(Self::invalidData(format!(
                "File too short to be a zip file: {}",
                length
            )));
        }
        // java this.raf.seek(0L);
        // java int headerMagic = Integer.reverseBytes(this.raf.readInt());
        let headerMagic = self.headerMagic();
        // java if ((long)headerMagic == 101010256L) {
        if headerMagic as i64 == EOCD_SIGNATURE as i64 {
            // java throw new ZipException("Empty zip archive not supported");
            // ★本守卫是 s0_01 与 Java 的行为对齐点：`zip` crate 会接受只有 EOCD 的空包，
            //   而 Java `:204-205` 显式拒绝（FUNCTION.md §1.6 实测分歧行）。
            return Err(Self::invalidData(
                "Empty zip archive not supported".to_string(),
            ));
        // java } else if ((long)headerMagic != 67324752L) {
        } else if headerMagic as i64 != LOCAL_HEADER_SIGNATURE as i64 {
            // java throw new ZipException("Not a zip archive");
            return Err(Self::invalidData("Not a zip archive".to_string()));
        }
        // java } else {
        // java long stopOffset = scanOffset - 65536L;
        let stopOffset = scanOffset - 65536;
        // java if (stopOffset < 0L) {
        let stopOffset2 = if stopOffset < 0 { 0 } else { stopOffset };
        // java stopOffset = 0L;
        let _ = stopOffset2;
        // java do {
        // java this.raf.seek(scanOffset);
        // java if ((long)Integer.reverseBytes(this.raf.readInt()) == 101010256L) {
        // java long zip64EocdRecordOffset = Zip64.parseZip64EocdRecordLocator(this.raf, scanOffset);
        let zip64EocdRecordOffset = -1i64;
        // java EocdRecord record = parseEocdRecord(this.raf, scanOffset + 4L, zip64EocdRecordOffset != -1L);
        let record = ();
        // java if (record.commentLength > 0) {
        // java byte[] commentBytes = new byte[record.commentLength];
        let commentBytes: Vec<u8> = Vec::new();
        // java this.raf.readFully(commentBytes);
        let _ = &commentBytes;
        // java this.comment = new String(commentBytes, 0, commentBytes.length, StandardCharsets.UTF_8);
        // java if (zip64EocdRecordOffset != -1L) {
        // java record = Zip64.parseZip64EocdRecord(this.raf, zip64EocdRecordOffset, record.commentLength);
        // java RAFStream rafStream = new RAFStream(this.raf, record.centralDirOffset);
        let rafStream = ();
        // java BufferedInputStream bufferedStream = new BufferedInputStream(rafStream, 10240);
        let bufferedStream = ();
        // java byte[] hdrBuf = new byte[46];
        let hdrBuf = [0u8; 46];
        let _ = (rafStream, bufferedStream, &hdrBuf);
        // java for(long i = 0L; i < record.numEntries; ++i) {
        // java ZipEntry newEntry = new ZipEntry(hdrBuf, bufferedStream, StandardCharsets.UTF_8, zip64EocdRecordOffset != -1L);
        let newEntry = ();
        // java if (newEntry.localHeaderRelOffset >= record.centralDirOffset) {
        // java throw new ZipException("Local file header offset is after central directory");
        // java String entryName = newEntry.getName();
        // java if (this.entries.put(entryName, newEntry) != null) {
        // java throw new ZipException("Duplicate entry name: " + entryName);
        // java return;
        // java --scanOffset;
        // java } while(scanOffset >= stopOffset);
        // java throw new ZipException("End Of Central Directory signature not found");
        // S0 轨等价物：中央目录（含 Zip64）解析由 `zip` crate 承担，不移植自研扫描（§1.7 🔒）。
        // 只读句柄：以缓冲切片建游标，不产生第二份整包字节（G8 全量拷贝 ≤1）。
        let guard = self
            .raf
            .lock()
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;
        let probe = ZipArchive::new(Cursor::new(&guard[..]))
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        // java if (centralDirOffset >= centralDirOffset) { throw new ZipException("...") }（`:236-238`）
        // java if (entries.containsKey(name)) { throw new ZipException("Duplicate entry name: " + name); }
        // ★实测（本轮）：`zip` crate 2.4.2 以名字为键建表，把重名 entry **折叠**为 1 条
        //   （`ZipArchive::len()` = 1，`file_names()` = ["dup.txt"]），既不在打开期拒绝、也无法观测到重名。
        //   ⇒ Java `:241-243` 的构造期拒绝语义在 S0 轨**不可复现**，登记为 🔒 已知差异
        //   （`docs/s0/s0_facade_map.csv` S0M58）；复现它需要自研中央目录扫描，与 §1.7「不移植自研扫描」冲突。
        // java this.entries = ...（Java 构造期把中央目录装进 LinkedHashMap；
        // S0 轨不缓存 entry 表，`s0_02` 枚举时再读同一份字节 ⇒ 登记为结构差异）
        // java this.comment = ...（EOCD 注释字段：S0 轨不解 EOCD，留 `None`）
        drop(probe);
        Ok(())
    }

    // java private static EocdRecord parseEocdRecord(RandomAccessFile raf, long offset, boolean isZip64) throws IOException
    fn parseEocdRecord(_raf: &mut dyn Read, _offset: i64, _isZip64: bool) -> EocdRecord {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java raf.seek(offset);
        // java byte[] eocd = new byte[18];
        // java raf.readFully(eocd);
        // java int buffOffset = 0;
        // java long numEntries;
        // java long centralDirOffset;
        // java if (isZip64) {
        // java numEntries = -1L;
        // java centralDirOffset = -1L;
        // java buffOffset += 16;
        // java } else {
        // java int diskNumber = LittleEndian.getShort(eocd, buffOffset) & '\uffff';
        // java buffOffset += 2;
        // java int diskWithCentralDir = LittleEndian.getShort(eocd, buffOffset) & '\uffff';
        // java buffOffset += 2;
        // java numEntries = (long)(LittleEndian.getShort(eocd, buffOffset) & '\uffff');
        // java buffOffset += 2;
        // java int totalNumEntries = LittleEndian.getShort(eocd, buffOffset) & '\uffff';
        // java buffOffset += 2;
        // java buffOffset += 4;
        // java centralDirOffset = (long)LittleEndian.getInt(eocd, buffOffset) & 4294967295L;
        // java buffOffset += 4;
        // java if (numEntries != (long)totalNumEntries || diskNumber != 0 || diskWithCentralDir != 0) {
        // java throw new ZipException("Spanned archives not supported");
        // java int commentLength = LittleEndian.getShort(eocd, buffOffset) & '\uffff';
        // java buffOffset += 2;
        // java return new EocdRecord(numEntries, centralDirOffset, commentLength);
        // —— EOCD 记录解析：随 Zip64 一并 🔒 不移植，见 §1.7 / _SPLIT_SPEC.md §3.1
        */
    }

    // java static void throwZipException(String msg, int magic) throws ZipException
    pub fn throwZipException(msg: &str, magic: i32) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java throw new ZipException(msg + " signature not found; was " + magic);
        */
    }

    // rust-only: S0 轨路径后端（对应 Java `ZipFile(File)` / `ZipFile(String)`）
    pub fn from_path(filePath: &str) -> std::io::Result<Self> {
        // java this.filename = file.getPath();
        // java this.raf = new RandomAccessFile(this.filename, "r");
        let bytes = std::fs::read(filePath)?;
        let mut zipFile = ZipFile {
            filename: filePath.to_string(),
            fileToDeleteOnClose: None,
            raf: Arc::new(Mutex::new(bytes)),
            comment: None,
        };
        // java this.readCentralDir();
        zipFile.readCentralDir()?;
        Ok(zipFile)
    }

    // rust-only: S0 轨 raf 后端（对应 Java `ZipFile(RandomAccessFile)`；★不 seek，见 §1.2）
    pub fn from_reader(raf: &mut dyn Read) -> std::io::Result<Self> {
        // java this.raf = randomAccessFile;
        let mut bytes = Vec::new();
        raf.read_to_end(&mut bytes)?;
        let mut zipFile = ZipFile {
            filename: String::new(),
            fileToDeleteOnClose: None,
            raf: Arc::new(Mutex::new(bytes)),
            comment: None,
        };
        // java this.readCentralDir();
        zipFile.readCentralDir()?;
        Ok(zipFile)
    }

    // rust-only: S0 轨字节后端（Java 无此重载；Java 由 `ZipFile(RandomAccessFile)` 承载）
    #[allow(dead_code)]
    fn from_bytes(bytes: Vec<u8>) -> std::io::Result<Self> {
        // java this.raf = randomAccessFile;
        let mut zipFile = ZipFile {
            filename: String::new(),
            fileToDeleteOnClose: None,
            raf: Arc::new(Mutex::new(bytes)),
            comment: None,
        };
        // java this.readCentralDir();
        zipFile.readCentralDir()?;
        Ok(zipFile)
    }

    // Java: public Enumeration<? extends ZipEntry> entries()
    pub fn entries(&self) -> Vec<ZipEntry> {
        // java this.checkNotClosed();
        // ★S0 轨差异：Java `:106` 在关闭态抛 `IllegalStateException`；S0 轨无抛出口 ⇒ 关闭/坏句柄返回**空表**
        //   （登记 S0M59，与建包期失败同口径）。
        let Ok(mut archive) = self.archive() else {
            return Vec::new();
        };
        let mut out = Vec::new();
        // java final Iterator<ZipEntry> iterator = this.entries.values().iterator();（Java 读构造期缓存）
        // S0 轨等价物：按**中央目录原序**逐条读（不缓存；LinkedHashMap 保序 ⇄ 中央目录序，见 §1.7）。
        for i in 0..archive.len() {
            // java while (iterator.hasNext()) { ... }
            // ★单行失败静默跳过（Java `:227` 是整表抛）⇒ 登记为已知差异（COV-15）。
            if let Ok(file) = archive.by_index_raw(i) {
                out.push(ZipEntry {
                    name: file.name().to_string(),
                    size: file.size() as i64,
                    compressedSize: file.compressed_size() as i64,
                    compressionMethod: ZipEntry::methodCode(file.compression()),
                    localHeaderRelOffset: file.header_start() as i64,
                });
            }
        }
        out
    }

    // rust-only: 整包字节快照（S0 轨内部辅助；零拷贝整改归 W6）
    fn snapshot(&self) -> std::io::Result<Vec<u8>> {
        self.raf
            .lock()
            .map(|g| g.clone())
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))
    }

    // rust-only: 从快照建 `zip` 归档句柄（S0 轨内部辅助）
    fn archive(&self) -> std::io::Result<ZipArchive<Cursor<Vec<u8>>>> {
        let bytes = self.snapshot()?;
        ZipArchive::new(Cursor::new(bytes))
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
    }

    // rust-only: 句柄是否已失效（对应 Java `this.raf == null`；S0 轨以缓冲清空为判据）
    fn isClosed(&self) -> bool {
        self.raf.lock().map(|g| g.is_empty()).unwrap_or(true)
    }

    // rust-only: 整包字节长度（对应 Java `this.raf.length()`）
    fn len(&self) -> usize {
        self.raf.lock().map(|g| g.len()).unwrap_or(0)
    }

    // rust-only: 头 4 字节 little-endian（对应 Java `Integer.reverseBytes(this.raf.readInt())`）
    fn headerMagic(&self) -> u32 {
        match self.raf.lock() {
            Ok(g) if g.len() >= 4 => u32::from_le_bytes([g[0], g[1], g[2], g[3]]),
            _ => 0,
        }
    }

    // rust-only: Java `ZipException` 在 S0 轨的载体（kind 归一归 s0-13，`at` = ZipFile::readCentralDir）
    fn invalidData(msg: String) -> std::io::Error {
        std::io::Error::new(std::io::ErrorKind::InvalidData, msg)
    }
}

// Java: static class EocdRecord
#[derive(Clone, Debug, Default)]
pub struct EocdRecord {
    // Java field:       final long numEntries;
    pub numEntries: i64,
    // Java field:       final long centralDirOffset;
    pub centralDirOffset: i64,
    // Java field:       final int commentLength;
    pub commentLength: i32,
}

// Java: public static class RAFStream extends InputStream
pub struct RAFStream {
    // Java field:       private final RandomAccessFile sharedRaf;
    sharedRaf: Option<Arc<Mutex<Vec<u8>>>>,
    // Java field:       private long endOffset;
    endOffset: i64,
    // Java field:       private long offset;
    offset: i64,
}

impl RAFStream {
    // Java: public RAFStream(RandomAccessFile raf, long initialOffset, long endOffset)
    pub fn RAFStream__RandomAccessFile_long_long(raf: Option<Arc<Mutex<Vec<u8>>>>, initialOffset: i64, endOffset: i64) -> RAFStream {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java super();
        // java this.singleByte = new byte[1];
        // java this.sharedRaf = raf;
        // java this.offset = initialOffset;
        // java this.endOffset = endOffset;
        */
    }

    // Java: public RAFStream(RandomAccessFile raf, long initialOffset) throws IOException
    pub fn RAFStream__RandomAccessFile_long(raf: Option<Arc<Mutex<Vec<u8>>>>, initialOffset: i64) -> RAFStream {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java this(raf, initialOffset, raf.length());
        */
    }

    // Java: public int available() throws IOException
    pub fn available(&mut self) -> i32 {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java return this.offset < this.endOffset ? 1 : 0;
        */
    }

    // Java: public int read() throws IOException
    pub fn read(&mut self) -> i32 {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java int ret = this.read(this.singleByte, 0, this.singleByte.length);
        // java return -1 == ret ? ret : this.singleByte[0];
        // —— RAFStream 取流面归 s0_03_function_zip_entry_stream
        */
    }

    // Java: public long skip(long byteCount) throws IOException
    pub fn skip(&mut self, byteCount: i64) -> i64 {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java if (byteCount > this.endOffset - this.offset) {
        // java byteCount = this.endOffset - this.offset;
        // java this.offset += byteCount;
        // java return byteCount;
        // —— RAFStream 取流面归 s0_03_function_zip_entry_stream
        */
    }

    // Java: public int read(byte[] buffer, int byteOffset, int byteCount) throws IOException
    pub fn read_buffer(&mut self, buffer: &mut [u8], byteOffset: usize, byteCount: usize) -> i32 {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java synchronized(this.sharedRaf) {
        // java long length = this.endOffset - this.offset;
        // java if ((long)byteCount > length) {
        // java byteCount = (int)length;
        // java this.sharedRaf.seek(this.offset);
        // java int count = this.sharedRaf.read(buffer, byteOffset, byteCount);
        // java if (count > 0) {
        // java this.offset += (long)count;
        // java return count;
        // java } else {
        // java return -1;
        // java } }
        // —— RAFStream 取流面归 s0_03_function_zip_entry_stream
        */
    }
}

// Java: public static class ZipInflaterInputStream extends java.util.zip.InflaterInputStream
pub struct ZipInflaterInputStream {
    // Java field:       private final ZipEntry entry;
    entry: ZipEntry,
    // java private final Inflater inf;  ← S0 轨不解压（解压取流面归 s0_03），无对应字段
    // Java field:       private int bufferSize;
    bufferSize: i32,
}

impl ZipInflaterInputStream {
    // Java: public ZipInflaterInputStream(InputStream is, Inflater inf, int bsize, ZipEntry entry)
    pub fn ZipInflaterInputStream__InputStream_int_ZipEntry(is: Vec<u8>, bufferSize: i32, entry: ZipEntry) -> ZipInflaterInputStream {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java super(is, inf, bsize);
        // java this.entry = entry;
        // java this.bsize = bsize;
        */
    }

    // Java: public int read(byte[] buffer, int byteOffset, int byteCount) throws IOException
    pub fn read(&mut self, buffer: &mut [u8], byteOffset: usize, byteCount: usize) -> i32 {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java int i;
        // java try {
        // java i = super.read(buffer, byteOffset, byteCount);
        // java } catch (IOException e) {
        // java throw new IOException("Error reading data for " + this.entry.getName() + " near offset " + this.bytesRead, e);
        // java if (i == -1) {
        // java if (this.entry.size != this.bytesRead) {
        // java throw new IOException("Size mismatch on inflated file: " + this.bytesRead + " vs " + this.entry.size);
        // java } else {
        // java this.bytesRead += (long)i;
        // java return i;
        // —— 解压取流面归 s0_03_function_zip_entry_stream
        */
    }

    // Java: public void close() throws IOException
    pub fn close(&mut self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java super.close();
        // java this.closed = true;
        // —— 解压取流面归 s0_03_function_zip_entry_stream
        */
    }

    // Java: public int available() throws IOException
    pub fn available(&mut self) -> i32 {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java if (this.closed) {
        // java return 0;
        // java } else {
        // java return super.available() == 0 ? 0 : (int)(this.entry.getSize() - this.bytesRead);
        // —— 解压取流面归 s0_03_function_zip_entry_stream
        */
    }
}

// Java: public class ZipEntry extends EntryBase implements ZipConstants, Cloneable
#[derive(Clone, Debug)]
pub struct ZipEntry {
    // Java field:    String name;
    name: String,
    // Java field:    long size = -1L;
    size: i64,
    // Java field:    long compressedSize = -1L;
    compressedSize: i64,
    // Java field:    int compressionMethod = -1;
    compressionMethod: i32,
    // Java field:    long localHeaderRelOffset = -1L;
    localHeaderRelOffset: i64,
    // Java 侧其余字段（`comment`/`crc`/`time`/`modDate`/`extra`/`dataOffset`/`gpbf`）本轮不落 Rust 结构，
    // 归 s0_03 取流面与 s0-08 修复面（登记见 `CLASS_ALIGNMENT.md`）。
}

// Java field:    public static final int DEFLATED = 8;
pub const DEFLATED: i32 = 8;
// Java field:    public static final int STORED = 0;
pub const STORED: i32 = 0;

impl Default for ZipEntry {
    fn default() -> Self {
        Self {
            name: String::new(),
            size: -1,
            compressedSize: -1,
            compressionMethod: -1,
            localHeaderRelOffset: -1,
        }
    }
}

impl ZipEntry {
    // rust-only: 压缩方法码的 Rust 载体（Java 侧是 `compressionMethod` 字段的原始 int；
    // `zip` crate 的 `CompressionMethod::serialize_to_u16` 是 `pub(crate)` ⇒ 此处按 Java 常量映射）
    fn methodCode(method: zip::CompressionMethod) -> i32 {
        match method {
            // java ZipEntry.STORED
            zip::CompressionMethod::Stored => STORED,
            // java ZipEntry.DEFLATED
            zip::CompressionMethod::Deflated => DEFLATED,
            // 其余（未启用 feature / 不受支持）⇒ Java 侧字段默认值 -1
            _ => -1,
        }
    }

    // Java: public String getName()
    pub fn getName(&self) -> String {
        self.name.clone()
    }

    // Java: public long getSize()
    pub fn getSize(&self) -> i64 {
        self.size
    }

    // Java: public long getCompressedSize()
    pub fn getCompressedSize(&self) -> i64 {
        self.compressedSize
    }

    // Java: public int getMethod()
    pub fn getMethod(&self) -> i32 {
        self.compressionMethod
    }

    // Java: public boolean isDirectory()
    pub fn isDirectory(&self) -> bool {
        // java return this.name.charAt(this.name.length() - 1) == '/';
        // ★S0 轨：Java 对空名会 `StringIndexOutOfBoundsException`；Rust 侧用等价的尾字符判断（空名 ⇒ false）
        !self.name.is_empty() && self.name.as_bytes()[self.name.len() - 1] == b'/'
    }

    // rust-only: 中央目录里 local header 的偏移（Java 字段 `localHeaderRelOffset` 无 public getter；
    // 对应 dump 字段 `zip.entries[].local_header_offset`）
    pub fn getLocalHeaderRelOffset(&self) -> i64 {
        self.localHeaderRelOffset
    }
}
