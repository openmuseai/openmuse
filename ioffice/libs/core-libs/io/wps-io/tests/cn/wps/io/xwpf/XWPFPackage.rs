// function_id=09_function_xwpf_package
// status=loop — executable tests live beside src, not in features/
// cargo test --manifest-path libs/core-libs/io/wps-io/Cargo.toml --test 09_function_xwpf_package
// src:  libs/core-libs/io/wps-io/src/cn/wps/io/xwpf/XWPFPackage.rs
// test: libs/core-libs/io/wps-io/tests/cn/wps/io/xwpf/XWPFPackage.rs

use core_libs_io::io::opc::internal::ContentTypeManager::TYPES_NAMESPACE_URI;
use core_libs_io::io::opc::PackageRelationshipTypes::PackageRelationshipTypes;
use core_libs_io::io::opc::PackagingURIHelper::PackagingURIHelper;
use core_libs_io::io::xwpf::XWPFPackage::XWPFPackage;
use std::io::{Cursor, Write};
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

const MAIN_CT: &str =
    "application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml";
const RELS_CT: &str = "application/vnd.openxmlformats-package.relationships+xml";
const XML_CT: &str = "application/xml";
const RELS_NS: &str = "http://schemas.openxmlformats.org/package/2006/relationships";
const OFFICE_DOC: &str = PackageRelationshipTypes::CORE_DOCUMENT;
const CORE_PROPS: &str = PackageRelationshipTypes::CORE_PROPERTIES;

fn types_xml() -> String {
    format!(
        r#"<?xml version="1.0"?>
<Types xmlns="{TYPES_NAMESPACE_URI}">
  <Default Extension="rels" ContentType="{RELS_CT}"/>
  <Default Extension="xml" ContentType="{XML_CT}"/>
  <Override PartName="/word/document.xml" ContentType="{MAIN_CT}"/>
</Types>"#
    )
}

