// function_id=40_function_docprops_extended
// cargo test --manifest-path libs/core-libs/io/wps-io/Cargo.toml --test 40_function_docprops_extended
// src:  libs/core-libs/io/wps-io/src/cn/wps/io/opc/internal/unmarshallers/ExtendedPackagePropertiesUnmarshaller.rs
//       libs/core-libs/io/wps-io/src/cn/wps/io/opc/internal/ExtendedPackagePropertiesPart.rs
// test: libs/core-libs/io/wps-io/tests/cn/wps/io/opc/internal/unmarshallers/ExtendedPackagePropertiesUnmarshaller.rs
//
// 覆盖矩阵 §3.2 的 DE1–DE19：`docProps/app.xml` 字节流 → 27 个具名属性
// （Java `ExtendedPackagePropertiesUnmarshaller.java:55-189`）。

use core_libs_io::io::opc::internal::unmarshallers::ExtendedPackagePropertiesUnmarshaller::ExtendedPackagePropertiesUnmarshaller;
use core_libs_io::io::opc::internal::unmarshallers::UnmarshallContext::UnmarshallContext;
use core_libs_io::io::opc::internal::ExtendedPackagePropertiesPart::ExtendedPackagePropertiesPart;
use core_libs_io::io::opc::PackagePartName::PackagePartName;
use core_libs_io::io::opc::PackagingURIHelper::PackagingURIHelper;

const EXT: &str = "http://schemas.openxmlformats.org/officeDocument/2006/extended-properties";

fn part_name(s: &str) -> PackagePartName {
    PackagingURIHelper::createPartName_str(s).unwrap()
}

fn context() -> UnmarshallContext {
    UnmarshallContext::new(None, part_name("/docProps/app.xml"))
}

fn unmarshall(xml: &str) -> Result<ExtendedPackagePropertiesPart, String> {
    let ctx = context();
    let mut input = std::io::Cursor::new(xml.as_bytes().to_vec());
    ExtendedPackagePropertiesUnmarshaller::new()
        .unmarshall(&ctx, Some(&mut input))
        .map(|opt| opt.expect("unmarshall 成功时必须产出 part"))
}

