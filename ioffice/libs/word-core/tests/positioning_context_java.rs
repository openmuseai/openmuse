#![allow(non_snake_case)]
use std::collections::HashMap;
use io_wps_io::cn::wps::io::dom::io::WMap::WMap;
use io_wps_io::cn::wps::io::xwpf::SaxAttributes::SaxAttributes;
use word_core::cn::wps::moffice::writer::io::reader::dmlReader::PositioningContext::PositioningContext;
use word_core::cn::wps::moffice::writer::io::reader::dmlReader::context::Tile::Tile;
use word_core::cn::wps::moffice::writer::io::reader::dmlReader::helper::EffectHelper::EffectHelper;
use drawing::cn::wps::moffice::drawing::graphics::TileFlipMode::TileFlipMode;
use word_core::cn::wps::moffice::writer::io::reader::dmlReader::context::{
    ColorChoice::ColorChoice, Duotone::Duotone, ScrgbClr::ScrgbClr,
    SrgbClr::SrgbClr, SysClr::SysClr, SchemeClr::SchemeClr, PrstClr::PrstClr,
};

#[derive(Default)]
struct Attrs(HashMap<(String, String), String>);
impl Attrs {
    fn with(mut self, namespace: &str, name: &str, value: &str) -> Self {
        self.0.insert((namespace.into(), name.into()), value.into()); self
    }
}
impl SaxAttributes for Attrs {
    fn getLength(&self) -> usize { self.0.len() }
    fn getURI(&self, _: usize) -> Option<String> { None }
    fn getLocalName(&self, _: usize) -> Option<String> { None }
    fn getQName(&self, _: usize) -> Option<String> { None }
    fn getType_by_index(&self, _: usize) -> Option<String> { None }
    fn getValue_by_index(&self, _: usize) -> Option<String> { None }
    fn getIndex_by_uri(&self, _: &str, _: &str) -> i32 { -1 }
    fn getIndex_by_qname(&self, _: &str) -> i32 { -1 }
    fn getType_by_uri(&self, _: &str, _: &str) -> Option<String> { None }
    fn getType_by_qname(&self, _: &str) -> Option<String> { None }
    fn getValue_by_uri(&self, uri: &str, localName: &str) -> Option<String> {
        self.0.get(&(uri.into(), localName.into())).cloned()
    }
    fn getValue_by_qname(&self, _: &str) -> Option<String> { None }
}

#[test]
fn java_inline_and_anchor_state() {
    let mut ctxt = PositioningContext::new();
    assert_eq!(ctxt.mWrapType, 3);
    assert_eq!(ctxt.mPositionH, None);
    let extent = Attrs::default().with("", "cx", "12700").with("", "cy", "25400");
    ctxt.setExtent(&extent);
    assert_eq!((ctxt.mWidth, ctxt.mHeight), (Some(12700), Some(25400)));
    let attrs = Attrs::default().with("", "distL", "12700").with("", "behindDoc", "1")
        .with("", "locked", "false").with("", "relativeHeight", "4");
    ctxt.setAnchorAttrs(&attrs);
    assert_eq!(ctxt.mDistanceL, Some(1.0));
    assert!(ctxt.mBehindDoc);
    assert!(!ctxt.mLocked);
    assert_eq!(ctxt.mRelativeHeight, Some(4));
    ctxt.setPositionH("42");
    assert_eq!(ctxt.mPositionH, Some(42));
    ctxt.setDocPr(&Attrs::default().with("", "name", "Picture 1").with("", "descr", "cover"));
    assert_eq!(ctxt.mName.as_deref(), Some("Picture 1"));
    assert_eq!(ctxt.mDescr.as_deref(), Some("cover"));
}

