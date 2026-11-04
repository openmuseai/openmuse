use ss_core::cn::wps::et::ss::formula::ptg::Ptg::{err_ptg, PtgKind};
use ss_core::cn::wps::et::ss::interfaces::IEvalName::IEvalName;
use ss_core::cn::wps::moss::engine::eval::EvaluationName::EvaluationName;
use ss_core::cn::wps::moss::engine::linktb::NameRecord::NameRecord;
use ss_core::org::apache::poi::ss::SpreadsheetVersion::SpreadsheetVersion;
use ss_core::java_runtime::{Rc, RefCell};

#[test]
fn evaluation_name_follows_java_hidden_xlfn_function_rule() {
    let mut record = NameRecord::NameRecord(SpreadsheetVersion::EXCEL2007);
    record.setNameText("_xlfn.CONCAT");
    record.setHidden(true);
    record.setNameDefinition(vec![err_ptg(29)]);
    let record = Rc::new(RefCell::new(record));
    let name = EvaluationName::EvaluationName(Some(record.clone()), 4);
    let iface: &dyn IEvalName = &name;
    assert_eq!(iface.getNameText(), "_xlfn.CONCAT");
    assert!(iface.isFunctionName());
    assert!(name.hasFormula());
    assert!(name.isFunctionName());
    assert_eq!(name.getNameText(), "_xlfn.CONCAT");
    assert_eq!(name.getNameDefinition().borrow()[0].as_ref().unwrap().borrow().getSid(), 28);
    assert!(matches!(name.createNamePtg().kind, PtgKind::Name { index: 4 }));
    assert!(matches!(name.createNameXPtg(7).kind, PtgKind::NameX { sheet_ref: 7, name_number: 5 }));
    record.borrow_mut().setNameDefinition(vec![err_ptg(7)]);
    assert!(!name.isFunctionName());
}
