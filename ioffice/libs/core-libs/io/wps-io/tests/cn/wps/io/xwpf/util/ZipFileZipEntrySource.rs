// function_id=16_function_zip_open
// cargo test --manifest-path libs/core-libs/io/wps-io/Cargo.toml --test 16_function_zip_open

use core_libs_io::io::opc::internal::ZipHelper::ZipHelper;
use core_libs_io::io::xwpf::util::ZipFileZipEntrySource::ZipFileZipEntrySource;
use core_libs_io::io::xwpf::util::ZipEntrySource::{FakeZipEntry, ZipEntryRef, ZipEntrySource};
use core_libs_io::io::xwpf::util::ZipInputStreamZipEntrySource::ZipInputStreamZipEntrySource;
use std::io::{Cursor, Read, Write};

// ── ★2026-09-20 测试助手：`ZipEntry` 已是**真实类型**（`ZipEntryRef` 封闭枚举），
//    `getInputStream*` 的形参不再接受 `&dyn Any` ⇒ 测试也要传真实 entry。
//    Java 侧调用方本来就持有 `ZipEntry` 对象；Rust 的枚举变体持有值，故由借用构造时需一次 clone。
fn ref_of(e: &FakeZipEntry) -> ZipEntryRef {
    ZipEntryRef::Fake(e.clone())
}

/// 按名字造一个 `ZipEntry`（旧测试用 `&"name".to_string()` 直接传名字 —— 那是 `dyn Any` +
//  `downcast_ref::<String>()` 的 Rust-only 旁路，真实类型下**编译期即被禁止**）。
/// 文件后端按 `entry.getName()` 回查中央目录，故 `data` 内容无关紧要。
fn entry_named(name: &str) -> ZipEntryRef {
    ZipEntryRef::Fake(FakeZipEntry::new(name, Vec::new()))
}
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

fn zip_bytes(entries: &[(&str, &[u8])]) -> Vec<u8> {
    let cursor = Cursor::new(Vec::new());
    let mut writer = zip::ZipWriter::new(cursor);
    let options = zip::write::SimpleFileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated);
    for (name, bytes) in entries {
        writer.start_file(*name, options).unwrap();
        writer.write_all(bytes).unwrap();
    }
    writer.finish().unwrap().into_inner()
}

fn temp_path(tag: &str) -> PathBuf {
    static SEQ: AtomicU64 = AtomicU64::new(0);
    std::env::temp_dir().join(format!(
        "ioffice-16-zip-open-{}-{}-{}.zip",
        tag,
        std::process::id(),
        SEQ.fetch_add(1, Ordering::Relaxed)
    ))
}

fn write_temp_zip(bytes: &[u8]) -> PathBuf {
    let path = temp_path("tmp");
    std::fs::write(&path, bytes).unwrap();
    path
}

fn read_all(mut stream: impl Read) -> Vec<u8> {
    let mut bytes = Vec::new();
    stream.read_to_end(&mut bytes).unwrap();
    bytes
}

/// Java 两步：`ZipFile zipFile = ZipHelper.openZipFile(path);` + `new ZipFileZipEntrySource(zipFile)`。
fn open_source(path: &str) -> ZipFileZipEntrySource {
    let zip_file = ZipHelper::openZipFile(path).expect("openZipFile must return a handle");
    ZipFileZipEntrySource::new(zip_file)
}

fn hello_bytes() -> Vec<u8> {
    zip_bytes(&[("hello.txt", b"abc")])
}

/// Java 两步的便利形态（Rust-only 入口 `from_path` 合并了 Java 的两步）。
fn hello_source() -> (ZipFileZipEntrySource, PathBuf) {
    let path = write_temp_zip(&hello_bytes());
    let source = ZipFileZipEntrySource::from_path(path.to_str().unwrap()).unwrap();
    (source, path)
}

