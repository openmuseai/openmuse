use ss_core::cn::wps::et::ss::util::primeval::objmap::IntObjectHashMap::IntObjectHashMap;

#[test]
fn int_object_hash_map_keeps_java_bucket_chain_order_across_resize() {
    let mut map = IntObjectHashMap::IntObjectHashMap();
    // Java capacity 4, threshold 3. Keys 0/4 collide in the first table.
    map.put(0, "zero");
    map.put(4, "four");
    map.put(1, "one");
    assert_eq!(map.iterator().collect::<Vec<_>>(), [(4, &"four"), (0, &"zero"), (1, &"one")]);
    // Fourth insertion triggers doubleCapacity before adding the new entry.
    map.put(8, "eight");
    assert_eq!(map.size(), 4);
    assert_eq!(map.get(4), Some(&"four"));
    assert_eq!(map.put(4, "new four"), Some("four"));
    assert_eq!(map.remove(0), Some("zero"));
    assert!(!map.containsKey(0));
}
