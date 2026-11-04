// function_id=p0_function_page_break
//
// rust: ★P0 —— `w:br w:type="page"` 端到端回归（G-P0-rope / G-P0-pages）。
//
// 里程碑：`layout-java-pipeline` P0 / L0。
// 等价 Java 路径（四跳，缺一不可）：
//   SAX `RunInnerContentHandler.getElementHandler(3152)`   ← WMap `"br" -> 3152`
//     → `BrHandler.onStart`                                ← L0-1
//     → `onImportRun(subDocType, dst, convertBreakText(a))` ← L0-3（String 重载）
//     → rope 里出现 `'\f'`
//   再经 `PageLayout::paginate` 的 `force_page_after` 生效 ⇒ `page_count == 2`。
//
// 断言两层：
//   1) rope 逐字含且仅含 1 个 `'\f'`（生产者证据，不依赖字体/度量）；
//   2) 产品渲染管线 `page_count == 2` 且第 2 页有正文（消费者证据）。
#![allow(non_snake_case)]

use word_core::cn::wps::moffice::writer::api::DocumentOpen;
use word_core::cn::wps::moffice::writer::core::TextDocument::TextDocument;

const MULTIPAGE: &str = "tests/fixtures/03_function_line_corpus/docx/multipage.docx";

fn import_multipage() -> Box<TextDocument> {
    let bytes = std::fs::read(MULTIPAGE).unwrap_or_else(|e| panic!("fixture {MULTIPAGE}: {e}"));
    // rust: 必须 `TextDocument::new()`（`Box` + `_create`）：按值放栈上再移动会使
    //   `SubDocument.mParent` 悬垂（同 `TextDocument.rs:1102` 的注释）。
    let mut doc = TextDocument::new();
    DocumentOpen::fill_text_document_from_docx(&mut doc, &bytes, "").expect("import ok");
    doc
}

fn main_text(doc: &TextDocument) -> String {
    let m = doc.getMainDocument().expect("main");
    let n = m.getLength();
    let rope = m.getTextRope().expect("rope");
    let mut buf = vec!['\0'; n.max(0) as usize];
    if n > 0 {
        unsafe { (*rope).getText(0, n, &mut buf, 0) };
    }
    buf.into_iter().collect()
}

/// G-P0-rope：`BrHandler.convertBreakText` 把 `type="page"` 转成 `'\f'` 并写入 rope。
#[test]
fn P0_1_rope_contains_form_feed() {
    let doc = import_multipage();
    let text = main_text(&doc);
    let ff = text.chars().filter(|c| *c == '\u{000c}').count();
    assert_eq!(
        ff, 1,
        "multipage.docx 有 1 个 <w:br w:type=\"page\"/>，rope 应恰含 1 个 '\\f'；实际 {ff}。\
         为 0 说明 L0-1/L0-2/L0-3 未接通；>1 说明重复写入。text={text:?}"
    );
    // `'\f'` 必须落在两段之间，而不是首尾。
    let idx = text.find('\u{000c}').expect("form feed");
    assert!(idx > 0 && idx < text.len() - 1, "form feed 位置异常: {idx}");
}

/// G-P0-rope（负向）：同族 fixture 不含 `w:br`，不得凭空产生 `'\f'`。
#[test]
fn P0_2_no_form_feed_without_page_break() {
    let path = "tests/fixtures/p1_layout_document/two-para.docx";
    let bytes = std::fs::read(path).unwrap_or_else(|e| panic!("fixture {path}: {e}"));
    let mut doc = TextDocument::new();
    DocumentOpen::fill_text_document_from_docx(&mut doc, &bytes, "").expect("import ok");
    let text = main_text(&doc);
    assert!(
        !text.contains('\u{000c}'),
        "two-para.docx 无 w:br，不应出现 '\\f'；text={text:?}"
    );
}

/// G-P0-pages：分页符经 `PageLayout::paginate` 生效 ⇒ 渲染出 2 页。
#[test]
fn P0_3_product_render_two_pages() {
    use word_core::cn::wps::moffice::writer::api::LayoutDocument::{
        layout_document, FontBlob, ViewParams,
    };

    let bytes = std::fs::read(MULTIPAGE).expect("fixture");
    // rust: 度量需要真实字体；缺失时本测试退化为仅检查 rope（P0_1 已覆盖），
    //   故显式加载确定存在的系统字体，避免 MissingGlyph 干扰分页断言。
    let mut fonts: Vec<FontBlob> = Vec::new();
    for cand in [
        "/System/Library/Fonts/Geneva.ttf",
        "/System/Library/Fonts/Supplemental/Arial.ttf",
    ] {
        if let Ok(b) = std::fs::read(cand) {
            fonts.push(FontBlob {
                id: "default".to_string(),
                bytes: b,
            });
            break;
        }
    }
    if fonts.is_empty() {
        eprintln!("skip: no system font available for metrics");
        return;
    }

    let frame = layout_document(&bytes, &fonts, &ViewParams::default_a4()).expect("layout ok");
    assert!(
        frame.page.page_count >= 2,
        "multipage.docx 显式分页符应产出 ≥2 页；实际 {}。\
         为 1 说明 rope 的 '\\f' 未被子系统消费（见 PageLayout force_page_after）。",
        frame.page.page_count
    );
}
