use std::cell::RefCell;
use std::rc::Rc;

use ppt_core::cn::wps::drawing::dmlr::handler::ElementHandler;
use ppt_core::cn::wps::drawing::dmlr::handler::shape::textshape::text::text_model_surface::HandlerCallback;
use ppt_core::cn::wps::show::pptxr::handler::timing::BuildListHandler::BuildListHandler;
use ppt_core::cn::wps::show::pptxr::handler::timing::TimingHandler::TimingHandler;
use ppt_core::cn::wps::show::pptxr::handler::timing::timing_model_surface::{Build, TimeNode, Timing};

fn cb() -> Rc<RefCell<HandlerCallback>> {
    Rc::new(RefCell::new(HandlerCallback::pptx()))
}

#[test]
fn t01_empty_end_does_not_write() {
    let timing = Rc::new(RefCell::new(Timing::create()));
    let mut h = TimingHandler::TimingHandler(timing.clone(), cb());
    h.endElement(3211609);
    let t = timing.borrow();
    assert!(!t.tn_written);
    assert!(!t.bld_written);
    assert!(h.timeNodeList.is_none());
    assert!(h.buildList.is_none());
}

#[test]
fn t02_tnlst_dispatch_shares_list() {
    let timing = Rc::new(RefCell::new(Timing::create()));
    let mut h = TimingHandler::TimingHandler(timing, cb());
    let child = h.enterSubElement(3211422).expect("tnLst");
    assert_eq!(child.kind(), "TimeNodeListHandler");
    let list = h.timeNodeList.clone().expect("list");
    list.borrow_mut().push(TimeNode::create());
    assert_eq!(list.borrow().len(), 1);
}

#[test]
fn t03_build_put_types() {
    let list = Rc::new(RefCell::new(Vec::<Rc<RefCell<Build>>>::new()));
    let mut h = BuildListHandler::BuildListHandler(list.clone(), cb());
    let p = h.enterSubElement(3211433).expect("bldP");
    assert_eq!(p.kind(), "BuildParagraphHandler");
    let d = h.enterSubElement(3211434).expect("bldDgm");
    assert_eq!(d.kind(), "BuildDiagramHandler");
    let c = h.enterSubElement(3211435).expect("bldChart");
    assert_eq!(c.kind(), "OleChartBuildHandler");
    let g = h.enterSubElement(3211436).expect("bldGraphic");
    assert_eq!(g.kind(), "GraphicalObjectBuildHandler");
    let types: Vec<i32> = list.borrow().iter().map(|b| b.borrow().type_.unwrap()).collect();
    assert_eq!(types, vec![3, 0, 2, 1]);
}

#[test]
fn t04_end_writes_saved_nodes() {
    let timing = Rc::new(RefCell::new(Timing::create()));
    let mut h = TimingHandler::TimingHandler(timing.clone(), cb());
    let _ = h.enterSubElement(3211422);
    let list = h.timeNodeList.clone().unwrap();
    list.borrow_mut().push(TimeNode::create());
    let _ = h.enterSubElement(3211437);
    h.buildList.as_ref().unwrap().borrow_mut().push(Build::create());
    h.endElement(3211609);
    let t = timing.borrow();
    assert!(t.tn_written);
    assert!(t.bld_written);
    assert!(t.time_nodes.as_ref().unwrap()[0].borrow().saved);
    assert!(t.builds.as_ref().unwrap()[0].borrow().saved);
}

#[test]
fn t05_unknown_sub_is_none() {
    let timing = Rc::new(RefCell::new(Timing::create()));
    let mut h = TimingHandler::TimingHandler(timing, cb());
    assert!(h.enterSubElement(3211420).is_none());
}
