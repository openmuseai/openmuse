//! KFS `Zip64` 写侧（`insertZip64ExtendedInfoToExtras` / `refreshZip64ExtendedInfo`）
//! 与 `try_*` 错误层的对齐回归测试。
//!
//! 写侧 zip64 的真实用途是「先写占位记录、拿到真实尺寸后再回填」：
//!   `insertZip64ExtendedInfoToExtras` → 写数据 → `refreshZip64ExtendedInfo` → 写中央目录。
//! 这里不去造 4GB 文件，而是直接对字节布局与解析结果做断言：
//!   * insert 的 28 字节布局（header id=1、长度 24、size/csize/offset）；
//!   * method != 0 时 size/csize 先写 0，由 refresh 回填；
//!   * 0xFFFFFFFF 这种「4GB 以上/未知」标记值经 parse 后被真实值替换（含 >4GB 的数值）；
//!   * extras 放不下时 `try_` 返回 Err（旧实现只有 panic）。
//! 另外把带 zip64 extra 的归档落盘，供 python zipfile 这类独立实现复验 extra 布局。

use core_libs_kfs_app::kfs::zip::Zip64::Zip64;
use core_libs_kfs_app::kfs::zip::ZipEntry::ZipEntry;
use core_libs_kfs_app::kfs::zip::ZipInputStream::ZipInputStream;
use core_libs_kfs_app::kfs::zip::ZipOutputStream::ZipOutputStream;
use flate2::Crc;
use std::fs::File;
use std::io::{Cursor, Write};
use std::sync::{Arc, Mutex};

const MARKER: i64 = 4294967295; // Java: Zip64.MAX_ZIP_ENTRY_AND_ARCHIVE_SIZE

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

fn entry(name: &str, method: i32) -> ZipEntry {
    let mut ze = ZipEntry::new__string(name.to_string());
    ze.setMethod(method);
    ze
}

fn u64_at(bytes: &[u8], off: usize) -> i64 {
    i64::from_le_bytes([
        bytes[off],
        bytes[off + 1],
        bytes[off + 2],
        bytes[off + 3],
        bytes[off + 4],
        bytes[off + 5],
        bytes[off + 6],
        bytes[off + 7],
    ])
}

// ---------- insert 布局 ----------

#[test]
fn insert_writes_28_byte_record_for_stored_entry() {
    let mut ze = entry("stored.bin", 0);
    ze.setSize(1234);
    ze.setCompressedSize(1234);
    ze.localHeaderRelOffset = 0x1122334455;

    Zip64::try_insertZip64ExtendedInfoToExtras(&mut ze).expect("insert must succeed");

    let extra = ze.getExtra().expect("extra must be set");
    assert_eq!(extra.len(), 28, "zip64 记录固定 28 字节");
    assert_eq!(u16::from_le_bytes([extra[0], extra[1]]), 1, "header id 必须是 1");
    assert_eq!(u16::from_le_bytes([extra[2], extra[3]]), 24, "记录长度必须是 24");
    // method == 0 → 记录里带真实 size / compressedSize
    assert_eq!(u64_at(extra, 4), 1234);
    assert_eq!(u64_at(extra, 12), 1234);
    assert_eq!(u64_at(extra, 20), 0x1122334455);
}

#[test]
fn insert_elides_sizes_for_deflated_entry_and_refresh_fills_them() {
    let mut ze = entry("deflated.bin", 8);
    // 写之前尺寸未知 → 先插占位记录
    Zip64::try_insertZip64ExtendedInfoToExtras(&mut ze).expect("insert must succeed");
    {
        let extra = ze.getExtra().unwrap();
        assert_eq!(u64_at(extra, 4), 0, "method != 0 时 size 占位为 0（与 Java 一致）");
        assert_eq!(u64_at(extra, 12), 0, "method != 0 时 csize 占位为 0");
    }

    // 数据写完后拿到真实尺寸（这里是 >4GB 的数值），refresh 回填
    let big = 5_368_709_120_i64; // 5 GiB
    ze.setSize(big);
    ze.setCompressedSize(3_221_225_472); // 3 GiB
    ze.localHeaderRelOffset = 4_294_967_296; // 4 GiB
    Zip64::refreshZip64ExtendedInfo(&mut ze);

    let extra = ze.getExtra().unwrap();
    assert_eq!(u64_at(extra, 4), big);
    assert_eq!(u64_at(extra, 12), 3_221_225_472);
    assert_eq!(u64_at(extra, 20), 4_294_967_296);
}

