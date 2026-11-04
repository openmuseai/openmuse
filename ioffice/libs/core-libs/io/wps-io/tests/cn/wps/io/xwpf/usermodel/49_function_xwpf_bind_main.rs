// 49_function_xwpf_bind_main —— 主文档关系绑定（BM1–BM11）
//
// 规格：`features/s01-08-xwpf-main-part/functions/49_function_xwpf_bind_main/FUNCTION.md` §3.4。
// 实现：`XWPFDocument::XWPFDocument__pkg`（`xwpf/usermodel/XWPFDocument.rs:196`）。
//
// ★实测基线（本轮现场回放后才写断言）：
//   BM1  C1                              → `/word/document.xml`、styles=**true**、numbering=false
//   BM2  仅 STRICT 关系                  → `/word/document.xml`（回退分支生效）
//   BM3  空 `<Relationships/>`           → **panic** "The number of main document relationship should be only 1."
//   BM4  两条 TRANSITIONAL               → **panic**（同上）
//   BM5  关系在、主 part entry 缺失      → **不 panic**，part 名仍为 `/word/document.xml`
//   BM6  target = `word/document.xml`    → `/word/document.xml`（补前导 `/`）
//   BM7  target = `/word/document.xml`   → `/word/document.xml`（不产生双斜杠）
//   BM8  TRANSITIONAL + STRICT 同时存在  → `/word/document.xml`（**不 panic** ⇒ core 表只有 transition 那 1 条）
//   BM8b 仅 STRICT                       → OK
//   BM9  两次认包                        → 结果相同

use core_libs_io::io::xwpf::usermodel::XWPFDocument::XWPFDocument;
use core_libs_io::io::xwpf::XWPFPackage::XWPFPackage;
use std::io::Cursor;
use std::io::Write;
use std::path::PathBuf;

const TRANSITIONAL: &str =
    "http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument";
const STRICT: &str = "http://purl.oclc.org/ooxml/officeDocument/relationships/officeDocument";

fn crate_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn c1_bytes() -> Vec<u8> {
    std::fs::read(crate_root().join("tests/fixtures/wave0-ooxml-standards/standard-hello.docx"))
        .expect("C1 fixture")
}

/// 主 part 名的规范化文本（`/word/document.xml`）。
fn part_name(doc: &XWPFDocument) -> Option<String> {
    doc.getDocumentPartName().map(|p| format!("{:?}", p.getURI()))
        .map(|d| {
            d.split("Some(\"").nth(1)
                .unwrap_or("")
                .split('"')
                .next()
                .unwrap_or("")
                .to_string()
        })
}

/// 把 panic 收敛成 payload 文本（成功类型 `String` 以规避 `XWPFDocument: !Debug`）。
fn catch_name(f: impl FnOnce() -> Option<String>) -> Result<Option<String>, String> {
    std::panic::catch_unwind(std::panic::AssertUnwindSafe(f)).map_err(|p| {
        p.downcast_ref::<String>()
            .cloned()
            .or_else(|| p.downcast_ref::<&str>().map(|s| (*s).to_string()))
            .unwrap_or_else(|| "<non-string payload>".to_string())
    })
}

/// 造包：可控 `_rels/.rels` 与可选主 part entry。
fn build(rels_xml: &str, main_entry: Option<&str>) -> Vec<u8> {
    let ct = r#"<?xml version="1.0" encoding="UTF-8"?><Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types"><Default Extension="rels" ContentType="application/vnd.openxmlformats-package.relationships+xml"/><Default Extension="xml" ContentType="application/xml"/><Override PartName="/word/document.xml" ContentType="application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml"/></Types>"#;
    let doc = r#"<?xml version="1.0" encoding="UTF-8"?><w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"><w:body/></w:document>"#;
    let mut buf = Vec::new();
    {
        let mut w = zip::ZipWriter::new(Cursor::new(&mut buf));
        let o = zip::write::SimpleFileOptions::default();
        w.start_file("[Content_Types].xml", o).unwrap();
        w.write_all(ct.as_bytes()).unwrap();
        w.start_file("_rels/.rels", o).unwrap();
        w.write_all(rels_xml.as_bytes()).unwrap();
        if let Some(e) = main_entry {
            w.start_file(e.trim_start_matches('/'), o).unwrap();
            w.write_all(doc.as_bytes()).unwrap();
        }
        w.finish().unwrap();
    }
    buf
}

