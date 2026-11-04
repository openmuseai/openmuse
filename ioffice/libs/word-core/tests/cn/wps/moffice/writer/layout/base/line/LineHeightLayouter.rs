// function_id=05_function_flow_text_line
// cargo test --test 05_function_flow_text_line
// ★wr-29 05 —— 本回合交付：LineHeightLayouter 真实（467 行 Java 移植）+ GraphAlignTool
//   getGraphVMetrics 真实；flowTextLine/LineLayouter.flow 编排链 继续 loop（下一批）。
#![allow(non_snake_case)]

use std::sync::Arc;

use word_core::cn::wps::moffice::writer::api::LayoutDocument::DEFAULT_MVP_FONT;
use word_core::cn::wps::moffice::writer::layout::base::env::DocumentEnv::DocumentEnv;
use word_core::cn::wps::moffice::writer::layout::base::env::LayoutState::LayoutState;
use word_core::cn::wps::moffice::writer::layout::base::env::MainState::MainState;
use word_core::cn::wps::moffice::writer::layout::base::env::ParaPropEnv::ParaPropEnv;
use word_core::cn::wps::moffice::writer::layout::base::env::PropertyToolFactory::PropertyToolFactory;
use word_core::cn::wps::moffice::writer::layout::base::env::ViewEnv::ViewEnv;
use word_core::cn::wps::moffice::writer::layout::base::graph::CharsNode::CharsNode;
use word_core::cn::wps::moffice::writer::layout::base::graph::GraphAlignTool::GraphAlignTool;
use word_core::cn::wps::moffice::writer::layout::base::graph::GraphFactory::GraphFactory;
use word_core::cn::wps::moffice::writer::layout::base::graph::Graphs::Graphs as RealGraphs;
use word_core::cn::wps::moffice::writer::layout::base::graph::SrcNode::{SrcNode, TYPE_SRC_CORECHARS};
use word_core::cn::wps::moffice::writer::cache::TypoList::TypoList;
use word_core::cn::wps::moffice::writer::cache::TypoObjBase::TypoObjBase;
use word_core::cn::wps::moffice::writer::cache::TypoSnapshot::TypoSnapshot;
use word_core::cn::wps::moffice::writer::cache::TypoTextLine::TypoTextLine;
use word_core::cn::wps::moffice::writer::cache::TypoRectCoreImpl::TypoRectCoreImpl;
use word_core::cn::wps::moffice::writer::layout::base::line::ILineHeightCallback::ILineHeightCallback;
use word_core::cn::wps::moffice::writer::layout::base::line::SubLineEnv::SubLineEnv;
use word_core::cn::wps::moffice::writer::layout::base::TextLayouter::TextLayouter;
use word_core::cn::wps::moffice::writer::layout::base::line::LineLayouter::LineLayouter;
use word_core::cn::wps::moffice::writer::layout::base::line::SubLineLayouter::SubLineLayouter;
use word_core::cn::wps::moffice::writer::layout::base::line::align::AlignLeftHandler::AlignLeftHandler;
use word_core::cn::wps::moffice::writer::layout::base::line::charSpace::WhiteSpaceHandler::WhiteSpaceHandler;
use word_core::cn::wps::moffice::writer::layout::base::line::LineHeightLayouter::LineHeightLayouter;
use word_core::cn::wps::moffice::writer::layout::text::FontHost;
use word_core::cn::wps::moffice::writer::layout::text::FontStack::FontStack;

fn inject_fonts() {
    let stack = FontStack::from_font_bytes(DEFAULT_MVP_FONT.to_vec()).expect("mvp latin");
    FontHost::set_font_table(Arc::new(stack));
}

fn make_state() -> LayoutState {
    let mut st = LayoutState::default();
    st.paraEnv.curParaPropEnv = Some(ParaPropEnv::default());
    if let Some(env) = st.paraEnv.curParaPropEnv.as_mut() {
        env.end = 12;
        env.paraProperty = Some(util::moffice::property::PropertySet::PropertySet::default());
    }
    // A4-ish 页面，避免 last-line 收缩路径干扰主断言
    st.pageEnv.width = 11906;
    st.pageEnv.height = 16838;
    st.pageEnv.marginLeft = 1134;
    st.pageEnv.marginRight = 1134;
    st.pageEnv.marginTop = 1440;
    st.pageEnv.marginBottom = 1440;
    st.pageEnv.maxLayoutHeight = 12000;
    st
}

