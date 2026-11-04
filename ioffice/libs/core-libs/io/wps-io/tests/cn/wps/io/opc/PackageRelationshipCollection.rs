// function_id=04_function_opc_relationships
// status=loop — executable tests live beside src, not in features/
// cargo test --manifest-path libs/core-libs/io/wps-io/Cargo.toml --test 04_function_opc_relationships
// src:  libs/core-libs/io/wps-io/src/cn/wps/io/opc/PackageRelationshipCollection.rs
// test: libs/core-libs/io/wps-io/tests/cn/wps/io/opc/PackageRelationshipCollection.rs

use core_libs_io::io::opc::PackageRelationshipCollection::PackageRelationshipCollection;
use core_libs_io::io::opc::TargetMode::TargetMode;
use core_libs_io::io::util::URI::URI;
use std::io::Cursor;

const RELS_NS: &str = "http://schemas.openxmlformats.org/package/2006/relationships";
const OFFICE_DOC: &str =
    "http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument";
const CORE_PROPS: &str =
    "http://schemas.openxmlformats.org/package/2006/relationships/metadata/core-properties";

fn sample_package_rels() -> String {
    format!(
        r#"<?xml version="1.0"?>
<Relationships xmlns="{RELS_NS}">
  <Relationship Id="rId1" Type="{OFFICE_DOC}" Target="word/document.xml"/>
</Relationships>"#
    )
}

fn coll_from_xml(xml: &str) -> PackageRelationshipCollection {
    let mut coll = PackageRelationshipCollection::new();
    let mut input = Cursor::new(xml.as_bytes().to_vec());
    coll.parseRelationshipsXml(&mut input);
    coll
}

fn target_text(coll: &PackageRelationshipCollection, id: &str) -> String {
    coll.getRelationshipByID(id)
        .expect(id)
        .getTargetURI()
        .toString()
}

/// R1 / C01: 包级 officeDocument → word/document.xml。
#[test]
fn r1_office_document_target() {
    let coll = coll_from_xml(&sample_package_rels());
    assert_eq!(coll.size(), 1);
    let rel = coll.getRelationshipByID("rId1").unwrap();
    assert_eq!(rel.getId(), "rId1");
    assert_eq!(rel.getRelationshipType(), OFFICE_DOC);
    assert_eq!(rel.getTargetMode(), TargetMode::INTERNAL);
    assert!(target_text(&coll, "rId1").contains("word/document.xml"));
}

/// R2 / C02: 同 Type 两条，Id 可区分。
#[test]
fn r2_multiple_rels_keep_ids() {
    let xml = format!(
        r#"<Relationships xmlns="{RELS_NS}">
  <Relationship Id="rId1" Type="{OFFICE_DOC}" Target="word/document.xml"/>
  <Relationship Id="rId2" Type="{OFFICE_DOC}" Target="word/glossary/document.xml"/>
</Relationships>"#
    );
    let coll = coll_from_xml(&xml);
    assert_eq!(coll.size(), 2);
    assert_eq!(coll.getRelationships(Some(OFFICE_DOC)).size(), 2);
    assert!(coll.getRelationshipByID("rId1").is_some());
    assert!(coll.getRelationshipByID("rId2").is_some());
    assert!(target_text(&coll, "rId2").contains("glossary"));
}

/// R3 / C03: 缺 Type 不得插入空边。
#[test]
#[should_panic(expected = "Can't read relationships part")]
fn r3_rejects_missing_type() {
    let xml = format!(
        r#"<Relationships xmlns="{RELS_NS}">
  <Relationship Id="rId1" Target="word/document.xml"/>
</Relationships>"#
    );
    let _ = coll_from_xml(&xml);
}

