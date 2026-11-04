// function_id=34_function_rels_add
// status=loop — executable tests live beside src, not in features/
// cargo test --manifest-path libs/core-libs/io/wps-io/Cargo.toml --test 34_function_rels_add
// src:  libs/core-libs/io/wps-io/src/cn/wps/io/opc/PackageRelationshipCollection.rs
//       libs/core-libs/io/wps-io/src/cn/wps/io/opc/PackageRelationship.rs
// test: libs/core-libs/io/wps-io/tests/cn/wps/io/opc/PackageRelationshipCollection_rels_add.rs
//
// 覆盖 `addRelationship(URI, TargetMode, String, String)`（Java `:86-102`）、`addRelationship(PackageRelationship)`
// （Java `:82-84`）、`addReservedRelationshipId`（Java `:78-80`）。
// ★本轮（2026-09-18）把 `PackageRelationship.container/source` 从「值（`Box<dyn Any>`，恒 `None`）」
//   改成**引用语义裸指针**（Java 的字段本来就是引用）⇒ 新建的关系**真的带上** container / sourcePart，
//   `getPackage()` / `getSource()` / `getSourceURI()` 不再恒为空（A34-7/8）。

use core_libs_io::io::opc::PackageRelationship::PackageRelationship;
use core_libs_io::io::opc::PackageRelationshipCollection::PackageRelationshipCollection;
use core_libs_io::io::opc::PackagePart::PackagePart;
use core_libs_io::io::opc::TargetMode::TargetMode;
use core_libs_io::io::opc::OPCPackage::OPCPackage;
use core_libs_io::io::util::URI::URI;
use std::io::Write;

const IMAGE: &str = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/image";
const MAIN_CT: &str =
    "application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml";

fn uri(s: &str) -> URI {
    URI::new(s).unwrap()
}

fn coll() -> PackageRelationshipCollection {
    PackageRelationshipCollection::new()
}

fn add(c: &mut PackageRelationshipCollection, target: &str, id: Option<&str>) -> String {
    c.addRelationship(uri(target), TargetMode::INTERNAL, IMAGE, id)
        .getId()
        .to_string()
}

/// A34-1 / 主路径：显式且未被占用的 id **原样保留**。
#[test]
fn a34_1_explicit_free_id_is_kept() {
    let mut c = coll();
    assert_eq!(add(&mut c, "media/i.png", Some("rId5")), "rId5");
    assert_eq!(c.size(), 1);
    assert!(c.getRelationshipByID("rId5").is_some());
}

/// A34-2 / 边界：Java `if (id == null || get(id) != null)` ⇒ **占用即重新分配**，
/// 起点是 `relationshipsByID.size()`（Java `:88-96`）⇒ 3 条在位、请求 `rId1` 时得到 `rId4`。
#[test]
fn a34_2_occupied_id_is_reallocated_from_size() {
    let mut c = coll();
    assert_eq!(add(&mut c, "a.xml", Some("rId1")), "rId1");
    assert_eq!(add(&mut c, "b.xml", Some("rId2")), "rId2");
    assert_eq!(add(&mut c, "c.xml", Some("rId3")), "rId3");
    // 请求已被占用的 rId1 ⇒ i = size() = 3 ⇒ ++i = 4 ⇒ "rId4"
    assert_eq!(add(&mut c, "d.xml", Some("rId1")), "rId4");
    assert_eq!(c.size(), 4);
    // 原来的 rId1 未被覆盖
    assert_eq!(c.getRelationshipByID("rId1").unwrap().getTargetURI().toString(), "/a.xml");
}

/// A34-3 / 主路径：`id == null` ⇒ 空集合上得到 `rId1`（`i = 0` ⇒ `++i`）。
#[test]
fn a34_3_null_id_allocates_from_one() {
    let mut c = coll();
    assert_eq!(add(&mut c, "a.xml", None), "rId1");
    assert_eq!(add(&mut c, "b.xml", None), "rId2");
}

/// A34-4 / 边界：**保留 id** 被跳过（Java `do { … } while (… || reservedRelationIDs.contains(id))`）。
#[test]
fn a34_4_reserved_ids_are_skipped() {
    let mut c = coll();
    c.addReservedRelationshipId("rId1".to_string());
    c.addReservedRelationshipId("rId2".to_string());
    // 空集合：i = 0 ⇒ 候选 rId1（被保留）⇒ rId2（被保留）⇒ rId3
    assert_eq!(add(&mut c, "a.xml", None), "rId3");
}

/// A34-5 / 边界：显式 id 已被占用时**走重新分配**，不会静默覆盖（与 `addRelationship_rel` 不同）。
#[test]
fn a34_5_explicit_occupied_id_never_overwrites() {
    let mut c = coll();
    let first = add(&mut c, "a.xml", Some("rId1"));
    let second = add(&mut c, "b.xml", Some("rId1"));
    assert_eq!((first.as_str(), second.as_str()), ("rId1", "rId2"));
    assert_eq!(c.size(), 2);
}

