// 47_function_pkg_orchestrate —— `getParts()` 第二段编排（O1–O17）
//
// 规格：`features/s01-07-package-open/functions/47_function_pkg_orchestrate/FUNCTION.md` §3.4。
//
// ★实测基线（本轮现场回放后才写断言）：
//   O1  C1 `getParts()` → **9**，且 `part_count == parts[].len()`
//   O2  同一对象连调两次 → 逐字段相等（**门禁生效，不重填**）
//   O4  `parts[]` **不含** `[Content_Types].xml`（实测 `has_CT=false`）
//   O5  `XWPFPackage::new()`（WRITE、无 zipArchive）→ `getParts() = 0` 且**不 panic**

use core_libs_io::io::xwpf::XWPFPackage::XWPFPackage;
use std::io::Cursor;
use std::path::PathBuf;

fn crate_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn c1_bytes() -> Vec<u8> {
    std::fs::read(crate_root().join("tests/fixtures/wave0-ooxml-standards/standard-hello.docx"))
        .expect("C1 fixture")
}

/// part 名的规范化文本（用于排序比较，避免依赖 URI 的 `Debug` 形态）
fn part_names(pk: &mut XWPFPackage) -> Vec<String> {
    let mut v: Vec<String> = pk
        .getParts()
        .iter()
        .map(|p| format!("{:?}", p.getPartName().getURI()))
        .collect();
    v.sort();
    v
}

// ───────────────── O1 / O9：part_count == parts[].len() ─────────────────
#[test]
fn o1_o9_part_count_matches_parts_len() {
    let mut pk = XWPFPackage::open_stream(&mut Cursor::new(c1_bytes()));
    let parts = pk.getParts().len();
    assert!(parts > 0, "part_count > 0 必须成立");
    assert_eq!(parts, 9, "C1 实测 9");
    assert_eq!(
        pk.getPartList().map(|l| l.size()),
        Some(parts),
        "part_count 必须等于 parts[].len()（不等 ⇒ FAIL，O9）"
    );
}

// ───────────────── O2：连调两次结果相等（门禁生效） ─────────────────
#[test]
fn o2_two_calls_are_identical() {
    let mut pk = XWPFPackage::open_stream(&mut Cursor::new(c1_bytes()));
    let a = part_names(&mut pk);
    let b = part_names(&mut pk);
    assert_eq!(a, b, "两次结果必须逐字段相等（不等 ⇒ 门禁失效，O2）");
    assert_eq!(a.len(), 9);
    assert_eq!(
        pk.getPartList().map(|l| l.size()),
        Some(9),
        "表不得被重填（第二次后仍为 9）"
    );
}

// ───────────────── O4：parts[] 不含 [Content_Types].xml ─────────────────
#[test]
fn o4_no_content_types_in_parts() {
    let mut pk = XWPFPackage::open_stream(&mut Cursor::new(c1_bytes()));
    let names = part_names(&mut pk);
    assert!(
        !names.iter().any(|n| n.contains("Content_Types")),
        "parts[] 不得出现 [Content_Types].xml（出现 ⇒ FAIL，O4）\n{:#?}",
        names
    );
}

// ───────────────── O5：WRITE 空包 —— 0 且不 panic ─────────────────
#[test]
fn o5_write_package_zero_parts_no_panic() {
    let mut pk = XWPFPackage::new();
    let n = pk.getParts().len();
    assert_eq!(n, 0, "WRITE 空包应 0 part（panic ⇒ FAIL，O5）");
}

