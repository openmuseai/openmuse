// Test 01_function_slide_id_map
use ppt_core::cn::wps::show::pptxr::ppt::slides::PptxrSlides::{OpcrRelationship, PptxrSlides};

#[test]
fn t01_initialize_null_then_clear() {
    PptxrSlides::initialize();
    assert_eq!(
        PptxrSlides::getSlideIdByFileName(Some("any.xml".into())),
        0
    );
    PptxrSlides::put_slide_id("slide1.xml".into(), 42);
    assert_eq!(
        PptxrSlides::getSlideIdByFileName(Some("slide1.xml".into())),
        42
    );
    PptxrSlides::initialize();
    assert_eq!(
        PptxrSlides::getSlideIdByFileName(Some("slide1.xml".into())),
        0
    );
}

#[test]
fn t02_get_slide_id_miss_zero() {
    PptxrSlides::initialize();
    assert_eq!(
        PptxrSlides::getSlideIdByFileName(Some("unknown.xml".into())),
        0
    );
}

#[test]
fn t03_relationship_target_basename() {
    assert_eq!(
        PptxrSlides::getRelationShipTargetFileName(None),
        ""
    );
    let rel = OpcrRelationship {
        target: "ppt/slides/slide1.xml".into(),
        ..Default::default()
    };
    assert_eq!(
        PptxrSlides::getRelationShipTargetFileName(Some(&rel)),
        "slide1.xml"
    );
    let rel = OpcrRelationship {
        target: "slide1.xml".into(),
        ..Default::default()
    };
    assert_eq!(
        PptxrSlides::getRelationShipTargetFileName(Some(&rel)),
        "slide1.xml"
    );
    let rel = OpcrRelationship {
        target: "a/b/".into(),
        ..Default::default()
    };
    assert_eq!(PptxrSlides::getRelationShipTargetFileName(Some(&rel)), "");
}
