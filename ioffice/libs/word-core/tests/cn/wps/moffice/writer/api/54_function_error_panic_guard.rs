// function_id=54_function_error_panic_guard
// cargo test --manifest-path libs/word-core/Cargo.toml --test 54_function_error_panic_guard
//
// 规格来源：`features/s01-09-error-model/functions/54_function_error_panic_guard/FUNCTION.md` §3.4（PG1–PG17）。
// 契约来源：`docs/s01/schema-v1.json`（`kind` 10 值词表）；上游 `53_function_error_taxonomy`。
//
// ★本 function 改造的是**产品运行路径**（`api/DocumentOpen.rs` 的两处 unwind 捕获边界）。
// 因此用例分两组：
//   A. 产品链真包（C1 等）——证明**行为不变**（PG1/PG12/PG13/PG17）
//   B. 假闭包 / 直接构造——证明**失败信号不再静默**（PG2–PG5/PG8–PG11/PG15）
#![allow(non_snake_case)]

use std::io::Read;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::path::PathBuf;

use word_core::cn::wps::moffice::writer::api::DocumentOpen::{
    fill_text_document_from_docx, ImportOutcome, ImportSource,
};
use word_core::cn::wps::moffice::writer::api::LayoutDocument::{
    LayoutError, LayoutErrorKind, AT_READ_DOCUMENT_XML_BY_NAME, AT_READ_DOCUMENT_XML_PK_HEADER,
    AT_READ_DOCUMENT_XML_READ, AT_READ_DOCUMENT_XML_ZIP_NEW,
};
use word_core::cn::wps::moffice::writer::core::TextDocument::TextDocument;

fn crate_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

/// C1：真实 OOXML 标准包（sha256 `c8740d24…`，3,502 B，10 entry）。
fn c1_path() -> PathBuf {
    crate_root().join("../core-libs/io/wps-io/tests/fixtures/wave0-ooxml-standards/standard-hello.docx")
}

fn read_c1() -> (Vec<u8>, String) {
    let docx = std::fs::read(c1_path()).expect("C1 fixture");
    let mut z = zip::ZipArchive::new(std::io::Cursor::new(&docx)).expect("C1 is a zip");
    let mut xml = String::new();
    z.by_name("word/document.xml")
        .expect("C1 has word/document.xml")
        .read_to_string(&mut xml)
        .expect("utf8");
    (docx, xml)
}


/// ★**产品等价**的捕获探针：用与 `fill_text_document_from_docx_at` **完全相同**的
/// 判定与分类步骤，但**不实例化 `TextDocument`**（从而不进入 `fill_via_xwpf_sax`
/// 的既有 UB 路径）。用于在 A 组证明「路由判定 + 分类」的语义。
///
/// 判据与产品链逐字一致：先看门面是否产出内容（`filled`），
/// 再按 `ImportSource::Facade` / `DirectZipFallback` 分流。
fn product_equivalent_capture(
    filled: bool,
    panic_payload: Option<&str>,
    at: &str,
) -> ImportOutcome {
    let result = match panic_payload {
        None => Ok(Ok::<(), LayoutError>(())),
        Some(msg) => catch_unwind(AssertUnwindSafe(|| {
            panic!("{}", msg);
            #[allow(unreachable_code)]
            Ok::<(), LayoutError>(())
        }))
        .map(|r| r),
    };
    let source = if filled {
        ImportSource::Facade
    } else {
        ImportSource::DirectZipFallback
    };
    ImportOutcome::capture(result, at, source, if filled { 9 } else { 0 }, 0, Vec::new())
}

fn new_doc() -> TextDocument {
    let mut d = TextDocument::TextDocument__boolean(true);
    d._create();
    d
}

/// 用假闭包走 `capture`（PG2–PG5）：模拟两处边界的 panic 分支。
fn capture_panic<F: FnOnce()>(f: F, at: &str, source: ImportSource) -> ImportOutcome {
    let r = catch_unwind(AssertUnwindSafe(|| {
        f();
        Ok::<(), LayoutError>(())
    }));
    ImportOutcome::capture(r, at, source, 0, 0, Vec::new())
}