fn core_chars_node() -> CharsNode {
    let mut src = SrcNode::SrcNode();
    src.r#type = TYPE_SRC_CORECHARS;
    let mut cn = CharsNode::CharsNode();
    cn.finalRunProp = Some(util::moffice::property::PropertySet::PropertySet::default());
    cn.srcRef = Some(Box::leak(Box::new(src)) as *mut SrcNode);
    cn.isInMath = false;
    cn
}

/// 产出 "Hello World!" 的 graphs（GraphFactory 真实量宽，wr-29 04）
fn build_hello_graphs(st: &mut LayoutState, gf: &mut GraphFactory) {
    let text: Vec<char> = "Hello World!".chars().collect();
    st.typoPara.allocGraphs(12);
    st.typoPara.putChars__chararr_int_int_int(&text, 0, 12, 0);
    let cn = core_chars_node();
    gf.createNodes(&cn, 0, 12, st);
}

/// 捕获 ILineHeightCallback 的探针
#[derive(Default)]
struct Probe {
    lineHeight: i32,
    alignLine: i32,
    ascPos: i32,
    descPos: i32,
    forceVCenter: bool,
}
impl ILineHeightCallback for Probe {
    fn setLineHeight(&mut self, v: i32) { self.lineHeight = v; }
    fn setAlignLine(&mut self, v: i32) { self.alignLine = v; }
    fn setAscDesPos(&mut self, a: i32, d: i32) { self.ascPos = a; self.descPos = d; }
    fn setForceVCenterAlign(&mut self, v: bool) { self.forceVCenter = v; }
}

/// LH1: hello 单行（10pt）行高 = getGraphVMetrics(wordAlign=4) 和的整化；baseline 居中通道 0
#[test]
fn LH1_hello_line_height_matches_graph_metrics() {
    inject_fonts();
    let mut docEnv = DocumentEnv::default();
    docEnv.propToolFactory = Some(PropertyToolFactory::default());
    docEnv.fontStyleCacheFactory = Some(Default::default());
    let mut gf = GraphFactory::GraphFactory(MainState::default(), docEnv, ViewEnv::default());
    // LineHeightLayouter 独立持有一份 env（仅读默认旗标；与 gf 的 env 解耦）
    let lh_doc = Box::leak(Box::new(DocumentEnv::default())) as *mut DocumentEnv;
    let lh_view = Box::leak(Box::new(ViewEnv::default())) as *mut ViewEnv;
    let mut st = make_state();
    build_hello_graphs(&mut st, &mut gf);
    assert!(st.graphList.size() >= 1);

    // 参考：GraphAlignTool.getGraphVMetrics 对首图 wordAlign=4（基线对齐）
    let first = st.graphList.firstElement();
    let mut ad = [0.0f32; 2];
    GraphAlignTool::getGraphVMetrics(unsafe { &*first }, 4, false, &mut ad);
    let expect_line = (ad[0] + ad[1] + 0.5) as i32;
    let expect_baseline = (ad[0] + 0.5) as i32;
    assert!(expect_line > 0, "MvpLatin 应有真实度量");

    let mut lhl = LineHeightLayouter::LineHeightLayouter(lh_doc, lh_view);
    let mut probe = Probe::default();
    lhl.calcLineHeightAndAlignmentLine(&mut st, &mut probe);

    assert_eq!(probe.lineHeight, expect_line, "行高 = ceil 化通道高（单 spacing 规则 0）");
    assert_eq!(probe.alignLine, expect_baseline, "baseline = maxAscent 向上整化（通道 0）");
    assert_eq!(probe.ascPos, probe.alignLine - (ad[0] + 0.5) as i32);
    assert_eq!(probe.descPos, probe.alignLine + (ad[1] + 0.5) as i32);
    assert!(!probe.forceVCenter);
}