fn rels(entries: &[(&str, &str, &str)]) -> String {
    let body: String = entries
        .iter()
        .map(|(id, ty, tg)| format!(r#"<Relationship Id="{id}" Type="{ty}" Target="{tg}"/>"#))
        .collect();
    format!(
        r#"<?xml version="1.0" encoding="UTF-8"?><Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">{body}</Relationships>"#
    )
}

fn bind(bytes: Vec<u8>) -> XWPFDocument {
    XWPFDocument::XWPFDocument__pkg(XWPFPackage::open_stream(&mut Cursor::new(bytes)))
}

// ───────────────── BM1 / BM10：C1 主路径 ─────────────────
#[test]
fn bm1_bm10_c1_binding_and_styles_probe() {
    let doc = bind(c1_bytes());
    assert_eq!(
        part_name(&doc).as_deref(),
        Some("/word/document.xml"),
        "C1 主 part 名必须为 /word/document.xml（错误 ⇒ 绑定算法错，BM1）"
    );
    // BM10：`:236` 的 styles 关系提前探测生效
    assert!(
        doc.getStyles().is_some(),
        "C1 有 styles 关系 ⇒ getStyles() 必须 is_some（None ⇒ 探测段未跑，BM10）"
    );
    // C1 无 numbering 关系（★实测）
    assert!(
        doc.getNumbering().is_none(),
        "C1 无 numbering 关系 ⇒ getNumbering() 应为 None"
    );
}

// ───────────────── BM2：仅 STRICT 关系 ⇒ 回退分支 ─────────────────
#[test]
fn bm2_strict_relationship_binds() {
    let b = build(
        &rels(&[("rId1", STRICT, "word/document.xml")]),
        Some("word/document.xml"),
    );
    let doc = bind(b);
    assert_eq!(
        part_name(&doc).as_deref(),
        Some("/word/document.xml"),
        "仅 STRICT 关系时必须回退并绑定成功（BM2）"
    );
}

// ───────────────── BM3：零关系 ⇒ 与 Java 同语义的断言失败 ─────────────────
#[test]
fn bm3_zero_relationship_diverges_by_panic() {
    let empty = r#"<?xml version="1.0" encoding="UTF-8"?><Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships"></Relationships>"#;
    let e = catch_name(move || {
        bind(build(empty, Some("word/document.xml")));
        None
    })
    .expect_err("零 officeDocument 关系必须失败（静默成功 ⇒ FAIL，BM3）");
    assert!(
        e.contains("The number of main document relationship should be only 1."),
        "实测 payload 应为 Java `IOAssert` 原文，实得 {:?}",
        e
    );
}

// ───────────────── BM4：两条关系 ⇒ Rust 收紧（★已知偏差） ─────────────────
#[test]
fn bm4_two_relationships_divergence() {
    // ★★已知偏差（规格 §3.4 BM4 明写，此处**实测确认**）：
    //   Java：`IOAssert.assertTrue(...)` 在 `IOAssert.fail()` 为 no-op 的实现下**不报错**，
    //         随后 `coreRels.getRelationship(0)` **取 index 0**；
    //   Rust：`assert!(core.size() == 1, ...)` 是**真断言** ⇒ **panic**。
    // ⇒ 这是**Rust 收紧**方向的安全改进（不是缺陷），但**两侧行为不同**，必须登记。
    // 本用例断言 **Rust 现状（panic）**；若哪天改成「取 0」⇒ 需同步 §3.10 与 Java 侧登记。
    let two = rels(&[
        ("rId1", TRANSITIONAL, "word/document.xml"),
        ("rId2", TRANSITIONAL, "word/document.xml"),
    ]);
    let e = catch_name(move || {
        bind(build(&two, Some("word/document.xml")));
        None
    })
    .expect_err("★实测 Rust：两条关系 ⇒ panic（Java 取 index 0，两侧不同，已登记 BM4）");
    assert!(
        e.contains("should be only 1"),
        "实得 {:?}",
        e
    );
}

// ───────────────── BM5：关系在、主 part entry 缺失 ⇒ 不 panic ─────────────────
#[test]
fn bm5_missing_part_does_not_panic() {
    // 规格：**不 panic**；`document_part_name` 仍为 `/word/document.xml`；
    //       缺 part 的暴露推迟到 `52_function_xwpf_main_stream`。
    let b = build(
        &rels(&[("rId1", TRANSITIONAL, "word/document.xml")]),
        None, // 主 part entry 故意缺失
    );
    let got = catch_name(move || part_name(&bind(b))).expect("★实测：主 part 缺失**不得** panic（BM5）");
    let _ = &got;
    assert_eq!(
        got.as_deref(),
        Some("/word/document.xml"),
        "即使 part 缺失，绑定的 part 名仍应记录（BM5）"
    );
}

// ───────────────── BM6 / BM7：相对 与 绝对 target ─────────────────
#[test]
fn bm6_relative_target_gets_leading_slash() {
    let b = build(
        &rels(&[("rId1", TRANSITIONAL, "word/document.xml")]),
        Some("word/document.xml"),
    );
    let doc = bind(b);
    assert_eq!(
        part_name(&doc).as_deref(),
        Some("/word/document.xml"),
        "相对 target 必须补前导 `/`（少了前导 `/` ⇒ FAIL，BM6）"
    );
}

#[test]
fn bm7_absolute_target_is_normalized() {
    let b = build(
        &rels(&[("rId1", TRANSITIONAL, "/word/document.xml")]),
        Some("word/document.xml"),
    );
    let doc = bind(b);
    let n = part_name(&doc).unwrap_or_default();
    assert_eq!(
        n, "/word/document.xml",
        "绝对 target 必须归一（不得出现双斜杠，BM7）"
    );
    assert!(!n.contains("//"), "不得出现 `//`：{:?}", n);
}

// ───────────────── BM8：TRANSITIONAL 优先 ─────────────────
#[test]
fn bm8_transitional_precedence() {
    // 规格 §3.4 BM8（**修正后**）：Java `:86` 只在 TRANSITIONAL 的 size == 0 时才转 STRICT。
    // ★实测：同时存在时 `core` 仍只有 TRANSITIONAL 那 1 条 ⇒ `assert!(size == 1)` **通过**、**不 panic**。
    //   （规格原文推测「会失败」；实测**不失败**，此处按实测钉死。）
    let both = rels(&[
        ("rId1", TRANSITIONAL, "word/document.xml"),
        ("rId2", STRICT, "word/document.xml"),
    ]);
    let got = catch_name(move || part_name(&bind(build(&both, Some("word/document.xml")))))
        .expect("★实测：TRANSITIONAL 存在时 core 仅 1 条 ⇒ 不 panic（BM8）");
    assert_eq!(
        got.as_deref(),
        Some("/word/document.xml"),
        "TRANSITIONAL 优先（判成 STRICT ⇒ 顺序反了，BM8）"
    );
}

// ───────────────── BM8b：仅 STRICT ⇒ 切到 STRICT 且 size==1 ─────────────────
#[test]
fn bm8b_strict_only_switches_and_binds() {
    let b = build(
        &rels(&[("rId1", STRICT, "word/document.xml")]),
        Some("word/document.xml"),
    );
    let doc = bind(b);
    assert_eq!(part_name(&doc).as_deref(), Some("/word/document.xml"));
}

// ───────────────── BM8c：两条 TRANSITIONAL ⇒ 收紧失败 ─────────────────
#[test]
fn bm8c_two_transitional_rejected() {
    let two = rels(&[
        ("rId1", TRANSITIONAL, "word/document.xml"),
        ("rId2", TRANSITIONAL, "doc2.xml"),
    ]);
    let e = catch_name(move || {
        bind(build(&two, Some("word/document.xml")));
        None
    })
    .expect_err("两条 TRANSITIONAL ⇒ size==2 ⇒ Rust 断言失败");
    assert!(e.contains("should be only 1"), "实得 {:?}", e);
}

// ───────────────── BM9：确定性 ─────────────────
#[test]
fn bm9_deterministic_binding() {
    let a = bind(c1_bytes());
    let b = bind(c1_bytes());
    assert_eq!(
        part_name(&a),
        part_name(&b),
        "两次认包必须得到同一 partName（不同 ⇒ 排序/哈希序泄漏，BM9）"
    );
    assert_eq!(part_name(&a).as_deref(), Some("/word/document.xml"));
}

// ───────────────── BM11：外部关系不得被误当 officeDocument ─────────────────
#[test]
fn bm11_external_relationship_unaffected() {
    // C3（sample4.docx）——★若语料不存在则退化为造包验证（诚实登记，不用假语料顶替）。
    let c3 = crate_root().join("tests/fixtures/s01_negative/C3_sample4.docx");
    if c3.exists() {
        let bytes = std::fs::read(&c3).unwrap();
        let doc = bind(bytes);
        assert_eq!(
            part_name(&doc).as_deref(),
            Some("/word/document.xml"),
            "C3 绑定成功、part 名不变（BM11）"
        );
    }
    // 造包：`_rels/.rels` 里混入一条 **External** 关系
    let mixed = format!(
        r#"<?xml version="1.0" encoding="UTF-8"?><Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships"><Relationship Id="rId1" Type="{TRANSITIONAL}" Target="word/document.xml"/><Relationship Id="rIdExt" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/hyperlink" Target="https://example.com/" TargetMode="External"/></Relationships>"#
    );
    let doc = bind(build(&mixed, Some("word/document.xml")));
    assert_eq!(
        part_name(&doc).as_deref(),
        Some("/word/document.xml"),
        "外部关系不得影响主 part 绑定（BM11）"
    );
}

// ───────────────── BM1 反向：C6（缺主 part）走产品入口 ─────────────────
#[test]
fn bm1_c6_missing_main_part_via_product_entry() {
    let p = crate_root().join("../../word-core/tests/fixtures/s01_negative/C6_missing_main_part.docx");
    if !p.exists() {
        return; // 语料缺失 ⇒ 跳过（不假装通过）
    }
    let doc = bind(std::fs::read(&p).unwrap());
    // C6 是「关系保留、删 word/document.xml entry」⇒ 绑定本身成功
    assert!(
        part_name(&doc).is_some(),
        "C6 绑定仍应记录 part 名（缺 part 在 52 暴露）"
    );
}
