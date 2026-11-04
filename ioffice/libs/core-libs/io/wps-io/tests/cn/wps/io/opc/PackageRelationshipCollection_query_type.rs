// function_id=38_function_rels_query_type
// status=loop — executable tests live beside src, not in features/
// cargo test --manifest-path libs/core-libs/io/wps-io/Cargo.toml --test 38_function_rels_query_type
// src:  libs/core-libs/io/wps-io/src/cn/wps/io/opc/PackageRelationshipCollection.rs
//       libs/core-libs/io/wps-io/src/cn/wps/io/opc/OPCPackage.rs
// test: libs/core-libs/io/wps-io/tests/cn/wps/io/opc/PackageRelationshipCollection_query_type.rs
//
// 覆盖查询/迭代面：`getRelationshipByID`（Java `:138`）、`getRelationships(String typeFilter)`（Java `:112-118`，
// 经拷贝构造器 `PackageRelationshipCollection(coll, filter)`）、`iterator(String typeFilter)`（Java `:210-222`）
// 以及包级 `getRelationships()` / `getRelationshipsByType(String)`（Java `OPCPackage.java:374-388`）。
//
// ★与 34/37 的引用语义改动联动：过滤出的**子集合**元素现在照抄 `container`/`source` 引用 ⇒ 子集合里的
//   关系仍能正确解析 target（FW2），且与父集合解耦（Q38-10）。

use core_libs_io::io::opc::OPCPackage::OPCPackage;
use core_libs_io::io::opc::PackageAccess::PackageAccess;
use core_libs_io::io::opc::PackageRelationshipCollection::PackageRelationshipCollection;
use core_libs_io::io::opc::TargetMode::TargetMode;
use core_libs_io::io::util::URI::URI;
use std::io::Write;

const TYPES_NS: &str = "http://schemas.openxmlformats.org/package/2006/content-types";
const RELS_NS: &str = "http://schemas.openxmlformats.org/package/2006/relationships";
const OFFICE_DOC: &str =
    "http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument";
const IMAGE: &str = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/image";

fn uri(s: &str) -> URI {
    URI::new(s).unwrap()
}

fn coll_with(entries: &[(&str, &str, &str)]) -> PackageRelationshipCollection {
    let mut c = PackageRelationshipCollection::new();
    for (id, rel_type, target) in entries {
        c.addRelationship(uri(target), TargetMode::INTERNAL, rel_type, Some(id));
    }
    c
}

fn sorted_ids(c: &PackageRelationshipCollection) -> Vec<String> {
    let mut v: Vec<String> = c.iter().map(|r| r.getId().to_string()).collect();
    v.sort();
    v
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

/// Q38-1 / 主路径：按 id 命中。
#[test]
fn q38_1_lookup_by_id_hits() {
    let c = coll_with(&[("rId1", OFFICE_DOC, "word/document.xml"), ("rId2", IMAGE, "media/i.png")]);
    let rel = c.getRelationshipByID("rId1").expect("rId1");
    assert_eq!(rel.getRelationshipType(), OFFICE_DOC);
    assert_eq!(rel.getTargetURI().toString(), "/word/document.xml");
}

/// Q38-2 / 边界：按 id 未命中 ⇒ `None`（Java 返回 `null`），不报错。
#[test]
fn q38_2_lookup_by_id_miss_is_none() {
    let c = coll_with(&[("rId1", OFFICE_DOC, "word/document.xml")]);
    assert!(c.getRelationshipByID("rIdX").is_none());
    assert!(c.getRelationshipByID("").is_none());
}

/// Q38-3 / 主路径：按 type 过滤（命中 1 条 / 未知 type 0 条），子集合为**新集合**。
#[test]
fn q38_3_type_filter_hits_and_misses() {
    let c = coll_with(&[
        ("rId1", OFFICE_DOC, "word/document.xml"),
        ("rId2", IMAGE, "media/i1.png"),
        ("rId3", IMAGE, "media/i2.png"),
    ]);
    let docs = c.getRelationships(Some(OFFICE_DOC));
    assert_eq!(docs.size(), 1);
    assert_eq!(sorted_ids(&docs), vec!["rId1"]);
    let images = c.getRelationships(Some(IMAGE));
    assert_eq!(sorted_ids(&images), vec!["rId2", "rId3"]);
    assert_eq!(c.getRelationships(Some("urn:unknown")).size(), 0);
}

/// Q38-4 / 边界：`filter == None` ⇒ 全部（Java: 拷贝构造器逐条拷入）；`filter == Some("")` ⇒ 0 条
/// （空串不等于任何 type）。
#[test]
fn q38_4_none_filter_is_all_and_empty_filter_is_none() {
    let c = coll_with(&[("rId1", OFFICE_DOC, "a.xml"), ("rId2", IMAGE, "b.png")]);
    assert_eq!(sorted_ids(&c.getRelationships(None)), vec!["rId1", "rId2"]);
    assert_eq!(c.getRelationships(Some("")).size(), 0);
}

/// Q38-5 / 登记性（类型级差异）：Java `getRelationshipsByType(null)` 抛
/// `IllegalArgumentException("relationshipType")`（`OPCPackage.java:378-384`）；
/// Rust 形参是 `&str` ⇒ `null` **不可表达**，因此该分支在 Rust 侧不存在（§1.7.1 ②）。
#[test]
fn q38_5_null_type_filter_is_unrepresentable() {
    let mut p = pkg_with_package_rels();
    // 合法入参路径：按 type 查（Java 非 null 分支）
    assert_eq!(p.getRelationshipsByType(OFFICE_DOC).size(), 1);
    // 空串（≠ null）：Java 会走 `getRelationshipsHelper("")` ⇒ 0 条
    assert_eq!(p.getRelationshipsByType("").size(), 0);
}

/// Q38-6 / 登记性：本层对「未知 id」的**唯一**信号是 `None`（Java `null`）；
/// 文档里提到的 `relationship_not_found` 是**消费方**（docprops）的错误分类，不在本 function 的 src 面（§1.7.1 ③）。
#[test]
fn q38_6_unknown_id_signal_is_none_only() {
    let c = coll_with(&[("rId1", OFFICE_DOC, "a.xml")]);
    assert!(c.getRelationshipByID("rId404").is_none());
    assert_eq!(c.size(), 1, "查询不得改变集合");
}

/// Q38-7 / ★**显式分叉用例**：Java `OPCPackage.isRelationshipExists(rel)` 用**引用同一性**
/// （`for (PackageRelationship r : getRelationships()) if (r == rel) return true;`，`OPCPackage.java:411-418`）；
/// Rust 该方法是 `todo!("DEFERRED_NON_FUNCTION")`（未改写）⇒ 本用例**钉住停车状态**，避免误以为已实现。
#[test]
fn q38_7_is_relationship_exists_is_still_deferred() {
    let mut p = pkg_with_package_rels();
    // 自造一个「等值但不同一」的关系（Java 用 `r == rel` 判同一性 ⇒ 该对象在 Java 侧会返回 false）
    let rel = core_libs_io::io::opc::PackageRelationship::PackageRelationship::new(
        Some(p.as_ref() as *const OPCPackage),
        None,
        uri("word/document.xml"),
        TargetMode::INTERNAL,
        OFFICE_DOC.to_string(),
        "rId1".to_string(),
    );
    let err = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| p.isRelationshipExists(&rel)))
        .unwrap_err();
    let msg = panic_message(err);
    assert!(
        msg.contains("DEFERRED_NON_FUNCTION") || msg.contains("not yet implemented"),
        "★Java 语义 = 引用同一性；Rust 现状 = 停车（实测 payload = {msg:?}）"
    );
}

