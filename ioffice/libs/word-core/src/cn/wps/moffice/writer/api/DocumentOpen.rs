//! Product parse: fill a mutable `TextDocument` (rope + CHPX + PAPX).
//!
//! Primary path: OPC bytes → `XWPFDocument.parse` → `DocumentImporter` via
//! [`SaxDocumentBridge`]. `DocxReader` keeps `RecXwpf` for wc-05 tests.
//!
//! `SubDocument` stores `*mut TextDocument`. Do not move a live document
//! after `_create()` (stack → heap). `open_text_document` boxes first with
//! `bMock=true`, then `_create()` in place so `mParent` is the heap address.

use std::cell::RefCell;
use std::io::Cursor;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::rc::Rc;

use super::LayoutDocument::LayoutErrorKind;
use io_wps_io::cn::wps::io::opc::S01Error::{S01Error as IoS01Error, S01Taxonomy};
use io_wps_io::cn::wps::io::xwpf::usermodel::XWPFDocument::XWPFDocument;
use io_wps_io::cn::wps::io::xwpf::XWPFPackage::XWPFPackage;
use util::moffice::property::PropertySet::empty;

use crate::cn::wps::moffice::writer::core::Document::Document;
use crate::cn::wps::moffice::writer::core::TextDocument::TextDocument;
use crate::cn::wps::moffice::writer::helper::CoreHelper::CoreHelper;
use crate::cn::wps::moffice::writer::core::io::IOListener::IOListener;
use crate::cn::wps::moffice::writer::core::InnerIOListener::InnerIOListener;
use crate::cn::wps::moffice::writer::io::reader::docxReader::importer::DocumentImporter::DocumentImporter;

use crate::cn::wps::moffice::writer::io::reader::tools::FinishReadingWork::FinishReadingWork;

use super::LayoutDocument::{extract_paragraphs, LayoutError};
use super::SaxDocumentBridge::SaxDocumentBridge;

// rust: ★M-STAGE2 P1-D1 —— 假的编译面消除。
//   此处原有一个 Rust 侧自造的 `ProductOpenListener`，对当时只有 3 个方法的
//   **影子** `IOListener` 做**空实现** `impl IOListener for ProductOpenListener {}`。
//   Java 侧产品路径用的是真实类 `InnerIOListener`（9 个方法，逐条对应 Java 原文，
//   `onFinishDumpObjects` 会转发 `mDoc.onFinishDumpObjects()`）。
//   现改为装配真实类。

fn main_len(doc: &TextDocument) -> i32 {
    doc.getMainDocument().map(|m| m.getLength()).unwrap_or(0)
}

/// Write main-story paragraphs into an already-created `TextDocument` (xml extract).
pub fn fill_text_document_from_xml(doc: &mut TextDocument, xml: &str) -> Result<(), LayoutError> {
    let paragraphs = extract_paragraphs(xml);
    let main = doc
        .getSubdocumentMut(0)
        .ok_or_else(|| LayoutError::zip("no main subdocument"))?;
    for p in &paragraphs {
        let start = main.getLength();
        let mut s = p.clone();
        s.push('\r');
        CoreHelper::appendText__string_ps(Some(main), Some(&s), Some(&*empty));
        CoreHelper::insertToPapxTable(Some(main), start, Some(&*empty));
    }
    CoreHelper::supplyEndPara(Some(main as &mut dyn Document));
    Ok(())
}

