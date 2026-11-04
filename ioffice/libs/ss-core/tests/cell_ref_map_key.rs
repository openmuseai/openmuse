use ss_core::cn::wps::et::ss::dependent::refer::base::CellRefMap::CellRefMap;

#[test]
fn java_cell_position_bit_layout_and_signed_casts() {
    assert_eq!(CellRefMap::cellPos2Long(1, 2, 3), 0x0001_0000_0002_0003);
    assert_eq!(CellRefMap::long2CellPos(0x0001_0000_0002_0003), [1, 2, 3]);
    assert_eq!(CellRefMap::long2CellPos(CellRefMap::cellPos2Long(-1, -1, -1)), [65535, -1, 65535]);
}
