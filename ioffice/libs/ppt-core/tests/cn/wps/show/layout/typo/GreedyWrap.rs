// Test 01_function_greedy_wrap — MVP 贪心断行（pl-05）
// 冻结矩阵 FUNCTION.md §3.2 GW1–GW8。
use ppt_core::cn::wps::show::layout::typo::GreedyWrap::greedy_wrap;
use ppt_core::cn::wps::show::layout::typo::TextBoxGeometry::RectF;
use wpslayout::cn::wps::text::layout::data::SpanInfo::SpanInfo;
use wpslayout::cn::wps::text::layout::data::TextBoxData::TextBoxData;
use wpslayout::cn::wps::text::layout::data::TextData::TextData;

fn para(text: &str, font: f32) -> TextData {
    let mut td = TextData::new();
    let n = text.chars().filter(|c| *c != '\r').count() as i32;
    td.mSourceText = Some(text.to_string());
    td.mFirstSpan.mFontSize = font;
    td.mFirstSpan.mStartPos = 0;
    td.mFirstSpan.mEndPos = n.max(1);
    td
}

fn box_of(paras: Vec<TextData>, wrap: bool, anchor: i32) -> TextBoxData {
    let mut b = TextBoxData::new();
    b.setWrapTextInShape(wrap);
    b.setAnchor(anchor);
    for p in paras {
        b.addTextData(p);
    }
    b
}

fn rect(w: f32, h: f32) -> RectF {
    RectF {
        left: 0.0,
        top: 0.0,
        right: w,
        bottom: h,
    }
}

#[test]
fn gw1_cjk_char_wrap() {
    let b = box_of(vec![para("你好世界", 10.0)], true, 0);
    let adv = vec![vec![10.0, 10.0, 10.0, 10.0]];
    let lines = greedy_wrap(&b, rect(25.0, 1000.0), &adv);
    assert_eq!(lines.len(), 2);
    assert_eq!(lines[0].text, "你好");
    assert_eq!((lines[0].startCp, lines[0].endCp), (0, 2));
    assert_eq!(lines[1].text, "世界");
    assert_eq!((lines[1].startCp, lines[1].endCp), (2, 4));
    assert!(lines[0].w <= 25.0);
}

#[test]
fn gw2_latin_word_break() {
    let b = box_of(vec![para("ab cd ef", 10.0)], true, 0);
    // ab=10, space=5, cd=10, space=5, ef=10
    let adv = vec![vec![5.0, 5.0, 5.0, 5.0, 5.0, 5.0, 5.0, 5.0]];
    let lines = greedy_wrap(&b, rect(25.0, 1000.0), &adv);
    assert_eq!(lines[0].text, "ab cd");
    assert!(!lines[0].text.contains("ef"));
    assert_eq!(lines[1].text.trim(), "ef");
}

#[test]
fn gw3_overflow_token() {
    let b = box_of(vec![para("WWWW", 10.0)], true, 0);
    let adv = vec![vec![10.0, 10.0, 10.0, 10.0]];
    let lines = greedy_wrap(&b, rect(25.0, 1000.0), &adv);
    assert_eq!(lines.len(), 1);
    assert_eq!(lines[0].text, "WWWW");
    assert_eq!(lines[0].w, 40.0);
}

#[test]
fn gw4_crlf_two_para() {
    let b = box_of(vec![para("A", 10.0), para("B", 10.0)], true, 0);
    let adv = vec![vec![10.0], vec![10.0]];
    let lines = greedy_wrap(&b, rect(100.0, 1000.0), &adv);
    assert_eq!(lines.len(), 2);
    assert_eq!(lines[0].text, "A");
    assert_eq!(lines[1].text, "B");
    assert!(!lines.iter().any(|l| l.text.contains('\r')));
}

#[test]
fn gw5_nowrap() {
    let s: String = "中".repeat(20);
    let b = box_of(vec![para(&s, 10.0)], false, 0);
    let adv = vec![vec![10.0; 20]];
    let lines = greedy_wrap(&b, rect(10.0, 1000.0), &adv);
    assert_eq!(lines.len(), 1);
    assert_eq!(lines[0].text.chars().count(), 20);
}

#[test]
fn gw6_empty_para_placeholder() {
    let b = box_of(vec![para("\r", 12.0)], true, 0);
    let adv = vec![vec![]];
    let lines = greedy_wrap(&b, rect(100.0, 1000.0), &adv);
    assert_eq!(lines.len(), 1);
    assert_eq!(lines[0].text, "");
    assert_eq!(lines[0].h, 12.0);
}

#[test]
fn gw7_line_height_max_font() {
    let mut td = para("ab", 10.0);
    td.mFirstSpan.mEndPos = 1;
    let mut other = SpanInfo::new();
    other.mFontSize = 20.0;
    other.mStartPos = 1;
    other.mEndPos = 2;
    td.mOtherSpans.push(other);
    let b = box_of(vec![td], false, 0);
    let adv = vec![vec![10.0, 10.0]];
    let lines = greedy_wrap(&b, rect(100.0, 1000.0), &adv);
    assert_eq!(lines.len(), 1);
    assert_eq!(lines[0].h, 20.0);
}

#[test]
fn gw8_anchor_bottom() {
    let b = box_of(vec![para("中", 10.0)], true, 1);
    let adv = vec![vec![10.0]];
    let lines = greedy_wrap(&b, rect(100.0, 100.0), &adv);
    assert_eq!(lines.len(), 1);
    assert!((lines[0].y - 90.0).abs() < 0.01, "y={}", lines[0].y);
}
