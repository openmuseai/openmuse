// function_id=03_function_graph_reader
#![allow(non_snake_case)]

use std::fs;

use word_core::cn::wps::moffice::writer::api::DocumentOpen;
use word_core::cn::wps::moffice::writer::core::Document::Document;
use word_core::cn::wps::moffice::writer::core::TextDocument::TextDocument;
use word_core::cn::wps::moffice::writer::layout::base::env::DocumentEnv::DocumentEnv;
use word_core::cn::wps::moffice::writer::layout::base::env::LayoutState::LayoutState;
use word_core::cn::wps::moffice::writer::layout::base::env::MainState::MainState;
use word_core::cn::wps::moffice::writer::layout::base::env::ParaPropEnv::ParaPropEnv;
use word_core::cn::wps::moffice::writer::layout::base::env::PropertyToolFactory::PropertyToolFactory;
use word_core::cn::wps::moffice::writer::layout::base::env::ViewEnv::ViewEnv;
use word_core::cn::wps::moffice::writer::layout::base::graph::GraphReader::GraphReader;
use word_core::cn::wps::moffice::writer::layout::base::graph::SrcNode::{SrcNode, TYPE_SRC_CORECHARS};

const HELLO: &str = "tests/fixtures/p1_layout_document/hello-world.docx";
const HELLO_SHA256: &str = "69063664483d139c76647e6a5b29754c79158e14bc5c480c5fef85bad5dda98e";

fn sha2_hex(path: &str) -> String {
    let out = std::process::Command::new("shasum")
        .args(["-a", "256", path])
        .output()
        .expect("shasum");
    let s = String::from_utf8_lossy(&out.stdout);
    s.split_whitespace().next().unwrap_or("").to_string()
}

/// hello-world.docx 真实 parse → GraphReader + LayoutState（document/paraEnv 已装配）
/// rust: ★悬垂指针修复 —— doc（TextDocument）必须与 main_ptr 同生命周期存活；
///       早期版本 setup 返回后 drop doc，后续 prepareGraphs 经悬垂 main_ptr 解引用
///       （被后续 parse 复用内存后破坏 chpx 树）。现以 Box::leak 保活。
fn setup() -> (GraphReader, LayoutState, *mut dyn Document, i32) {
    assert_eq!(sha2_hex(HELLO), HELLO_SHA256);
    let bytes = fs::read(HELLO).expect("hello-world.docx");
    let mut doc = TextDocument::new();
    DocumentOpen::fill_text_document_from_docx(&mut doc, &bytes, "").expect("import");
    let _doc_keep: &'static mut TextDocument = Box::leak(doc);
    let main: &dyn Document = _doc_keep.getMainDocument().expect("main");
    let doc_len: i32 = main.getLength();
    assert!(doc_len > 0);
    let main_ptr: *mut dyn Document = main as *const dyn Document as *mut dyn Document;

    let mut docEnv = DocumentEnv::default();
    docEnv.propToolFactory = Some(PropertyToolFactory::default());

    let gr = GraphReader::GraphReader(MainState::default(), docEnv, ViewEnv::default());

    let mut st = LayoutState::default();
    st.document = Some(main_ptr);
    st.paraEnv.curParaPropEnv = Some(ParaPropEnv::default());
    if let Some(env) = st.paraEnv.curParaPropEnv.as_mut() {
        env.end = doc_len;
        env.paraProperty = Some(util::moffice::property::PropertySet::PropertySet::default());
    }
    (gr, st, main_ptr, doc_len)
}

fn add_core_src(st: &mut LayoutState, main_ptr: Option<*mut dyn Document>, doc_len: i32) {
    let src: *mut SrcNode = st.srcList.obtainAndAddLast();
    unsafe {
        (*src).r#type = TYPE_SRC_CORECHARS;
        (*src).doc = main_ptr;
        (*src).cp = 0;
        (*src).length = doc_len;
        (*src).revisionFinal = true;
        (*src).data = None;
        (*src).finalRunProp = None;
    }
}

