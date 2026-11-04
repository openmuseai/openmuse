// function_id=37_function_rels_pkg_level
// status=loop — executable tests live beside src, not in features/
// cargo test --manifest-path libs/core-libs/io/wps-io/Cargo.toml --test 37_function_rels_pkg_level
// src:  libs/core-libs/io/wps-io/src/cn/wps/io/opc/OPCPackage.rs
//       libs/core-libs/io/wps-io/src/cn/wps/io/opc/PackageRelationshipCollection.rs
// test: libs/core-libs/io/wps-io/tests/cn/wps/io/opc/OPCPackage_rels_pkg_level.rs
//
// 覆盖 `OPCPackage.ensureRelationships()`（Java `OPCPackage.java`：`try { this.relationships =
// new PackageRelationshipCollection(this); } catch (InvalidFormatException e) { …空集合 }`）。
// ★本轮（2026-09-18）把 Rust 的「硬编 `/_rels/.rels` 字面量 + 只 guard 取流」重写为 Java 形态：
//   ① 关系 part 名走 `PackageRelationshipCollection::getRelationshipPartName(null)`（不再硬编）；
//   ② **复刻 try/catch 降级**：解析抛错（Java `InvalidFormatException`，如 M4.1）⇒ 空集合、不 panic；
//   ③ 集合的 `container` 接线到本包（后续新建的关系带上包引用）。

use core_libs_io::io::opc::OPCPackage::OPCPackage;
use core_libs_io::io::opc::PackageAccess::PackageAccess;
use core_libs_io::io::opc::PackagePartName::PackagePartName;
use core_libs_io::io::opc::PackagingURIHelper::PackagingURIHelper;
use core_libs_io::io::opc::TargetMode::TargetMode;
use std::io::Write;

const TYPES_NS: &str = "http://schemas.openxmlformats.org/package/2006/content-types";
const RELS_NS: &str = "http://schemas.openxmlformats.org/package/2006/relationships";
const OFFICE_DOC: &str =
    "http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument";
const CORE_PROPS: &str =
    "http://schemas.openxmlformats.org/package/2006/relationships/metadata/core-properties";
const CORE_PROPS_ALT: &str =
    "http://schemas.openxmlformats.org/officeDocument/2006/relationships/metadata/core-properties";
const IMAGE: &str = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/image";

fn types_xml() -> String {
    format!(
        r#"<?xml version="1.0"?><Types xmlns="{TYPES_NS}"><Default Extension="xml" ContentType="application/xml"/><Default Extension="rels" ContentType="application/vnd.openxmlformats-package.relationships+xml"/></Types>"#
    )
}

fn zip_bytes(entries: &[(&str, &[u8])]) -> Vec<u8> {
    let cursor = std::io::Cursor::new(Vec::new());
    let mut writer = zip::ZipWriter::new(cursor);
    let options = zip::write::SimpleFileOptions::default();
    for (name, bytes) in entries {
        writer.start_file(*name, options).unwrap();
        writer.write_all(bytes).unwrap();
    }
    writer.finish().unwrap().into_inner()
}

/// 造包；`pkg_rels` 为 `None` 时**不含** `/_rels/.rels`。`build_parts=false` 时故意不建 partList（验懒建）。
fn pkg(pkg_rels: Option<&str>, build_parts: bool) -> Box<OPCPackage> {
    let types = types_xml();
    let mut entries: Vec<(&str, &[u8])> = vec![
        ("[Content_Types].xml", types.as_bytes()),
        ("word/document.xml", b"<w:document/>"),
    ];
    if let Some(rels) = pkg_rels {
        entries.push(("_rels/.rels", rels.as_bytes()));
    }
    let bytes = zip_bytes(&entries);
    let mut p = Box::new(OPCPackage::from_stream(&mut std::io::Cursor::new(bytes)));
    if build_parts {
        p.getParts();
    }
    p
}

fn office_doc_rels() -> String {
    format!(
        r#"<?xml version="1.0"?><Relationships xmlns="{RELS_NS}"><Relationship Id="rId1" Type="{OFFICE_DOC}" Target="word/document.xml"/></Relationships>"#
    )
}

