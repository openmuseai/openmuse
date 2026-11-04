use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Mutex;

use ppt_core::cn::wps::show::pptxr::ppt::theme::PptxrTheme::{
    OpcrPackagePart, PptResourceFactory, PptxrTheme,
};
use ppt_core::cn::wps::show::util::layout_shape::ThemeObj;

static RES_LOCK: Mutex<()> = Mutex::new(());

fn theme() -> Rc<RefCell<ThemeObj>> {
    Rc::new(RefCell::new(ThemeObj::create()))
}

fn part(
    bytes: &[u8],
    rids: HashMap<String, i32>,
    read_err: bool,
    load_err: bool,
) -> OpcrPackagePart {
    OpcrPackagePart {
        bytes: bytes.to_vec(),
        rid_pic: rids,
        get_stream_err: false,
        read_err,
        load_err,
    }
}

#[test]
fn t01_null_part_returns_none() {
    let mut t = PptxrTheme::PptxrTheme(theme(), None, None, None, 0);
    assert!(t.open().is_none());
}

#[test]
fn t02_open_returns_rid_map() {
    let mut rids = HashMap::new();
    rids.insert("rId2".into(), 7);
    let mut t = PptxrTheme::PptxrTheme(
        theme(),
        Some(part(b"<a:theme/>", rids.clone(), false, false)),
        None,
        None,
        0,
    );
    assert_eq!(t.open(), Some(rids));
}

#[test]
fn t03_parse_throwable_returns_none() {
    let mut t = PptxrTheme::PptxrTheme(
        theme(),
        Some(part(b"THROW", HashMap::new(), true, false)),
        None,
        None,
        0,
    );
    assert!(t.open().is_none());
}

#[test]
fn t04_load_throwable_returns_none() {
    let mut t = PptxrTheme::PptxrTheme(
        theme(),
        Some(part(b"<a:theme/>", HashMap::new(), false, true)),
        None,
        None,
        0,
    );
    assert!(t.open().is_none());
}

#[test]
fn t05_build_default_theme_sets_path() {
    let _g = RES_LOCK.lock().unwrap();
    PptResourceFactory::setTestTheme(Some(b"<a:theme/>".to_vec()), Some("theme1.xml".into()));
    let th = PptxrTheme::buildDefaultTheme().unwrap();
    assert_eq!(th.borrow().theme_path.as_deref(), Some("theme1.xml"));
}

#[test]
fn t06_build_default_missing_stream() {
    let _g = RES_LOCK.lock().unwrap();
    PptResourceFactory::setTestTheme(None, None);
    let err = PptxrTheme::buildDefaultTheme().unwrap_err();
    assert_eq!(err.kind(), std::io::ErrorKind::NotFound);
    assert!(err.to_string().contains("The resources is not exist."));
}

#[test]
fn t07_build_default_parse_fail_closes_stream() {
    let _g = RES_LOCK.lock().unwrap();
    PptResourceFactory::setTestTheme(Some(b"THROW".to_vec()), Some("p".into()));
    assert!(PptxrTheme::buildDefaultTheme().is_err());
    assert!(PptxrTheme::lastDefaultStreamClosed());
}
