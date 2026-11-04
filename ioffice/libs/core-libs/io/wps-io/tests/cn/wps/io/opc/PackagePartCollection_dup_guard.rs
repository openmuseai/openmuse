// function_id=30_function_parts_dup_guard
// status=loop — executable tests live beside src, not in features/
// cargo test --manifest-path libs/core-libs/io/wps-io/Cargo.toml --test 30_function_parts_dup_guard
// src:  libs/core-libs/io/wps-io/src/cn/wps/io/xwpf/XWPFPackage.rs（读路径门禁 Java `:178-182`）
//       libs/core-libs/io/wps-io/src/cn/wps/io/opc/PackagePartCollection.rs（集合语义）
//       libs/core-libs/io/wps-io/src/cn/wps/io/opc/OPCPackage.rs（写路径 `createPart` 门禁 Java `:291-305`）
// test: libs/core-libs/io/wps-io/tests/cn/wps/io/opc/PackagePartCollection_dup_guard.rs
//
// 本 function = **重名 part 门禁（M1.12）+ 集合级语义**。
// Java 三处相关点：
//   ① `XWPFPackage.getParts`（`:178-182`）：把 `getPartsImpl()` 的数组灌进**新**集合，逐条 `containsKey(part.getPartName())`
//      ⇒ 命中即 `throw new InvalidFormatException("A part with the name '" + … + "' already exist : … [M1.12]")`
//   ② `PackagePartCollection.put/remove`（`:17-23`）：**只是** `super.put/remove`，无门禁、无 `registerPartNameStr` 维护
//   ③ `OPCPackage.createPart`（`:291-305`，门禁 `:296-297`）：写路径的 M1.12（Rust 同名 fn 在场但**无门禁** ⇒ 见 §1.7.1）
// ★真·重名 entry 的构造方式（本轮实测）：`zip` crate 拒**同名** entry，但 Java 的
//   `ZipHelper.getOPCNameFromZipItemName`（`:60-73`）对 `word/x.xml` 与 `/word/x.xml` **映射到同一个 OPC 名**
//   ⇒ 用「带前导斜杠 / 不带」两条 entry 就能造出**同一 part 名**的真重复。

use core_libs_io::io::opc::internal::ContentTypeManager::TYPES_NAMESPACE_URI;
use core_libs_io::io::opc::OPCPackage::OPCPackage;
use core_libs_io::io::opc::PackageAccess::PackageAccess;
use core_libs_io::io::opc::PackagePartCollection::PackagePartCollection;
use core_libs_io::io::opc::PackagePartName::PackagePartName;
use core_libs_io::io::opc::PackagingURIHelper::PackagingURIHelper;
use core_libs_io::io::xwpf::XWPFPackage::XWPFPackage;
use std::io::{Cursor, Read, Write};

const XML_CT: &str = "application/xml";
const RELS_CT: &str = "application/vnd.openxmlformats-package.relationships+xml";

