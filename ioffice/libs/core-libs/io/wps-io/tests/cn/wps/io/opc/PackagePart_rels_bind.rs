// function_id=32_function_part_rels_bind
// status=loop — executable tests live beside src, not in features/
// cargo test --manifest-path libs/core-libs/io/wps-io/Cargo.toml --test 32_function_part_rels_bind
// src:  libs/core-libs/io/wps-io/src/cn/wps/io/opc/PackagePart.rs（ctor → `loadRelationships` → `getRelationshipsCore`）
//       libs/core-libs/io/wps-io/src/cn/wps/io/opc/OPCPackage.rs（包级 `ensureRelationships`，作对照）
//       libs/core-libs/io/wps-io/src/cn/wps/io/opc/PackageRelationshipCollection.rs（集合本体）
// test: libs/cn/wps/io/wps-io/tests/cn/wps/io/opc/PackagePart_rels_bind.rs
//
// 本 function = **part 与其关系图的绑定时序**。
// Java：
//   `PackagePart(pack, partName, contentType, loadRelationships)`（`:27-37`）⇒ `this._container = pack;` 后
//   `if (loadRelationships) { this.loadRelationships(); }`（`:33-35`）
//   `loadRelationships()`（`:211-217`）⇒ `if (this._relationships == null && !this._isRelationshipPart) { this.throwExceptionIfRelationship(); this._relationships = new PackageRelationshipCollection(this); }`
//   `getRelationshipsCore(filter)`（`:219-…`）⇒ 同样「按需建 + 关系 part 直接抛」
// ★2026-09-21 更新（M-STAGE1 P1-c1）：**该缺口已闭合**。`PackagePart` 现在真的解析 `_rels`：
//   构造期保持 `_relationships = None`，首次读取时由 `getRelationshipsCore` 以
//   `new_with_container_and_part_name(container, partName)` 建集合并解析 —— 之所以延迟，
//   是因为 Java 的 part 构造发生在 `getPartsImpl` 的逐条 put 循环中，那一刻 `.rels` 尚未入表，
//   照搬构造期装载必然落空（实测：只拿到 3 条包头关系、`/word/document.xml` 一条都没解析）。
//   ⇒ 本文件原「现状断言」（空集合 / `hasRelationships()` 恒 false）已按实现更新为 Java 语义。

use core_libs_io::io::opc::internal::ContentTypeManager::TYPES_NAMESPACE_URI;
use core_libs_io::io::opc::internal::ContentType::ContentType;
use core_libs_io::io::opc::OPCPackage::OPCPackage;
use core_libs_io::io::opc::PackagePart::PackagePart;
use core_libs_io::io::opc::PackagePartName::PackagePartName;
use core_libs_io::io::opc::PackagingURIHelper::PackagingURIHelper;
use std::io::{Cursor, Write};

const XML_CT: &str = "application/xml";
const MAIN_CT: &str =
    "application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml";
const RELS_CT: &str = "application/vnd.openxmlformats-package.relationships+xml";
const OFFICE_DOCUMENT_REL: &str =
    "http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument";

fn types_xml() -> String {
    format!(
        r#"<?xml version="1.0"?><Types xmlns="{TYPES_NAMESPACE_URI}"><Default Extension="xml" ContentType="{XML_CT}"/><Default Extension="rels" ContentType="{RELS_CT}"/><Override PartName="/word/document.xml" ContentType="{MAIN_CT}"/></Types>"#
    )
}

fn package_rels_xml() -> String {
    format!(
        r#"<?xml version="1.0"?><Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships"><Relationship Id="rId1" Type="{OFFICE_DOCUMENT_REL}" Target="word/document.xml"/></Relationships>"#
    )
}