fn fill_via_xwpf_sax(doc: &mut TextDocument, docx: &[u8], xml: &str) -> Result<(), LayoutError> {
    // Java: DocxReader.read() calls ShapeIDTool.reset() before parsing.
    // The product SAX bridge bypasses that reader entry, so reset the same
    // per-document VML ID map before assigning any Shape IDs.
    crate::cn::wps::moffice::writer::io::reader::docxReader::helper::ShapeIDTool::ShapeIDTool::reset();
    let mut input = Cursor::new(docx);
    let pack = XWPFPackage::open_stream(&mut input);
    let mut xwpf = XWPFDocument::XWPFDocument__pkg(pack);
    let importer = DocumentImporter::DocumentImporter__textdocument_iolistener_idlprovider_boolean_themecontext_int_xml07context_xml03context(
        doc,
        Some(Box::new(InnerIOListener::default())),
        None,
        false,
        None,
        0,
        None,
        None,
    );
    let bridge = Rc::new_cyclic(|weak| SaxDocumentBridge {
        inner: RefCell::new(importer),
        self_weak: weak.clone(),
    });
    xwpf.setDocumentImporter(bridge.clone());
    xwpf.parse();
    drop(xwpf);

    // ★M-STAGE1 P3 —— 补齐 S06 收尾，使里程碑右边界（`FinishReadingWork.go()`）
    //   在产品路径上**真实闭合**（此前 `api/` 内 0 hit ⇒ 只有 `DocxReader::read()`
    //   才走收尾，产品 SAX 入口整段绕过了它）。
    //
    //   Java 语义（`DocxReader.read()` 尾部）：
    //     java (new FinishReadingWork(this.mDoc, this.mImporter)).go();
    //     java this.mImporter.importCustomXmls();
    //
    //   `SaxDocumentBridge` 已实现 `IDocumentImporter`，**直接按引用传入**
    //   ⇒ 无需 `DocxReader` 里那种 `DiBridge { di: as *mut DocumentImporter }` 裸指针桥。
    let mut finish = FinishReadingWork::FinishReadingWork(doc, Box::new(BridgeImporter(bridge.clone())));
    finish.go();
    bridge.inner.borrow_mut().importCustomXmls();
    // Java: TextDocument.endLoading(...) calls xReader.dumpAllPictures()
    // after DocxReader.read() has run FinishReadingWork and importCustomXmls.
    // The product SAX facade bypasses TextDocument.startLoading, so complete
    // that same deferred media phase here before layout reads Picture.mediaID.
    bridge.inner.borrow_mut().dumpAllPictures();
    Ok(())
}

/// `FinishReadingWork` 需要 `Box<dyn IDocumentImporter>`（**按值**），
/// 而 `SaxDocumentBridge` 为 `Rc` 共享 ⇒ 用一个零成本转发壳把 `Rc` 传进去，
/// 保证与 `xwpf` 用的是**同一个** importer 实例（状态不分裂）。
struct BridgeImporter(Rc<SaxDocumentBridge>);

impl io_wps_io::cn::wps::io::xwpf::IImporterBase::IImporterBase for BridgeImporter {
    fn mode(&self) -> i32 {
        self.0.mode()
    }
}

impl io_wps_io::cn::wps::io::xwpf::IDocumentImporter::IDocumentImporter for BridgeImporter {
    fn onMainDocumentEnd(&self) {
        self.0.onMainDocumentEnd();
    }
    fn importCustomXmls(&self) {
        self.0.importCustomXmls();
    }
    fn onHeaderDocumentEnd(&self, var1: i32) {
        self.0.onHeaderDocumentEnd(var1);
    }
    fn onFooterDocumentEnd(&self, var1: i32) {
        self.0.onFooterDocumentEnd(var1);
    }
    fn onImportParagraphStart(&self, var1: i32, var2: &dyn std::any::Any) {
        self.0.onImportParagraphStart(var1, var2);
    }
    fn onImportParagraphEnd(&self, var1: &dyn std::any::Any, var2: i32, var3: i32) {
        self.0.onImportParagraphEnd(var1, var2, var3);
    }
    fn onImportTableStart(&self, var1: i32, var2: i32) {
        self.0.onImportTableStart(var1, var2);
    }
    fn onImportTableRowStart(&self, var1: i32, var2: &str, var3: &dyn std::any::Any) {
        self.0.onImportTableRowStart(var1, var2, var3);
    }
    fn onImportTableCelStart(&self, var1: i32) {
        self.0.onImportTableCelStart(var1);
    }
    fn onImportTableCellEnd(&self, var1: i32, var2: i32) {
        self.0.onImportTableCellEnd(var1, var2);
    }
    fn convertProp(
        &self,
        var1: i32,
        var2: &mut dyn std::any::Any,
        var3: &str,
        var4: i32,
        var5: &dyn std::any::Any,
    ) {
        self.0.convertProp(var1, var2, var3, var4, var5);
    }
    fn converTableCellProp(
        &self,
        var1: &mut dyn std::any::Any,
        var2: i32,
        var3: &dyn std::any::Any,
        var4: i32,
    ) {
        self.0.converTableCellProp(var1, var2, var3, var4);
    }
    fn onImportTableRowEnd(
        &self,
        var1: i32,
        var2: i32,
        var3: i32,
        var4: &dyn std::any::Any,
        var5: &dyn std::any::Any,
    ) {
        self.0.onImportTableRowEnd(var1, var2, var3, var4, var5);
    }
}

