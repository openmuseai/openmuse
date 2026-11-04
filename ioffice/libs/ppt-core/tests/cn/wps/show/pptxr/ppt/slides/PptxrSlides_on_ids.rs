// Test 02_function_on_ids
use std::cell::RefCell;
use std::rc::Rc;

use ppt_core::cn::wps::show::pptxr::ppt::slides::PptxrSlides::{
    OpcrPackagePart, OpcrRelationship, OpcrRelationships, PptxrSlides, UnReadSlides,
};
use ppt_core::cn::wps::show::util::layout_shape::KmoPresentation;

fn rel(target: &str, part_name: &str, has_part: bool) -> OpcrRelationship {
    OpcrRelationship {
        target: target.into(),
        part_name: part_name.into(),
        part: if has_part {
            Some(OpcrPackagePart::default())
        } else {
            None
        },
        ..Default::default()
    }
}

fn slides_with_rels(rels: OpcrRelationships, pres: KmoPresentation) -> PptxrSlides {
    PptxrSlides::initialize();
    PptxrSlides::PptxrSlides(
        Rc::new(RefCell::new(pres)),
        Some(rels),
        None,
        None,
        None,
        None,
        0,
        None,
    )
}

#[test]
fn t01_on_slide_ids_valid() {
    let mut rels = OpcrRelationships::default();
    rels.put(
        "rId1".into(),
        rel("ppt/slides/slide1.xml", "ppt/slides/slide1.xml", true),
    );
    rels.put(
        "rId2".into(),
        rel("ppt/slides/slide2.xml", "ppt/slides/slide2.xml", true),
    );
    rels.put(
        "rId3".into(),
        rel("ppt/slides/slide3.xml", "ppt/slides/slide3.xml", true),
    );
    let mut s = slides_with_rels(rels, KmoPresentation::default());
    s.onSlideIds(vec!["rId1".into(), "rId2".into(), "rId3".into()]);
    assert_eq!(s.slides_reader_len(), 3);
    assert_eq!(s.pres().borrow().slideCount(), 3);
    assert_eq!(s.slide_rids(), &["rId1", "rId2", "rId3"]);
    assert_eq!(
        PptxrSlides::getSlideIdByFileName(Some("slide1.xml".into())) > 0,
        true
    );
    assert_eq!(
        PptxrSlides::getSlideIdByFileName(Some("slide2.xml".into())) > 0,
        true
    );
    assert_eq!(
        PptxrSlides::getSlideIdByFileName(Some("slide3.xml".into())) > 0,
        true
    );
}

#[test]
fn t02_missing_rel_and_null_part_skip() {
    let mut rels = OpcrRelationships::default();
    rels.put(
        "rId1".into(),
        rel("ppt/slides/slide1.xml", "ppt/slides/slide1.xml", true),
    );
    rels.put(
        "rId3".into(),
        rel("ppt/slides/slide3.xml", "ppt/slides/slide3.xml", false),
    );
    let mut s = slides_with_rels(rels, KmoPresentation::default());
    s.onSlideIds(vec![
        "rId1".into(),
        "rIdMissing".into(),
        "rId3".into(),
        "rId4".into(),
    ]);
    assert_eq!(s.slides_reader_len(), 1);
    assert_eq!(s.pres().borrow().slideCount(), 1);
    assert_eq!(s.create_slide_count(), 4);
    assert_eq!(
        PptxrSlides::getSlideIdByFileName(Some("slide3.xml".into())),
        0
    );
}

#[test]
fn t03_create_slide_count_truncate() {
    let mut pres = KmoPresentation::default();
    pres.selection.actived_slide_index = 12;
    let c = UnReadSlides::getCreateSlideCount(&pres, 25);
    assert!(c >= 12 && c <= 22);
    assert_eq!(c, 22);
    let pres5 = KmoPresentation::default();
    assert_eq!(UnReadSlides::getCreateSlideCount(&pres5, 5), 5);
}

#[test]
fn t04_on_slide_master_ids() {
    let mut rels = OpcrRelationships::default();
    rels.put(
        "m1".into(),
        rel("ppt/slideMasters/s1.xml", "ppt/slideMasters/s1.xml", true),
    );
    rels.put(
        "m2".into(),
        rel("ppt/slideMasters/s2.xml", "ppt/slideMasters/s2.xml", true),
    );
    let mut s = slides_with_rels(rels, KmoPresentation::default());
    s.onSlideMasterIds(vec!["m1".into(), "missing".into(), "m2".into()]);
    assert_eq!(s.masters_reader_len(), 2);
    let mut names = s.masters_part_names();
    names.sort();
    assert_eq!(
        names,
        vec![
            "ppt/slideMasters/s1.xml".to_string(),
            "ppt/slideMasters/s2.xml".to_string()
        ]
    );
    let masters = s.pres().borrow().masters.clone();
    assert_eq!(masters.len(), 2);
    assert!(masters.iter().all(|m| m.io_reading));
}

#[test]
fn t05_selection_init_and_history() {
    let mut rels = OpcrRelationships::default();
    for i in 1..=3 {
        rels.put(
            format!("rId{i}"),
            rel(
                &format!("ppt/slides/slide{i}.xml"),
                &format!("ppt/slides/slide{i}.xml"),
                true,
            ),
        );
    }
    let rids = vec!["rId1".into(), "rId2".into(), "rId3".into()];

    let mut p0 = KmoPresentation::default();
    p0.selection.history_slide_index = 0;
    let mut s0 = slides_with_rels(rels.clone(), p0);
    s0.onSlideIds(rids.clone());
    assert_eq!(s0.pres().borrow().selection.init_count, 1);
    assert!(s0.pres().borrow().selection.select_slide_calls.is_empty());

    let mut p_oob = KmoPresentation::default();
    p_oob.selection.history_slide_index = 9;
    let mut s1 = slides_with_rels(rels.clone(), p_oob);
    s1.onSlideIds(rids.clone());
    assert_eq!(s1.pres().borrow().selection.init_count, 1);
    assert!(s1.pres().borrow().selection.select_slide_calls.is_empty());

    let mut p_ok = KmoPresentation::default();
    p_ok.selection.history_slide_index = 1;
    let mut s2 = slides_with_rels(rels, p_ok);
    s2.onSlideIds(rids);
    assert_eq!(s2.pres().borrow().selection.init_count, 1);
    assert_eq!(s2.pres().borrow().selection.select_slide_calls, vec![1]);
}