/// LH2: 空 graphList → datas 全 0 → doCommit 直通（Java 同路径）
#[test]
fn LH2_empty_graphs_zero_metrics() {
    inject_fonts();
    let lh_doc = Box::leak(Box::new(DocumentEnv::default())) as *mut DocumentEnv;
    let lh_view = Box::leak(Box::new(ViewEnv::default())) as *mut ViewEnv;
    let mut st = make_state();
    let mut lhl = LineHeightLayouter::LineHeightLayouter(lh_doc, lh_view);
    let mut probe = Probe::default();
    lhl.calcLineHeightAndAlignmentLine(&mut st, &mut probe);
    assert_eq!(probe.lineHeight, 0);
    assert_eq!(probe.alignLine, 0);
    assert_eq!(probe.ascPos, 0);
    assert_eq!(probe.descPos, 0);
}

/// LH3: 多图同 style（Hello 合并图）→ 单组度量；行高不变（同一字体）
#[test]
fn LH3_merged_hello_reuses_style_metrics() {
    inject_fonts();
    let mut docEnv = DocumentEnv::default();
    docEnv.propToolFactory = Some(PropertyToolFactory::default());
    docEnv.fontStyleCacheFactory = Some(Default::default());
    let mut gf = GraphFactory::GraphFactory(MainState::default(), docEnv, ViewEnv::default());
    let mut st = make_state();
    build_hello_graphs(&mut st, &mut gf);
    let lh_doc = Box::leak(Box::new(DocumentEnv::default())) as *mut DocumentEnv;
    let lh_view = Box::leak(Box::new(ViewEnv::default())) as *mut ViewEnv;
    let mut lhl = LineHeightLayouter::LineHeightLayouter(lh_doc, lh_view);
    let mut probe = Probe::default();
    lhl.calcLineHeightAndAlignmentLine(&mut st, &mut probe);
    assert!(probe.lineHeight > 0);

    // updateLineHeightAndAlignmentLine：lastGraphIndex 在首图内 → 不增补度量（同 style）
    let mut probe2 = Probe::default();
    let lastIdx = unsafe { (*st.graphList.lastElement()).graphIndex };
    lhl.updateLineHeightAndAlignmentLine(lastIdx, &mut st, &mut probe2);
    assert_eq!(probe2.lineHeight, probe.lineHeight);
    assert_eq!(probe2.alignLine, probe.alignLine);
}
/// TL1: snapshot 数据面（flat heap）：alloc/set/get 逐位精确；OOB 读 0；set 越界自动扩堆
#[test]
fn TL1_snapshot_data_plane_alloc_set_get() {
    let mut snap = TypoSnapshot::new();
    assert_eq!(snap.alloc(4), 1, "0 保留为 null");
    assert_eq!(snap.getAllocSize(), 5);
    snap.set__int_int(3, 777);
    assert_eq!(snap.get__int(3), 777);
    assert_eq!(snap.get__int(1), 0, "未写 0");
    assert_eq!(snap.get__int(9999), 0, "OOB 读 0");
    snap.set__int_int(50, 42); // 越界写 → 扩堆
    assert_eq!(snap.get__int(50), 42);
    snap.set__int_int_int(10, 1, 2);
    assert_eq!(snap.get__int(10), 1);
    assert_eq!(snap.get__int(11), 2);
    snap.set__int_int_int_int(20, 1, 2, 3);
    assert_eq!(snap.get__int(22), 3);
    snap.set__int_int_int_int_int(30, 1, 2, 3, 4);
    assert_eq!(snap.get__int(33), 4);
}

/// TL2: TypoList 行列表（size/get/quickGet/first/last）真实读
#[test]
fn TL2_typo_list_roundtrip() {
    let mut snap = TypoSnapshot::new();
    let pList = TypoList::alloc__typosnapshot_int(&mut snap, 4);
    assert_eq!(TypoList::size__int_typosnapshot(pList, &snap), 0);
    TypoList::insert__int_int_int_typosnapshot(101, 0, pList, &mut snap);
    TypoList::insert__int_int_int_typosnapshot(202, 1, pList, &mut snap);
    TypoList::insert__int_int_int_typosnapshot(303, 2, pList, &mut snap);
    assert_eq!(TypoList::size__int_typosnapshot(pList, &snap), 3);
    assert_eq!(TypoList::first__int_typosnapshot(pList, &mut snap), 101);
    assert_eq!(TypoList::last__int_typosnapshot(pList, &mut snap), 303);
    assert_eq!(TypoList::get__int_int_typosnapshot(1, pList, &mut snap), 202);
    assert_eq!(TypoList::get__int_int_typosnapshot(7, pList, &mut snap), 0, "OOB 0");
    assert_eq!(TypoList::quickGet__int_int_typosnapshot(2, pList, &mut snap), 303);
    assert_eq!(TypoList::size__int_typosnapshot(0, &snap), 0, "pList==0 → 0");
}

