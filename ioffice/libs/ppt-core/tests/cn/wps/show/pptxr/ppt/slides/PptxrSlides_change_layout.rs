// Test 04_function_change_layout
use std::rc::Rc;

use ppt_core::cn::wps::show::pptxr::ppt::slides::PptxrSlides::PptxrSlides;
use ppt_core::cn::wps::show::util::layout_shape::{
    HeaderFooter, KmoShapeTree, KmoShowShape, KmoSlide, KmoSlideLayout, KmoSlideMaster, Placeholder,
};

fn ph_shape(shape_type: i32, placeholder: bool, ph_type: i32) -> KmoShowShape {
    KmoShowShape {
        shape_type,
        is_placeholder: placeholder,
        ph: Placeholder {
            has_type: true,
            type_: ph_type,
            ..Default::default()
        },
        ..Default::default()
    }
}

fn layout_with_phs(phs: Vec<KmoShowShape>, master: KmoSlideMaster) -> KmoSlideLayout {
    KmoSlideLayout {
        master: Some(Rc::new(master)),
        sp_tree: Some(Rc::new(KmoShapeTree {
            children: phs,
            ..Default::default()
        })),
        ..Default::default()
    }
}

#[test]
fn t01_type_not_one_or_not_placeholder_not_copied() {
    let mut slide = KmoSlide::default();
    let layout = layout_with_phs(
        vec![
            ph_shape(2, true, 0),
            ph_shape(1, false, 0),
        ],
        KmoSlideMaster::default(),
    );
    PptxrSlides::changeLayout(&mut slide, &layout);
    assert!(slide.layout.is_some());
    assert!(slide.spTree().group_transform_inited);
    assert_eq!(slide.spTree().insert_calls, 1);
    assert_eq!(slide.spTree().last_insert_len, 0);
    assert_eq!(slide.spTree().last_insert_index, 0);
    assert_eq!(slide.spTree().shapeCount(), 0);
}

#[test]
fn t02_ph_456_header_footer() {
    let mut layout = KmoSlideLayout {
        master: Some(Rc::new(KmoSlideMaster {
            header_footer: None,
            ..Default::default()
        })),
        ..Default::default()
    };
    let dt = ph_shape(1, true, 4);
    assert!(!PptxrSlides::shapeNeedCopyFromLayout(&layout, &dt));

    let mut copy_ok = |ph_type: i32, hf: HeaderFooter, expect: bool| {
        layout.master = Some(Rc::new(KmoSlideMaster {
            header_footer: Some(hf),
            ..Default::default()
        }));
        let sp = ph_shape(1, true, ph_type);
        assert_eq!(PptxrSlides::shapeNeedCopyFromLayout(&layout, &sp), expect);
    };

    copy_ok(
        4,
        HeaderFooter {
            has_date_time: false,
            is_date_time: true,
            ..Default::default()
        },
        true,
    );
    copy_ok(
        4,
        HeaderFooter {
            has_date_time: true,
            is_date_time: false,
            ..Default::default()
        },
        false,
    );
    copy_ok(
        5,
        HeaderFooter {
            has_slide_num: false,
            is_slide_num: true,
            ..Default::default()
        },
        true,
    );
    copy_ok(
        5,
        HeaderFooter {
            has_slide_num: true,
            is_slide_num: false,
            ..Default::default()
        },
        false,
    );
    copy_ok(
        6,
        HeaderFooter {
            has_footer: false,
            is_footer: true,
            ..Default::default()
        },
        true,
    );
    copy_ok(
        6,
        HeaderFooter {
            has_footer: true,
            is_footer: false,
            ..Default::default()
        },
        false,
    );
}

#[test]
fn t03_copy_clears_text_and_empty_phs_skip_insert() {
    let mut slide = KmoSlide::default();
    let empty_layout = layout_with_phs(vec![], KmoSlideMaster::default());
    PptxrSlides::changeLayout(&mut slide, &empty_layout);
    assert!(slide.spTree().group_transform_inited);
    assert_eq!(slide.spTree().insert_calls, 0);

    let mut slide2 = KmoSlide::default();
    let layout = layout_with_phs(vec![ph_shape(1, true, 0)], KmoSlideMaster::default());
    PptxrSlides::changeLayout(&mut slide2, &layout);
    assert_eq!(slide2.spTree().insert_calls, 1);
    assert_eq!(slide2.spTree().last_insert_index, 0);
    assert_eq!(slide2.spTree().shapeCount(), 1);
    let copied = slide2.spTree().shapeAt(0);
    assert!(copied.no_grp);
    assert!(copied.isPlaceHolder());
    assert_eq!(copied.ph().r#type(), 0);
    assert!(copied.cleared_text);
    assert!(PptxrSlides::shapeNeedClearContentText(&copied));
}
