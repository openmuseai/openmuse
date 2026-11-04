use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Mutex;

use ppt_core::cn::wps::show::pptxr::ppt::theme::PptxrTheme::{
    OpcrPackagePart, PptResourceFactory, PptxrMedias,
};
use ppt_core::cn::wps::show::pptxr::ppt::theme::PptxrThemes::{
    OpcrRelationship, PptFileCache, PptxrThemes, THEME_RELS_TYPE,
};
use ppt_core::cn::wps::show::util::layout_shape::KmoPresentation;

static LOCK: Mutex<()> = Mutex::new(());

fn pres() -> KmoPresentation {
    KmoPresentation {
        slide_width: 0,
        slide_height: 0,
        themes: Vec::new(),
        tmp_path: "/tmp/ppt".into(),
        ..Default::default()
    }
}

fn theme_part(rids: HashMap<String, i32>) -> OpcrPackagePart {
    OpcrPackagePart {
        bytes: b"<a:theme/>".to_vec(),
        rid_pic: rids,
        get_stream_err: false,
        read_err: false,
        load_err: false,
    }
}

fn theme_rels(part_name: &str, part: Option<OpcrPackagePart>) -> OpcrRelationship {
    OpcrRelationship {
        part_name: part_name.into(),
        rels_type: THEME_RELS_TYPE.into(),
        part,
    }
}

#[test]
fn t01_miss_puts_cache() {
    let mut p = pres();
    let mut themes = PptxrThemes::PptxrThemes(0);
    let rels = theme_rels("ppt/theme/theme1.xml", Some(theme_part(HashMap::new())));
    let a = themes.open(&mut p, &rels, None).unwrap();
    let b = themes.open(&mut p, &rels, None).unwrap();
    assert!(Rc::ptr_eq(&a, &b));
}

#[test]
fn t02_hit_same_instance() {
    let mut p = pres();
    let mut themes = PptxrThemes::PptxrThemes(0);
    let rels = theme_rels("ppt/theme/theme1.xml", Some(theme_part(HashMap::new())));
    let first = themes.open(&mut p, &rels, None).unwrap();
    let second = themes.open(&mut p, &rels, None).unwrap();
    assert!(Rc::ptr_eq(&first, &second));
}

#[test]
fn t03_add_theme_false() {
    let mut p = pres();
    let mut themes = PptxrThemes::PptxrThemes(0);
    let rels = theme_rels("ppt/theme/theme1.xml", Some(theme_part(HashMap::new())));
    themes
        .open_addTheme(&mut p, &rels, None::<PptxrMedias>, false)
        .unwrap();
    assert_eq!(p.themes.len(), 0);
}

#[test]
fn t04_add_theme_true_three_arg() {
    let mut p = pres();
    let mut themes = PptxrThemes::PptxrThemes(0);
    let rels = theme_rels("ppt/theme/theme1.xml", Some(theme_part(HashMap::new())));
    let th = themes.open(&mut p, &rels, None).unwrap();
    assert_eq!(p.themes.len(), 1);
    assert!(Rc::ptr_eq(&p.themes[0], &th));
}

#[test]
fn t05_null_part_build_default() {
    let _g = LOCK.lock().unwrap();
    PptResourceFactory::setTestTheme(Some(b"<a:theme/>".to_vec()), Some("default.xml".into()));
    let mut p = pres();
    let mut themes = PptxrThemes::PptxrThemes(0);
    let rels = theme_rels("ppt/theme/missing.xml", None);
    let th = themes.open(&mut p, &rels, None).unwrap();
    assert_eq!(th.borrow().theme_path.as_deref(), Some("default.xml"));
    let again = themes.open(&mut p, &rels, None).unwrap();
    assert!(Rc::ptr_eq(&th, &again));
}

#[test]
fn t06_theme_rels_rid_and_path() {
    let _g = LOCK.lock().unwrap();
    let mut p = pres();
    let mut themes = PptxrThemes::PptxrThemes(0);
    let mut rids = HashMap::new();
    rids.insert("rId2".into(), 9);
    let rels = theme_rels("ppt/theme/theme1.xml", Some(theme_part(rids.clone())));
    let th = themes.open(&mut p, &rels, None).unwrap();
    assert_eq!(th.borrow().rid2_media_id, Some(rids));
    assert_eq!(th.borrow().theme_path.as_deref(), Some("/tmp/ppt/theme.xml"));
}

#[test]
fn t07_dump_path_cache() {
    let _g = LOCK.lock().unwrap();
    PptFileCache::resetDumpFileCallCount();
    let mut themes = PptxrThemes::PptxrThemes(0);
    let rels1 = theme_rels("ppt/theme/theme1.xml", Some(theme_part(HashMap::new())));
    let p1 = themes.dump(&rels1, "/tmp/ppt".into()).unwrap();
    assert_eq!(PptFileCache::dumpFileCallCount(), 1);
    let rels2 = theme_rels("ppt/theme/theme1.xml", None);
    let p2 = themes.dump(&rels2, "/tmp/ppt".into()).unwrap();
    assert_eq!(p1, p2);
    assert_eq!(PptFileCache::dumpFileCallCount(), 1);
}