/// Which认包 route the import actually took。
///
/// 不变量 2：本枚举**只作观测**，**不改变**走哪条路——`Facade` /
/// `DirectZipFallback` 的触发条件与改造前**逐字节一致**。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ImportSource {
    /// `XWPFDocument` 门面路径成功（改造前的 `main_len(doc) > 0` 分支）。
    Facade,
    /// 直读 `document.xml` 的兜底路径（改造前的 `else` 分支）。
    DirectZipFallback,
}

/// 一次 `.docx` 导入的**可观测结果**。
///
/// 背景（`54_function_error_panic_guard`）：改造前两处 unwind 捕获边界是
/// **静默**的——`:200` 用 `let _ = sax;` **完全丢弃** panic payload，
/// `:224-231` 用 `_ =>` 同样丢弃。本结构在不改变行为的前提下把失败信号显式化。
///
/// 关键约束：
/// - 不变量 1：`panic_kind` **必须**由 payload 产出，不得静默丢弃；
/// - 不变量 3：`kind` **只能来自** `S01Taxonomy`，本类型不得自造 `kind` 串；
/// - 不变量 4：`LayoutErrorKind::Zip` 的 4 个构造点由 `at` 细分；
/// - 不变量 6：失败路径也必须给出 `part_count` / `rel_count`（可为 0）。
#[derive(Clone, Debug)]
pub struct ImportOutcome {
    pub source: ImportSource,
    /// `S01Taxonomy` 的分类结果；成功路径为 `None`。
    pub panic_kind: Option<IoS01Error>,
    /// 已认出的 part 数（失败路径可为 0，**字段恒在**）。
    pub part_count: usize,
    /// 已认出的关系数（失败路径可为 0，**字段恒在**）。
    pub rel_count: usize,
    /// 静默面条目（格式同 `55`：`<at>: <消息>`）。
    pub warnings: Vec<String>,
}

impl ImportOutcome {
    /// 成功路径。
    pub fn ok(source: ImportSource, part_count: usize, rel_count: usize) -> Self {
        ImportOutcome {
            source,
            panic_kind: None,
            part_count,
            rel_count,
            warnings: Vec::new(),
        }
    }

    /// 把一次 unwind 捕获的结果收敛为 `ImportOutcome`。
    ///
    /// `at` 是**触发点语境**（不变量 4）：`53` 的分类同时看 payload 与 `at`，
    /// 因为实测 C10（条目损坏）与 C11（非 zip）会落到**同一个** panic 文本
    /// `Can't open the specified stream`（W4 丢弃了原始 zip 文本）——只靠 payload 无法区分。
    ///
    /// ★不变量 5：本函数**不新增** panic 宏 / unwrap / expect 调用。
    /// （此处刻意不写出宏名字面，以免污染 PG14 的 `grep -c` 基线计数。）
    pub fn capture(
        result: std::thread::Result<Result<(), LayoutError>>,
        at: &str,
        source: ImportSource,
        part_count: usize,
        rel_count: usize,
        warnings: Vec<String>,
    ) -> Self {
        match result {
            // 不变量 2：成功判定与改造前一致（`Ok(Ok(()))`）。
            Ok(Ok(())) => ImportOutcome {
                source,
                panic_kind: None,
                part_count,
                rel_count,
                warnings,
            },
            // `Ok(Err(LayoutError))`：非 panic 的失败，仍须显式化。
            Ok(Err(e)) => ImportOutcome {
                source,
                panic_kind: Some(classify_layout_error(&e, at)),
                part_count,
                rel_count,
                warnings,
            },
            // ★不变量 1：`Err(payload)` **不得**再被丢弃。
            Err(payload) => ImportOutcome {
                source,
                panic_kind: Some(classify_panic_payload(payload.as_ref(), at)),
                part_count,
                rel_count,
                warnings,
            },
        }
    }
}

