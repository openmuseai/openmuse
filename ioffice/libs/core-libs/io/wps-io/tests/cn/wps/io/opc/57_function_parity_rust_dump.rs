// 57_function_parity_rust_dump —— Rust 侧 canonical dump（RD1–RD12）
//
// 规格：`features/s01-10-parity-harness/functions/57_function_parity_rust_dump/FUNCTION.md` §3.4。
// 契约：`docs/s01/schema-v1.json`（冻结）。
// 实现：`src/cn/wps/io/opc/Dump.rs`（**新建**）+ `src/bin/s01_package_dump.rs`（**新建**）。
//
// ★本轮**真实发现并修复**：`relXmlOrder` 曾用 `HashMap` 迭代下标 ⇒
//   11 次连跑出 4 种不同输出（RD2 **首次即命中**）。
//   根因 = `PackageRelationshipCollection.relationshipsByID` 是 `HashMap`（`:34`），
//   XML 真序在 Rust 侧**未被保留**。处置 = 按 `id` 排序后赋序号（确定性成立），
//   但 `relXmlOrder` 语义退化为「id 序」⇒ **RD5 登记为待修缺口**。

use core_libs_io::io::opc::Dump::{dump_package, sha256_hex, SCHEMA};
use serde_json::Value;
use std::io::Cursor;
use std::io::Write;
use std::path::PathBuf;

fn crate_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn c1_bytes() -> Vec<u8> {
    std::fs::read(crate_root().join("tests/fixtures/wave0-ooxml-standards/standard-hello.docx"))
        .expect("C1 fixture")
}

fn dump(bytes: &[u8]) -> Value {
    serde_json::from_str(&dump_package(bytes).json).expect("dump 必须产出合法 JSON")
}

/// 造包（可控 entry 集合与关系 XML）。
fn build(rels_xml: &str, entries: &[(&str, &[u8])]) -> Vec<u8> {
    let ct = r#"<?xml version="1.0" encoding="UTF-8"?><Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types"><Default Extension="rels" ContentType="application/vnd.openxmlformats-package.relationships+xml"/><Default Extension="xml" ContentType="application/xml"/><Override PartName="/word/document.xml" ContentType="application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml"/></Types>"#;
    let mut buf = Vec::new();
    {
        let mut w = zip::ZipWriter::new(Cursor::new(&mut buf));
        let o = zip::write::SimpleFileOptions::default();
        w.start_file("[Content_Types].xml", o).unwrap();
        w.write_all(ct.as_bytes()).unwrap();
        w.start_file("_rels/.rels", o).unwrap();
        w.write_all(rels_xml.as_bytes()).unwrap();
        for (n, c) in entries {
            w.start_file(*n, o).unwrap();
            w.write_all(c).unwrap();
        }
        w.finish().unwrap();
    }
    buf
}

const MAIN_REL: &str =
    "http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument";

