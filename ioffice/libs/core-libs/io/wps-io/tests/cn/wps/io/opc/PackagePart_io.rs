// function_id=31_function_part_io
// status=loop — executable tests live beside src, not in features/
// cargo test --manifest-path libs/core-libs/io/wps-io/Cargo.toml --test 31_function_part_io
// src:  libs/core-libs/io/wps-io/src/cn/wps/io/opc/PackagePart.rs（`getInputStream*` / `getOutputStream*` / `setContentType`）
//       libs/core-libs/io/wps-io/src/cn/wps/io/opc/OPCPackage.rs（`getPartsImpl` 造 part；`part_from_entry`）
// test: libs/core-libs/io/wps-io/tests/cn/wps/io/opc/PackagePart_io.rs
//
// 本 function = **part 的输入/输出流**。
// Java：
//   `PackagePart.getInputStream()`（`:191`）⇒ `try { return this.getInputStreamImpl(false); } catch (Exception e) { throw new IOException("Can't obtain the input stream from " + this._partName.getName()); }`
//   `PackagePart.getInputStream(boolean)`（`:182`）⇒ 同上（参数透传）
//   `getInputStreamImpl` 由子类覆写：`ZipPackagePart` = `this.getPackage().getZipArchive().getInputStream(this.zipEntry, isOriginInput)`
//   `PackagePart.getOutputStream*`（`:16x`）⇒ 基类恒 `null`（写侧由子类实现）
// ★本 crate 里 part **不再**自带构造期复制的字节（`05` 已删 `zip_payload`/`setZipPayload`）；取流一律
//   回到 `zipEntry` + zip 源（`PackagePart.zipArchive`）⇒ IO-04/IO-05 的「无源」分支因此可被真实构造。

use core_libs_io::io::xwpf::util::ZipEntrySource::{FakeZipEntry, ZipEntryRef};
use core_libs_io::io::opc::internal::ContentTypeManager::TYPES_NAMESPACE_URI;
use core_libs_io::io::opc::OPCPackage::OPCPackage;
use core_libs_io::io::opc::PackagePart::PackagePart;
use core_libs_io::io::opc::PackagePartName::PackagePartName;
use core_libs_io::io::opc::PackagingURIHelper::PackagingURIHelper;
use std::io::{Cursor, Read, Write};

const XML_CT: &str = "application/xml";
const MAIN_CT: &str =
    "application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml";
const RELS_CT: &str = "application/vnd.openxmlformats-package.relationships+xml";

const DOC_BYTES: &[u8] = b"<w:document><w:body>hello part io</w:body></w:document>";
const RELS_BYTES: &[u8] = br#"<?xml version="1.0"?><Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships"/>"#;
const PNG_BYTES: &[u8] = b"\x89PNG\r\n\x1a\n0123456789";

fn types_xml() -> String {
    format!(
        r#"<?xml version="1.0"?><Types xmlns="{TYPES_NAMESPACE_URI}"><Default Extension="xml" ContentType="{XML_CT}"/><Default Extension="rels" ContentType="{RELS_CT}"/><Default Extension="png" ContentType="image/png"/><Override PartName="/word/document.xml" ContentType="{MAIN_CT}"/></Types>"#
    )
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

/// 内存后端（`ZipInputStreamZipEntrySource`）。
fn memory_pkg() -> Box<OPCPackage> {
    let types = types_xml();
    let mut input = Cursor::new(zip_bytes(&[
        ("[Content_Types].xml", types.as_bytes()),
        ("word/document.xml", DOC_BYTES),
        ("word/_rels/document.xml.rels", RELS_BYTES),
        ("word/media/a.png", PNG_BYTES),
        ("word/empty.xml", b""),
    ]));
    let mut pkg = Box::new(OPCPackage::from_stream(&mut input));
    pkg.getParts();
    pkg
}

fn name(s: &str) -> PackagePartName {
    PackagingURIHelper::createPartName_str(s).unwrap()
}

fn read_all(part: &PackagePart, is_origin: bool) -> Result<Vec<u8>, String> {
    match part.getInputStream(is_origin) {
        Ok(mut stream) => {
            let mut buf = Vec::new();
            stream.read_to_end(&mut buf).map_err(|e| e.to_string())?;
            Ok(buf)
        }
        Err(e) => Err(e.to_string()),
    }
}

fn panic_message(err: Box<dyn std::any::Any + Send>) -> String {
    if let Some(s) = err.downcast_ref::<String>() {
        s.clone()
    } else if let Some(s) = err.downcast_ref::<&'static str>() {
        (*s).to_string()
    } else {
        "<非字符串 payload>".to_string()
    }
}

