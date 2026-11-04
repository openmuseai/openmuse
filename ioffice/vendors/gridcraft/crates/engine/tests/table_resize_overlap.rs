use gridcraft_engine::Session;
use serde_json::json;

fn table() -> Session {
    let mut s = Session::new();
    s.new_workbook();
    s.execute("range.setValues", json!({"range":"A1", "values":[["Item", "Qty"], ["x", "7"]]})).unwrap();
    s.execute("insert.table", json!({"range":"A1:B2", "header":true, "name":"T"})).unwrap();
    s
}

fn table_range(s: &Session) -> String {
    s.doc().unwrap().wb.active().unwrap().tables[0].range.a1()
}

#[test]
fn resize_to_disjoint_range_is_rejected_and_keeps_table() {
    let mut s = table();
    assert!(s.execute("table.resize", json!({"table":"T", "range":"D1:E2"})).is_err());
    assert_eq!(table_range(&s), "A1:B2");
    assert_eq!(s.doc().unwrap().wb.active().unwrap().tables[0].columns[1].name, "Qty");
}

#[test]
fn resize_to_overlapping_range_still_works() {
    let mut s = table();
    s.execute("table.resize", json!({"table":"T", "range":"B1:C2"})).unwrap();
    assert_eq!(table_range(&s), "B1:C2");
}