/// Z-open-path：`openZipFile(path)` 成功 ⇒ 句柄可枚举，`size()`/`getName()` 与 Java 同口径。
#[test]
fn z_open_path_returns_handle() {
    let path = write_temp_zip(&hello_bytes());
    let zip_file = ZipHelper::openZipFile(path.to_str().unwrap()).expect("path open");
    // Java: `ZipFile.getName()` = 传入路径原文
    assert_eq!(zip_file.getName().to_string(), path.to_str().unwrap());
    let source = ZipFileZipEntrySource::new(zip_file);
    assert_eq!(source.size(), 1);
    assert_eq!(source.getEntries().unwrap()[0].getName(), "hello.txt");
    assert_eq!(source.getName().as_deref(), path.to_str());
    let _ = std::fs::remove_file(&path);
}

/// Z-open-path-missing：不存在路径 ⇒ `None`（**不 panic**，不变量 1）。
#[test]
fn z_open_path_missing_returns_none() {
    assert!(ZipHelper::openZipFile("/no/such/ioffice-16-missing.zip").is_none());
    assert!(ZipFileZipEntrySource::from_path("/no/such/ioffice-16-missing.zip").is_err());
}

/// Z-open-path-nonzip：非 zip（C11）⇒ `None`。
#[test]
fn z_open_path_non_zip_returns_none() {
    let bad = write_temp_zip(b"not a zip");
    assert!(ZipHelper::openZipFile(bad.to_str().unwrap()).is_none());
    assert!(ZipFileZipEntrySource::from_path(bad.to_str().unwrap()).is_err());
    let _ = std::fs::remove_file(&bad);
}

/// Z-open-path-truncated：截断包（C10）⇒ `None`（Java: 中央目录找不到 ⇒ IOException ⇒ null）。
#[test]
fn z_open_path_truncated_returns_none() {
    let full = hello_bytes();
    assert!(full.len() > 8);
    let truncated = full[..full.len() / 2].to_vec();
    let path = write_temp_zip(&truncated);
    assert!(ZipHelper::openZipFile(path.to_str().unwrap()).is_none());
    assert!(ZipFileZipEntrySource::from_path(path.to_str().unwrap()).is_err());
    let _ = std::fs::remove_file(&path);
}

/// Z-open-path-empty：0 字节文件 ⇒ `None`。
#[test]
fn z_open_path_empty_file_returns_none() {
    let path = temp_path("empty");
    std::fs::write(&path, b"").unwrap();
    // 0 字节：Java kfs `readCentralDir` 找不到 EOCD ⇒ IOException ⇒ null
    assert!(ZipHelper::openZipFile(path.to_str().unwrap()).is_none());
    let _ = std::fs::remove_file(&path);
}

/// Z-raf-string：`openZipFile_raf(&String)` ⇒ `Some`。
#[test]
fn z_raf_string_returns_handle() {
    let path = write_temp_zip(&hello_bytes());
    let as_string: String = path.to_str().unwrap().to_string();
    let source =
        ZipFileZipEntrySource::new(ZipHelper::openZipFile_raf(&as_string).expect("String 分支"));
    assert_eq!(source.size(), 1);
    let _ = std::fs::remove_file(&path);
}

/// Z-raf-pathbuf：`openZipFile_raf(&PathBuf)` ⇒ `Some`。
#[test]
fn z_raf_pathbuf_returns_handle() {
    let path = write_temp_zip(&hello_bytes());
    let source =
        ZipFileZipEntrySource::new(ZipHelper::openZipFile_raf(&path).expect("PathBuf 分支"));
    assert_eq!(source.size(), 1);
    let _ = std::fs::remove_file(&path);
}

/// Z-raf-invalid：非法 `Any` 类型 ⇒ `None`（**不 panic**）。
#[test]
fn z_raf_invalid_type_returns_none() {
    assert!(ZipHelper::openZipFile_raf(&42i32).is_none());
}

