// function_id=10_function_handler_props
// cargo test --manifest-path libs/core-libs/io/wps-io/Cargo.toml --test 10_function_handler_props

use std::cell::RefCell;
use std::rc::Rc;

use core_libs_io::io::xwpf::IDocumentImporter::IDocumentImporter;
use core_libs_io::io::xwpf::IImporterBase::IImporterBase;
use core_libs_io::io::xwpf::element_handler::ParagraphSimple::ParagraphSimple;
use core_libs_io::io::xwpf::element_handler::PprHandler::{PprHandler, PprSubHandler};
use core_libs_io::io::xwpf::element_handler::PHandler::PHandler;
use core_libs_util::moffice::property::MutablePropertySet::MutablePropertySet;

const NS: &str = "http://schemas.openxmlformats.org/wordprocessingml/2006/main";
const JC: i32 = 3385;
const PPR: i32 = 110226;
const RPR: i32 = 112148;
const RPR_B: i32 = 98;
const SKIP_TABS: i32 = 3403232;
const IND: i32 = 115029;

struct RecordingImporter {
    convert_ids: RefCell<Vec<i32>>,
    run_ids: RefCell<Vec<i32>>,
    paragraph_run_bold: RefCell<Option<i32>>,
}

impl RecordingImporter {
    fn new() -> Rc<Self> {
        Rc::new(Self {
            convert_ids: RefCell::new(Vec::new()),
            run_ids: RefCell::new(Vec::new()),
            paragraph_run_bold: RefCell::new(None),
        })
    }
}

impl IImporterBase for RecordingImporter {
    fn mode(&self) -> i32 {
        0
    }
}

impl IDocumentImporter for RecordingImporter {
    fn onImportParagraphEnd(&self, paragraph: &dyn std::any::Any, _sub: i32, _layer: i32) {
        let paragraph = paragraph.downcast_ref::<ParagraphSimple>().expect("paragraph model");
        *self.paragraph_run_bold.borrow_mut() = paragraph.getRunProp().map(|p| p.getInt_with_default(RPR_B, 0));
    }
    fn convertProp(
        &self,
        _var1: i32,
        prop: &mut dyn std::any::Any,
        _var3: &str,
        propId: i32,
        _var5: &dyn std::any::Any,
    ) {
        self.convert_ids.borrow_mut().push(propId);
        if let Some(p) = prop.downcast_mut::<MutablePropertySet>() {
            p.setInt(propId, 1);
        }
    }

    fn convertRunProp(
        &self,
        prop: &mut dyn std::any::Any,
        _var2: &str,
        elementId: i32,
        _var4: &dyn std::any::Any,
    ) {
        self.run_ids.borrow_mut().push(elementId);
        if let Some(p) = prop.downcast_mut::<MutablePropertySet>() {
            p.setInt(elementId, 1);
        }
    }

    fn correctProps(
        &self,
        _var1: i32,
        prop: &dyn std::any::Any,
        _var3: Option<&dyn std::any::Any>,
    ) -> Box<dyn std::any::Any + Send + Sync> {
        if let Some(p) = prop.downcast_ref::<MutablePropertySet>() {
            Box::new(MutablePropertySet::new_from_mutable(p))
        } else {
            Box::new(MutablePropertySet::new())
        }
    }
}

#[test]
fn hp7_sax_paragraph_rpr_reaches_paragraph_mark() {
    let rec = RecordingImporter::new();
    let importer: Rc<dyn IDocumentImporter> = rec.clone();
    let mut paragraph = PHandler::new_rc(dummy_part(), importer, 0, 0);
    paragraph.onStart(112, NS, "p", "w:p", &());
    let ppr = paragraph.getElementHandler_impl(PPR, NS).expect("pPr");
    use core_libs_io::io::simple::ElementHandlerSimple::ElementHandlerSimple as SaxHandler;
    let rpr = SaxHandler::getElementHandler(&*ppr.borrow(), RPR, NS).expect("pPr/rPr");
    let bold = SaxHandler::getElementHandler(&*rpr.borrow(), RPR_B, NS).expect("pPr/rPr/b");
    SaxHandler::onStart_with_id(&mut *bold.borrow_mut(), RPR_B, NS, "b", "w:b", &()).unwrap();
    SaxHandler::onEnd_with_id(&mut *rpr.borrow_mut(), RPR, NS).unwrap();
    SaxHandler::onEnd_with_id(&mut *ppr.borrow_mut(), PPR, NS).unwrap();
    paragraph.onEnd(112, NS);
    assert_eq!(*rec.paragraph_run_bold.borrow(), Some(1));
    assert_eq!(&*rec.run_ids.borrow(), &[RPR_B]);
}

fn dummy_part() -> Box<dyn std::any::Any + Send + Sync> {
    Box::new(())
}

fn make_ppr(importer: Rc<dyn IDocumentImporter>) -> PprHandler {
    PprHandler::new(dummy_part(), importer)
}

