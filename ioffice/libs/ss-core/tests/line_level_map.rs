use ss_core::cn::wps::et::ss::util::Area::Area;
use ss_core::cn::wps::et::ss::util::indexmap::SparseIndexMap::SparseIndexMap;
use ss_core::cn::wps::et::ss::util::level::LineLevelMap::LineLevelMap;
use ss_core::org::apache::poi::ss::SpreadsheetVersion::SpreadsheetVersion;

#[test]
fn sparse_index_map_keeps_java_sorted_sparse_iteration_and_boundaries() {
    let mut map = SparseIndexMap::SparseIndexMap(65536);
    assert_eq!(map.getFirstIndex(), -1);
    assert_eq!(map.put(200, "b"), None);
    assert_eq!(map.put(1, "a"), None);
    assert_eq!(map.put(200, "c"), Some("b"));
    assert_eq!(map.iterator().collect::<Vec<_>>(), [(1, &"a"), (200, &"c")]);
    assert_eq!(map.iteratorRange(2, 300).collect::<Vec<_>>(), [(200, &"c")]);
    assert_eq!(map.remove(1), Some("a"));
    // java firstIndex/lastIndex do not shrink after remove.
    assert_eq!((map.getFirstIndex(), map.getLastIndex()), (1, 200));
}

#[test]
fn line_level_map_queries_horizontal_before_vertical_with_duplicates() {
    for version in [SpreadsheetVersion::EXCEL97, SpreadsheetVersion::EXCEL2007] {
        let mut map = LineLevelMap::LineLevelMap(version);
        map.insertHorizontalLine(4, 0, 10, "h4");
        map.insertVerticalLine(0, 10, 3, "v3");
        map.insertHorizontalLine(4, 2, 6, "h4b");
        map.insertHorizontalLine(4, 0, 10, "h4");
        map.insertHorizontalLine(1, 1, 2, "h1");
        let mut point = Vec::new();
        map.queryReferred(4, 3, &mut point);
        let h4_order = if version == SpreadsheetVersion::EXCEL97 { ["h4", "h4", "h4b"] } else { ["h4", "h4b", "h4"] };
        assert_eq!(point, [h4_order[0], h4_order[1], h4_order[2], "v3"]);
        let mut area = Vec::new();
        map.queryReferred__area(&Area::Area(1, 4, 2, 3), &mut area);
        assert_eq!(area, ["h1", h4_order[0], h4_order[1], h4_order[2], "v3"]);
        let mut all = Vec::new();
        map.queryAll(&mut all);
        assert_eq!(all, area);
        assert_eq!(map.removeHorizontalLine(4, 0, 10, &"h4"), Some("h4"));
        assert_eq!(map.removeVerticalLine(0, 10, 3, &"v3"), Some("v3"));
        let mut remaining = Vec::new();
        map.queryReferred(4, 3, &mut remaining);
        let remaining_expected = if version == SpreadsheetVersion::EXCEL97 { ["h4", "h4b"] } else { ["h4b", "h4"] };
        assert_eq!(remaining, remaining_expected);
    }
}
