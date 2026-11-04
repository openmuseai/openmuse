use ss_core::{java_runtime::Rc,cn::wps::{moss::{app::KmoBookApi::KmoBookApi,engine::grid::ExpFmlManager::{ExpFmlManager,ExpFml},util::KmoRect::KmoRect},et::ss::formula::Formula::Formula}};
use kfc::cn::wps::memor4j::KmoAtomStore::KmoAtomStore;
use serde_json::{json,Value};
fn range(f:Option<Rc<ExpFml>>)->Value{f.map_or(Value::Null,|f|json!([f.firstRow,f.lastRow,f.firstCol,f.lastCol]))}
#[test]
fn original_android_formula_ranges_cache_and_snapshots(){
    let root=std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let p=root.join("rewrite-review/milestones/m4-edit-save/evidence/raw-edit-r0b26");
    let expected:Value=serde_json::from_slice(&std::fs::read(p.join("java.json")).unwrap()).unwrap();
    let api=KmoBookApi::KmoBookApi();assert_eq!(api.open(root.join("rewrite-review/milestones/m1-xlsx-grid-read/fixtures/basic-types.xlsx").to_str().unwrap(),None,true),1);
    let book=api.book();let sheet=book.sheetAt(0);let sheet=sheet.borrow();let mgr=sheet.getExpFmlManager().borrow();
    book.transaction().reset();let f=Formula::create(&[],book.getSpreadsheetVersion());let array=mgr.registerArrayFml(80,82,40,41,f.copy());
    mgr.registerArrayFml(85,85,43,43,f.copy());let shared=mgr.registerSharedFml(90,92,45,46,f.copy());shared.addShareCell__2(91,46);
    let mut out=vec![];
    for row in 78..=94{for col in 38..=48{let rect=KmoRect::KmoRect__bounds(row,col,row+1,col+1);out.push(json!({"part":mgr.isPartOfArrFml(row,col),"intersect":mgr.isIntersectWithArrFml(&rect),"contains":mgr.isContainsAllIntersectArrFml(&rect),"multi":mgr.isMultiArrayFml(row,col)}));}}
    ExpFmlManager::enableCache();out.push(range(mgr.query(90,45,true)));out.push(range(mgr.query(90,45,false)));ExpFmlManager::disableCache();
    out.push(range(mgr.query(91,46,true)));out.push(range(mgr.query(91,45,true)));
    book.transaction().start();mgr.registerArrayFml(100,102,40,42,f.copy());mgr.unregister__fml(array,false);book.transaction().commit();
    out.push(range(mgr.query(100,40,false)));out.push(range(mgr.query(80,40,false)));
    KmoAtomStore::sharedStore().as_mut().unwrap().undo();out.push(range(mgr.query(100,40,false)));out.push(range(mgr.query(80,40,false)));
    KmoAtomStore::sharedStore().as_mut().unwrap().redo();out.push(range(mgr.query(100,40,false)));out.push(range(mgr.query(80,40,false)));
    let actual=Value::Array(out);std::fs::write(p.join("rust-exp-ranges.json"),serde_json::to_vec_pretty(&actual).unwrap()).unwrap();assert_eq!(actual,expected["expRanges"]);
}