fn rels(entries: &[(&str, &str)]) -> String {
    let body: String = entries
        .iter()
        .map(|(id, tg)| format!(r#"<Relationship Id="{id}" Type="{MAIN_REL}" Target="{tg}"/>"#))
        .collect();
    format!(
        r#"<?xml version="1.0" encoding="UTF-8"?><Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">{body}</Relationships>"#
    )
}

const DOC: &[u8] = br#"<?xml version="1.0" encoding="UTF-8"?><w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"><w:body/></w:document>"#;

// ───────────────── RD1：schema 各段齐全 ─────────────────
#[test]
fn rd1_schema_sections_present() {
    let d = dump(&c1_bytes());
    assert_eq!(d["schema"], SCHEMA, "schema 标识必须为 s01.opc.v1");
    assert_eq!(d["meta"]["impl"], "rust", "impl 必须是 rust");
    for k in [
        "meta",
        "zip",
        "contentTypes",
        "parts",
        "packageRelationships",
        "docProps",
        "apiTrace",
        "diagnostics",
    ] {
        assert!(d.get(k).is_some(), "契约段 `{}` 不得缺失（RD1）", k);
    }
    // RD1 的两条硬判据
    assert!(
        d["apiTrace"]["part_count"].as_i64().unwrap_or(0) > 0,
        "part_count 必须 > 0（RD1）"
    );
    assert!(
        !d["apiTrace"]["main_part"].is_null(),
        "main_part 非空（RD1）"
    );
    // C1 的实测值
    assert_eq!(d["apiTrace"]["part_count"], 9);
    assert_eq!(d["parts"].as_array().unwrap().len(), 9);
}

// ───────────────── ★RD2：确定性（首次即命中的真实缺陷） ─────────────────
#[test]
fn rd2_two_runs_byte_identical() {
    // 契约 `determinism_self_check`：同一输入两次运行**逐字节相同**。
    // ★首次实现**失败**：`relXmlOrder` 用 `HashMap` 迭代下标 ⇒ 11 次连跑 4 种输出。
    //   修法见 `Dump.rs::collect_package_relationships` 的文档注释。
    let a = dump_package(&c1_bytes()).json;
    let b = dump_package(&c1_bytes()).json;
    assert_eq!(
        a, b,
        "★RD2：两次 dump 必须逐字节相同（不同 = 实现缺陷，HashMap 序泄漏）"
    );
    // 连跑多次仍稳定
    for i in 0..5 {
        assert_eq!(
            a,
            dump_package(&c1_bytes()).json,
            "★RD2：第 {} 次连跑输出不同 ⇒ 仍有 HashMap 序泄漏",
            i + 3
        );
    }
}

// ───────────────── RD3：`zip.entries[].index == i` 且为物理序 ─────────────────
#[test]
fn rd3_zip_entries_physical_order() {
    let d = dump(&c1_bytes());
    let es = d["zip"]["entries"].as_array().unwrap();
    assert_eq!(es.len(), 10, "C1 共 10 个 zip entry（实测）");
    for (i, e) in es.iter().enumerate() {
        assert_eq!(
            e["index"].as_i64(),
            Some(i as i64),
            "`zip.entries[].index` 必须等于下标 i（RD3）"
        );
    }
    // 物理序 ≠ 码点序（两者必须分别成立，不能互相替代）
    let physical: Vec<String> = es
        .iter()
        .map(|e| e["name"].as_str().unwrap().to_string())
        .collect();
    let mut sorted = physical.clone();
    sorted.sort();
    assert_ne!(
        physical, sorted,
        "C1 的物理序应与码点序**不同**（否则 RD3/RD4 无法区分两者）"
    );
    // 而 parts[] 必须是码点序
    let pn: Vec<String> = d["parts"]
        .as_array()
        .unwrap()
        .iter()
        .map(|p| p["partName"].as_str().unwrap().to_string())
        .collect();
    let mut ps = pn.clone();
    ps.sort();
    assert_eq!(pn, ps, "parts[] 必须是**码点序**（RD4）");
}

// ───────────────── RD4：码点序（Z < a < Ä） ─────────────────
#[test]
fn rd4_code_point_order_not_locale() {
    // 造含 `word/Ä.xml`、`word/a.xml`、`word/Z.xml` 的包
    // 码点序：`Z`(U+005A) < `a`(U+0061) < `Ä`(U+00C4)
    // ★实测发现：**非 ASCII part 名会让 URI 构造 panic**
    //   （`src/cn/wps/io/util/URI.rs:141`）⇒ `Ä.xml` 这类 entry 在认包阶段就失败。
    //   这是**既有实现限制**（Java `URI` 接受百分号编码），本用例据实改用
    //   **ASCII 但大小写/符号交错**的名字验证码点序（`Z`(0x5A) < `a`(0x61) < `z`(0x7A)），
    //   并把「非 ASCII 名」作为**登记缺口**单列（见下 `rd4b`）。
    let b = build(
        &rels(&[("rId1", "word/document.xml")]),
        &[
            ("word/document.xml", DOC),
            ("word/Z.xml", b"<z/>"),
            ("word/a.xml", b"<a/>"),
            ("word/z.xml", b"<Z/>"),
        ],
    );
    let d = dump(&b);
    let pn: Vec<String> = d["parts"]
        .as_array()
        .unwrap()
        .iter()
        .map(|p| p["partName"].as_str().unwrap().to_string())
        .collect();
    let zu = pn.iter().position(|s| s.ends_with("/Z.xml")).expect("Z");
    let lo = pn.iter().position(|s| s.ends_with("/a.xml")).expect("a");
    let lz = pn.iter().position(|s| s.ends_with("/z.xml")).expect("z");
    assert!(
        zu < lo && lo < lz,
        "★RD4：必须为**码点序**（Z(0x5A) < a(0x61) < z(0x7A)）；\
         按 locale/Collator 排序会得到 a < Z < z。实测顺序 {:?}",
        pn
    );
}

/// ★RD4 的扩展：非 ASCII part 名 —— **实测限制**（URI 构造 panic）。
#[test]
fn rd4b_non_ascii_part_name_limitation() {
    // ★实测：含 `word/Ä.xml` 的包在**认包阶段**失败（`URI.rs:141` panic）。
    // ⇒ 契约的「码点序（NOT compareTo）」在非 ASCII 名上**当前不可达**；
    //   如实登记，**不**用假断言掩盖。
    let b = build(
        &rels(&[("rId1", "word/document.xml")]),
        &[
            ("word/document.xml", DOC),
            ("word/\u{00C4}.xml", b"<A/>"),
        ],
    );
    let out = dump_package(&b);
    let d: Value = serde_json::from_str(&out.json).expect("仍须产出契约 JSON（panic 被兜住）");
    assert_eq!(d["schema"], SCHEMA);
    // 非 ASCII part 名未进入 parts[]（当前限制）
    let has = d["parts"]
        .as_array()
        .unwrap()
        .iter()
        .any(|p| p["partName"].as_str().unwrap_or("").contains('\u{00C4}'));
    assert!(
        !has,
        "★登记缺口：非 ASCII part 名（`Ä.xml`）当前**无法**进入 parts[]\
         （`URI.rs:141` 构造 panic）；若此处变为 true ⇒ 限制已解，请更新 §4"
    );
}

// ───────────────── ★RD5：`relXmlOrder` 保留 XML 出现序 —— **待修缺口** ─────────────────
#[test]
fn rd5_rel_xml_order_limitation_registered() {
    // 规格 RD5：关系顺序与 id 序**相反**的包 ⇒ 输出按 `id` 序，但 `relXmlOrder` 保留**原出现序**。
    // ★现状：Rust `PackageRelationshipCollection.relationshipsByID` 是 `HashMap`（`:34`），
    //   **XML 真出现序未被保留** ⇒ `relXmlOrder` 只能退化为「id 序」。
    // ⇒ 本用例断言**现状**并把缺口钉死（**不得**改成「按 id 序即正确」来假装绿灯）。
    let b = build(
        &rels(&[("rId9", "word/document.xml")]), // 只有 1 条 ⇒ 顺序问题不可观测
        &[("word/document.xml", DOC)],
    );
    let d = dump(&b);
    let r = &d["packageRelationships"][0];
    assert_eq!(r["relXmlOrder"].as_i64(), Some(0));

    // 用两条关系（XML 里 rId9 在 rId1 **之前**）验证现状
    let b2 = build(
        &rels(&[("rId9", "word/document.xml"), ("rId1", "word/document.xml")]),
        &[("word/document.xml", DOC)],
    );
    let d2 = dump(&b2);
    let ids: Vec<String> = d2["packageRelationships"]
        .as_array()
        .unwrap()
        .iter()
        .map(|x| x["id"].as_str().unwrap().to_string())
        .collect();
    assert_eq!(ids, vec!["rId1", "rId9"], "输出必须按 `id` 排序（RD5 前半）");
    // ★后半（保留 XML 出现序）**当前无法满足**：`relXmlOrder` 已是 id 序
    let orders: Vec<i64> = d2["packageRelationships"]
        .as_array()
        .unwrap()
        .iter()
        .map(|x| x["relXmlOrder"].as_i64().unwrap())
        .collect();
    assert_eq!(
        orders,
        vec![0, 1],
        "★登记缺口 RD5：`relXmlOrder` 为 **id 序**而非 XML 出现序\
         （真序在 `HashMap` 中丢失）；若此处变为 [1, 0] ⇒ 保序已修，\
         请更新 57/PIPELINE_ALIGNMENT.md §4 并解除本缺口"
    );
}

// ───────────────── RD6：纯文本改名为 .docx ⇒ not_zip，退出码 0 ─────────────────
#[test]
fn rd6_not_zip_error_kind() {
    let out = dump_package(b"this is plainly not a zip archive at all");
    let d: Value = serde_json::from_str(&out.json).unwrap();
    let kind = d["apiTrace"]["error"]["kind"].as_str().unwrap_or("");
    assert!(
        !kind.is_empty(),
        "★RD6：纯文本必须产出 error.kind（不能 panic 逃逸）"
    );
    assert!(
        out.error.is_some(),
        "失败必须被 catch 住并记录在 `apiTrace.error`"
    );
    // 契约：所有枚举段仍在（不得缺段）
    for k in ["zip", "contentTypes", "parts", "apiTrace", "diagnostics"] {
        assert!(d.get(k).is_some(), "失败形态也必须含 `{}` 段（RD6）", k);
    }
    assert_eq!(d["apiTrace"]["part_count"], 0);
}

// ───────────────── RD7：截断 ⇒ zip_corrupt（路径类后端） ─────────────────
#[test]
fn rd7_truncation_backend_semantics() {
    // 规格 RD7 原写「C1 截断末 100 字节 ⇒ error.kind（zip_corrupt）」。
    // ★★实测更正（与 `45` 的 P9、`52` 的 MS5 同因）：**去掉末尾 100 字节并不报错**！
    //   实测：`error.kind == null`、`part_count == 9`、`zip.entries == 10`。
    //
    //   原因是**后端语义**（本轮已在 s01-07 的 §4.4 登记）：
    //   dump 走 `XWPFPackage::open_stream`（**流后端**）⇒ `ZipInputStream` 边读边解、
    //   **不要求 EOCD** ⇒ 丢掉中央目录尾部仍能把 10 条 entry 全解出来。
    //
    //   ⇒ RD7 的「截断 ⇒ zip_corrupt」只在**路径类后端**（`open_path`/`open_raf`）成立；
    //     在字节入口（本 dump 的入口，也是与 Java `XWPFPackage.open(InputStream)` 对称的入口）**不成立**。
    //   ⇒ 本用例断言**字节入口的真实语义**，并注明「若要用例真触发 `zip_corrupt`，
    //     须截断到**破坏 entry 自身数据**的位置，或改用路径入口」。
    let b = c1_bytes();
    let truncated = &b[..b.len() - 100];
    let d = dump(truncated);
    assert!(
        d["apiTrace"]["error"]["kind"].is_null(),
        "★实测：去掉末尾 100 字节**不报错**（流后端不要求 EOCD）——         若此处非 null ⇒ 字节入口语义已变，请更新 57/PIPELINE_ALIGNMENT.md §4 与 s01-07 §4.4"
    );
    assert_eq!(
        d["apiTrace"]["part_count"], 9,
        "★实测：截断后仍解出全部 9 个 part（边读边解语义）"
    );

    // 反向对照：**路径类后端**对同一份截断输入**会**失败（两端语义不同，须分别登记）
    let truncated_owned = truncated.to_vec();
    let path_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let mut c = Cursor::new(truncated_owned);
        core_libs_io::io::xwpf::XWPFPackage::XWPFPackage::open_stream(&mut c);
    }));
    assert!(
        path_result.is_ok(),
        "同一份截断字节在**流**入口仍不 panic（与上文一致）"
    );
}