fn doc(body: &str) -> String {
    format!(r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?><Properties xmlns="{EXT}">{body}</Properties>"#)
}

/// 27 个 dump 键 ↔ getter（键名口径：getter 名去 `get`/`Property` 后首字母小写；
/// `mMClips` 是唯一有歧义项，见 §1.6 不变量 1，此处采用 `mMClips`）。
fn value_of(p: &ExtendedPackagePropertiesPart, key: &str) -> Option<String> {
    match key {
        "application" => p.getApplicationProperty().map(|v| v.to_string()),
        "appVersion" => p.getAppVersionProperty().map(|v| v.to_string()),
        "characters" => p.getCharactersProperty().map(|v| v.to_string()),
        "company" => p.getCompanyProperty().map(|v| v.to_string()),
        "digSig" => p.getDigSigProperty().map(|v| v.to_string()),
        "docSecurity" => p.getDocSecurityProperty().map(|v| v.to_string()),
        "headingPairs" => p.getHeadingPairsProperty().map(|v| v.to_string()),
        "hiddenSlides" => p.getHiddenSlidesProperty().map(|v| v.to_string()),
        "hLinks" => p.getHLinksProperty().map(|v| v.to_string()),
        "hyperlinkBase" => p.getHyperlinkBaseProperty().map(|v| v.to_string()),
        "hyperlinksChanged" => p.getHyperlinksChangedProperty().map(|v| v.to_string()),
        "lines" => p.getLinesProperty().map(|v| v.to_string()),
        "linksUpToDate" => p.getLinksUpToDateProperty().map(|v| v.to_string()),
        "manager" => p.getManagerProperty().map(|v| v.to_string()),
        "mMClips" => p.getMMClipsProperty().map(|v| v.to_string()),
        "notes" => p.getNotesProperty().map(|v| v.to_string()),
        "pages" => p.getPagesProperty().map(|v| v.to_string()),
        "paragraphs" => p.getParagraphsProperty().map(|v| v.to_string()),
        "presentationFormat" => p.getPresentationFormatProperty().map(|v| v.to_string()),
        "scaleCrop" => p.getScaleCropProperty().map(|v| v.to_string()),
        "sharedDoc" => p.getSharedDocProperty().map(|v| v.to_string()),
        "slides" => p.getSlidesProperty().map(|v| v.to_string()),
        "template" => p.getTemplateProperty().map(|v| v.to_string()),
        "titlesOfParts" => p.getTitlesOfPartsProperty().map(|v| v.to_string()),
        "totalTime" => p.getTotalTimeProperty().map(|v| v.to_string()),
        "wordCount" => p.getWordCountProperty().map(|v| v.to_string()),
        "numberCharacters" => p.getNumberCharactersProperty().map(|v| v.to_string()),
        other => panic!("未知键 {other}"),
    }
}

const KEYS_27: [&str; 27] = [
    "application", "appVersion", "characters", "company", "digSig", "docSecurity", "headingPairs",
    "hiddenSlides", "hLinks", "hyperlinkBase", "hyperlinksChanged", "lines", "linksUpToDate",
    "manager", "mMClips", "notes", "pages", "paragraphs", "presentationFormat", "scaleCrop",
    "sharedDoc", "slides", "template", "titlesOfParts", "totalTime", "wordCount", "numberCharacters",
];

/// 19 个**可被 setter 写入**的键（其余 8 个：4 个恒 null getter + `characters`/`lines`/`notes`/`paragraphs` 无人赋值）
const KEYS_SETTABLE_19: [&str; 19] = [
    "application", "appVersion", "company", "docSecurity", "hiddenSlides", "hyperlinkBase",
    "hyperlinksChanged", "linksUpToDate", "manager", "mMClips", "pages", "presentationFormat",
    "scaleCrop", "sharedDoc", "slides", "template", "totalTime", "wordCount", "numberCharacters",
];

/// 8 个恒 `null` 的键（不变量 2/3/4）
const KEYS_ALWAYS_NULL_8: [&str; 8] = [
    "characters", "digSig", "headingPairs", "hLinks", "lines", "notes", "paragraphs", "titlesOfParts",
];

/// DE1 / 主路径：造一份齐全 `app.xml` ⇒ **19 个可写键全有值**、8 个恒 null 键仍 `null`。
#[test]
fn de1_all_settable_present() {
    let xml = doc(concat!(
        "<Application>Microsoft Office Word</Application>",
        "<AppVersion>16.0000</AppVersion>",
        "<Company>ioffice</Company>",
        "<DocSecurity>0</DocSecurity>",
        "<HiddenSlides>7</HiddenSlides>",
        "<HyperlinkBase>https://example.com/</HyperlinkBase>",
        "<HyperlinksChanged>true</HyperlinksChanged>",
        "<LinksUpToDate>false</LinksUpToDate>",
        "<Manager>M</Manager>",
        "<MMClips>3</MMClips>",
        "<Pages>12</Pages>",
        "<PresentationFormat>On-screen Show</PresentationFormat>",
        "<ScaleCrop>true</ScaleCrop>",
        "<SharedDoc>false</SharedDoc>",
        "<Slides>5</Slides>",
        "<Template>Normal.dotm</Template>",
        "<TotalTime>9</TotalTime>",
        "<Words>123</Words>",
        "<CharactersWithSpaces>456</CharactersWithSpaces>",
        // 下面这些**故意存在**（丢值点/恒 null 点）：
        "<Characters>400</Characters>",
        "<Lines>20</Lines>",
        "<Paragraphs>30</Paragraphs>",
        "<Notes>2</Notes>",
        "<DigSig>sig</DigSig>",
        "<HeadingPairs>hp</HeadingPairs>",
        "<HLinks>hl</HLinks>",
        "<TitlesOfParts>top</TitlesOfParts>",
    ));
    let p = unmarshall(&xml).expect("齐全 app.xml 必须解析成功");
    for key in KEYS_SETTABLE_19 {
        assert!(value_of(&p, key).is_some(), "{key} 应被 setter 写入");
    }
    assert_eq!(value_of(&p, "application").as_deref(), Some("Microsoft Office Word"));
    assert_eq!(value_of(&p, "pages").as_deref(), Some("12"));
    assert_eq!(value_of(&p, "wordCount").as_deref(), Some("123"));
    assert_eq!(value_of(&p, "numberCharacters").as_deref(), Some("456"), "读 CharactersWithSpaces");
    assert_eq!(value_of(&p, "scaleCrop").as_deref(), Some("true"));
    for key in KEYS_ALWAYS_NULL_8 {
        assert_eq!(value_of(&p, key), None, "{key} 恒 null（不变量 2/3/4）");
    }
}

/// DE2 / 真语料 C1（`standard-hello.docx` 的 app.xml）⇒ 5 有值 / 22 `null`。
#[test]
fn de2_corpus_c1() {
    let xml = read_part(fixture_path("standard-hello.docx"), "docProps/app.xml");
    let p = unmarshall(&xml).expect("C1 的 app.xml 必须解析成功");
    assert_eq!(value_of(&p, "application").as_deref(), Some("Microsoft Office Word"));
    assert_eq!(value_of(&p, "appVersion").as_deref(), Some("16.0000"));
    assert_eq!(value_of(&p, "docSecurity").as_deref(), Some("0"));
    assert_eq!(value_of(&p, "pages").as_deref(), Some("1"));
    assert_eq!(value_of(&p, "wordCount").as_deref(), Some("2"));
    // ★丢值点：XML 里有 <Paragraphs>1</Paragraphs>，但 loadParagraphs 零调用 ⇒ null
    assert_eq!(value_of(&p, "paragraphs"), None, "丢值点（不变量 2）");
    let present: Vec<&str> = KEYS_27.iter().copied().filter(|k| value_of(&p, k).is_some()).collect();
    assert_eq!(present.len(), 5, "C1 有值项数应为 5，实测 {present:?}");
}

/// DE3 / 真语料 `sample2`（本 function 主锚点）⇒ 8 有值 / 19 `null`，含 3 个丢值点。
#[test]
fn de3_corpus_sample2() {
    let xml = read_part(sample2_path(), "docProps/app.xml");
    let p = unmarshall(&xml).expect("sample2 的 app.xml 必须解析成功");
    assert_eq!(value_of(&p, "template").as_deref(), Some("Normal.dotm"));
    assert_eq!(value_of(&p, "totalTime").as_deref(), Some("4"));
    assert_eq!(value_of(&p, "pages").as_deref(), Some("8"));
    assert_eq!(value_of(&p, "wordCount").as_deref(), Some("869"));
    assert_eq!(value_of(&p, "docSecurity").as_deref(), Some("0"));
    assert_eq!(value_of(&p, "scaleCrop").as_deref(), Some("false"));
    assert_eq!(value_of(&p, "linksUpToDate").as_deref(), Some("false"));
    assert_eq!(value_of(&p, "numberCharacters").as_deref(), Some("5814"));
    // ★3 个丢值点：XML 里都有值，视图必须为 null
    assert_eq!(value_of(&p, "characters"), None, "丢值点：<Characters>4956</Characters>（不变量 3）");
    assert_eq!(value_of(&p, "lines"), None, "丢值点：<Lines>41</Lines>（不变量 2）");
    assert_eq!(value_of(&p, "paragraphs"), None, "丢值点：<Paragraphs>11</Paragraphs>（不变量 2）");
    let present: Vec<&str> = KEYS_27.iter().copied().filter(|k| value_of(&p, k).is_some()).collect();
    assert_eq!(present.len(), 8, "sample2 有值项数应为 8，实测 {present:?}");
}

/// DE4 / 边界：只有 `<Properties/>` ⇒ 27 键全 `null`，且键**全在**。
#[test]
fn de4_all_absent() {
    let p = unmarshall(&doc("")).expect("空 Properties 必须解析成功");
    for key in KEYS_27 {
        assert_eq!(value_of(&p, key), None, "{key} 应为 null");
    }
}

/// DE5 / 丢值点：`<Characters>` 有值 ⇒ `characters` 仍 `null`。
#[test]
fn de5_characters_never_set() {
    let p = unmarshall(&doc("<Characters>4956</Characters>")).unwrap();
    assert_eq!(value_of(&p, "characters"), None, "mCharacters 无 setter 调用（不变量 3）");
}

/// DE6 / 丢值点：`<Lines>`/`<Paragraphs>`/`<Notes>` 有值 ⇒ 对应键仍 `null`。
#[test]
fn de6_lines_notes_paragraphs_never_set() {
    let p = unmarshall(&doc("<Lines>41</Lines><Paragraphs>11</Paragraphs><Notes>3</Notes>")).unwrap();
    assert_eq!(value_of(&p, "lines"), None, "loadLines 零调用");
    assert_eq!(value_of(&p, "paragraphs"), None, "loadParagraphs 零调用");
    assert_eq!(value_of(&p, "notes"), None, "loadNotes 零调用");
}

/// DE7 / `numberCharacters` 读 `<CharactersWithSpaces>`，**不读** `<Characters>`。
#[test]
fn de7_number_characters_source() {
    let with_spaces = unmarshall(&doc("<CharactersWithSpaces>5814</CharactersWithSpaces>")).unwrap();
    assert_eq!(value_of(&with_spaces, "numberCharacters").as_deref(), Some("5814"));
    let without = unmarshall(&doc("<Characters>4956</Characters>")).unwrap();
    assert_eq!(value_of(&without, "numberCharacters"), None, "不得误读 <Characters>");
}

/// DE8 / 4 个恒 null getter：即使 XML 里有对应元素也仍 `null`。
#[test]
fn de8_four_always_null() {
    let p = unmarshall(&doc(
        "<DigSig>sig</DigSig><HeadingPairs>hp</HeadingPairs><HLinks>hl</HLinks><TitlesOfParts>top</TitlesOfParts>",
    ))
    .unwrap();
    assert_eq!(value_of(&p, "digSig"), None);
    assert_eq!(value_of(&p, "headingPairs"), None);
    assert_eq!(value_of(&p, "hLinks"), None);
    assert_eq!(value_of(&p, "titlesOfParts"), None);
}

/// DE9 / 边界：空文本元素 —— **分两类**（Java 逐行，★实测口径）：
/// 整数型 `load*` 判 `value.length() > 0` ⇒ `null`；字符串型（`loadApplication` 等）**无长度判定** ⇒ `""`。
#[test]
fn de9_empty_text_null() {
    let p = unmarshall(&doc("<Words></Words><Application/><Company/><Pages> </Pages>")).unwrap();
    assert_eq!(value_of(&p, "wordCount"), None, "整数型：空文本 ⇒ null（length()>0 判定）");
    assert_eq!(
        value_of(&p, "application"),
        Some(String::new()),
        "字符串型：Java `loadApplication:196-199` 只判 null 元素 ⇒ 空文本给 \"\""
    );
    assert_eq!(value_of(&p, "company"), Some(String::new()));
    assert_eq!(value_of(&p, "pages"), None, "整数型空格文本 ⇒ trim 后解析失败 ⇒ null");
}

/// DE10 / 布尔：`false` ⇒ `Some(false)`（**不是 `None`**）。
#[test]
fn de10_bool_false_vs_none() {
    let p = unmarshall(&doc("<ScaleCrop>false</ScaleCrop><LinksUpToDate>false</LinksUpToDate>")).unwrap();
    assert!(p.getScaleCropProperty().is_some(), "false 必须区别于「没这个值」");
    assert_eq!(p.getScaleCropProperty(), Some(false));
    assert_eq!(p.getLinksUpToDateProperty(), Some(false));
    // 缺席才是 None
    let absent = unmarshall(&doc("")).unwrap();
    assert_eq!(absent.getScaleCropProperty(), None);
}

/// DE11 / 畸形布尔：`yes` ⇒ `false`（Java `new Boolean("yes")` = false，不抛错）。
#[test]
fn de11_bool_malformed() {
    let p = unmarshall(&doc("<ScaleCrop>yes</ScaleCrop>")).unwrap();
    assert_eq!(p.getScaleCropProperty(), Some(false), "非 \"true\" 一律 false（Java Boolean(String) 语义）");
    let upper = unmarshall(&doc("<ScaleCrop>TRUE</ScaleCrop>")).unwrap();
    assert_eq!(upper.getScaleCropProperty(), Some(true), "Java Boolean 不区分大小写");
}

/// DE12 / 畸形整数：`abc` ⇒ `null`（Java `TypeConversion.string2Integer` catch ⇒ null）。
#[test]
fn de12_int_malformed() {
    let p = unmarshall(&doc("<Words>abc</Words>")).unwrap();
    assert_eq!(value_of(&p, "wordCount"), None, "NumberFormatException ⇒ null（两侧一致）");
    // Java 会先 trim 再 Long.parseLong；溢出走 `l.intValue()` 窄化
    let padded = unmarshall(&doc("<Words> 42 </Words>")).unwrap();
    assert_eq!(value_of(&padded, "wordCount").as_deref(), Some("42"), "Java 先 trim 再解析");
    let sci = unmarshall(&doc("<Words>1e3</Words>")).unwrap();
    assert_eq!(value_of(&sci, "wordCount").as_deref(), Some("1000"), "Java 先 BigDecimal.toPlainString()");
}

/// DE13 / 失败：非 well-formed XML ⇒ `Err`（Java：DocumentException ⇒ IOException）。
#[test]
fn de13_malformed_xml_err() {
    let ctx = context();
    let mut input = std::io::Cursor::new(doc("<Words>2</Words>").replace("</Properties>", "").into_bytes());
    let got = ExtendedPackagePropertiesUnmarshaller::new().unmarshall(&ctx, Some(&mut input));
    assert!(got.is_err(), "截断 XML 必须返回 Err");
}

/// DE14 / 失败：空输入（0 字节）⇒ `Err`。
#[test]
fn de14_empty_input_err() {
    let ctx = context();
    let mut input = std::io::Cursor::new(Vec::new());
    let got = ExtendedPackagePropertiesUnmarshaller::new().unmarshall(&ctx, Some(&mut input));
    assert!(got.is_err(), "0 字节输入必须返回 Err");
}

/// DE15 / 失败（Java 不可达分支）：`input = None` ⇒ 不 panic、返回可解释错误。
#[test]
fn de15_input_none_no_panic() {
    let ctx = context();
    let got = ExtendedPackagePropertiesUnmarshaller::new().unmarshall(&ctx, None);
    assert!(got.is_err(), "input=None 且 context 无 zipEntry/package ⇒ Err（Java 该分支不可达）");
}

/// DE16 / 键集合**恰为 27**（不含 `mZipEntry`、不多不少）。
#[test]
fn de16_key_set_is_27() {
    assert_eq!(KEYS_27.len(), 27);
    assert_eq!(KEYS_SETTABLE_19.len() + KEYS_ALWAYS_NULL_8.len(), 27);
    let p = unmarshall(&doc("")).unwrap();
    // 逐个 getter 可调用（键集合存在性）＋ 集合切分不自相矛盾
    for key in KEYS_27 {
        let _ = value_of(&p, key);
    }
    for key in KEYS_SETTABLE_19 {
        assert!(!KEYS_ALWAYS_NULL_8.contains(&key), "{key} 不应同时属于两个集合");
    }
}

/// DE17 / 幂等/确定性：同一字节流两次 ⇒ 27 键取值完全相同。
#[test]
fn de17_deterministic() {
    let xml = doc("<Application>A</Application><Pages>3</Pages><ScaleCrop>false</ScaleCrop>");
    let a = unmarshall(&xml).unwrap();
    let b = unmarshall(&xml).unwrap();
    for key in KEYS_27 {
        assert_eq!(value_of(&a, key), value_of(&b, key), "{key} 两次解析必须一致");
    }
}

/// DE18 / 无副作用：调用前后 `context` 不变（纯函数）。
#[test]
fn de18_no_side_effect() {
    let ctx = context();
    let before = ctx.getPartName().as_ref().map(|n| n.getName().to_string());
    let mut input = std::io::Cursor::new(doc("<Pages>1</Pages>").into_bytes());
    let _ = ExtendedPackagePropertiesUnmarshaller::new().unmarshall(&ctx, Some(&mut input));
    let after = ctx.getPartName().as_ref().map(|n| n.getName().to_string());
    assert_eq!(after, before, "context 不得被 unmarshall 改写");
}

/// DE19 / 字符串原样保留（不 trim）：`<Application>` 与 `<AppVersion>` 逐字相等。
#[test]
fn de19_string_verbatim() {
    let p = unmarshall(&doc("<Application>  Microsoft Office Word  </Application><AppVersion>16.0000</AppVersion>")).unwrap();
    assert_eq!(value_of(&p, "application").as_deref(), Some("  Microsoft Office Word  "));
    assert_eq!(value_of(&p, "appVersion").as_deref(), Some("16.0000"));
}

/// §3.12 产品接线（wiring）：`XWPFPackage::open_stream` → `getParts()` → `getExtendedPackageProperties()`
/// 必须是**真解出来的值**（此前是 `new(None, name)` 空壳）。
#[test]
fn wiring_xwpfpackage_getparts_extended_properties() {
    let bytes = std::fs::read(fixture_path("standard-hello.docx")).expect("语料必须在库");
    let mut input = std::io::Cursor::new(bytes);
    let mut package = core_libs_io::io::xwpf::XWPFPackage::XWPFPackage::open_stream(&mut input);
    let _parts = package.getParts();
    let props = package.getExtendedPackageProperties();
    assert_eq!(
        props.getApplicationProperty(),
        Some("Microsoft Office Word"),
        "产品入口必须返回真实解出的 app.xml 值（空壳 ⇒ 此处为 None）"
    );
    assert_eq!(props.getAppVersionProperty(), Some("16.0000"));
    assert_eq!(props.getDocSecurityProperty(), Some(0));
    assert_eq!(props.getPagesProperty(), Some(1));
    assert_eq!(props.getWordCountProperty(), Some(2));
    assert_eq!(props.getParagraphsProperty(), None, "丢值点在产品入口同样为 null");
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

fn read_part(path: std::path::PathBuf, entry: &str) -> String {
    let bytes = std::fs::read(&path)
        .unwrap_or_else(|e| panic!("语料 {} 不可读：{e}", path.display()));
    let mut archive = zip::ZipArchive::new(std::io::Cursor::new(bytes)).expect("zip 可打开");
    let mut file = archive.by_name(entry).unwrap_or_else(|e| panic!("语料缺 {entry}：{e}"));
    let mut xml = String::new();
    std::io::Read::read_to_string(&mut file, &mut xml).expect("条目可读");
    xml
}
