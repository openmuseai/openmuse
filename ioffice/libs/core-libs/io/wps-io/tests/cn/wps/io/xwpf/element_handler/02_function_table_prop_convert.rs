// function_id=02_function_table_prop_convert
// D2: tblPr/tcPr 子元素进入 PropChildHandler / TcPrChildHandler，并改到共享的 MutablePropertySet
// cargo test --manifest-path libs/core-libs/io/wps-io/Cargo.toml --test 02_function_table_prop_convert

use core_libs_io::io::opt::AttributesImpl::AttributesImpl;
use core_libs_io::io::xwpf::IDocumentImporter::IDocumentImporter;
use core_libs_io::io::xwpf::IImporterBase::IImporterBase;
use core_libs_io::io::xwpf::element_handler::DocumentHandler::DocumentHandler;
use core_libs_io::io::xwpf::element_handler::TblHandler::TblHandler;
use core_libs_io::io::xwpf::element_handler::TcPrCellMarginChildHandler::TcPrCellMarginChildHandler;
use core_libs_io::io::xwpf::element_handler::TcPrChildCellMarginHandler::TcPrChildCellMarginHandler;
use core_libs_io::io::xwpf::element_handler::TcPrChildHandler::TcPrChildHandler;
use core_libs_io::io::xwpf::element_handler::TcPrChildTcBorederHandler::TcPrChildTcBorederHandler;
use core_libs_io::io::xwpf::element_handler::TcPrBaseHandler::TcPrBaseHandler;
use core_libs_io::io::xwpf::element_handler::TcPrChangeHandler::TcPrChangeHandler;
use core_libs_io::io::xwpf::element_handler::TrHandler::TrHandler;
use core_libs_io::io::xwpf::element_handler::prop::PropBaseHandler::PropBaseHandler;
use core_libs_io::io::xwpf::element_handler::prop::PropChangeHandler::PropChangeHandler;
use core_libs_io::io::xwpf::element_handler::prop::PropHandler::PropHandler;
use core_libs_util::moffice::property::MutablePropertySet::MutablePropertySet;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

struct Rec {
    events: RefCell<Vec<String>>,
}

impl IImporterBase for Rec {
    fn mode(&self) -> i32 {
        0
    }
}

impl IDocumentImporter for Rec {
    fn convertProp(&self, prop_type: i32, prop: &mut dyn std::any::Any, _ns: &str, element_id: i32, _attrs: &dyn std::any::Any) {
        self.events.borrow_mut().push(format!("convert {prop_type} {element_id}"));
        if let Some(set) = prop.downcast_mut::<MutablePropertySet>() {
            if element_id == 96307760 {
                set.setInt(901, 7);
            } else {
                if set.getInt_with_default(901, 0) == 7 {
                    self.events.borrow_mut().push("change-kept".to_string());
                }
                set.setInt(295, 720);
            }
        }
    }
    fn converTableCellProp(&self, prop: &mut dyn std::any::Any, element_id: i32, _attrs: &dyn std::any::Any, cell_index: i32) {
        self.events.borrow_mut().push(format!("cell-prop {element_id} cell={cell_index}"));
        if let Some(set) = prop.downcast_mut::<MutablePropertySet>() {
            if element_id == -2065786911 {
                set.setInt(902, 3);
            } else if set.getInt_with_default(902, 0) == 3 {
                self.events.borrow_mut().push("cell-change-kept".to_string());
            }
            set.setInt(114632, cell_index);
        }
    }
}

fn attrs() -> AttributesImpl {
    AttributesImpl::new_with_maps(HashMap::new(), None)
}

