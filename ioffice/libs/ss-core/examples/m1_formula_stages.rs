//! Read-only observer attached to the original reader completion callback.
use ss_core::cn::wps::moss::{
    app::{
        event::KmoBookOpenEventListener::KmoBookOpenEventListener, KmoBook::KmoBook,
        KmoBookApi::KmoBookApi,
    },
    engine::grid::KmoFormulaData::KmoFormulaData,
};
use std::sync::{Arc, Mutex, Weak};
fn snapshot(book: &KmoBook, stage: &str) -> serde_json::Value {
    let mut cells = Vec::new();
    if book.sheetCount() > 0 {
        let sheet = book.sheetAt(0);
        let sheet = sheet.borrow();
        let store = sheet.getCellValueStore();
        for col in 0..3 {
            let cell = sheet.getCellData(0, col);
            let raw_type = sheet.getCellValueType__2(0, col);
            let id = sheet.getCellValueId(0, col);
            let raw_value = match raw_type {
                1 => store.borrow_mut().doubleValue(id).to_string(),
                2 => store.borrow_mut().stringValue(id),
                5 => store.borrow().booleanValue(id).to_string(),
                6 => store.borrow().errorValue(id).to_string(),
                _ => String::new(),
            };
            let mut f = KmoFormulaData::KmoFormulaData();
            store.borrow().formulaValue(cell.id, &mut f);
            let bytes = store.borrow().formulaTokens(f.getTokensId());
            cells.push(serde_json::json!({"row":0,"col":col,"cellId":cell.id,"formulaType":cell.formulaType,"cacheId":id,"rawType":raw_type,"rawValue":raw_value,"tokensId":f.getTokensId(),"encodedLen":f.getTokensPlainSize(),"calcFlags":f.getCalcFlags(),"tokenHex":bytes.iter().map(|b|format!("{b:02x}")).collect::<String>()}));
        }
    }
    serde_json::json!({"stage":stage,"engineId":"ss-core","dirty":book.isDirty(),"readOnly":book.getReadOnly(),"cells":cells})
}
struct Observer {
    book: Weak<KmoBook>,
    stages: Arc<Mutex<Vec<serde_json::Value>>>,
}
impl KmoBookOpenEventListener for Observer {
    fn bookDidFinishReadingEverything(&self) {
        self.stages
            .lock()
            .unwrap()
            .push(snapshot(&self.book.upgrade().unwrap(), "reader_callback"));
    }
}
fn main() {
    let path = std::env::args().nth(1).expect("xlsx path");
    let api = KmoBookApi::KmoBookApi();
    let book = api.book();
    let stages = Arc::new(Mutex::new(Vec::new()));
    let listener = Arc::new(Observer {
        book: Arc::downgrade(&book),
        stages: stages.clone(),
    });
    assert_eq!(api.open__listener(&path, None, true, Some(listener)), 1);
    stages
        .lock()
        .unwrap()
        .push(snapshot(&book, "after_open_return"));
    // Native elapsed-time observation. No UI callback is invented for ui_post.
    std::thread::sleep(std::time::Duration::from_millis(1000));
    let mut late = snapshot(&book, "ui_plus_1000ms");
    late["evaluationGaps"] = serde_json::to_value(
        book.getCalculationChain()
            .unwrap()
            .calcChain
            .unportedEvaluationBranches(),
    )
    .unwrap();
    stages.lock().unwrap().push(late);
    for value in stages.lock().unwrap().iter() {
        println!("{value}");
    }
}