#[test]
fn insert_keeps_existing_extras_after_the_record() {
    // Java: 新记录写在最前面，旧 extras 整体后移
    let mut ze = entry("with-extra.bin", 0);
    ze.setSize(7);
    ze.setCompressedSize(7);
    ze.setExtra(Some(vec![0x99, 0x99, 0x00, 0x00]));

    Zip64::try_insertZip64ExtendedInfoToExtras(&mut ze).unwrap();

    let extra = ze.getExtra().unwrap();
    assert_eq!(extra.len(), 28 + 4);
    assert_eq!(&extra[28..], &[0x99, 0x99, 0x00, 0x00], "旧 extras 必须保留在后面");

    // 读回来时 zip64 记录会被剥离，旧 extras 留下
    let mut read_back = entry("with-extra.bin", 0);
    read_back.setSize(7);
    read_back.setCompressedSize(7);
    read_back.setExtra(Some(extra.clone()));
    assert!(Zip64::try_parseZip64ExtendedInfo(&mut read_back, true).unwrap());
    assert_eq!(read_back.getExtra().unwrap(), &vec![0x99, 0x99, 0x00, 0x00]);
}

#[test]
fn insert_reports_full_extras_as_error() {
    let mut ze = entry("full.bin", 0);
    let original = vec![0u8; 65508]; // 65508 + 28 = 65536 > 65535
    ze.setExtra(Some(original.clone()));

    let error = Zip64::try_insertZip64ExtendedInfoToExtras(&mut ze)
        .expect_err("extras 放不下时必须返回 Err");
    assert!(
        error.to_string().contains("No space in extras"),
        "unexpected error: {error}"
    );
    assert_eq!(ze.getExtra().unwrap(), &original, "失败时不应改动 extras");
}

// ---------- parse：4GB 标记值 ----------

#[test]
fn parse_fills_above_4gb_values_from_markers() {
    let mut ze = entry("huge.bin", 0);
    ze.setSize(MARKER);
    ze.setCompressedSize(MARKER);
    ze.localHeaderRelOffset = MARKER;
    ze.setExtra(Some(vec![0u8; 0])); // 先占位，下面用 insert 造出规范记录
    Zip64::try_insertZip64ExtendedInfoToExtras(&mut ze).unwrap();

    // 模拟「中央目录里尺寸字段是 0xFFFFFFFF、真实值在 zip64 记录里」
    let mut real = entry("huge.bin", 0);
    real.setSize(MARKER);
    real.setCompressedSize(MARKER);
    real.localHeaderRelOffset = MARKER;
    {
        let stub = ze.getExtra().unwrap().clone();
        let mut extra = stub.clone();
        extra[4..12].copy_from_slice(&5_368_709_120_i64.to_le_bytes()); // 5 GiB
        extra[12..20].copy_from_slice(&4_294_967_297_i64.to_le_bytes());
        extra[20..28].copy_from_slice(&9_000_000_000_i64.to_le_bytes());
        real.setExtra(Some(extra));
    }

    assert!(Zip64::try_parseZip64ExtendedInfo(&mut real, true).unwrap());
    assert_eq!(real.getSize(), 5_368_709_120);
    assert_eq!(real.getCompressedSize(), 4_294_967_297);
    assert_eq!(real.localHeaderRelOffset, 9_000_000_000);
}

