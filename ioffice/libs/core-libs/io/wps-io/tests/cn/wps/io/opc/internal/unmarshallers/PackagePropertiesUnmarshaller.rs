// function_id=39_function_docprops_core
// cargo test --manifest-path libs/core-libs/io/wps-io/Cargo.toml --test 39_function_docprops_core
// src:  libs/core-libs/io/wps-io/src/cn/wps/io/opc/internal/unmarshallers/PackagePropertiesUnmarshaller.rs
//       （通路依赖：dom/io/SAXReader.rs、dom/io/SAXContentHandler.rs、dom/tree/DefaultElement.rs、dom/tree/DefaultText.rs、
//         dom/tree/FlyweightText.rs、dom/tree/FlyweightAttribute.rs、dom/tree/DefaultAttribute.rs、dom/tree/DefaultDocument.rs）
// test: libs/core-libs/io/wps-io/tests/cn/wps/io/opc/internal/unmarshallers/PackagePropertiesUnmarshaller.rs
//
// 覆盖矩阵 §3.2 的 DC1–DC15：`docProps/core.xml` 字节流 → 16 项类型化属性（Java `PackagePropertiesUnmarshaller.java:53-112`）。

use core_libs_io::io::opc::internal::unmarshallers::PackagePropertiesUnmarshaller::PackagePropertiesUnmarshaller;
use core_libs_io::io::opc::internal::unmarshallers::UnmarshallContext::UnmarshallContext;
use core_libs_io::io::opc::internal::PackagePropertiesPart::PackagePropertiesPart;
use core_libs_io::io::opc::PackagePartName::PackagePartName;
use core_libs_io::io::opc::PackagingURIHelper::PackagingURIHelper;

const CP: &str = "http://schemas.openxmlformats.org/package/2006/metadata/core-properties";
const DC: &str = "http://purl.org/dc/elements/1.1/";
const DCTERMS: &str = "http://purl.org/dc/terms/";
const XSI: &str = "http://www.w3.org/2001/XMLSchema-instance";

fn part_name(s: &str) -> PackagePartName {
    PackagingURIHelper::createPartName_str(s).unwrap()
}

fn context(part: &str) -> UnmarshallContext {
    UnmarshallContext::new(None, part_name(part))
}

fn context_without_part_name() -> UnmarshallContext {
    // Java: `new UnmarshallContext(pkg, (PackagePartName)null)` 形态（`PackagePropertiesUnmarshaller:54` 之前的 NPE 场景）
    UnmarshallContext::new_with_null_part_name(None)
}

fn unmarshall(xml: &str) -> Result<PackagePropertiesPart, String> {
    let ctx = context("/docProps/core.xml");
    let mut input = std::io::Cursor::new(xml.as_bytes().to_vec());
    PackagePropertiesUnmarshaller::new().unmarshall(&ctx, Some(&mut input))
}

fn all_sixteen() -> String {
    format!(
        r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<cp:coreProperties xmlns:cp="{CP}" xmlns:dc="{DC}" xmlns:dcterms="{DCTERMS}" xmlns:xsi="{XSI}">
  <dc:title>题目</dc:title>
  <dc:subject>主题</dc:subject>
  <dc:creator>作者</dc:creator>
  <cp:keywords>k1,k2</cp:keywords>
  <dc:description>描述</dc:description>
  <cp:lastModifiedBy>最后修改者</cp:lastModifiedBy>
  <cp:revision>3</cp:revision>
  <dcterms:created xsi:type="dcterms:W3CDTF">2026-08-18T00:00:00Z</dcterms:created>
  <dcterms:modified xsi:type="dcterms:W3CDTF">2026-08-19T12:34:56Z</dcterms:modified>
  <cp:category>分类</cp:category>
  <cp:contentStatus>草稿</cp:contentStatus>
  <cp:contentType>type/a</cp:contentType>
  <dc:identifier>ID-1</dc:identifier>
  <dc:language>zh-CN</dc:language>
  <cp:lastPrinted>2026-08-01T00:00:00Z</cp:lastPrinted>
  <cp:version>1.0</cp:version>
</cp:coreProperties>"#
    )
}

