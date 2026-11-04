// function_id=02_function_unzip
// status=loop — executable tests live beside src, not in features/
// cargo test --manifest-path libs/core-libs/io/wps-io/Cargo.toml --test 02_function_unzip
// src:  libs/core-libs/io/wps-io/src/cn/wps/io/xwpf/util/ZipEntrySource.rs
// test: libs/core-libs/io/wps-io/tests/cn/wps/io/xwpf/util/ZipEntrySource.rs

use core_libs_io::io::opc::internal::ZipHelper::ZipHelper;
use core_libs_io::io::xwpf::util::ZipEntrySource::{FakeZipEntry, ZipEntryRef, ZipEntrySource};
use core_libs_io::io::xwpf::util::ZipFileZipEntrySource::ZipFileZipEntrySource;
use core_libs_io::io::xwpf::util::ZipInputStreamZipEntrySource::{
    EntryEnumerator, ZipInputStreamZipEntrySource,
};
use core_libs_kfs::kfs::zip::ZipInputStream::ZipInputStream;
use std::fs::File;
use std::io::{Cursor, Read, Write};
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

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

fn zip_bytes_with(entries: &[(&str, &[u8])], method: zip::CompressionMethod) -> Vec<u8> {
    let cursor = Cursor::new(Vec::new());
    let mut writer = zip::ZipWriter::new(cursor);
    let options = zip::write::SimpleFileOptions::default().compression_method(method);
    for (name, bytes) in entries {
        writer.start_file(*name, options).unwrap();
        writer.write_all(bytes).unwrap();
    }
    writer.finish().unwrap().into_inner()
}

fn zip_bytes(entries: &[(&str, &[u8])]) -> Vec<u8> {
    zip_bytes_with(entries, zip::CompressionMethod::Deflated)
}

fn read_all(mut stream: impl Read) -> Vec<u8> {
    let mut bytes = Vec::new();
    stream.read_to_end(&mut bytes).unwrap();
    bytes
}

fn write_temp_zip(bytes: &[u8]) -> PathBuf {
    static SEQ: AtomicU64 = AtomicU64::new(0);
    let path = std::env::temp_dir().join(format!(
        "ioffice-02-unzip-{}-{}.zip",
        std::process::id(),
        SEQ.fetch_add(1, Ordering::Relaxed)
    ));
    std::fs::write(&path, bytes).unwrap();
    path
}

fn stream_source(entries: &[(&str, &[u8])]) -> ZipInputStreamZipEntrySource {
    let mut input = Cursor::new(zip_bytes(entries));
    ZipInputStreamZipEntrySource::try_new(&mut input).unwrap()
}

/// U1 / C01: 最小 deflate zip 能列出指定 entry 且流字节等于写入值。
#[test]
fn u1_min_zip_roundtrip_bytes() {
    let source = stream_source(&[("hello.txt", b"abc")]);
    assert_eq!(source.size(), 1);
    let entries = source.getEntries().as_ref().unwrap();
    assert_eq!(entries[0].getName(), "hello.txt");
    assert_eq!(read_all(source.getInputStream(&ref_of(&entries[0]))), b"abc");
}

/// U2 / C02: 最小 OPC 三件套名字保持，写入顺序可枚举。
#[test]
fn u2_lists_opc_well_known_names() {
    let source = stream_source(&[
        ("[Content_Types].xml", b"<Types/>"),
        ("_rels/.rels", b"<Relationships/>"),
        ("word/document.xml", b"<w:document/>"),
    ]);
    let names: Vec<&str> = source
        .getEntries()
        .as_ref()
        .unwrap()
        .iter()
        .map(|entry| entry.getName())
        .collect();
    assert_eq!(
        names,
        vec![
            "[Content_Types].xml",
            "_rels/.rels",
            "word/document.xml"
        ]
    );
}

/// U3 / C03: 非 zip 不得得到可用 ZipEntrySource。
#[test]
fn u3_rejects_non_zip() {
    let mut input = Cursor::new(b"not a zip".to_vec());
    assert!(ZipInputStreamZipEntrySource::try_new(&mut input).is_err());
}