/// OB1: 行矩形（TypoObjBase.getTop/Bottom/Left/Right）；TypoTextLine 行对象 alloc + 写入
#[test]
fn OB1_line_rect_and_textline_alloc() {
    let mut snap = TypoSnapshot::new();
    let mut tl = TypoTextLine::new();
    let ptr = tl.alloc(&mut snap);
    assert_eq!(ptr, 1);
    // 行矩形 [ptr+3..+6]
    snap.set__int_int(ptr + 4, 100); // top
    snap.set__int_int(ptr + 6, 260); // bottom
    snap.set__int_int(ptr + 3, 40);  // left
    snap.set__int_int(ptr + 5, 600); // right
    assert_eq!(TypoObjBase::getTop__int_typosnapshot(ptr, &snap), 100);
    assert_eq!(TypoObjBase::getBottom__int_typosnapshot(ptr, &snap), 260);
    assert_eq!(TypoObjBase::getLeft__int_typosnapshot(ptr, &snap), 40);
    assert_eq!(TypoObjBase::getRight__int_typosnapshot(ptr, &snap), 600);
    // 行高字段 +19（getLines）/+24（getLineAfter）
    snap.set__int_int(ptr + 19, 3);
    snap.set__int_int(ptr + 24, 20);
    assert_eq!(TypoTextLine::getLines__int_typosnapshot(ptr, &snap), 3);
    assert_eq!(TypoTextLine::getLineAfter__int_typosnapshot(ptr, &snap), 20);
    // 实例镜像 roundtrip
    tl.setPara__int(9);
    tl.setTextRectTop(88);
    tl.setGraphIndexStart(0);
    assert_eq!(tl.getPara(), 9);
    assert_eq!(tl.getTextRectTop(), 88);
    assert_eq!(tl.getGraphIndexStart(), 0);
}

/// LSB1: consumeGraphsBefore 消费 'Hello' 图之前 → graphList 缩为 3 图、cp=5（getNextCP 精确）、mFirstGraphIndex=5
#[test]
fn LSB1_consume_hello_before() {
    inject_fonts();
    let mut docEnv = DocumentEnv::default();
    docEnv.propToolFactory = Some(PropertyToolFactory::default());
    docEnv.fontStyleCacheFactory = Some(Default::default());
    let mut gf = GraphFactory::GraphFactory(MainState::default(), docEnv, ViewEnv::default());
    let mut st = make_state();
    let text: Vec<char> = "Hello World!".chars().collect();
    st.typoPara.allocGraphs(12);
    st.typoPara.putChars__chararr_int_int_int(&text, 0, 12, 0);
    // 真实管线由 GraphReader.commitReadText 维护映射（Java TypoPara snapshot map 对等）
    st.typoPara.putMap(0, 12, 0, 12);
    let cn = core_chars_node();
    gf.createNodes(&cn, 0, 12, &mut st);
    assert_eq!(st.graphList.size(), 4);

    let hello = unsafe { &*st.graphList.firstElement() };
    assert_eq!(hello.graphLength, 5);
    st.consumeGraphsBefore(hello, false);
    assert_eq!(st.graphList.size(), 3, "Hello 图被移除，剩 space/World/'!'");
    assert_eq!(st.mFirstGraphIndex, 5);
    assert_eq!(st.cp, 5, "getNextCP(4) → 0+4+1=5（cpRef=0）");
}

