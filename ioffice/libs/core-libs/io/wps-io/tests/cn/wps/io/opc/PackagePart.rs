// function_id=05_function_opc_parts
// status=loop — executable tests live beside src, not in features/
// cargo test --manifest-path libs/core-libs/io/wps-io/Cargo.toml --test 05_function_opc_parts
// src:  libs/core-libs/io/wps-io/src/cn/wps/io/opc/PackagePart.rs
// test: libs/core-libs/io/wps-io/tests/cn/wps/io/opc/PackagePart.rs

use core_libs_io::io::opc::internal::ContentTypeManager::ContentTypeManager;
use core_libs_io::io::opc::OPCPackage::{OPCPackage, MAX_SIZE_ENTRY};
use core_libs_io::io::opc::PackageAccess::PackageAccess;
use core_libs_io::io::opc::PackagePart::PackagePart;
use core_libs_io::io::opc::PackagePartCollection::PackagePartCollection;
use core_libs_io::io::opc::PackagingURIHelper::PackagingURIHelper;
use core_libs_io::io::opc::ZipPackagePart::ZipPackagePart;
use core_libs_io::io::xwpf::util::ZipEntrySource::{FakeZipEntry, ZipEntryRef};
use core_libs_io::io::xwpf::util::ZipInputStreamZipEntrySource::ZipInputStreamZipEntrySource;
use std::io::{Cursor, Read, Write};

const MAIN_CT: &str =
    "application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml";
const RELS_CT: &str = "application/vnd.openxmlformats-package.relationships+xml";
const XML_CT: &str = "application/xml";

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

fn stream_source(entries: &[(&str, &[u8])]) -> ZipInputStreamZipEntrySource {
    let mut input = Cursor::new(zip_bytes(entries));
    ZipInputStreamZipEntrySource::try_new(&mut input).unwrap()
}

fn part_name(name: &str) -> core_libs_io::io::opc::PackagePartName::PackagePartName {
    PackagingURIHelper::createPartName_str(name).expect(name)
}

/// ★返回 `Box<OPCPackage>`（不是裸 `OPCPackage`）：`getPartsImpl` 造出的 part 会持有
/// **指向所属包的共享引用指针**（Java `PackagePart._container`／`ZipPackagePart.getPackage()`；
/// 见 §1.7.1 ⑤）。Java 的引用指向堆对象、搬到哪都不变；Rust 的值一旦被 `move`，栈地址就变了
/// ⇒ 若 helper 直接返回 `OPCPackage`（按值 move），已造出的 part 里的指针立刻悬空。
/// `Box` 保证**被指对象**地址稳定（移动的只是 Box 句柄）⇒ 测试与真实用法都必须这样持有。
fn pkg_with_entries(
    entries: &[(&str, &[u8])],
    manager: Option<ContentTypeManager>,
) -> Box<OPCPackage> {
    let mut pkg = Box::new(OPCPackage::new(PackageAccess::READ));
    pkg.setZipArchive(Box::new(stream_source(entries)));
    if let Some(mgr) = manager {
        pkg.setContentTypeManager(mgr);
    }
    pkg.getPartsImpl();
    pkg
}

fn document_manager() -> ContentTypeManager {
    let mut mgr = ContentTypeManager::new(None, None);
    mgr.addOverrideContentType(&part_name("/word/document.xml"), MAIN_CT);
    mgr.addDefaultContentType("rels", RELS_CT);
    mgr.addDefaultContentType("xml", XML_CT);
    mgr
}

/// P1 / C01: Override 主文档入表且 CT 匹配。
#[test]
fn p1_document_part_registered() {
    let pkg = pkg_with_entries(&[("word/document.xml", b"<w:document/>")], Some(document_manager()));
    let part = pkg
        .getPartList()
        .unwrap()
        .get(&part_name("/word/document.xml"))
        .expect("document part");
    assert!(part.getContentType().toString().contains("wordprocessingml.document.main"));
    assert_eq!(pkg.getPartList().unwrap().size(), 1);
}

/// P2 / C02: 未知扩展、无 Override、guess null → 不入表。
#[test]
fn p2_unknown_extension_without_override() {
    let pkg = pkg_with_entries(&[("foo.bin", b"xxxx")], None);
    assert_eq!(pkg.getPartList().unwrap().size(), 0);
}

/// P3 / C03: 仅 Content_Types 不进入 partList。
#[test]
fn p3_content_types_not_a_package_part() {
    let pkg = pkg_with_entries(&[("[Content_Types].xml", b"<Types/>")], None);
    assert_eq!(pkg.getPartList().unwrap().size(), 0);
}

