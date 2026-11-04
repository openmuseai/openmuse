use serde_json::{json,Value};
use ss_core::{java_runtime::Rc,cn::wps::{moss::{app::{KmoBookApi::KmoBookApi,KmoSheet::KmoSheet},engine::{grid::{ExpFmlManager::ExpFml,KmoFormulaData::KmoFormulaData},eval::EvaluationWorkbook::EvaluationWorkbook}},et::ss::{formula::{Formula::Formula,parser::{FormulaParser,ParserOption::ParserOption},ptg::ExpPtg::ExpPtg},calcchain::FormulaHelper::FormulaHelper}}};
use kfc::cn::wps::memor4j::KmoAtomStore::KmoAtomStore;
fn range(f:Option<Rc<ExpFml>>)->Value{f.map_or(Value::Null,|f|json!([f.firstRow,f.lastRow,f.firstCol,f.lastCol]))}
fn record(s:&KmoSheet)->Value{
    let store=s.getCellValueStore();let mut cells=vec![];
    for row in 110..=112{let cell=s.getCellData(row,45);let mut f=KmoFormulaData::KmoFormulaData();store.borrow().formulaValue(cell.id,&mut f);
        let bytes:Vec<_>=cell.write(&mut [0;7]).to_vec();let tokens=store.borrow().formulaTokens(f.getTokensId());
        cells.push(json!({"bytes":bytes.iter().map(|b|format!("{b:02x}")).collect::<String>(),"tokens":tokens.iter().map(|b|format!("{b:02x}")).collect::<String>(),"plain":f.getTokensPlainSize(),"flags":f.getCalcFlags(),"bits":format!("{:016x}",s.getCellNumericValue(row,45).to_bits())}));
    }
    json!({"shared":range(s.getExpFmlManager().borrow().query(110,45,true)),"cells":cells})
}
#[test]
fn original_android_unlink_shared_formula_and_snapshot(){
    let root=std::path::Path::new(env!("CARGO_MANIFEST_DIR"));let p=root.join("rewrite-review/milestones/m4-edit-save/evidence/raw-edit-r0b26");
    let expected:Value=serde_json::from_slice(&std::fs::read(p.join("java.json")).unwrap()).unwrap();
    let api=KmoBookApi::KmoBookApi();assert_eq!(api.open(root.join("rewrite-review/milestones/m1-xlsx-grid-read/fixtures/basic-types.xlsx").to_str().unwrap(),None,true),1);
    std::thread::sleep(std::time::Duration::from_millis(1000));
    let book=api.book();let sheet=book.sheetAt(0);let s=sheet.borrow();let mgr=s.getExpFmlManager().borrow();book.transaction().reset();
    let ptgs=FormulaParser::parse("AS111+1",&EvaluationWorkbook::EvaluationWorkbook(&book),0,0,book.getSpreadsheetVersion(),&ParserOption::createOOXMLParserOption()).unwrap();
    let ptgs=FormulaHelper::replaceWithNPtgs(&ptgs,110,45,book.getSpreadsheetVersion());
    let shared=mgr.registerSharedFml(110,112,45,45,Formula::create(&ptgs,book.getSpreadsheetVersion()));
    let exp=Formula::create(&[ss_core::cn::wps::et::ss::formula::ptg::Ptg::exp_ptg(110,45,book.getSpreadsheetVersion())],book.getSpreadsheetVersion());let store=s.getCellValueStore();
    for row in 110..=112{s.setCellNumericValue(row,44,(row-109)as f64);s.setCellNumericValue(row,45,-1.0);let mut cell=s.getCellData(row,45);let mut data=KmoFormulaData::KmoFormulaData();
        data.setTokens(store.borrow_mut().newFormulaTokens(Some(&exp.getByteEncoding().unwrap().borrow())),exp.getEncodedTokenSize());data.setCacheValueId(cell.id);cell.id=store.borrow_mut().newFormulaValue(&data);cell.formulaType=3;s.setCellData(row,45,&cell);shared.addShareCell__2(row,45);
    }
    let cc=book.getCalculationChain().unwrap();cc.registerFormula__area(0,110,112,45,45);cc.markRecalculation__area(0,110,112,44,45,true);cc.synCalculate();let mut out=vec![record(&s)];
    book.transaction().start();mgr.unlinkSharedFormula(shared);book.transaction().commit();cc.synCalculate();out.push(record(&s));
    KmoAtomStore::sharedStore().as_mut().unwrap().undo();out.push(record(&s));KmoAtomStore::sharedStore().as_mut().unwrap().redo();out.push(record(&s));
    let actual=Value::Array(out);std::fs::write(p.join("rust-unlink-shared.json"),serde_json::to_vec_pretty(&actual).unwrap()).unwrap();assert_eq!(actual,expected["unlinkShared"]);
}
