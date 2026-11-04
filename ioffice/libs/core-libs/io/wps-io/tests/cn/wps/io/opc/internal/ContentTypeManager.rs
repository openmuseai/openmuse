// function_id=03_function_opc_content_types
// status=loop — executable tests live beside src, not in features/
// cargo test --manifest-path libs/core-libs/io/wps-io/Cargo.toml --test 03_function_opc_content_types
// src:  libs/core-libs/io/wps-io/src/cn/wps/io/opc/internal/ContentTypeManager.rs
// test: libs/core-libs/io/wps-io/tests/cn/wps/io/opc/internal/ContentTypeManager.rs

// ── ★2026-09-20 测试助手：`ZipEntry` 已是**真实类型**（`ZipEntryRef` 封闭枚举），
//    `getInputStream*` 的形参不再接受 `&dyn Any` ⇒ 测试也要传真实 entry。
fn ref_of(e: &FakeZipEntry) -> ZipEntryRef {
    ZipEntryRef::Fake(e.clone())
}
use core_libs_io::io::opc::internal::ContentTypeManager::{
    ContentTypeManager, CONTENT_TYPES_PART_NAME, TYPES_NAMESPACE_URI,
};
use core_libs_io::io::opc::internal::ZipContentTypeManager::ZipContentTypeManager;
use core_libs_io::io::opc::PackagingURIHelper::PackagingURIHelper;
use std::io::Cursor;

const MAIN_CT: &str =
    "application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml";
const RELS_CT: &str = "application/vnd.openxmlformats-package.relationships+xml";
const XML_CT: &str = "application/xml";

fn sample_types_xml() -> String {
    format!(
        r#"<?xml version="1.0"?>
<Types xmlns="{TYPES_NAMESPACE_URI}">
  <Default Extension="rels" ContentType="{RELS_CT}"/>
  <Default Extension="xml" ContentType="{XML_CT}"/>
  <Override PartName="/word/document.xml" ContentType="{MAIN_CT}"/>
</Types>"#
    )
}

fn mgr_from_xml(xml: &str) -> ZipContentTypeManager {
    let mut input = Cursor::new(xml.as_bytes().to_vec());
    ZipContentTypeManager::new(Some(&mut input), None)
}

fn part(name: &str) -> core_libs_io::io::opc::PackagePartName::PackagePartName {
    PackagingURIHelper::createPartName_str(name).expect(name)
}

/// CT1 / C01,C09: Override 优先；Default 覆盖其余 .xml。
#[test]
fn ct1_override_wins_for_document_xml() {
    assert_eq!(CONTENT_TYPES_PART_NAME, "[Content_Types].xml");
    let mgr = mgr_from_xml(&sample_types_xml());
    assert_eq!(
        mgr.getContentType(&part("/word/document.xml")).as_deref(),
        Some(MAIN_CT)
    );
    assert_eq!(
        mgr.getContentType(&part("/foo.xml")).as_deref(),
        Some(XML_CT)
    );
    assert_eq!(
        mgr.getContentType(&part("/_rels/.rels")).as_deref(),
        Some(RELS_CT)
    );
}

/// CT2 / C02: 只有 Default，主文档也走扩展名。
#[test]
fn ct2_default_by_extension() {
    let xml = format!(
        r#"<Types xmlns="{TYPES_NAMESPACE_URI}">
  <Default Extension="xml" ContentType="{XML_CT}"/>
</Types>"#
    );
    let mgr = mgr_from_xml(&xml);
    assert_eq!(
        mgr.getContentType(&part("/word/document.xml")).as_deref(),
        Some(XML_CT)
    );
    assert_eq!(mgr.getContentType(&part("/foo.xml")).as_deref(), Some(XML_CT));
}

/// CT3 / C03: 无 XML 标记不得当成空表。
#[test]
#[should_panic(expected = "Can't read content types part")]
fn ct3_rejects_non_xml() {
    let _ = mgr_from_xml("not xml");
}

