// function_id=42_function_docprops_views
// cargo test --manifest-path libs/core-libs/io/wps-io/Cargo.toml --test 42_function_docprops_views
// src:  libs/core-libs/io/wps-io/src/cn/wps/io/xwpf/XWPFPackage.rs
// test: libs/core-libs/io/wps-io/tests/cn/wps/io/xwpf/XWPFPackage.rs
//
// 覆盖矩阵 §3.2 的 DG1–DG15：`XWPFPackage::getParts` 的 docProps 分支 + 三个懒加载 getter
// （Java `XWPFPackage.java:198-216` / `:301-323`）。

use core_libs_io::io::opc::internal::ContentTypeManager::TYPES_NAMESPACE_URI;
use core_libs_io::io::opc::PackageRelationshipTypes::PackageRelationshipTypes;
use core_libs_io::io::opc::PackagingURIHelper::PackagingURIHelper;
use core_libs_io::io::xwpf::XWPFPackage::XWPFPackage;
use std::io::{Cursor, Write};

const MAIN_CT: &str = "application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml";
const RELS_CT: &str = "application/vnd.openxmlformats-package.relationships+xml";
const XML_CT: &str = "application/xml";
const CORE_CT: &str = "application/vnd.openxmlformats-package.core-properties+xml";
const APP_CT: &str = "application/vnd.openxmlformats-officedocument.extended-properties+xml";
const CUSTOM_CT: &str = "application/vnd.openxmlformats-officedocument.custom-properties+xml";
const RELS_NS: &str = "http://schemas.openxmlformats.org/package/2006/relationships";
const OFFICE_DOC: &str = PackageRelationshipTypes::CORE_DOCUMENT;
const NS_CP: &str = "http://schemas.openxmlformats.org/package/2006/metadata/core-properties";
const NS_DC: &str = "http://purl.org/dc/elements/1.1/";
const NS_EXT: &str = "http://schemas.openxmlformats.org/officeDocument/2006/extended-properties";
const NS_VT: &str = "http://schemas.openxmlformats.org/officeDocument/2006/docPropsVTypes";
const NS_CUSTOM: &str = "http://schemas.openxmlformats.org/officeDocument/2006/custom-properties";

// ---------- 造包工具 ----------

fn zip_bytes(entries: &[(String, Vec<u8>)]) -> Vec<u8> {
    let cursor = Cursor::new(Vec::new());
    let mut writer = zip::ZipWriter::new(cursor);
    let options = zip::write::SimpleFileOptions::default().compression_method(zip::CompressionMethod::Deflated);
    for (name, bytes) in entries {
        writer.start_file(name.clone(), options).unwrap();
        writer.write_all(bytes).unwrap();
    }
    writer.finish().unwrap().into_inner()
}

fn package_rels_xml() -> String {
    format!(
        r#"<?xml version="1.0"?><Relationships xmlns="{RELS_NS}"><Relationship Id="rId1" Type="{OFFICE_DOC}" Target="word/document.xml"/></Relationships>"#
    )
}

/// `overrides`: `(partName, contentType)` 列表；`extras`: 额外 entry。
fn package_with(overrides: &[(&str, &str)], extras: &[(&str, &str)]) -> Vec<u8> {
    let mut types = format!(
        r#"<?xml version="1.0"?><Types xmlns="{TYPES_NAMESPACE_URI}"><Default Extension="rels" ContentType="{RELS_CT}"/><Default Extension="xml" ContentType="{XML_CT}"/><Override PartName="/word/document.xml" ContentType="{MAIN_CT}"/>"#
    );
    for (name, ct) in overrides {
        types.push_str(&format!(r#"<Override PartName="{name}" ContentType="{ct}"/>"#));
    }
    types.push_str("</Types>");
    let mut entries: Vec<(String, Vec<u8>)> = vec![
        ("[Content_Types].xml".to_string(), types.into_bytes()),
        ("_rels/.rels".to_string(), package_rels_xml().into_bytes()),
        ("word/document.xml".to_string(), b"<w:document/>".to_vec()),
    ];
    for (name, body) in extras {
        entries.push((name.to_string(), body.as_bytes().to_vec()));
    }
    zip_bytes(&entries)
}

fn core_xml(title: &str) -> String {
    format!(
        r#"<?xml version="1.0" encoding="UTF-8"?><cp:coreProperties xmlns:cp="{NS_CP}" xmlns:dc="{NS_DC}"><dc:title>{title}</dc:title><dc:creator>ioffice</dc:creator></cp:coreProperties>"#
    )
}

fn app_xml() -> String {
    format!(
        r#"<?xml version="1.0" encoding="UTF-8"?><Properties xmlns="{NS_EXT}"><Application>Microsoft Office Word</Application><Pages>1</Pages></Properties>"#
    )
}

fn custom_xml() -> String {
    format!(
        r#"<?xml version="1.0" encoding="UTF-8"?><Properties xmlns="{NS_CUSTOM}" xmlns:vt="{NS_VT}"><property fmtid="{{A}}" pid="2" name="K"><vt:lpwstr>6</vt:lpwstr></property></Properties>"#
    )
}

fn open(bytes: Vec<u8>) -> XWPFPackage {
    let mut input = Cursor::new(bytes);
    XWPFPackage::open_stream(&mut input)
}

fn fixture_path(name: &str) -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/wave0-ooxml-standards")
        .join(name)
}

fn sample2_path() -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../../../samples/docx-mvp-render/flutter/assets/sample2.docx")
}