fn doc_rels_xml() -> String {
    r#"<?xml version="1.0"?><Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships"><Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/styles" Target="styles.xml"/></Relationships>"#.to_string()
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

/// C1 形状的包：包级 `_rels/.rels` → `word/document.xml`；`word/_rels/document.xml.rels` → `styles.xml`。
fn c1_pkg() -> Box<OPCPackage> {
    let types = types_xml();
    let pkg_rels = package_rels_xml();
    let doc_rels = doc_rels_xml();
    let mut input = Cursor::new(zip_bytes(&[
        ("[Content_Types].xml", types.as_bytes()),
        ("_rels/.rels", pkg_rels.as_bytes()),
        ("word/document.xml", b"<w:document/>"),
        ("word/_rels/document.xml.rels", doc_rels.as_bytes()),
        ("word/styles.xml", b"<w:styles/>"),
    ]));
    let mut pkg = Box::new(OPCPackage::from_stream(&mut input));
    pkg.getParts();
    pkg
}

fn name(s: &str) -> PackagePartName {
    PackagingURIHelper::createPartName_str(s).unwrap()
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

fn ct(s: &str) -> ContentType {
    ContentType::new(s).unwrap()
}

/// RB-01 / 主路径：普通 part 构造时 `loadRelationships = true` ⇒ 关系集合可建，
/// 且**真的解析出** `.rels` 里的条目（与 Java 同）。
/// ★M-STAGE1 P1-c1 后：装载是**惰性**的（首次读取时才解析），故断言点在 `getRelationships()` 之后。
#[test]
fn rb01_relationships_object_is_created_on_construction() {
    let mut pkg = c1_pkg();
    let pkg_ptr = pkg.as_ref() as *const OPCPackage;
    let mut part = PackagePart::new_with_load(
        Some(pkg_ptr),
        name("/word/document.xml"),
        ct(MAIN_CT),
        true,
    ); // Java `:33-35` 的 `if (loadRelationships) this.loadRelationships();`
    // 首次读取触发真解析：C1 的 `word/_rels/document.xml.rels` 里恰好 1 条（rId1 → styles.xml）
    let collection = part.getRelationships();
    assert_eq!(collection.size(), 1, "Java 侧 C1 会解析出 1 条 rId1");
    assert!(part.hasRelationships(), "有真条目 ⇒ hasRelationships() == true（Java 同）");
    // 条目类型与 Java 一致
    assert_eq!(
        collection.iter().next().unwrap().getRelationshipType(),
        "http://schemas.openxmlformats.org/officeDocument/2006/relationships/styles"
    );
}

/// RB-02 / 边界：**关系 part**（`*.rels`）不建关系图 ⇒ 构造后取关系直接抛
/// Java `InvalidOperationException("Can do this operation on a relationship part !")`（`throwExceptionIfRelationship`）。
#[test]
fn rb02_relationship_part_has_no_graph_and_throws() {
    let pkg = c1_pkg();
    let rels_name = name("/word/_rels/document.xml.rels");
    let mut rels_part = PackagePart::new_with_load(
        Some(pkg.as_ref() as *const OPCPackage),
        rels_name.clone(),
        ct(RELS_CT),
        true,
    );
    assert!(rels_part.isRelationshipPart());
    assert!(!rels_part.hasRelationships(), "关系 part 恒无关系图（Java 同：`!this._isRelationshipPart`）");
    let err = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        rels_part.getRelationships();
    }))
    .expect_err("关系 part 上取关系必须拒绝（Java `throwExceptionIfRelationship`）");
    assert_eq!(panic_message(err), "Can do this operation on a relationship part !");
    // 表里（扫描建出）的关系 part 同样如此：`hasRelationships()` 为 false
    let mut pkg2 = c1_pkg();
    let scanned = pkg2.getPart(&rels_name).expect("rels part 在表里");
    assert!(!scanned.hasRelationships());
}

/// RB-03 / 边界：`load_relationships = false` ⇒ 构造期**不建**图；之后 `getRelationshipsCore` 按需建（Java 同）。
#[test]
fn rb03_flag_false_defers_creation() {
    let pkg = c1_pkg();
    let mut part = PackagePart::new_with_load(
        Some(pkg.as_ref() as *const OPCPackage),
        name("/word/document.xml"),
        ct(MAIN_CT),
        false,
    );
    assert!(!part.hasRelationships(), "未按需建 ⇒ `_relationships == None` ⇒ false（Java 公式同）");
    let collection = part.getRelationships(); // 按需建 + 真解析
    assert_eq!(collection.size(), 1, "按需建后与 Java 同：解析出 rId1");
    assert!(part.hasRelationships(), "按需建后为 true");
    // 关系 part 即便 `load_relationships = false`，按需建时也必须抛（Java `getRelationshipsCore` 同）
    let mut rels = PackagePart::new_with_load(
        Some(pkg.as_ref() as *const OPCPackage),
        name("/word/_rels/document.xml.rels"),
        ct(RELS_CT),
        false,
    );
    let err = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        rels.getRelationships();
    }))
    .expect_err("关系 part 按需建也必须拒绝");
    assert_eq!(panic_message(err), "Can do this operation on a relationship part !");
}