/// LSB2: consume LAST graph → else 分支（srcList 联动）；空 srcList 安全
#[test]
fn LSB2_consume_last_graph_else_branch() {
    inject_fonts();
    let mut docEnv = DocumentEnv::default();
    docEnv.propToolFactory = Some(PropertyToolFactory::default());
    docEnv.fontStyleCacheFactory = Some(Default::default());
    let mut gf = GraphFactory::GraphFactory(MainState::default(), docEnv, ViewEnv::default());
    let mut st = make_state();
    let text: Vec<char> = "Hi!".chars().collect();
    st.typoPara.allocGraphs(3);
    st.typoPara.putChars__chararr_int_int_int(&text, 0, 3, 0);
    st.typoPara.putMap(0, 3, 0, 3);
    let cn = core_chars_node();
    gf.createNodes(&cn, 0, 3, &mut st);
    assert_eq!(st.graphList.size(), 2); // "Hi" 合并 + "!"
    let last = unsafe { &*st.graphList.lastElement() };
    st.consumeGraphsBefore(last, false);
    assert_eq!(st.graphList.size(), 0, "else 分支 clean graphList");
    assert_eq!(st.mFirstGraphIndex, 3);
    assert_eq!(st.cp, 3);
    assert!(!st.hasReadEnd);
}

/// WH1: WhiteSpaceHandler.handleSpace —— diffSBMB=true & !wordWrap → 空格压为半字宽（setCompactWidth 精确）
#[test]
fn WH1_space_compact_width_diffSBMB() {
    inject_fonts();
    let mut docEnv = DocumentEnv::default();
    docEnv.propToolFactory = Some(PropertyToolFactory::default());
    docEnv.fontStyleCacheFactory = Some(Default::default());
    docEnv.balanceSingleByteDoubleByteWidth = true;
    let mut gf = GraphFactory::GraphFactory(MainState::default(), docEnv, ViewEnv::default());
    let mut st = make_state();
    st.paraEnv.wordWrap = false;
    let text: Vec<char> = "Hi world!".chars().collect();
    st.typoPara.allocGraphs(9);
    st.typoPara.putChars__chararr_int_int_int(&text, 0, 9, 0);
    let cn = core_chars_node();
    gf.createNodes(&cn, 0, 9, &mut st);
    // 找空格图（runType=2）
    let mut space = std::ptr::null_mut();
    let mut ptr = st.graphList.firstElement();
    while !ptr.is_null() {
        if unsafe { (*ptr).runType } == 2 { space = ptr; }
        ptr = st.graphList.getNext(ptr);
    }
    assert!(!space.is_null());
    let spaceMw = unsafe { (*space).mWidth };
    assert!(spaceMw > 0);

    let lh_doc = Box::leak(Box::new(DocumentEnv::default())) as *mut DocumentEnv;
    let lh_view = Box::leak(Box::new(ViewEnv::default())) as *mut ViewEnv;
    unsafe { (*lh_doc).balanceSingleByteDoubleByteWidth = true; }
    let mut wsh = WhiteSpaceHandler::WhiteSpaceHandler(lh_doc, lh_view);
    wsh.setEnv(&st);
    let mut sub = SubLineEnv::SubLineEnv();
    wsh.setSubLineEnv(&mut sub);
    // prev = 'i' 图；cur = 空格图
    let mut curG = unsafe { &mut *space };
    let r = wsh.handleSpace(None, &mut curG);
    assert!(r);
    // diffSBMB+!wordWrap → setCompactWidth：新宽 = fontSpace 0 → 200/2 = 100
    assert_eq!(curG.mWidth, 100, "空格压半字宽（10pt → 100twip）");
}

/// AL1: AlignLeftHandler.adjustLine —— 行盒左对齐（layoutLeft/宽度 = lastRun.right - 压缩和）
#[test]
fn AL1_align_left_adjust_line() {
    inject_fonts();
    let mut docEnv = DocumentEnv::default();
    docEnv.propToolFactory = Some(PropertyToolFactory::default());
    docEnv.fontStyleCacheFactory = Some(Default::default());
    let mut gf = GraphFactory::GraphFactory(MainState::default(), docEnv, ViewEnv::default());
    let mut st = make_state();
    let text: Vec<char> = "Hi".chars().collect();
    st.typoPara.allocGraphs(2);
    st.typoPara.putChars__chararr_int_int_int(&text, 0, 2, 0);
    let cn = core_chars_node();
    gf.createNodes(&cn, 0, 2, &mut st);
    // lastRun：独立图节点（池节点引用不可克隆 → 用 standalone 图带 mWidth）
    let mut last = RealGraphs::default();
    last.mWidth = 300;
    last.offsetXTo(0);

    let mut sub = SubLineEnv::SubLineEnv();
    sub.layoutLeft = 100;
    sub.layoutRight = 900;
    sub.lastGraphs = Some(last);

    let mut h = AlignLeftHandler::AlignLeftHandler();
    h.super_.setSubLineEnv(&mut sub, &mut st);
    let mut line = TypoRectCoreImpl::default();
    h.adjustLine(&mut line);
    assert_eq!(line.getLeft(), 100);
    assert_eq!(line.width(), 300, "宽度 = lastRun.right(300) - 压缩和(0)");
}