fn open_file(path: std::path::PathBuf) -> XWPFPackage {
    open(std::fs::read(path).expect("语料必须在库"))
}

// ---------- DG1–DG2：真语料 ----------

/// DG1 / **C1**（core + app，无 custom）⇒ 两个视图有值；`customPackageProperties` 字段保持 `None`，
/// 但 `getCustomPackageProperties()` 仍返回一个**全无值 part**（不变量 4）。
#[test]
fn dg1_corpus_c1() {
    let mut pkg = open_file(fixture_path("standard-hello.docx"));
    pkg.getParts();
    let core = pkg.getPackageProperties();
    assert_eq!(
        core.getTitleProperty().getValue().map(|v| v.as_str()),
        Some("wave0 standard hello"),
        "C1 core 必须真解出来"
    );
    let ext = pkg.getExtendedPackageProperties();
    assert_eq!(ext.getApplicationProperty(), Some("Microsoft Office Word"));
    assert_eq!(ext.getPagesProperty(), Some(1));
    // custom：C1 无该 part ⇒ 字段未被赋值（懒加载 getter 造全无值 part）
    let custom = pkg.getCustomPackageProperties();
    assert!(!custom.hasProperty(), "C1 没有 docProps/custom.xml ⇒ 空表（不是有值）");
    assert!(custom.getPropertyArray().is_none());
}

/// DG2 / **`sample2`**（三份都有）⇒ 三视图都有值。
#[test]
fn dg2_corpus_sample2() {
    let mut pkg = open_file(sample2_path());
    pkg.getParts();
    // ★实测：sample2 的 core.xml **没有** `<dc:title>`，有值的是 `<dcterms:created>` / `<dcterms:modified>`
    // ⇒ 断言这两个（避免把「假设」写成判据）
    assert!(
        pkg.getPackageProperties().getCreatedProperty().hasValue(),
        "sample2 core 的 dcterms:created 必须有值"
    );
    assert!(
        pkg.getPackageProperties().getModifiedProperty().hasValue(),
        "sample2 core 的 dcterms:modified 必须有值"
    );
    assert!(!pkg.getPackageProperties().getTitleProperty().hasValue(), "sample2 core 无 dc:title（★实测）");
    let ext = pkg.getExtendedPackageProperties();
    assert_eq!(ext.getTemplateProperty(), Some("Normal.dotm"));
    assert_eq!(ext.getWordCountProperty(), Some(869));
    let custom = pkg.getCustomPackageProperties();
    assert!(custom.hasProperty(), "sample2 有 2 条自定义属性");
    assert_eq!(custom.getPropertyArray().as_ref().unwrap().len(), 2);
}

// ---------- DG3 / DG12：扫描顺序（不变量 2） ----------