fn value_of(p: &PackagePropertiesPart, which: &str) -> Option<String> {
    let v = match which {
        "title" => p.getTitleProperty(),
        "subject" => p.getSubjectProperty(),
        "creator" => p.getCreatorProperty(),
        "keywords" => p.getKeywordsProperty(),
        "description" => p.getDescriptionProperty(),
        "lastModifiedBy" => p.getLastModifiedByProperty(),
        "revision" => p.getRevisionProperty(),
        "created" => p.getCreatedProperty(),
        "modified" => p.getModifiedProperty(),
        "category" => p.getCategoryProperty(),
        "contentStatus" => p.getContentStatusProperty(),
        "contentType" => p.getContentTypeProperty(),
        "identifier" => p.getIdentifierProperty(),
        "language" => p.getLanguageProperty(),
        "lastPrinted" => p.getLastPrintedProperty(),
        "version" => p.getVersionProperty(),
        other => panic!("未知属性 {other}"),
    };
    v.getValue().cloned()
}

const SIXTEEN: [&str; 16] = [
    "title", "subject", "creator", "keywords", "description", "lastModifiedBy", "revision", "created",
    "modified", "category", "contentStatus", "contentType", "identifier", "language", "lastPrinted", "version",
];

/// DC1 / 主路径：16 个键全有值。
#[test]
fn dc1_all_sixteen_present() {
    let p = unmarshall(&all_sixteen()).expect("16 项全有值的 core.xml 必须解析成功");
    assert_eq!(value_of(&p, "title").as_deref(), Some("题目"));
    assert_eq!(value_of(&p, "subject").as_deref(), Some("主题"));
    assert_eq!(value_of(&p, "creator").as_deref(), Some("作者"));
    assert_eq!(value_of(&p, "keywords").as_deref(), Some("k1,k2"));
    assert_eq!(value_of(&p, "description").as_deref(), Some("描述"));
    assert_eq!(value_of(&p, "lastModifiedBy").as_deref(), Some("最后修改者"));
    assert_eq!(value_of(&p, "revision").as_deref(), Some("3"));
    assert_eq!(value_of(&p, "created").as_deref(), Some("2026-08-18T00:00:00Z"));
    assert_eq!(value_of(&p, "modified").as_deref(), Some("2026-08-19T12:34:56Z"));
    assert_eq!(value_of(&p, "category").as_deref(), Some("分类"));
    assert_eq!(value_of(&p, "contentStatus").as_deref(), Some("草稿"));
    assert_eq!(value_of(&p, "contentType").as_deref(), Some("type/a"));
    assert_eq!(value_of(&p, "identifier").as_deref(), Some("ID-1"));
    assert_eq!(value_of(&p, "language").as_deref(), Some("zh-CN"));
    assert_eq!(value_of(&p, "lastPrinted").as_deref(), Some("2026-08-01T00:00:00Z"));
    assert_eq!(value_of(&p, "version").as_deref(), Some("1.0"));
}

/// DC2 / 主语料（真语料）：`tests/fixtures/wave0-ooxml-standards/standard-hello.docx` 的 `docProps/core.xml`
/// 走同一入口，取值与语料逐字对比（语料 sha256 由测试自身计算，见 §3.11）。
#[test]
fn dc2_corpus_c1() {
    let bytes = std::fs::read(fixture_path()).expect("wave0 标准语料 standard-hello.docx 必须在库");
    let mut archive = zip::ZipArchive::new(std::io::Cursor::new(bytes.clone())).unwrap();
    let mut entry = archive.by_name("docProps/core.xml").expect("语料应含 docProps/core.xml");
    let mut xml = String::new();
    std::io::Read::read_to_string(&mut entry, &mut xml).unwrap();

    let p = unmarshall(&xml).expect("语料 core.xml 必须解析成功");
    assert_eq!(value_of(&p, "title").as_deref(), Some("wave0 standard hello"));
    assert_eq!(value_of(&p, "creator").as_deref(), Some("ioffice wave0 e2e"));
    assert_eq!(value_of(&p, "lastModifiedBy").as_deref(), Some("ioffice wave0 e2e"));
    assert_eq!(value_of(&p, "created").as_deref(), Some("2026-08-18T00:00:00Z"));
    assert_eq!(value_of(&p, "modified").as_deref(), Some("2026-08-18T00:00:00Z"));
    assert_eq!(value_of(&p, "subject"), None, "语料未写 subject ⇒ null");
    assert_eq!(value_of(&p, "keywords"), None);
}

