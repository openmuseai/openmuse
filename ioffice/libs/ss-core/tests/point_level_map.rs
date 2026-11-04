use ss_core::cn::wps::et::ss::util::Area::Area;
use ss_core::cn::wps::et::ss::util::level::PointLevelMap::PointLevelMap;
use ss_core::org::apache::poi::ss::SpreadsheetVersion::SpreadsheetVersion;

#[test]
fn point_level_map_preserves_java_bucket_order_duplicates_and_removal() {
    for version in [SpreadsheetVersion::EXCEL97, SpreadsheetVersion::EXCEL2007] {
        let mut map = PointLevelMap::PointLevelMap(version);
        map.insert(0, 0, "first");
        map.insert(200, 3, "second");
        map.insert(0, 0, "duplicate");
        map.insert(200, 3, "second");

        let mut point = Vec::new();
        map.queryAreaRefered(0, 0, &mut point);
        assert_eq!(point, ["first", "duplicate"]);

        let mut area = Vec::new();
        map.queryAreaReferred(&Area::Area(0, 200, 0, 3), &mut area);
        assert_eq!(area, ["first", "duplicate", "second", "second"]);

        let mut all = Vec::new();
        map.queryAll(&mut all);
        assert_eq!(all, area);
        assert_eq!(map.remove(200, 3, &"second"), Some("second"));
        assert_eq!(map.remove(200, 3, &"absent"), None);
        let mut remaining = Vec::new();
        map.queryAll(&mut remaining);
        assert_eq!(remaining, ["first", "duplicate", "second"]);
    }
}