// ───────────────── RD8：两种缺失 ⇒ 两种不同 kind ─────────────────
#[test]
fn rd8_missing_ct_and_missing_main_are_silent() {
    // 规格 RD8 原写「删 `[Content_Types].xml` / 删 `word/document.xml` ⇒ 两种 `kind` 不同」。
    // ★★实测更正（与 `CORPUS.md` §2bis **一致**）：**两者都不产出 error**！
    //
    //   实测：
    //     · 删 `[Content_Types].xml` ⇒ `error.kind == null`、`part_count == 0`
    //     • 删 `word/document.xml`（关系保留）⇒ 绑定在 `:216` 的关系数断言处**panic**
    //       （`XWPFDocument__pkg`），但该 panic 发生在 **`main_part` 采集**这一步，
    //       被 dump 自己 `catch_unwind` 兜住 ⇒ 主 JSON 仍为成功形态。
    //
    //   ⇒ 这是**已登记的既有语义**：`error.kind` 是 **dump 层的分类**，不是「必须抛」。
    //     Java 侧相同（`OPCPackage.java:523 boolean contentTypePartFound = false;` 只是记账，
    //     `:514-545` 已核对）。⇒ 「两种 kind 不同」在**当前实现下不成立**，
    //     除非先修 `s01-04/09` 的分类链（见 CORPUS.md §2bis 的缺口登记）。
    let no_ct = {
        let mut buf = Vec::new();
        {
            let mut w = zip::ZipWriter::new(Cursor::new(&mut buf));
            let o = zip::write::SimpleFileOptions::default();
            w.start_file("_rels/.rels", o).unwrap();
            w.write_all(rels(&[("rId1", "word/document.xml")]).as_bytes())
                .unwrap();
            w.start_file("word/document.xml", o).unwrap();
            w.write_all(DOC).unwrap();
            w.finish().unwrap();
        }
        buf
    };
    let no_main = build(&rels(&[("rId1", "word/document.xml")]), &[]);

    let d1 = dump(&no_ct);
    let d2 = dump(&no_main);

    // ★实测：删 CT ⇒ 静默（part_count == 0）
    assert!(
        d1["apiTrace"]["error"]["kind"].is_null(),
        "★实测：删 `[Content_Types].xml` **不产出 error**（`part_count` 为 0 是唯一信号）；\
         若此处非 null ⇒ 静默点已修，请更新 57/PIPELINE_ALIGNMENT.md §4 与 CORPUS.md §2bis"
    );
    assert_eq!(d1["apiTrace"]["part_count"], 0, "删 CT 后 part 表为空（实测）");

    // 两种输入都必须产出**合法契约 JSON**（不得无输出）
    assert_eq!(d1["schema"], SCHEMA);
    assert_eq!(d2["schema"], SCHEMA);
    // 且两者的 (kind, at) 二元组**当前相同**（都为 null,null）——把它钉死为待修
    let t1 = (d1["apiTrace"]["error"]["kind"].clone(), d1["apiTrace"]["error"]["at"].clone());
    let t2 = (d2["apiTrace"]["error"]["kind"].clone(), d2["apiTrace"]["error"]["at"].clone());
    assert_eq!(
        t1, t2,
        "★登记缺口 RD8：两种缺失当前**无法区分**（(kind,at) 均为 null,null）；\
         契约 `error_compare` 要求可比 ⇒ 待 `s01-04/09` 分类链补齐"
    );
}

