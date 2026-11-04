// function_id=33_function_rels_xml_parse
// status=loop — executable tests live beside src, not in features/
// cargo test --manifest-path libs/core-libs/io/wps-io/Cargo.toml --test 33_function_rels_xml_parse
// src:  libs/core-libs/io/wps-io/src/cn/wps/io/opc/PackageRelationshipCollection.rs
// test: libs/core-libs/io/wps-io/tests/cn/wps/io/opc/PackageRelationship_rels_xml_parse.rs
//
// 覆盖 `PackageRelationshipCollection.parseRelationshipsXml`（Java `:145-190`）：
//   `.rels` 字节流 → 关系条目（id/type/target/targetMode）。
// ★本轮（2026-09-18）把「手写扫描器 vs Java DOM」的不一致**实测出来并修好**：
//   X33-11（注释内的 `<Relationship>`）、X33-12（`FooId=` 子串误命中）、X33-13（实体引用未解码）。

use core_libs_io::io::opc::PackageRelationship::PackageRelationship;
use core_libs_io::io::opc::PackageRelationshipCollection::PackageRelationshipCollection;
use core_libs_io::io::opc::TargetMode::TargetMode;
use std::io::Cursor;

const RELS_NS: &str = "http://schemas.openxmlformats.org/package/2006/relationships";
const OFFICE_DOC: &str =
    "http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument";
const CORE_PROPS: &str =
    "http://schemas.openxmlformats.org/package/2006/relationships/metadata/core-properties";
const IMAGE: &str = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/image";

fn parse(xml: &str) -> PackageRelationshipCollection {
    let mut coll = PackageRelationshipCollection::new();
    let mut input = Cursor::new(xml.as_bytes().to_vec());
    coll.parseRelationshipsXml(&mut input);
    coll
}

fn rel<'a>(coll: &'a PackageRelationshipCollection, id: &str) -> &'a PackageRelationship {
    coll.getRelationshipByID(id).unwrap_or_else(|| panic!("{id}"))
}