/// DG3 / **N3 回归**：core-properties 的 `Override` 排在 app/custom **之前** ⇒ 三者都必须被挂上
/// （若扫描循环里是 `break` 而不是 `continue`，只会挂上 core）。
#[test]
fn dg3_core_first_no_truncation() {
    let bytes = package_with(
        &[
            ("/docProps/core.xml", CORE_CT),
            ("/docProps/app.xml", APP_CT),
            ("/docProps/custom.xml", CUSTOM_CT),
        ],
        &[
            ("docProps/core.xml", &core_xml("first-core")),
            ("docProps/app.xml", &app_xml()),
            ("docProps/custom.xml", &custom_xml()),
        ],
    );
    let mut pkg = open(bytes);
    pkg.getParts();
    assert_eq!(
        pkg.getPackageProperties().getTitleProperty().getValue().map(|v| v.as_str()),
        Some("first-core")
    );
    assert_eq!(
        pkg.getExtendedPackageProperties().getApplicationProperty(),
        Some("Microsoft Office Word"),
        "core 在前时 app 不得被截断（break ⇒ 此处为 None）"
    );
    assert!(pkg.getCustomPackageProperties().hasProperty(), "core 在前时 custom 不得被截断");
}

/// DG12 / **两个 core-properties part**：`extended`/`custom` 仍被赋值（`continue` 判据）；
/// ★「哪个 core 生效」在 Rust 侧因 `HashMap` 顺序**不判**（不变量 5，已登记）。
#[test]
fn dg12_two_core_parts() {
    let bytes = package_with(
        &[
            ("/docProps/core.xml", CORE_CT),
            ("/docProps/core2.xml", CORE_CT),
            ("/docProps/app.xml", APP_CT),
            ("/docProps/custom.xml", CUSTOM_CT),
        ],
        &[
            ("docProps/core.xml", &core_xml("core-one")),
            ("docProps/core2.xml", &core_xml("core-two")),
            ("docProps/app.xml", &app_xml()),
            ("docProps/custom.xml", &custom_xml()),
        ],
    );
    let mut pkg = open(bytes);
    pkg.getParts();
    let title = pkg
        .getPackageProperties()
        .getTitleProperty()
        .getValue()
        .map(|s| s.to_string());
    assert!(
        title.as_deref() == Some("core-one") || title.as_deref() == Some("core-two"),
        "两个 core-properties part 时取到其中之一即可（HashMap 顺序不判），实测 {title:?}"
    );
    assert_eq!(pkg.getExtendedPackageProperties().getPagesProperty(), Some(1), "不得因第二个 core 而截断");
    assert!(pkg.getCustomPackageProperties().hasProperty(), "不得因第二个 core 而截断");
}

// ---------- DG4–DG6 / DG11 / DG14：边界与失败 ----------

/// DG4 / **无任何 docProps** ⇒ 三个 getter 各返回一个全无值 part，且**不 panic**（不变量 4）。
#[test]
fn dg4_no_docprops_at_all() {
    let mut pkg = open(package_with(&[], &[]));
    pkg.getParts();
    let core = pkg.getPackageProperties();
    assert!(!core.getTitleProperty().hasValue(), "全无值 part ⇒ title 无值");
    let ext = pkg.getExtendedPackageProperties();
    assert_eq!(ext.getApplicationProperty(), None);
    let custom = pkg.getCustomPackageProperties();
    assert!(!custom.hasProperty());
}

/// DG5 / **只有 core** ⇒ core 有值，另两个无值。
#[test]
fn dg5_only_core() {
    let mut pkg = open(package_with(
        &[("/docProps/core.xml", CORE_CT)],
        &[("docProps/core.xml", &core_xml("only-core"))],
    ));
    pkg.getParts();
    assert_eq!(
        pkg.getPackageProperties().getTitleProperty().getValue().map(|v| v.as_str()),
        Some("only-core")
    );
    assert_eq!(pkg.getExtendedPackageProperties().getApplicationProperty(), None);
    assert!(!pkg.getCustomPackageProperties().hasProperty());
}

/// DG6 / **解析失败 ⇒ 视图不赋值**（Java `:212-213` 只记日志）：core.xml 是截断 XML
/// ⇒ `getParts` 不 panic、不 `Err`；`getPackageProperties()` 返回**全无值 part**（不是有值）。
#[test]
fn dg6_parse_failure_keeps_none() {
    let mut pkg = open(package_with(
        &[("/docProps/core.xml", CORE_CT), ("/docProps/app.xml", APP_CT)],
        &[
            ("docProps/core.xml", "<cp:coreProperties><dc:title>broken"),
            ("docProps/app.xml", &app_xml()),
        ],
    ));
    pkg.getParts();
    assert!(
        !pkg.getPackageProperties().getTitleProperty().hasValue(),
        "畸形 core.xml ⇒ 视图保持未赋值（空 part），不得 panic"
    );
    assert_eq!(
        pkg.getExtendedPackageProperties().getApplicationProperty(),
        Some("Microsoft Office Word"),
        "一个 part 解析失败不影响其他 part"
    );
}