/// U4 / C04: entry 名含 ../ 必须失败。
#[test]
fn u4_rejects_path_traversal_entry_name() {
    let mut input = Cursor::new(zip_bytes(&[("../evil.txt", b"bad")]));
    let error = match ZipInputStreamZipEntrySource::try_new(&mut input) {
        Ok(_) => panic!("path traversal entry was accepted"),
        Err(error) => error,
    };
    // Java: XWPFPackage.checkEntryNameLegal 抛 FileDamagedException（判据复用同一实现，文案=异常名）
    assert!(error.contains("FileDamagedException"), "{error}");
}

/// U5 / C05: 合法空 zip 是 0 entry，不是构造失败。
#[test]
fn u5_empty_zip_has_zero_entries() {
    let mut input = Cursor::new(zip_bytes(&[]));
    let source = ZipInputStreamZipEntrySource::try_new(&mut input).unwrap();
    assert_eq!(source.size(), 0);
    assert!(source.getEntries().as_ref().unwrap().is_empty());
}

/// U6 / C06: stored 压缩 + N 个 entry 保持写入顺序。
#[test]
fn u6_stored_n_entries_preserve_order() {
    let mut input = Cursor::new(zip_bytes_with(
        &[("a.txt", b"A"), ("b.txt", b"BB"), ("c.txt", b"CCC")],
        zip::CompressionMethod::Stored,
    ));
    let source = ZipInputStreamZipEntrySource::try_new(&mut input).unwrap();
    assert_eq!(source.size(), 3);
    let entries = source.getEntries().as_ref().unwrap();
    let names: Vec<&str> = entries.iter().map(|e| e.getName()).collect();
    assert_eq!(names, vec!["a.txt", "b.txt", "c.txt"]);
    assert_eq!(read_all(source.getInputStream(&ref_of(&entries[1]))), b"BB");
}

/// U7 / C07: 空 entry 可重复取流，两次读取得到同一空字节。
#[test]
fn u7_empty_entry_repeated_read() {
    let source = stream_source(&[("empty.bin", b"")]);
    let entries = source.getEntries().as_ref().unwrap();
    assert_eq!(read_all(source.getInputStream(&ref_of(&entries[0]))), b"");
    assert_eq!(read_all(source.getInputStream(&ref_of(&entries[0]))), b"");
}

/// U8 / C08: close 清空 entries；重复 close 不 panic；close 后 size=0。
#[test]
fn u8_stream_close_and_double_close() {
    let mut source = stream_source(&[("hello.txt", b"abc")]);
    source.close();
    assert!(source.getEntries().is_none());
    assert_eq!(source.size(), 0);
    source.close();
    assert!(source.getEntries().is_none());
    assert_eq!(source.size(), 0);
}

/// U9 / C09: origin flag 两值都解出同一字节（Java 忽略 flag，转调 getInputStream）。
#[test]
fn u9_origin_flag_both_values_same_bytes() {
    let source = stream_source(&[("hello.txt", b"abc")]);
    let entries = source.getEntries().as_ref().unwrap();
    let false_bytes = read_all(source.getInputStream_with_origin(&ref_of(&entries[0]), false));
    let true_bytes = read_all(source.getInputStream_with_origin(&ref_of(&entries[0]), true));
    assert_eq!(false_bytes, b"abc");
    assert_eq!(true_bytes, b"abc");
}

/// U10 / C10: `dyn ZipEntrySource` 与具体类型同一契约。
#[test]
fn u10_trait_object_enumerates_and_reads() {
    let concrete = stream_source(&[("hello.txt", b"abc")]);
    let mut source: Box<dyn ZipEntrySource> = Box::new(concrete);
    assert_eq!(ZipEntrySource::size(source.as_ref()), 1);
    let entry = source.getEntries().unwrap()[0].clone();
    assert_eq!(entry.getName(), "hello.txt");
    assert_eq!(
        read_all(ZipEntrySource::getInputStream(source.as_ref(), &ref_of(&entry)).unwrap()),
        b"abc"
    );
    assert_eq!(
        read_all(
            ZipEntrySource::getInputStream_with_flag(source.as_ref(), &ref_of(&entry), true).unwrap()
        ),
        b"abc"
    );
    assert!(source.as_any().downcast_ref::<ZipInputStreamZipEntrySource>().is_some());
    source.close();
    assert_eq!(ZipEntrySource::size(source.as_ref()), 0);
    assert!(source.getEntries().is_none());
}

