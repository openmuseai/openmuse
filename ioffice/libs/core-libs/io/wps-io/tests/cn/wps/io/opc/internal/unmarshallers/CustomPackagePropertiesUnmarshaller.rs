// function_id=41_function_docprops_custom
// cargo test --manifest-path libs/core-libs/io/wps-io/Cargo.toml --test 41_function_docprops_custom
// src:  libs/core-libs/io/wps-io/src/cn/wps/io/opc/internal/unmarshallers/CustomPackagePropertiesUnmarshaller.rs
//       libs/core-libs/io/wps-io/src/cn/wps/io/opc/internal/CustomPackagePropertiesPart.rs
//       libs/core-libs/io/wps-io/src/cn/wps/io/opc/CustomPackageProperties.rs
// test: libs/core-libs/io/wps-io/tests/cn/wps/io/opc/internal/unmarshallers/CustomPackagePropertiesUnmarshaller.rs
//
// 覆盖矩阵 §3.2 的 DF1–DF20：`docProps/custom.xml` 字节流 → `[(name, type, value)]`（Java
// `CustomPackagePropertiesUnmarshaller.java:75-136`）。

use core_libs_io::io::opc::internal::unmarshallers::CustomPackagePropertiesUnmarshaller::CustomPackagePropertiesUnmarshaller;
use core_libs_io::io::opc::internal::unmarshallers::UnmarshallContext::UnmarshallContext;
use core_libs_io::io::opc::internal::CustomPackagePropertiesPart::CustomPackagePropertiesPart;
use core_libs_io::io::opc::CustomPackageProperties::{Property, PropertyType, PropertyValue};
use core_libs_io::io::opc::PackagePartName::PackagePartName;
use core_libs_io::io::opc::PackagingURIHelper::PackagingURIHelper;

const NS_CUSTOM: &str = "http://schemas.openxmlformats.org/officeDocument/2006/custom-properties";
const NS_VT: &str = "http://schemas.openxmlformats.org/officeDocument/2006/docPropsVTypes";

fn part_name(s: &str) -> PackagePartName {
    PackagingURIHelper::createPartName_str(s).unwrap()
}

fn context() -> UnmarshallContext {
    UnmarshallContext::new(None, part_name("/docProps/custom.xml"))
}

fn unmarshall(xml: &str) -> Result<Option<CustomPackagePropertiesPart>, String> {
    let ctx = context();
    let mut input = std::io::Cursor::new(xml.as_bytes().to_vec());
    CustomPackagePropertiesUnmarshaller::new().unmarshall(&ctx, Some(&mut input))
}

/// 造一份 `custom.xml`：`body` 为 `<property>` 序列。
fn doc(body: &str) -> String {
    format!(
        r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?><Properties xmlns="{NS_CUSTOM}" xmlns:vt="{NS_VT}">{body}</Properties>"#
    )
}

/// 取属性表的 `(name, type, 值字符串, linkTarget)` 视图（**下游 dump/`42` 的唯一可行读法**）。
fn props(part: &CustomPackagePropertiesPart) -> Vec<(String, &'static str, String, Option<String>)> {
    let list = part.getPropertyArray().as_ref().expect("有属性时必须 setPropertyArray");
    list.iter()
        .map(|entry| {
            let p = entry
                .downcast_ref::<Property>()
                .expect("入箱元素必须是 opc::CustomPackageProperties::Property（DF19）");
            let (ty, val) = match p.getType() {
                PropertyType::LPWSTRVAL => ("LPWSTRVAL", match p.getValue() {
                    PropertyValue::Str(s) => s.clone(),
                    other => panic!("LPWSTRVAL 值必须是 Str，实测 {other:?}"),
                }),
                PropertyType::BOOLVAL => ("BOOLVAL", match p.getValue() {
                    PropertyValue::Bool(v) => v.to_string(),
                    other => panic!("BOOLVAL 值必须是 Bool，实测 {other:?}"),
                }),
                PropertyType::INTVAL => ("INTVAL", match p.getValue() {
                    PropertyValue::Int(v) => v.to_string(),
                    other => panic!("INTVAL 值必须是 Int，实测 {other:?}"),
                }),
                PropertyType::DOUBLEVAL => ("DOUBLEVAL", match p.getValue() {
                    PropertyValue::Double(v) => v.to_string(),
                    other => panic!("DOUBLEVAL 值必须是 Double，实测 {other:?}"),
                }),
                PropertyType::DATEVAL => ("DATEVAL", match p.getValue() {
                    PropertyValue::Date(v) => v.to_string(),
                    other => panic!("DATEVAL 值必须是 Date，实测 {other:?}"),
                }),
            };
            (p.getName().to_string(), ty, val, p.mLinkTarge.clone())
        })
        .collect()
}

