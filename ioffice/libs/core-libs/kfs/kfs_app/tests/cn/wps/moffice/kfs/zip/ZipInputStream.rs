//! KFS `ZipInputStream` 对齐回归测试。
//!
//! 这些用例瞄准的是 Java↔Rust 对齐历史上真正出错的地方：
//!   * `closeEntry()` 必须把“读多了的尾巴”回推（PushbackInputStream），否则下一个 entry 头错位；
//!   * method=8 必须真正解压（走 Inflater 而不是原样透传压缩字节）；
//!   * 每一步 `read()` 都要刷新 CRC，`closeEntry()` 才能通过 CRC/Size 校验；
//!   * 数据描述符（GPBF bit3）必须在 closeEntry 时回填 crc/size；
//!   * ZIP64 本地头（size 字段 = 0xFFFFFFFF）要真的走 `setCrc/setSize/setCompressedSize`。
//!
//! ZIP 字节在测试里手工构造（不引入 zip crate），这样每个字段都可控，
//! 包括“压缩数据跨 512 字节读缓冲”这种回推路径的触发条件。

use core_libs_kfs_app::kfs::zip::Deflater::Deflater;
use core_libs_kfs_app::kfs::zip::Inflater::Inflater;
use core_libs_kfs_app::kfs::zip::ZipInputStream::ZipInputStream;
use flate2::write::DeflateEncoder;
use flate2::{Compression, Crc};
use std::io::{Cursor, Write};

// ---------- 构造工具 ----------

fn crc32(data: &[u8]) -> u32 {
    let mut crc = Crc::new();
    crc.update(data);
    crc.sum()
}

// raw deflate（不带 zlib 头），与 new Inflater(true) 对应
fn deflate_raw(data: &[u8]) -> Vec<u8> {
    let mut encoder = DeflateEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(data).unwrap();
    encoder.finish().unwrap()
}

struct Spec {
    name: &'static str,
    data: Vec<u8>,
    // 0 = STORED, 8 = DEFLATED
    method: u16,
    // GPBF bit3：使用数据描述符
    data_descriptor: bool,
    // 本地头 size/csize = 0xFFFFFFFF + zip64 extended info
    zip64: bool,
    // 本地头 size/csize = 0xFFFFFFFF，但 extra 里只有无关记录、**没有** zip64 记录（悬空标记）
    // 注意：extra 必须存在（长度 > 0），否则 ZipInputStream 根本不会去解析 zip64（Java 同样如此）
    dangling_zip64: bool,
    // 本地头 size/csize = 0xFFFFFFFF，且完全没有 extra（长度 0）→ Java 不解析 zip64，不报错
    no_extra_zip64: bool,
    // 故意写坏数据描述符里的 CRC
    bad_descriptor_crc: bool,
}

fn spec(name: &'static str, data: &[u8], method: u16) -> Spec {
    Spec {
        name,
        data: data.to_vec(),
        method,
        data_descriptor: false,
        zip64: false,
        dangling_zip64: false,
        no_extra_zip64: false,
        bad_descriptor_crc: false,
    }
}