/// U11 / C11: 路径后端与流后端同一 entries/字节；close 后 size=-1。
#[test]
fn u11_zip_file_path_backend_roundtrip() {
    let bytes = zip_bytes(&[("hello.txt", b"abc")]);
    let path = write_temp_zip(&bytes);
    let mut source = ZipFileZipEntrySource::from_path(path.to_str().unwrap()).unwrap();
    assert_eq!(source.size(), 1);
    assert_eq!(source.getName().as_deref(), path.to_str());
    let entries = source.getEntries().unwrap();
    assert_eq!(entries[0].getName(), "hello.txt");
    let entry = &entries[0];
    assert_eq!(
        read_all(source.getInputStream(&ref_of(entry), false).unwrap().unwrap()),
        b"abc"
    );
    assert_eq!(
        read_all(source.getInputStream_default(&ref_of(entry)).unwrap().unwrap()),
        b"abc"
    );
    let name = "hello.txt".to_string();
    // Java `ZipFile.getInputStream(entry, isOriginInput)`（kfs `ZipFile.java:144-187`）：
    //   `isOriginInput == false` ⇒ 需要解压时包 `ZipInflaterInputStream`（返回 "abc"）；
    //   `isOriginInput == true`  ⇒ **不解压**，直接给 deflate 原始字节（`:178` 的 `!isOriginInput`）。
    // ⚠️ 本轮对齐修正：旧 Rust 实现把该标志吞掉（`_isOriginInput`）⇒ 两个分支都返回 "abc"，
    //    与 Java 分叉；现在透传给 kfs `ZipFile.getInputStream__zipentry_boolean`。
    assert_eq!(
        read_all(source.getInputStream(&entry_named(&name), false).unwrap().unwrap()),
        b"abc"
    );
    let raw_stored = read_all(source.getInputStream(&entry_named(&name), true).unwrap().unwrap());
    assert_ne!(
        raw_stored, b"abc",
        "isOriginInput=true 不解压 ⇒ 不得等于明文（Java kfs ZipFile.java:178）"
    );
    let evil = FakeZipEntry::new("../evil.txt", b"x".to_vec());
    assert!(source.getInputStream(&ref_of(&evil), false).is_err());
    source.close();
    assert_eq!(source.size(), -1);
    assert!(source.getEntries().is_none());
    assert!(source.getInputStream(&entry_named(&name), false).unwrap().is_none());
    source.close();
    assert_eq!(source.size(), -1);
    let _ = std::fs::remove_file(&path);
}

/// U12 / C12: ZipHelper::openZipFile 返回 **kfs ZipFile 句柄**（Java 同型）；缺失/坏 ZIP 返回 None。
#[test]
fn u12_zip_helper_open_zip_file() {
    let bytes = zip_bytes(&[("hello.txt", b"abc")]);
    let path = write_temp_zip(&bytes);
    // Java: ZipFile zipFile = ZipHelper.openZipFile(path); this.zipArchive = new ZipFileZipEntrySource(zipFile);
    let zip_file = ZipHelper::openZipFile(path.to_str().unwrap()).expect("existing zip");
    let mut source = ZipFileZipEntrySource::new(zip_file);
    assert_eq!(source.size(), 1);
    let entries = source.getEntries().unwrap();
    assert_eq!(read_all(source.getInputStream(&ref_of(&entries[0]), false).unwrap().unwrap()), b"abc");
    source.close();

    assert!(ZipHelper::openZipFile("/no/such/ioffice-02-unzip-missing.zip").is_none());

    let bad = write_temp_zip(b"not a zip");
    assert!(ZipHelper::openZipFile(bad.to_str().unwrap()).is_none());
    let _ = std::fs::remove_file(&path);
    let _ = std::fs::remove_file(&bad);
}