fn package_rels_xml() -> String {
    format!(
        r#"<?xml version="1.0"?>
<Relationships xmlns="{RELS_NS}">
  <Relationship Id="rId1" Type="{OFFICE_DOC}" Target="word/document.xml"/>
</Relationships>"#
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

fn min_docx_bytes() -> Vec<u8> {
    let types = types_xml();
    let rels = package_rels_xml();
    zip_bytes(&[
        ("[Content_Types].xml", types.as_bytes()),
        ("_rels/.rels", rels.as_bytes()),
        ("word/document.xml", b"<w:document/>"),
    ])
}

fn doc_name() -> core_libs_io::io::opc::PackagePartName::PackagePartName {
    PackagingURIHelper::createPartName_str("/word/document.xml").unwrap()
}

fn write_temp_docx() -> PathBuf {
    // ★修复既有 flake（本轮全量跑命中 1 次 `x4_open_raf` 失败；单跑通过）：
    //   旧实现只用**纳秒时间戳**做文件名 ⇒ 并行用例可能取到同一路径：一个用例
    //   `remove_file` 之后另一个用例还在读它 ⇒ `getPart` 找不到 part。
    //   ⇒ 与 `OPCPackage.rs`/`PackagePart.rs` 的同类修复同法：
    //   进程 id + 原子序号（**同一纳秒内也保证唯一**）+ `create_new` 抢占（已存在则换号重试）。
    static SEQ: AtomicU64 = AtomicU64::new(0);
    for _ in 0..1024 {
        let seq = SEQ.fetch_add(1, Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!(
            "ioffice-xwpf-09-{}-{}.docx",
            std::process::id(),
            seq
        ));
        match std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&path)
        {
            Ok(mut f) => {
                f.write_all(&min_docx_bytes()).unwrap();
                return path;
            }
            Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => continue,
            Err(e) => panic!("write temp docx: {e}"),
        }
    }
    panic!("write temp docx: exhausted 1024 attempts");
}

#[test]
fn x1_open_min_docx_has_parts() {
    let mut input = Cursor::new(min_docx_bytes());
    let mut pkg = XWPFPackage::open_stream(&mut input);
    assert!(!pkg.getParts().is_empty());
    assert!(pkg.getPart(&doc_name()).is_some());
    assert!(pkg.getPartList().unwrap().size() > 0);
}

#[test]
fn x2_core_properties_rel_optional() {
    let mut input = Cursor::new(min_docx_bytes());
    let mut pkg = XWPFPackage::open_stream(&mut input);
    assert_eq!(pkg.getRelationshipsByType(CORE_PROPS).size(), 0);
}

#[test]
fn x3_open_path_read() {
    let path = write_temp_docx();
    let mut pkg = XWPFPackage::open_path(path.to_str().unwrap(), 0);
    let ok = pkg.getPart(&doc_name()).is_some();
    let _ = std::fs::remove_file(&path);
    assert!(ok);
}

#[test]
fn x4_open_raf() {
    let path = write_temp_docx();
    let mut pkg = XWPFPackage::open_raf(&path, 0);
    let ok = pkg.getPart(&doc_name()).is_some();
    let _ = std::fs::remove_file(&path);
    assert!(ok);
}

#[test]
fn x5_open_path_write_skips_scan() {
    let path = write_temp_docx();
    let pkg = XWPFPackage::open_path(path.to_str().unwrap(), 1);
    let scanned = pkg.getPartList().is_some();
    let _ = std::fs::remove_file(&path);
    assert!(!scanned);
}

#[test]
#[should_panic(expected = "path")]
fn x6_empty_path_fails() {
    let _ = XWPFPackage::open_path("   ", 0);
}

// ═══════════════════════════════════════════════════════════════════════════
// ★2026-09-20 新增：`getParts()` 逻辑+门面对齐的真实管线用例组（`g1`–`g8`）
//
// 背景：用户点名第 4 项 —— "`getParts(&mut self) -> Vec<&PackagePart>` 这个方法逻辑和门面
// 似乎都没对齐"。本次改动落了四件事，下面每条用例都钉住其中一件，全部走**真实管线**
// （真 zip 字节 → 真解析 → 真 part 表 → 真 unmarshaller → 真 docProps 视图）：
//   ① `XWPFPackage::init()` 现在真的被调用（Java `OPCPackage` 构造器里的虚调用 `this.init()`）
//      ⇒ `partUnmarshallers` 表**不再为空**（旧状态：恒空 ⇒ Java 的表驱动派发无从生效）。
//   ② `getParts()` 的 `for` 循环体真的实现了 Java 的每条语句（旧状态：循环体只有注释、无代码）。
//   ③ `if (hasCorePropertiesPart) continue;` 是 **continue**（旧状态误写成 `break`，整个循环提前终止）。
//   ④ 派发经 `partUnmarshallers` 表 + `UnmarshallContext` 传入**真实包指针**（旧状态传 `None`）。
// ═══════════════════════════════════════════════════════════════════════════

const CORE_PROPS_CT: &str = "application/vnd.openxmlformats-package.core-properties+xml";
const APP_PROPS_CT: &str =
    "application/vnd.openxmlformats-officedocument.extended-properties+xml";

/// 造一个**真带 docProps/core.xml**（且具名标题/创建者）的最小 docx。
/// Java `XWPFPackage.getParts()` 的 `partUnmarshallers` 命中分支只在这类输入上才会走到。
fn docx_with_core_props() -> Vec<u8> {
    let types = format!(
        r#"<?xml version="1.0"?>
<Types xmlns="{TYPES_NAMESPACE_URI}">
  <Default Extension="rels" ContentType="{RELS_CT}"/>
  <Default Extension="xml" ContentType="{XML_CT}"/>
  <Override PartName="/word/document.xml" ContentType="{MAIN_CT}"/>
  <Override PartName="/docProps/core.xml" ContentType="{CORE_PROPS_CT}"/>
  <Override PartName="/docProps/app.xml" ContentType="{APP_PROPS_CT}"/>
</Types>"#
    );
    let rels = package_rels_xml();
    let core = br#"<?xml version="1.0" encoding="UTF-8"?>
<cp:coreProperties xmlns:cp="http://schemas.openxmlformats.org/package/2006/metadata/core-properties" xmlns:dc="http://purl.org/dc/elements/1.1/" xmlns:dcterms="http://purl.org/dc/terms/" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
  <dc:title>ioffice-s01-08</dc:title>
  <dc:creator>real-pipeline</dc:creator>
  <cp:lastModifiedBy>real-pipeline</cp:lastModifiedBy>
  <cp:revision>7</cp:revision>
</cp:coreProperties>"#;
    let app = br#"<?xml version="1.0" encoding="UTF-8"?>
<Properties xmlns="http://schemas.openxmlformats.org/officeDocument/2006/extended-properties">
  <Application>ioffice</Application>
  <Company>ioffice</Company>
</Properties>"#;
    zip_bytes(&[
        ("[Content_Types].xml", types.as_bytes()),
        ("_rels/.rels", rels.as_bytes()),
        ("word/document.xml", b"<w:document/>"),
        ("docProps/core.xml", core),
        ("docProps/app.xml", app),
    ])
}

fn write_temp_bytes(bytes: &[u8], tag: &str) -> PathBuf {
    static SEQ: AtomicU64 = AtomicU64::new(0);
    for _ in 0..1024 {
        let seq = SEQ.fetch_add(1, Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!(
            "ioffice-xwpf-09-{}-{}-{}.docx",
            tag,
            std::process::id(),
            seq
        ));
        match std::fs::OpenOptions::new().write(true).create_new(true).open(&path) {
            Ok(mut f) => {
                f.write_all(bytes).unwrap();
                return path;
            }
            Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => continue,
            Err(e) => panic!("write temp: {e}"),
        }
    }
    panic!("write temp: exhausted");
}

/// g1（门面）: `getParts()` 返回**整张 part 表**（Java `new ArrayList(this.partList.values())`），
/// 不是空表、也不是只含若干特例。
#[test]
fn g1_get_parts_returns_full_part_table() {
    let mut input = Cursor::new(docx_with_core_props());
    let mut pkg = XWPFPackage::open_stream(&mut input);
    let parts = pkg.getParts();
    let names: Vec<String> = parts
        .iter()
        .map(|p| p.getPartName().getName().to_string())
        .collect();
    // Java `getPartsImpl()` 的表来自 zip 扫描：本语料 5 个 entry − [Content_Types].xml = 4 个 part
    assert_eq!(parts.len(), 4, "part 表应含 4 条，实际 {names:?}");
    assert!(names.contains(&"/word/document.xml".to_string()), "{names:?}");
    assert!(!names.contains(&"/[Content_Types].xml".to_string()), "{names:?}");
}

/// g2（① 的观测点）: `XWPFPackage::init()` 真被调用 ⇒ `partUnmarshallers` 表非空
/// （Java `XWPFPackage.init()` 注册 3 条；旧状态恒空）。
#[test]
fn g2_xwpf_init_registers_part_unmarshallers() {
    let pkg = XWPFPackage::open_path(
        write_temp_bytes(&docx_with_core_props(), "g2").to_str().unwrap(),
        0,
    );
    assert!(
        pkg.getPartUnmarshallersLen() >= 3,
        "Java `XWPFPackage.init()` 注册 3 个属性 unmarshaller（core/extended/custom）；\
         实际 {} ⇒ `XWPFPackage::init()` 没有被调用",
        pkg.getPartUnmarshallersLen()
    );
}

/// g3（②+④ 的观测点，端到端）: 带 `docProps/core.xml` 的真语料走一遍
/// ⇒ 表命中 ⇒ `unmarshall` ⇒ **真正解析出**类型化 core 属性视图。
/// 这条是旧实现（循环体无代码、`UnmarshallContext` 传 `None`）拿不到的观测点。
#[test]
fn g3_core_properties_view_is_populated_by_real_pipeline() {
    let mut input = Cursor::new(docx_with_core_props());
    let mut pkg = XWPFPackage::open_stream(&mut input);
    let _ = pkg.getParts();
    let core = pkg
        .packagePropertiesView()
        .expect("★`getParts()` 必须经 `partUnmarshallers` 反序列化出 core 属性视图");
    assert_eq!(
        core.getTitleProperty().getValue().map(|v| v.as_str()),
        Some("ioffice-s01-08"),
        "标题应来自 `docProps/core.xml` 的真实解析结果"
    );
    assert_eq!(
        core.getCreatorProperty().getValue().map(|v| v.as_str()),
        Some("real-pipeline")
    );
}

/// g4（②的观测点）: 同一份语料里 extended 属性也走表驱动派发并落到
/// `extendedPackageProperties`（Java 的 `instanceof ExtendedPackagePropertiesPart` 分支）。
#[test]
fn g4_extended_properties_view_is_populated() {
    let mut input = Cursor::new(docx_with_core_props());
    let mut pkg = XWPFPackage::open_stream(&mut input);
    let _ = pkg.getParts();
    let ext = pkg
        .extendedPackagePropertiesView()
        .expect("★extended 属性视图应由表驱动派发填充");
    assert_eq!(ext.getApplicationProperty(), Some("ioffice"));
}

/// g5（③的观测点，回归钉子）: `hasCorePropertiesPart` 的 `continue` 语义 ——
/// 语料里**没有** core-properties 时，扫描不得提前终止（旧 `break` 写法会在第 2 轮就退出），
/// 故其后的 extended part 仍必须被处理到。
/// 这里用「无 core、有 extended」的语料把 `break` 会造成的缺失暴露出来。
#[test]
fn g5_scan_does_not_break_early_when_no_core_properties() {
    let types = format!(
        r#"<?xml version="1.0"?>
<Types xmlns="{TYPES_NAMESPACE_URI}">
  <Default Extension="rels" ContentType="{RELS_CT}"/>
  <Default Extension="xml" ContentType="{XML_CT}"/>
  <Override PartName="/word/document.xml" ContentType="{MAIN_CT}"/>
  <Override PartName="/docProps/app.xml" ContentType="{APP_PROPS_CT}"/>
</Types>"#
    );
    let app = br#"<?xml version="1.0" encoding="UTF-8"?>
<Properties xmlns="http://schemas.openxmlformats.org/officeDocument/2006/extended-properties">
  <Application>ioffice-no-core</Application>
</Properties>"#;
    let bytes = zip_bytes(&[
        ("[Content_Types].xml", types.as_bytes()),
        ("_rels/.rels", package_rels_xml().as_bytes()),
        ("word/document.xml", b"<w:document/>"),
        ("docProps/app.xml", app),
    ]);
    let mut input = Cursor::new(bytes);
    let mut pkg = XWPFPackage::open_stream(&mut input);
    let parts = pkg.getParts();
    assert_eq!(parts.len(), 3, "3 个 part（无 core）");
    assert!(pkg.packagePropertiesView().is_none(), "无 core-properties part ⇒ 视图为空");
    assert_eq!(
        pkg.extendedPackagePropertiesView()
            .and_then(|p| p.getApplicationProperty()),
        Some("ioffice-no-core"),
        "★`continue`(不是 `break`)：扫描必须走完所有 part，extended 视图仍要被填充"
    );
}

/// g6（门面）: `getParts()` 可重复调用且幂等（Java 只在 `partList == null` 时扫描一次；
/// 第二次调用直接返回既有表，不重复 unmarshall、不改变视图）。
#[test]
fn g6_get_parts_is_idempotent() {
    let mut input = Cursor::new(docx_with_core_props());
    let mut pkg = XWPFPackage::open_stream(&mut input);
    let first: Vec<String> = pkg
        .getParts()
        .iter()
        .map(|p| p.getPartName().getName().to_string())
        .collect();
    let second: Vec<String> = pkg
        .getParts()
        .iter()
        .map(|p| p.getPartName().getName().to_string())
        .collect();
    assert_eq!(first.len(), second.len());
    let mut a = first.clone();
    let mut b = second.clone();
    a.sort();
    b.sort();
    assert_eq!(a, b);
}

/// g7（门面）: `getParts()` 的结果与 `getPart(name)` 查表**指向同一批 part**
/// （Java：`getParts()` 返回的正是 `partList` 的值集合，`getPart` 也查同一张表）。
#[test]
fn g7_get_parts_agrees_with_get_part() {
    let mut input = Cursor::new(docx_with_core_props());
    let mut pkg = XWPFPackage::open_stream(&mut input);
    // 先借 `getParts()` 的快照（`&pkg`），作用域结束后再用 `&mut pkg` 调 `getPart`
    let (doc_name_from_parts, doc_ct, table_len) = {
        let parts = pkg.getParts();
        let doc = parts
            .iter()
            .find(|p| p.getPartName().getName() == "/word/document.xml")
            .expect("document.xml 在表内");
        (
            doc.getPartName().getName().to_string(),
            doc.getContentType().toString(),
            parts.len(),
        )
    };
    let by_name = pkg.getPart(&doc_name()).expect("`getPart` 也取到同一 part");
    assert_eq!(doc_name_from_parts, by_name.getPartName().getName());
    assert_eq!(doc_ct, by_name.getContentType().toString());
    assert_eq!(table_len, pkg.getPartList().unwrap().size(), "两处同一张表");
}

/// g8（真实 C1 语料）: 用冻结的 `standard-hello.docx`（含 `docProps/core.xml`）走同一条
/// 真实管线，确认 `getParts()` + docProps 视图在真实 OOXML 标准语料上同样成立。
#[test]
fn g8_real_c1_fixture_populates_core_properties() {
    let fixture = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/wave0-ooxml-standards/standard-hello.docx");
    let mut pkg = XWPFPackage::open_path(fixture.to_str().unwrap(), 0);
    let parts = pkg.getParts();
    assert_eq!(parts.len(), 9, "C1 语料 10 个 entry − [Content_Types].xml = 9 个 part");
    let core = pkg
        .packagePropertiesView()
        .expect("C1 有 docProps/core.xml ⇒ 视图必须被填充");
    assert!(
        core.getCreatorProperty().getValue().is_some(),
        "C1 的 core.xml 带 dc:creator"
    );
}