/// DG11 / 未注册 content type 的 part 不影响三视图。
#[test]
fn dg11_other_parts_ignored() {
    let mut pkg = open(package_with(
        &[("/word/styles.xml", XML_CT)],
        &[("word/styles.xml", "<w:styles/>")],
    ));
    pkg.getParts();

    assert_eq!(pkg.getExtendedPackageProperties().getApplicationProperty(), None);
    assert!(!pkg.getCustomPackageProperties().hasProperty());
}

/// DG14 / 空 part 表（只有 `[Content_Types].xml` + 主文档）⇒ 三视图无值且 getter 不 panic。
#[test]
fn dg14_empty_package() {
    let mut pkg = open(package_with(&[], &[]));
    let parts = pkg.getParts();
    assert!(parts.len() >= 1, "至少有主文档 part");
    assert!(!pkg.getPackageProperties().getSubjectProperty().hasValue());
    assert_eq!(pkg.getExtendedPackageProperties().getWordCountProperty(), None);
    assert!(!pkg.getCustomPackageProperties().hasProperty());
}

// ---------- DG7–DG10 / DG13 / DG15：懒加载、缓存、注册表、不改 part 表、确定性 ----------

/// DG7 / 懒加载幂等：`dg4` 的包上连调两次返回**同一对象**，且第二次不重建。
#[test]
fn dg7_lazy_idempotent() {
    let mut pkg = open(package_with(&[], &[]));
    pkg.getParts();
    let p1 = pkg.getPackageProperties() as *const _;
    let p2 = pkg.getPackageProperties() as *const _;
    assert_eq!(p1, p2, "懒加载必须返回同一 part（第二次不得重建）");
    let e1 = pkg.getExtendedPackageProperties() as *const _;
    let e2 = pkg.getExtendedPackageProperties() as *const _;
    assert_eq!(e1, e2);
    let c1 = pkg.getCustomPackageProperties() as *const _;
    let c2 = pkg.getCustomPackageProperties() as *const _;
    assert_eq!(c1, c2);
}

/// DG8 / `getParts` 缓存：第二次调用不重扫（视图对象保持同一）。
#[test]
fn dg8_getparts_cached() {
    let mut pkg = open_file(fixture_path("standard-hello.docx"));
    let n1 = pkg.getParts().len();
    let core1 = pkg.getPackageProperties() as *const _;
    let n2 = pkg.getParts().len();
    let core2 = pkg.getPackageProperties() as *const _;
    assert_eq!(n1, n2, "两次 getParts 必须给出同样的 part 数");
    assert_eq!(core1, core2, "第二次 getParts 不得重建视图");
}

/// DG9 / `init` 注册表三项**有效**：三种 content type 的 part 各自被真解析
/// （⚠️ `partUnmarshallers` 是 `pub(crate)`，集成测试只断言**可观测效果**，登记 §1.7.1）。
#[test]
fn dg9_registry_contents() {
    let mut pkg = open(package_with(
        &[
            ("/docProps/core.xml", CORE_CT),
            ("/docProps/app.xml", APP_CT),
            ("/docProps/custom.xml", CUSTOM_CT),
        ],
        &[
            ("docProps/core.xml", &core_xml("registry")),
            ("docProps/app.xml", &app_xml()),
            ("docProps/custom.xml", &custom_xml()),
        ],
    ));
    pkg.getParts();
    assert_eq!(
        pkg.getPackageProperties().getTitleProperty().getValue().map(|v| v.as_str()),
        Some("registry"),
        "core-properties 已注册"
    );
    assert_eq!(pkg.getExtendedPackageProperties().getApplicationProperty(), Some("Microsoft Office Word"));
    assert!(pkg.getCustomPackageProperties().hasProperty());
}