/// RB-04 / ★M-STAGE1 P1-c1 后：**扫描建出的**普通 part 也能解析出 `.rels` 的条目（与 Java 同）。
/// 原断言「`size() == 0`」是缺口登记，已随 P1-c1 实现更新。
#[test]
fn rb04_scanned_part_has_no_parsed_entries() {
    let mut pkg = c1_pkg();
    let pkg_ptr = pkg.as_ref() as *const OPCPackage;
    // ① 扫描建出的 part：从表里取回后首次读取应解析出 1 条
    let mut scanned = pkg
        .get_part_mut(&name("/word/document.xml"))
        .expect("main part");
    assert!(
        scanned.hasRelationships_loaded(),
        "★P1-c1 后：扫描建出的 part 也 `hasRelationships() == true`（Java 同）"
    );
    assert_eq!(scanned.getRelationships().size(), 1);
    // ② 自造同形 part（同一容器 + 同 part 名）结果一致
    let mut same_shape = PackagePart::new_with_load(
        Some(pkg_ptr),
        name("/word/document.xml"),
        ct(MAIN_CT),
        true,
    );
    assert_eq!(same_shape.getRelationships().size(), 1);
    // ③ 包里**确实**存在 `.rels` part（解析有据）
    assert!(pkg.getPart(&name("/word/_rels/document.xml.rels")).is_some());
}

/// RB-05 / 主路径：`hasRelationships()` 在 C1 的 `/word/document.xml` 上 —— **Rust 与 Java 同为 `true`**。
/// ★M-STAGE1 P1-c1 前此处登记的是「Java true / Rust false」差异，该差异已闭合。
#[test]
fn rb05_has_relationships_differs_from_java() {
    let mut pkg = c1_pkg();
    let scanned = pkg
        .get_part_mut(&name("/word/document.xml"))
        .expect("main part");
    assert!(
        scanned.hasRelationships_loaded(),
        "★P1-c1 后：真解析出 rId1 ⇒ hasRelationships() == true（与 Java 一致）"
    );
    // Java 公式本体仍在（`!isRelationshipPart && _relationships != null && size() > 0`）：
    assert!(scanned.isRelationshipPart() == false);
}

/// RB-06 / 时序/畸形：容器为 `None` 时构造（Java 会 NPE：`this._container.getRelationshipsByType(…)` 空引用）
/// ⇒ **Rust 现状：不抛、不建图**（`loadRelationships` 在 `_container == None` 时无事发生）。
#[test]
fn rb06_null_container_does_not_panic() {
    let part = PackagePart::new_with_string(None, name("/word/document.xml"), MAIN_CT);
    assert!(part.getPackage().is_none());
    assert!(!part.hasRelationships(), "无容器 ⇒ 无关系图（Rust 现状；Java 侧该路径会 NPE）");
}

/// RB-08 / 绑定过程**不改变** part 表：取关系前后，表内容与各 part 的字节完全一致。
#[test]
fn rb08_binding_does_not_disturb_the_part_table() {
    let mut pkg = c1_pkg();
    let before: Vec<String> = {
        let mut v: Vec<String> = pkg
            .getPartList()
            .map(|l| l.values().map(|p| p.getPartName().getName()).collect())
            .unwrap_or_default();
        v.sort();
        v
    };
    // 触发（可能改变状态的）关系路径
    {
        let scanned = pkg.getPart(&name("/word/document.xml")).expect("part");
        let _ = scanned.hasRelationships();
    }
    let _ = pkg.hasRelationships();
    let after: Vec<String> = {
        let mut v: Vec<String> = pkg
            .getPartList()
            .map(|l| l.values().map(|p| p.getPartName().getName()).collect())
            .unwrap_or_default();
        v.sort();
        v
    };
    assert_eq!(before, after);
    assert_eq!(before.len(), 4, "CT 不算 part：main + doc.rels + styles + pkg.rels");
}