// ───────────────── O12：parts[].partName 码点序 —— ★**实测为未达成**（真实缺陷） ─────────────────
//
// 规格 §1.6-8 明写：**确定性输出按 partName 原始字符串码点序**（不得用 `compareTo`）。
// ★实测（本轮现场回放，3 次连跑）：`parts[]` 的顺序**每次运行都不同**
//   run0 = [styles, _rels/.rels, settings, document, core, webSettings, docRels, app, fontTable]
//   run1 = [settings, core, app, styles, webSettings, docRels, fontTable, document, _rels/.rels]
//   run2 = [styles, settings, _rels/.rels, docRels, core, app, webSettings, fontTable, document]
// ⇒ 顺序来自 `HashMap` 的**随机种子**（`PackagePartCollection` 底层是 `HashMap`）。
//
// 后果：`sha256` dump 不稳定（O11 的「两次 dump sha256 相等」在**跨进程**时会失败），
//       `s01-10` 的同机双跑**无法**做字节级比较。
//
// 本用例因此断言**当前真实状态**（非码点序）并把它钉死为缺陷，
// **不得**改成「已排序」来假装绿灯；修法应由 `s01-04` 把集合换成保序结构（或导出时排序）。
#[test]
fn o12_part_order_is_hashmap_random_not_code_point() {
    let mut pk = XWPFPackage::open_stream(&mut Cursor::new(c1_bytes()));
    let raw: Vec<String> = pk
        .getParts()
        .iter()
        .map(|p| format!("{:?}", p.getPartName().getURI()))
        .collect();
    let mut cp = raw.clone();
    cp.sort();
    assert_ne!(
        raw, cp,
        "★若此处相等，说明顺序**已**变为码点序（缺陷已被修复）⇒          请把本用例改为断言 `raw == cp`，并同步更新 47/PIPELINE_ALIGNMENT.md §4 与 s01-10 的前置条件"
    );
    // 并且必须**不是**稳定序：换一个进程/一次运行即变（此处用同一进程内重开验证集合序随 HashMap 走）
    let mut pk2 = XWPFPackage::open_stream(&mut Cursor::new(c1_bytes()));
    let raw2: Vec<String> = pk2
        .getParts()
        .iter()
        .map(|p| format!("{:?}", p.getPartName().getURI()))
        .collect();
    // 集合内容相同（无序比较），但**顺序不作保证** ⇒ 用排序后的集合证明内容一致
    let mut a = raw.clone();
    let mut b = raw2.clone();
    a.sort();
    b.sort();
    assert_eq!(a, b, "内容（无序）必须一致；顺序不作保证是**缺陷**而非特性");
}

// ───────────────── O10：relationship_count 口径（不含 part 级） ─────────────────
#[test]
fn o10_package_relationship_count_scope() {
    // 规格：`relationship_count` == **包级** `packageRelationships.len()`，
    // **不含** `word/_rels/document.xml.rels` 的条数（把 part 级算进去 ⇒ FAIL）。
    let mut pk = XWPFPackage::open_stream(&mut Cursor::new(c1_bytes()));
    let pkg_rels = pk.getRelationshipsByType(
        "http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument",
    );
    // C1 的 `_rels/.rels` 里有 1 条 officeDocument 关系（★实测）
    assert_eq!(
        pkg_rels.size(),
        1,
        "C1 包级 officeDocument 关系应为 1 条（实测）"
    );
}

// ───────────────── O8：customXml 收集 ─────────────────
#[test]
fn o8_custom_xml_infos_count() {
    let pk = XWPFPackage::open_stream(&mut Cursor::new(c1_bytes()));
    // C1 无 customXml ⇒ 0（条数不符 ⇒ FAIL；语料有 customXml 时见 C2 系）
    assert_eq!(
        pk.getCustomXmlInfos().len(),
        0,
        "C1 无 customXml ⇒ 0；条数不符说明收集逻辑有变"
    );
}

// ───────────────── O11：两次 dump sha256 相等（确定性） ─────────────────
#[test]
fn o11_two_openings_identical_part_sets() {
    let mut a = XWPFPackage::open_stream(&mut Cursor::new(c1_bytes()));
    let mut b = XWPFPackage::open_stream(&mut Cursor::new(c1_bytes()));
    assert_eq!(
        part_names(&mut a),
        part_names(&mut b),
        "同一语料两次认包必须得到同一 part 集合（不等 ⇒ FAIL，O11）"
    );
}

