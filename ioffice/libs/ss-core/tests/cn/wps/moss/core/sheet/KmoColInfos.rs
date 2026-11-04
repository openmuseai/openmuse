use ss_core::cn::wps::moss::core::sheet::KmoColInfo::KmoColInfo;
use ss_core::cn::wps::moss::core::sheet::KmoColInfos::KmoColInfos;

fn width_at(infos: &KmoColInfos, col: i32) -> Option<(i32, i16, i16)> {
    infos.getColInfo(col).map(|info| {
        (
            info.getWidth(),
            info.getMinColIndex(),
            info.getMaxColIndex(),
        )
    })
}

#[test]
fn col1_lookup_uses_last_range_then_earlier_ranges() {
    let mut infos = KmoColInfos::KmoColInfos();
    infos.initFillColInfo(0, 2, 100, 15, 6);
    infos.initFillColInfo(5, 7, 200, 16, 6);
    assert_eq!(width_at(&infos, 1), Some((100, 0, 2)));
    assert_eq!(width_at(&infos, 6), Some((200, 5, 7)));
    assert_eq!(width_at(&infos, 4), None);
    assert_eq!(infos.getFirstCol(), 0);
    assert_eq!(infos.getLastCol(), 7);
}

#[test]
fn col2_split_keeps_width_on_each_side() {
    let mut infos = KmoColInfos::KmoColInfos();
    infos.initFillColInfo(0, 10, 180, 15, 6);
    let mid = infos.fitAndGetColInfo(5).unwrap();
    assert_eq!(mid.getMinColIndex(), 5);
    assert_eq!(mid.getMaxColIndex(), 5);
    assert_eq!(mid.getWidth(), 180);
    assert_eq!(width_at(&infos, 0), Some((180, 0, 4)));
    assert_eq!(width_at(&infos, 4), Some((180, 0, 4)));
    assert_eq!(width_at(&infos, 6), Some((180, 6, 10)));
    assert_eq!(width_at(&infos, 10), Some((180, 6, 10)));
}

#[test]
fn col3_split_at_ends_and_reversed_range_is_not_a_hit() {
    let mut infos = KmoColInfos::KmoColInfos();
    infos.initFillColInfo(2, 4, 90, 15, 6);
    infos.fitAndGetColInfo(2);
    assert_eq!(width_at(&infos, 2), Some((90, 2, 2)));
    assert_eq!(width_at(&infos, 3), Some((90, 3, 4)));
    let mut reversed = KmoColInfos::KmoColInfos();
    reversed.setColInfo(KmoColInfo::KmoColInfo(8, 3, 40, 15, 6));
    assert_eq!(width_at(&reversed, 5), None);
    assert_eq!(reversed.getColInfo(8).unwrap().getColSpan(), -4);
}