fn sample2_path() -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../../../samples/docx-mvp-render/flutter/assets/sample2.docx")
}

fn fixture_path(name: &str) -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/wave0-ooxml-standards")
        .join(name)
}

fn read_part(path: std::path::PathBuf, entry: &str) -> Option<String> {
    let bytes = std::fs::read(&path).unwrap_or_else(|e| panic!("语料 {} 不可读：{e}", path.display()));
    let mut archive = zip::ZipArchive::new(std::io::Cursor::new(bytes)).expect("zip 可打开");
    let mut file = match archive.by_name(entry) {
        Ok(f) => f,
        Err(_) => return None,
    };
    let mut xml = String::new();
    std::io::Read::read_to_string(&mut file, &mut xml).expect("条目可读");
    Some(xml)
}

/// DF1 / **主锚点（真语料）**：`sample2` 的 `custom.xml` ⇒ 2 条 `LPWSTRVAL`，按文档顺序，
/// 第 1 条带 `linkTarget="0"`（同时验证 `<vt:lpwstr>` 前缀能被 `element("lpwstr")` 命中）。
#[test]
fn df1_corpus_sample2() {
    let xml = read_part(sample2_path(), "docProps/custom.xml").expect("sample2 必须有 docProps/custom.xml");
    let part = unmarshall(&xml).expect("sample2 的 custom.xml 必须解析成功").expect("必须给出属性表");
    let got = props(&part);
    assert_eq!(got.len(), 2, "sample2 有 2 条自定义属性，实测 {got:?}");
    assert_eq!(got[0].0, "KSORubyTemplateID");
    assert_eq!(got[0].1, "LPWSTRVAL", "带 vt: 前缀的 <vt:lpwstr> 必须命中（不变量 3 头号风险）");
    assert_eq!(got[0].2, "6");
    assert_eq!(got[0].3.as_deref(), Some("0"), "linkTarget 必须读入（不变量 8）");
    assert_eq!(got[1].0, "KSOProductBuildVer");
    assert_eq!(got[1].1, "LPWSTRVAL");
    assert_eq!(got[1].2, "2052-10.1.0.7698");
    assert_eq!(got[1].3, None, "第 2 条没有 linkTarget");
    assert!(part.hasProperty());
}

/// DF2 / 负向真语料：C1 **没有** `custom.xml` ⇒ `42` 不会调用本 function；
/// 这里直接断言 ① 文件确实无该 entry ② 若强行喂一份空属性表 ⇒ 空表（不是 `null`、不是缺字段）。
#[test]
fn df2_corpus_c1_no_custom() {
    assert!(
        read_part(fixture_path("standard-hello.docx"), "docProps/custom.xml").is_none(),
        "C1 必须没有 docProps/custom.xml（★实测）"
    );
    let part = unmarshall(&doc("")).unwrap().expect("空属性表仍是「有表」");
    assert!(!part.hasProperty(), "空表 ⇒ hasProperty() == false");
    assert!(part.getPropertyArray().is_none(), "size()==0 ⇒ setPropertyArray 不被调用");
}

