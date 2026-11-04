// function_id=64_function_stage1_dual_run
// cargo test --manifest-path libs/word-core/Cargo.toml --test 64_function_stage1_dual_run
//
// ★M-STAGE1 P4 —— 首份**真实语料**上的 Stage1 双端差异报告。
//
// P4 目标（`M_STAGE1_PLAN.md`）：`FinishReadingWork` 6 步 + `correctDocumentData` 子步
// + 启用 `Stage1Compare` 出首份真实双端差异报告。
//
// ★本用例的诚实边界（必须说清，避免"假装绿"）：
//   - **Rust 侧**：真实产出（真语料 → 产品入口 → `PropDump::dump`），本文件断言其**结构与稳定性**；
//   - **Java 侧**：本仓**当前无 Java 参考 dump**（全仓 0 个 `*.stage1.json`；
//     `58_function_parity_java_dump` 因缺 androidTest assets 仍为 `spec`）⇒
//     **无法做真实的双端比对**。因此本文件把 Java 侧作为**显式待补输入**处理：
//     `DR*` 用"契约 + 自比对"锁住 Rust 侧的形状，真正的 java-vs-rust 差异报告
//     由 `Stage1Compare::diff` 在 Java dump 到位后立即启用（见 `DR5`）。
//
//   这样做的原因：**不得用空 fixture 冒充双端一致**。DR5 明确断言"当前无 Java dump"，
//   使该缺口**可观测**而非被静默跳过。
#![allow(non_snake_case)]

use std::path::PathBuf;

use word_core::cn::wps::moffice::writer::api::DocumentOpen::fill_text_document_from_docx;
use word_core::cn::wps::moffice::writer::api::stage1::PropDump::{PropDump, Stage1Props};
use word_core::cn::wps::moffice::writer::api::stage1::Stage1Compare::{Stage1Compare, Stage1Doc};
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
/// C6′：最小文档 / 无 styles・settings・fontTable（918 B，sha256 `69063664483d139c`）。
fn c6_path() -> PathBuf {
    crate_root().join("tests/fixtures/p1_layout_document/hello-world.docx")
}

fn document_xml(docx: &[u8]) -> String {
    use std::io::Read;
    let mut z = zip::ZipArchive::new(std::io::Cursor::new(docx)).expect("zip");
    let mut xml = String::new();
    z.by_name("word/document.xml")
        .expect("word/document.xml")
        .read_to_string(&mut xml)
        .expect("utf8");
    xml
}

/// 产品路径导入 + Stage1 dump（P4 的真实数据流）。
// rust: ★P0 —— `TextDocument::new()` 现返回 `Box<Self>`（见 `TextDocument.rs:1102`：
//   Java `new TextDocument()` 在堆上；按值 `_create` 后再 move 会使 `SubDocument.mParent`
//   悬垂）。本测试此前按值持有 ⇒ `E0308`，属切线前既有破损，一并修正。
fn import_and_dump(path: &PathBuf) -> (Box<TextDocument>, Stage1Props) {
    let docx = std::fs::read(path).unwrap_or_else(|e| panic!("fixture {}: {e}", path.display()));
    let xml = document_xml(&docx);
    let mut doc = TextDocument::new();
    fill_text_document_from_docx(&mut doc, &docx, &xml).expect("import ok");
    let props = PropDump::dump(&doc);
    (doc, props)
}

fn main_text(doc: &TextDocument) -> String {
    let m = doc.getMainDocument().expect("main");
    let n = m.getLength();
    let mut buf = vec!['\0'; n.max(0) as usize];
    let rp = m.getTextRope().expect("rope");
    // SAFETY: 探针性质；`rp` 指向 `doc` 内活对象，生命周期覆盖本次调用。
    let rope = unsafe { &mut *rp };
    rope.getText(0, n, &mut buf, 0);
    buf.into_iter().collect()
}

// ───────────────── DR1：真实语料上产出**非空** Stage1 dump ─────────────────

/// DR1：C1 必须产出 papx/chpx。`len == 0` 说明 dump 根本没走通（假绿）。
#[test]
fn DR1_c1_produces_non_empty_stage1_dump() {
    let (_doc, props) = import_and_dump(&c1_path());
    eprintln!("DR1 C1: papx={} chpx={}", props.papx.len(), props.chpx.len());
    assert!(
        !props.papx.is_empty(),
        "C1 的 PAPX dump 不得为空（为空即 dump 未生效）"
    );
    assert!(
        !props.chpx.is_empty(),
        "C1 的 CHPX dump 不得为空（为空即 dump 未生效）"
    );
}

