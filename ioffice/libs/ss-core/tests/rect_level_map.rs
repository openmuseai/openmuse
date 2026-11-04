use ss_core::cn::wps::et::ss::util::Area::Area;
use ss_core::cn::wps::et::ss::util::level::RectLevelMap::RectLevelMap;
use ss_core::org::apache::poi::ss::SpreadsheetVersion::SpreadsheetVersion;

#[test]
fn rect_level_map_preserves_java_overlap_order_duplicates_and_first_removal() {
    for version in [SpreadsheetVersion::EXCEL97, SpreadsheetVersion::EXCEL2007] {
        let mut map = RectLevelMap::RectLevelMap(version);
        map.insert(0, 20, 0, 7, "first");
        map.insert(10, 30, 4, 12, "second");
        map.insert(0, 20, 0, 7, "first");
        map.insert(200, 220, 10, 20, "distant");

        let mut point = Vec::new();
        map.queryAreaReferred(15, 5, &mut point);
        assert_eq!(point, ["first", "first", "second"]);

        let mut area = Vec::new();
        map.queryAreaReferred__area(&Area::Area(19, 200, 6, 10), &mut area);
        assert_eq!(area, ["first", "first", "second", "distant"]);

        let mut all = Vec::new();
        map.queryAll(&mut all);
        assert_eq!(all, ["first", "first", "second", "distant"]);
        assert_eq!(map.remove(0, 20, 0, 7, &"first"), Some("first"));
        assert_eq!(map.remove(0, 20, 0, 7, &"absent"), None);
        let mut remaining = Vec::new();
        map.queryAreaReferred(15, 5, &mut remaining);
        assert_eq!(remaining, ["first", "second"]);
    }
}
