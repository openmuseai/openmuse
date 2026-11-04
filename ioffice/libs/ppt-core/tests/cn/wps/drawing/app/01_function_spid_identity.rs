//! 01_function_spid_identity — DgStoreData $spt/$spid and KmoShowShape.type()/id()

use ppt_core::cn::wps::drawing::app::DgStoreData::DgStoreData;
use ppt_core::cn::wps::show::app::shape::KmoShapeTree::KmoShapeTree;
use ppt_core::cn::wps::show::app::shape::KmoShowShape::KmoShowShape;
use ppt_core::cn::wps::show::app::slide::ISlide::{
    Background, ColorMapping, ISlide, KmoPresentation, KmoSlideEventListeners, KmoTheme, KsoWMInfo,
};

struct DummySlide {
    pres: KmoPresentation,
    theme: KmoTheme,
    events: KmoSlideEventListeners,
    bg: Background,
    wm: KsoWMInfo,
    cm: ColorMapping,
}

impl DummySlide {
    fn new() -> Self {
        DummySlide {
            pres: KmoPresentation::default(),
            theme: KmoTheme,
            events: KmoSlideEventListeners::new(),
            // rust: ★W6c-4 —— 三个真身类型不再是单元结构体 ⇒ 必须构造实例。
            //   owner: tests    wave: M-PPT-KMOSLIDE-W6c-4
            bg: Background::new(),
            wm: KsoWMInfo::new(),
            cm: ColorMapping::create(),
        }
    }
}

impl ISlide for DummySlide {
    fn r#type(&self) -> i32 {
        0
    }
    fn id(&self) -> i64 {
        0
    }
    fn getPresentation(&self) -> &KmoPresentation {
        &self.pres
    }
    fn theme(&self) -> &KmoTheme {
        &self.theme
    }
    fn eventListeners(&self) -> &KmoSlideEventListeners {
        &self.events
    }
    fn spTree(&self) -> &KmoShapeTree {
        panic!("DummySlide.spTree is unused by 01_function_spid_identity")
    }
    fn replaceSpid(&self, _var1: i32, _var2: i32) {}
    fn background(&self) -> &Background {
        &self.bg
    }
    fn ksoWMInfo(&self) -> &KsoWMInfo {
        &self.wm
    }
    fn colorMapping(&self) -> &ColorMapping {
        &self.cm
    }
    fn isIOReading(&self) -> bool {
        false
    }
}

#[test]
fn id1_query_type_high_bits() {
    assert_eq!(DgStoreData::queryType((1 << 20) | 7), 1);
}

#[test]
fn id2_query_type_zero() {
    assert_eq!(DgStoreData::queryType(0), 0);
}

#[test]
fn id3_store_index2id_roundtrip() {
    let store = DgStoreData::DgStoreData_new();
    let id = store.helper_storeIndex2Id(1, 3);
    assert_eq!(DgStoreData::queryType(id), 1);
    assert_eq!(id & 0x000F_FFFF, 3);
}

#[test]
fn id4_types_0_through_5() {
    let store = DgStoreData::DgStoreData_new();
    for t in 0..=5 {
        let id = store.helper_storeIndex2Id(t, 0);
        assert_eq!(DgStoreData::queryType(id), t, "type {t}");
        assert_eq!(id & 0x000F_FFFF, 0);
    }
}

#[test]
fn id5_kmo_type_delegates_query_type() {
    let store = DgStoreData::DgStoreData_new();
    let spid = store.helper_storeIndex2Id(1, 9);
    let dummy = DummySlide::new();
    let shape = KmoShowShape::KmoShowShape__islide_int_int(&dummy, 1, spid);
    assert_eq!(shape.id(), spid);
    assert_eq!(shape.r#type(), 1);
    assert_eq!(shape.r#type(), DgStoreData::queryType(spid));
}