/// U13 / C13: RAF/file 后端：`from_file` 与 `openZipFile_raf(PathBuf)` 都能枚举。
#[test]
fn u13_raf_and_file_backend() {
    let bytes = zip_bytes(&[("hello.txt", b"abc")]);
    let path = write_temp_zip(&bytes);
    let file = File::open(&path).unwrap();
    let from_file = ZipFileZipEntrySource::from_file(file, path.to_str().unwrap()).unwrap();
    assert_eq!(from_file.size(), 1);
    assert_eq!(from_file.getEntries().unwrap()[0].getName(), "hello.txt");

    let helper = ZipFileZipEntrySource::new(
        ZipHelper::openZipFile_raf(&path).expect("raf path"),
    );
    assert_eq!(helper.size(), 1);
    assert_eq!(
        read_all(
            helper
                .getInputStream(&ref_of(&helper.getEntries().unwrap()[0]), false)
                .unwrap()
                .unwrap()
        ),
        b"abc"
    );

    // Rust-only 路径入口（Rust 侧合并 Java 的两步）
    let via_new = ZipFileZipEntrySource::from_path(path.to_str().unwrap()).unwrap();
    assert_eq!(via_new.size(), 1);

    assert!(ZipHelper::openZipFile_raf(&42i32).is_none());
    let _ = std::fs::remove_file(&path);
}

/// U14 / C14: 截断 zip —— **流式后端按 Java 语义返回已收集的 entries（不是 Err）**；
/// 文件后端（Java: `new ZipFile(f)` 解析中央目录）仍必须拒绝。
#[test]
fn u14_truncated_stream_keeps_partial_entries_like_java() {
    let full = zip_bytes(&[("hello.txt", b"abcdefghij")]);
    assert!(full.len() > 8);
    let truncated = full[..full.len() / 2].to_vec();
    let mut input = Cursor::new(truncated.clone());
    // Java: getNextEntry/read 抛 IOException → catch → IOLog.e(TAG, "IOException", e)
    //       → Helper.close(inp); return;  ⇒ **返回已收集的 entries**，不向上抛。
    // 旧实现（zip crate）在这里整体 Err，等于把部分结果全部丢掉 —— 本轮对齐 Java 后行为变更。
    let source = ZipInputStreamZipEntrySource::try_new(&mut input)
        .expect("Java 语义：截断流的 IOException 被吞掉，返回已收集的 entries");
    assert_eq!(source.size(), 1, "截断点之前那条 entry 仍然可用");
    assert_eq!(source.getEntries().as_ref().unwrap()[0].getName(), "hello.txt");

    let path = write_temp_zip(&truncated);
    assert!(ZipFileZipEntrySource::from_path(path.to_str().unwrap()).is_err());
    assert!(ZipHelper::openZipFile(path.to_str().unwrap()).is_none());
    let _ = std::fs::remove_file(&path);
}

fn fixture_bytes(rel: &str) -> Vec<u8> {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(rel);
    std::fs::read(&path).unwrap_or_else(|e| panic!("missing fixture {}: {e}", path.display()))
}

/// C1-corpus: 真语料 physical-order names + uncompressed_size 对冻结 JSON。
#[test]
fn c1_corpus_entry_names_and_sizes() {
    let bytes = fixture_bytes("tests/fixtures/wave0-ooxml-standards/standard-hello.docx");
    assert_eq!(bytes.len(), 3502, "C1 size must match frozen sha256 input");
    let gold = include_str!("../../../../../fixtures/02_function_unzip/c1-zip-entries.json");
    assert!(gold.contains("c8740d24b0943ac2c52039d168b12e01db43b0dcde11d23476b0138eddca8110"));
    let mut input = Cursor::new(bytes);
    let source = ZipInputStreamZipEntrySource::try_new(&mut input).unwrap();
    let expected_names = [
        "[Content_Types].xml",
        "_rels/.rels",
        "word/_rels/document.xml.rels",
        "word/document.xml",
        "word/styles.xml",
        "word/settings.xml",
        "word/webSettings.xml",
        "word/fontTable.xml",
        "docProps/core.xml",
        "docProps/app.xml",
    ];
    let expected_sizes = [1333usize, 600, 698, 658, 273, 182, 186, 246, 638, 429];
    assert_eq!(source.size(), 10);
    let entries = source.getEntries().as_ref().unwrap();
    for (i, entry) in entries.iter().enumerate() {
        assert_eq!(entry.getName(), expected_names[i], "index {i}");
        // Java: FakeZipEntry(String,ZipInputStream) 只调 super(entryName) ⇒ getSize() 继承默认 -1
        // （`ZipEntry.java:19`）；真实长度只能靠读流。
        assert_eq!(entry.getSize(), -1, "Java ZipEntry.size 未 setSize ⇒ -1；index {i}");
        assert_eq!(read_all(source.getInputStream(&ref_of(entry))).len(), expected_sizes[i]);
    }
}