#[test]
fn parse_without_zip64_record_is_an_error_when_markers_present() {
    let mut ze = entry("no-extra.bin", 0);
    ze.setSize(MARKER); // 本地头声称「尺寸在 zip64 记录里」，但没有记录
    ze.setCompressedSize(MARKER);

    let error = Zip64::try_parseZip64ExtendedInfo(&mut ze, false)
        .expect_err("缺少 zip64 记录时必须返回 Err");
    assert!(
        error.to_string().contains("no zip64 extended information"),
        "unexpected error: {error}"
    );
    assert_eq!(error.kind(), std::io::ErrorKind::InvalidData);
}

#[test]
fn parse_keeps_markers_for_local_deflated_header_like_java() {
    // Java 的语义（刻意不对齐成“更好”的行为）：
    // fromCentralDirectory == false 且 method != 0 时不读 size/csize，
    // 也不读 localHeaderRelOffset（那一项只在中央目录里出现）——标记值原样留着。
    let mut ze = entry("local-deflated.bin", 8);
    ze.setSize(MARKER);
    ze.setCompressedSize(MARKER);
    Zip64::try_insertZip64ExtendedInfoToExtras(&mut ze).unwrap();

    let mut probe = entry("local-deflated.bin", 8);
    probe.setSize(MARKER);
    probe.setCompressedSize(MARKER);
    probe.setExtra(ze.getExtra().cloned());
    assert!(Zip64::try_parseZip64ExtendedInfo(&mut probe, false).unwrap());
    assert_eq!(probe.getSize(), MARKER);
    assert_eq!(probe.getCompressedSize(), MARKER);
}

// ---------- 与写入链路打通 ----------

#[test]
fn archive_with_zip64_extra_round_trips_and_lands_on_disk() {
    // 真实工作流：entry 带 zip64 占位记录 → 写数据 → closeEntry 时回填 → 读回来
    let payload = b"zip64 extra carrying entry".repeat(64);
    let mut crc = Crc::new();
    crc.update(&payload);

    let mut ze = entry("zip64-stored.bin", 0);
    ze.setSize(payload.len() as i64);
    ze.setCompressedSize(payload.len() as i64);
    ze.setCrc(crc.sum() as i64);
    // 本 entry 是该归档的第一个，本地头偏移就是 0（insert 会把当时的 localHeaderRelOffset 一起写进记录）
    ze.localHeaderRelOffset = 0;
    Zip64::try_insertZip64ExtendedInfoToExtras(&mut ze).unwrap();
    let inserted_extra = ze.getExtra().unwrap().clone();

    let sink = SharedBuf::new();
    {
        let mut zos = ZipOutputStream::new(Box::new(sink.clone()));
        zos.putNextEntry(ze.clone());
        zos.write_buf(&payload, 0, payload.len());
        zos.finish();
    }
    let archive = sink.take();

    // 读侧：extra 会被 parseZip64ExtendedInfo 剥离，数据照常读出
    let mut zis = ZipInputStream::new(Box::new(Cursor::new(archive.clone())));
    let read_entry = zis.getNextEntry().expect("first entry");
    assert_eq!(read_entry.getName(), "zip64-stored.bin");
    assert_eq!(read_all(&mut zis), payload);

    // Java 的 closeEntry 会把 entry 的 extra **原样**写进中央目录，并不会顺手 refresh 它
    // （Java 侧 insertZip64ExtendedInfoToExtras/refreshZip64ExtendedInfo 全仓没有调用方，
    //   属于公共工具 API）。这里断言这份“不refresh”的行为被原样保留，避免被“顺手修好”。
    let cdir_extra_start = archive
        .windows(inserted_extra.len())
        .rposition(|w| w == inserted_extra.as_slice())
        .expect("中央目录里应当出现 entry 的 extra 原始字节");
    assert!(cdir_extra_start > 0);

    // 落盘供独立实现（python zipfile）解析 extra 布局
    let path = std::env::var("KFS_ZIP64_OUTPUT_PATH")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|_| std::env::temp_dir().join("kfs_zip64_extra_roundtrip.zip"));
    File::create(&path).unwrap().write_all(&archive).unwrap();
    eprintln!("KFS_ZIP64_OUTPUT_PATH={}", path.display());
}

