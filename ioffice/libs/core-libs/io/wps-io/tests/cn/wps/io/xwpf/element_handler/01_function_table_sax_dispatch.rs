// function_id=01_function_table_sax_dispatch
// D1: DocumentHandler → DocumentBodyHandler → TblHandler → TrHandler → TcHandler → PHandler
// cargo test --manifest-path libs/core-libs/io/wps-io/Cargo.toml --test 01_function_table_sax_dispatch

use core_libs_io::io::opt::AttributesImpl::AttributesImpl;
use core_libs_io::io::opt::XmlStringBuilder::XmlStringBuilder;
use core_libs_io::io::xwpf::IDocumentImporter::IDocumentImporter;
use core_libs_io::io::xwpf::IImporterBase::IImporterBase;
use core_libs_io::io::xwpf::element_handler::DocumentHandler::DocumentHandler;
use core_libs_io::io::xwpf::element_handler::PHandler::PHandler;
use core_libs_io::io::xwpf::element_handler::TblHandler::TblHandler;
use core_libs_io::io::xwpf::element_handler::TcHandler::TcHandler;
use core_libs_io::io::xwpf::element_handler::TcPrHandler::TcPrHandler;
use core_libs_io::io::xwpf::element_handler::prop::PropHandler::PropHandler;
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
    fn onImportTableStart(&self, sub: i32, layer: i32) {
        self.events.borrow_mut().push(format!("table-start {sub} {layer}"));
    }
    fn onImportTableRowStart(&self, sub: i32, para: &str, _prop: &dyn std::any::Any) {
        self.events.borrow_mut().push(format!("row-start {sub} para={para}"));
    }
    fn onImportTableRowEnd(
        &self,
        sub: i32,
        layer: i32,
        cells: i32,
        _row: &dyn std::any::Any,
        _change: &dyn std::any::Any,
    ) {
        self.events
            .borrow_mut()
            .push(format!("row-end {sub} {layer} cells={cells}"));
    }
    fn onImportTableCelStart(&self, sub: i32) {
        self.events.borrow_mut().push(format!("cell-start {sub}"));
    }
    fn onImportTableCellEnd(&self, sub: i32, layer: i32) {
        self.events.borrow_mut().push(format!("cell-end {sub} {layer}"));
    }
}

fn empty_attrs() -> AttributesImpl {
    AttributesImpl::new_with_maps(HashMap::new(), None)
}

fn para_id_attrs(value: &str) -> AttributesImpl {
    let mut name = XmlStringBuilder::new();
    for ch in "paraId".chars() {
        name.append_char(ch);
    }
    let mut text = XmlStringBuilder::new();
    for ch in value.chars() {
        text.append_char(ch);
    }
    let mut attrs = AttributesImpl::new_with_maps(HashMap::new(), None);
    attrs.addAttriteName(&name);
    attrs.addAttriteValue(&text);
    attrs
}

fn start(handler: &core_libs_io::io::simple::ElementHandlerSimple::ElementHandlerRef, id: i32, attrs: &AttributesImpl) {
    handler
        .borrow_mut()
        .onStart_with_id(id, "", "", "", attrs)
        .unwrap();
}

fn end(handler: &core_libs_io::io::simple::ElementHandlerSimple::ElementHandlerRef, id: i32) {
    handler.borrow_mut().onEnd_with_id(id, "").unwrap();
}

