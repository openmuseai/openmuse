// function_id=63_function_xwpf_assembly_loop
// cargo test --manifest-path libs/word-core/Cargo.toml --test 63_function_xwpf_assembly_loop
//
// ★M-STAGE1 P3 —— 「原始文档输入 → XWPFDocument 装配完成」闭环的**产品路径**端到端验证。
//
// 里程碑判据（`M_STAGE1_PLAN.md` §0）：右边界 = `XWPFDocument.parse()` +
// `FinishReadingWork.go()`，止于 S07 之前。本用例验证该边界在**产品入口**
// （`api/DocumentOpen::fill_text_document_from_docx`）上真实闭合，而非仅门面内可见。
//
// ★为什么要这组用例（此前的盲区）：
//   产品入口的成功判据是 `main_len(doc) > 0`；**若 SAX 路径产出为空，会静默落到
//   正则兜底**（`fill_text_document_from_xml`），而兜底同样能产出 > 0 的长度
//   ⇒ 「Facade 成功」有可能是**兜底伪装**。仅看 `ImportSource::Facade` 无法区分。
//   因此 AL1–AL3 用**逐字正文比对**证明内容来自 XWPF 装配链。
//
// 实测（本文件固化的判据）：
//   C1   3,502 B  → `"Hello, wave0!\r"`   （`<w:t>Hello, wave0!</w:t>` + 段落标记）
//   C6′    918 B  → `"Hello world\r"`
//   C2a 36,741 B  → 两段正文 + 2 个 `\r`
//
// 语料锚定见 `M_STAGE1_CORPUS.md`（含 sha256）。
#![allow(non_snake_case)]

use std::io::Read;
use std::path::PathBuf;

use word_core::cn::wps::moffice::writer::api::DocumentOpen::{
    fill_text_document_from_docx, ImportSource,
};
use word_core::cn::wps::moffice::writer::core::TextDocument::TextDocument;

fn crate_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

/// C1：真实 OOXML 标准包（3,502 B，sha256 `c8740d24b0943ac2`）。
fn c1_path() -> PathBuf {
    crate_root()
        .join("../core-libs/io/wps-io/tests/fixtures/wave0-ooxml-standards/standard-hello.docx")
}
/// C2a：多段 + 显式分页（36,741 B，sha256 `2659c77f91335f6d`）。
fn c2a_path() -> PathBuf {
    crate_root().join("tests/fixtures/03_function_line_corpus/docx/multipage.docx")
}
/// C6′：**最小**文档 —— 无 styles/settings/fontTable（918 B，sha256 `69063664483d139c`）。
fn c6_path() -> PathBuf {
    crate_root().join("tests/fixtures/p1_layout_document/hello-world.docx")
}

/// 取出 `word/document.xml` 原始文本（作为**独立于 XWPF 链**的期望值来源）。
fn document_xml(docx: &[u8]) -> String {
    let mut z = zip::ZipArchive::new(std::io::Cursor::new(docx)).expect("corpus is a zip");
    let mut xml = String::new();
    z.by_name("word/document.xml")
        .expect("corpus has word/document.xml")
        .read_to_string(&mut xml)
        .expect("utf8");
    xml
}

/// 从 `<w:t>` 直接抽出期望正文（**不经 XWPF**，用于交叉验证）。
fn expected_text(xml: &str) -> String {
    let mut out = String::new();
    let mut rest = xml;
    while let Some(i) = rest.find("<w:t") {
        let Some(gt) = rest[i..].find('>') else { break };
        let start = i + gt + 1;
        let Some(end) = rest[start..].find("</w:t>") else { break };
        out.push_str(&rest[start..start + end]);
        rest = &rest[start + end..];
    }
    out
}