// ---------- try_ 错误层 ----------

#[test]
fn try_layer_reports_write_state_errors_instead_of_panicking() {
    // 没有 active entry：Java 抛 ZipException("No active entry") → try_ 返回 Err
    let sink = SharedBuf::new();
    let mut zos = ZipOutputStream::new(Box::new(sink.clone()));
    let error = zos
        .try_write_buf(b"data", 0, 4)
        .expect_err("no active entry must be an error");
    assert!(error.to_string().contains("No active entry"), "{error}");
    assert_eq!(error.kind(), std::io::ErrorKind::InvalidData);

    // 空流 finish：Java 抛 ZipException("No entries") → Err
    let error = zos.try_finish().expect_err("empty stream must be an error");
    assert!(error.to_string().contains("No entries"), "{error}");

    // 关闭后 finish：Java 抛 IOException("Stream is closed") → Err
    let sink2 = SharedBuf::new();
    let mut zos2 = ZipOutputStream::new(Box::new(sink2));
    let mut ze = entry("a.bin", 8);
    ze.setMethod(8);
    zos2.try_putNextEntry__zipentry_listener_callback(ze, None, None)
        .unwrap();
    zos2.try_write_buf(b"hello", 0, 5).unwrap();
    zos2.try_finish().unwrap();
    zos2.try_close().unwrap();
    let error = zos2.try_finish().expect_err("closed stream must be an error");
    assert!(error.to_string().contains("Stream is closed"), "{error}");
}

#[test]
fn try_layer_reports_stored_size_mismatch() {
    let sink = SharedBuf::new();
    let mut zos = ZipOutputStream::new(Box::new(sink));
    let mut ze = entry("bad.txt", 0);
    let mut crc = Crc::new();
    crc.update(b"abcdefghij");
    ze.setCrc(crc.sum() as i64);
    ze.setSize(10);
    ze.setCompressedSize(10);
    zos.try_putNextEntry__zipentry_listener_callback(ze, None, None)
        .unwrap();
    zos.try_write_buf(b"abcd", 0, 4).unwrap();

    let error = zos.try_closeEntry().expect_err("size mismatch must be an error");
    assert!(error.to_string().contains("size mismatch"), "{error}");
}

#[test]
fn try_layer_reports_io_failure_from_the_sink() {
    // 底层写入失败（磁盘满/管道断）必须原样冒泡，而不是被吞掉
    struct FailingSink;
    impl Write for FailingSink {
        fn write(&mut self, _buf: &[u8]) -> std::io::Result<usize> {
            Err(std::io::Error::new(
                std::io::ErrorKind::WriteZero,
                "disk full",
            ))
        }
        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    let mut zos = ZipOutputStream::new(Box::new(FailingSink));
    let mut ze = entry("a.bin", 8);
    ze.setMethod(8);
    let error = zos
        .try_putNextEntry__zipentry_listener_callback(ze, None, None)
        .expect_err("写局部头失败必须冒泡");
    assert_eq!(error.kind(), std::io::ErrorKind::WriteZero);
    assert!(error.to_string().contains("disk full"));
}

#[test]
fn try_layer_duplicate_entry_error() {
    let sink = SharedBuf::new();
    let mut zos = ZipOutputStream::new(Box::new(sink));
    let mut first = entry("dup.bin", 8);
    first.setMethod(8);
    zos.try_putNextEntry__zipentry_listener_callback(first, None, None)
        .unwrap();
    zos.try_closeEntry().unwrap();

    let mut second = entry("dup.bin", 8);
    second.setMethod(8);
    let error = zos
        .try_putNextEntry__zipentry_listener_callback(second, None, None)
        .expect_err("重名 entry 必须报错");
    assert!(error.to_string().contains("Entry already exists"), "{error}");
}