/// IO-01 / 主路径：主 part 取流 = zip 里那份字节（内存后端）。
#[test]
fn io01_main_part_bytes() {
    let mut pkg = memory_pkg();
    let part = pkg.getPart(&name("/word/document.xml")).expect("main part");
    assert_eq!(read_all(part, false).unwrap(), DOC_BYTES);
    // CT 来自 Override（证明 part 是经 CT 表建成的，而非旁路）
    assert_eq!(part.getContentType().toString(), MAIN_CT);
}

/// IO-02 / 主路径：`.rels` part 取流。
#[test]
fn io02_rels_part_bytes() {
    let mut pkg = memory_pkg();
    let part = pkg
        .getPart(&name("/word/_rels/document.xml.rels"))
        .expect("rels part");
    assert_eq!(read_all(part, false).unwrap(), RELS_BYTES);
    assert!(part.isRelationshipPart(), "`.rels` part 必须被标为关系 part");
}

/// IO-03 / 边界：0 字节 part ⇒ 空流（长度 0）。
#[test]
fn io03_zero_byte_part() {
    let mut pkg = memory_pkg();
    let part = pkg.getPart(&name("/word/empty.xml")).expect("empty part");
    let bytes = read_all(part, false).unwrap();
    assert!(bytes.is_empty(), "0 字节 part 必须给空流，实际 {} 字节", bytes.len());
}

/// IO-04 / 失败：**无源** part（既无 `zipEntry` 也无 zip 源）⇒ Java 的 IOException
/// `"Can't obtain the input stream from <partName>"`。
/// ★改动前该情形返回的是 part 自带的那份 payload（`zip_payload`），因此这条分支测不到；`05` 删掉 payload 后才有真实入口。
#[test]
fn io04_part_without_any_source_errors_like_java() {
    let part = PackagePart::new_with_string(None, name("/word/document.xml"), MAIN_CT);
    let err = read_all(&part, false).unwrap_err();
    assert_eq!(
        err,
        "Can't obtain the input stream from /word/document.xml",
        "★消息必须与 Java `getInputStream()` 的拼接结果一致"
    );
    // 两种取值都走同一失败路径（Java `getInputStream(boolean)` 同一 catch）
    assert!(read_all(&part, true).is_err());
}

/// IO-05 / 畸形（登记）：`zipEntry` 承载了**非** `FakeZipEntry` 对象这一分支，**测试侧无法构造** ——
/// `PackagePart.zipEntry`/`zipArchive` 是 `pub(crate)`（本 crate 的 part 由 `OPCPackage::part_from_entry` 独占写入），
/// 且没有任何 public setter。本用例因此做**源码级**断言：该分支所用判据仍在（`downcast_ref::<FakeZipEntry>`），
/// 并在 §1.7.1 ⑧ 登记「Java 无此分支 / Rust 分支不可测」。
#[test]
fn io05_non_fake_entry_branch_is_source_pinned_only() {
    let src = include_str!("../../../../../src/cn/wps/io/opc/PackagePart.rs");
    assert!(
        src.contains("downcast_ref::<FakeZipEntry>()"),
        "★`getInputStreamImpl` 必须仍按 Java 的 `ZipPackagePart` 路径从承载 entry 取流"
    );
    assert!(
        src.contains("Can't obtain the input stream from"),
        "★失败文案（Java `getInputStream()` 的拼接）必须留在实现里"
    );
}