fn types_xml() -> String {
    format!(
        r#"<?xml version="1.0"?><Types xmlns="{TYPES_NAMESPACE_URI}"><Default Extension="xml" ContentType="{XML_CT}"/><Default Extension="rels" ContentType="{RELS_CT}"/></Types>"#
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

fn open(bytes: Vec<u8>) -> Box<OPCPackage> {
    let mut input = Cursor::new(bytes);
    Box::new(OPCPackage::from_stream(&mut input))
}

fn doc_name() -> PackagePartName {
    PackagingURIHelper::createPartName_str("/word/document.xml").unwrap()
}

/// ★重名 part 的 zip：`word/document.xml`（内容 A）与 `/word/document.xml`（内容 B）
/// ⇒ Java/Rust 的 OPC 名都是 `/word/document.xml`（`ZipHelper:60-73`）。
fn dup_zip() -> Vec<u8> {
    let types = types_xml();
    zip_bytes(&[
        ("[Content_Types].xml", types.as_bytes()),
        ("word/document.xml", b"AAAA-first"),
        ("/word/document.xml", b"BBBB-second"),
    ])
}

fn part_names(pkg: &mut OPCPackage) -> Vec<String> {
    let mut names: Vec<String> = match pkg.getPartList() {
        Some(list) => list.values().map(|p| p.getPartName().getName()).collect(),
        None => Vec::new(),
    };
    names.sort();
    names
}

/// DG-01 / C9：两条 entry 映射到**同一 part 名** ⇒ 表里该名只有 1 条（`HashMap` 键去重）。
/// ★同时钉住「M1.12 读路径门禁在两侧都**不可达**」这一实测结论（DG-07）：
/// Java 的 `getPartsImpl()` 返回的是**按 part 名做键的 map 的 values**（`:566`/`:583` 都是 `partList.put`）
/// ⇒ 到达 `XWPFPackage.getParts` 的数组里不可能出现同名两条 ⇒ 门禁 `:178-182` 是防御性死代码。
#[test]
fn dg01_two_entries_same_part_name_collapse_to_one() {
    let mut pkg = open(dup_zip());
    pkg.getParts();
    assert_eq!(
        part_names(&mut pkg),
        vec!["/word/document.xml".to_string()],
        "同一 OPC 名只保留一条（键去重）"
    );
}

/// DG-02 / 边界：同一 part 名两条 entry ⇒ 取值 = **后到者**（`HashMap::insert` 覆盖；Java `super.put` 同）。
#[test]
fn dg02_last_write_wins() {
    let mut pkg = open(dup_zip());
    pkg.getParts();
    let part = pkg.getPart(&doc_name()).expect("part");
    let mut body = String::new();
    part.getInputStream_default().unwrap().read_to_string(&mut body).unwrap();
    assert_eq!(
        body, "BBBB-second",
        "★后到者（`/word/document.xml` 那条）覆盖先到者；Java `super.put` 同语义"
    );
}

/// DG-03 / 主路径：`part_count`（表大小）= 去重后条数 ≠ entry 数（3 条 entry：CT + 2 条同名 part ⇒ 1 条 part）。
#[test]
fn dg03_part_count_is_deduped() {
    let mut pkg = open(dup_zip());
    pkg.getParts();
    assert_eq!(pkg.getPartList().map(|l| l.size()), Some(1), "entry 3 条 ⇒ part 1 条");
}

/// DG-04 / 集合语义：`put`（覆盖并返回旧值）、`get`、`remove`、`values`、`size`
/// —— Java `PackagePartCollection.put/remove` 都只是 `super` 调用（无门禁）⇒ Rust 同。
#[test]
fn dg04_collection_put_get_remove_semantics() {
    let mut collection = PackagePartCollection::new();
    assert_eq!(collection.size(), 0);
    let name = doc_name();
    let first = core_libs_io::io::opc::PackagePart::PackagePart::new_with_string(
        None,
        name.clone(),
        XML_CT,
    );
    let second = core_libs_io::io::opc::PackagePart::PackagePart::new_with_string(
        None,
        name.clone(),
        RELS_CT,
    );
    assert!(collection.put(name.clone(), first).is_none(), "首次 put 无旧值");
    let replaced = collection.put(name.clone(), second).expect("覆盖时应返回旧值");
    assert_eq!(replaced.getContentType().toString(), XML_CT, "返回的是被覆盖的那个");
    assert_eq!(collection.size(), 1, "键相同 ⇒ 不增长");
    assert_eq!(collection.get(&name).unwrap().getContentType().toString(), RELS_CT);
    assert_eq!(collection.values().count(), 1);
    assert!(collection.remove(&name).is_some());
    assert_eq!(collection.size(), 0);
    assert!(collection.get(&name).is_none());
}

/// DG-05 / ★语义差异（已登记）：Java `clone()` = `super.clone()`（`HashMap.clone` **连条目一起**浅拷贝）；
/// Rust `clone_collection` 目前返回**空 map**（`map` 未复制，注释里写明「ownership glue: empty map」）⇒ 钉住现状。
#[test]
fn dg05_clone_semantics_snapshot() {
    let mut collection = PackagePartCollection::new();
    let name = doc_name();
    collection.put(
        name.clone(),
        core_libs_io::io::opc::PackagePart::PackagePart::new_with_string(None, name.clone(), XML_CT),
    );
    assert_eq!(collection.size(), 1);
    let cloned = collection.clone_collection();
    assert_eq!(
        cloned.size(),
        0,
        "★当前实测：Rust 克隆**不带条目**（Java `super.clone()` 会带）⇒ 偏差已登记 §1.7.1 ③"
    );
}

/// DG-07 / 门禁可达性（实测 + 源码对读）：读路径 M1.12 门禁在**两侧都不可达**（见 `dg01` 注释），
/// 但 Java 原文与消息必须**原样留在 Rust 里**（规则 11：不得删注释）。
/// 本用例做两件事：①源码级断言（message 片段 + `containsKey` 检查点仍在）；
/// ②行为级断言：真重复输入**不 panic**（= 门禁未触发，与 Java 相同）。
#[test]
fn dg07_m112_guard_reachability() {
    let src = include_str!("../../../../../src/cn/wps/io/xwpf/XWPFPackage.rs");
    assert!(
        src.contains("already exist : Packages shall not contain equivalent part names"),
        "★M1.12 的消息片段必须留在源码注释里（Java `:181`）"
    );
    assert!(
        src.contains("containsKey(part.getPartName())"),
        "★门禁检查点（Java `:178`）必须留在交错模板里"
    );
    // 行为：真重复输入不报错（两侧一致 —— 去重发生在 `getPartsImpl` 的键上）
    let mut pkg = open(dup_zip());
    pkg.getParts();
    assert_eq!(pkg.getPartList().map(|l| l.size()), Some(1));
    // 且必须**没有**任何 M1.12 异常/panic：上一行能跑完即为证据（catch_unwind 由 harness 承担）
}

/// DG-09 / 确定性：同一份重名 zip 跑两次 ⇒ 表内容与取值一致（覆盖顺序稳定）。
#[test]
fn dg09_deterministic_across_runs() {
    let bytes = dup_zip();
    let mut first = open(bytes.clone());
    let mut second = open(bytes);
    first.getParts();
    second.getParts();
    assert_eq!(part_names(&mut first), part_names(&mut second));
    let mut a = String::new();
    let mut b = String::new();
    first
        .getPart(&doc_name())
        .unwrap()
        .getInputStream_default()
        .unwrap()
        .read_to_string(&mut a)
        .unwrap();
    second
        .getPart(&doc_name())
        .unwrap()
        .getInputStream_default()
        .unwrap()
        .read_to_string(&mut b)
        .unwrap();
    assert_eq!(a, b);
    assert_eq!(a, "BBBB-second");
}

/// DR-00 / 读路径经 `XWPFPackage::open_stream` 也走通（本 function 的**入口**面；不跑 `DocxReader.read()`）。
#[test]
fn dr00_xwpf_read_path_sees_the_deduped_table() {
    let mut input = Cursor::new(dup_zip());
    let mut pkg = XWPFPackage::open_stream(&mut input);
    pkg.getParts();
    let count = pkg.getPartList().map(|l| l.size());
    assert_eq!(count, Some(1), "★XWPF 读路径同样只有 1 条（`rawPartList`/重建集合的差异见 §1.7.1 ②）");
}

/// 读 panic 载荷为文本（`panic!("字面量")` ⇒ `&'static str`；带参数 ⇒ `String`）。
fn panic_message(err: Box<dyn std::any::Any + Send>) -> String {
    if let Some(s) = err.downcast_ref::<String>() {
        s.clone()
    } else if let Some(s) = err.downcast_ref::<&'static str>() {
        (*s).to_string()
    } else {
        "<非字符串 payload>".to_string()
    }
}

/// DG-08 / 写路径 M1.12（Java `OPCPackage.createPart`，门禁 `:296-297`）：
/// ① 只读包 ⇒ 先 `throwExceptionIfReadOnly`；② 首次 `createPart` 成功并**登记 CT**（Java `addContentType`）；
/// ③ 同名再来一次 ⇒ `InvalidOperationException`（Rust = panic，载荷 = Java 的整句消息）。
/// ⚠未覆盖：Java 的 `!isDeleted()` **例外分支**（同名的 part 被标删后可重建）——本仓
/// `OPCPackage::getPart`/`getPartList` 只给 `&`，测试拿不到可变 part ⇒ 无法构造「已标删」状态（§1.7.1 ⑤ 登记）。
#[test]
fn dg08_write_path_guard_and_ct_registration() {
    use core_libs_io::io::opc::internal::ContentTypeManager::ContentTypeManager;

    // ① 只读包 ⇒ 立刻失败（Java `throwExceptionIfReadOnly`）
    let mut read_only = open(zip_bytes(&[("word/document.xml", b"<w:document/>")]));
    let err = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        read_only.createPart(&doc_name(), XML_CT);
    }))
    .unwrap_err();
    let msg = panic_message(err);
    assert!(msg.contains("read-only") || msg.contains("read only"), "只读门禁消息 = {msg:?}");

    // ② 可写包：首次成功 + CT 登记
    let mut pkg = Box::new(OPCPackage::new(PackageAccess::WRITE));
    let mut manager = ContentTypeManager::new(None, None);
    manager.addDefaultContentType("xml", XML_CT);
    pkg.setContentTypeManager(manager);
    assert!(pkg.createPart(&doc_name(), RELS_CT).is_some(), "首次 createPart 应成功");
    assert_eq!(pkg.getPartList().map(|l| l.size()), Some(1));
    assert_eq!(
        pkg.getPart(&doc_name()).map(|p| p.getContentType().toString()).as_deref(),
        Some(RELS_CT),
        "★Java `this.contentTypeManager.addContentType(partName, contentType)` 必须登记（part 的 CT 取到新值）"
    );

    // ③ 同名再来 ⇒ M1.12
    let err = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        pkg.createPart(&doc_name(), XML_CT);
    }))
    .unwrap_err();
    assert_eq!(
        panic_message(err),
        "A part with the name '/word/document.xml' already exists : Packages shall not contain equivalent part names and package implementers shall neither create nor recognize packages with equivalent part names. [M1.12]",
        "★消息必须与 Java `:297` 的拼接结果逐字一致"
    );
    assert_eq!(pkg.getPartList().map(|l| l.size()), Some(1), "失败后不得改动表");
}
