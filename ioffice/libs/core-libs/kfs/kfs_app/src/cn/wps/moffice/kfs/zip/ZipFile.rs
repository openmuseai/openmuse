#![allow(
    non_snake_case,
    dead_code,
    unused_variables,
    unused_imports,
    unused_mut,
    unused_assignments,
    unused_parens
)]
// Java: import cn.wps.moffice.kfs.File
use crate::kfs::mfs::File::File;
// Java: import cn.wps.moffice.kfs.RandomAccessFile
use crate::kfs::mfs::RandomAccessFile::RandomAccessFile;
// Java: import cn.wps.moffice.kfs.zip.ZipEntry
use crate::kfs::zip::Inflater::Inflater;
use crate::kfs::zip::InflaterInputStream::InflaterInputStream;
use crate::kfs::zip::Streams::Streams;
use crate::kfs::zip::Zip64::Zip64;
use crate::kfs::zip::ZipEntry::ZipEntry;

use std::collections::HashMap;
use std::io::Read;

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

// Java: static class EocdRecord
#[derive(Default, Clone)]
pub struct EocdRecord {
    // Java field:       final long numEntries;
    pub numEntries: i64,
    // Java field:       final long centralDirOffset;
    pub centralDirOffset: i64,
    // Java field:       final int commentLength;
    pub commentLength: i32,
}

impl EocdRecord {
    pub fn new(numEntries: i64, centralDirOffset: i64, commentLength: i32) -> Self {
        Self {
            numEntries,
            centralDirOffset,
            commentLength,
        }
    }
}

// Java: public static class RAFStream extends InputStream
pub struct RAFStream {
    // Java field:       private final RandomAccessFile sharedRaf;
    sharedRaf: RandomAccessFile,
    // Java field:       private long endOffset;
    pub endOffset: i64,
    // Java field:       private long offset;
    pub offset: i64,
}

impl RAFStream {
    // Java: public RAFStream(RandomAccessFile raf, long initialOffset, long endOffset)
    pub fn new(raf: RandomAccessFile, initialOffset: i64, endOffset: i64) -> RAFStream {
        Self {
            sharedRaf: raf,
            endOffset,
            offset: initialOffset,
        }
    }

    // Java: public RAFStream(RandomAccessFile raf, long initialOffset) throws IOException
    pub fn new__raf_long(mut raf: RandomAccessFile, initialOffset: i64) -> RAFStream {
        let end = raf.length();
        Self {
            sharedRaf: raf,
            endOffset: end,
            offset: initialOffset,
        }
    }

    // Java: public int available() throws IOException
    pub fn available(&self) -> i32 {
        if self.offset < self.endOffset {
            1
        } else {
            0
        }
    }

    // Java: public int read() throws IOException
    pub fn read_byte(&mut self) -> i32 {
        let mut buf = [0u8; 1];
        let ret = self.read_buf(&mut buf, 0, 1);
        if ret == -1 {
            -1
        } else {
            buf[0] as i32
        }
    }

    // Java: public int read(byte[] buffer, int byteOffset, int byteCount) throws IOException
    pub fn read_buf(&mut self, buffer: &mut [u8], byteOffset: usize, mut byteCount: usize) -> i32 {
        let length = (self.endOffset - self.offset) as usize;
        if byteCount > length {
            byteCount = length;
        }
        if byteCount == 0 {
            return -1;
        }
        self.sharedRaf.seek(self.offset);
        let count =
            self.sharedRaf
                .read__bytearr_int_int(buffer, byteOffset as i32, byteCount as i32);
        if count > 0 {
            self.offset += count as i64;
            count
        } else {
            -1
        }
    }

    // Java: public long skip(long byteCount) throws IOException
    pub fn skip(&mut self, mut byteCount: i64) -> i64 {
        if byteCount > self.endOffset - self.offset {
            byteCount = self.endOffset - self.offset;
        }
        self.offset += byteCount;
        byteCount
    }
}

impl Read for RAFStream {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let ret = self.read_buf(buf, 0, buf.len());
        if ret < 0 {
            Ok(0)
        } else {
            Ok(ret as usize)
        }
    }
}