/// Q38-8 / 主路径（FW2）：过滤出的子集合元素**仍能正确解析 target**，并照抄引用（container/source）。
#[test]
fn q38_8_filtered_entries_keep_resolvable_targets() {
    let mut p = pkg_with_package_rels();
    let by_type = p.getRelationshipsByType(OFFICE_DOC);
    let rel = by_type.getRelationship(0).expect("officeDocument");
    assert_eq!(rel.getTargetURI().toString(), "/word/document.xml");
    assert_eq!(
        *rel.getPackage(),
        Some(p.as_ref() as *const OPCPackage),
        "★子集合元素照抄包引用（引用语义）"
    );
}

/// Q38-9 / 确定性：同一集合两次过滤，排序后的 id 列表相同（顺序本身不作契约，理由同 `35` §1.7.1 ③）。
#[test]
fn q38_9_filter_is_deterministic() {
    let c = coll_with(&[
        ("rId1", IMAGE, "media/i1.png"),
        ("rId2", IMAGE, "media/i2.png"),
        ("rId3", OFFICE_DOC, "word/document.xml"),
    ]);
    let first = sorted_ids(&c.getRelationships(Some(IMAGE)));
    let second = sorted_ids(&c.getRelationships(Some(IMAGE)));
    assert_eq!(first, second);
    assert_eq!(first, vec!["rId1", "rId2"]);
}

/// Q38-10 / 解耦：子集合是**新拷贝**（Java 拷贝构造器）⇒ `clear()` 子集合不影响父集合。
#[test]
fn q38_10_sub_collection_is_decoupled_from_parent() {
    let c = coll_with(&[("rId1", IMAGE, "a.png"), ("rId2", IMAGE, "b.png")]);
    let mut sub = c.getRelationships(Some(IMAGE));
    assert_eq!(sub.size(), 2);
    sub.clear();
    assert_eq!(sub.size(), 0);
    assert_eq!(c.size(), 2, "★父集合不受影响（Java 拷贝构造器语义）");
    assert!(c.getRelationshipByID("rId1").is_some());
}

fn pkg_with_package_rels() -> Box<OPCPackage> {
    let types = format!(
        r#"<?xml version="1.0"?><Types xmlns="{TYPES_NS}"><Default Extension="xml" ContentType="application/xml"/><Default Extension="rels" ContentType="application/vnd.openxmlformats-package.relationships+xml"/></Types>"#
    );
    let rels = format!(
        r#"<?xml version="1.0"?><Relationships xmlns="{RELS_NS}"><Relationship Id="rId1" Type="{OFFICE_DOC}" Target="word/document.xml"/><Relationship Id="rId2" Type="{IMAGE}" Target="media/i1.png"/></Relationships>"#
    );
    let cursor = std::io::Cursor::new(Vec::new());
    let mut writer = zip::ZipWriter::new(cursor);
    let options = zip::write::SimpleFileOptions::default();
    for (name, bytes) in [
        ("[Content_Types].xml", types.as_bytes()),
        ("_rels/.rels", rels.as_bytes()),
        ("word/document.xml", b"<w:document/>".as_slice()),
        ("media/i1.png", b"PNG".as_slice()),
    ] {
        writer.start_file(name, options).unwrap();
        writer.write_all(bytes).unwrap();
    }
    let bytes = writer.finish().unwrap().into_inner();
    let mut p = Box::new(OPCPackage::from_stream(&mut std::io::Cursor::new(bytes)));
    p.getParts();
    p
}