/// DC2b / 合成 zip 容器：`docProps/core.xml` 条目（Deflate 压缩）走同一入口。
#[test]
fn dc2b_from_real_zip_entry() {
    let mut zip_bytes = Vec::new();
    {
        let mut writer = zip::ZipWriter::new(std::io::Cursor::new(&mut zip_bytes));
        let options: zip::write::FileOptions<'_, ()> =
            zip::write::FileOptions::default().compression_method(zip::CompressionMethod::Deflated);
        writer.start_file("docProps/core.xml", options).unwrap();
        std::io::Write::write_all(&mut writer, all_sixteen().as_bytes()).unwrap();
        writer.finish().unwrap();
    }
    let mut archive = zip::ZipArchive::new(std::io::Cursor::new(zip_bytes)).unwrap();
    let mut entry = archive.by_name("docProps/core.xml").expect("zip 内应有 docProps/core.xml");
    let mut xml = String::new();
    std::io::Read::read_to_string(&mut entry, &mut xml).unwrap();

    let p = unmarshall(&xml).expect("zip 条目的 core.xml 必须解析成功");
    for key in SIXTEEN {
        assert!(value_of(&p, key).is_some(), "{key} 在 zip 通路下也应有值");
    }
    assert_eq!(value_of(&p, "title").as_deref(), Some("题目"));
    assert_eq!(value_of(&p, "modified").as_deref(), Some("2026-08-19T12:34:56Z"));
}

/// §3.12 产品接线（wiring）：`XWPFPackage::open_stream` → `getParts()` → `getPackageProperties()`
/// 必须是**真解出来的值**，而不是 `42` 之前那种空壳（PLAN.md §0.3「空 vs 有值」）。
#[test]
fn wiring_xwpfpackage_getparts_core_properties() {
    let bytes = std::fs::read(fixture_path()).expect("语料必须在库");
    let mut input = std::io::Cursor::new(bytes);
    let mut package = core_libs_io::io::xwpf::XWPFPackage::XWPFPackage::open_stream(&mut input);
    let _parts = package.getParts();
    // Java: XWPFPackage.getParts → partUnmarshaller.unmarshall(context, part.getInputStream())
    //       ⇒ this.packageProperties
    let props = package.getPackageProperties();
    assert_eq!(
        props.getTitleProperty().getValue().map(|v| v.as_str()),
        Some("wave0 standard hello"),
        "产品入口必须返回真实解出的 core.xml 值（空壳 ⇒ 此处为 None）"
    );
    assert_eq!(
        props.getCreatorProperty().getValue().map(|v| v.as_str()),
        Some("ioffice wave0 e2e")
    );
    assert_eq!(
        props.getLastModifiedByProperty().getValue().map(|v| v.as_str()),
        Some("ioffice wave0 e2e")
    );
    assert_eq!(
        props.getCreatedProperty().getValue().map(|v| v.as_str()),
        Some("2026-08-18T00:00:00Z")
    );
    assert_eq!(
        props.getModifiedProperty().getValue().map(|v| v.as_str()),
        Some("2026-08-18T00:00:00Z")
    );
    assert_eq!(props.getKeywordsProperty().getValue(), None, "语料未写 ⇒ null（不是空串）");
}

fn fixture_path() -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/wave0-ooxml-standards/standard-hello.docx")
}