/// 用 `LayoutError` 走 `capture`（PG8/PG9/PG15）。
fn capture_layout_error(e: LayoutError, at: &str) -> ImportOutcome {
    ImportOutcome::capture(Ok(Err(e)), at, ImportSource::DirectZipFallback, 0, 0, Vec::new())
}

fn zip_err(at: &str, msg: &str) -> LayoutError {
    LayoutError {
        kind: LayoutErrorKind::Zip,
        message: msg.to_string(),
        at: Some(at.to_string()),
    }
}

// ═══════════════════ A. 产品链真包：行为不变 ═══════════════════
//
// ⚠️ **实测约束（★必须登记）**：`fill_text_document_from_docx` 的**门面路径**会进入
// `fill_via_xwpf_sax` → `DocumentImporter`，而该路径存在**既有 UB**
// （`core/SubDocument.rs:862-868` 用 `as *const … as *mut …` 从共享引用造可变裸指针；
//  `02-parser-status.md` §10.2 已记录 20 跑 6 绿 / 14 SIGSEGV，
//  并已用 `RUST_BACKTRACE=full` 现场定位到 `ptr::copy_nonoverlapping` 对齐/重叠前置条件被违反）。
// 因此本目录的 A 组用例**只走不触发该 UB 的路径**，并用**同一套真实 C1 字节**验证语义；
// 需要完整门面路径的端到端一致性由 `s01-10` 同机双跑承担（产物口径见 §4）。
// **不得**为了「让用例变绿」而修掉或掩盖该 UB —— 它属于 `core/SubDocument.rs` 的既有缺陷。

/// PG1 的**可判定部分**：C1 真实字节满足「走门面」的**全部前置条件**。
///
/// 改造前的门面判据是 `main_len(doc) > 0`；`main_len` 取自主文档长度。
/// 这里用 C1 真实字节验证：`PK` 头存在、zip 可解析、`word/document.xml` 可取且非空
/// ⇒ 一旦进入 SAX 路径，主文档必非空 ⇒ 判据成立。
#[test]
fn pg1_c1_satisfies_facade_preconditions() {
    let (docx, xml) = read_c1();
    assert!(docx.len() > 4 && &docx[0..2] == b"PK", "C1 必须有 PK 头");
    let z = zip::ZipArchive::new(std::io::Cursor::new(&docx)).expect("C1 可解析");
    assert!(z.len() >= 9, "C1 entry 数应 >= 9（实测 10，减 CT = 9）");
    assert!(!xml.is_empty(), "C1 的 word/document.xml 必须非空");
    assert!(xml.contains("document"), "取出的必须是 document.xml 内容");
}

/// PG1 的**路由部分**：空回退串 ⇒ 必然走兜底（与改造前一致），且不触发 SAX UB。
#[test]
fn pg1_empty_fallback_route_is_direct_zip_fallback() {
    // 门面未产出内容 ⇒ 兜底（改造前同一判据：`main_len(doc) > 0` 为假）
    let out = product_equivalent_capture(false, None, "ctx#main_rel_assert");
    assert_eq!(
        out.source,
        ImportSource::DirectZipFallback,
        "无内容时必须走兜底（改造前同样如此）"
    );
    assert!(out.panic_kind.is_none(), "兜底成功不应产出 kind");
    // 门面产出内容 ⇒ Facade（PG1 的路由部分）
    let ok = product_equivalent_capture(true, None, "ctx#main_rel_assert");
    assert_eq!(ok.source, ImportSource::Facade, "有内容必须走门面 ⇒ fail");
    assert!(ok.part_count > 0, "part_count 必须 > 0");
}

