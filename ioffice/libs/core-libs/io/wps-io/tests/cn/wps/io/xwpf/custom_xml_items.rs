// function_id=43_function_docprops_custom_xml
// cargo test --manifest-path libs/core-libs/io/wps-io/Cargo.toml --test 43_function_docprops_custom_xml
// src:  libs/core-libs/io/wps-io/src/cn/wps/io/xwpf/XWPFPackage.rs
// test: libs/core-libs/io/wps-io/tests/cn/wps/io/xwpf/custom_xml_items.rs
//
// 覆盖矩阵 §3.2 的 DH1–DH19：`customXml/item*.xml` + 其关系 → `CustomXmlInfo[(itemPath, itemPropPath)]`
// （Java `XWPFPackage.java:186-188` / `:230-256`；Rust `XWPFPackage::collectCustomXmlInfo`）。

use core_libs_io::io::opc::internal::ContentTypeManager::TYPES_NAMESPACE_URI;
use core_libs_io::io::opc::PackageRelationshipTypes::PackageRelationshipTypes;
use core_libs_io::io::xwpf::XWPFPackage::XWPFPackage;
use std::io::{Cursor, Write};

const MAIN_CT: &str = "application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml";
const RELS_CT: &str = "application/vnd.openxmlformats-package.relationships+xml";
const XML_CT: &str = "application/xml";
const PROPS_CT: &str = "application/vnd.openxmlformats-officedocument.customXmlProperties+xml";
const RELS_NS: &str = "http://schemas.openxmlformats.org/package/2006/relationships";
const OFFICE_DOC: &str = PackageRelationshipTypes::CORE_DOCUMENT;
const REL_CUSTOM_XML_PROPS: &str =
    "http://schemas.openxmlformats.org/officeDocument/2006/relationships/customXmlProps";
const REL_IMAGE: &str = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/image";

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