/// IO-06 / 写侧（登记）：基类 `getOutputStream*` 恒 `None`（Java 基类恒 `null`；写侧由子类实现）。
#[test]
fn io06_output_stream_is_none_on_base() {
    let part = PackagePart::new_with_string(None, name("/word/document.xml"), MAIN_CT);
    assert!(part.getOutputStream().is_none(), "Java 基类 `getOutputStream(…)` 恒 null");
    // Java: `protected OutputStream getOutputStreamImpl(ZipEntry entry, ZipOutputStream.IListener listener, ZipOutputStream.DeflateDataCallBack cb) { return null; }`
    //       ⇒ 三参形态同样恒 `None`。
    // ★2026-09-20：第一个形参已是**真实类型** `&ZipEntryRef`（旧写法传 `&()` 走 `dyn Any` 壳）；
    //   另两个（listener / callback）仍是 `&dyn Any` —— kfs 侧尚无本仓统一边界类型，
    //   且真正的消费点按 `Option<Box<dyn …>>` 收所有权（见 `PackagePart.rs` 该方法的依据注释）。
    let entry = ZipEntryRef::Fake(FakeZipEntry::new("word/document.xml", b"x".to_vec()));
    assert!(part.getOutputStream_zip(&entry, &(), &()).is_none());
}

/// IO-07 / 边界：`setContentType` —— 未挂包的 part 可改；**已挂包**（`_container == Some`）的 part 不可改
/// （Java `PackagePart.setContentType` 的同名判据；消息与判据逐字对齐）。
#[test]
fn io07_set_content_type_guard() {
    // 未挂包 ⇒ 允许
    let mut detached = PackagePart::new_with_string(None, name("/word/document.xml"), XML_CT);
    detached.setContentType(MAIN_CT);
    assert_eq!(detached.getContentType().toString(), MAIN_CT);

    // 已挂包 ⇒ 拒绝。构造：`new_with_string(Some(pkg as *const _), …)`（与 `part_from_entry` 同一挂包约定）；
    // 包用 `Box` 保证地址稳定（`05` §1.7.1 ③ 的稳定性规则）。
    let mut pkg = memory_pkg();
    let scanned = pkg.getPart(&name("/word/document.xml")).expect("part");
    assert!(
        scanned.getPackage().is_some(),
        "经扫描建出的 part 必须挂包（Java `this._container = pack`）"
    );
    let mut attached = PackagePart::new_with_string(
        Some(pkg.as_ref() as *const OPCPackage),
        name("/word/document.xml"),
        XML_CT,
    );
    let err = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        attached.setContentType(MAIN_CT);
    }))
    .expect_err("已挂包的 part 不允许改 CT");
    assert_eq!(
        panic_message(err),
        "You can't change the content type of a part.",
        "★消息必须与 Java `PackagePart.setContentType` 的 InvalidOperationException 逐字一致"
    );
}

/// IO-08 / 幂等：同一 part 连续取流两次 ⇒ 字节相同（每次向源要一份新流，Java 同）。
#[test]
fn io08_stream_is_repeatable() {
    let mut pkg = memory_pkg();
    let part = pkg.getPart(&name("/word/document.xml")).expect("part");
    let first = read_all(part, false).unwrap();
    let second = read_all(part, false).unwrap();
    assert_eq!(first, second);
    assert_eq!(first, DOC_BYTES);
}

/// IO-09 / 全集：包内每个 part 取流 ⇒ 与写入 zip 时的字节逐一相等（=「独立算 sha256 表」的等价判据：
/// 期望值来自 zip **写入侧**的原始字节，与取流实现无共享代码）。
/// ⚠C1 真语料的 sha256 表需要 `tests/fixtures/`（本轮仓库内缺语料，见 §1.7.1 ⑤），故用自造包代替。
#[test]
fn io09_every_part_streams_its_own_bytes() {
    let mut pkg = memory_pkg();
    let expected: Vec<(&str, &[u8])> = vec![
        ("/word/document.xml", DOC_BYTES),
        ("/word/_rels/document.xml.rels", RELS_BYTES),
        ("/word/media/a.png", PNG_BYTES),
        ("/word/empty.xml", b""),
    ];
    for (part_name, bytes) in expected {
        let part = pkg
            .getPart(&name(part_name))
            .unwrap_or_else(|| panic!("part 缺失: {part_name}"));
        assert_eq!(read_all(part, false).unwrap(), bytes, "part = {part_name}");
    }
    assert_eq!(pkg.getPartList().map(|l| l.size()), Some(4));
}