/// GR1: doPrepareGraphs 空 srcList 不读（hasContents false → prepareGraphs 早退）
#[test]
fn GR1_do_prepare_empty_srcList() {
    let (mut gr, mut st, _main_ptr, _len) = setup();
    assert!(!st.hasContents());
    assert!(!gr.prepareGraphs(0, &mut st));
    assert!(st.charsList.isEmpty());
    assert_eq!(gr.current_cp(), -1);
}

/// GR2: initFilter 对 hello 不 panic（无 域/脚注/数学/OMML → 过滤器集合空）
#[test]
fn GR2_init_filter_hello_no_panic() {
    let (mut gr, mut st, _main_ptr, _len) = setup();
    gr.initFilter(st.document, 0, 1000, &mut st);
}

/// GR3: hello CORECHARS → doPrepareGraphs 填 charsList（真实节点存储 GR-D1）
#[test]
fn GR3_hello_core_chars_fills_chars_list() {
    let (mut gr, mut st, main_ptr, doc_len) = setup();
    add_core_src(&mut st, Some(main_ptr), doc_len);
    let ok = gr.prepareGraphs(0, &mut st);
    // createNodes 仍为抽象空钩（GR-D2：量宽归 04），故 prepareGraphs 返回 hasMaths/hasEmbed = false
    assert!(!ok);
    assert!(!st.charsList.isEmpty(), "hello CORECHARS 应填 charsList");
    assert!(gr.current_cp() > 0, "mCurCP 应推进过正文");
    // 每个 CharsNode 都带 finalRunProp（env PropertyTool 合并的真实 run prop）
    unsafe {
        let mut node = st.charsList.firstElement();
        let mut count = 0;
        while !node.is_null() {
            let cn: &word_core::cn::wps::moffice::writer::layout::base::graph::CharsNode::CharsNode = &*node;
            assert!(cn.finalRunProp.is_some(), "CharsNode 必须携带 finalRunProp");
            count += 1;
            node = st.charsList.getNext(node);
        }
        assert!(count > 0, "至少一个 CharsNode");
    }
}

/// GR4: doCleanGraphReader 复位
#[test]
fn GR4_do_clean_graph_reader() {
    let (mut gr, mut st, main_ptr, doc_len) = setup();
    add_core_src(&mut st, Some(main_ptr), doc_len);
    gr.prepareGraphs(0, &mut st);
    assert!(!st.charsList.isEmpty());
    // ★复位：doCleanGraphReader 应清空读绳状态
    gr.doCleanGraphReader();
    assert_eq!(gr.current_cp(), -1);
    // 复用同 srcList 再读一次仍应填 charsList（复位后状态一致）
    st.charsList.clean();
    gr.prepareGraphs(0, &mut st);
    assert!(!st.charsList.isEmpty(), "复位后再次 prepareGraphs 应重新填 charsList");
}

/// GR5: KPList 真实节点存储 —— obtainAndAddLast/firstElement/getNext/lastElement/clean 语义
#[test]
fn GR5_kplist_real_node_storage() {
    let mut st = LayoutState::default();
    add_core_src(&mut st, None, 5);
    unsafe {
        (*st.srcList.firstElement()).r#type = 7;
    }
    assert_eq!(unsafe { (*st.srcList.firstElement()).r#type }, 7);
    let second: *mut SrcNode = st.srcList.obtainAndAddLast();
    unsafe {
        (*second).r#type = 8;
    }
    let first: *mut SrcNode = st.srcList.firstElement();
    assert_eq!(unsafe { (*first).r#type }, 7);
    assert_eq!(unsafe { (*st.srcList.getNext(first)).r#type }, 8);
    assert!(st.srcList.getNext(second).is_null());
    assert_eq!(unsafe { (*st.srcList.lastElement()).r#type }, 8);
    st.srcList.clean();
    assert!(st.srcList.isEmpty());
}