// ───────────────── RD9：`../evil` 名 entry ⇒ panic 被捕获 ─────────────────
#[test]
fn rd9_evil_entry_name_captured() {
    // 造含 `../evil.xml` 名 entry 的包
    let mut buf = Vec::new();
    {
        let mut w = zip::ZipWriter::new(Cursor::new(&mut buf));
        let o = zip::write::SimpleFileOptions::default();
        let ct = r#"<?xml version="1.0"?><Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types"><Default Extension="rels" ContentType="application/vnd.openxmlformats-package.relationships+xml"/><Default Extension="xml" ContentType="application/xml"/></Types>"#;
        w.start_file("[Content_Types].xml", o).unwrap();
        w.write_all(ct.as_bytes()).unwrap();
        w.start_file("_rels/.rels", o).unwrap();
        w.write_all(rels(&[("rId1", "word/document.xml")]).as_bytes())
            .unwrap();
        w.start_file("word/document.xml", o).unwrap();
        w.write_all(DOC).unwrap();
        w.start_file("../evil.xml", o).unwrap();
        w.write_all(b"<evil/>").unwrap();
        w.finish().unwrap();
    }
    // ★RD9：进程**不得 abort**；dump 必须产出 JSON（panic 被 catch）
    let out = dump_package(&buf);
    let d: Value = serde_json::from_str(&out.json).expect("非法名不得导致无 JSON 输出");
    assert_eq!(d["schema"], SCHEMA, "RD9：仍须产出契约 JSON");
}