/// R4 / C04: 空 Relationships，循环 0 次。
#[test]
fn r4_empty_relationships() {
    let xml = format!(r#"<Relationships xmlns="{RELS_NS}"/>"#);
    let coll = coll_from_xml(&xml);
    assert_eq!(coll.size(), 0);
}

/// R5 / C05: TargetMode=External 保持外部 URI。
#[test]
fn r5_external_target_mode() {
    let xml = format!(
        r#"<Relationships xmlns="{RELS_NS}">
  <Relationship Id="rId9" Type="{OFFICE_DOC}" Target="https://example.com/doc" TargetMode="External"/>
</Relationships>"#
    );
    let coll = coll_from_xml(&xml);
    let rel = coll.getRelationshipByID("rId9").unwrap();
    assert_eq!(rel.getTargetMode(), TargetMode::EXTERNAL);
    assert!(rel.getTargetURI().toString().contains("example.com/doc"));
}

/// R6 / C06: 两条 core-properties → M4.1。
#[test]
#[should_panic(expected = "OPC Compliance error [M4.1]")]
fn r6_duplicate_core_properties_rejected() {
    let xml = format!(
        r#"<Relationships xmlns="{RELS_NS}">
  <Relationship Id="rId1" Type="{CORE_PROPS}" Target="docProps/core.xml"/>
  <Relationship Id="rId2" Type="{CORE_PROPS}" Target="docProps/core2.xml"/>
</Relationships>"#
    );
    let _ = coll_from_xml(&xml);
}

/// R7 / C07: Target 反斜杠换成 `/`。
#[test]
fn r7_backslash_target_normalized() {
    let xml = format!(
        r#"<Relationships xmlns="{RELS_NS}">
  <Relationship Id="rId1" Type="{OFFICE_DOC}" Target="word\document.xml"/>
</Relationships>"#
    );
    let coll = coll_from_xml(&xml);
    let text = target_text(&coll, "rId1");
    assert!(!text.contains('\\'), "{text}");
    assert!(text.contains("word/document.xml") || text.contains("document.xml"), "{text}");
}

/// R8 / C08: type filter 只留下匹配边。
#[test]
fn r8_type_filter() {
    let xml = format!(
        r#"<Relationships xmlns="{RELS_NS}">
  <Relationship Id="rId1" Type="{OFFICE_DOC}" Target="word/document.xml"/>
  <Relationship Id="rId2" Type="{CORE_PROPS}" Target="docProps/core.xml"/>
</Relationships>"#
    );
    let coll = coll_from_xml(&xml);
    let office = coll.getRelationships(Some(OFFICE_DOC));
    assert_eq!(office.size(), 1);
    assert!(office.getRelationshipByID("rId1").is_some());
    assert!(office.getRelationshipByID("rId2").is_none());
    assert_eq!(coll.iter_with_filter(CORE_PROPS).len(), 1);
}

/// R9 / C09: 缺 Target panic。
#[test]
#[should_panic(expected = "Can't read relationships part")]
fn r9_rejects_missing_target() {
    let xml = format!(
        r#"<Relationships xmlns="{RELS_NS}">
  <Relationship Id="rId1" Type="{OFFICE_DOC}"/>
</Relationships>"#
    );
    let _ = coll_from_xml(&xml);
}

/// R10 / C10: addRelationship 在 id 占用时分配新 rId。
#[test]
fn r10_add_relationship_allocates_id() {
    let mut coll = PackageRelationshipCollection::new();
    let uri = URI::new("word/document.xml").unwrap();
    let first = coll
        .addRelationship(uri.clone(), TargetMode::INTERNAL, OFFICE_DOC, Some("rId1"))
        .getId()
        .to_string();
    assert_eq!(first, "rId1");
    let second = coll
        .addRelationship(uri, TargetMode::INTERNAL, OFFICE_DOC, Some("rId1"))
        .getId()
        .to_string();
    assert_ne!(second, "rId1");
    assert!(second.starts_with("rId"));
    assert_eq!(coll.size(), 2);
}

/// R11 / C11: remove + clear。
#[test]
fn r11_remove_and_clear() {
    let mut coll = coll_from_xml(&sample_package_rels());
    coll.removeRelationship("rId1");
    assert_eq!(coll.size(), 0);
    let mut coll = coll_from_xml(&sample_package_rels());
    coll.clear();
    assert_eq!(coll.size(), 0);
    assert!(coll.getRelationshipByID("rId1").is_none());
}

/// R12 / C12: 非 XML 不得空表成功。
#[test]
#[should_panic(expected = "Can't read relationships part")]
fn r12_rejects_non_xml() {
    let _ = coll_from_xml("not xml");
}

// ---------------------------------------------------------------------------
// R13…R17（★2026-09-18 新增）：**构造器解析尾巴**（Java `PackageRelationshipCollection.java:50-66`）
//   Java: this.relationshipPartName = getRelationshipPartName(part);
//         this.relationshipPart = container.getPart(this.relationshipPartName);
//         if (this.relationshipPart != null) this.parseRelationshipsPart(this.relationshipPart);
//   ★改动前 Rust 只写注释、**不解析**（`relationshipPartName`/`relationshipPart` 恒 `None`）⇒ 这是
//     `s01-04` 的 `32` §1.7.1 ②「空集合」的真正根因之一（另一部分是 `PackagePart::loadRelationships`
//     用无容器的 `new()`）。
// 测试驱动：造含 `_rels` 的真实包（内存 zip）→ 用「同名的自有 part」作 `sourcePart`
//   （Java 传的是包内那个 part 的引用；Rust 没有可变/可克隆的 part 句柄 ⇒ 用等值替身，见 §1.7.1 ②）。
// ---------------------------------------------------------------------------

use core_libs_io::io::opc::internal::ContentTypeManager::TYPES_NAMESPACE_URI;
use core_libs_io::io::opc::OPCPackage::OPCPackage;
use core_libs_io::io::opc::PackagePart::PackagePart;
use core_libs_io::io::opc::PackagePartName::PackagePartName;
use core_libs_io::io::opc::PackagingURIHelper::PackagingURIHelper;
use std::io::Write;

const XML_CT: &str = "application/xml";
const MAIN_CT: &str =
    "application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml";
const RELS_CT: &str = "application/vnd.openxmlformats-package.relationships+xml";

fn types_xml() -> String {
    format!(
        r#"<?xml version="1.0"?><Types xmlns="{TYPES_NAMESPACE_URI}"><Default Extension="xml" ContentType="{XML_CT}"/><Default Extension="rels" ContentType="{RELS_CT}"/><Override PartName="/word/document.xml" ContentType="{MAIN_CT}"/></Types>"#
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

fn name(s: &str) -> PackagePartName {
    PackagingURIHelper::createPartName_str(s).unwrap()
}

/// C1 形状：包级 `_rels/.rels`（officeDocument）+ `word/document.xml` + `word/_rels/document.xml.rels`。
fn c1_pkg() -> Box<OPCPackage> {
    let types = types_xml();
    let pkg_rels = sample_package_rels();
    let doc_rels = format!(
        r#"<?xml version="1.0"?><Relationships xmlns="{RELS_NS}"><Relationship Id="rId7" Type="{CORE_PROPS}" Target="docProps/core.xml"/></Relationships>"#
    );
    let mut input = Cursor::new(zip_bytes(&[
        ("[Content_Types].xml", types.as_bytes()),
        ("_rels/.rels", pkg_rels.as_bytes()),
        ("word/document.xml", b"<w:document/>"),
        ("word/_rels/document.xml.rels", doc_rels.as_bytes()),
        ("docProps/core.xml", b"<cp:coreProperties/>"),
    ]));
    let mut pkg = Box::new(OPCPackage::from_stream(&mut input));
    pkg.getParts(); // 建 partList（解析时要按名查 `.rels` part）
    pkg
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

/// R13 / 主路径（★本 Loop 落地的能力）：带容器的构造器**真解析**该 part 的 `.rels`。
#[test]
fn r13_container_ctor_parses_the_source_parts_rels() {
    let pkg = c1_pkg();
    // `sourcePart` 用「同名的自有 part」作等值替身（Java 传包内引用；Rust 无可变 part 句柄）。
    let source = PackagePart::new_with_string(None, name("/word/document.xml"), MAIN_CT);
    let coll = PackageRelationshipCollection::new_with_container_and_part(
        Some(pkg.as_ref() as *const OPCPackage),
        // ★2026-09-20：`part` 形参由 `Option<Box<dyn Any>>` 改为真实类型别名 `Option<*const PackagePart>`
        //   （Java 存的是引用，不搬走对象）⇒ 传 source 的地址；`source` 是本测试的局部，存活到调用结束。
        Some(&source as *const PackagePart),
    );
    assert_eq!(coll.size(), 1, "★构造期应解析出 `word/_rels/document.xml.rels` 里的 1 条关系");
    assert_eq!(coll.getRelationshipByID("rId7").expect("rId7").getRelationshipType(), CORE_PROPS);
    // 目标已解析为**绝对 part 名**（Java `PackageRelationship` 构造时用 PackagingURIHelper 解析）
    let target = target_text(&coll, "rId7");
    assert!(target.ends_with("docProps/core.xml"), "target = {target}");
}

/// R14 / 包级构造器：`part == null` ⇒ 关系 part 名取 `PACKAGE_ROOT_PART_NAME` ⇒ 解析 `/_rels/.rels`
/// （Java `PackageRelationshipCollection(OPCPackage)`；`OPCPackage::ensureRelationships` 走的就是这条语义）。
#[test]
fn r14_container_only_ctor_parses_package_level_rels() {
    let pkg = c1_pkg();
    let coll = PackageRelationshipCollection::new_with_container_and_part(
        Some(pkg.as_ref() as *const OPCPackage),
        None,
    );
    assert_eq!(coll.size(), 1, "包级 `/_rels/.rels` 应被解析出 1 条 officeDocument 关系");
    assert_eq!(coll.getRelationshipByID("rId1").expect("rId1").getRelationshipType(), OFFICE_DOC);
}

/// R15 / 失败：`container == null` ⇒ Java `IllegalArgumentException("container")`（Rust = panic 同文案）。
#[test]
fn r15_null_container_is_rejected() {
    let err = std::panic::catch_unwind(|| {
        PackageRelationshipCollection::new_with_container_and_part(None, None);
    })
    .unwrap_err();
    assert_eq!(panic_message(err), "container");
}

/// R16 / 失败：`sourcePart` 本身是**关系 part**（`*.rels`）⇒ Java `IllegalArgumentException("part")`。
#[test]
fn r16_relationship_part_source_is_rejected() {
    let pkg = c1_pkg();
    let source = PackagePart::new_with_string(None, name("/word/_rels/document.xml.rels"), RELS_CT);
    assert!(source.isRelationshipPart());
    let err = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        PackageRelationshipCollection::new_with_container_and_part(
            Some(pkg.as_ref() as *const OPCPackage),
            // ★2026-09-20：`part` 形参由 `Option<Box<dyn Any>>` 改为真实类型别名 `Option<*const PackagePart>`
        //   （Java 存的是引用，不搬走对象）⇒ 传 source 的地址；`source` 是本测试的局部，存活到调用结束。
        Some(&source as *const PackagePart),
        );
    }))
    .unwrap_err();
    assert_eq!(panic_message(err), "part");
}

/// R17 / 边界：包内**没有**对应 `.rels` part ⇒ Java `relationshipPart == null` ⇒ 跳过解析、集合为空、不报错。
#[test]
fn r17_missing_rels_part_yields_empty_collection() {
    let mut input = Cursor::new(zip_bytes(&[
        ("[Content_Types].xml", types_xml().as_bytes()),
        ("word/document.xml", b"<w:document/>"),
    ]));
    let mut pkg = Box::new(OPCPackage::from_stream(&mut input));
    pkg.getParts();
    let source = PackagePart::new_with_string(None, name("/word/document.xml"), MAIN_CT);
    let coll = PackageRelationshipCollection::new_with_container_and_part(
        Some(pkg.as_ref() as *const OPCPackage),
        // ★2026-09-20：`part` 形参由 `Option<Box<dyn Any>>` 改为真实类型别名 `Option<*const PackagePart>`
        //   （Java 存的是引用，不搬走对象）⇒ 传 source 的地址；`source` 是本测试的局部，存活到调用结束。
        Some(&source as *const PackagePart),
    );
    assert_eq!(coll.size(), 0);
}