/// Z-new-zipfile：Java 签名构造 `new(ZipFile)` 与 Rust-only `from_path` 同结果。
/// （原 Z-new-untyped「`new(Box::new(42i32))` ⇒ 空壳」已退场：Java ctor **断言非空**
///   ⇒ 空状态只能由 `close()` 到达，见 `z_close_is_idempotent_and_size_is_minus_one`。）
#[test]
fn z_new_zipfile_matches_from_path() {
    let path = write_temp_zip(&hello_bytes());
    let via_new =
        ZipFileZipEntrySource::new(ZipHelper::openZipFile(path.to_str().unwrap()).unwrap());
    let via_from_path = ZipFileZipEntrySource::from_path(path.to_str().unwrap()).unwrap();
    assert_eq!(via_new.size(), via_from_path.size());
    assert_eq!(
        via_new.getEntries().unwrap()[0].getName(),
        via_from_path.getEntries().unwrap()[0].getName()
    );
    assert_eq!(via_new.getName(), via_from_path.getName());
    let _ = std::fs::remove_file(&path);
}

/// Z-from-file：`from_file(File, name)` 与 `from_path` 同结果（`(name, data)` 全等）。
#[test]
fn z_from_file_matches_from_path() {
    let path = write_temp_zip(&hello_bytes());
    let from_file = ZipFileZipEntrySource::from_file(
        std::fs::File::open(&path).unwrap(),
        path.to_str().unwrap(),
    )
    .unwrap();
    let from_path = ZipFileZipEntrySource::from_path(path.to_str().unwrap()).unwrap();
    let a = from_file.getEntries().unwrap();
    let b = from_path.getEntries().unwrap();
    assert_eq!(a.len(), b.len());
    for (x, y) in a.iter().zip(b.iter()) {
        assert_eq!(x.getName(), y.getName());
        assert_eq!(
            read_all(from_file.getInputStream(&ref_of(x), false).unwrap().unwrap()),
            read_all(from_path.getInputStream(&ref_of(y), false).unwrap().unwrap())
        );
    }
    let _ = std::fs::remove_file(&path);
}

/// Z-close：`close()` 幂等；之后 `size()==-1`；`getEntries()==None`。
#[test]
fn z_close_is_idempotent_and_size_is_minus_one() {
    let (mut source, path) = hello_source();
    assert_eq!(source.size(), 1);
    source.close();
    assert_eq!(source.size(), -1);
    assert!(source.getEntries().is_none());
    assert!(source.getName().is_none());
    source.close();
    assert_eq!(source.size(), -1);
    assert!(source.getEntries().is_none());
    let _ = std::fs::remove_file(&path);
}

/// Z-name-echo：`getName()` **逐字**等于传入路径（含 `//`、`./` 冗余段，不被规范化）。
#[test]
fn z_get_name_echoes_path_verbatim() {
    let path = write_temp_zip(&hello_bytes());
    let verbatim = format!(
        "{}//./{}",
        path.parent().unwrap().to_str().unwrap(),
        path.file_name().unwrap().to_str().unwrap()
    );
    let source = ZipFileZipEntrySource::from_path(&verbatim).unwrap();
    assert_eq!(source.getName().as_deref(), Some(verbatim.as_str()));
    let _ = std::fs::remove_file(&path);
}

/// Z-parity-bytes：同一份 zip 字节，路径后端与流后端的 `(name, data)` 序列逐项相等。
#[test]
fn z_path_backend_matches_stream_backend() {
    let bytes = zip_bytes(&[
        ("[Content_Types].xml", b"<Types/>"),
        ("word/document.xml", b"<w:document/>"),
        ("docProps/app.xml", b"<Properties/>"),
    ]);
    let path = write_temp_zip(&bytes);
    let file_source = ZipFileZipEntrySource::from_path(path.to_str().unwrap()).unwrap();
    let mut cursor = Cursor::new(bytes.clone());
    let stream_source = ZipInputStreamZipEntrySource::try_new(&mut cursor).unwrap();

    let file_entries = file_source.getEntries().unwrap();
    let stream_entries = stream_source.getEntries().as_ref().unwrap();
    assert_eq!(file_entries.len(), stream_entries.len());
    for (f, s) in file_entries.iter().zip(stream_entries.iter()) {
        assert_eq!(f.getName(), s.getName());
        assert_eq!(
            read_all(file_source.getInputStream(&ref_of(f), false).unwrap().unwrap()),
            read_all(ZipEntrySource::getInputStream(&stream_source, &ref_of(s)).unwrap())
        );
    }
    let _ = std::fs::remove_file(&path);
}