fn build_zip(specs: &[Spec]) -> Vec<u8> {
    let mut out: Vec<u8> = Vec::new();
    for item in specs {
        let payload = if item.method == 8 {
            deflate_raw(&item.data)
        } else {
            item.data.clone()
        };
        let crc = crc32(&item.data);
        let name = item.name.as_bytes();

        let mut extra: Vec<u8> = Vec::new();
        if item.dangling_zip64 {
            // 一个无关的 extra 记录（id=0x9999, length=0），只为了让 extraLength > 0
            extra.extend_from_slice(&0x9999u16.to_le_bytes());
            extra.extend_from_slice(&0u16.to_le_bytes());
        }
        if item.zip64 {
            // zip64 extended info: header id = 1，长度 16，依次是 uncompressed / compressed size
            extra.extend_from_slice(&1u16.to_le_bytes());
            extra.extend_from_slice(&16u16.to_le_bytes());
            extra.extend_from_slice(&(item.data.len() as u64).to_le_bytes());
            extra.extend_from_slice(&(payload.len() as u64).to_le_bytes());
        }

        // local file header（注意 ZipInputStream 读进 hdrBuf 的 26 字节是从文件偏移 4 开始的）
        out.extend_from_slice(&0x04034b50u32.to_le_bytes());
        // version needed：Java 侧是 `peekShort(0) & 255 > 20` 就报错，所以这里必须 <= 20
        out.extend_from_slice(&20u16.to_le_bytes());
        let gpbf: u16 = if item.data_descriptor { 8 } else { 0 };
        out.extend_from_slice(&gpbf.to_le_bytes());
        out.extend_from_slice(&item.method.to_le_bytes());
        out.extend_from_slice(&0u16.to_le_bytes()); // last mod time
        out.extend_from_slice(&33u16.to_le_bytes()); // last mod date (1980-01-01)
        if item.data_descriptor {
            out.extend_from_slice(&0u32.to_le_bytes());
            out.extend_from_slice(&0u32.to_le_bytes());
            out.extend_from_slice(&0u32.to_le_bytes());
        } else if item.zip64 || item.dangling_zip64 || item.no_extra_zip64 {
            out.extend_from_slice(&crc.to_le_bytes());
            out.extend_from_slice(&0xFFFFFFFFu32.to_le_bytes());
            out.extend_from_slice(&0xFFFFFFFFu32.to_le_bytes());
        } else {
            out.extend_from_slice(&crc.to_le_bytes());
            out.extend_from_slice(&(payload.len() as u32).to_le_bytes());
            out.extend_from_slice(&(item.data.len() as u32).to_le_bytes());
        }
        out.extend_from_slice(&(name.len() as u16).to_le_bytes());
        out.extend_from_slice(&(extra.len() as u16).to_le_bytes());
        out.extend_from_slice(name);
        out.extend_from_slice(&extra);
        out.extend_from_slice(&payload);

        if item.data_descriptor {
            let descriptor_crc = if item.bad_descriptor_crc { !crc } else { crc };
            out.extend_from_slice(&0x08074b50u32.to_le_bytes());
            out.extend_from_slice(&descriptor_crc.to_le_bytes());
            out.extend_from_slice(&(payload.len() as u32).to_le_bytes());
            out.extend_from_slice(&(item.data.len() as u32).to_le_bytes());
        }
    }
    // central directory 签名：ZipInputStream 见到它就置 entriesEnd 并返回 null
    out.extend_from_slice(&0x02014b50u32.to_le_bytes());
    out
}

fn open(bytes: Vec<u8>) -> ZipInputStream {
    ZipInputStream::new(Box::new(Cursor::new(bytes)))
}

// Java 调用方（ZipInputStreamZipEntrySource.FakeZipEntry）的读法：read(buffer) 直到 <= 0
fn read_all(stream: &mut ZipInputStream) -> Vec<u8> {
    let mut out = Vec::new();
    let mut buffer = [0u8; 1024];
    let capacity = buffer.len();
    loop {
        // Rust: Java 风格的 read(buf, off, len) 签名在这里要先把长度取出来（借用检查）
        let read = stream.read(&mut buffer, 0, capacity);
        if read <= 0 {
            break;
        }
        out.extend_from_slice(&buffer[..read as usize]);
    }
    out
}

// ---------- 用例 ----------

#[test]
fn stored_then_deflated_entries_stay_aligned() {
    // 小 entry 的压缩数据远小于 512 字节读缓冲 —— 这会强制 closeEntry 走
    // diff = entryIn - inB 的回推路径，是“下一个 entry 头错位”那个 bug 的探针。
    let mut specs = vec![
        spec("stored.txt", b"stored payload", 0),
        spec("deflated.txt", &vec![b'x'; 5000], 8),
        spec("small.txt", b"hi", 8),
    ];
    specs[1].data = b"hello deflate world, hello deflate world".repeat(20);
    let zip = build_zip(&specs);

    let mut stream = open(zip);
    let mut names = Vec::new();
    while let Some(entry) = stream.getNextEntry() {
        names.push(entry.getName().to_string());
        let data = read_all(&mut stream);
        match entry.getName() {
            "stored.txt" => assert_eq!(data, b"stored payload"),
            "deflated.txt" => assert_eq!(data, specs[1].data),
            "small.txt" => assert_eq!(data, b"hi"),
            other => panic!("unexpected entry {other}"),
        }
    }
    // 名称顺序正确 == 每个 entry 的头部偏移都正确
    assert_eq!(names, vec!["stored.txt", "deflated.txt", "small.txt"]);
}

