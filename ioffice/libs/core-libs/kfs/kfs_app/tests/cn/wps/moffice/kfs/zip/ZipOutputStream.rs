//! KFS `ZipOutputStream` 写侧对齐回归测试。
//!
//! 写侧最容易出错的三处（旧实现全中）：
//!   * `write(byte[],int,int)` 没接 deflater —— 方法 8 的数据被原样写出，但头/中央目录声称是压缩态；
//!   * CRC 从不更新 —— 数据描述符与中央目录里的 CRC 恒为 0；
//!   * `closeEntry()` 漏掉 `super.finish()` —— deflate 尾巴没刷出去（`tbytes`/`getTotalIn/Out` 也就对不上）。
//!
//! 这里写一个归档再用 kfs 自己的读侧读回来，等价于 Java 侧
//! “ZipOutputStream 写 → ZipInputStream 读” 的自洽性；外部实现的交叉验证
//! （python zipfile / unzip -t）针对 `written_archive_is_self_consistent_and_landed_on_disk`
//! 落盘的那个文件，见该用例注释。

use core_libs_kfs_app::kfs::zip::ZipEntry::ZipEntry;
use core_libs_kfs_app::kfs::zip::ZipInputStream::ZipInputStream;
use core_libs_kfs_app::kfs::zip::ZipOutputStream::ZipOutputStream;
use flate2::Crc;
use std::fs::File;
use std::io::{Cursor, Write};
use std::sync::{Arc, Mutex};

// ---------- 工具 ----------

/// `ZipOutputStream` 拿走 `Box<dyn Write>` 的所有权，所以用共享缓冲把字节取回来。
#[derive(Clone)]
struct SharedBuf(Arc<Mutex<Vec<u8>>>);

impl SharedBuf {
    fn new() -> Self {
        Self(Arc::new(Mutex::new(Vec::new())))
    }
    fn take(&self) -> Vec<u8> {
        self.0.lock().unwrap().clone()
    }
}

impl Write for SharedBuf {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.0.lock().unwrap().extend_from_slice(buf);
        Ok(buf.len())
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

fn crc32(data: &[u8]) -> i64 {
    let mut crc = Crc::new();
    crc.update(data);
    crc.sum() as i64
}

fn read_all(stream: &mut ZipInputStream) -> Vec<u8> {
    let mut out = Vec::new();
    let mut buffer = [0u8; 4096];
    let capacity = buffer.len();
    loop {
        let read = stream.read(&mut buffer, 0, capacity);
        if read <= 0 {
            break;
        }
        out.extend_from_slice(&buffer[..read as usize]);
    }
    out
}

fn deflated_payload() -> Vec<u8> {
    // 200KB 可压缩数据：反复跨越 512B 的 deflate 输出缓冲与 512B 的读缓冲
    b"the quick brown fox jumps over the lazy dog. ".repeat(4500)
}

const STORED_PAYLOAD: &[u8] = b"stored payload, written on the STORED path";

// ---------- 用例 ----------

#[test]
fn deflated_and_stored_entries_round_trip() {
    let payload = deflated_payload();
    let sink = SharedBuf::new();

    {
        let mut zos = ZipOutputStream::new(Box::new(sink.clone()));

        // 压缩条目：method = 8，头部三个字段留 0，真实值走数据描述符
        let mut deflated = ZipEntry::new__string("deflated.bin".to_string());
        deflated.setMethod(8);
        zos.putNextEntry(deflated);
        zos.write_buf(&payload, 0, payload.len());

        // 存储条目：Java 侧要求 putNextEntry 之前给出 crc/size，否则 ZipException
        let mut stored = ZipEntry::new__string("stored.txt".to_string());
        stored.setMethod(0);
        stored.setCrc(crc32(STORED_PAYLOAD));
        stored.setSize(STORED_PAYLOAD.len() as i64);
        stored.setCompressedSize(STORED_PAYLOAD.len() as i64);
        // 这一步内部会先 closeEntry(压缩条目)，正好覆盖“一个流里连续写多个条目”
        zos.putNextEntry(stored);
        zos.write_buf(STORED_PAYLOAD, 0, STORED_PAYLOAD.len());

        zos.finish();
        zos.close();
    }

    let archive = sink.take();
    assert_eq!(&archive[..4], &[0x50, 0x4b, 0x03, 0x04], "缺少局部文件头");

    let mut zis = ZipInputStream::new(Box::new(Cursor::new(archive)));
    let mut names = Vec::new();
    while let Some(entry) = zis.getNextEntry() {
        names.push(entry.getName().to_string());
        let data = read_all(&mut zis);
        match entry.getName() {
            "deflated.bin" => assert_eq!(data, payload, "压缩条目内容不一致"),
            "stored.txt" => assert_eq!(data, STORED_PAYLOAD, "存储条目内容不一致"),
            other => panic!("unexpected entry {other}"),
        }
    }
    assert_eq!(names, vec!["deflated.bin", "stored.txt"]);
}

#[test]
fn deflate_actually_compresses() {
    let payload = deflated_payload();
    let sink = SharedBuf::new();
    {
        let mut zos = ZipOutputStream::new(Box::new(sink.clone()));
        let mut entry = ZipEntry::new__string("big.bin".to_string());
        entry.setMethod(8);
        zos.putNextEntry(entry);
        zos.write_buf(&payload, 0, payload.len());
        zos.finish();
    }
    let archive = sink.take();

    // 写侧接上 deflater 之后，200KB 高度可压缩数据必须明显变小；
    // 旧实现是直通写（归档 ≈ payload + 头部开销），这条断言会失败。
    assert!(
        archive.len() < payload.len() / 4,
        "没有真正压缩：archive={} payload={}",
        archive.len(),
        payload.len()
    );

    // 回读后 CRC/尺寸都由数据描述符回填并校验过，内容必须完全一致
    let mut zis = ZipInputStream::new(Box::new(Cursor::new(archive)));
    assert!(zis.getNextEntry().is_some());
    assert_eq!(read_all(&mut zis), payload);
}

#[test]
fn write_without_active_entry_is_rejected_like_java() {
    let sink = SharedBuf::new();
    let mut zos = ZipOutputStream::new(Box::new(sink.clone()));
    let mut entry = ZipEntry::new__string("a.bin".to_string());
    entry.setMethod(8);
    zos.putNextEntry(entry);
    zos.write_buf(b"hello", 0, 5);

    // Java: finish() → closeEntry() → currentEntry = null，此后 write() 会在入口
    //       `if (this.currentEntry == null) throw new ZipException("No active entry")` 处报错。
    zos.finish();
    let panicked = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        zos.write_buf(b"x", 0, 1);
    }))
    .is_err();
    assert!(panicked, "没有 active entry 时写入必须失败");

    // 同一个门禁在 closeEntry() 之后也成立（Java 会等到 putNextEntry 才重新置位 currentEntry）
    let sink2 = SharedBuf::new();
    let mut zos2 = ZipOutputStream::new(Box::new(sink2));
    let mut entry2 = ZipEntry::new__string("b.bin".to_string());
    entry2.setMethod(8);
    zos2.putNextEntry(entry2);
    zos2.closeEntry();
    let panicked = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        zos2.write_buf(b"x", 0, 1);
    }))
    .is_err();
    assert!(panicked, "closeEntry 之后写入必须失败");
}