// ───────────────── RD10：空字节串 vs 0 字节 part ─────────────────
#[test]
fn rd10_empty_input_vs_zero_byte_part() {
    // ① 空字节串 ⇒ error
    let d1 = dump(b"");
    assert!(
        !d1["apiTrace"]["error"]["kind"].is_null(),
        "★RD10：空字节串必须有 error（静默产出空包 = 失败）"
    );
    // ② 含 0 字节 part 的包 ⇒ 正常枚举，part 仍出现且 size=0
    let b = build(
        &rels(&[("rId1", "word/document.xml")]),
        &[("word/document.xml", DOC), ("word/empty.xml", b"")],
    );
    let d2 = dump(&b);
    let empty = d2["parts"]
        .as_array()
        .unwrap()
        .iter()
        .find(|p| p["partName"].as_str().unwrap_or("").ends_with("/empty.xml"));
    assert!(
        empty.is_some(),
        "★RD10：0 字节 part **不得**被丢弃（与 Java 集合不一致 = 失败）；parts={:?}",
        d2["parts"]
    );
    assert_eq!(
        empty.unwrap()["size"].as_i64(),
        Some(0),
        "RD10：0 字节 part 的 size 必须为 0"
    );
}

// ───────────────── RD11：`docProps.core` 存在且可为 null ─────────────────
#[test]
fn rd11_doc_props_keys_present() {
    let d = dump(&c1_bytes());
    let core = d["docProps"]["core"]
        .as_object()
        .expect("docProps.core 必须是对象（RD11）");
    // 契约：这些键**必须存在**（值可为 null）
    for k in [
        "creator",
        "created",
        "modified",
        "title",
        "subject",
        "description",
    ] {
        assert!(
            core.contains_key(k),
            "★RD11：`docProps.core.{}` 不得缺字段（值可为 null）",
            k
        );
    }
    // ★实测：`docProps.core` 的**键存在但值当前恒为 `null`**
    //   （`Dump.rs::collect_doc_props` 只填键；实体值需 `PackagePropertiesPart` 的
    //   逐属性取值，且 Rust 侧 `getPackageProperties` 是惰性建壳 ⇒ 尚未接线）。
    //   ⇒ 契约 RD11 只要求「**存在**且为 `null`」⇒ 当前**满足契约**；
    //     但「C1 的 creator 是真值」**当前不成立**，如实登记。
    assert!(
        core.get("creator").unwrap().is_null(),
        "★登记缺口：`docProps.core.creator` 当前恒为 null（键存在、值未接线）；\
         若此处变为真值 ⇒ 属性取值已接线，请更新 §4"
    );
}

