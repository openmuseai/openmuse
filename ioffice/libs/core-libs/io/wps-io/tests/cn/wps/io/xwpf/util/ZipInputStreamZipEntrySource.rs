// function_id=17_function_zip_entry_stream
// cargo test --manifest-path libs/core-libs/io/wps-io/Cargo.toml --test 17_function_zip_entry_stream

use core_libs_io::io::opc::internal::ZipHelper::ZipHelper;
use core_libs_io::io::xwpf::util::ZipEntrySource::{FakeZipEntry, ZipEntryRef, ZipEntrySource};
use core_libs_io::io::xwpf::util::ZipFileZipEntrySource::ZipFileZipEntrySource;
use core_libs_io::io::xwpf::util::ZipInputStreamZipEntrySource::ZipInputStreamZipEntrySource;
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

fn write_temp_zip(bytes: &[u8]) -> PathBuf {
    static SEQ: AtomicU64 = AtomicU64::new(0);
    let path = std::env::temp_dir().join(format!(
        "ioffice-17-entry-stream-{}-{}.zip",
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

fn path_source(entries: &[(&str, &[u8])]) -> (ZipFileZipEntrySource, PathBuf) {
    let path = write_temp_zip(&zip_bytes(entries));
    // Java: ZipFile zipFile = ZipHelper.openZipFile(path); new ZipFileZipEntrySource(zipFile)
    let zip_file = ZipHelper::openZipFile(path.to_str().unwrap()).expect("path open");
    let source = ZipFileZipEntrySource::new(zip_file);
    (source, path)
}

fn read_all(mut stream: impl Read) -> Vec<u8> {
    let mut bytes = Vec::new();
    stream.read_to_end(&mut bytes).unwrap();
    bytes
}

fn sha256_hex(bytes: &[u8]) -> String {
    let mut child = std::process::Command::new("shasum")
        .args(["-a", "256"])
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .expect("shasum");
    {
        let stdin = child.stdin.as_mut().expect("stdin");
        stdin.write_all(bytes).unwrap();
    }
    let out = child.wait_with_output().expect("shasum wait");
    assert!(out.status.success(), "shasum failed: {}", String::from_utf8_lossy(&out.stderr));
    let text = String::from_utf8(out.stdout).unwrap();
    text.split_whitespace().next().unwrap().to_string()
}

#[test]
fn s_stream_read_returns_written_bytes() {
    let source = stream_source(&[("hello.txt", b"abc")]);
    let entries = source.getEntries().as_ref().unwrap();
    assert_eq!(read_all(source.getInputStream(&ref_of(&entries[0]))), b"abc");
}

#[test]
fn s_stream_repeated_read_is_stable() {
    let source = stream_source(&[("hello.txt", b"abc")]);
    let entries = source.getEntries().as_ref().unwrap();
    assert_eq!(read_all(source.getInputStream(&ref_of(&entries[0]))), b"abc");
    assert_eq!(read_all(source.getInputStream(&ref_of(&entries[0]))), b"abc");
}

#[test]
fn s_stream_zero_byte_entry() {
    let source = stream_source(&[("empty.bin", b"")]);
    let entries = source.getEntries().as_ref().unwrap();
    assert_eq!(read_all(source.getInputStream(&ref_of(&entries[0]))), b"");
    assert_eq!(read_all(source.getInputStream(&ref_of(&entries[0]))), b"");
}

#[test]
fn s_stream_origin_flag_ignored() {
    let source = stream_source(&[("hello.txt", b"abc")]);
    let entries = source.getEntries().as_ref().unwrap();
    let a = read_all(source.getInputStream_with_origin(&ref_of(&entries[0]), false));
    let b = read_all(source.getInputStream_with_origin(&ref_of(&entries[0]), true));
    assert_eq!(a, b"abc");
    assert_eq!(a, b);
}

#[test]
fn s_stream_read_after_close() {
    let mut source = stream_source(&[("hello.txt", b"abc")]);
    let entry = source.getEntries().as_ref().unwrap()[0].clone();
    source.close();
    assert_eq!(read_all(source.getInputStream(&ref_of(&entry))), b"abc");
}

#[test]
fn s_trait_stream_read() {
    let source = stream_source(&[("hello.txt", b"abc")]);
    let entries = source.getEntries().as_ref().unwrap();
    let as_trait: &dyn ZipEntrySource = &source;
    let stream = as_trait.getInputStream(&ref_of(&entries[0])).expect("trait stream");
    assert_eq!(read_all(stream), b"abc");
}

#[test]
fn s_path_read_by_name() {
    let (source, path) = path_source(&[("hello.txt", b"abc")]);
    let name = "hello.txt".to_string();
    let stream = source.getInputStream(&entry_named(&name), false).unwrap().expect("some");
    assert_eq!(read_all(stream), b"abc");
    let _ = std::fs::remove_file(&path);
}

#[test]
fn s_path_read_by_entry() {
    let (source, path) = path_source(&[("hello.txt", b"abc")]);
    let entry = FakeZipEntry::new("hello.txt", b"unused-payload".to_vec());
    let stream = source.getInputStream(&ref_of(&entry), false).unwrap().expect("some");
    assert_eq!(read_all(stream), b"abc");
    let _ = std::fs::remove_file(&path);
}

#[test]
fn s_path_default_matches_flag_false() {
    let (source, path) = path_source(&[("hello.txt", b"abc")]);
    let name = "hello.txt".to_string();
    let a = read_all(source.getInputStream(&entry_named(&name), false).unwrap().unwrap());
    let b = read_all(source.getInputStream_default(&entry_named(&name)).unwrap().unwrap());
    assert_eq!(a, b);
    let _ = std::fs::remove_file(&path);
}

#[test]
fn s_path_unknown_name_is_none() {
    let (source, path) = path_source(&[("hello.txt", b"abc")]);
    let name = "nope.txt".to_string();
    assert!(source.getInputStream(&entry_named(&name), false).unwrap().is_none());
    let _ = std::fs::remove_file(&path);
}

#[test]
fn s_path_traversal_rejected() {
    let (source, path) = path_source(&[("hello.txt", b"abc")]);
    let evil = FakeZipEntry::new("../evil.txt", b"x".to_vec());
    let err = match source.getInputStream(&ref_of(&evil), false) {
        Err(err) => err,
        Ok(_) => panic!("must reject"),
    };
    assert_eq!(err.kind(), std::io::ErrorKind::InvalidData);
    assert!(err.to_string().contains("Unsafe zip file"));
    let _ = std::fs::remove_file(&path);
}

#[test]
fn s_path_absent_entry_is_none() {
    // ★本用例原名 `s_path_untyped_any_is_none`，体是 `getInputStream(&42i32, false)` 期望 `None`
    //   —— 它断言的是**旧缺陷**：形参是 `&dyn Any`，转不上型就静默当作「entry 不存在」返回 None。
    //   现在形参是真实类型 `&ZipEntryRef` ⇒ 传 `&42i32` **编译期即被拒绝**（"任意类型塞进 entry 位"
    //   这一整类调用在 Rust 侧已不可能写出），故该断言无处安放、已删除。
    //   ⇒ 保留同一路径下**语义正确**的那一半：entry 类型对、但不在包里 ⇒ `Ok(None)`。
    let (source, path) = path_source(&[("hello.txt", b"abc")]);
    let absent = entry_named("not-in-archive.txt");
    assert!(
        source.getInputStream(&absent, false).unwrap().is_none(),
        "类型正确的 entry 但不在包里 ⇒ Ok(None)（Java: `zipArchive.getEntry(name)` 返回 null）"
    );
    let _ = std::fs::remove_file(&path);
}

#[test]
fn s_path_closed_source_is_none() {
    let (mut source, path) = path_source(&[("hello.txt", b"abc")]);
    source.close();
    let name = "hello.txt".to_string();
    assert!(source.getInputStream(&entry_named(&name), false).unwrap().is_none());
    let _ = std::fs::remove_file(&path);
}

#[test]
fn s_trait_path_not_found_is_err() {
    let (source, path) = path_source(&[("hello.txt", b"abc")]);
    let as_trait: &dyn ZipEntrySource = &source;
    // ★2026-09-20：旧写法这里留了个 `missing` 局部（`FakeZipEntry::new("nope.txt", …)`）但从未使用
    //   —— 因为真正传进去的是**名字字符串**（`dyn Any` + `downcast_ref::<String>()` 旁路）。
    //   真实类型下改为传一个"类型正确但不在包里"的 entry，签名与语义同时对齐 Java。
    let err = match as_trait.getInputStream(&entry_named("nope.txt")) {
        Err(err) => err,
        Ok(_) => panic!("trait not found"),
    };
    assert_eq!(err, "entry not found");
    let _ = std::fs::remove_file(&path);
}

#[test]
fn c1_entry_bytes_digests() {
    let bytes = std::fs::read(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures/wave0-ooxml-standards/standard-hello.docx"),
    )
    .unwrap();
    assert_eq!(bytes.len(), 3502);
    let mut input = Cursor::new(bytes);
    let source = ZipInputStreamZipEntrySource::try_new(&mut input).unwrap();
    let expected = [
        (
            "[Content_Types].xml",
            "7b620185757cce71161cacd693f09f9861ea5e2aed6535116bb2b5f3713c41a2",
        ),
        (
            "_rels/.rels",
            "f86f1893aecb510b25f41025ced61dd4a921d4188a6056e2f3fb17af8545e8e6",
        ),
        (
            "word/_rels/document.xml.rels",
            "c66dc0215e8f7a8d3ad50582287738b7759a7985ca428edbe6613e2451b8618a",
        ),
        (
            "word/document.xml",
            "a8a6a617255998545e01601291df9987cf26faf0c3c7d2a73825f6cff7133f71",
        ),
        (
            "word/styles.xml",
            "f637ffa3f50aa97d37d0ef2b17d7cf134fab2293d5dff3a4f9446c1ccafa8590",
        ),
        (
            "word/settings.xml",
            "e71adaa5b9eb3e974104b4f72184c61d1191f9e2b66bdecb64f61eaf8c9be17e",
        ),
        (
            "word/webSettings.xml",
            "f0ad5b8163102d0ffbad1286c3aa67f3754a75d10eda9d69d9e10480b504ca42",
        ),
        (
            "word/fontTable.xml",
            "ebbba141fa5a7d757eae618136c9d27323ddf1847f8c9fcbe55c3f6f3926aa20",
        ),
        (
            "docProps/core.xml",
            "9ce6943f07a077aefe86673406cc513727bc83bd2d4cc3fe7fca14efe6773639",
        ),
        (
            "docProps/app.xml",
            "1cc392edeba62e0fb2b88b65649774ed243fb4182a42e312aef83bc36ae58929",
        ),
    ];
    let entries = source.getEntries().as_ref().unwrap();
    assert_eq!(entries.len(), expected.len());
    for (i, entry) in entries.iter().enumerate() {
        assert_eq!(entry.getName(), expected[i].0, "index {i}");
        let digest = sha256_hex(&read_all(source.getInputStream(&ref_of(entry))));
        assert_eq!(digest, expected[i].1, "index {i} {}", entry.getName());
    }
}

#[test]
fn s_two_backends_same_bytes() {
    let bytes = zip_bytes(&[("hello.txt", b"abc")]);
    let mut input = Cursor::new(bytes.clone());
    let stream = ZipInputStreamZipEntrySource::try_new(&mut input).unwrap();
    let path = write_temp_zip(&bytes);
    let file = ZipFileZipEntrySource::new(ZipHelper::openZipFile(path.to_str().unwrap()).unwrap());
    let stream_entry = &stream.getEntries().as_ref().unwrap()[0];
    let file_entry = &file.getEntries().unwrap()[0];
    assert_eq!(stream_entry.getName(), file_entry.getName());
    assert_eq!(
        read_all(stream.getInputStream(&ref_of(stream_entry))),
        read_all(file.getInputStream(&ref_of(file_entry), false).unwrap().unwrap())
    );
    let _ = std::fs::remove_file(&path);
}

#[test]
fn s_two_cursors_independent() {
    let source = stream_source(&[("hello.txt", b"abc")]);
    let entries = source.getEntries().as_ref().unwrap();
    let mut a = source.getInputStream(&ref_of(&entries[0]));
    let mut b = source.getInputStream(&ref_of(&entries[0]));
    let mut a0 = [0u8; 1];
    let mut b0 = [0u8; 1];
    a.read_exact(&mut a0).unwrap();
    b.read_exact(&mut b0).unwrap();
    assert_eq!(a0, [b'a']);
    assert_eq!(b0, [b'a']);
    assert_eq!(read_all(a), b"bc");
    assert_eq!(read_all(b), b"bc");
}
