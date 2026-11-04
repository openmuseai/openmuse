use std::cell::Cell;
use std::collections::HashMap;
use std::rc::Rc;

use ppt_core::cn::wps::show::pptxr::ppt::slides::HandlerCallbackImpl::{
    compile_surface::{Blip, DiagramPair, OpcrRelationship, OpcrRelationships, PptxrMedias, Shape},
    HandlerCallbackImpl,
};
use ppt_core::cn::wps::show::pptxr::ppt::slides::SlideHandlerCallbackImpl::SlideHandlerCallbackImpl;

fn empty_callback() -> HandlerCallbackImpl {
    HandlerCallbackImpl::HandlerCallbackImpl(None, None, None, None, None, None, 0)
}

#[test]
fn t01_load_empty_maps() {
    let mut cb = empty_callback();
    cb.load();
    assert!(cb.getChartMap().is_empty());
    assert!(cb.getDiagramPairs().is_empty());
    assert!(cb.getRidPicMediaIds().is_none());
    assert_eq!(cb.getCoreShapeId(42), 42);
}

#[test]
fn t02_blip_embed_open_media_once() {
    let mut rels = OpcrRelationships::default();
    rels.rels.insert(
        "rId1".to_string(),
        OpcrRelationship {
            id: "rId1".to_string(),
        },
    );
    let open_count = Rc::new(Cell::new(0));
    let medias = PptxrMedias {
        open_count: open_count.clone(),
        last_id: 0,
    };
    let mut cb = HandlerCallbackImpl::HandlerCallbackImpl(
        None,
        None,
        None,
        Some(rels),
        Some(medias),
        None,
        0,
    );
    let mut blip = Blip::default();
    cb.onBlipEmbed("rId1".to_string(), &mut blip);
    assert_eq!(open_count.get(), 1);
    assert_eq!(blip.embedMediaID, Some(1));
    let ids = cb.getRidPicMediaIds().expect("rid map");
    assert_eq!(ids.get("rId1"), Some(&1));

    let mut miss = Blip::default();
    cb.onBlipEmbed("rIdMissing".to_string(), &mut miss);
    assert_eq!(open_count.get(), 1);
    assert!(miss.embedMediaID.is_none());

    let mut empty_embed = Blip::default();
    cb.onBlipEmbed(String::new(), &mut empty_embed);
    assert_eq!(open_count.get(), 1);

    let mut slide = SlideHandlerCallbackImpl::SlideHandlerCallbackImpl(
        None, None, None, None, None, None, 0,
    );
    slide.load();
}

#[test]
fn t03_chart_diagram_collect_only() {
    let mut cb = empty_callback();
    cb.onChartRelationShip("rIdChart".to_string(), Shape::default());
    cb.addDiagramPair(
        7,
        "c".to_string(),
        "d".to_string(),
        "l".to_string(),
        "q".to_string(),
    );
    cb.load();
    assert_eq!(cb.getChartMap().len(), 1);
    assert!(cb.getChartMap().contains_key("rIdChart"));
    let pair = cb.getDiagramPairs().get(&7).expect("diagram");
    assert_eq!(
        pair,
        &DiagramPair::new("c".to_string(), "q".to_string(), "l".to_string(), "d".to_string())
    );

    let mut slide = SlideHandlerCallbackImpl::SlideHandlerCallbackImpl(
        None, None, None, None, None, None, 0,
    );
    slide.onChartRelationShip("rId2".to_string(), Shape::default());
    slide.addDiagramPair(
        1,
        "c".to_string(),
        "d".to_string(),
        "l".to_string(),
        "q".to_string(),
    );
    assert_eq!(slide.getChartMap().len(), 1);
    assert_eq!(slide.getDiagramPairs().len(), 1);
    let _unused: HashMap<String, Shape> = HashMap::new();
}