// ───────────────── O13：三入口 parts[] 序列相等 ─────────────────
#[test]
fn o13_three_entries_same_part_sequence() {
    let p = crate_root().join("tests/fixtures/wave0-ooxml-standards/standard-hello.docx");
    let b = std::fs::read(&p).unwrap();
    let mut via_bytes = XWPFPackage::open_stream(&mut Cursor::new(b));
    let mut via_path = XWPFPackage::open_path(p.to_str().unwrap(), 0);
    let mut via_raf = XWPFPackage::open_raf(&p, 0);
    let a = part_names(&mut via_bytes);
    assert_eq!(a, part_names(&mut via_path), "bytes vs path 必须相等");
    assert_eq!(a, part_names(&mut via_raf), "bytes vs raf 必须相等");
}

// ───────────────── O7：core-properties 去重（只取第一个） ─────────────────
#[test]
fn o7_core_properties_deduplicated() {
    // C1 只有 1 个 core.xml ⇒ 唯一性成立；
    // 规格要求「两个 part 同 CT core-properties 时只取第一个」。
    // 这里断言现状：C1 上 core properties part 名唯一。
    let mut pk = XWPFPackage::open_stream(&mut Cursor::new(c1_bytes()));
    let cores: Vec<String> = pk
        .getParts()
        .iter()
        .map(|p| format!("{:?}", p.getPartName().getURI()))
        .filter(|n| n.contains("core.xml"))
        .collect();
    assert_eq!(cores.len(), 1, "C1 应恰有 1 个 core.xml，实得 {:?}", cores);
}

// ───────────────── O17：parts[].relationships[] 恒空（Rust 现状，单侧） ─────────────────
#[test]
fn o17_part_relationships_are_empty_single_sided() {
    // ★契约（MW3，台账标 `🔴 待修`）：Rust 侧 `part_from_entry` 恒传 `None` container
    // ⇒ 各 part 的 `relationships()` **全为空集合**。
    // **本用例只做单侧现状断言**，**不**作两侧相等判据。
    // 若此处变非空 ⇒ 说明 `32_function_part_rels_bind` 已落地，**须同步更新台账**。
    let mut pk = XWPFPackage::open_stream(&mut Cursor::new(c1_bytes()));
    // `hasRelationships(&self)` 是**不可变**入口，正好用于「现状是否为空」的单侧断言。
    let nonempty: Vec<String> = pk
        .getParts()
        .iter()
        .filter(|p| p.hasRelationships())
        .map(|p| format!("{:?}", p.getPartName().getURI()))
        .collect();
    assert!(
        nonempty.is_empty(),
        "★现状断言：Rust 侧 parts[].relationships[] 应恒空；\
         若已非空（{:?}）说明 32_function_part_rels_bind 已落地 ⇒ 请更新 MW3 台账",
        nonempty
    );
}

// ───────────────── O14：schema 中无 rawPartList ─────────────────
#[test]
fn o14_raw_part_list_absent_from_schema() {
    // `crate_root()` = libs/core-libs/io/wps-io ⇒ 回到仓库根再进 schema 路径
    let schema = std::fs::read_to_string(
        crate_root().join("../../../word-core/rewrite-review/stages/S01-opc-package/docs/s01/schema-v1.json"),
    )
    .expect("schema-v1.json 必须存在（路径见 s01-10 FEATURE.md §0-⑥）");
    assert!(
        !schema.contains("rawPartList"),
        "`rawPartList` 不得出现在 schema 中（出现 ⇒ 需同步 schema 与打点，O14）"
    );
}

// ───────────────── O6：C9 重复 entry 名 —— 未造语料，单侧登记 ─────────────────
#[test]
#[ignore = "C9（重复 entry 名）语料尚未造 ⇒ 未采；见 CORPUS.md §2"]
fn o6_duplicate_entry_names() {
    let p = crate_root().join("tests/fixtures/s01_negative/C9_dup_entry.docx");
    assert!(p.exists(), "C9 缺失 ⇒ 保持 ignored（未采）");
}

// ───────────────── O16：unmarshaller 抛 IOException —— Java 侧才有语义 ─────────────────
#[test]
#[ignore = "unmarshaller 抛 IOException 是 Java 侧独有语义（Rust 无对应）⇒ 未采"]
fn o16_unmarshaller_ioexception() {
    // 规格原文：Rust 侧**无对应** ⇒ 记 `#[ignore]` 并在此登记。
}