#[test]
fn data_descriptor_entry_is_sized_from_descriptor() {
    // 流式写入的 entry：本地头里 crc/size 都是 0，真实值在数据描述符里。
    let mut item = spec("streamed.bin", &vec![7u8; 3000], 8);
    item.data_descriptor = true;
    let zip = build_zip(&[item]);

    let mut stream = open(zip);
    let entry = stream.getNextEntry().expect("first entry");
    assert_eq!(entry.getName(), "streamed.bin");
    let data = read_all(&mut stream);
    assert_eq!(data, vec![7u8; 3000]);
    // 数据描述符在 closeEntry 里被读取并校验（CRC + Size 都要对得上）
    assert!(stream.try_closeEntry().is_ok());
}

#[test]
fn crc_mismatch_is_reported() {
    // DEFLATED + 数据描述符（流式写入的常见形态），把 DD 里的 CRC 写坏
    let mut item = spec("broken.bin", &b"corrupted payload".repeat(30), 8);
    item.data_descriptor = true;
    item.bad_descriptor_crc = true;
    let zip = build_zip(&[item]);

    let mut stream = open(zip);
    assert!(stream.getNextEntry().is_some());
    let _ = read_all(&mut stream);
    let error = stream
        .try_closeEntry()
        .expect_err("CRC mismatch must be reported");
    assert!(
        error.to_string().contains("CRC mismatch"),
        "unexpected error: {error}"
    );
}

#[test]
fn stored_entry_with_data_descriptor_fails_like_java() {
    // 记录一个 Java 自身的限制（Rust 侧刻意保持一致，不要“顺手修好”）：
    // STORED entry 的读取依赖本地头里的 size，而数据描述符形态下本地头 size = 0，
    // 于是 read() 立刻返回 -1、skipAll 什么都没消费，closeEntry 会在数据区起点
    // 直接去读描述符 → `unknown format (EXTSIG=...)`。
    // 真实归档里流式写入的 entry 都是 DEFLATED，所以 Java 一直是这个行为。
    let mut item = spec("stored-streamed.bin", b"payload", 0);
    item.data_descriptor = true;
    let zip = build_zip(&[item]);

    let mut stream = open(zip);
    assert!(stream.getNextEntry().is_some());
    assert!(read_all(&mut stream).is_empty());
    let error = stream
        .try_closeEntry()
        .expect_err("must fail the same way Java does");
    assert!(
        error.to_string().contains("unknown format (EXTSIG="),
        "unexpected error: {error}"
    );
}

#[test]
fn zip64_local_header_sizes_are_applied() {
    // 本地头 size/csize = 0xFFFFFFFF，真实尺寸在 zip64 extended info 里。
    // 旧实现的条件 `ceSize != 0xFFFFFFFF` 会跳过 setCrc/setSize，导致 size 一直是 -1。
    let mut item = spec("zip64-stored.bin", &vec![3u8; 70000], 0);
    item.zip64 = true;
    let zip = build_zip(&[item]);

    let mut stream = open(zip);
    let entry = stream.getNextEntry().expect("first entry");
    assert_eq!(entry.getName(), "zip64-stored.bin");
    assert_eq!(entry.getSize(), 70000);
    assert_eq!(entry.getCompressedSize(), 70000);
    let data = read_all(&mut stream);
    assert_eq!(data.len(), 70000);
    assert!(data.iter().all(|b| *b == 3));
}