/// C9-dup: 重复 entry 名不得 panic；记录 zip crate 实际枚举。
#[test]
fn c9_duplicate_entry_names() {
    let bytes = fixture_bytes("tests/fixtures/02_function_unzip/C09-dup-entry-name.docx");
    let mut input = Cursor::new(bytes);
    let source = ZipInputStreamZipEntrySource::try_new(&mut input).expect("dup names are not a zip error");
    let names: Vec<String> = source
        .getEntries()
        .as_ref()
        .unwrap()
        .iter()
        .map(|e| e.getName().to_string())
        .collect();
    assert!(!names.is_empty());
    assert!(names.iter().all(|n| n == "dup.txt"));
}

/// C10-truncated: 截断语料按 Java 语义返回**截断点之前**的 entries（★实测 5/10 条），不报错。
/// ⚠️ 与旧 zip crate 实现（整体 Err）相反，是本轮对齐 Java 后的**行为变更**；
///    因此 s01 层的 `zip_corrupt` 判定不能依赖本入口的 Err（`not_zip` 仍可判定：见 `try_new` 的签名前置检查）。
#[test]
fn c10_truncated_corpus_keeps_partial_entries_like_java() {
    let bytes = fixture_bytes("tests/fixtures/02_function_unzip/C10-truncated.docx");
    let mut input = Cursor::new(bytes);
    let source = ZipInputStreamZipEntrySource::try_new(&mut input)
        .expect("Java 语义：截断语料的 IOException 被吞掉，返回已收集 entries");
    assert_eq!(source.size(), 5, "C1 语料 10 条，截断后剩前 5 条（★实测）");
    assert_eq!(
        source.getEntries().as_ref().unwrap()[0].getName(),
        "[Content_Types].xml"
    );
}

// ---------------------------------------------------------------------------
// 对齐 Java 构造器后新增的观测点（kfs ZipInputStream 流式路径）
// ---------------------------------------------------------------------------

/// 对齐-1: Java 签名入口 `ZipInputStreamZipEntrySource(ZipInputStream)` 可用，
/// 且与 `try_new(&mut dyn Read)` 适配入口得到同一结果。
#[test]
fn aligned_from_zip_input_stream_matches_reader_entry_point() {
    let bytes = zip_bytes(&[("a.txt", b"A"), ("b.txt", b"BB")]);
    let mut zis = ZipInputStream::new(Box::new(Cursor::new(bytes.clone())));
    let source = ZipInputStreamZipEntrySource::from_zip_input_stream(&mut zis);
    assert_eq!(source.size(), 2);
    let names: Vec<&str> = source
        .getEntries()
        .as_ref()
        .unwrap()
        .iter()
        .map(|e| e.getName())
        .collect();
    assert_eq!(names, vec!["a.txt", "b.txt"]);
    assert_eq!(
        read_all(source.getInputStream(&ref_of(&source.getEntries().as_ref().unwrap()[1]))),
        b"BB"
    );

    let mut input = Cursor::new(bytes);
    let via_reader = ZipInputStreamZipEntrySource::try_new(&mut input).unwrap();
    assert_eq!(via_reader.size(), source.size());
}

/// 对齐-2: 非法 entry 名在 **构造期** 就被拒（Java: `XWPFPackage.checkEntryNameLegal` 抛
/// FileDamagedException，unchecked ⇒ 从构造器直接抛出，不被 catch 吞掉）。
#[test]
#[should_panic(expected = "FileDamagedException")]
fn aligned_illegal_entry_name_panics_like_java() {
    let bytes = zip_bytes(&[("word/../evil.xml", b"x")]);
    let mut zis = ZipInputStream::new(Box::new(Cursor::new(bytes)));
    let _ = ZipInputStreamZipEntrySource::from_zip_input_stream(&mut zis);
}