/// 读取主文档正文（经 `ITextRope`，即 XWPF 装配链的**真实落点**）。
fn main_text(doc: &TextDocument) -> String {
    let m = doc.getMainDocument().expect("main subdocument");
    let n = m.getLength();
    let mut buf = vec!['\0'; n.max(0) as usize];
    let rp = m.getTextRope().expect("main text rope");
    // SAFETY: 探针性质 —— `rp` 指向 `doc` 内部活对象，生命周期覆盖本次调用；
    //         既有 `getTextRope` 签名即返回裸指针（见 §14.5 独立待办）。
    let rope = unsafe { &mut *rp };
    rope.getText(0, n, &mut buf, 0);
    buf.into_iter().collect()
}

// rust: ★P0 —— `TextDocument::new()` 返回 `Box<Self>`（`TextDocument.rs:1102`）⇒
//   返回值同样装箱。切线前既有破损，一并修正。
fn import(path: &PathBuf) -> (Box<TextDocument>, ImportSource, i32) {
    let docx = std::fs::read(path).unwrap_or_else(|e| panic!("fixture {}: {e}", path.display()));
    let xml = document_xml(&docx);
    let mut doc = TextDocument::new();
    let outcome = fill_text_document_from_docx(&mut doc, &docx, &xml).expect("import ok");
    let len = doc.getMainDocument().map(|m| m.getLength()).unwrap_or(0);
    (doc, outcome.source, len)
}

// ───────────────────────── AL1：闭环产出**正确正文**（C1） ─────────────────────────

/// AL1：产品入口产出的正文，必须**逐字等于** `document.xml` 里 `<w:t>` 的内容
/// 加上段落标记。这是「装配闭环」的核心判据 —— 非空长度不足以证明。
#[test]
fn AL1_c1_product_path_yields_exact_document_text() {
    let (doc, src, len) = import(&c1_path());
    let got = main_text(&doc);
    let docx = std::fs::read(c1_path()).unwrap();
    let want = expected_text(&document_xml(&docx));

    assert_eq!(
        src,
        ImportSource::Facade,
        "C1 必须走 XWPF 门面路径（Facade），而非正则兜底"
    );
    assert_eq!(want, "Hello, wave0!", "语料锚定：C1 的 <w:t> 内容");
    assert_eq!(
        got,
        format!("{want}\r"),
        "C1 正文必须是 <w:t> 内容 + 1 个段落标记"
    );
    assert_eq!(len, 14, "C1 main_len = 13 字符 + 1 段落标记");
}

// ──────────────────── AL2：最小文档（缺可选 part）仍闭合（C6′） ────────────────────

/// AL2：C6′ **无** styles/settings/fontTable ⇒ 验证 P1 的「缺可选 part 不阻断装配」。
#[test]
fn AL2_c6_prime_minimal_doc_still_assembles() {
    let (doc, src, len) = import(&c6_path());
    let got = main_text(&doc);
    let docx = std::fs::read(c6_path()).unwrap();
    let want = expected_text(&document_xml(&docx));

    assert_eq!(src, ImportSource::Facade, "C6′ 必须走 Facade 路径");
    assert_eq!(want, "Hello world", "语料锚定：C6′ 的 <w:t> 内容");
    assert_eq!(got, format!("{want}\r"));
    assert_eq!(len, 12);
}

// ─────────────────── AL3：多段文档的段落标记数量正确（C2a） ───────────────────

/// AL3：C2a 有 2 个 `<w:p>` ⇒ 正文须含 **2** 个段落标记，且首段内容逐字正确。
/// 段落标记数是「段落结构」是否被装配的判据（只灌文本会漏掉分页/段落边界）。
#[test]
fn AL3_c2a_multi_paragraph_structure_preserved() {
    let (doc, src, len) = import(&c2a_path());
    let got = main_text(&doc);
    let docx = std::fs::read(c2a_path()).unwrap();
    let xml = document_xml(&docx);
    let para_count = xml.matches("<w:p ").count() + xml.matches("<w:p>").count();

    assert_eq!(src, ImportSource::Facade, "C2a 必须走 Facade 路径");
    assert_eq!(
        got.matches('\r').count(),
        para_count,
        "段落标记数须等于 <w:p> 数（{para_count}）"
    );
    assert!(
        got.contains("First page content before the explicit page break."),
        "首段正文须逐字出现；实得前缀: {:?}",
        got.chars().take(80).collect::<String>()
    );
    assert_eq!(len, got.chars().count() as i32);
}