#[test]
fn document_handler_dispatches_body_then_paragraph_and_table() {
    let rec = Rc::new(Rec { events: RefCell::new(Vec::new()) });
    let importer: Rc<dyn IDocumentImporter> = rec.clone();
    let document = DocumentHandler::new_rc(importer, None);

    assert!(document.getElementHandler(1, "").is_none());

    let body = document.getElementHandler(3029410, "").expect("body");
    let body_again = document.getElementHandler(3029410, "").expect("body cache");
    assert!(Rc::ptr_eq(&body, &body_again));

    let paragraph = body.borrow().getElementHandler(112, "").expect("p");
    let paragraph_again = body.borrow().getElementHandler(112, "").expect("p cache");
    assert!(Rc::ptr_eq(&paragraph, &paragraph_again));
    assert!(paragraph.borrow().as_any().downcast_ref::<PHandler>().is_some());
    assert!(body.borrow().getElementHandler(1, "").is_none());
    assert!(body.borrow().getElementHandler(-906275931, "").is_some());

    let table = body.borrow().getElementHandler(114622, "").expect("tbl");
    let table_again = body.borrow().getElementHandler(114622, "").expect("tbl cache");
    assert!(Rc::ptr_eq(&table, &table_again));
    assert_eq!(
        table.borrow().as_any().downcast_ref::<TblHandler>().unwrap().table_layer(),
        1
    );

    start(&table, 114622, &empty_attrs());
    let tbl_pr = table.borrow().getElementHandler(110154336, "").expect("tblPr");
    let tbl_pr_again = table.borrow().getElementHandler(110154336, "").expect("tblPr cache");
    assert!(Rc::ptr_eq(&tbl_pr, &tbl_pr_again));
    assert!(tbl_pr.borrow().as_any().downcast_ref::<PropHandler>().is_some());
    let tbl_grid = table.borrow().getElementHandler(-1516130268, "").expect("tblGrid");
    assert!(!Rc::ptr_eq(&tbl_pr, &tbl_grid));
    assert!(tbl_grid.borrow().as_any().downcast_ref::<PropHandler>().is_some());
    assert!(table.borrow().getElementHandler(1, "").is_none());
    // sdt / alternateContent 仍未接成真实 handler，本项不记通过。
    assert!(table.borrow().getElementHandler(113731, "").is_none());
    assert!(table.borrow().getElementHandler(1590178495, "").is_none());

    let row = table.borrow().getElementHandler(3710, "").expect("tr");
    let row_again = table.borrow().getElementHandler(3710, "").expect("tr cache");
    assert!(Rc::ptr_eq(&row, &row_again));
    let attrs = para_id_attrs("ROW1");
    start(&row, 3710, &attrs);

    let cell0 = row.borrow().getElementHandler(3695, "").expect("tc0");
    assert_eq!(
        cell0.borrow().as_any().downcast_ref::<TcHandler>().unwrap().mCellIndex.get(),
        0
    );
    let cell1 = row.borrow().getElementHandler(3695, "").expect("tc1");
    assert!(Rc::ptr_eq(&cell0, &cell1));
    assert_eq!(
        cell1.borrow().as_any().downcast_ref::<TcHandler>().unwrap().mCellIndex.get(),
        1
    );

    start(&cell1, 3695, &empty_attrs());
    let cell_p = cell1.borrow().getElementHandler(112, "").expect("cell p");
    let cell_p_again = cell1.borrow().getElementHandler(112, "").expect("cell p cache");
    assert!(Rc::ptr_eq(&cell_p, &cell_p_again));
    assert!(cell_p.borrow().as_any().downcast_ref::<PHandler>().is_some());
    let nested = cell1.borrow().getElementHandler(114622, "").expect("nested tbl");
    assert_eq!(
        nested.borrow().as_any().downcast_ref::<TblHandler>().unwrap().table_layer(),
        2
    );
    let tc_pr = cell1.borrow().getElementHandler(3553489, "").expect("tcPr");
    assert!(tc_pr.borrow().as_any().downcast_ref::<TcPrHandler>().unwrap().mCellIndex.get() == 1);
    let tc_pr_again = cell1.borrow().getElementHandler(3553489, "").expect("tcPr cache");
    assert!(Rc::ptr_eq(&tc_pr, &tc_pr_again));
    assert!(cell1.borrow().getElementHandler(378782847, "").is_none());
    assert!(cell1.borrow().getElementHandler(1, "").is_none());
    end(&cell1, 3695);

    let tr_pr = row.borrow().getElementHandler(3567904, "").expect("trPr");
    let tr_pr_ex = row.borrow().getElementHandler(-1515863245, "").expect("tblPrEx");
    assert!(tr_pr.borrow().as_any().downcast_ref::<PropHandler>().is_some());
    assert!(tr_pr_ex.borrow().as_any().downcast_ref::<PropHandler>().is_some());
    assert!(!Rc::ptr_eq(&tr_pr, &tr_pr_ex));
    assert!(row.borrow().getElementHandler(113731, "").is_none());
    assert!(row.borrow().getElementHandler(1, "").is_none());
    end(&row, 3710);

    start(&row, 3710, &empty_attrs());
    let cell_reset = row.borrow().getElementHandler(3695, "").expect("tc reset");
    assert!(Rc::ptr_eq(&cell0, &cell_reset));
    assert_eq!(
        cell_reset.borrow().as_any().downcast_ref::<TcHandler>().unwrap().mCellIndex.get(),
        0
    );
    end(&row, 3710);
    end(&table, 114622);

    assert_eq!(
        rec.events.borrow().as_slice(),
        &[
            "table-start 0 1",
            "row-start 0 para=ROW1",
            "cell-start 0",
            "cell-end 0 1",
            "row-end 0 1 cells=2",
            "row-start 0 para=ROW1",
            "row-end 0 1 cells=1",
        ]
    );
}