/// 对齐-3: 名字门禁只有**一份判据实现**（OPCPackage 的 `contains("../")`），
/// 抛异常的 Java 外壳与 try_ 通道共用它。
#[test]
fn aligned_name_guard_uses_one_shared_judgement() {
    use core_libs_io::io::opc::OPCPackage::OPCPackage;
    use core_libs_io::io::xwpf::XWPFPackage::XWPFPackage;
    assert!(OPCPackage::try_checkEntryNameLegal("../evil").is_err());
    assert!(XWPFPackage::try_checkEntryNameLegal("../evil").is_err());
    assert!(XWPFPackage::try_checkEntryNameLegal("word/document.xml").is_ok());
    // Java 外壳：同一个判据 → panic（消息与旧实现一致，未改文案）
    let panicked = std::panic::catch_unwind(|| OPCPackage::checkEntryNameLegal("../evil"));
    assert!(panicked.is_err());
}

/// 对齐-4: 正常读完一条 entry 后 `Helper.close(inp)` 被调用（Java: 循环结束时 Helper.close(inp)）。
/// 可观测量：源构造完成后底层 kfs 流已 close ⇒ 再读必须失败（"Stream is closed"）。
#[test]
fn aligned_helper_close_is_called_when_loop_ends() {
    let bytes = zip_bytes(&[("only.txt", b"data")]);
    let mut zis = ZipInputStream::new(Box::new(Cursor::new(bytes)));
    let source = ZipInputStreamZipEntrySource::from_zip_input_stream(&mut zis);
    assert_eq!(source.size(), 1);
    let mut probe = [0u8; 1];
    let after_close = zis.try_read(&mut probe, 0, 1);
    assert!(
        after_close.is_err(),
        "构造结束后流应已关闭（Helper.close(inp)），实际: {after_close:?}"
    );
}

/// 对齐-5: 本地头**谎报**较小的 size ⇒ `FakeZipEntry` 按声明尺寸提前读完，
/// 随后 `closeEntry(inp)` 的收尾（skipAll + 回推修正）在记账不一致时报 IOException
/// ⇒ Java 语义：`closeEntry` 返回 false（ExecutionException → cancel(true)）
/// ⇒ 构造器**立刻 return**，而 `zipEntries.add(entry)` 在它之后 ⇒ 这条 entry 被丢弃，
/// 最终 0 条、不 panic（★实测，与 Java 逐语句顺序一致）。
#[test]
fn aligned_close_entry_failure_drops_entry_like_java() {
    let mut bytes = zip_bytes_with(
        &[("lie.bin", b"0123456789abcdef")],
        zip::CompressionMethod::Stored,
    );
    // 本地头：偏移 18 = compressed size、22 = uncompressed size（都改成 4）
    bytes[18..22].copy_from_slice(&4u32.to_le_bytes());
    bytes[22..26].copy_from_slice(&4u32.to_le_bytes());
    let mut input = Cursor::new(bytes);
    let source = ZipInputStreamZipEntrySource::try_new(&mut input)
        .expect("谎报尺寸不构成构造失败（Java 的 catch 只吞 IOException，不抛）");
    assert_eq!(
        source.size(),
        0,
        "closeEntry 返回 false → 该 entry 尚未 add，构造器直接 return"
    );
}

/// C11: 文本改后缀不得走 try_new 成功。
#[test]
fn c11_not_a_zip_corpus_rejected() {
    let bytes = fixture_bytes("tests/fixtures/02_function_unzip/C11-not-a-zip.docx");
    let mut input = Cursor::new(bytes);
    assert!(ZipInputStreamZipEntrySource::try_new(&mut input).is_err());
}