/// K37-1 / 主路径：包级 `/_rels/.rels` 懒建 + 真解析；并验证集合/关系**带上包引用**（引用语义）。
#[test]
fn k37_1_package_level_relationships_are_lazily_built_and_parsed() {
    let mut p = pkg(Some(&office_doc_rels()), true);
    // ★`hasRelationships()` 在 Rust 侧会先懒建（Java 是纯 getter，要求先 ensure）⇒ 不做「建前为 false」断言。
    p.ensureRelationships();
    assert!(p.hasRelationships());
    let by_type = p.getRelationshipsByType(OFFICE_DOC);
    assert_eq!(by_type.size(), 1, "应解析出 1 条 officeDocument 关系");
    let rel = by_type.getRelationship(0).expect("officeDocument 关系");
    assert_eq!(rel.getId(), "rId1");
    assert_eq!(rel.getTargetURI().toString(), "/word/document.xml");
    // ★引用语义：解析出的关系带的是**本包**的指针（Java `this.container`）
    assert_eq!(*rel.getPackage(), Some(p.as_ref() as *const OPCPackage));
}

/// K37-2 / 幂等：二次调用不重建（Java `if (this.relationships == null)`）——集合与内容稳定。
#[test]
fn k37_2_ensure_relationships_is_idempotent() {
    let mut p = pkg(Some(&office_doc_rels()), true);
    p.ensureRelationships();
    let first = p.getRelationships().size();
    p.ensureRelationships();
    let second = p.getRelationships().size();
    assert_eq!((first, second), (1, 1));
    assert!(p.hasRelationships());
}

/// K37-3 / ★**降级路径**（Java `catch (InvalidFormatException)`）：`/_rels/.rels` 里出现**两条
/// core-properties**（M4.1）⇒ Java 抛 `InvalidFormatException` 并被 `catch` ⇒ **空集合、包照常可用**。
/// Rust 改前：`parseRelationshipsXml` 的 `panic!("OPC Compliance error [M4.1]…")` 会**穿透** `ensureRelationships`。
#[test]
fn k37_3_malformed_package_rels_degrades_to_empty_collection() {
    let bad = format!(
        // ★必须用**同一个** core-properties type 两次（判据是字符串相等；`.../officeDocument/...` 变体不算 M4.1）
        r#"<?xml version="1.0"?><Relationships xmlns="{RELS_NS}"><Relationship Id="rId1" Type="{CORE_PROPS}" Target="docProps/core.xml"/><Relationship Id="rId2" Type="{CORE_PROPS}" Target="docProps/core2.xml"/></Relationships>"#
    );
    let mut p = pkg(Some(&bad), true);
    p.ensureRelationships(); // ★不得 panic
    assert!(!p.hasRelationships(), "降级 ⇒ 空集合（Java: catch 后 new PackageRelationshipCollection()）");
    assert_eq!(p.getRelationships().size(), 0);
    // 包其余功能仍可用（Java 语义：关系解析失败不影响包本身）
    assert!(p.getPart(&part_name("/word/document.xml")).is_some());
}

/// K37-4 / 边界：包里**没有** `/_rels/.rels` ⇒ 空集合、不报错（Java：`container.getPart(...)` 返回 null ⇒ 跳过解析）。
#[test]
fn k37_4_missing_package_rels_yields_empty_collection() {
    let mut p = pkg(None, true);
    p.ensureRelationships();
    assert!(!p.hasRelationships());
    assert_eq!(p.getRelationships().size(), 0);
    assert_eq!(p.getRelationshipsByType(OFFICE_DOC).size(), 0);
}

/// K37-5 / 懒建：调用前**故意不建** `partList` ⇒ `ensureRelationships` 内部触发 `getPartsImpl()`
/// （Java `container.getPart(...)` 内部同样触发 `getParts()`）后仍能解析出来。
#[test]
fn k37_5_part_list_is_built_lazily_by_ensure_relationships() {
    let mut p = pkg(Some(&office_doc_rels()), false);
    p.ensureRelationships();
    assert_eq!(p.getRelationshipsByType(OFFICE_DOC).size(), 1, "懒建 partList 后仍应解析出关系");
}

/// K37-6 / 集成：`OPCPackage::addRelationship` 在包级新建的关系**带上包引用**（Java：该集合由
/// `new PackageRelationshipCollection(this)` 建出 ⇒ `this.container` 非空）。
#[test]
fn k37_6_package_level_add_relationship_carries_the_package() {
    let mut p = pkg(None, true);
    let target = part_name("/word/document.xml");
    let rel = p.addRelationship(&target, TargetMode::INTERNAL, IMAGE, Some("rIdX"));
    assert_eq!(rel.getId(), "rIdX");
    assert_eq!(
        *rel.getPackage(),
        Some(p.as_ref() as *const OPCPackage),
        "★包级新建关系应携带本包引用（引用语义对齐）"
    );
}

fn part_name(s: &str) -> PackagePartName {
    PackagingURIHelper::createPartName_str(s).unwrap()
}