#[test]
fn hp1_jc_center_writes_bucket() {
    let rec = RecordingImporter::new();
    let importer: Rc<dyn IDocumentImporter> = rec.clone();
    let mut ppr = make_ppr(importer);
    match ppr.getElementHandler(JC, NS) {
        Some(PprSubHandler::PropChild(child)) => {
            child.onStart(JC, NS, "jc", "w:jc", &());
        }
        _other => panic!("expected PropChild for jc"),
    }
    ppr.onEnd(PPR, NS);
    let para = ParagraphSimple::new(dummy_part(), ppr);
    let prop = para.getParaProp().expect("paraProp after onEnd");
    assert!(!prop.isEmpty(), "HP1: jc must write the para bucket");
    assert!(
        rec.convert_ids.borrow().contains(&JC),
        "HP1: importer.convertProp must see jc=3385, got {:?}",
        rec.convert_ids.borrow()
    );
}

#[test]
fn hp2_empty_ppr_does_not_pollute_run() {
    let rec = RecordingImporter::new();
    let importer: Rc<dyn IDocumentImporter> = rec.clone();
    let mut ppr = make_ppr(importer);
    ppr.onEnd(PPR, NS);
    assert!(
        rec.convert_ids.borrow().is_empty(),
        "empty pPr must not call convertProp"
    );
    let para = ParagraphSimple::new(dummy_part(), ppr);
    if let Some(prop) = para.getParaProp() {
        assert!(prop.isEmpty(), "empty pPr paraProp must be empty");
    }
    assert!(para.getRunProp().is_none(), "empty pPr must not create runProp");
}

#[test]
fn hp3_rpr_b_writes_run_bucket() {
    let rec = RecordingImporter::new();
    let importer: Rc<dyn IDocumentImporter> = rec.clone();
    let mut ppr = make_ppr(importer);
    match ppr.getElementHandler(RPR, NS) {
        Some(PprSubHandler::Rpr(rpr)) => {
            let child = rpr
                .getElementHandler(RPR_B, NS)
                .expect("RprChildHandler for b");
            child.onStart(RPR_B, NS, "b", "w:b", &());
            rpr.onEnd(RPR, NS);
        }
        _other => panic!("expected Rpr for 112148"),
    }
    assert!(
        rec.run_ids.borrow().contains(&RPR_B),
        "HP3: convertRunProp must see b=98, got {:?}",
        rec.run_ids.borrow()
    );
    let run = ppr.getRunProp().expect("runProp after rPr onEnd");
    assert!(!run.isEmpty(), "HP3: run bucket must be non-empty");
    assert!(ppr.getProp().is_none(), "rPr must not fill para mPropertySet without pPr onEnd");
}

#[test]
fn hp4_clear_prop_drops_bucket() {
    let rec = RecordingImporter::new();
    let importer: Rc<dyn IDocumentImporter> = rec.clone();
    let mut ppr = make_ppr(importer);
    if let Some(PprSubHandler::PropChild(child)) = ppr.getElementHandler(JC, NS) {
        child.onStart(JC, NS, "jc", "w:jc", &());
    }
    ppr.onEnd(PPR, NS);
    assert!(ppr.getProp().is_some());
    ppr.clearProp();
    assert!(ppr.getProp().is_none(), "clearProp must null mPropertySet");
}

#[test]
fn hp5_skip_element_does_not_convert() {
    let rec = RecordingImporter::new();
    let importer: Rc<dyn IDocumentImporter> = rec.clone();
    let mut ppr = make_ppr(importer);
    if let Some(PprSubHandler::PropChild(child)) = ppr.getElementHandler(SKIP_TABS, NS) {
        child.onStart(SKIP_TABS, NS, "tabs", "w:tabs", &());
    } else {
        panic!("default getElementHandler should still return PropChildHandler");
    }
    ppr.onEnd(PPR, NS);
    assert!(
        !rec.convert_ids.borrow().contains(&SKIP_TABS),
        "3403232 convertProp returns -1"
    );
    if let Some(prop) = ppr.getProp() {
        assert!(prop.isEmpty());
    }
}

#[test]
fn hp6_third_sub_handler_sets_parent_id() {
    let rec = RecordingImporter::new();
    let importer: Rc<dyn IDocumentImporter> = rec.clone();
    let mut ppr = make_ppr(importer);
    let parent_id = match ppr.getElementHandler(JC, NS) {
        Some(PprSubHandler::PropChild(child)) => {
            child.onStart(JC, NS, "jc", "w:jc", &());
            let grand = child
                .getElementHandler(IND, NS)
                .expect("115029 isGetThirdSubElementHandler");
            grand.mParentId
        }
        _ => panic!("need PropChild"),
    };
    assert_eq!(parent_id, JC, "grandchild mParentId must be filterIndex/jc");
}
