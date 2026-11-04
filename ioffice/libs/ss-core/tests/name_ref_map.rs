use ss_core::cn::wps::et::ss::dependent::refer::base::NameRefMap::NameRefMap;
use ss_core::cn::wps::et::ss::util::primeval::objmap::LongObjectHashMap::LongObjectHashMap;
use ss_core::org::apache::poi::ss::SpreadsheetVersion::SpreadsheetVersion;

#[test]
fn long_object_hash_map_preserves_java_bucket_and_chain_order() {
    let mut map = LongObjectHashMap::LongObjectHashMap();
    map.put(0, "zero");
    map.put(4, "four");
    map.put(1, "one");
    assert_eq!(map.iterator().map(|(key, _)| key).collect::<Vec<_>>(), [4, 0, 1]);
    assert_eq!(map.put(4, "new four"), Some("four"));
    assert_eq!(map.get(4), Some(&"new four"));
    assert_eq!(map.remove(0), Some("zero"));
}

#[test]
fn name_ref_map_preserves_duplicates_and_first_match_removal() {
    let mut map = NameRefMap::NameRefMap(SpreadsheetVersion::EXCEL2007);
    map.addToMap(7, "same", false);
    map.addToMap(7, "same", false);
    map.addToMap(8, "other", false);
    let mut hits = Vec::new();
    map.queryReferred(7, &mut hits);
    assert_eq!(hits, ["same", "same"]);
    assert_eq!(map.removeFromMap(7, &"same"), Some("same"));
    hits.clear();
    map.queryReferred(7, &mut hits);
    assert_eq!(hits, ["same"]);
    assert_eq!(map.removeFromMap(7, &"missing"), None);
}

#[test]
fn name_ref_map_reverse_operation_matches_java_branch_order() {
    let mut map = NameRefMap::NameRefMap(SpreadsheetVersion::EXCEL2007);
    map.addToMap(7, "formula", true);
    map.willSwap(true);
    let mut hits = Vec::new();
    map.queryReferred(7, &mut hits);
    assert!(hits.is_empty());
    map.didFinishSwapping(false);
    map.queryReferred(7, &mut hits);
    assert_eq!(hits, ["formula"]);
}

#[derive(Clone, Debug)]
struct DirectionalValue { accepts_other: bool }

impl PartialEq for DirectionalValue {
    fn eq(&self, _other: &Self) -> bool { self.accepts_other }
}

#[test]
fn name_removal_uses_java_array_list_search_argument_direction() {
    // Java ArrayList.indexOf(o) invokes o.equals(element), not element.equals(o).
    let mut map = NameRefMap::NameRefMap(SpreadsheetVersion::EXCEL2007);
    map.addToMap(9, DirectionalValue { accepts_other: false }, false);
    assert!(map.removeFromMap(9, &DirectionalValue { accepts_other: true }).is_some());
}