/// SL1: SubLineLayouter.flow —— "Hello World!" 全图收入单子行（fit 路径）
///      mContinueFlow=true、first/last 镜像、位置累加（last right = 累计宽）
#[test]
fn SL1_flow_fits_hello_single_subline() {
    inject_fonts();
    let mut docEnv = DocumentEnv::default();
    docEnv.propToolFactory = Some(PropertyToolFactory::default());
    docEnv.fontStyleCacheFactory = Some(Default::default());
    let mut gf = GraphFactory::GraphFactory(MainState::default(), docEnv, ViewEnv::default());
    let mut st = make_state();
    let text: Vec<char> = "Hello World!".chars().collect();
    st.typoPara.allocGraphs(12);
    st.typoPara.putChars__chararr_int_int_int(&text, 0, 12, 0);
    let cn = core_chars_node();
    gf.createNodes(&cn, 0, 12, &mut st);
    assert_eq!(st.graphList.size(), 4);
    let first = st.graphList.firstElement();
    st.mGraphsIt.seek(first);

    let lh_doc = Box::leak(Box::new(DocumentEnv::default())) as *mut DocumentEnv;
    let lh_view = Box::leak(Box::new(ViewEnv::default())) as *mut ViewEnv;
    let mut sl = SubLineLayouter::SubLineLayouter(MainState::default(), DocumentEnv::default(), ViewEnv::default());
    sl.mCharSpaceHandler = Some(Box::new(WhiteSpaceHandler::WhiteSpaceHandler(lh_doc, lh_view)));
    sl.mHAlignHandler = Some(Box::new(AlignLeftHandler::AlignLeftHandler()));

    let mut sub = SubLineEnv::SubLineEnv();
    sub.left = 0;
    sub.right = 10000;
    let cont = sl.flow(&mut sub, true, &mut st);
    assert!(cont, "宽子行 → 继续流");
    assert!(sub.firstGraphs.is_some() && sub.lastGraphs.is_some(), "first/last 已收图");
    let firstRt = sub.firstGraphs.as_ref().unwrap().runType;
    let lastRt = sub.lastGraphs.as_ref().unwrap().runType;
    assert_eq!(firstRt, 1, "首图 'Hello'");
    assert_eq!(lastRt, 18, "末图 '!'");
    let lastRight = sub.lastGraphs.as_ref().unwrap().getRight();
    assert!(lastRight > 0, "位置累加 → last right>0");
    assert!(lastRight <= 10000, "未溢出");
}

/// SL2: 空 flow（游标空）→ 直接返回；子行无图
#[test]
fn SL2_flow_empty_cursor() {
    let mut st = LayoutState::default();
    st.paraEnv.curParaPropEnv = Some(ParaPropEnv::default());
    st.pageEnv.width = 11906;
    st.pageEnv.marginLeft = 1134;
    st.pageEnv.marginRight = 1134;
    let mut sl = SubLineLayouter::SubLineLayouter(MainState::default(), DocumentEnv::default(), ViewEnv::default());
    let mut sub = SubLineEnv::SubLineEnv();
    sub.left = 0;
    sub.right = 10000;
    let cont = sl.flow(&mut sub, true, &mut st);
    assert!(cont, "空游标 → 空流返回");
    assert!(sub.firstGraphs.is_none());
}