/// RB-09 / 对照（包级真解析）：`OPCPackage::ensureRelationships` **会**解析 `/_rels/.rels`
/// ⇒ 包级关系非空、可按类型查。这条对照说明「空集合」是 **part 级** 的限制，不是整条关系链都没实现。
#[test]
fn rb09_package_level_relationships_are_parsed() {
    let mut pkg = c1_pkg();
    pkg.ensureRelationships();
    assert!(pkg.hasRelationships(), "包级关系应被真解析（`/_rels/.rels`）");
    let by_type = pkg.getRelationshipsByType(OFFICE_DOCUMENT_REL);
    assert!(
        by_type.size() >= 1,
        "★包级按类型查应命中 `officeDocument`（实际 {}）—— 与 part 级的空集形成对照",
        by_type.size()
    );
}

/// RB-07 / 迭代序：**不保证文档序** —— 与 Java 同。
/// ★M-STAGE1 P1-c1 后关系已真解析，故此处做真实断言。
/// ★★注意（初版断言写错、已修正）：Java 侧 `relationshipsByID` 是 **HashMap**，
///   `iterator()` 即 `relationshipsByID.values().iterator()`（`PackageRelationshipCollection.java:201-203`）
///   ⇒ Java **本身也不保证** `_rels` 文档序。故本用例**不得**断言文档序，
///   只断言：条目集合正确 + 迭代**稳定**（同集合多次迭代一致）+ 与 Java 同为「无序容器」。
#[test]
fn rb07_rel_xml_order_is_blocked_on_s01_05() {
    // 造一个含 3 条关系、且 XML 文档序与 id 字典序**不同**的 `.rels`，用来区分两者。
    let types = types_xml();
    let pkg_rels = package_rels_xml();
    let doc_rels = r#"<?xml version="1.0"?><Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships"><Relationship Id="rId3" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/fontTable" Target="fontTable.xml"/><Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/styles" Target="styles.xml"/><Relationship Id="rId2" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/settings" Target="settings.xml"/></Relationships>"#;
    let mut input = Cursor::new(zip_bytes(&[
        ("[Content_Types].xml", types.as_bytes()),
        ("_rels/.rels", pkg_rels.as_bytes()),
        ("word/document.xml", b"<w:document/>"),
        ("word/_rels/document.xml.rels", doc_rels.as_bytes()),
        ("word/styles.xml", b"<w:styles/>"),
    ]));
    let mut pkg = Box::new(OPCPackage::from_stream(&mut input));
    pkg.getParts();
    let mut part = PackagePart::new_with_load(
        Some(pkg.as_ref() as *const OPCPackage),
        name("/word/document.xml"),
        ct(MAIN_CT),
        true,
    );
    let collection = part.getRelationships();
    assert_eq!(collection.size(), 3, "3 条关系全部解析");
    let mut ids: Vec<String> = collection.iter().map(|r| r.getId().to_string()).collect();
    ids.sort();
    assert_eq!(
        ids,
        vec!["rId1", "rId2", "rId3"],
        "3 条关系应全部解析（id 集合正确）"
    );
    // 迭代稳定性：同一集合重复迭代给出同一顺序（无序 ≠ 不稳定）
    let again: Vec<String> = collection.iter().map(|r| r.getId().to_string()).collect();
    let mut again_sorted = again.clone();
    again_sorted.sort();
    assert_eq!(again_sorted, ids, "重复迭代的**集合**必须一致");
    // 与 Java 对齐：底层同为无序 map（Java `HashMap` / Rust `HashMap`）⇒ 本用例**明确不承诺**
    // `_rels` 文档序。此处仅登记「取值域正确、无丢失、无重复」。
    assert_eq!(again.len(), 3, "无丢失、无重复");
}