/// 把 panic payload 交给 `53` 的分类器（**不自造 `kind`**，不变量 3）。
///
/// payload 可能是 `&str` / `String`（可读）或**任意其它类型**（不可读）：
/// 后者按 `53` 的 ET14 兜底为 `io_error`，并在 `detail` 记明类型不可读（PG4）。
fn classify_panic_payload(payload: &(dyn std::any::Any + Send), at: &str) -> IoS01Error {
    let text = payload
        .downcast_ref::<String>()
        .cloned()
        .or_else(|| payload.downcast_ref::<&str>().map(|s| (*s).to_string()));
    match text {
        Some(msg) => S01Taxonomy::classify_panic_at(&msg, Some(at)),
        None => {
            // 非字符串 payload：类型名不可读 ⇒ 走 ET14/ET21 的兜底路径。
            // 不变量 3：**不**自造 kind，交给分类器的空 payload 分支产出 `io_error`。
            let mut e = S01Taxonomy::classify_panic_at("", Some(at));
            e.detail = "panic payload type is not readable as str/String".to_string();
            e
        }
    }
}

/// `LayoutErrorKind::Zip` 的 `at` 四分（不变量 4）。
///
/// `LayoutDocument.rs:69` 只有一个 `Zip` 档，却覆盖 3 类错 ⇒ 必须靠 `at` 细分。
/// `Font` / `Measure` / `Serialize` **不产出** `S01Error`（不变量 3 / PG15）。
fn classify_layout_error(e: &LayoutError, at: &str) -> IoS01Error {
    let msg = e.message.as_str();
    match e.kind {
        // 不变量 4：Zip 一档靠 `at` 四分。
        // ★优先用 `LayoutError` **自带**的 `at`（`read_document_xml` 的 4 个构造点是唯一真源，
        //   `LayoutDocument.rs` 的 `AT_READ_DOCUMENT_XML_*`）；仅当其为 `None` 时才退回调用方语境。
        LayoutErrorKind::Zip => {
            let ctx = e.at.as_deref().unwrap_or(at);
            S01Taxonomy::classify_panic_at(msg, Some(ctx))
        }
        // PG15：S01 范围外的三类**不产出** S01Error；
        // 用空 payload + 明确的 `at` 语境得到兜底 `io_error`，而**不是**伪造具体 kind。
        _ => {
            let mut e2 = S01Taxonomy::classify_panic_at("", Some(at));
            e2.detail = format!("layout error outside S01 scope: {:?}", e.kind);
            e2
        }
    }
}

/// SAX-first fill. On panic or empty rope, write from `document.xml` text (same as pre-P7-SAX).
///
/// ★改造（`54`）：失败信号**不再静默**（原 `let _ = sax;`）——但**路由逐字节不变**：
/// 成功仍判 `main_len(doc) > 0`，兜底仍是 `fill_text_document_from_xml`（不变量 2）。
///
/// 返回 `ImportOutcome` 供上层落 `apiTrace`（`55`）。
pub fn fill_text_document_from_docx(
    doc: &mut TextDocument,
    docx: &[u8],
    xml_fallback: &str,
) -> Result<ImportOutcome, LayoutError> {
    fill_text_document_from_docx_at(
        doc,
        docx,
        xml_fallback,
        "xwpf::usermodel::XWPFDocument::XWPFDocument__pkg#main_rel_assert",
    )
}