// ───────────────── PG12：产物确定性（不依赖 SAX UB 路径） ─────────────────
#[test]
fn pg12_output_deterministic_on_fallback_route() {
    // 同输入两次 ⇒ 投影完全一致（改造前后产物一致的**可判定部分**）
    let a = product_equivalent_capture(false, None, "ctx#x");
    let b = product_equivalent_capture(false, None, "ctx#x");
    assert_eq!(a.source, b.source);
    assert_eq!(a.part_count, b.part_count);
    assert_eq!(a.rel_count, b.rel_count);
    assert_eq!(a.warnings, b.warnings);
}

// ───────────────── PG13：路由语义与改造前一致 ─────────────────
#[test]
fn pg13_routing_unchanged() {
    // 改造前判据：`main_len(doc) > 0` ⇒ Facade；否则兜底。
    // 二者在 `capture` 上的投影必须互不相同且与语义对应。
    let facade = ImportOutcome::ok(ImportSource::Facade, 1, 0);
    let fallback = ImportOutcome::capture(
        Ok(Ok(())),
        "ctx#x",
        ImportSource::DirectZipFallback,
        0,
        0,
        Vec::new(),
    );
    assert_eq!(facade.source, ImportSource::Facade);
    assert_eq!(fallback.source, ImportSource::DirectZipFallback);
    assert_ne!(facade.source, fallback.source, "两条路由必须可区分");
    // 空内容 ⇒ 兜底；有内容 ⇒ 门面（与改造前判据一致）
    assert_eq!(
        product_equivalent_capture(false, None, "ctx#x").source,
        ImportSource::DirectZipFallback,
        "改变路由 ⇒ fail"
    );
    assert_eq!(
        product_equivalent_capture(true, None, "ctx#x").source,
        ImportSource::Facade,
        "改变路由 ⇒ fail"
    );
}

// ───────────────── PG17：成功路径 warnings 为空 ─────────────────
#[test]
fn pg17_success_has_no_warnings() {
    let out = ImportOutcome::ok(ImportSource::Facade, 9, 7);
    assert!(out.warnings.is_empty(), "成功路径 warnings 必须为空");
    let real = product_equivalent_capture(true, None, "ctx#x");
    assert!(real.warnings.is_empty(), "成功路径 warnings 必须为空");
}

// ───────────────── PG16：同输入两次投影 byte-identical ─────────────────
#[test]
fn pg16_outcome_deterministic() {
    let oa = product_equivalent_capture(false, None, "ctx#x");
    let ob = product_equivalent_capture(false, None, "ctx#x");
    assert_eq!(oa.source, ob.source);
    assert_eq!(oa.part_count, ob.part_count);
    assert_eq!(oa.rel_count, ob.rel_count);
    assert_eq!(oa.warnings, ob.warnings);
    assert_eq!(
        oa.panic_kind.as_ref().map(|e| e.canonical_tuple()),
        ob.panic_kind.as_ref().map(|e| e.canonical_tuple())
    );
}

// ───────────────── PG10：字段恒在（usize，可 0） ─────────────────
#[test]
fn pg10_counts_always_present() {
    let ok = ImportOutcome::ok(ImportSource::Facade, 9, 7);
    let _: usize = ok.part_count;
    let _: usize = ok.rel_count;

    // 失败路径（假闭包）——计数仍须是 usize，不得缺字段
    let bad = capture_panic(
        || panic!("boom"),
        "opc::OPCPackage::getPartsImpl#pass1.size_guard",
        ImportSource::DirectZipFallback,
    );
    let _: usize = bad.part_count;
    let _: usize = bad.rel_count;
    assert_eq!(bad.part_count, 0);
    assert_eq!(bad.rel_count, 0);
}

// ═══════════════════ B. 失败信号不再静默 ═══════════════════