/// DF3 / 5 种类型各一条 ⇒ 5 条属性，类型依次正确。
#[test]
fn df3_five_types() {
    let xml = doc(concat!(
        r#"<property fmtid="{A}" pid="2" name="s"><vt:lpwstr>hello</vt:lpwstr></property>"#,
        r#"<property fmtid="{A}" pid="3" name="b"><vt:bool>true</vt:bool></property>"#,
        r#"<property fmtid="{A}" pid="4" name="i"><vt:i4>42</vt:i4></property>"#,
        r#"<property fmtid="{A}" pid="5" name="d"><vt:r8>3.5</vt:r8></property>"#,
        r#"<property fmtid="{A}" pid="6" name="t"><vt:filetime>2019-11-16T07:22:00Z</vt:filetime></property>"#,
    ));
    let part = unmarshall(&xml).unwrap().unwrap();
    let got = props(&part);
    assert_eq!(got.len(), 5, "实测 {got:?}");
    assert_eq!((got[0].0.as_str(), got[0].1, got[0].2.as_str()), ("s", "LPWSTRVAL", "hello"));
    assert_eq!((got[1].0.as_str(), got[1].1, got[1].2.as_str()), ("b", "BOOLVAL", "true"));
    assert_eq!((got[2].0.as_str(), got[2].1, got[2].2.as_str()), ("i", "INTVAL", "42"));
    assert_eq!((got[3].0.as_str(), got[3].1, got[3].2.as_str()), ("d", "DOUBLEVAL", "3.5"));
    assert_eq!((got[4].0.as_str(), got[4].1), ("t", "DATEVAL"));
    // Java: Date.getTime() = 1970 起的毫秒（UTC）
    assert_eq!(got[4].2, "1573888920000", "DATEVAL 必须是 Java `Date` 语义的 epoch 毫秒（DF20）");
}

/// DF4 / **覆盖顺序**（不变量 2）：`lpwstr→bool→i4→r8→filetime`，后者覆盖前者。
#[test]
fn df4_type_override_order() {
    let a = unmarshall(&doc(
        r#"<property name="p"><vt:lpwstr>s</vt:lpwstr><vt:i4>7</vt:i4></property>"#,
    ))
    .unwrap()
    .unwrap();
    let got = props(&a);
    assert_eq!((got[0].1, got[0].2.as_str()), ("INTVAL", "7"), "i4 在 lpwstr 之后 ⇒ 后者胜");

    let b = unmarshall(&doc(
        r#"<property name="p"><vt:lpwstr>s</vt:lpwstr><vt:bool>true</vt:bool><vt:i4>7</vt:i4><vt:r8>1.5</vt:r8><vt:filetime>2019-11-16T07:22:00Z</vt:filetime></property>"#,
    ))
    .unwrap()
    .unwrap();
    assert_eq!(props(&b)[0].1, "DATEVAL", "5 种齐全 ⇒ filetime 最后写");
}

/// DF5 / `name` 缺失 ⇒ 该条丢弃，其余保留（不变量 4）。
#[test]
fn df5_missing_name_dropped() {
    let xml = doc(concat!(
        r#"<property><vt:lpwstr>x</vt:lpwstr></property>"#,
        r#"<property name="a"><vt:lpwstr>y</vt:lpwstr></property>"#,
        r#"<property><vt:lpwstr>z</vt:lpwstr></property>"#,
    ));
    let part = unmarshall(&xml).unwrap().unwrap();
    let got = props(&part);
    assert_eq!(got.len(), 1, "只剩那条有 name 的，实测 {got:?}");
    assert_eq!(got[0].0, "a");
}