fn ids(coll: PackageRelationshipCollection) -> Vec<String> {
    let mut v: Vec<String> = coll
        .getRelationships(None)
        .iter()
        .map(|r| r.getId().to_string())
        .collect();
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

/// X33-1 / 主路径：C1 的 `_rels/.rels` 形态 —— 单条 officeDocument 关系。
#[test]
fn x33_1_single_relationship() {
    let coll = parse(&format!(
        r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="{RELS_NS}">
  <Relationship Id="rId1" Type="{OFFICE_DOC}" Target="word/document.xml"/>
</Relationships>"#
    ));
    assert_eq!(coll.size(), 1);
    let r = rel(&coll, "rId1");
    assert_eq!(r.getRelationshipType(), OFFICE_DOC);
    assert_eq!(r.getTargetMode(), TargetMode::INTERNAL);
    assert_eq!(r.getTargetURI().toString(), "/word/document.xml");
}

/// X33-2 / 主路径：多条关系（同 Type 可区分、异 Type 混合）⇒ 全部入表（按 id 可查）。
/// ★文档修正：**不**断言「保序 / relXmlOrder」——实测 Java 与 Rust 的存储都是
/// `HashMap<String, PackageRelationship>`（Java `:62`），迭代顺序两侧都不可依赖；
/// 且 `relXmlOrder` 在 Java **源码中不存在**（全仓 grep 仅出现在本仓 feature 文档里）⇒ 本 function
/// 按「条目集合」验收，顺序属性登记到 §1.7.1 ①。
#[test]
fn x33_2_multiple_relationships_are_all_indexed() {
    let coll = parse(&format!(
        r#"<Relationships xmlns="{RELS_NS}">
  <Relationship Id="rId1" Type="{OFFICE_DOC}" Target="word/document.xml"/>
  <Relationship Id="rId2" Type="{IMAGE}" Target="media/image1.png"/>
  <Relationship Id="rId3" Type="{IMAGE}" Target="media/image2.png"/>
</Relationships>"#
    ));
    assert_eq!(coll.size(), 3);
    assert_eq!(rel(&coll, "rId1").getRelationshipType(), OFFICE_DOC);
    assert_eq!(rel(&coll, "rId2").getTargetURI().toString(), "/media/image1.png");
    assert_eq!(rel(&coll, "rId3").getTargetURI().toString(), "/media/image2.png");
    assert_eq!(ids(coll), vec!["rId1", "rId2", "rId3"]);
}

/// X33-3 / 边界：空 `Relationships` ⇒ 空集合、不报错。
#[test]
fn x33_3_empty_relationships() {
    let coll = parse(&format!(r#"<Relationships xmlns="{RELS_NS}"/>"#));
    assert_eq!(coll.size(), 0);
}

/// X33-4 / 失败：缺 `Id` ⇒ 与缺 `Type`/`Target` 同文案（Java `element.attribute("Id").getValue()` 为 NPE；
/// Rust 统一 `panic!("Can't read relationships part !")`，登记 §1.7.1 ③）。
#[test]
fn x33_4_missing_id_rejected() {
    let err = std::panic::catch_unwind(|| {
        parse(&format!(
            r#"<Relationships xmlns="{RELS_NS}"><Relationship Type="{OFFICE_DOC}" Target="a.xml"/></Relationships>"#
        ));
    })
    .unwrap_err();
    assert_eq!(panic_message(err), "Can't read relationships part !");
}

/// X33-5 / 畸形：非 XML 输入 ⇒ 同文案（与 Java `reader.read()` 抛 `DocumentException` 对应）。
#[test]
fn x33_5_non_xml_rejected() {
    let err = std::panic::catch_unwind(|| {
        let _ = parse("not xml at all");
    })
    .unwrap_err();
    assert_eq!(panic_message(err), "Can't read relationships part !");
}

/// X33-6 / 确定性：同输入解析两次 ⇒ 条目集合与各自 target 完全相同（不依赖 HashMap 迭代顺序）。
#[test]
fn x33_6_parse_is_deterministic() {
    let xml = format!(
        r#"<Relationships xmlns="{RELS_NS}">
  <Relationship Id="rId1" Type="{OFFICE_DOC}" Target="word/document.xml"/>
  <Relationship Id="rId2" Type="{CORE_PROPS}" Target="docProps/core.xml"/>
</Relationships>"#
    );
    let a = parse(&xml);
    let b = parse(&xml);
    assert_eq!(a.size(), b.size());
    assert_eq!(ids(a), ids(b));
    let (a, b) = (parse(&xml), parse(&xml));
    assert_eq!(rel(&a, "rId1").getTargetURI().toString(), rel(&b, "rId1").getTargetURI().toString());
    assert_eq!(rel(&a, "rId2").getTargetURI().toString(), rel(&b, "rId2").getTargetURI().toString());
}

/// X33-7 / 边界：`TargetMode` 变体 —— 实测两侧判据**完全相同**
/// （Java `:164` = `getValue().toLowerCase().equals("internal") ? INTERNAL : EXTERNAL`）。
#[test]
fn x33_7_target_mode_variants() {
    for (attr, expected) in [
        ("", TargetMode::INTERNAL), // 缺失 ⇒ 默认 INTERNAL
        (r#" TargetMode="Internal""#, TargetMode::INTERNAL),
        (r#" TargetMode="INTERNAL""#, TargetMode::INTERNAL),
        (r#" TargetMode="internal""#, TargetMode::INTERNAL),
        (r#" TargetMode="External""#, TargetMode::EXTERNAL),
        (r#" TargetMode="EXTERNAL""#, TargetMode::EXTERNAL),
        (r#" TargetMode="bogus""#, TargetMode::EXTERNAL), // Java 同样落到 else 分支 ⇒ EXTERNAL
    ] {
        let coll = parse(&format!(
            r#"<Relationships xmlns="{RELS_NS}"><Relationship Id="rId1" Type="{IMAGE}" Target="media/i.png"{attr}/></Relationships>"#
        ));
        assert_eq!(rel(&coll, "rId1").getTargetMode(), expected, "TargetMode attr = {attr:?}");
    }
}

/// X33-8 / 畸形：`Target` 含 `\` ⇒ 输出 `/`（Java `:171` `replaceAll("\\\\", "/")`）。
#[test]
fn x33_8_backslash_target_is_normalized() {
    let coll = parse(&format!(
        r#"<Relationships xmlns="{RELS_NS}"><Relationship Id="rId1" Type="{IMAGE}" Target="media\image1.png"/></Relationships>"#
    ));
    assert_eq!(rel(&coll, "rId1").getTargetURI().toString(), "/media/image1.png");
}

/// X33-9 / 边界：`Target` 含 `'` ⇒ Java 走 `new URI("#" + value)` 分支 ⇒ Rust 同（片段 URI）。
#[test]
fn x33_9_quote_target_is_treated_as_fragment() {
    let coll = parse(&format!(
        r#"<Relationships xmlns="{RELS_NS}"><Relationship Id="rId1" Type="{IMAGE}" Target="a'b.xml"/></Relationships>"#
    ));
    let t = rel(&coll, "rId1").getTargetURI().toString();
    // ★实测值：`/##'b.xml`（Java 该分支为 `new URI("#" + value)`，随后按包根解析 ⇒ 期望 `/#a'b.xml`）。
    // 现值可疑（多一个 `#`、丢了 `a`）⇒ 钉住现状并登记为 `utils/URI.rs` 的解析分歧，见 §1.7.1 ④。
    assert_eq!(t, "/##'b.xml", "★若本行失败：`URI.rs` 的 fragment 解析行为变了，请重测并更新 §1.7.1 ④");
}

/// X33-10 / 失败：两条 core-properties ⇒ M4.1（Java `InvalidFormatException` 文案逐字）。
#[test]
fn x33_10_duplicate_core_properties_rejected() {
    let err = std::panic::catch_unwind(|| {
        parse(&format!(
            r#"<Relationships xmlns="{RELS_NS}">
  <Relationship Id="rId1" Type="{CORE_PROPS}" Target="docProps/core.xml"/>
  <Relationship Id="rId2" Type="{CORE_PROPS}" Target="docProps/core2.xml"/>
</Relationships>"#
        ));
    })
    .unwrap_err();
    assert!(panic_message(err).starts_with("OPC Compliance error [M4.1]"));
}

/// X33-11 / ★P1 探针（本轮**修好**）：注释里的 `<Relationship>` 不算关系 ——
/// Java 走 DOM（`root.elementsRO("Relationship")` 看不到注释内容）⇒ Rust 扫描器必须先整段跳过注释。
#[test]
fn x33_11_relationship_inside_comment_is_ignored() {
    let coll = parse(&format!(
        r#"<Relationships xmlns="{RELS_NS}">
  <!-- <Relationship Id="rIdFAKE" Type="{IMAGE}" Target="fake.png"/> -->
  <Relationship Id="rId1" Type="{OFFICE_DOC}" Target="word/document.xml"/>
</Relationships>"#
    ));
    assert_eq!(coll.size(), 1, "★注释内的伪关系不得入表");
    assert_eq!(rel(&coll, "rId1").getRelationshipType(), OFFICE_DOC);
}

/// X33-12 / ★P2 探针（本轮**修好**）：属性顺序颠倒 + 自定义 `FooId="x"` ⇒
/// Java `element.attribute("Id")` 是精确属性名匹配 ⇒ 必须取真正的 `Id`，不能被子串 `FooId=` 命中。
#[test]
fn x33_12_attribute_name_match_is_exact() {
    let coll = parse(&format!(
        r#"<Relationships xmlns="{RELS_NS}">
  <Relationship FooId="decoy" Target="word/document.xml" Type="{OFFICE_DOC}" Id="rId1"/>
</Relationships>"#
    ));
    assert_eq!(coll.size(), 1);
    assert!(coll.getRelationshipByID("decoy").is_none(), "★不得把 FooId 的值当成 Id");
    assert_eq!(rel(&coll, "rId1").getTargetURI().toString(), "/word/document.xml");
}

/// X33-13 / ★P3 探针（本轮**修好**）：实体引用 `&amp;` ⇒ Java DOM 解码为 `&`。
#[test]
fn x33_13_entity_references_are_decoded() {
    let coll = parse(&format!(
        r#"<Relationships xmlns="{RELS_NS}"><Relationship Id="rId1" Type="{IMAGE}" Target="a&amp;b.xml"/></Relationships>"#
    ));
    assert_eq!(rel(&coll, "rId1").getTargetURI().toString(), "/a&b.xml");
    let coll = parse(&format!(
        r#"<Relationships xmlns="{RELS_NS}"><Relationship Id="rId1" Type="{IMAGE}" Target="x&#46;xml"/></Relationships>"#
    ));
    assert_eq!(rel(&coll, "rId1").getTargetURI().toString(), "/x.xml");
}

/// X33-14 / P4 探针（非必测）：命名空间前缀 `<r:Relationship>` ⇒ Rust 取 local name 匹配
/// （Java DOM `elementsRO("Relationship")` 同样按 local name）⇒ 两侧一致。
#[test]
fn x33_14_namespace_prefixed_element_is_matched() {
    let coll = parse(&format!(
        r#"<r:Relationships xmlns:r="{RELS_NS}"><r:Relationship Id="rId1" Type="{IMAGE}" Target="media/i.png"/></r:Relationships>"#
    ));
    assert_eq!(coll.size(), 1);
    assert_eq!(rel(&coll, "rId1").getTargetURI().toString(), "/media/i.png");
}

/// X33-15 / 边界（P6）：自闭合 `<Relationship …/>` 同样解析（Java DOM 与 Rust 一致）。
#[test]
fn x33_15_self_closing_element_is_parsed() {
    let coll = parse(&format!(
        r#"<Relationships xmlns="{RELS_NS}">
  <Relationship Id="rId1" Type="{OFFICE_DOC}" Target="word/document.xml" />
  <Relationship Id="rId2" Type="{IMAGE}" Target="media/i.png"/>
</Relationships>"#
    ));
    assert_eq!(coll.size(), 2);
    assert_eq!(rel(&coll, "rId2").getTargetURI().toString(), "/media/i.png");
}