// ──────────── AL4：闭环**不依赖**正则兜底（反证：兜底不是内容来源） ────────────

/// AL4：把 `document.xml` 换成**不含任何 `<w:p>`** 的文本后走同一入口。
/// 若「Facade 成功」其实来自正则兜底，则此场景会得到非空长度（兜底也无从抽取）；
/// 反之 SAX 仍按 part 装配，长度由 document.xml 的真实结构决定。
///
/// 本用例锁定一个**可观测差异**：门面路径读的是 zip 内的 part，
/// **不是**传进来的 `xml_fallback` 字符串。
#[test]
fn AL4_facade_reads_zip_part_not_the_passed_xml_string() {
    let docx = std::fs::read(&c1_path()).unwrap();
    // 传入一个**伪造的** xml_fallback（真实 zip 里仍是 "Hello, wave0!"）。
    let fake = "<w:document><w:body><w:p><w:r><w:t>FAKE-SHOULD-NOT-APPEAR</w:t></w:r></w:p></w:body></w:document>";
    let mut doc = TextDocument::new();
    let outcome = fill_text_document_from_docx(&mut doc, &docx, fake).expect("import ok");
    let got = main_text(&doc);

    assert!(
        !got.contains("FAKE-SHOULD-NOT-APPEAR"),
        "门面路径不得采用传入的 xml_fallback 文本；实得 {got:?}"
    );
    if outcome.source == ImportSource::Facade {
        assert!(
            got.contains("Hello, wave0!"),
            "Facade 路径必须从 zip 内的 document.xml part 装配；实得 {got:?}"
        );
    }
}

// ──────────── AL5：★`SubDocument.mPLCHeader` 真实建立（修复「只读未写」缺口） ────────────

/// AL5：`SubDocument.createPLC()` 的 `case 2`（页眉页脚子文档）在 Java 里是
/// `this.mPLC.add(this.mPLCHeader = new PLCHeader(this));`。
/// 修复前 Rust 侧只写了 `let _ = self.mPLCHeader.as_ref();`（**只读未写**）
/// ⇒ `mPLCHeader` 永远为 `None`，`getPLCHeader()` 恒返回 `None`，
/// 页眉页脚的 PLC 从未建立 —— 这会让 `HdrFtrDocumentImporter` 的构造
/// （`this.mPLCHeader = this.mDocument.getPLCHeader();`）拿到 `null`。
///
/// 本用例锁定修复后的**可观测行为**：type-2（HEADER_FOOTER）子文档的
/// `getPLCHeader()` 必须为 `Some`。
#[test]
fn AL5_header_footer_subdocument_has_a_real_plc_header() {
    let mut doc = TextDocument::new();
    let sub = doc
        .createSubDocument(2)
        .expect("type-2 (HEADER_FOOTER) subdocument must exist");
    assert!(
        sub.getPLCHeader().is_some(),
        "SubDocument.createPLC() case 2 必须真实赋值 mPLCHeader（Java: this.mPLCHeader = new PLCHeader(this)）"
    );
}

/// AL5b：反证 —— type-0（MAIN）子文档**不应**有 header PLC（Java `case 0` 只建
/// `mSections` / `mFields`）。用于证明 AL5 不是「恒为 Some」的空断言。
#[test]
fn AL5b_main_subdocument_has_no_header_plc() {
    let mut doc = TextDocument::new();
    let sub = doc.createSubDocument(0).expect("type-0 (MAIN) subdocument");
    assert!(
        sub.getPLCHeader().is_none(),
        "type-0 子文档不应建立 mPLCHeader（Java case 0 无此语句）"
    );
}
