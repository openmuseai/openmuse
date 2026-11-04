use ss_core::cn::wps::et::ss::dependent::refer::base::AreaRefMap::AreaRefMap;
use ss_core::cn::wps::et::ss::dependent::refer::base::CellRefMap::CellRefMap;
use ss_core::cn::wps::et::ss::util::Area::Area;
use ss_core::org::apache::poi::ss::SpreadsheetVersion::SpreadsheetVersion;

#[test]
fn cell_ref_map_preserves_sheet_identity_duplicates_and_java_query_order() {
    let mut map = CellRefMap::CellRefMap(SpreadsheetVersion::EXCEL2007);
    map.addToMap(7, 5, 3, "first", false);
    map.addToMap(7, 5, 3, "duplicate", false);
    map.addToMap(8, 5, 3, "other sheet", false);
    let mut point = Vec::new();
    map.queryReferred(7, 5, 3, &mut point);
    assert_eq!(point, ["first", "duplicate"]);
    let mut area = Vec::new();
    map.queryReferred__area(8, &Area::Area(0, 10, 0, 10), &mut area);
    assert_eq!(area, ["other sheet"]);
    assert_eq!(map.removeFromMap(7, 5, 3, &"first"), Some("first"));
    assert_eq!(map.removeFromMap(7, 5, 3, &"missing"), None);
}

#[test]
fn area_ref_map_routes_single_row_then_column_then_rectangle() {
    let mut map = AreaRefMap::AreaRefMap(SpreadsheetVersion::EXCEL2007);
    let horizontal = Area::Area(5, 5, 1, 8);
    let vertical = Area::Area(1, 8, 3, 3);
    let rectangle = Area::Area(2, 7, 2, 6);
    map.addToMap(7, &horizontal, "horizontal", false);
    map.addToMap(7, &vertical, "vertical", false);
    map.addToMap(7, &rectangle, "rectangle", false);
    map.addToMap(8, &rectangle, "other sheet", false);
    let mut point = Vec::new();
    map.queryAreaReferred(7, 5, 3, &mut point);
    assert_eq!(point, ["horizontal", "vertical", "rectangle"]);
    let mut area = Vec::new();
    map.queryAreaReferred__area(8, &Area::Area(5, 5, 3, 3), &mut area);
    assert_eq!(area, ["other sheet"]);
    assert_eq!(map.removeFromMap(7, &vertical, &"vertical"), Some("vertical"));
    let mut remaining = Vec::new();
    map.queryAreaReferred(7, 5, 3, &mut remaining);
    assert_eq!(remaining, ["horizontal", "rectangle"]);
}

#[test]
fn cell_ref_map_reverse_op_follows_java_undo_redo_order() {
    let mut map = CellRefMap::CellRefMap(SpreadsheetVersion::EXCEL2007);
    map.addToMap(4, 6, 8, "formula", true);
    let mut hits = Vec::new();
    map.queryReferred(4, 6, 8, &mut hits);
    assert_eq!(hits, ["formula"]);
    // Java AtomSnapshot.swap(true): willSwap before rawData exchange.
    map.willSwap(true);
    hits.clear();
    map.queryReferred(4, 6, 8, &mut hits);
    assert!(hits.is_empty());
    // Java AtomSnapshot.swap(false): didFinishSwapping after rawData exchange.
    // This narrow check exercises the recorded Backup; shared KmoAtom swap wiring
    // is tracked separately by the milestone.
    map.didFinishSwapping(false);
    hits.clear();
    map.queryReferred(4, 6, 8, &mut hits);
    assert_eq!(hits, ["formula"]);
    assert_eq!(map.removeFromMap(4, 6, 8, &"formula"), Some("formula"));
    map.willSwap(true);
    hits.clear();
    map.queryReferred(4, 6, 8, &mut hits);
    assert_eq!(hits, ["formula"]);
}

#[test]
fn area_ref_map_reverse_op_handles_three_geometry_paths() {
    let geometries = [
        Area::Area(5, 5, 1, 8),
        Area::Area(1, 8, 3, 3),
        Area::Area(2, 7, 2, 6),
    ];
    for area in geometries {
        let mut map = AreaRefMap::AreaRefMap(SpreadsheetVersion::EXCEL2007);
        map.addToMap(7, &area, "formula", true);
        let mut hits = Vec::new();
        map.queryAreaReferred(7, 5, 3, &mut hits);
        assert_eq!(hits, ["formula"]);
        map.willSwap(true);
        hits.clear();
        map.queryAreaReferred(7, 5, 3, &mut hits);
        assert!(hits.is_empty());
        map.didFinishSwapping(false);
        hits.clear();
        map.queryAreaReferred(7, 5, 3, &mut hits);
        assert_eq!(hits, ["formula"]);
        assert_eq!(map.removeFromMap(7, &area, &"formula"), Some("formula"));
        map.willSwap(true);
        hits.clear();
        map.queryAreaReferred(7, 5, 3, &mut hits);
        assert_eq!(hits, ["formula"]);
    }
}