/// C1-open：真语料（10 entry）经路径入口打开，物理序与 `02` 的流后端一致。
#[test]
fn c1_open_corpus_by_path() {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/wave0-ooxml-standards/standard-hello.docx");
    let source = open_source(path.to_str().unwrap());
    assert_eq!(source.size(), 10);
    let names: Vec<&str> = source
        .getEntries()
        .unwrap()
        .iter()
        .map(|e| e.getName())
        .collect();
    assert_eq!(names[0], "[Content_Types].xml");
    assert_eq!(names[9], "docProps/app.xml");
}

/// Z-raf-vs-path：`openZipFile_raf(&path)` 与 `openZipFile(path)` 得到同一份 entries。
#[test]
fn z_raf_equals_path() {
    let path = write_temp_zip(&zip_bytes(&[
        ("a.xml", b"a"),
        ("b.xml", b"bb"),
        ("c.xml", b"ccc"),
    ]));
    let by_path = ZipFileZipEntrySource::from_path(path.to_str().unwrap()).unwrap();
    let by_raf =
        ZipFileZipEntrySource::new(ZipHelper::openZipFile_raf(&path).expect("raf 入口"));
    let a = by_path.getEntries().unwrap();
    let b = by_raf.getEntries().unwrap();
    assert_eq!(a.len(), b.len());
    for (x, y) in a.iter().zip(b.iter()) {
        assert_eq!(x.getName(), y.getName());
        assert_eq!(
            read_all(by_path.getInputStream(&ref_of(x), false).unwrap().unwrap()),
            read_all(by_raf.getInputStream(&ref_of(y), false).unwrap().unwrap())
        );
    }
    let _ = std::fs::remove_file(&path);
}

/// Z-order：物理序（中央目录写入序）在路径后端保持 —— kfs `ZipFile.entries()` 必须按
/// Java `LinkedHashMap` 语义返回，**不能**退化成 HashMap 迭代序。
#[test]
fn z_entry_physical_order_preserved() {
    let names = [
        "zzz-first.xml",
        "aaa-middle.xml",
        "mmm-last.xml",
        "[Content_Types].xml",
        "word/document.xml",
    ];
    let entries: Vec<(&str, &[u8])> = names.iter().map(|n| (*n, b"x".as_slice())).collect();
    let path = write_temp_zip(&zip_bytes(&entries));
    let source = ZipFileZipEntrySource::from_path(path.to_str().unwrap()).unwrap();
    let got: Vec<&str> = source
        .getEntries()
        .unwrap()
        .iter()
        .map(|e| e.getName())
        .collect();
    assert_eq!(got, names.to_vec());
    // 确定性：同路径两次打开得到同名序列
    let again = ZipFileZipEntrySource::from_path(path.to_str().unwrap()).unwrap();
    let got2: Vec<&str> = again
        .getEntries()
        .unwrap()
        .iter()
        .map(|e| e.getName())
        .collect();
    assert_eq!(got, got2);
    let _ = std::fs::remove_file(&path);
}

/// Z-dir-path：传目录路径（畸形输入）—— **期望值未采**（依赖底层 `read` 语义），
/// 保守断言「不 panic」（不变量 1）。
#[test]
fn z_open_directory_path() {
    let dir = std::env::temp_dir();
    let _outcome = ZipHelper::openZipFile(dir.to_str().unwrap());
    let _outcome_err = ZipFileZipEntrySource::from_path(dir.to_str().unwrap());
}