/// DG10 / **不改 `partList` 成员集合**（不变量 7）。
#[test]
fn dg10_partlist_keys_unchanged() {
    let mut pkg = open_file(sample2_path());
    // `PackagePartCollection` 无 keySet 访问器 ⇒ 用「size + 代表键的存在性」等价断言成员集合不变
    // （登记 §1.7.1：Java 侧是 HashMap.keySet()）
    let probe: Vec<core_libs_io::io::opc::PackagePartName::PackagePartName> = [
        "/docProps/core.xml",
        "/docProps/app.xml",
        "/docProps/custom.xml",
        "/word/document.xml",
    ]
    .iter()
    .map(|n| PackagingURIHelper::createPartName_str(n).unwrap())
    .collect();
    let before: (usize, Vec<bool>) = {
        let list = pkg.getPartList().expect("partList 必须非空（前置）");
        (list.size(), probe.iter().map(|n| list.get(n).is_some()).collect())
    };
    let _ = pkg.getParts();
    let after: (usize, Vec<bool>) = {
        let list = pkg.getPartList().unwrap();
        (list.size(), probe.iter().map(|n| list.get(n).is_some()).collect())
    };
    assert!(before.0 > 0, "partList 必须非空（前置）");
    assert_eq!(before, after, "本 function 不得增删 part 表成员（size + 代表键）");
}

/// DG13 / 三个视图经 getter 与字段语义一致（字段是 `pub(crate)` ⇒ 集成测试用 getter + 空表判定）。
#[test]
fn dg13_fields_direct() {
    let mut pkg = open(package_with(
        &[("/docProps/core.xml", CORE_CT)],
        &[("docProps/core.xml", &core_xml("direct"))],
    ));
    pkg.getParts();
    // 有该 part ⇒ getter 返回真值；无该 part ⇒ getter 返回全无值 part（字段保持 None）
    assert_eq!(
        pkg.getPackageProperties().getTitleProperty().getValue().map(|v| v.as_str()),
        Some("direct")
    );
    assert_eq!(pkg.getExtendedPackageProperties().getApplicationProperty(), None);
    assert!(!pkg.getCustomPackageProperties().hasProperty());
    // 二次访问稳定（不重建 ⇒ 前面断言过的值仍在）
    assert_eq!(
        pkg.getPackageProperties().getTitleProperty().getValue().map(|v| v.as_str()),
        Some("direct")
    );
}

/// DG15 / **确定性**：同一份 C1（只有一个 core-properties part）两次独立开包，三视图取值完全相同。
#[test]
fn dg15_deterministic() {
    let bytes = std::fs::read(fixture_path("standard-hello.docx")).unwrap();
    let mut a = open(bytes.clone());
    a.getParts();
    let a_core = a.getPackageProperties().getTitleProperty().getValue().map(|s| s.to_string());
    let a_ext = a.getExtendedPackageProperties().getApplicationProperty().map(|s| s.to_string());
    let a_custom = a.getCustomPackageProperties().hasProperty();
    let mut b = open(bytes);
    b.getParts();
    let b_core = b.getPackageProperties().getTitleProperty().getValue().map(|s| s.to_string());
    let b_ext = b.getExtendedPackageProperties().getApplicationProperty().map(|s| s.to_string());
    let b_custom = b.getCustomPackageProperties().hasProperty();
    assert_eq!((a_core, a_ext, a_custom), (b_core, b_ext, b_custom));
}

/// §3.12 产品接线（与 `39`/`40`/`41` 联调）：`open_stream` → `getParts` → 三视图与 `39`/`40`/`41`
/// 的 unmarshaller 取值**逐字一致**（同一份 `sample2`）。
#[test]
fn wiring_three_views_from_real_corpus() {
    let mut pkg = open_file(sample2_path());
    pkg.getParts();
    // core（39 的判据值）★实测：sample2 有值的是 dcterms:created/modified（无 dc:title）
    assert!(pkg.getPackageProperties().getCreatedProperty().hasValue());
    assert!(pkg.getPackageProperties().getModifiedProperty().hasValue());
    // extended（40 的判据值）
    let ext = pkg.getExtendedPackageProperties();
    assert_eq!(ext.getTemplateProperty(), Some("Normal.dotm"));
    assert_eq!(ext.getPagesProperty(), Some(8));
    assert_eq!(ext.getNumberCharactersProperty(), Some(5814));
    assert_eq!(ext.getParagraphsProperty(), None, "丢值点在视图层同样为 null");
    // custom（41 的判据值）
    let custom = pkg.getCustomPackageProperties();
    assert_eq!(custom.getPropertyArray().as_ref().unwrap().len(), 2);
}