/// A34-6 / 主路径：`addRelationship(PackageRelationship)` 是**按 id 覆盖**（Java `put` 语义）。
#[test]
fn a34_6_add_relationship_object_overwrites_by_id() {
    let mut c = coll();
    // Java 构造器要求 `pkg != null`（`PackageRelationship.java:25-27`，Rust 同为 `panic!("pkg")`）⇒ 给真实包引用。
    let pkg = pkg_with_doc();
    let rel = PackageRelationship::new(
        Some(pkg.as_ref() as *const OPCPackage),
        None,
        uri("media/one.png"),
        TargetMode::INTERNAL,
        IMAGE.to_string(),
        "rId1".to_string(),
    );
    c.addRelationship_rel(rel);
    assert_eq!(c.size(), 1);
    assert_eq!(c.getRelationshipByID("rId1").unwrap().getTargetURI().toString(), "/media/one.png");

    let rel2 = PackageRelationship::new(
        Some(pkg.as_ref() as *const OPCPackage),
        None,
        uri("media/two.png"),
        TargetMode::INTERNAL,
        IMAGE.to_string(),
        "rId1".to_string(),
    );
    c.addRelationship_rel(rel2);
    assert_eq!(c.size(), 1, "★同 id ⇒ 覆盖（Java `put`）");
    assert_eq!(c.getRelationshipByID("rId1").unwrap().getTargetURI().toString(), "/media/two.png");
}

fn pkg_with_doc() -> Box<OPCPackage> {
    let types = r#"<?xml version="1.0"?><Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types"><Default Extension="xml" ContentType="application/xml"/><Override PartName="/word/document.xml" ContentType="application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml"/></Types>"#;
    let cursor = std::io::Cursor::new(Vec::new());
    let mut writer = zip::ZipWriter::new(cursor);
    let options = zip::write::SimpleFileOptions::default();
    for (name, bytes) in [
        ("[Content_Types].xml", types.as_bytes()),
        ("word/document.xml", b"<w:document/>".as_slice()),
    ] {
        writer.start_file(name, options).unwrap();
        writer.write_all(bytes).unwrap();
    }
    let bytes = writer.finish().unwrap().into_inner();
    let mut pkg = Box::new(OPCPackage::from_stream(&mut std::io::Cursor::new(bytes)));
    pkg.getParts();
    pkg
}

/// A34-7 / ★本轮对齐的核心：带容器的集合新建的关系**带上 container 与 sourcePart 的引用** ——
/// `getPackage()` / `getSource()` / `getSourceURI()` 与 Java 一致（改前恒为 `None` / 包根）。
#[test]
fn a34_7_container_and_source_are_carried_by_new_relationships() {
    let pkg = pkg_with_doc();
    let source = PackagePart::new_with_string(None, pkg_part_name(), MAIN_CT);
    let mut c = PackageRelationshipCollection::new_with_container_and_part(
        Some(pkg.as_ref() as *const OPCPackage),
        // ★2026-09-20：`part` 形参由 `Option<Box<dyn Any>>` 改为真实类型别名 `Option<*const PackagePart>`
        //   （Java 存的是引用，不搬走对象）⇒ 传 source 的地址；`source` 是本测试的局部，存活到调用结束。
        Some(&source as *const PackagePart),
    );
    let id = add(&mut c, "word/styles.xml", Some("rId9"));
    let rel = c.getRelationshipByID(&id).expect("rId9");
    assert!(rel.getPackage().is_some(), "★container 必须随关系携带（Java 字段 = this.container）");
    assert_eq!(
        *rel.getPackage(),
        Some(pkg.as_ref() as *const OPCPackage),
        "★必须是**同一个**包引用"
    );
    assert!(rel.getSource().is_some(), "★sourcePart 必须随关系携带（Java 字段 = this.sourcePart）");
    assert_eq!(
        rel.getSourceURI().toString(),
        "/word/document.xml",
        "★Java: this.source._partName.getURI()"
    );
}

/// A34-8 / 包级（`sourcePart == null`）⇒ `getSourceURI()` = 包根（Java `PACKAGE_ROOT_URI`）。
#[test]
fn a34_8_package_level_source_uri_is_package_root() {
    let pkg = pkg_with_doc();
    let mut c = PackageRelationshipCollection::new_with_container_and_part(
        Some(pkg.as_ref() as *const OPCPackage),
        None,
    );
    let id = add(&mut c, "word/document.xml", Some("rId1"));
    let rel = c.getRelationshipByID(&id).expect("rId1");
    assert!(rel.getPackage().is_some());
    assert!(rel.getSource().is_none());
    assert_eq!(rel.getSourceURI().toString(), "/");
}

/// A34-9 / **登记性**用例（现状断言 + Java 侧 latent throw 证据）：
/// Java 的 `PackagePart.addRelationship*` 先 `new PackageRelationshipCollection()`（Java `:62/:104/:127`），
/// 其 `container == null` ⇒ 集合的 `addRelationship` 会 `new PackageRelationship(null, …)` ⇒
/// `PackageRelationship.java:25-30` 抛 `IllegalArgumentException("pkg")`（**Java 侧该路径必然抛**）。
/// Rust 侧集合同样允许无容器，`addRelationship` 直接构造（不走受校验的构造器）⇒ **成功**。
/// ★这是登记的有意偏差：Rust 的 `PackagePart::loadRelationships` 占位路径依赖它（见 §1.7.1 ①）。
#[test]
fn a34_9_container_less_collection_adds_without_java_latent_throw() {
    let mut c = coll();
    assert_eq!(add(&mut c, "media/i.png", None), "rId1", "★Rust 成功（Java 该路径抛 IllegalArgumentException(\"pkg\")）");
    assert!(c.getRelationshipByID("rId1").unwrap().getPackage().is_none());
}

fn pkg_part_name() -> core_libs_io::io::opc::PackagePartName::PackagePartName {
    core_libs_io::io::opc::PackagingURIHelper::PackagingURIHelper::createPartName_str("/word/document.xml")
        .unwrap()
}