// Java: public class ZipFile implements Closeable, ZipConstants
pub struct ZipFile {
    // Java field:    private final String filename;
    pub filename: String,
    // Java field:    private File fileToDeleteOnClose;
    fileToDeleteOnClose: Option<String>,
    // Java field:    private RandomAccessFile raf;
    pub raf: Option<RandomAccessFile>,
    // Java field:    private final LinkedHashMap<String, ZipEntry> entries;
    entries: HashMap<String, ZipEntry>,
    // Rust-only: Java 用 LinkedHashMap ⇒ `entries()` 保持**中央目录写入序**（s01 dump 的
    //            `zip.entries[].index` 依赖它）；std 无插入序 Map，故另存一份名字序列。
    entryOrder: Vec<String>,
    // Java field:    private String comment;
    comment: Option<String>,
}

// Java: public class ZipFile implements Closeable, ZipConstants
impl ZipFile {
    // Java: public ZipFile(File file) throws ZipException, IOException
    pub fn new(file: &File) -> ZipFile {
        let path = file.getPath();
        let mut file_mut = File::new__string(path.clone());
        let raf = RandomAccessFile::new(&mut file_mut, "r");
        let mut zf = ZipFile {
            filename: path,
            fileToDeleteOnClose: None,
            raf: Some(raf),
            entries: HashMap::new(),
            entryOrder: Vec::new(),
            comment: None,
        };
        zf.readCentralDir();
        zf
    }

    // Java: public ZipFile(String name) throws IOException
    pub fn new__string(name: String) -> ZipFile {
        let mut file = File::new__string(name);
        Self::new(&file)
    }

    /// Compatibility aliases for generated uof-io call sites.
    pub fn new_from_file(path: &str) -> std::io::Result<ZipFile> {
        Self::try_new__string(path.to_string())
    }

