use kfc::cn::wps::memor4j::KmoAtomStore::{KmoAtomStore, KmoAtomStoreTrait};
use ss_core::cn::wps::et::ss::dependent::refer::base::{AreaRefMap::AreaRefMap, CellRefMap::CellRefMap, NameRefMap::NameRefMap};
use ss_core::cn::wps::et::ss::util::Area::Area;
use ss_core::org::apache::poi::ss::SpreadsheetVersion::SpreadsheetVersion;

fn store<R>(operation: impl FnOnce(&mut dyn KmoAtomStoreTrait) -> R) -> R {
    let mut store = KmoAtomStore::sharedStore();
    operation(store.as_mut().unwrap().as_mut())
}

fn snapshot(stage: &str, cells: &CellRefMap<String>, areas: &AreaRefMap<String>, names: &NameRefMap<String>) -> serde_json::Value {
    let (mut point, mut area, mut name) = (Vec::new(), Vec::new(), Vec::new());
    cells.queryReferred(7, 2, 3, &mut point);
    areas.queryAreaReferred(7, 2, 3, &mut area);
    names.queryReferred(5, &mut name);
    let list = |values: Vec<String>| format!("[{}]", values.join(", "));
    serde_json::json!({"stage": stage, "point": list(point), "area": list(area), "name": list(name)})
}

#[test]
fn original_java_reference_map_transaction_stages_match_live_rust_objects() {
    KmoAtomStore::enable();
    KmoAtomStore::clearUp();
    let sv = SpreadsheetVersion::EXCEL97;
    let (mut cells, mut areas, mut names) = (CellRefMap::CellRefMap(sv), AreaRefMap::AreaRefMap(sv), NameRefMap::NameRefMap(sv));
    let (rect, line) = (Area::Area(1, 4, 1, 4), Area::Area(2, 2, 1, 4));
    let mut actual = Vec::new();
    store(|s| s.start());
    cells.addToMap(7, 2, 3, "point".to_string(), true);
    areas.addToMap(7, &rect, "rect".to_string(), true);
    areas.addToMap(7, &line, "line".to_string(), true);
    names.addToMap(5, "first".to_string(), true);
    names.addToMap(5, "second".to_string(), true);
    assert!(store(|s| s.commit()));
    actual.push(snapshot("add_commit", &cells, &areas, &names));
    store(|s| s.undo()); actual.push(snapshot("add_undo", &cells, &areas, &names));
    store(|s| s.redo()); actual.push(snapshot("add_redo", &cells, &areas, &names));
    store(|s| s.start());
    cells.removeFromMap(7, 2, 3, &"point".to_string());
    areas.removeFromMap(7, &rect, &"rect".to_string());
    names.removeFromMap(5, &"first".to_string());
    assert!(store(|s| s.commit()));
    actual.push(snapshot("delete_commit", &cells, &areas, &names));
    store(|s| s.undo()); actual.push(snapshot("delete_undo", &cells, &areas, &names));
    store(|s| s.redo()); actual.push(snapshot("delete_redo", &cells, &areas, &names));
    store(|s| s.start());
    cells.addToMap(7, 2, 3, "point".to_string(), true);
    cells.addToMap(7, 2, 3, "point".to_string(), true);
    areas.addToMap(7, &rect, "rect".to_string(), true);
    names.addToMap(5, "first".to_string(), true);
    actual.push(snapshot("before_discard", &cells, &areas, &names));
    store(|s| s.discard()); actual.push(snapshot("after_discard", &cells, &areas, &names));
    let expected: Vec<serde_json::Value> = include_str!("../rewrite-review/milestones/m1-xlsx-grid-read/evidence/java-reference-map-snapshots-r0b20.jsonl")
        .lines().map(|line| serde_json::from_str(line).unwrap()).collect();
    assert_eq!(actual, expected);
    KmoAtomStore::clearUp();
}