#[test]
fn dangling_zip64_marker_is_reported_as_error() {
    // 本地头的 size/csize 是 0xFFFFFFFF（“尺寸在 zip64 记录里”），extra 存在但没有 zip64 记录。
    // Java 在 Zip64.parseZip64ExtendedInfo 里抛 ZipException；
    // Rust 侧 ZipInputStream::try_getNextEntry 现在把这个错误**返回**（而不是内部 panic）。
    // 反例（同样对齐 Java）：extra 长度就是 0 时，Java 直接 currentEntryIsZip64 = false，
    // 根本不解析 zip64，也就不会报错 —— 见 dangling_zip64_without_extras_is_not_an_error。
    let mut item = spec("dangling.bin", b"whatever", 8);
    item.dangling_zip64 = true;
    let zip = build_zip(&[item]);

    let mut stream = open(zip);
    let error = stream
        .try_getNextEntry()
        .expect_err("悬空的 zip64 标记必须报错");
    assert!(
        error.to_string().contains("no zip64 extended information"),
        "unexpected error: {error}"
    );
}

#[test]
fn dangling_zip64_without_extras_is_not_an_error() {
    // Java: extraLength == 0 → 不调用 Zip64.parseZip64ExtendedInfo，直接 currentEntryIsZip64 = false。
    // 所以“没有任何 extra”的悬空标记不会报错（这是 Java 的既有语义，不要“顺手修好”）。
    let mut item = spec("no-extra.bin", b"whatever", 8);
    item.no_extra_zip64 = true;
    let zip = build_zip(&[item]);

    let mut stream = open(zip);
    let entry = stream
        .try_getNextEntry()
        .expect("extra 为空时不应报错（与 Java 一致）")
        .expect("应当读到 entry");
    assert_eq!(entry.getSize(), 4294967295);
}

#[test]
fn available_follows_java_semantics() {
    let zip = build_zip(&[spec("a.txt", b"abcdef", 0)]);
    let mut stream = open(zip);

    // Java: currentEntry == null 时 available() 返回 1（条件整体为假）
    assert_eq!(stream.available(), 1);

    assert!(stream.getNextEntry().is_some());
    assert_eq!(stream.available(), 1);
    let _ = read_all(&mut stream);
    // inRead >= size → 0
    assert_eq!(stream.available(), 0);

    // 消费完 entry 后 getNextEntry 会看到中央目录签名并返回 None
    assert!(stream.getNextEntry().is_none());
}

#[test]
fn deflater_inflater_round_trip_uses_raw_deflate() {
    let source = b"raw deflate round trip payload".repeat(50);

    let mut deflater = Deflater::new(-1, true);
    let mut compressed = Vec::new();
    deflater.setInput(&source, 0, source.len() as i32);
    deflater.finish();
    let mut buffer = [0u8; 4096];
    let capacity = buffer.len() as i32;
    loop {
        let written = deflater.deflate(&mut buffer, 0, capacity, 4);
        compressed.extend_from_slice(&buffer[..written as usize]);
        if deflater.finished() {
            break;
        }
        if written == 0 && deflater.needsInput() {
            panic!("deflater made no progress");
        }
    }
    assert_eq!(deflater.getTotalIn(), source.len() as i32);
    assert_eq!(deflater.getTotalOut(), compressed.len() as i32);

    let mut inflater = Inflater::new(true);
    let mut restored = Vec::new();
    let mut offset = 0usize;
    loop {
        if inflater.needsInput() && offset < compressed.len() {
            let len = std::cmp::min(4096, compressed.len() - offset);
            inflater.setInput(&compressed, offset as i32, len as i32);
            offset += len;
        }
        let produced = inflater.inflate(&mut buffer, 0, capacity as usize).unwrap();
        restored.extend_from_slice(&buffer[..produced as usize]);
        if inflater.finished() {
            break;
        }
        if produced == 0 && offset >= compressed.len() {
            break;
        }
    }
    assert_eq!(restored, source);
}

#[test]
fn closed_stream_matches_java_exception_mapping() {
    let zip = build_zip(&[spec("a.txt", b"abc", 0)]);
    let mut stream = open(zip);
    stream.close();
    let error = stream
        .try_read(&mut [0u8; 1], 0, 1)
        .expect_err("read after close must fail");
    assert!(
        error.to_string().contains("Stream is closed"),
        "unexpected error: {error}"
    );
    // panic 外壳保留 Java 的抛出语义与异常名
    let panicked = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        stream.available();
    }))
    .is_err();
    assert!(panicked);
}