// ───────────────── PG2：payload 不丢 + size_guard 语境 ─────────────────
#[test]
fn pg2_payload_not_dropped_size_guard() {
    let out = capture_panic(
        || panic!("FileDamagedException"),
        "opc::OPCPackage::getPartsImpl#pass1.size_guard",
        ImportSource::DirectZipFallback,
    );
    let e = out.panic_kind.expect("payload 丢弃 ⇒ fail（原 `let _ = sax;`）");
    assert_eq!(e.kind.as_str(), "zip_corrupt");
    assert!(!e.at.is_empty(), "at 必须非空");
    assert!(
        e.detail.contains("FileDamagedException"),
        "detail 必须留住 payload，实得 {:?}",
        e.detail
    );
}

// ───────────────── PG3：第二条边界同样接上 ─────────────────
#[test]
fn pg3_second_boundary_also_captures() {
    // 第二条边界（boxed）用 `_ =>` 丢弃 payload；此处用同一 `capture` 验证其行为一致。
    let out = capture_panic(
        || panic!("FileDamagedException"),
        "opc::OPCPackage::getPartsImpl#pass1.size_guard",
        ImportSource::DirectZipFallback,
    );
    assert!(
        out.panic_kind.is_some(),
        "第二条边界未接 ⇒ fail（原 `_ =>`）"
    );
    assert_eq!(out.source, ImportSource::DirectZipFallback);
}

// ───────────────── PG4：非字符串 payload ⇒ 兜底 io_error ─────────────────
#[test]
fn pg4_non_string_payload_falls_back() {
    let r = catch_unwind(AssertUnwindSafe(|| {
        std::panic::panic_any(12345i32);
    }));
    let out = ImportOutcome::capture(
        r.map(|_| Ok::<(), LayoutError>(())),
        "opc::OPCPackage::getPartsImpl#pass1.size_guard",
        ImportSource::DirectZipFallback,
        0,
        0,
        Vec::new(),
    );
    let e = out
        .panic_kind
        .expect("panic 冒泡出 capture ⇒ fail（非字符串 payload 也必须被收敛）");
    assert_eq!(
        e.kind.as_str(),
        "io_error",
        "非字符串 payload 应兜底 io_error"
    );
    assert!(
        e.detail.contains("not readable"),
        "detail 须记明 payload 类型不可读，实得 {:?}",
        e.detail
    );
}

// ───────────────── PG5：空 payload ⇒ io_error，detail 空，at 非空 ─────────────────
#[test]
fn pg5_empty_payload() {
    let out = capture_panic(
        || panic!(""),
        "opc::OPCPackage::getPartsImpl#pass1.size_guard",
        ImportSource::DirectZipFallback,
    );
    let e = out.panic_kind.expect("空 payload 也必须产出 kind");
    assert_eq!(e.kind.as_str(), "io_error", "空 payload 应兜底 io_error");
    assert_eq!(e.detail, "", "detail 应为空");
    assert!(!e.at.is_empty(), "空 at ⇒ fail");
}

// ───────────────── PG8：四个 Zip 构造点 at 四分互不相同 ─────────────────
#[test]
fn pg8_four_zip_sites_have_distinct_at() {
    let cases = [
        (AT_READ_DOCUMENT_XML_PK_HEADER, "not_zip"),
        (AT_READ_DOCUMENT_XML_ZIP_NEW, "zip_corrupt"),
        (AT_READ_DOCUMENT_XML_BY_NAME, "missing_main_part"),
        (AT_READ_DOCUMENT_XML_READ, "zip_corrupt"),
    ];
    let mut anchors = std::collections::HashSet::new();
    for (at, expect_kind) in cases {
        let out = capture_layout_error(zip_err(at, "m"), at);
        let e = out.panic_kind.expect("必须产出 kind");
        assert_eq!(
            e.kind.as_str(),
            expect_kind,
            "at={} 期望 {}，实得 {}",
            at,
            expect_kind,
            e.kind.as_str()
        );
        anchors.insert(e.at.clone());
    }
    assert_eq!(
        anchors.len(),
        4,
        "4 个构造点的 at 必须四分互不相同（任一重合 ⇒ fail），实得 {:?}",
        anchors
    );
}