    // Rust-native: `new__string` 的可恢复版本。
    // Java 侧 `public ZipFile(String name) throws IOException`；本仓移植把 throw 全写成了 panic
    // （本文件 10 处 + 被调类 `parseEocdRecord`/`Zip64::parseZip64EocdRecord*`/`ZipEntry::new__*`/
    //  `RandomAccessFile::new`），且 `RandomAccessFile` **没有 try_ 层** ⇒ 此处用 catch_unwind
    // 把 panic 收敛成 `io::Error`，等价 Java 的 `throws IOException` 语义，
    // 供 `ZipHelper::openZipFile`（Java: `catch (IOException) → return null`）这类调用方使用。
    // ⚠️ 这是**桥接**，不是最终形态：kfs 读侧补 try_ 层后应撤掉（登记在 16 FUNCTION.md §1.7）。
    pub fn try_new__string(name: String) -> std::io::Result<ZipFile> {
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| Self::new__string(name)))
            .map_err(|payload| {
                let message = payload
                    .downcast_ref::<&str>()
                    .map(|s| (*s).to_string())
                    .or_else(|| payload.downcast_ref::<String>().cloned())
                    .unwrap_or_else(|| "ZipException: unknown".to_string());
                std::io::Error::new(std::io::ErrorKind::InvalidData, message)
            })
    }

    /// Rust-native: `new__randomaccessfile` 的可恢复版本（同上桥接）。
    pub fn try_new_from_raf(raf: RandomAccessFile) -> std::io::Result<ZipFile> {
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| Self::new__randomaccessfile(raf)))
            .map_err(|payload| {
                let message = payload
                    .downcast_ref::<&str>()
                    .map(|s| (*s).to_string())
                    .or_else(|| payload.downcast_ref::<String>().cloned())
                    .unwrap_or_else(|| "ZipException: unknown".to_string());
                std::io::Error::new(std::io::ErrorKind::InvalidData, message)
            })
    }

    pub fn new_from_raf(raf: RandomAccessFile, _name: String) -> std::io::Result<ZipFile> {
        Ok(Self::new__randomaccessfile(raf))
    }

    pub fn getInputStreamStatic(
        raf: RandomAccessFile,
        entry: &ZipEntry,
        is_origin_input: bool,
    ) -> std::io::Result<Box<dyn Read>> {
        Ok(Self::getInputStream_static(raf, entry, is_origin_input))
    }

    // Java: public ZipFile(RandomAccessFile randomAccessFile) throws IOException
    pub fn new__randomaccessfile(randomAccessFile: RandomAccessFile) -> ZipFile {
        let mut zf = ZipFile {
            filename: String::new(),
            fileToDeleteOnClose: None,
            raf: Some(randomAccessFile),
            entries: HashMap::new(),
            entryOrder: Vec::new(),
            comment: None,
        };
        zf.readCentralDir();
        zf
    }

    // Java: public ZipFile(File file, int mode) throws IOException
    pub fn new__file_int(file: &File, mode: i32) -> ZipFile {
        let path = file.getPath();
        let mut file_mut = File::new__string(path.clone());
        let raf = RandomAccessFile::new(&mut file_mut, "r");
        let mut zf = ZipFile {
            filename: path.clone(),
            fileToDeleteOnClose: if (mode & OPEN_DELETE) != 0 {
                Some(path)
            } else {
                None
            },
            raf: Some(raf),
            entries: HashMap::new(),
            entryOrder: Vec::new(),
            comment: None,
        };
        zf.readCentralDir();
        zf
    }

    // Java: protected void finalize() throws IOException
    pub fn finalize(&mut self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION */
    }
    // Rust-only: Java 的 `finalize()` **空体**（无任何语句）；Rust 侧无 finalize 语义
    //            （资源释放走 `close()` / `Drop`）⇒ 按硬规则 11 停车。

    // Java: public void close() throws IOException
    pub fn close(&mut self) {
        if let Some(mut local_raf) = self.raf.take() {
            local_raf.close();
        }
        if let Some(ref path) = self.fileToDeleteOnClose {
            let _ = std::fs::remove_file(path);
        }
        self.fileToDeleteOnClose = None;
    }

    // Java: private void checkNotClosed()
    fn checkNotClosed(&self) {
        if self.raf.is_none() {
            panic!("IllegalStateException: Zip file closed");
        }
    }

    // Java: public Enumeration<? extends ZipEntry> entries()
    pub fn entries(&self) -> Vec<ZipEntry> {
        self.checkNotClosed();
        // Rust-only: 按 entryOrder（= Java LinkedHashMap 的插入序 = 中央目录序）返回；
        //            旧实现用 `entries.values()` ⇒ HashMap 迭代序，**物理序丢失**。
        self.entryOrder
            .iter()
            .filter_map(|name| self.entries.get(name).cloned())
            .collect()
    }

    // Java: public boolean hasMoreElements()
    /* DEFERRED_NON_FUNCTION
    pub fn hasMoreElements(&self) -> bool {
        todo!("DEFERRED_NON_FUNCTION");
    }
    */

    // Java: public ZipEntry nextElement()
    /* DEFERRED_NON_FUNCTION
    pub fn nextElement(&mut self) -> ZipEntry {
        todo!("DEFERRED_NON_FUNCTION");
    }
    */
    // Rust-only: 上面两个方法属 Java `entries()` 里的**匿名 Enumeration**；Rust 侧改为返回
    //            `Vec<ZipEntry>`（快照，无 live 游标）⇒ 语义等价、结构不同（见 §1.7 登记）。

    // Java: public String getComment()
    pub fn getComment(&self) -> &Option<String> {
        self.checkNotClosed();
        &self.comment
    }

    // Java: public ZipEntry getEntry(String entryName)
    pub fn getEntry(&self, entryName: &str) -> Option<&ZipEntry> {
        self.checkNotClosed();
        let ze = self.entries.get(entryName);
        if ze.is_some() {
            return ze;
        }
        let with_slash = format!("{}/", entryName);
        self.entries.get(&with_slash)
    }

    // Java: public InputStream getInputStream(ZipEntry entry) throws IOException
    // Rust-only: 收 `&self`（Java 该方法不改状态；旧实现误写成 `&mut self` ⇒ 迫使上层用 RefCell 包句柄）
    pub fn getInputStream(&self, entry: &ZipEntry) -> Option<Box<dyn Read>> {
        self.getInputStream__zipentry_boolean(entry, false)
    }

    // Java: public InputStream getInputStream(ZipEntry entry, boolean isOriginInput) throws IOException
    pub fn getInputStream__zipentry_boolean(
        &self,
        entry: &ZipEntry,
        isOriginInput: bool,
    ) -> Option<Box<dyn Read>> {
        let entry_name = entry.getName().to_string();
        let found = self.entries.get(&entry_name)?.clone();

        let mut file = File::new__string(self.filename.clone());
        let raf = RandomAccessFile::new(&mut file, "r");
        Some(Self::getInputStream_static(raf, &found, isOriginInput))
    }

    // Java: public static InputStream getInputStream(RandomAccessFile raf, ZipEntry entry) throws IOException
    pub fn getInputStream__raf_entry(raf: RandomAccessFile, entry: &ZipEntry) -> Box<dyn Read> {
        Self::getInputStream_static(raf, entry, false)
    }

    // Java: public static InputStream getInputStream(RandomAccessFile raf, ZipEntry entry, boolean isOriginInput) throws IOException
    fn getInputStream_static(
        raf: RandomAccessFile,
        entry: &ZipEntry,
        isOriginInput: bool,
    ) -> Box<dyn Read> {
        let mut raf_stream = RAFStream::new__raf_long(raf, entry.localHeaderRelOffset);
        let mut hdr = [0u8; 30];
        Streams::readFully(&mut raf_stream, &mut hdr, 0, 30);

        let local_magic = i32::from_le_bytes([hdr[0], hdr[1], hdr[2], hdr[3]]);
        if local_magic != 0x04034b50_u32 as i32 {
            panic!(
                "ZipException: Local File Header signature not found; was {}",
                local_magic
            );
        }

        let gpbf = u16::from_le_bytes([hdr[6], hdr[7]]) as i32;
        if (gpbf & 1) != 0 {
            panic!("ZipException: Invalid General Purpose Bit Flag: {}", gpbf);
        }

        let file_name_length = u16::from_le_bytes([hdr[26], hdr[27]]) as i64;
        let extra_field_length = u16::from_le_bytes([hdr[28], hdr[29]]) as i64;
        raf_stream.skip(file_name_length + extra_field_length);

        if entry.compressionMethod == 0 {
            raf_stream.endOffset = raf_stream.offset + entry.size;
            Box::new(raf_stream)
        } else {
            raf_stream.endOffset = raf_stream.offset + entry.compressedSize;
            if entry.getMode() != 1 && !isOriginInput {
                let buf_size = std::cmp::max(1024, std::cmp::min(entry.getSize(), 65535) as i32);
                let inf_stream = InflaterInputStream::new__input_deflater_int(
                    Box::new(raf_stream),
                    // Java: new ZipInflaterInputStream(rafStream, new Inflater(true), bufSize, entry);
                    // Rust: inf 是 java.util.zip.Inflater（nowrap=true == raw deflate），
                    //       旧实现误传了项目自己的 Deflater（java.util.zip.Deflater 的子类）。
                    Inflater::new(true),
                    buf_size,
                );
                Box::new(InflaterInputStreamReader { inner: inf_stream })
            } else {
                Box::new(raf_stream)
            }
        }
    }

    // Java: public String getName()
    pub fn getName(&self) -> &str {
        &self.filename
    }

    // Java: static void throwZipException(String msg, int magic) throws ZipException
    pub fn throwZipException(msg: &str, magic: i32) {
        // Java: throw new ZipException(msg + " signature not found; was " + magic);
        crate::kfs::zip::ZipErrors::panicLikeJava(&std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            format!("ZipException: {} signature not found; was {}", msg, magic),
        ));
    }

    // Java: public int size()
    pub fn size(&self) -> usize {
        self.checkNotClosed();
        self.entries.len()
    }

    // Java: private void readCentralDir() throws IOException
    fn readCentralDir(&mut self) {
        let raf = self.raf.as_mut().unwrap();
        let file_length = raf.length();
        let mut scanOffset: i64 = file_length - 22;
        if scanOffset < 0 {
            panic!(
                "ZipException: File too short to be a zip file: {}",
                file_length
            );
        }

        raf.seek(0);
        let headerMagic = raf.readInt().swap_bytes();
        if headerMagic == 0x06054b50_u32 as i32 {
            panic!("ZipException: Empty zip archive not supported");
        }
        if headerMagic != 0x04034b50_u32 as i32 {
            panic!("ZipException: Not a zip archive");
        }

        let stopOffset: i64 = std::cmp::max(scanOffset - 65536, 0);

        loop {
            raf.seek(scanOffset);
            let sig = raf.readInt().swap_bytes();
            if sig == 0x06054b50_u32 as i32 {
                let zip64_eocd_offset = Zip64::parseZip64EocdRecordLocator(raf, scanOffset);

                let mut record =
                    Self::parseEocdRecord(raf, scanOffset + 4, zip64_eocd_offset != -1);

                if record.commentLength > 0 {
                    let mut comment_bytes = vec![0u8; record.commentLength as usize];
                    raf.readFully(&mut comment_bytes);
                    self.comment = Some(String::from_utf8_lossy(&comment_bytes).to_string());
                }

                if zip64_eocd_offset != -1 {
                    record =
                        Zip64::parseZip64EocdRecord(raf, zip64_eocd_offset, record.commentLength);
                }

                let filename = self.filename.clone();
                let mut central_file = File::new__string(filename);
                let central_raf = RandomAccessFile::new(&mut central_file, "r");
                let mut raf_stream = RAFStream::new__raf_long(central_raf, record.centralDirOffset);

                for _i in 0..record.numEntries {
                    let mut hdr_buf = [0u8; 46];
                    let new_entry = ZipEntry::new__bytearr_inputstream_charset_boolean(
                        &mut hdr_buf,
                        &mut raf_stream,
                        "UTF-8",
                        zip64_eocd_offset != -1,
                    );

                    if new_entry.localHeaderRelOffset >= record.centralDirOffset {
                        panic!("ZipException: Local file header offset is after central directory");
                    }

                    let entry_name = new_entry.getName().to_string();
                    if self.entries.contains_key(&entry_name) {
                        panic!("ZipException: Duplicate entry name: {}", entry_name);
                    }
                    // Rust-only: 中央目录序（Java LinkedHashMap 的插入序）
                    self.entryOrder.push(entry_name.clone());
                    self.entries.insert(entry_name, new_entry);
                }
                return;
            }
            scanOffset -= 1;
            if scanOffset < stopOffset {
                break;
            }
        }
        panic!("ZipException: End Of Central Directory signature not found");
    }

    // Java: private static EocdRecord parseEocdRecord(RandomAccessFile raf, long offset, boolean isZip64) throws IOException
    fn parseEocdRecord(raf: &mut RandomAccessFile, offset: i64, isZip64: bool) -> EocdRecord {
        raf.seek(offset);
        let mut eocd = [0u8; 18];
        raf.readFully(&mut eocd);

        let numEntries: i64;
        let centralDirOffset: i64;

        if isZip64 {
            numEntries = -1;
            centralDirOffset = -1;
        } else {
            let disk_number = u16::from_le_bytes([eocd[0], eocd[1]]) as i32;
            let disk_with_central_dir = u16::from_le_bytes([eocd[2], eocd[3]]) as i32;
            numEntries = u16::from_le_bytes([eocd[4], eocd[5]]) as i64;
            let total_num_entries = u16::from_le_bytes([eocd[6], eocd[7]]) as i32;
            centralDirOffset =
                (u32::from_le_bytes([eocd[12], eocd[13], eocd[14], eocd[15]])) as i64;

            if numEntries != total_num_entries as i64
                || disk_number != 0
                || disk_with_central_dir != 0
            {
                panic!("ZipException: Spanned archives not supported");
            }
        }

        let commentLength = u16::from_le_bytes([eocd[16], eocd[17]]) as i32;
        EocdRecord {
            numEntries,
            centralDirOffset,
            commentLength,
        }
    }
}

struct InflaterInputStreamReader {
    inner: InflaterInputStream,
}

impl Read for InflaterInputStreamReader {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let ret = self.inner.read__bytearr_int_int(buf, 0, buf.len());
        if ret < 0 {
            Ok(0)
        } else {
            Ok(ret as usize)
        }
    }
}
