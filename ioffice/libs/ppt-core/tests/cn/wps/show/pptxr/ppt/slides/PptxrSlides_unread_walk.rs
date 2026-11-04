// Test 03_function_unread_walk
use std::cell::RefCell;
use std::rc::Rc;

use ppt_core::cn::wps::show::pptxr::ppt::slides::PptxrSlides::{
    take_open_events, set_first_slide_io_finished, IDLProvider, KmoPresOpenEventListener,
    OpcrPackagePart, OpcrRelationship, OpcrRelationships, PptxrMedias, PptxrSlides, UnReadSlides,
};
use ppt_core::cn::wps::show::util::layout_shape::KmoPresentation;

const SLIDE_MASTER: &str =
    "http://schemas.openxmlformats.org/officeDocument/2006/relationships/slideMaster";

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

fn layout_rel_to_master(master_part: &str) -> OpcrRelationship {
    let mut layout_rels = OpcrRelationships::default();
    layout_rels.put(
        "rIdM".into(),
        OpcrRelationship {
            target: master_part.into(),
            part_name: master_part.into(),
            rels_type: SLIDE_MASTER.into(),
            part: None,
        },
    );
    OpcrRelationship {
        target: "ppt/slideLayouts/layout1.xml".into(),
        part_name: "ppt/slideLayouts/layout1.xml".into(),
        part: Some(OpcrPackagePart {
            relationships: Some(layout_rels),
            // 真实 part 的 XML 字节；本用例只走查 rels，不需要 part 内容。
            bytes: Vec::new(),
        }),
        ..Default::default()
    }
}

fn slides_with(
    rels: OpcrRelationships,
    pres: KmoPresentation,
    listener: Option<KmoPresOpenEventListener>,
    dl: Option<IDLProvider>,
) -> PptxrSlides {
    PptxrSlides::initialize();
    set_first_slide_io_finished(false);
    let _ = take_open_events();
    PptxrSlides::PptxrSlides(
        Rc::new(RefCell::new(pres)),
        Some(rels),
        listener,
        dl,
        Some(PptxrMedias::default()),
        None,
        0,
        None,
    )
}

fn attach_layout(s: &mut PptxrSlides, master_part: &str) {
    let layout = layout_rel_to_master(master_part);
    for sr in s.slides_reader_mut() {
        sr.layout_rels = Some(layout.clone());
    }
}

fn master_before_slide(events: &[String]) {
    let mut last_master = -1i32;
    let mut last_slide = -1i32;
    for (i, e) in events.iter().enumerate() {
        if e == "master.open" {
            last_master = i as i32;
        }
        if e == "slide.open" {
            assert!(
                last_master > last_slide,
                "slide.open at {i} without a newer master.open: {events:?}"
            );
            last_slide = i as i32;
        }
    }
    assert!(events.iter().any(|e| e == "slide.open"));
}

#[test]
fn t01_master_open_before_slide_open() {
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
        "m1".into(),
        rel(
            "ppt/slideMasters/s1.xml",
            "ppt/slideMasters/s1.xml",
            true,
        ),
    );
    let finishes = Rc::new(RefCell::new(Vec::new()));
    let listener = KmoPresOpenEventListener {
        finishes: finishes.clone(),
    };
    let mut s = slides_with(rels, KmoPresentation::default(), Some(listener), None);
    s.onSlideMasterIds(vec!["m1".into()]);
    s.onSlideIds(vec!["rId1".into(), "rId2".into()]);
    attach_layout(&mut s, "ppt/slideMasters/s1.xml");
    s.open().unwrap();
    let events = take_open_events();
    master_before_slide(&events);
    assert!(events.iter().any(|e| e == "openLayout"));
    assert!(events.iter().any(|e| e == "setLayout"));
    assert_eq!(*finishes.borrow(), vec![0, 1]);
    assert!(s.pres().borrow().slides.iter().all(|sl| sl.layout.is_some()));
}

#[test]
fn t02_no_master_uses_default_layout() {
    let mut rels = OpcrRelationships::default();
    rels.put(
        "rId1".into(),
        rel("ppt/slides/slide1.xml", "ppt/slides/slide1.xml", true),
    );
    let finishes = Rc::new(RefCell::new(Vec::new()));
    let listener = KmoPresOpenEventListener {
        finishes: finishes.clone(),
    };
    let mut s = slides_with(rels, KmoPresentation::default(), Some(listener), None);
    s.onSlideIds(vec!["rId1".into()]);
    s.open().unwrap();
    let events = take_open_events();
    assert!(events.iter().any(|e| e == "buildDefaultSlideMaster"));
    assert!(events.iter().any(|e| e == "buildDefaultLayout"));
    assert!(events.iter().any(|e| e == "setLayout"));
    assert_eq!(s.pres().borrow().masterCount(), 1);
    assert!(s.pres().borrow().slides[0].layout.is_some());
    assert_eq!(*finishes.borrow(), vec![0]);
}

#[test]
fn t03_first_screen_delay_create_slides() {
    let mut rels = OpcrRelationships::default();
    let mut rids = Vec::new();
    for i in 1..=12 {
        let rid = format!("rId{i}");
        rels.put(
            rid.clone(),
            rel(
                &format!("ppt/slides/slide{i}.xml"),
                &format!("ppt/slides/slide{i}.xml"),
                true,
            ),
        );
        rids.push(rid);
    }
    let mut s = slides_with(rels, KmoPresentation::default(), None, None);
    s.onSlideIds(rids);
    assert_eq!(s.slides_reader_len(), 10);
    assert_eq!(s.create_slide_count(), 10);
    s.open().unwrap();
    assert_eq!(s.slides_reader_len(), 12);
    assert_eq!(s.create_slide_count(), 12);
    assert_eq!(s.pres().borrow().slideCount(), 12);
    assert_eq!(s.pres().borrow().list_changed, 1);
}

#[test]
fn t04_force_quit_before_slide_open() {
    let mut rels = OpcrRelationships::default();
    rels.put(
        "rId1".into(),
        rel("ppt/slides/slide1.xml", "ppt/slides/slide1.xml", true),
    );
    rels.put(
        "rId2".into(),
        rel("ppt/slides/slide2.xml", "ppt/slides/slide2.xml", true),
    );
    let finishes = Rc::new(RefCell::new(Vec::new()));
    let listener = KmoPresOpenEventListener {
        finishes: finishes.clone(),
    };
    let mut s = slides_with(
        rels,
        KmoPresentation::default(),
        Some(listener),
        Some(IDLProvider { force_quit: true }),
    );
    s.onSlideIds(vec!["rId1".into(), "rId2".into()]);
    let err = s.open().unwrap_err();
    assert!(err.to_string().contains("ForceQuitException"));
    let events = take_open_events();
    assert!(!events.iter().any(|e| e == "slide.open"));
    assert!(finishes.borrow().is_empty());
}

#[test]
fn t05_unread_next_priority_then_active() {
    let mut u = UnReadSlides::UnReadSlides(5);
    let mut pres = KmoPresentation::default();
    pres.selection.priority_list = Some(vec![4, 1]);
    assert_eq!(u.next(&pres), 4);
    assert_eq!(u.next(&pres), 1);

    let mut u2 = UnReadSlides::UnReadSlides(5);
    let mut p2 = KmoPresentation::default();
    p2.selection.priority_list = None;
    p2.selection.actived_slide_index = 2;
    p2.selection.history_slide_index = -1;
    assert_eq!(u2.next(&p2), 2);
    assert_eq!(u2.next(&p2), 3);
    assert_eq!(u2.next(&p2), 1);
}