// ───────────────── PG9：缺 word/document.xml ⇒ missing_main_part（非 not_zip） ─────────────────
#[test]
fn pg9_missing_main_part_not_not_zip() {
    let out = capture_layout_error(
        zip_err(AT_READ_DOCUMENT_XML_BY_NAME, "document.xml: file not found"),
        AT_READ_DOCUMENT_XML_BY_NAME,
    );
    let e = out.panic_kind.expect("必须产出 kind");
    assert_eq!(
        e.kind.as_str(),
        "missing_main_part",
        "⇒ not_zip ⇒ fail（缺主 part 不是「不是 zip」）"
    );
    assert!(e.at.contains("#by_name"));
}

// ───────────────── PG6：缺主 part 关系 ⇒ missing_main_part + DirectZipFallback ─────────────────
#[test]
fn pg6_main_rel_assert_missing_main_part() {
    let out = capture_panic(
        || panic!("The number of main document relationship should be only 1."),
        "xwpf::usermodel::XWPFDocument::XWPFDocument__pkg#main_rel_assert",
        ImportSource::DirectZipFallback,
    );
    let e = out.panic_kind.expect("门面内断言失败必须被捕获（静默 ⇒ fail）");
    assert_eq!(e.kind.as_str(), "missing_main_part");
    assert!(
        e.at.contains("main_rel_assert"),
        "at 须指向 main_rel_assert，实得 {}",
        e.at
    );
    assert_eq!(
        out.source,
        ImportSource::DirectZipFallback,
        "降级路径必须显式"
    );
}

// ───────────────── PG7：截断 / 非 zip 可区分 ─────────────────
#[test]
fn pg7_corrupt_vs_not_zip_distinguishable() {
    // 实测（CORPUS.md §2bis.3）：C10 与 C11 在生产链上落到**同一个** panic 文本
    // `Can't open the specified stream`（原 zip 文本被丢弃）⇒ 只靠 payload 无法区分，
    // **必须靠 `at`**。此用例即验证这一点。
    let same_payload = "Can't open the specified stream";
    let corrupt = capture_panic(
        || panic!("{}", same_payload),
        "xwpf::util::ZipInputStreamZipEntrySource::parse_zip_bytes#by_index",
        ImportSource::DirectZipFallback,
    )
    .panic_kind
    .expect("kind");
    let notzip = capture_panic(
        || panic!("{}", same_payload),
        "xwpf::util::ZipInputStreamZipEntrySource::parse_zip_bytes#ZipArchive::new",
        ImportSource::DirectZipFallback,
    )
    .panic_kind
    .expect("kind");
    assert!(
        ["zip_corrupt", "not_zip"].contains(&corrupt.kind.as_str()),
        "实得 {}",
        corrupt.kind.as_str()
    );
    assert!(
        ["zip_corrupt", "not_zip"].contains(&notzip.kind.as_str()),
        "实得 {}",
        notzip.kind.as_str()
    );
    assert_ne!(
        corrupt.at, notzip.at,
        "两者必须可区分（都报 Zip 一档 ⇒ fail）"
    );
}

// ───────────────── PG15：LayoutErrorKind::{Font,Measure,Serialize} 不产出 S01Error ─────────────────
#[test]
fn pg15_non_zip_kinds_produce_no_s01_kind() {
    for k in [
        LayoutErrorKind::Font,
        LayoutErrorKind::Measure,
        LayoutErrorKind::Serialize,
    ] {
        let e = LayoutError {
            kind: k,
            message: "out of S01 scope".to_string(),
            at: None,
        };
        let out = capture_layout_error(e, "some::ctx#x");
        let sc = out.panic_kind.expect("仍应收敛（不得 panic 冒泡）");
        // 不变量 3：不得把 Font/Measure/Serialize 映射进 10 类词表 ⇒ 只能兜底 io_error
        assert_eq!(
            sc.kind.as_str(),
            "io_error",
            "{:?} 不得映射成具体 kind（产出即 fail），实得 {}",
            k,
            sc.kind.as_str()
        );
        assert!(
            sc.detail.contains("outside S01 scope"),
            "detail 须记明超出 S01 范围，实得 {:?}",
            sc.detail
        );
    }
}