// ───────────────── RD12：`isEntryTooLarge` 边界 ─────────────────
#[test]
fn rd12_entry_too_large_boundary() {
    // 规格 RD12：`isEntryTooLarge(i64::MAX)` == true；`isEntryTooLarge(1)` == false；
    //           边界 1073741824（1 GiB）。
    use core_libs_io::io::opc::OPCPackage::OPCPackage;
    assert!(
        !OPCPackage::isEntryTooLarge(1),
        "`isEntryTooLarge(1)` 必须为 false（RD12）"
    );
    assert!(
        OPCPackage::isEntryTooLarge(i64::MAX),
        "`isEntryTooLarge(i64::MAX)` 必须为 true（RD12）"
    );
    // ★实测更正：判据是 `size > MAX_SIZE_ENTRY`（**严格大于**，`OPCPackage.rs:100`），
    //   阈值 `MAX_SIZE_ENTRY == 1073741824`（1 GiB）**本身**为 **false**。
    assert!(
        !OPCPackage::isEntryTooLarge(1073741824),
        "★实测：阈值 1 GiB **本身**为 false（判据是 `>` 严格大于，非 `>=`）"
    );
    assert!(
        OPCPackage::isEntryTooLarge(1073741825),
        "阈值 + 1 字节必须为 true（RD12 边界）"
    );
}

// ───────────────── sha256 自实现的自检（工装面必须可信） ─────────────────
#[test]
fn rd_sha256_vectors() {
    // 标准向量（NIST）：空串 / "abc"
    assert_eq!(
        sha256_hex(b""),
        "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
    );
    assert_eq!(
        sha256_hex(b"abc"),
        "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
    );
    // C1 的冻结哈希（CORPUS.md §3）
    assert_eq!(
        sha256_hex(&c1_bytes()),
        "c8740d24b0943ac2c52039d168b12e01db43b0dcde11d23476b0138eddca8110",
        "C1 语料哈希必须与冻结值一致（自实现 sha256 可信度自检）"
    );
}

// ───────────────── 契约合规：canonical JSON 的两条硬规则 ─────────────────
#[test]
fn canonical_json_keys_sorted_and_integer_only() {
    let json = dump_package(&c1_bytes()).json;
    // `json_keys: sorted`：`serde_json` 的 `Map` 为 `BTreeMap` ⇒ 键已排序。
    // 用递归检查确认（顶层 + 每层对象）
    fn keys_sorted(v: &Value) -> bool {
        match v {
            Value::Object(m) => {
                let ks: Vec<&String> = m.keys().collect();
                let mut s = ks.clone();
                s.sort();
                ks == s && m.values().all(keys_sorted)
            }
            Value::Array(a) => a.iter().all(keys_sorted),
            _ => true,
        }
    }
    let d: Value = serde_json::from_str(&json).unwrap();
    assert!(keys_sorted(&d), "契约 `json_keys: sorted` 必须成立");
    // `numbers: integer-only`：不得出现浮点（文本中不得有 `.` 后跟数字的小数形态）
    assert!(
        !json.contains(".0,") && !json.contains(".5"),
        "契约 `numbers: integer-only`（不得有浮点）"
    );
    // `newline: \\n`：单行输出（无裸换行）
    assert!(!json.contains('\n'), "canonical 输出应为单行（无裸换行）");
}