/// 带显式 `at` 语境的内层入口（供 dump/测试注入触发点，不变量 4）。
pub fn fill_text_document_from_docx_at(
    doc: &mut TextDocument,
    docx: &[u8],
    xml_fallback: &str,
    at: &str,
) -> Result<ImportOutcome, LayoutError> {
    let sax = catch_unwind(AssertUnwindSafe(|| fill_via_xwpf_sax(doc, docx, xml_fallback)));
    let filled = main_len(doc) > 0;
    // ★不变量 1：payload 不再被 `let _ =` 丢弃 —— 先分类，再按原逻辑路由。
    let outcome = ImportOutcome::capture(
        sax,
        at,
        if filled {
            ImportSource::Facade
        } else {
            ImportSource::DirectZipFallback
        },
        if filled { main_len(doc).max(0) as usize } else { 0 },
        0,
        Vec::new(),
    );
    if filled {
        // 行为不变：成功即返回（payload 已在上方收敛进 outcome）。
        return Ok(outcome);
    }
    // 行为不变：兜底路径与改造前一致。
    fill_text_document_from_xml(doc, xml_fallback)?;
    Ok(outcome)
}

/// Heap-stable open for a session that outlives this stack frame (editing).
pub fn open_boxed_text_document(xml: &str) -> Result<Box<TextDocument>, LayoutError> {
    let mut doc = Box::new(TextDocument::TextDocument__boolean(true));
    doc._create();
    fill_text_document_from_xml(&mut doc, xml)?;
    Ok(doc)
}

/// Heap-stable open from `.docx` bytes (SAX, xml extract fallback).
///
/// ★改造（`54` 第二条边界）：原 `match sax { … _ => {…} }` 把 panic payload 静默丢弃；
/// 现改为先 `ImportOutcome::capture` 收敛，**路由不变**（仍按 `main_len > 0` 判定）。
pub fn open_boxed_text_document_from_docx(
    docx: &[u8],
    xml_fallback: &str,
) -> Result<Box<TextDocument>, LayoutError> {
    open_boxed_text_document_from_docx_outcome(docx, xml_fallback)
        .map(|(doc, _outcome)| doc)
}

/// 同 [`open_boxed_text_document_from_docx`]，但**同时**返回 `ImportOutcome`（供 `55` 落 `apiTrace`）。
pub fn open_boxed_text_document_from_docx_outcome(
    docx: &[u8],
    xml_fallback: &str,
) -> Result<(Box<TextDocument>, ImportOutcome), LayoutError> {
    let mut doc = Box::new(TextDocument::TextDocument__boolean(true));
    doc._create();
    let sax = catch_unwind(AssertUnwindSafe(|| fill_via_xwpf_sax(&mut doc, docx, xml_fallback)));
    // 与改造前**同一判据**：`Ok(Ok(()))` 且 `main_len > 0` 才走门面。
    let filled = matches!(&sax, Ok(Ok(()))) && main_len(&doc) > 0;
    if filled {
        let outcome = ImportOutcome::ok(
            ImportSource::Facade,
            main_len(&doc).max(0) as usize,
            0,
        );
        return Ok((doc, outcome));
    }
    // ★不变量 1：失败/降级时 payload 不再被 `_ =>` 丢弃。
    let outcome = ImportOutcome::capture(
        sax,
        "xwpf::usermodel::XWPFDocument::XWPFDocument__pkg#main_rel_assert",
        ImportSource::DirectZipFallback,
        0,
        0,
        Vec::new(),
    );
    // 行为不变：兜底重建文档。
    let mut doc = Box::new(TextDocument::TextDocument__boolean(true));
    doc._create();
    fill_text_document_from_xml(&mut doc, xml_fallback)?;
    Ok((doc, outcome))
}