#[test]
fn java_wrap_and_relative_position_codes() {
    let mut ctxt = PositioningContext::new();
    let wrap = Attrs::default().with("", "distL", "12700")
        .with(WMap::MAIN_NAMESPACING(), "wrapText", "largest");
    ctxt.setWrapInfo(-1292678373, &wrap);
    assert_eq!(ctxt.mWrapType, 5);
    assert_eq!(ctxt.mWrapText, 3);
    assert_eq!(ctxt.mDistanceL, Some(1.0));
    ctxt.setPositionRelH(&Attrs::default().with(WMap::MAIN_NAMESPACING(), "relativeFrom", "column"));
    ctxt.setPositionRelV(&Attrs::default().with(WMap::MAIN_NAMESPACING(), "relativeFrom", "paragraph"));
    assert_eq!((ctxt.mPosRelH, ctxt.mPosRelV), (2, 2));
    ctxt.setPositionAlignH(Some("center"));
    ctxt.setPositionAlignV(Some("bottom"));
    assert_eq!((ctxt.mPosAlignH, ctxt.mPosAlignV), (Some(2), Some(3)));
    ctxt.setWrapPolygon(0, &Attrs::default().with("", "edited", "1"));
    ctxt.setWrapcoords(0, &Attrs::default().with("", "x", "17").with("", "y", "-3"));
    let points = ctxt.mWrapcoords.as_ref().unwrap();
    assert_eq!((points[0].x, points[0].y), (17.0, -3.0));
}

#[test]
fn java_tile_and_effect_alignment() {
    let attrs = Attrs::default().with("", "tx", "12700").with("", "ty", "25400")
        .with("", "sx", "50000").with("", "sy", "100000")
        .with(WMap::MAIN_NAMESPACING(), "algn", "ctr")
        .with(WMap::MAIN_NAMESPACING(), "flip", "xy");
    let tile = Tile::new(&attrs);
    assert_eq!((tile.mTx, tile.mTy), (Some(1.0), Some(2.0)));
    assert_eq!((tile.mSx, tile.mSy), (Some(0.5), Some(1.0)));
    assert_eq!(tile.mAlgn, 3);
    assert!(tile.mFlip == TileFlipMode::xy);
    assert_eq!(EffectHelper::string2RectAlignmentType(Some("none")), Some(-1));
    assert_eq!(EffectHelper::string2RectAlignmentType(Some("unknown")), None);
    let edge = EffectHelper::createSoftEdge(Some(&Attrs::default().with("", "rad", "25400"))).unwrap();
    assert_eq!(edge.getRad(), 2);
    let reflection_attrs = Attrs::default().with("", "blurRad", "12700")
        .with("", "rotWithShape", "false")
        .with(WMap::MAIN_NAMESPACING(), "stA", "50000")
        .with(WMap::MAIN_NAMESPACING(), "dir", "60000")
        .with(WMap::MAIN_NAMESPACING(), "algn", "ctr");
    let reflection = EffectHelper::createReflection(Some(&reflection_attrs)).unwrap();
    assert_eq!(reflection.getReflectionBlurRad(), 1.0);
    assert_eq!(reflection.getReflectionStartAlpha(), 0.5);
    assert_eq!(reflection.getReflectionDir(), 1.0);
    assert_eq!(reflection.getReflectionAlgn(), 3);
    assert!(!reflection.getReflectionRotWithShape());
}

#[test]
fn java_picture_color_contexts() {
    let srgb = SrgbClr::new(&Attrs::default().with("", "val", "00FF00"));
    assert_eq!(srgb.mVal, Some(0x00ff00));
    assert_eq!(srgb.mAlpha, None);
    let scrgb = ScrgbClr::new(&Attrs::default().with("", "r", "FF").with("", "g", "80"));
    assert_eq!((scrgb.mRed, scrgb.mGreen, scrgb.mBlue), (Some(255), Some(128), None));
    let sys = SysClr::new(&Attrs::default().with(WMap::MAIN_NAMESPACING(), "val", "window")
        .with("", "lastClr", "FFFFFF"));
    assert_eq!(sys.mSysClr.as_deref(), Some("window"));
    assert_eq!(sys.mSysLastClr, Some(0xffffff));
    let scheme = SchemeClr::new(&Attrs::default().with(WMap::MAIN_NAMESPACING(), "val", "accent1"));
    let preset = PrstClr::new(&Attrs::default().with(WMap::MAIN_NAMESPACING(), "val", "red"));
    assert!(scheme.mVal.is_some());
    assert!(preset.mVal.is_some());
    let mut duo = Duotone::default();
    duo.addColor(ColorChoice { mSrgbClr: Some(srgb), ..ColorChoice::default() });
    duo.addColor(ColorChoice { mSchemeClr: Some(scheme), ..ColorChoice::default() });
    assert_eq!(duo.size(), 2);
    assert_eq!(duo.getColorChoiceByIndex(0).mSrgbClr.as_ref().unwrap().mVal, Some(0x00ff00));
}