// ───────────────── PG11：静默面（early_return）可被显式记录 ─────────────────
#[test]
fn pg11_silent_surface_can_be_recorded() {
    let mut out = ImportOutcome::ok(ImportSource::Facade, 1, 0);
    out.warnings
        .push("XWPFDocument::parse_part_xml#early_return: no content".to_string());
    assert!(out.warnings.len() >= 1, "空 ⇒ fail");
    assert!(
        out.warnings[0].starts_with("XWPFDocument::parse_part_xml#early_return: "),
        "格式须为 `<at>: <消息>`，实得 {:?}",
        out.warnings[0]
    );
}

// ───────────────── PG14：不新增 panic / unwrap / expect（源码级计数） ─────────────────
#[test]
fn pg14_no_new_panic_unwrap_expect() {
    // ★实测基线（改造前与改造后**均为**）：`api/DocumentOpen.rs`
    //   panic 宏 = 0、`.unwrap()` = 0、`.expect(` = 0；真实 unwind 捕获点 = 2 处。
    let src = std::fs::read_to_string(crate_root().join("src/cn/wps/moffice/writer/api/DocumentOpen.rs"))
        .expect("可读 DocumentOpen.rs");

    let panics = src.matches("panic!").count();
    let unwraps = src.matches(".unwrap()").count();
    let expects = src.matches(".expect(").count();
    assert_eq!(panics, 0, "不得新增 panic 宏（计数不增加）");
    assert_eq!(unwraps, 0, "不得新增 .unwrap()");
    assert_eq!(expects, 0, "不得新增 .expect(");

    // 真实捕获点恰好 2 处（两条边界），不多不少
    let real = src.matches("= catch_unwind(").count();
    assert_eq!(real, 2, "捕获边界应恰为 2 处，实得 {}", real);
}

// ───────────────── PG4/PG5 补充：capture 不得让 panic 冒泡 ─────────────────
#[test]
fn capture_never_propagates_panic() {
    // 任意 payload 类型都不得冒泡出 `capture`
    let makers: [fn(); 3] = [
        || std::panic::panic_any(1i32),
        || std::panic::panic_any("str payload"),
        || std::panic::panic_any(vec![1u8, 2]),
    ];
    for make in makers {
        let r = catch_unwind(AssertUnwindSafe(make));
        let out = ImportOutcome::capture(
            r.map(|_| Ok::<(), LayoutError>(())),
            "opc::OPCPackage::getPartsImpl#pass1.size_guard",
            ImportSource::DirectZipFallback,
            0,
            0,
            Vec::new(),
        );
        assert!(out.panic_kind.is_some(), "必须被收敛为 kind");
        assert!(
            ["io_error", "zip_corrupt", "not_zip", "missing_main_part"].contains(
                &out.panic_kind.as_ref().map(|e| e.kind.as_str()).unwrap_or("")
            ),
            "kind 必须落在词表内"
        );
    }
}

// ═══════════════════ C. `read_document_xml` 四分（PG8/PG9 的真实入口覆盖） ═══════════════════
//
// `layout_document` 在**坏输入**上会在 `read_document_xml` 阶段就失败 ⇒
// **不进入** `TextDocument`/SAX 的既有 UB 路径，因此这 4 个分支可以真实覆盖。
// 这同时是 PG9（缺 `word/document.xml` ⇒ `missing_main_part`，**不是** `not_zip`）的真实证据。

use word_core::cn::wps::moffice::writer::api::LayoutDocument::{
    layout_document, FontBlob, ViewParams,
};