/// P4 / C04: Content Types 文件名 19 字节、大小写不敏感。
#[test]
fn p4_is_part_name_content_types() {
    assert!(OPCPackage::isPartNameContentTypes("[Content_Types].xml"));
    assert!(OPCPackage::isPartNameContentTypes("[CONTENT_TYPES].XML"));
    assert!(!OPCPackage::isPartNameContentTypes("word/document.xml"));
    assert!(!OPCPackage::isPartNameContentTypes("[Content_Types].xml "));
}

/// P5 / C05: `../` panic。
#[test]
#[should_panic(expected = "FileDamagedException")]
fn p5_rejects_path_traversal() {
    OPCPackage::checkEntryNameLegal("../evil.xml");
}

/// P6 / C06: 1GB 门闩是严格大于。
#[test]
fn p6_max_size_entry_gate() {
    assert_eq!(MAX_SIZE_ENTRY, 1073741824);
    assert!(!OPCPackage::isEntryTooLarge(MAX_SIZE_ENTRY as i64));
    assert!(OPCPackage::isEntryTooLarge(MAX_SIZE_ENTRY as i64 + 1));
}

/// P7 / C07: 无 manager 时 png 靠 guess 入表。
#[test]
fn p7_guess_png_content_type() {
    let pkg = pkg_with_entries(&[("word/media/a.png", b"\x89PNG")], None);
    let part = pkg
        .getPartList()
        .unwrap()
        .get(&part_name("/word/media/a.png"))
        .expect("png part");
    assert_eq!(part.getContentType().toString(), "image/png");
}

/// P8 / C08: collection put/get/remove。
#[test]
fn p8_collection_put_get_remove() {
    let mut coll = PackagePartCollection::new();
    let name = part_name("/word/document.xml");
    let part = PackagePart::new_with_string(None, name.clone(), MAIN_CT);
    coll.put(name.clone(), part);
    assert_eq!(coll.size(), 1);
    assert!(coll.get(&name).unwrap().getContentType().toString().contains("wordprocessingml"));
    assert!(coll.remove(&name).is_some());
    assert_eq!(coll.size(), 0);
    assert!(coll.get(&name).is_none());
}

/// P9 / C09: ZipPackagePart 持 FakeZipEntry 可再解压。
/// ★2026-09-18：`ZipPackagePart.getInputStreamImpl` 已与 Java 对齐为**单行委托**
/// （`this.getPackage().getZipArchive().getInputStream(this.zipEntry, isOriginInput)`）⇒
/// part 必须真的挂在包上（Java 侧同样如此：容器为 null 会 NPE），故本用例改为经 `OPCPackage` 取流。
#[test]
fn p9_zip_package_part_stream() {
    let fake = FakeZipEntry::new("word/document.xml", b"abc".to_vec());
    let mut pkg = OPCPackage::new(PackageAccess::READ);
    pkg.setZipArchive(Box::new(ZipInputStreamZipEntrySource::from_entries(vec![
        fake.clone(),
    ])));
    let part = ZipPackagePart::new_with_entry(
        Some(&pkg as *const OPCPackage),
        // ★2026-09-20：`ZipPackagePart.zipEntry` 已由 `Box<dyn Any>` 改为真实类型 `ZipEntryRef`
        Some(ZipEntryRef::Fake(fake)),
        part_name("/word/document.xml"),
        XML_CT,
    );
    let mut bytes = Vec::new();
    part.getInputStreamImpl(false).read_to_end(&mut bytes).unwrap();
    assert_eq!(bytes, b"abc");
}

/// P10 / C10: rels 与 document 都入表。
#[test]
fn p10_rels_and_document_both_registered() {
    let pkg = pkg_with_entries(
        &[
            ("_rels/.rels", b"<Relationships/>"),
            ("word/document.xml", b"<w:document/>"),
        ],
        Some(document_manager()),
    );
    let list = pkg.getPartList().unwrap();
    assert!(list.get(&part_name("/_rels/.rels")).is_some());
    assert!(list.get(&part_name("/word/document.xml")).is_some());
    assert_eq!(list.size(), 2);
}

/// P11 / C11: buildPartName 对 CT 文件返回 None。
#[test]
fn p11_build_part_name_skips_content_types() {
    assert!(OPCPackage::buildPartName("[Content_Types].xml").is_none());
    let name = OPCPackage::buildPartName("word/document.xml").unwrap();
    assert!(name.getName().contains("word/document.xml"));
}

/// P12 / C12: 无 archive / 空 zip → 空表。
#[test]
fn p12_empty_archive_is_empty_part_list() {
    let mut pkg = OPCPackage::new(PackageAccess::READ);
    pkg.getPartsImpl();
    assert_eq!(pkg.getPartList().unwrap().size(), 0);

    let pkg = pkg_with_entries(&[], None);
    assert_eq!(pkg.getPartList().unwrap().size(), 0);
}