/// CT4 / C04: 合法非 Types 根 → 空表，查询 None（Java elementsRO 空循环）。
#[test]
fn ct4_non_types_root_is_empty_table() {
    let mgr = mgr_from_xml("<foo/>");
    assert!(mgr.getContentType(&part("/foo.xml")).is_none());
}

/// CT5 / C05: 空 Types，循环 0 次。
#[test]
fn ct5_empty_types_has_no_mappings() {
    let xml = format!(r#"<Types xmlns="{TYPES_NAMESPACE_URI}"/>"#);
    let mgr = mgr_from_xml(&xml);
    assert!(mgr.getContentType(&part("/foo.xml")).is_none());
}

/// CT6 / C06: N Default + N Override。
#[test]
fn ct6_n_defaults_and_overrides() {
    let xml = format!(
        r#"<Types xmlns="{TYPES_NAMESPACE_URI}">
  <Default Extension="xml" ContentType="{XML_CT}"/>
  <Default Extension="rels" ContentType="{RELS_CT}"/>
  <Override PartName="/word/document.xml" ContentType="{MAIN_CT}"/>
  <Override PartName="/docProps/core.xml" ContentType="application/vnd.openxmlformats-package.core-properties+xml"/>
</Types>"#
    );
    let mgr = mgr_from_xml(&xml);
    assert_eq!(
        mgr.getContentType(&part("/word/document.xml")).as_deref(),
        Some(MAIN_CT)
    );
    assert_eq!(
        mgr.getContentType(&part("/docProps/core.xml")).as_deref(),
        Some("application/vnd.openxmlformats-package.core-properties+xml")
    );
    assert_eq!(mgr.getContentType(&part("/a.xml")).as_deref(), Some(XML_CT));
    assert_eq!(mgr.getContentType(&part("/_rels/.rels")).as_deref(), Some(RELS_CT));
}

/// CT7 / C07: Default Extension 大写，查询小写扩展名仍命中。
#[test]
fn ct7_default_extension_lowercased() {
    let xml = format!(
        r#"<Types xmlns="{TYPES_NAMESPACE_URI}">
  <Default Extension="XML" ContentType="{XML_CT}"/>
</Types>"#
    );
    let mgr = mgr_from_xml(&xml);
    assert_eq!(mgr.getContentType(&part("/a.xml")).as_deref(), Some(XML_CT));
}

/// CT8 / C08: 未知 part → None。
#[test]
fn ct8_unknown_part_is_none() {
    let mgr = mgr_from_xml(&sample_types_xml());
    assert!(mgr.getContentType(&part("/word/missing.bin")).is_none());
}

/// CT10 / C10: addContentType — 新扩展进 Default；同扩展不同 CT 进 Override。
#[test]
fn ct10_add_content_type_default_then_override() {
    let mut mgr = ContentTypeManager::new(None, None);
    mgr.addContentType(&part("/a.xml"), XML_CT);
    assert_eq!(mgr.getContentType(&part("/b.xml")).as_deref(), Some(XML_CT));
    mgr.addContentType(&part("/word/document.xml"), MAIN_CT);
    assert_eq!(
        mgr.getContentType(&part("/word/document.xml")).as_deref(),
        Some(MAIN_CT)
    );
    assert_eq!(mgr.getContentType(&part("/b.xml")).as_deref(), Some(XML_CT));
}

/// CT11 / C11: isContentTypeRegister。
#[test]
fn ct11_is_content_type_register() {
    let mgr = mgr_from_xml(&sample_types_xml());
    assert!(mgr.isContentTypeRegister(MAIN_CT));
    assert!(mgr.isContentTypeRegister(XML_CT));
    assert!(!mgr.isContentTypeRegister("application/octet-stream"));
}

/// CT12 / C12: clearOverride 后主文档改走 Default；clearAll 全空。
#[test]
fn ct12_clear_override_then_all() {
    let mut mgr = mgr_from_xml(&sample_types_xml());
    mgr.clearOverrideContentTypes();
    assert_eq!(
        mgr.getContentType(&part("/word/document.xml")).as_deref(),
        Some(XML_CT)
    );
    mgr.clearAll();
    assert!(mgr.getContentType(&part("/foo.xml")).is_none());
    assert!(mgr.getContentType(&part("/word/document.xml")).is_none());
}

/// CT13 / C13: remove override 后走 Default；remove 仅 Default 的 part 删扩展。
#[test]
fn ct13_remove_content_type() {
    let mut mgr = mgr_from_xml(&sample_types_xml());
    mgr.removeContentType(&part("/word/document.xml"));
    assert_eq!(
        mgr.getContentType(&part("/word/document.xml")).as_deref(),
        Some(XML_CT)
    );
    mgr.removeContentType(&part("/foo.xml"));
    assert!(mgr.getContentType(&part("/bar.xml")).is_none());
}

/// CT14 / C14: in=None 不 parse，空表。
#[test]
fn ct14_new_without_stream_is_empty() {
    let mgr = ContentTypeManager::new(None, None);
    assert!(mgr.getContentType(&part("/foo.xml")).is_none());
    assert!(!mgr.isContentTypeRegister(XML_CT));
}

// ---------- CT15–CT20（★2026-09-18 补齐 §3.2 矩阵里原标「待建」的 5 项 + CT-11 的归一消息面）----------

/// C1 语料里的 `[Content_Types].xml` 原文：**走本 feature 自己的文件后端**取（不用第三方 zip 直接抠），
/// 顺带覆盖 `16` → `17` 的取流链。
fn c1_content_types_bytes() -> Vec<u8> {
    use core_libs_io::io::xwpf::util::ZipFileZipEntrySource::ZipFileZipEntrySource;
    use std::io::Read as _;

    let fixture = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/wave0-ooxml-standards/standard-hello.docx");
    let source = ZipFileZipEntrySource::from_path(fixture.to_str().unwrap()).expect("C1 应能打开");
    let entry = source
        .getEntries()
        .expect("C1 有 entry")
        .iter()
        .find(|e| e.getName() == CONTENT_TYPES_PART_NAME)
        .expect("C1 必有 [Content_Types].xml");
    let mut stream = source
        .getInputStream(&ref_of(entry), false)
        .expect("取流不应失败")
        .expect("应有该 entry 的流");
    let mut bytes = Vec::new();
    stream.read_to_end(&mut bytes).expect("读到底");
    bytes
}

fn panic_message<F: FnOnce() -> R + std::panic::UnwindSafe, R>(f: F) -> String {
    let payload = match std::panic::catch_unwind(f) {
        Ok(_) => panic!("本用例期望 panic，但调用正常返回"),
        Err(payload) => payload,
    };
    payload
        .downcast_ref::<String>()
        .cloned()
        .or_else(|| payload.downcast_ref::<&str>().map(|s| s.to_string()))
        .unwrap_or_default()
}

/// CT15 / CT-18: 同一份 CT 字节流解析两次 ⇒ 两张表**逐字段相等**（排序后 canonical 比较）。
#[test]
fn ct15_same_ct_bytes_parsed_twice_are_equal() {
    let bytes = c1_content_types_bytes();
    let mut first_input = Cursor::new(bytes.clone());
    let first = ZipContentTypeManager::new(Some(&mut first_input), None);
    let mut second_input = Cursor::new(bytes);
    let second = ZipContentTypeManager::new(Some(&mut second_input), None);

    let canonical_defaults = |m: &ZipContentTypeManager| {
        m.getDefaultContentTypes()
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect::<std::collections::BTreeMap<_, _>>()
    };
    let canonical_overrides = |m: &ZipContentTypeManager| {
        m.getOverrideContentTypes()
            .map(|map| {
                map.iter()
                    .map(|(k, v)| (k.clone(), v.clone()))
                    .collect::<std::collections::BTreeMap<_, _>>()
            })
            .unwrap_or_default()
    };
    assert_eq!(canonical_defaults(&first), canonical_defaults(&second));
    assert_eq!(canonical_overrides(&first), canonical_overrides(&second));
    // 空表不算「相等」的假绿：C1 的两张表都非空
    assert!(!canonical_defaults(&first).is_empty());
    assert!(!canonical_overrides(&first).is_empty());
}

/// CT16 / CT-07b: C1 真实 CT（★实测 1333 B）⇒ `Default` 4 条 / `Override` 7 条。
#[test]
fn ct16_c1_real_content_types_counts() {
    let bytes = c1_content_types_bytes();
    assert_eq!(bytes.len(), 1333, "C1 的 [Content_Types].xml 长度是语料事实");
    let mut input = Cursor::new(bytes);
    let mgr = ZipContentTypeManager::new(Some(&mut input), None);

    let defaults = mgr.getDefaultContentTypes();
    let overrides = mgr.getOverrideContentTypes().expect("有 Override");
    assert_eq!(defaults.len(), 4, "defaults: {:?}", defaults.keys());
    assert_eq!(overrides.len(), 7, "overrides: {:?}", overrides.keys());
    // 四个扩展名（★实测）与关键 Override 键
    for ext in ["rels", "xml", "png", "jpeg"] {
        assert!(defaults.contains_key(ext), "缺扩展名 {ext}");
    }
    assert_eq!(
        mgr.getContentType(&part("/word/document.xml")).as_deref(),
        Some(MAIN_CT)
    );
}

/// CT17 / CT-10: `Default` 缺 `ContentType` ⇒ Java 在 `attrContentType.getValue()` 处 **NPE**
/// （RuntimeException，**不被 ctor 的 catch(InvalidFormatException) 捕获**）⇒ Rust 同为 panic。
#[test]
fn ct17_default_missing_content_type_rejected() {
    let xml = format!(
        r#"<?xml version="1.0"?>
<Types xmlns="{TYPES_NAMESPACE_URI}">
  <Default Extension="x"/>
</Types>"#
    );
    let message = panic_message(|| mgr_from_xml(&xml));
    assert_eq!(message, "NullPointerException");
}

/// CT18 / CT-11(a): `Override` 缺 `PartName` ⇒ 同上 NPE 路径。
#[test]
fn ct18_override_missing_part_name_rejected() {
    let xml = format!(
        r#"<?xml version="1.0"?>
<Types xmlns="{TYPES_NAMESPACE_URI}">
  <Override ContentType="{XML_CT}"/>
</Types>"#
    );
    let message = panic_message(|| mgr_from_xml(&xml));
    assert_eq!(message, "NullPointerException");
}

/// CT19 / CT-12: 两条同 `PartName` 的 `Override` ⇒ **后者生效**（`HashMap.put` 语义）。
#[test]
fn ct19_duplicate_override_last_wins() {
    let xml = format!(
        r#"<?xml version="1.0"?>
<Types xmlns="{TYPES_NAMESPACE_URI}">
  <Override PartName="/word/document.xml" ContentType="{XML_CT}"/>
  <Override PartName="/word/document.xml" ContentType="{MAIN_CT}"/>
</Types>"#
    );
    let mgr = mgr_from_xml(&xml);
    assert_eq!(
        mgr.getContentType(&part("/word/document.xml")).as_deref(),
        Some(MAIN_CT),
        "后一条 Override 必须覆盖前一条"
    );
    assert_eq!(mgr.getOverrideContentTypes().unwrap().len(), 1);
}

/// CT20 / CT-11(b): `PartName` 非法（`new URI(...)` 抛 `URISyntaxException`）⇒ Java 的 ctor
/// `catch (InvalidFormatException)` **把消息归一成字面量** `"Can't read content types part !"`；
/// Rust 必须同消息（**不是** `PackagingURIHelper` 的原始 URI 报错）。
#[test]
fn ct20_illegal_part_name_normalized_message() {
    let xml = format!(
        r#"<?xml version="1.0"?>
<Types xmlns="{TYPES_NAMESPACE_URI}">
  <Override PartName="ht tp://x" ContentType="{XML_CT}"/>
</Types>"#
    );
    // 先确认这个 PartName 在 helper 层确实失败（否则本用例会退化成通不过 parse 的假绿）
    assert!(PackagingURIHelper::createPartName_str("ht tp://x").is_err());
    let message = panic_message(|| mgr_from_xml(&xml));
    assert_eq!(message, "Can't read content types part !");
}

// ---------- CT21–CT24（★2026-09-18：容器守卫与 M2.4 分支落地后补的用例）----------
//
// 覆盖 `container`（Java `protected OPCPackage container`）被真正用起来的三处代码：
//   · `removeContentType` 第一段守卫（Java `:83-94`）
//   · `removeContentType` 第二段守卫 = Rule M2.4 自检（Java `:100-110`）
//   · `getContentType` 的 M2.4 分支（Java `:151-152`）
//
// 构造：`ZipInputStreamZipEntrySource` 造**真容器**（`getPartsImpl()` 建 partList），再把 CT manager
// 绑到该容器上 —— 正是 Java `new ZipContentTypeManager(stream, this)` 的同一关系。
// ⚠️ zip 里的 `[Content_Types].xml` 必须写**完整**（否则 `getPartsImpl` 第二阶段查不到 CT ⇒ 走
//    `guessContentTypeByExtension`；若 partList 里已有该 part 且表里查不到，Java 的 M2.4 分支会直接抛）。
// ⚠️ Java 侧事实：容器里的 part 走 `getContentType` 时**要么命中、要么抛 M2.4**（见 CT24），
//    因此守卫 2 里那句 `throw new InvalidOperationException("Rule M2.4 is not respected: ...")`
//    在 Java 中**不可达**（CT23 钉住这一点）—— Rust 保留该语句以求逐语句一致。

use core_libs_io::io::opc::OPCPackage::OPCPackage;
use core_libs_io::io::opc::PackageAccess::PackageAccess;
use core_libs_io::io::xwpf::util::ZipEntrySource::{FakeZipEntry, ZipEntryRef};
use core_libs_io::io::xwpf::util::ZipInputStreamZipEntrySource::ZipInputStreamZipEntrySource;

const PNG_CT: &str = "image/png";
const JPG_CT: &str = "image/jpeg";

/// 造一个真容器：`ct_in_zip` 写进 `[Content_Types].xml`（必须完整），其余为 part 字节。
fn pkg_with(ct_in_zip: &str, parts: Vec<(&str, &[u8])>) -> OPCPackage {
    let mut entries = vec![FakeZipEntry::new(CONTENT_TYPES_PART_NAME, ct_in_zip.as_bytes().to_vec())];
    entries.extend(parts.into_iter().map(|(name, bytes)| FakeZipEntry::new(name, bytes.to_vec())));
    let mut pkg = OPCPackage::new(PackageAccess::READ);
    pkg.setZipArchive(Box::new(ZipInputStreamZipEntrySource::from_entries(entries)));
    pkg.getPartsImpl();
    pkg
}

fn types_xml(defaults: &[(&str, &str)]) -> String {
    let rows: String = defaults
        .iter()
        .map(|(ext, ct)| format!("  <Default Extension=\"{ext}\" ContentType=\"{ct}\"/>\n"))
        .collect();
    format!("<?xml version=\"1.0\"?>\n<Types xmlns=\"{TYPES_NAMESPACE_URI}\">\n{rows}</Types>")
}

/// CT21: 容器里还有**同扩展名**的其它 part ⇒ Java 不删该 Default（守卫 1 拦截侧）。
#[test]
fn ct21_default_kept_when_container_has_sibling_extension() {
    let ct_xml = types_xml(&[("xml", XML_CT), ("rels", RELS_CT)]);
    let pkg = pkg_with(
        &ct_xml,
        vec![("word/document.xml", b"doc"), ("word/header1.xml", b"hdr")],
    );
    let container = &pkg as *const OPCPackage;
    let mut input = Cursor::new(ct_xml.clone().into_bytes());
    let mut mgr = ContentTypeManager::new(Some(&mut input), Some(container));

    mgr.removeContentType(&part("/word/document.xml"));

    // 容器里 `/word/header1.xml` 的扩展名同为 `xml`（且 ≠ 被删者）⇒ 守卫命中 ⇒ Default 保留
    assert_eq!(
        mgr.getDefaultContentTypes().get("xml").map(|s| s.as_str()),
        Some(XML_CT),
        "同扩展名兄弟 part 存在时不得删 Default: {:?}",
        mgr.getDefaultContentTypes()
    );
}

/// CT22: 容器里**没有**同扩展名的 part ⇒ Java 删掉该 Default（守卫 1 放行侧）。
#[test]
fn ct22_default_removed_when_container_has_no_sibling_extension() {
    let ct_xml = types_xml(&[("jpg", JPG_CT), ("png", PNG_CT), ("rels", RELS_CT)]);
    let pkg = pkg_with(&ct_xml, vec![("word/media/image.png", b"png")]);
    let container = &pkg as *const OPCPackage;
    let mut input = Cursor::new(ct_xml.clone().into_bytes());
    let mut mgr = ContentTypeManager::new(Some(&mut input), Some(container));

    // `/word/media/image.jpg` 不在容器里 ⇒ 无同扩展名兄弟 ⇒ 允许删 `jpg` Default
    mgr.removeContentType(&part("/word/media/image.jpg"));

    assert!(!mgr.getDefaultContentTypes().contains_key("jpg"), "无兄弟 part 时应删 Default");
    assert_eq!(
        mgr.getDefaultContentTypes().get("png").map(|s| s.as_str()),
        Some(PNG_CT),
        "其它扩展名不得被牵连删除"
    );
}

/// CT23: 守卫 2 的**可达**行为 —— 表里查不到的容器 part ⇒ `getContentType` 先抛 M2.4
/// （Java `:151-152`），因此 Java 那句 `"Rule M2.4 is not respected: ..."` 不可达（死代码，Rust 保留）。
#[test]
fn ct23_rule_m2_4_self_check_reaches_m2_4_exception() {
    let ct_xml = types_xml(&[("xml", XML_CT), ("rels", RELS_CT)]);
    let pkg = pkg_with(&ct_xml, vec![("word/document.xml", b"doc")]);
    // ⚠️ 绑给 mgr 的表只认识 rels ⇒ 容器里的 `/word/document.xml` 在**本 mgr**里查不到
    let reduced = types_xml(&[("rels", RELS_CT)]);
    let container = &pkg as *const OPCPackage;
    let mut input = Cursor::new(reduced.into_bytes());
    let mut mgr = ContentTypeManager::new(Some(&mut input), Some(container));

    let message = panic_message(std::panic::AssertUnwindSafe(|| {
        mgr.removeContentType(&part("/foo.xml"))
    }));
    assert!(
        message.starts_with("Rule M2.4 exception : this error should NEVER happen"),
        "守卫 2 的可达 panic 应来自 getContentType 的 M2.4 分支（Java 里 `Rule M2.4 is not respected` 不可达）: {message}"
    );
}

/// CT24: `getContentType` 的 M2.4 分支（Java `:151-152`）——表里查不到、但容器里**确有**该 part。
#[test]
fn ct24_get_content_type_rule_m2_4_exception() {
    let ct_xml = types_xml(&[("xml", XML_CT), ("rels", RELS_CT)]);
    let pkg = pkg_with(&ct_xml, vec![("word/document.xml", b"doc")]);
    let reduced = types_xml(&[("rels", RELS_CT)]);
    let container = &pkg as *const OPCPackage;
    let mut input = Cursor::new(reduced.into_bytes());
    let mgr = ContentTypeManager::new(Some(&mut input), Some(container));

    let message = panic_message(std::panic::AssertUnwindSafe(|| {
        let _ = mgr.getContentType(&part("/word/document.xml"));
    }));
    assert_eq!(
        message,
        "Rule M2.4 exception : this error should NEVER happen, if so please send a mail to the developers team, thanks !"
    );

    // 反例：容器里**没有**的 part ⇒ 走 Java `else return null`
    assert!(mgr.getContentType(&part("/nope.xml")).is_none());

    // 反例：表里命中的 part ⇒ 直接返回值，不触发 M2.4
    assert_eq!(mgr.getContentType(&part("/word/rels-thing.rels")).map(|s| s.to_string()), Some(RELS_CT.to_string()));
}