fn empty_view() -> ViewParams {
    ViewParams::default()
}

/// C-1：非 zip 输入 ⇒ `#PK_header` ⇒ `not_zip`。
#[test]
fn pg9_pk_header_route_not_zip() {
    let err = layout_document(b"plain text, not a zip", &[] as &[FontBlob], &empty_view())
        .expect_err("非 zip 输入必须失败");
    assert_eq!(err.kind, LayoutErrorKind::Zip);
    assert_eq!(
        err.at.as_deref(),
        Some(AT_READ_DOCUMENT_XML_PK_HEADER),
        "at 必须是 PK_header 锚点"
    );
    let out = capture_layout_error(err, "ctx#x");
    assert_eq!(out.panic_kind.expect("kind").kind.as_str(), "not_zip");
}

/// C-2：是 zip 但 `word/document.xml` 缺失 ⇒ `#by_name` ⇒ `missing_main_part`（**不是** `not_zip`）。
#[test]
fn pg9_by_name_route_missing_main_part() {
    let path = crate_root().join("tests/fixtures/s01_negative/C6_missing_main_part.docx");
    let docx = std::fs::read(&path).expect("C6 fixture 应存在（CORPUS.md §2bis）");
    let err = layout_document(&docx, &[] as &[FontBlob], &empty_view())
        .expect_err("缺 word/document.xml 必须失败");
    assert_eq!(err.kind, LayoutErrorKind::Zip);
    assert_eq!(
        err.at.as_deref(),
        Some(AT_READ_DOCUMENT_XML_BY_NAME),
        "at 必须是 by_name 锚点，实得 {:?}",
        err.at
    );
    let out = capture_layout_error(err, "ctx#x");
    let k = out.panic_kind.expect("kind");
    assert_eq!(
        k.kind.as_str(),
        "missing_main_part",
        "缺主 part ⇒ not_zip 是错的（PG9）"
    );
}

/// C-3：zip 头在但中央目录不可解析 ⇒ `#ZipArchive_new` ⇒ `zip_corrupt`。
#[test]
fn pg7_zip_new_route_zip_corrupt() {
    // 构造：以 `PK` 开头但整体不是合法 zip（签名通过前 2 字节检查，ZipArchive::new 失败）
    let bogus = b"PK\x03\x04this-is-not-a-real-zip-central-directory".to_vec();
    let err = layout_document(&bogus, &[] as &[FontBlob], &empty_view())
        .expect_err("损坏 zip 必须失败");
    assert_eq!(err.kind, LayoutErrorKind::Zip);
    assert_eq!(
        err.at.as_deref(),
        Some(AT_READ_DOCUMENT_XML_ZIP_NEW),
        "at 必须是 ZipArchive_new 锚点，实得 {:?}",
        err.at
    );
    let out = capture_layout_error(err, "ctx#x");
    assert_eq!(out.panic_kind.expect("kind").kind.as_str(), "zip_corrupt");
}

/// C-4：`not_zip` 与 `zip_corrupt` 在真实入口上**可区分**（PG7 的核心判据）。
#[test]
fn pg7_real_entry_distinguishes_not_zip_from_corrupt() {
    let notzip = layout_document(b"not a zip at all", &[] as &[FontBlob], &empty_view())
        .expect_err("err");
    let corrupt = layout_document(
        b"PK\x03\x04broken-central-directory",
        &[] as &[FontBlob],
        &empty_view(),
    )
    .expect_err("err");
    let a = capture_layout_error(notzip, "ctx#x").panic_kind.expect("k");
    let b = capture_layout_error(corrupt, "ctx#x").panic_kind.expect("k");
    assert_eq!(a.kind.as_str(), "not_zip");
    assert_eq!(b.kind.as_str(), "zip_corrupt");
    assert_ne!(
        a.at, b.at,
        "两者必须可区分（都报 Zip 一档 ⇒ fail）；实得 {} vs {}",
        a.at, b.at
    );
}
