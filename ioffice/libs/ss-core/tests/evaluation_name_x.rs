use ss_core::cn::wps::et::ss::formula::ptg::Ptg::PtgKind;
use ss_core::cn::wps::moss::app::KmoBook::KmoBook;
use ss_core::cn::wps::moss::engine::eval::EvaluationExtName::EvaluationExtName;
use ss_core::cn::wps::moss::engine::eval::EvaluationName::EvaluationName;
use ss_core::cn::wps::moss::engine::eval::EvaluationWorkbook::EvaluationWorkbook;
use ss_core::cn::wps::moss::engine::linktb::NameRecord::NameRecord;

#[test]
fn name_x_resolves_local_and_addin_names_to_java_dynamic_types() {
    let book = KmoBook::KmoBook__parent(false);
    book.appendSheet();
    let workbook = EvaluationWorkbook::EvaluationWorkbook(&book);
    let mut local_record = NameRecord::NameRecord(book.getSpreadsheetVersion());
    local_record.setNameText("LOCAL_NAME");
    let local_sheet_ref = {
        let link = book.getLinkTable();
        let mut table = link.borrow_mut();
        table.addName(local_record);
        table.checkNullSupBookExternSheet(0)
    };
    let local = workbook.getNameByNameXPtg(local_sheet_ref, 0).unwrap();
    assert!(local.as_any().is::<EvaluationName>());
    assert_eq!(local.getNameText(), "LOCAL_NAME");
    let local_sheet = workbook.getExtSheet(local_sheet_ref);
    assert_eq!(local_sheet.getBookType(), 1);
    // Fresh original Android R42 observation: a local SupBookRecord has a null
    // encoded URL, and the original getBookName -> getURL path throws NPE.
    let root=std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let evidence=root.join("rewrite-review/milestones/m4-edit-save/evidence/null-supbook-r0b42");
    let expected:serde_json::Value=serde_json::from_slice(&std::fs::read(evidence.join("java.json")).unwrap()).unwrap();
    let failure=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||local_sheet.getBookName())).expect_err("original local SupBook URL is null");
    let message=failure.downcast_ref::<String>().map(String::as_str).or_else(||failure.downcast_ref::<&str>().copied()).unwrap_or("");
    assert!(message.starts_with("NullPointerException"));
    let actual=serde_json::json!({"bookType":local_sheet.getBookType(),"bookName":{"result":"NullPointerException"}});
    assert_eq!(actual,expected);
    std::fs::write(evidence.join("rust.json"),serde_json::to_vec_pretty(&actual).unwrap()).unwrap();
    assert_eq!(local_sheet.getSheetName(0), Some(book.sheetAt(0).borrow().name().to_string()));
    assert_eq!(local_sheet.getSheetName(-1), None);

    let name_x = book.getLinkTable().borrow_mut().checkUDFName("MY_UNKNOWN_UDF");
    let PtgKind::NameX { sheet_ref, name_number } = name_x.kind else { panic!("NameXPtg") };
    let external = workbook.getNameByNameXPtg(sheet_ref, name_number - 1).unwrap();
    assert!(external.as_any().is::<EvaluationExtName>());
    assert_eq!(external.getNameText(), "MY_UNKNOWN_UDF");
    assert_eq!(external.getSheetIndexOneBased(), 0);
    let external = external.as_any().downcast_ref::<EvaluationExtName>().unwrap();
    let definition = external.getNameDefinition().unwrap();
    assert!(matches!(definition[0].kind, PtgKind::Err(23)));
}