/// DC3 / 边界：16 个键全缺失（空 `cp:coreProperties`）⇒ 全 `null`。
#[test]
fn dc3_all_absent_all_null() {
    let xml = format!(r#"<?xml version="1.0"?><cp:coreProperties xmlns:cp="{CP}"/>"#);
    let p = unmarshall(&xml).expect("空 coreProperties 必须解析成功");
    for key in SIXTEEN {
        assert_eq!(value_of(&p, key), None, "{key} 应为 null");
    }
}

/// DC4 / 边界：只有 `title`。
#[test]
fn dc4_partial_absent() {
    let xml = format!(
        r#"<?xml version="1.0"?><cp:coreProperties xmlns:cp="{CP}" xmlns:dc="{DC}"><dc:title>only</dc:title></cp:coreProperties>"#
    );
    let p = unmarshall(&xml).unwrap();
    assert_eq!(value_of(&p, "title").as_deref(), Some("only"));
    for key in SIXTEEN.iter().filter(|k| **k != "title") {
        assert_eq!(value_of(&p, key), None, "{key} 应为 null");
    }
}

/// DC5 / 边界：元素存在但空文本 ⇒ `null` 而非 `""`。
#[test]
fn dc5_empty_text_is_null() {
    let xml = format!(
        r#"<?xml version="1.0"?><cp:coreProperties xmlns:cp="{CP}" xmlns:dc="{DC}"><dc:title></dc:title><dc:creator/></cp:coreProperties>"#
    );
    let p = unmarshall(&xml).unwrap();
    assert_eq!(value_of(&p, "title"), None, "空文本必须为 null（Java `setStringValue` 语义）");
    assert_eq!(value_of(&p, "creator"), None);
}

/// DC6 / 主路径：日期三项合法值往返不变（不做时区改写）。
#[test]
fn dc6_dates_roundtrip() {
    let p = unmarshall(&all_sixteen()).unwrap();
    assert_eq!(value_of(&p, "created").as_deref(), Some("2026-08-18T00:00:00Z"));
    assert_eq!(value_of(&p, "modified").as_deref(), Some("2026-08-19T12:34:56Z"));
    assert_eq!(value_of(&p, "lastPrinted").as_deref(), Some("2026-08-01T00:00:00Z"));
}

/// DC7 / 边界（对齐 Java 真实语义）：宽松解析 ⇒ 前缀可解析的日期原样保留；完全不可解析 ⇒
/// Java `setDateValue` 抛 `InvalidFormatException`，被 `setCreatedProperty(String)` 的空 catch 吞掉 ⇒ 属性保持 `null`。
#[test]
fn dc7_date_malformed() {
    let parse_created = |raw: &str| -> Option<String> {
        let xml = format!(
            r#"<?xml version="1.0"?><cp:coreProperties xmlns:cp="{CP}" xmlns:dcterms="{DCTERMS}"><dcterms:created>{raw}</dcterms:created></cp:coreProperties>"#
        );
        value_of(&unmarshall(&xml).unwrap(), "created")
    };
    // Java `SimpleDateFormat("yyyy-MM-dd'T'HH:mm:ss'Z'")` + `ParsePosition(0)` 宽松解析：
    // 秒之后字面量 `'Z'` 不匹配即停止，但已解析出的日期有效 ⇒ 返回非 null ⇒ 属性被赋值
    for raw in [
        "2026-08-18T00:00:00Z",
        "2026-08-18T00:00:00.123Z",
        "2026-08-18T00:00:00+08:00",
        "2026-08-18",
    ] {
        assert_eq!(parse_created(raw).as_deref(), Some(raw), "{raw} 可解析 ⇒ 原样保留");
    }
    // 连年份都解析不出来 ⇒ `df.parse` 返回 null ⇒ InvalidFormatException ⇒ 空 catch ⇒ 属性仍为 null
    for raw in ["not-a-date", "T00:00:00Z"] {
        assert_eq!(parse_created(raw), None, "{raw} 不可解析 ⇒ 属性保持 null（Java 空 catch 语义）");
    }
}

/// DC8 / 失败：非 well-formed（截断标签）⇒ `Err`。
#[test]
fn dc8_malformed_xml_err() {
    let xml = format!(r#"<?xml version="1.0"?><cp:coreProperties xmlns:cp="{CP}"><dc:title>x"#);
    let mut input = std::io::Cursor::new(xml.as_bytes().to_vec());
    let got = PackagePropertiesUnmarshaller::new()
        .unmarshall(&context("/docProps/core.xml"), Some(&mut input));
    assert!(got.is_err(), "截断 XML 必须返回 Err（Java: DocumentException ⇒ IOException）");
}

/// DC9 / 失败：空输入（0 字节）⇒ `Err`。
#[test]
fn dc9_empty_input_err() {
    let mut input = std::io::Cursor::new(Vec::new());
    let got = PackagePropertiesUnmarshaller::new()
        .unmarshall(&context("/docProps/core.xml"), Some(&mut input));
    assert!(got.is_err(), "0 字节输入必须返回 Err");
}

/// DC10 / 失败（Java 不可达分支）：`input = None` ⇒ 不 panic、返回可解释错误。
#[test]
fn dc10_input_none_no_panic() {
    let got = PackagePropertiesUnmarshaller::new().unmarshall(&context("/docProps/core.xml"), None);
    assert!(got.is_err(), "input=None 且 context 无 zipEntry ⇒ Err（Java 该分支不可达，Rust 显式返回错误）");
}

/// DC11 / 失败：`context.getPartName() == None` ⇒ `Err("partName")`。
#[test]
fn dc11_missing_part_name_err() {
    let ctx = context_without_part_name();
    let mut input = std::io::Cursor::new(all_sixteen().into_bytes());
    let err = match PackagePropertiesUnmarshaller::new().unmarshall(&ctx, Some(&mut input)) {
        Ok(_) => panic!("partName 为 null 必须返回 Err"),
        Err(e) => e,
    };
    assert_eq!(err, "partName");
}

/// DC12 / 幂等/确定性：同一字节流两次 ⇒ 16 项取值完全相同。
#[test]
fn dc12_deterministic() {
    let a = unmarshall(&all_sixteen()).unwrap();
    let b = unmarshall(&all_sixteen()).unwrap();
    for key in SIXTEEN {
        assert_eq!(value_of(&a, key), value_of(&b, key), "{key} 两次解析必须一致");
    }
}

/// DC13 / 无副作用：调用前后 `context` 的 partName 不变。
#[test]
fn dc13_no_side_effect() {
    let ctx = context("/docProps/core.xml");
    let before = ctx.getPartName().as_ref().map(|n| n.getName().to_string());
    let mut input = std::io::Cursor::new(all_sixteen().into_bytes());
    let _ = PackagePropertiesUnmarshaller::new().unmarshall(&ctx, Some(&mut input));
    let after = ctx.getPartName().as_ref().map(|n| n.getName().to_string());
    assert_eq!(after, before, "context 不得被 unmarshall 改写");
}

/// DC14 / 命名空间矩阵：`<dc:title>` 命中；**无前缀** `<title>` 不命中；错误命名空间 `<x:title>` 不命中。
#[test]
fn dc14_namespace_matrix() {
    let hit = format!(
        r#"<?xml version="1.0"?><cp:coreProperties xmlns:cp="{CP}" xmlns:dc="{DC}"><dc:title>T</dc:title></cp:coreProperties>"#
    );
    assert_eq!(value_of(&unmarshall(&hit).unwrap(), "title").as_deref(), Some("T"));

    let no_prefix = format!(
        r#"<?xml version="1.0"?><cp:coreProperties xmlns:cp="{CP}"><title>T</title></cp:coreProperties>"#
    );
    assert_eq!(value_of(&unmarshall(&no_prefix).unwrap(), "title"), None, "无命名空间的 title 不命中 DC");

    let wrong_ns = format!(
        r#"<?xml version="1.0"?><cp:coreProperties xmlns:cp="{CP}" xmlns:x="urn:wrong"><x:title>T</x:title></cp:coreProperties>"#
    );
    assert_eq!(value_of(&unmarshall(&wrong_ns).unwrap(), "title"), None, "错误命名空间不命中");

    let default_ns = format!(
        r#"<?xml version="1.0"?><cp:coreProperties xmlns:cp="{CP}" xmlns="{DC}"><title>T</title></cp:coreProperties>"#
    );
    assert_eq!(
        value_of(&unmarshall(&default_ns).unwrap(), "title").as_deref(),
        Some("T"),
        "默认命名空间（xmlns=）应命中 DC"
    );
}

/// DC15 / trait impl 路径（`.map(|p| p.base)`）可调用且不 panic。
#[test]
fn dc15_trait_impl_smoke() {
    // trait impl 路径（`.map(|p| p.base)`）由 `unmarshall` 返回的 part 承载；`base` 非公开 ⇒ 以公开 getter 断言其可见行为。
    let p = unmarshall(&all_sixteen()).unwrap();
    assert_eq!(p.getTitleProperty().getValue().cloned().as_deref(), Some("题目"));
    assert!(p.getTitleProperty().hasValue());
}