/// `overrides`: 额外 `<Override>`；`entries`: 额外 zip entry（`[Content_Types].xml` 里
/// `xml → application/xml` 是 Default ⇒ `customXml/item*.xml`、`itemProps*.xml` 都命中）
fn package_with(overrides: &[(&str, &str)], entries: &[(&str, &str)]) -> Vec<u8> {
    let mut types = format!(
        r#"<?xml version="1.0"?><Types xmlns="{TYPES_NAMESPACE_URI}"><Default Extension="rels" ContentType="{RELS_CT}"/><Default Extension="xml" ContentType="{XML_CT}"/><Override PartName="/word/document.xml" ContentType="{MAIN_CT}"/>"#
    );
    for (name, ct) in overrides {
        types.push_str(&format!(r#"<Override PartName="{name}" ContentType="{ct}"/>"#));
    }
    types.push_str("</Types>");
    let rels = format!(
        r#"<?xml version="1.0"?><Relationships xmlns="{RELS_NS}"><Relationship Id="rId1" Type="{OFFICE_DOC}" Target="word/document.xml"/></Relationships>"#
    );
    let mut all: Vec<(String, Vec<u8>)> = vec![
        ("[Content_Types].xml".to_string(), types.into_bytes()),
        ("_rels/.rels".to_string(), rels.into_bytes()),
        ("word/document.xml".to_string(), b"<w:document/>".to_vec()),
    ];
    for (name, body) in entries {
        all.push((name.to_string(), body.as_bytes().to_vec()));
    }
    zip_bytes(&all)
}

/// 一个 item part 的标准搭配：`customXml/item{N}.xml` + 其 `.rels` + `customXml/itemProps{N}.xml`。
/// ★**实测**：`item{N}.xml` 靠 Default `xml → application/xml`；但 `itemProps{N}.xml` **必须有 Override**
/// （`customXmlProperties+xml`），否则它只是 `application/xml` ⇒ `itemPropPath` 为空（Java 同）。
fn item_entries(n: usize, item_body: &str, props_body: &str) -> Vec<(String, String)> {
    vec![
        (format!("customXml/item{n}.xml"), item_body.to_string()),
        (format!("customXml/itemProps{n}.xml"), props_body.to_string()),
        (
            format!("customXml/_rels/item{n}.xml.rels"),
            format!(
                r#"<?xml version="1.0"?><Relationships xmlns="{RELS_NS}"><Relationship Id="rId1" Type="{REL_CUSTOM_XML_PROPS}" Target="itemProps{n}.xml"/></Relationships>"#
            ),
        ),
    ]
}

fn open(bytes: Vec<u8>) -> XWPFPackage {
    let mut input = Cursor::new(bytes);
    XWPFPackage::open_stream(&mut input)
}

fn corpus(path: &str) -> XWPFPackage {
    open(std::fs::read(path).expect("语料必须在库"))
}

fn fixture(name: &str) -> String {
    format!("{}/tests/fixtures/wave0-ooxml-standards/{name}", env!("CARGO_MANIFEST_DIR"))
}

fn sample2() -> String {
    format!("{}/../../../../samples/docx-mvp-render/flutter/assets/sample2.docx", env!("CARGO_MANIFEST_DIR"))
}

fn bold() -> String {
    format!(
        "{}/../../../word-core/tests/fixtures/03_function_line_corpus/docx/bold.docx",
        env!("CARGO_MANIFEST_DIR")
    )
}

/// 语料里某个 entry 的字节（用于比对临时文件内容）
fn entry_bytes(path: &str, name: &str) -> Vec<u8> {
    let file = std::fs::File::open(path).unwrap();
    let mut zip = zip::ZipArchive::new(file).unwrap();
    let mut entry = zip.by_name(name).unwrap();
    let mut bytes = Vec::new();
    std::io::Read::read_to_end(&mut entry, &mut bytes).unwrap();
    bytes
}

// ---------- DH1–DH3：真语料 ----------

/// DH1 / `sample2` ⇒ **恰好 1 条**，`itemPath`/`itemPropPath` 都非空且指向**真实存在**的临时文件。
#[test]
fn dh1_corpus_sample2() {
    let path = sample2();
    let mut pkg = corpus(&path);
    let _ = pkg.getParts();
    let infos = pkg.getCustomXmlInfos();
    assert_eq!(infos.len(), 1, "sample2 有 1 个 customXml item");
    let info = &infos[0];
    assert!(!info.mPathItem.is_empty(), "itemPath 必须非空");
    assert!(!info.mPathItemProp.is_empty(), "itemPropPath 必须非空（item1.xml.rels 指向 itemProps1.xml）");
    // 临时文件真实存在且内容 == item part 内容（DH13 同源断言）
    assert_eq!(
        std::fs::read(&info.mPathItem).expect("item 临时文件必须存在"),
        entry_bytes(&path, "customXml/item1.xml"),
        "临时文件内容必须等于 customXml/item1.xml 的字节"
    );
    assert_eq!(
        std::fs::read(&info.mPathItemProp).expect("itemProps 临时文件必须存在"),
        entry_bytes(&path, "customXml/itemProps1.xml"),
        "itemPropPath 指向的临时文件内容必须等于 itemProps1.xml 的字节"
    );
}

/// DH2 / 第二份真语料 `bold.docx` ⇒ 1 条（走 `Default xml → application/xml`，不是 `Override`）。
#[test]
fn dh2_corpus_bold() {
    let path = bold();
    let mut pkg = corpus(&path);
    let _ = pkg.getParts();
    let infos = pkg.getCustomXmlInfos();
    assert_eq!(infos.len(), 1);
    assert!(!infos[0].mPathItem.is_empty());
    assert!(!infos[0].mPathItemProp.is_empty(), "bold.docx 的 item1.xml.rels 也指向 itemProps1.xml");
}

/// DH3 / **C1 负向**：无 `customXml/*` ⇒ 空数组（不是缺字段）。
#[test]
fn dh3_corpus_c1_empty() {
    let mut pkg = corpus(&fixture("standard-hello.docx"));
    let _ = pkg.getParts();
    assert!(pkg.getCustomXmlInfos().is_empty(), "C1 无 customXml item ⇒ 空数组");
}

/// DH4 / **`collectCustomXmlInfo` 真的被调用**（不是内联循环）：源码审计（读本 crate 的 `XWPFPackage.rs`）。
#[test]
fn dh4_collect_is_called() {
    let src = std::fs::read_to_string(format!("{}/src/cn/wps/io/xwpf/XWPFPackage.rs", env!("CARGO_MANIFEST_DIR")))
        .expect("src 必须可读");
    let call_sites: Vec<&str> = src
        .lines()
        .map(|l| l.trim())
        .filter(|l| l.starts_with("self.collectCustomXmlInfo("))
        .collect();
    assert!(
        !call_sites.is_empty(),
        "必须有 ≥1 个活调用点（Java `:187`）★当前为 0 就是没接上"
    );
    // 「只剩一份活逻辑」：内联循环里不得再有 `CustomXmlInfo::new(`
    assert_eq!(
        src.matches("CustomXmlInfo::new(").count(),
        1,
        "`CustomXmlInfo::new` 只应出现在 `collectCustomXmlInfo` 里（内联重复逻辑必须已删）"
    );
}

// ---------- DH5–DH10：造包边界 ----------

/// DH5 / item **无关系**（无 `.rels`）⇒ 仍 1 条，`itemPropPath` 为空串（Java `null` 的 Rust 表达）。
#[test]
fn dh5_no_relationship() {
    let bytes = package_with(
        &[],
        &[("customXml/item1.xml", "<root>1</root>")],
    );
    let mut pkg = open(bytes);
    let _ = pkg.getParts();
    let infos = pkg.getCustomXmlInfos();
    assert_eq!(infos.len(), 1);
    assert!(!infos[0].mPathItem.is_empty());
    assert_eq!(infos[0].mPathItemProp, "", "无关系 ⇒ itemPropPath 为空串");
}

/// DH6 / 关系 0 的 **Type 不是 customXmlProps**（目标 part 的 content type 不匹配）⇒ 空串。
#[test]
fn dh6_wrong_rel_type() {
    let bytes = package_with(
        &[],
        &[
            ("customXml/item1.xml", "<root>1</root>"),
            ("customXml/other.xml", "<other/>"),
            (
                "customXml/_rels/item1.xml.rels",
                &format!(
                    r#"<?xml version="1.0"?><Relationships xmlns="{RELS_NS}"><Relationship Id="rId1" Type="{REL_IMAGE}" Target="other.xml"/></Relationships>"#
                ),
            ),
        ],
    );
    let mut pkg = open(bytes);
    let _ = pkg.getParts();
    let infos = pkg.getCustomXmlInfos();
    assert_eq!(infos.len(), 1);
    assert_eq!(infos[0].mPathItemProp, "", "目标 content type ≠ customXmlProps ⇒ 空串");
}

/// DH7 / 关系 0 的 **Target 为空/畸形** ⇒ 空串、**不 panic**。
#[test]
fn dh7_null_target_uri() {
    let bytes = package_with(
        &[],
        &[
            ("customXml/item1.xml", "<root>1</root>"),
            (
                "customXml/_rels/item1.xml.rels",
                &format!(
                    r#"<?xml version="1.0"?><Relationships xmlns="{RELS_NS}"><Relationship Id="rId1" Type="{REL_CUSTOM_XML_PROPS}" Target=""/></Relationships>"#
                ),
            ),
        ],
    );
    let mut pkg = open(bytes);
    let _ = pkg.getParts();
    let infos = pkg.getCustomXmlInfos();
    assert_eq!(infos.len(), 1);
    assert_eq!(infos[0].mPathItemProp, "", "Target 为空 ⇒ itemPropPath 空串，且不 panic");
}

/// DH8 / **悬空关系**（指向 part 表里不存在的 part）⇒ 空串、不 panic。
#[test]
fn dh8_dangling_relationship() {
    let bytes = package_with(
        &[],
        &[
            ("customXml/item1.xml", "<root>1</root>"),
            (
                "customXml/_rels/item1.xml.rels",
                &format!(
                    r#"<?xml version="1.0"?><Relationships xmlns="{RELS_NS}"><Relationship Id="rId1" Type="{REL_CUSTOM_XML_PROPS}" Target="missing.xml"/></Relationships>"#
                ),
            ),
        ],
    );
    let mut pkg = open(bytes);
    let _ = pkg.getParts();
    let infos = pkg.getCustomXmlInfos();
    assert_eq!(infos.len(), 1);
    assert_eq!(infos[0].mPathItemProp, "", "悬空目标 ⇒ 空串，且不 panic");
}

/// DH9 / `dumpPackagePart` 失败 ⇒ **不 push**（Java `:233-235`）。
/// ★**实测边界**：集成测试无法让 `Platform::createTempFile`/zip 读流失败 ⇒ 用「item part 不在 part 表」
/// 这一等价失败路径断言「只记名、不产生条目、不 panic」（登记 §1.7.1 ④）。
#[test]
fn dh9_dump_fail_no_push() {
    // content type 命中但 zip 里没有该 entry ⇒ part 表里也不会有它 ⇒ 不产生条目
    let bytes = package_with(&[("/customXml/item1.xml", XML_CT)], &[]);
    let mut pkg = open(bytes);
    let _ = pkg.getParts();
    assert!(pkg.getCustomXmlInfos().is_empty(), "取不到 part ⇒ 不 push，且不 panic");
}

/// DH10 / **异常被吞**（Java `:252-254`）：第 1 个 item 的 `.rels` 是畸形 ⇒ 该条**不 push**、
/// 第 2 个 item **仍被处理**（不变量 5）。
#[test]
fn dh10_exception_swallowed() {
    let mut entries = vec![
        ("customXml/item1.xml".to_string(), "<root>1</root>".to_string()),
        // 畸形 .rels：`Relationship` 缺 `Id` ⇒ 解析失败（Java 抛 InvalidFormatException，被 catch (Throwable) 吞掉）
        (
            "customXml/_rels/item1.xml.rels".to_string(),
            format!(r#"<?xml version="1.0"?><Relationships xmlns="{RELS_NS}"><Relationship Type="{REL_CUSTOM_XML_PROPS}" Target="itemProps1.xml"/></Relationships>"#),
        ),
    ];
    entries.extend(item_entries(2, "<root>2</root>", "<props>2</props>"));
    let borrowed: Vec<(&str, &str)> = entries.iter().map(|(a, b)| (a.as_str(), b.as_str())).collect();
    let mut pkg = open(package_with(&[("/customXml/itemProps2.xml", PROPS_CT)], &borrowed));
    let _ = pkg.getParts();
    let infos = pkg.getCustomXmlInfos();
    // 本包的 `itemProps2.xml` 带 Override（`customXmlProperties+xml`）⇒ 不命中 item 判定（DH14 里那种
    // 用 Default `application/xml` 的 itemProps 才会额外命中）⇒ 候选只有 item1（畸形 rels ⇒ 不 push）与 item2
    let contents: Vec<Vec<u8>> = infos.iter().map(|i| std::fs::read(&i.mPathItem).unwrap()).collect();
    assert_eq!(infos.len(), 1, "畸形 rels 的那条不 push，item2 仍被处理（Java catch (Throwable) 语义）");
    assert!(
        !contents.iter().any(|c| c == b"<root>1</root>"),
        "item1（.rels 畸形）不得产生条目"
    );
    assert!(contents.iter().any(|c| c == b"<root>2</root>"), "item2 必须仍被处理");
    assert!(
        infos.iter().any(|i| !i.mPathItemProp.is_empty()),
        "关系正常的那条必须有 itemPropPath"
    );
}

// ---------- DH11–DH16：多 item、顺序、判定边界 ----------

/// DH11 / **3 个 item** ⇒ 3 条（每个都带 itemProps）。
#[test]
fn dh11_multiple_items() {
    let mut entries: Vec<(String, String)> = Vec::new();
    for n in 1..=3 {
        entries.extend(item_entries(n, &format!("<root>{n}</root>"), &format!("<props>{n}</props>")));
    }
    let borrowed: Vec<(&str, &str)> = entries.iter().map(|(a, b)| (a.as_str(), b.as_str())).collect();
    let overrides: Vec<(&str, &str)> = (1..=3)
        .map(|n| match n {
            1 => ("/customXml/itemProps1.xml", PROPS_CT),
            2 => ("/customXml/itemProps2.xml", PROPS_CT),
            _ => ("/customXml/itemProps3.xml", PROPS_CT),
        })
        .collect();
    let mut pkg = open(package_with(&overrides, &borrowed));
    let _ = pkg.getParts();
    let infos = pkg.getCustomXmlInfos();
    // ★实测：每个 n 产出 `item{n}.xml` 与 `itemProps{n}.xml` **两个**命中项（DH14 的 Java `startsWith` 语义）
    assert_eq!(infos.len(), 3, "3 个 item{{n}} ⇒ 3 条（itemProps{{n}} 带 Override ⇒ 不命中 item 判定）");
    let item_like = infos.iter().filter(|i| !i.mPathItemProp.is_empty()).count();
    assert_eq!(item_like, 3, "3 条都通过各自 .rels 解出了 itemPropPath");
}

/// DH12 / **顺序不确定但内容确定**（不变量 6）：两次独立开包，条目数相同、且**按内容排序后**完全相同
/// ⇒ dump 侧若要判等必须排序（`itemPath` 是随机临时路径，不能作为排序键）。
#[test]
fn dh12_order_deterministic() {
    let mut entries: Vec<(String, String)> = Vec::new();
    for n in 1..=3 {
        entries.extend(item_entries(n, &format!("<root>{n}</root>"), &format!("<props>{n}</props>")));
    }
    let borrowed: Vec<(&str, &str)> = entries.iter().map(|(a, b)| (a.as_str(), b.as_str())).collect();
    let overrides: Vec<(&str, &str)> = (1..=3)
        .map(|n| match n {
            1 => ("/customXml/itemProps1.xml", PROPS_CT),
            2 => ("/customXml/itemProps2.xml", PROPS_CT),
            _ => ("/customXml/itemProps3.xml", PROPS_CT),
        })
        .collect();
    let bytes = package_with(&overrides, &borrowed);
    let collect = |bytes: Vec<u8>| -> Vec<Vec<u8>> {
        let mut pkg = open(bytes);
        let _ = pkg.getParts();
        let mut contents: Vec<Vec<u8>> = pkg
            .getCustomXmlInfos()
            .iter()
            .map(|i| std::fs::read(&i.mPathItem).unwrap())
            .collect();
        contents.sort();
        contents
    };
    let a = collect(bytes.clone());
    let b = collect(bytes);
    assert_eq!(a.len(), 3, "3 个 item{{n}}（见 DH11）");
    assert_eq!(a, b, "排序后两次必须完全相同（不变量 6）");
}

/// DH13 / `itemPath` 指向真实文件且内容 == item part（`sample2`，与 DH1 配对断言 getter 侧）。
#[test]
fn dh13_temp_file_content() {
    let path = sample2();
    let mut pkg = corpus(&path);
    let _ = pkg.getParts();
    let infos = pkg.getCustomXmlInfos();
    let item = std::fs::read(&infos[0].mPathItem).unwrap();
    assert!(item.starts_with(b"<?xml"), "★实测：item 临时文件是原始 XML 字节");
    assert_eq!(item, entry_bytes(&path, "customXml/item1.xml"));
}

/// DH14 / 判定边界：`/customXml/itemProps1.xml` **也**以 `/customXml/item` 开头
/// ⇒ **它也会被当成 item 收集**（Java `:186` 的 `startsWith` 判定如此）。
#[test]
fn dh14_itemprops_also_matched() {
    let mut pkg = open(package_with(
        &[],
        &[
            ("customXml/item1.xml", "<root>1</root>"),
            ("customXml/itemProps1.xml", "<props>1</props>"),
        ],
    ));
    let _ = pkg.getParts();
    assert_eq!(
        pkg.getCustomXmlInfos().len(),
        2,
        "`itemProps1.xml` 也命中 `startsWith(\"/customXml/item\")` ⇒ 也被收集（Java 同）"
    );
}

/// DH15 / `/customXml/data.xml`（不以 `item` 开头）⇒ 不收集。
#[test]
fn dh15_non_item_not_matched() {
    let mut pkg = open(package_with(&[], &[("customXml/data.xml", "<data/>")]));
    let _ = pkg.getParts();
    assert!(pkg.getCustomXmlInfos().is_empty(), "不以 /customXml/item 开头 ⇒ 不收集");
}

/// DH16 / 名字对但 **content type ≠ `application/xml`** ⇒ 不收集（不变量 1）。
#[test]
fn dh16_wrong_content_type() {
    let mut pkg = open(package_with(
        &[("/customXml/item1.xml", PROPS_CT)],
        &[("customXml/item1.xml", "<root>1</root>")],
    ));
    let _ = pkg.getParts();
    assert!(
        pkg.getCustomXmlInfos().is_empty(),
        "content type = customXmlProperties 而不是 application/xml ⇒ 不收集"
    );
}

// ---------- DH17–DH19：口径、出口、确定性 ----------

/// DH17 / `itemPropPath` 的「空」表达：Rust 侧是**空串**（Java `null`），字段类型不是 `Option`。
#[test]
fn dh17_empty_item_prop_path() {
    let mut pkg = open(package_with(&[], &[("customXml/item1.xml", "<root>1</root>")]));
    let _ = pkg.getParts();
    let infos = pkg.getCustomXmlInfos();
    assert_eq!(infos[0].mPathItemProp, "");
    // 类型层面已由 `String`（非 Option）保证；此处再钉一次「空串 = 无 itemProps」的表达口径
    assert_eq!(infos[0].mPathItemProp.as_str(), "", "空串就是「无 itemProps」的表达（Java null）");
}

/// DH18 / 出口 `getCustomXmlInfos()` 可读、条数与内容与内部一致。
#[test]
fn dh18_getter() {
    let mut pkg = corpus(&sample2());
    let _ = pkg.getParts();
    let infos = pkg.getCustomXmlInfos();
    assert_eq!(infos.len(), 1);
    // 两个 public 字段可读（Java `public String mPathItem` / `mPathItemProp`）
    let _ = infos[0].mPathItem.len();
    let _ = infos[0].mPathItemProp.len();
    // 二次调用稳定
    assert_eq!(pkg.getCustomXmlInfos().len(), 1);
}

/// DH19 / **确定性**：同一份 `sample2` 两次独立打开，排序后的条目内容完全相同。
#[test]
fn dh19_deterministic() {
    let path = sample2();
    let collect = || -> Vec<Vec<u8>> {
        let mut pkg = corpus(&path);
        let _ = pkg.getParts();
        let mut contents: Vec<Vec<u8>> = pkg
            .getCustomXmlInfos()
            .iter()
            .map(|i| {
                let mut both = std::fs::read(&i.mPathItem).unwrap();
                both.extend(std::fs::read(&i.mPathItemProp).unwrap());
                both
            })
            .collect();
        contents.sort();
        contents
    };
    assert_eq!(collect(), collect());
}