#[test]
fn stored_size_mismatch_is_rejected_like_java() {
    let sink = SharedBuf::new();
    let mut zos = ZipOutputStream::new(Box::new(sink.clone()));
    let mut entry = ZipEntry::new__string("bad.txt".to_string());
    entry.setMethod(0);
    entry.setCrc(crc32(b"abcdefghij"));
    entry.setSize(10);
    entry.setCompressedSize(10);
    zos.putNextEntry(entry);

    // 只写 4 字节，closeEntry 时 Java 会抛 ZipException("... size mismatch EntrySize is 10, tbytes is 4")
    zos.write_buf(b"abcd", 0, 4);
    let panicked = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        zos.closeEntry();
    }))
    .is_err();
    assert!(panicked, "STORED 条目 size 与 tbytes 不一致时必须报错");
}

#[test]
fn written_archive_is_self_consistent_and_landed_on_disk() {
    // 除了自洽回读，本用例把归档落到磁盘，供**独立实现**交叉验证：
    //   python3 -c "import zipfile,sys; z=zipfile.ZipFile(sys.argv[1]); print(z.testzip(), z.namelist())" <path>
    //   unzip -t <path>
    // 默认路径在临时目录，可用 KFS_ZIP_OUTPUT_PATH 覆盖。
    let path = std::env::var("KFS_ZIP_OUTPUT_PATH")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|_| std::env::temp_dir().join("kfs_zipoutput_roundtrip.zip"));

    let payload = deflated_payload();
    {
        let mut zos = ZipOutputStream::new(Box::new(File::create(&path).unwrap()));
        let mut entry = ZipEntry::new__string("on_disk.bin".to_string());
        entry.setMethod(8);
        zos.putNextEntry(entry);
        zos.write_buf(&payload, 0, payload.len());

        let mut stored = ZipEntry::new__string("small.txt".to_string());
        stored.setMethod(0);
        stored.setCrc(crc32(STORED_PAYLOAD));
        stored.setSize(STORED_PAYLOAD.len() as i64);
        stored.setCompressedSize(STORED_PAYLOAD.len() as i64);
        zos.putNextEntry(stored);
        zos.write_buf(STORED_PAYLOAD, 0, STORED_PAYLOAD.len());

        // Java 的 close() 会先 finish()（写 EOCD）再 def.end() 再关底层流
        zos.close();
    }

    let mut zis = ZipInputStream::new(Box::new(File::open(&path).unwrap()));
    assert!(zis.getNextEntry().is_some());
    assert_eq!(read_all(&mut zis), payload);
    assert!(zis.getNextEntry().is_some());
    assert_eq!(read_all(&mut zis), STORED_PAYLOAD);
    assert!(zis.getNextEntry().is_none());
    eprintln!("KFS_ZIP_OUTPUT_PATH={}", path.display());
}