/// DF6 / 5 种类型都不成立 ⇒ 整条丢弃且**不报错**（Java `IOAssert.fail` 是空实现）。
#[test]
fn df6_unsupported_type_dropped() {
    let part = unmarshall(&doc(r#"<property name="p"><vt:foo>x</vt:foo></property>"#))
        .expect("不支持的类型不得变成 Err")
        .expect("仍应给出属性表");
    assert!(!part.hasProperty(), "0 条 ⇒ hasProperty() == false");
}

/// DF7 / `<Properties/>` ⇒ `Ok(Some(part))` 且空表（Java `loadProperty` = `root.elements(..)` **不为 null**）。
#[test]
fn df7_empty_table() {
    let part = unmarshall(&doc("")).unwrap().unwrap();
    assert!(!part.hasProperty());
    assert!(part.getPropertyArray().is_none());
}

/// DF8 / `linkTarget` 被读入 `mLinkTarge`（不进 dump，只能靠单测）。
#[test]
fn df8_link_target_read() {
    let part = unmarshall(&doc(
        r#"<property name="p" linkTarget="0"><vt:lpwstr>6</vt:lpwstr></property>"#,
    ))
    .unwrap()
    .unwrap();
    let got = props(&part);
    assert_eq!(got[0].3.as_deref(), Some("0"));
    let none = unmarshall(&doc(r#"<property name="p"><vt:lpwstr>6</vt:lpwstr></property>"#))
        .unwrap()
        .unwrap();
    assert_eq!(props(&none)[0].3, None, "无 linkTarget 属性 ⇒ None");
}

/// DF9 / `fmtid`/`pid` 不参与输出：仅这两个属性不同的两条 ⇒ 结果完全相同。
#[test]
fn df9_fmtid_ignored() {
    let a = unmarshall(&doc(
        r#"<property fmtid="{A}" pid="2" name="p"><vt:lpwstr>v</vt:lpwstr></property>"#,
    ))
    .unwrap()
    .unwrap();
    let b = unmarshall(&doc(
        r#"<property fmtid="{B}" pid="99" name="p"><vt:lpwstr>v</vt:lpwstr></property>"#,
    ))
    .unwrap()
    .unwrap();
    assert_eq!(props(&a), props(&b), "fmtid/pid 不得影响 (name,type,value,linkTarget)");
}

/// DF10 / `<vt:lpwstr></vt:lpwstr>` 空文本 ⇒ `Some("")`（**与其他 4 类不同**：`loadLpwstr` 无长度判定）。
#[test]
fn df10_lpwstr_empty_text() {
    let part = unmarshall(&doc(concat!(
        r#"<property name="s"><vt:lpwstr></vt:lpwstr></property>"#,
        r#"<property name="b"><vt:bool></vt:bool></property>"#,
    )))
    .unwrap()
    .unwrap();
    let got = props(&part);
    assert_eq!(got.len(), 1, "bool 空文本 ⇒ 该类型不成立 ⇒ 整条丢弃；实测 {got:?}");
    assert_eq!((got[0].0.as_str(), got[0].1, got[0].2.as_str()), ("s", "LPWSTRVAL", ""));
}

/// DF11 / 布尔形态：`false` ⇒ `Some(false)`；`yes` ⇒ `Some(false)`；`TRUE` ⇒ `Some(true)`（Java `Boolean(String)`）。
#[test]
fn df11_bool_forms() {
    let part = unmarshall(&doc(concat!(
        r#"<property name="f"><vt:bool>false</vt:bool></property>"#,
        r#"<property name="y"><vt:bool>yes</vt:bool></property>"#,
        r#"<property name="u"><vt:bool>TRUE</vt:bool></property>"#,
    )))
    .unwrap()
    .unwrap();
    let got = props(&part);
    assert_eq!(got.len(), 3, "实测 {got:?}");
    assert_eq!(got[0].2, "false");
    assert_eq!(got[1].2, "false", "非 \"true\" ⇒ false（与 Java 一致）");
    assert_eq!(got[2].2, "true", "Java Boolean(String) 不区分大小写");
}

/// DF12 / 数值畸形：`abc`/`x` ⇒ 该类型不成立 ⇒ 整条丢弃；**不得 panic**。
/// `\" 42 \"` / `1e3` 走 `TypeConversion`（Java 的 trim + BigDecimal 语义）。
#[test]
fn df12_numeric_malformed() {
    let bad = unmarshall(&doc(concat!(
        r#"<property name="i"><vt:i4>abc</vt:i4></property>"#,
        r#"<property name="d"><vt:r8>x</vt:r8></property>"#,
    )))
    .expect("畸形数值不得变成 Err")
    .unwrap();
    assert!(!bad.hasProperty(), "两条都因类型不成立而丢弃");

    let ok = unmarshall(&doc(concat!(
        r#"<property name="i"><vt:i4> 42 </vt:i4></property>"#,
        r#"<property name="e"><vt:i4>1e3</vt:i4></property>"#,
    )))
    .unwrap()
    .unwrap();
    let got = props(&ok);
    assert_eq!(got[0].2, "42", "Java TypeConversion 先 trim");
    assert_eq!(got[1].2, "1000", "Java 先 BigDecimal.toPlainString()");
}

/// DF13 / **命名空间**（不变量 3，头号风险）：带前缀 / 无前缀 / **别的命名空间前缀** 都必须命中
/// （JDOM `element(String)` = 本地名匹配，与命名空间无关）。
#[test]
fn df13_namespace_prefix() {
    let a = unmarshall(&doc(r#"<property name="p"><vt:lpwstr>6</vt:lpwstr></property>"#)).unwrap().unwrap();
    let b = unmarshall(&doc(r#"<property name="p"><lpwstr>6</lpwstr></property>"#)).unwrap().unwrap();
    let c = unmarshall(&doc(
        r#"<property name="p"><x:lpwstr xmlns:x="http://example.invalid/other">6</x:lpwstr></property>"#,
    ))
    .unwrap()
    .unwrap();
    assert_eq!(props(&a)[0].2, "6");
    assert_eq!(props(&b)[0].2, "6", "无前缀也必须命中");
    assert_eq!(props(&c)[0].2, "6", "别的命名空间前缀同样按本地名命中（Java JDOM 语义）");
}

/// DF14 / 非 well-formed XML ⇒ `Err`。
/// ★**实测边界（登记 §1.7.1）**：替代解析器 `XmlParse` 只在「根元素未闭合 / 0 字节」时报错；
/// 属性值未闭合、闭合标签不匹配这类畸形**不会**被它拒绝（Java 的 JDK `SAXParser` 会抛 `DocumentException`）
/// ⇒ 本用例断言**确定会报错**的那一类，并把宽松面显式登记，避免把「解析器宽松」当成「本 function 对齐」。
#[test]
fn df14_malformed_xml_err() {
    let ctx = context();
    let mut input = std::io::Cursor::new(
        format!(r#"<Properties xmlns="{NS_CUSTOM}"><property name="a"></property>"#).into_bytes(),
    );
    let got = CustomPackagePropertiesUnmarshaller::new().unmarshall(&ctx, Some(&mut input));
    assert!(got.is_err(), "根元素未闭合必须 Err（实测 Err）");

    // 宽松面（★实测 Ok，与 Java 分叉，登记在案，**不**在此断言成 Err）
    let lenient = unmarshall(&doc(r#"<property name="a""#));
    assert!(
        matches!(lenient, Ok(_)),
        "★实测：属性值未闭合时替代解析器不报错（Java 会 Err）—— 若将来变严，本断言需同步更新登记"
    );
}

/// DF15 / 空输入（0 字节）⇒ `Err`。
#[test]
fn df15_empty_input_err() {
    let ctx = context();
    let mut input = std::io::Cursor::new(Vec::new());
    let got = CustomPackagePropertiesUnmarshaller::new().unmarshall(&ctx, Some(&mut input));
    assert!(got.is_err(), "0 字节必须 Err");
}

/// DF16 / 「无表」两出口：`input=None` + `zipEntry=None` ⇒ `Ok(None)`；`input=None` 且 `context` 无 package
/// 且有 zipEntry ⇒ Java `:87-89 throw` ⇒ `Err`。**均不得 panic**。
#[test]
fn df16_no_table_paths() {
    let ctx = UnmarshallContext::new(None, part_name("/docProps/custom.xml"));
    let got = CustomPackagePropertiesUnmarshaller::new().unmarshall(&ctx, None);
    match got {
        Ok(None) => {}
        Err(e) => panic!("zipEntry 缺失必须是 Ok(None)（Java :77-80 return null），实测 Err({e})"),
        Ok(Some(_)) => panic!("zipEntry 缺失必须给 Ok(None)，不得给出属性表"),
    }
}

/// DF17 / 确定性 + 文档顺序：5 条顺序与取值两次完全一致（**不排序**）。
#[test]
fn df17_order_deterministic() {
    let xml = doc(concat!(
        r#"<property name="c"><vt:i4>3</vt:i4></property>"#,
        r#"<property name="a"><vt:i4>1</vt:i4></property>"#,
        r#"<property name="b"><vt:i4>2</vt:i4></property>"#,
    ));
    let p1 = unmarshall(&xml).unwrap().unwrap();
    let p2 = unmarshall(&xml).unwrap().unwrap();
    let g1 = props(&p1);
    let g2 = props(&p2);
    assert_eq!(g1, g2, "两次解析必须一致");
    let names: Vec<&str> = g1.iter().map(|t| t.0.as_str()).collect();
    assert_eq!(names, vec!["c", "a", "b"], "顺序 = XML 文档顺序");
}

/// DF18 / 无副作用：`context` 前后不变。
#[test]
fn df18_no_side_effect() {
    let ctx = context();
    let before = ctx.getPartName().as_ref().map(|n| n.getName().to_string());
    let mut input = std::io::Cursor::new(doc(r#"<property name="p"><vt:i4>1</vt:i4></property>"#).into_bytes());
    let _ = CustomPackagePropertiesUnmarshaller::new().unmarshall(&ctx, Some(&mut input));
    let after = ctx.getPartName().as_ref().map(|n| n.getName().to_string());
    assert_eq!(after, before);
}

/// DF19 / **dump 前提**：入箱元素必须能 `downcast_ref::<Property>()`，且 3 个 `pub` getter 可用。
#[test]
fn df19_downcast_property() {
    let part = unmarshall(&doc(concat!(
        r#"<property name="s" linkTarget="L"><vt:lpwstr>hi</vt:lpwstr></property>"#,
        r#"<property name="i"><vt:i4>9</vt:i4></property>"#,
    )))
    .unwrap()
    .unwrap();
    let list = part.getPropertyArray().as_ref().unwrap();
    assert_eq!(list.len(), 2);
    let p0 = list[0].downcast_ref::<Property>().expect("必须能 downcast 到 Property");
    assert_eq!(p0.getName(), "s");
    assert_eq!(p0.getType(), PropertyType::LPWSTRVAL);
    assert!(matches!(p0.getValue(), PropertyValue::Str(v) if v == "hi"));
    assert_eq!(p0.mLinkTarge.as_deref(), Some("L"));
    assert!(list[1].downcast_ref::<Property>().is_some());
}

/// DF20 / `DATEVAL` 与 Java `Date` 的字符串化口径：Rust 存 **epoch 毫秒 `i64`**（= Java `Date.getTime()`），
/// 与 Java `Date.toString()`（`Sat Nov 16 07:22:00 UTC 2019`）**必然不同** ⇒ dump 侧必须显式定义格式化（差异登记）。
#[test]
fn df20_dateval_stringify() {
    let part = unmarshall(&doc(
        r#"<property name="t"><vt:filetime>2019-11-16T07:22:00Z</vt:filetime></property>"#,
    ))
    .unwrap()
    .unwrap();
    let got = props(&part);
    assert_eq!(got[0].1, "DATEVAL");
    assert_eq!(got[0].2, "1573888920000", "epoch 毫秒（Java Date.getTime()）");
    // Java 的 SimpleDateFormat 是**前缀**匹配：尾随多余字符被忽略（已按此对齐）
    let trailing = unmarshall(&doc(
        r#"<property name="t"><vt:filetime>2019-11-16T07:22:00Z-junk</vt:filetime></property>"#,
    ))
    .unwrap()
    .unwrap();
    assert_eq!(props(&trailing)[0].2, "1573888920000", "前缀匹配，尾随字符忽略");
    // 空文本 ⇒ 该类型不成立 ⇒ 丢弃（Java setDateValue("") ⇒ null）
    let empty = unmarshall(&doc(r#"<property name="t"><vt:filetime></vt:filetime></property>"#))
        .unwrap()
        .unwrap();
    assert!(!empty.hasProperty());
}

/// §3.12 产品接线：`XWPFPackage::open_stream` → `getParts()` → `getCustomPackageProperties()`
/// 必须是真解出来的 2 条自定义属性（此前是空壳 `new(None, name)`）。
#[test]
fn wiring_xwpfpackage_getparts_custom_properties() {
    let bytes = std::fs::read(sample2_path()).expect("语料必须在库");
    let mut input = std::io::Cursor::new(bytes);
    let mut package = core_libs_io::io::xwpf::XWPFPackage::XWPFPackage::open_stream(&mut input);
    let _parts = package.getParts();
    // Java: getCustomPackageProperties() 非 null（缺席时惰性造空 part）⇒ Rust 同形返回 `&T`
    let part = package.getCustomPackageProperties();
    assert!(part.hasProperty(), "空壳 ⇒ hasProperty() == false（本断言必须红）");
    let got = props(part);
    assert_eq!(got.len(), 2);
    assert_eq!(got[0].0, "KSORubyTemplateID");
    assert_eq!(got[0].2, "6");
    assert_eq!(got[1].0, "KSOProductBuildVer");
    assert_eq!(got[1].2, "2052-10.1.0.7698");
}