/// Z-lazy-stream（本轮新增）：路径后端的 `getInputStream` 走 kfs —— 从**文件**读，
/// 字节与物化缓存逐项相等（Java 侧同样从文件懒读，不整包进堆）。
#[test]
fn z_lazy_entry_stream_reads_from_file() {
    let payload = b"lazy-stream-payload-0123456789".repeat(64);
    let path = write_temp_zip(&zip_bytes(&[("payload.bin", payload.as_slice())]));
    let source = ZipFileZipEntrySource::from_path(path.to_str().unwrap()).unwrap();
    let entry = &source.getEntries().unwrap()[0];
    let from_file = read_all(source.getInputStream(&ref_of(entry), false).unwrap().unwrap());
    assert_eq!(from_file, payload);
    // Java: `getInputStream(entry)` 委托给 `getInputStream(entry, false)`
    let default_stream = read_all(source.getInputStream_default(&ref_of(entry)).unwrap().unwrap());
    assert_eq!(default_stream, payload);
    // 未知名字 ⇒ Ok(None)（Java: `ZipFile.getInputStream` 的判空路径）
    assert!(source
        .getInputStream(&entry_named("nope.bin"), false)
        .unwrap()
        .is_none());
    // 不安全名字 ⇒ Err("Unsafe zip file.")（Java `ZipFileZipEntrySource.java:41-44`）
    let evil = FakeZipEntry::new("../evil.txt", b"x".to_vec());
    assert!(source.getInputStream(&ref_of(&evil), false).is_err());
    let _ = std::fs::remove_file(&path);
}

/// Z-kfs-metadata（本轮新增）：`FakeZipEntry` 之外的元数据直接来自 kfs 中央目录
/// （声明尺寸 / 压缩方法）—— 证明「来源是 kfs `ZipFile`」而非自造解析器。
#[test]
fn z_entries_metadata_comes_from_kfs_central_dir() {
    let payload = vec![b'k'; 4096];
    let path = write_temp_zip(&zip_bytes(&[("big.bin", payload.as_slice())]));
    let source = ZipFileZipEntrySource::from_path(path.to_str().unwrap()).unwrap();
    let meta = source.getZipEntryMetadata("big.bin").expect("kfs ZipEntry");
    // Java `ZipEntry.getSize()`（未压缩声明尺寸）—— 与本 function 的物化字节数一致
    assert_eq!(meta.getSize(), payload.len() as i64);
    assert_eq!(meta.getCompressedSize() < meta.getSize(), true);
    assert_eq!(meta.getMethod(), 8);
    assert!(!meta.isDirectory());
    assert!(source.getZipEntryMetadata("nope.bin").is_none());

    // ★2026-09-18：`getEntries()` 的 entry 也带上同一份中央目录元数据（Java 文件后端的 entry
    // 就是 kfs `ZipEntry`）—— `19` 的尺寸门禁读的就是这里的 `getSize()`（声明尺寸，非字节长度）。
    let entries = source.getEntries().expect("entries");
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].getName(), "big.bin");
    assert_eq!(entries[0].getSize(), meta.getSize());
    assert_eq!(entries[0].getMethod(), meta.getMethod());
    assert!(!entries[0].isDirectory());
    let _ = std::fs::remove_file(&path);
}

/// 契约：`&dyn ZipEntrySource` 与具体类型同一行为（本 function 的交接面）。
#[test]
fn z_trait_object_contract() {
    let (mut source, path) = hello_source();
    {
        let as_trait: &dyn ZipEntrySource = &source;
        assert_eq!(as_trait.size(), 1);
        assert_eq!(as_trait.getEntries().unwrap()[0].getName(), "hello.txt");
        let entry = &as_trait.getEntries().unwrap()[0];
        assert_eq!(read_all(as_trait.getInputStream(&ref_of(entry)).unwrap()), b"abc");
    }
    // Rust-only: 借用结束后才能取 `&mut`（trait `close(&mut self)`）
    ZipEntrySource::close(&mut source);
    let as_trait: &dyn ZipEntrySource = &source;
    assert_eq!(as_trait.size(), -1);
    assert!(as_trait.getEntries().is_none());
    let _ = std::fs::remove_file(&path);
}