/// 造包后**按值返回**（包被 `move`）——刻意模拟 `XWPFPackage::open_stream` 的做法：
/// Java 的 part 持有的是**堆对象引用**，包怎么搬都有效；Rust 值语义下若 part 在取流时解引用
/// `_container`，这里就会拿到悬空指针（★实测曾 SIGBUS）。part 因此改为记录 zip 源的**堆地址**
/// （见 `PackagePart.zipArchive` 字段注释）⇒ 本用例是该设计的回归护栏。
fn pkg_with_entries_moved(entries: &[(&str, &[u8])]) -> OPCPackage {
    let mut pkg = OPCPackage::new(PackageAccess::READ);
    pkg.setZipArchive(Box::new(stream_source(entries)));
    pkg.getPartsImpl();
    pkg
}

/// P11 / ★Java 结构：`getPartsImpl` 造出的 part 持 **`zipEntry` + zip 源**（Java `ZipPackagePart` 的字段），
/// 取流时才向源要字节；不再是「把解码字节存进 part 的 `zip_payload`」那种 Rust-only 形态。
/// 因此（a）包被 move 后仍能取流（Java 引用语义），（b）`_container` 已按 Java 设为所属包
/// （`getPackage()` 非空），（c）可重复取流。
/// ⚠️ 仍未对齐处（已登记 §1.7.1 ⑥）：Rust 值语义下 part 里存的是 entry 的**克隆**，
///    而 Java 共享同一 `ZipEntry` 对象引用 ⇒ memory 源的 `data` 会多一份副本。
#[test]
fn p11_parts_are_entry_backed_and_survive_a_package_move() {
    let ct = br#"<?xml version="1.0"?><Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types"><Default Extension="xml" ContentType="application/xml"/></Types>"#;
    let mut moved = pkg_with_entries_moved(&[
        ("[Content_Types].xml", ct),
        ("word/document.xml", b"<w:document>HELLO</w:document>"),
    ]);
    let part = moved.getPart(&part_name("/word/document.xml")).expect("part");
    // (a) 包已 move ⇒ 仍按 Java 路径 `getPackage().getZipArchive().getInputStream(zipEntry, …)` 取到字节
    let mut first = String::new();
    part.getInputStream_default()
        .expect("moved package 的 part 仍可取流")
        .read_to_string(&mut first)
        .unwrap();
    assert_eq!(first, "<w:document>HELLO</w:document>");
    // (c) 可重复取流（每次新开一个读句柄，Java 同）
    let mut second = String::new();
    part.getInputStream_default().unwrap().read_to_string(&mut second).unwrap();
    assert_eq!(second, first);
    // (b) Java `PackagePart(OPCPackage pack, …)` ⇒ part 知道所属包
    assert!(part.getPackage().is_some(), "part 的 `_container` 应按 Java 设为所属包");
}

/// P12 / ★Java 语义：`getInputStream(isOriginInput)` 的 `isOriginInput` 必须**透传到 zip 源**。
/// 文件后端下 `true` ⇒ 不解压（raw），`false` ⇒ 解码后字节（kfs `ZipFile.getInputStream(entry, flag)`）。
/// ★改动前 part 自带一份构造期复制的**解码字节** ⇒ 两种取值都返回解码字节（Java 分叉被吞掉）。
#[test]
fn p12_is_origin_input_reaches_the_zip_source_like_java() {
    let payload: &[u8] = b"<w:document>COMPRESSIBLE_COMPRESSIBLE_COMPRESSIBLE_COMPRESSIBLE</w:document>";
    let path = std::env::temp_dir().join(format!(
        "ioffice-05-entry-backed-{}.zip",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    std::fs::write(&path, zip_bytes(&[("word/document.xml", payload)])).unwrap();
    let source = core_libs_io::io::xwpf::util::ZipFileZipEntrySource::ZipFileZipEntrySource::from_path(
        path.to_str().unwrap(),
    )
    .unwrap();
    let mut pkg = Box::new(OPCPackage::new(PackageAccess::READ));
    pkg.setZipArchive(Box::new(source));
    let mut manager = ContentTypeManager::new(None, None);
    manager.addDefaultContentType("xml", XML_CT);
    pkg.setContentTypeManager(manager);
    pkg.getPartsImpl();

    let part = pkg.getPart(&part_name("/word/document.xml")).expect("file part");
    let mut decoded = Vec::new();
    part.getInputStream(false).unwrap().read_to_end(&mut decoded).unwrap();
    let mut raw = Vec::new();
    part.getInputStream(true).unwrap().read_to_end(&mut raw).unwrap();
    assert_eq!(decoded, payload, "`isOriginInput = false` ⇒ 解码后的原字节");
    assert_ne!(
        raw, decoded,
        "★`isOriginInput = true` ⇒ 不解压（raw）；相等说明标志被吞掉（改动前的行为 = 缺陷）"
    );
    let _ = std::fs::remove_file(&path);
}