#[test]
fn tbl_pr_child_mutates_shared_table_prop_and_cell_pr_uses_cell_index() {
    let rec = Rc::new(Rec { events: RefCell::new(Vec::new()) });
    let importer: Rc<dyn IDocumentImporter> = rec.clone();
    let document = DocumentHandler::new_rc(importer, None);
    let body = document.getElementHandler(3029410, "").unwrap();
    let table = body.borrow().getElementHandler(114622, "").unwrap();
    let tbl_pr = table.borrow().getElementHandler(110154336, "").unwrap();
    assert!(tbl_pr.borrow().as_any().downcast_ref::<PropHandler>().is_some());

    let width = tbl_pr.borrow().getElementHandler(3553369, "").expect("tblW child");
    let width_again = tbl_pr.borrow().getElementHandler(3553369, "").expect("cached child");
    assert!(Rc::ptr_eq(&width, &width_again));
    let change = tbl_pr.borrow().getElementHandler(96307760, "").expect("tblPrChange");
    let change_again = tbl_pr.borrow().getElementHandler(96307760, "").expect("cached change");
    assert!(Rc::ptr_eq(&change, &change_again));
    assert!(change.borrow().as_any().downcast_ref::<PropChangeHandler>().is_some());
    assert!(change.borrow().getElementHandler(1, "").is_none());
    change.borrow_mut().onStart_with_id(96307760, "", "", "", &attrs()).unwrap();
    let change_base = change.borrow().getElementHandler(110154336, "").expect("base inside change");
    assert!(change_base.borrow().as_any().downcast_ref::<PropBaseHandler>().is_some());
    let change_width = change_base.borrow().getElementHandler(3553369, "").expect("tblW inside change");
    change_width.borrow_mut().onStart_with_id(3553369, "", "", "", &attrs()).unwrap();

    width.borrow_mut().onStart_with_id(3553369, "", "", "", &attrs()).unwrap();
    assert_eq!(
        table.borrow().as_any().downcast_ref::<TblHandler>().unwrap().getProp().getInt(295),
        720
    );
    assert_eq!(
        table.borrow().as_any().downcast_ref::<TblHandler>().unwrap().getProp().getInt_with_default(901, 0),
        0
    );

    let row = table.borrow().getElementHandler(3710, "").unwrap();
    row.borrow_mut().onStart_with_id(3710, "", "", "", &attrs()).unwrap();
    let cell = row.borrow().getElementHandler(3695, "").unwrap();
    let tc_pr = cell.borrow().getElementHandler(3553489, "").unwrap();
    let tc_w = tc_pr.borrow().getElementHandler(114632, "").expect("tcW");
    assert!(tc_w.borrow().as_any().downcast_ref::<TcPrChildHandler>().is_some());
    let tc_w_again = tc_pr.borrow().getElementHandler(318038960, "").expect("gridSpan same child");
    assert!(Rc::ptr_eq(&tc_w, &tc_w_again));
    assert_eq!(tc_w.borrow().as_any().downcast_ref::<TcPrChildHandler>().unwrap().mCellIndex.get(), 0);
    tc_w.borrow_mut().onStart_with_id(114632, "", "", "", &attrs()).unwrap();

    let border = tc_pr.borrow().getElementHandler(-1034858408, "").expect("tcBorders");
    let border_again = tc_pr.borrow().getElementHandler(-1034858408, "").expect("new border");
    assert!(!Rc::ptr_eq(&border, &border_again));
    assert!(border.borrow().as_any().downcast_ref::<TcPrChildTcBorederHandler>().is_some());
    assert!(!Rc::ptr_eq(&border, &tc_w));
    let top = border.borrow().getElementHandler(115029, "").expect("top border child");
    top.borrow_mut().onStart_with_id(115029, "", "", "", &attrs()).unwrap();

    let margin = tc_pr.borrow().getElementHandler(110154863, "").expect("tcMar");
    assert!(margin.borrow().as_any().downcast_ref::<TcPrChildCellMarginHandler>().is_some());
    let margin_child = margin.borrow().getElementHandler(115029, "").expect("top margin");
    let margin_child_again = margin.borrow().getElementHandler(3317767, "").expect("cached margin child");
    assert!(Rc::ptr_eq(&margin_child, &margin_child_again));
    assert!(margin_child.borrow().as_any().downcast_ref::<TcPrCellMarginChildHandler>().is_some());
    margin_child.borrow_mut().onStart_with_id(115029, "", "", "", &attrs()).unwrap();

    let tc_change = tc_pr.borrow().getElementHandler(-2065786911, "").expect("tcPrChange");
    assert!(tc_change.borrow().as_any().downcast_ref::<TcPrChangeHandler>().is_some());
    assert!(tc_change.borrow().getElementHandler(1, "").is_none());
    tc_change.borrow_mut().onStart_with_id(-2065786911, "", "", "", &attrs()).unwrap();
    let tc_change_base = tc_change.borrow().getElementHandler(3553489, "").expect("tcPr inside change");
    assert!(tc_change_base.borrow().as_any().downcast_ref::<TcPrBaseHandler>().is_some());
    let tc_change_width = tc_change_base.borrow().getElementHandler(114632, "").expect("tcW inside change");
    tc_change_width.borrow_mut().onStart_with_id(114632, "", "", "", &attrs()).unwrap();
    assert_eq!(
        row.borrow().as_any().downcast_ref::<TrHandler>().unwrap().getProp().getInt_with_default(902, 0),
        0
    );

    assert_eq!(
        rec.events.borrow().as_slice(),
        &[
            "convert 4 96307760".to_string(),
            "convert 4 3553369".to_string(),
            "change-kept".to_string(),
            "convert 4 3553369".to_string(),
            "cell-prop 114632 cell=0".to_string(),
            "cell-prop 37 cell=0".to_string(),
            "cell-prop 31 cell=0".to_string(),
            "cell-prop -2065786911 cell=0".to_string(),
            "cell-prop 114632 cell=0".to_string(),
            "cell-change-kept".to_string(),
        ]
    );
}