// ───────────────── DR2：dump 反映**段落数**（结构正确性） ─────────────────

/// DR2：C2a 有 2 个 `<w:p>` ⇒ PAPX 条目数须与之匹配。
/// 这验证 dump 走的是**真实属性表**，而非返回固定/默认值。
#[test]
fn DR2_c2a_papx_count_matches_paragraph_count() {
    let (_doc, props) = import_and_dump(&c2a_path());
    let docx = std::fs::read(c2a_path()).unwrap();
    let xml = document_xml(&docx);
    let para_count = xml.matches("<w:p ").count() + xml.matches("<w:p>").count();
    eprintln!(
        "DR2 C2a: para_count={} papx={} chpx={}",
        para_count,
        props.papx.len(),
        props.chpx.len()
    );
    assert_eq!(
        props.papx.len(),
        para_count,
        "PAPX 条目数须等于 <w:p> 数（{para_count}）"
    );
}

// ───────────────── DR3：dump 是**确定性**的（可复现） ─────────────────

/// DR3：同一输入两次 dump 必须逐字节相同。
/// 若不同，说明 dump 依赖 HashMap 迭代序之类的非确定状态
/// （本仓已知缺陷：`parts[]` 顺序随机）⇒ 双端比对将不可复现。
#[test]
fn DR3_dump_is_deterministic_across_runs() {
    let (_, a) = import_and_dump(&c1_path());
    let (_, b) = import_and_dump(&c1_path());
    assert_eq!(
        a, b,
        "同一语料两次 dump 必须完全一致（否则双端比对不可复现）"
    );

    let ja = serde_json::to_string(&a).expect("serialize a");
    let jb = serde_json::to_string(&b).expect("serialize b");
    assert_eq!(ja, jb, "序列化后的 JSON 亦须一致");
    eprintln!("DR3 C1 JSON 长度 = {}", ja.len());
}

// ───────────────── DR4：`Stage1Compare` 对**真实** dump 自比必须相等 ─────────────────

/// DR4：把同一个**真实** dump 同时当作 java/rust 两侧喂给 `Stage1Compare::diff`，
/// 必须 `Ok`。这验证比较器在真实数据形状上不会误报
/// （此前 `03_function_stage1_compare` 的 6 个用例全是内存构造的小 fixture）。
#[test]
fn DR4_compare_real_dump_with_itself_is_equal() {
    for (name, p) in [("C1", c1_path()), ("C2a", c2a_path()), ("C6'", c6_path())] {
        let (doc, props) = import_and_dump(&p);
        let text = main_text(&doc);
        let d = Stage1Doc {
            text: text.clone(),
            props: props.clone(),
        };
        match Stage1Compare::diff(&d, &d) {
            Ok(()) => eprintln!("DR4 {name}: self-diff OK (papx={} chpx={})", props.papx.len(), props.chpx.len()),
            Err(e) => panic!("DR4 {name}: 真实 dump 自比居然不等: {}", e.path),
        }
    }
}

// ───────────────── DR5：★Java 参考 dump 缺失必须**可观测** ─────────────────

/// DR5：显式断言"当前无 Java 参考 dump"这一**缺口**，使其不被静默跳过。
///
/// 一旦 `58_function_parity_java_dump` 补齐并落地 `*.stage1.json`，
/// 本用例会**失败**，提示改为真实双端比对（这正是期望行为：
/// 缺口补上时测试应当提醒我们升级判据）。
#[test]
fn DR5_java_reference_dump_absence_is_observable() {
    let dir = crate_root().join("tests/fixtures/stage1");
    let found: Vec<_> = if dir.is_dir() {
        std::fs::read_dir(&dir)
            .expect("read stage1 fixture dir")
            .filter_map(|e| e.ok())
            .map(|e| e.file_name().to_string_lossy().to_string())
            .filter(|n| n.ends_with(".stage1.json"))
            .collect()
    } else {
        Vec::new()
    };

    eprintln!(
        "DR5: Java 参考 dump 目录 = {} (存在={}), 找到 {} 个 *.stage1.json",
        dir.display(),
        dir.is_dir(),
        found.len()
    );

    assert!(
        found.is_empty(),
        "★Java 参考 dump 已到位（{:?}）⇒ 请把 DR4/DR5 升级为**真实双端比对**：\n\
         用 serde_json 反序列化 Java dump 为 Stage1Doc，与 Rust 侧 dump 调 Stage1Compare::diff，\n\
         并按 M_STAGE1_PLAN.md §4 P4 产出差异报告。",
        found
    );
}