/// LL1: LineLayouter.flow —— "Hello World!" → 1 行 TypoTextLine 写入 curPage.lines
///      （05 验收：hello ≥1 TypoTextLine；行宽未溢出 → 单行）
#[test]
fn LL1_flow_hello_produces_text_line() {
    inject_fonts();
    let mut docEnv = DocumentEnv::default();
    docEnv.propToolFactory = Some(PropertyToolFactory::default());
    docEnv.fontStyleCacheFactory = Some(Default::default());
    let mut gf = GraphFactory::GraphFactory(MainState::default(), docEnv, ViewEnv::default());
    let mut st = make_state();
    st.paraEnv.isFirstLineOfPara = true;
    st.paraEnv.alignment = 0; // jc=left → AlignLeft（真实）
    let text: Vec<char> = "Hello World!".chars().collect();
    st.typoPara.allocGraphs(12);
    st.typoPara.putChars__chararr_int_int_int(&text, 0, 12, 0);
    st.typoPara.putMap(0, 12, 0, 12);
    let cn = core_chars_node();
    gf.createNodes(&cn, 0, 12, &mut st);
    // 图游标（Java GraphReader 预备后在 flow 前 seekBegin；Rust mirror 显式 seek 首图）
    let first = st.graphList.firstElement();
    st.mGraphsIt.seek(first);

    // 页面信封
    st.curPage = Some(word_core::cn::wps::moffice::writer::cache::TypoPage::TypoPage::default());
    let mut ll = LineLayouter::LineLayouter(MainState::default(), DocumentEnv::default(), ViewEnv::default());
    // 装配 handler（真实 WhiteSpace/AlignLeft）
    let lh_doc = Box::leak(Box::new(DocumentEnv::default())) as *mut DocumentEnv;
    let lh_view = Box::leak(Box::new(ViewEnv::default())) as *mut ViewEnv;
    ll.super_mut().mCharSpaceHandler = Some(Box::new(WhiteSpaceHandler::WhiteSpaceHandler(lh_doc, lh_view)));
    ll.super_mut().mHAlignHandler = Some(Box::new(AlignLeftHandler::AlignLeftHandler()));

    ll.flow(&mut st);
    assert!(st.validLine, "行有效");
    let page = st.curPage.as_ref().unwrap();
    assert_eq!(page.lines_len(), 1, "05 验收：hello ≥1 TypoTextLine");
    let line = page.lines_first().unwrap();
    assert_eq!(line.getStartCP(), 0);
    assert_eq!(line.getEndCP(), 12);
    
    
    
}

/// TL1: TextLayouter.doFlowTextLine —— 图 → LineLayouter.flow → 行读取推进（05 外层编排）
///      行写入 curPage.lines（LL1 已证）、linePos 0→1、curLayoutHeight 推进、不 finishPage
#[test]
fn TL1_do_flow_text_line_reads_back_line() {
    inject_fonts();
    let mut docEnv = DocumentEnv::default();
    docEnv.propToolFactory = Some(PropertyToolFactory::default());
    docEnv.fontStyleCacheFactory = Some(Default::default());
    let mut gf = GraphFactory::GraphFactory(MainState::default(), docEnv, ViewEnv::default());
    let mut st = make_state();
    st.paraEnv.isFirstLineOfPara = true;
    st.paraEnv.alignment = 0;
    let text: Vec<char> = "Hello World!".chars().collect();
    st.typoPara.allocGraphs(12);
    st.typoPara.putChars__chararr_int_int_int(&text, 0, 12, 0);
    st.typoPara.putMap(0, 12, 0, 12);
    let cn = core_chars_node();
    gf.createNodes(&cn, 0, 12, &mut st);
    let first = st.graphList.firstElement();
    st.mGraphsIt.seek(first);
    st.curPage = Some(word_core::cn::wps::moffice::writer::cache::TypoPage::TypoPage::default());
    st.cp = 12;

    let mut tl = TextLayouter::TextLayouter(MainState::default(), DocumentEnv::default(), ViewEnv::default());
    assert!(tl.m_line_layouter_is_some(), "ctor 装配 mLineLayouter");
    let finish = tl.doFlowTextLine(&mut st);
    assert!(!finish, "hello 单页不 finishPage");
    assert!(st.validLine);
    assert_eq!(st.linePos, 1, "行推进");
    let page = st.curPage.as_ref().unwrap();
    assert_eq!(page.lines_len(), 1);
    let line = page.lines_first().unwrap();
    assert_eq!(line.getStartCP(), 0);
    assert_eq!(line.getEndCP(), 12);
    assert!(st.curLayoutHeight > 0, "curLayoutHeight 已推进（行底 + lineAfter）");
}