/// C12: 目录 entry + 0 字节 part 不失败；0 字节 part size==0。
#[test]
fn c12_directory_entry_and_zero_byte_part() {
    let bytes = fixture_bytes("tests/fixtures/02_function_unzip/C12-zero-byte-and-dir.docx");
    let mut input = Cursor::new(bytes);
    let source = ZipInputStreamZipEntrySource::try_new(&mut input).unwrap();
    let entries = source.getEntries().as_ref().unwrap();
    let names: Vec<&str> = entries.iter().map(|e| e.getName()).collect();
    assert!(names.iter().any(|n| *n == "word/empty.xml"));
    assert!(names.iter().any(|n| n.ends_with('/')), "directory entry should appear: {names:?}");
    let empty = entries.iter().find(|e| e.getName() == "word/empty.xml").unwrap();
    // Java: 声明尺寸未设置 ⇒ -1（0 字节由读流长度体现）
    assert_eq!(empty.getSize(), -1);
    assert!(read_all(source.getInputStream(&ref_of(empty))).is_empty());
}

/// E-new-swallow: 非 zip 经 `new` 得到 0 entry（Java catch 吞错对照）。
#[test]
fn e_new_swallows_parse_error() {
    let mut input = Cursor::new(b"not a zip".to_vec());
    let source = ZipInputStreamZipEntrySource::new(&mut input);
    assert_eq!(source.size(), 0);
}

/// EntryEnumerator: hasMoreElements/nextElement 保持 zip 物理序。
#[test]
fn enumerator_physical_order() {
    let source = stream_source(&[("a.txt", b"A"), ("b.txt", b"BB")]);
    let mut enumerator = EntryEnumerator::EntryEnumerator(&source);
    assert!(enumerator.hasMoreElements());
    assert_eq!(enumerator.nextElement().getName(), "a.txt");
    assert!(enumerator.hasMoreElements());
    let second = enumerator.nextElement();
    assert_eq!(second.getName(), "b.txt");
    assert_eq!(read_all(source.getInputStream(&ref_of(&second))), b"BB");
    assert!(!enumerator.hasMoreElements());
}

/// EntryEnumerator: 先取枚举器再 close 后仍可遍历（快照，不借用 zipEntries）。
#[test]
fn enumerator_survives_close() {
    let mut source = stream_source(&[("hello.txt", b"abc")]);
    let mut enumerator = EntryEnumerator::EntryEnumerator(&source);
    source.close();
    assert!(source.getEntries().is_none());
    assert!(enumerator.hasMoreElements());
    assert_eq!(enumerator.nextElement().getName(), "hello.txt");
    assert!(!enumerator.hasMoreElements());
}

/// FakeZipEntry: 目录名尾 `/` 走 ZipEntry.isDirectory 语义。
#[test]
fn fake_zip_entry_is_directory_from_name() {
    let dir = FakeZipEntry::new("word/", Vec::new());
    let file = FakeZipEntry::new("word/document.xml", b"<w/>".to_vec());
    assert!(dir.isDirectory());
    assert!(!file.isDirectory());
    assert_eq!(dir.getMethod(), -1);
    assert_eq!(file.getMethod(), -1);
}

/// FakeZipEntry(entryName, inp): 从 kfs `ZipInputStream` 物化 data，可重复 getInputStream。
#[test]
fn fake_zip_entry_ctor_reads_stream() {
    let bytes = zip_bytes(&[("part.bin", b"payload-bytes")]);
    let mut inp = ZipInputStream::new(Box::new(Cursor::new(bytes)));
    // Java: ZipEntry zipEntry = inp.getNextEntry();  → 流定位到该 entry 的数据
    let zip_entry = inp.getNextEntry().expect("first entry");
    assert_eq!(zip_entry.getName(), "part.bin");
    // Java: FakeZipEntry entry = new FakeZipEntry(entryName, inp);
    let entry = FakeZipEntry::FakeZipEntry("part.bin", &mut inp);
    assert_eq!(entry.getName(), "part.bin");
    // Java: ctor 只调 super(entryName) ⇒ `getSize()` = -1（长度 = 13 由 data 体现）
    assert_eq!(entry.getSize(), -1);
    assert_eq!(read_all(entry.getInputStream()), b"payload-bytes");
    assert_eq!(read_all(entry.getInputStream()), b"payload-bytes");
}

