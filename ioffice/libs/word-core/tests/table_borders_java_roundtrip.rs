use word_core::cn::wps::moffice::writer::data::value::BRC::BRC;
use word_core::cn::wps::moffice::writer::data::value::TableBorders::TableBorders;

#[test]
fn table_borders_store_and_read_each_java_side() {
    let mut borders = TableBorders::new();
    for side in 0..6 {
        assert!(borders.getBorder(side).is_none(), "an absent border must stay absent");
    }
    for side in 0..6 {
        let brc = BRC::new__float_int((side + 1) as f32 / 2.0, 1);
        borders.setBorder(side, &brc);
    }
    for side in 0..6 {
        let brc = borders.getBorder(side).expect("stored Java BRC side");
        assert_eq!(brc.getBorderType(), 1);
        assert_eq!(brc.getLineWidth(), (side + 1) as f32 / 2.0);
    }
    assert!(borders.getBorder(-1).is_none());
    assert!(borders.getBorder(6).is_none());
}

#[test]
fn table_borders_merge_uses_direct_border_over_style() {
    let style = TableBorders::new__brcarr(&vec![BRC::new__float_int(0.5, 1); 6]);
    let mut direct = TableBorders::new();
    direct.setBorder(1, &BRC::new__float_int(1.0, 1));
    let merged = TableBorders::merge(&style, &direct);
    assert_eq!(merged.getBorder(0).expect("style top").getLineWidth(), 0.5);
    assert_eq!(merged.getBorder(1).expect("direct left").getLineWidth(), 1.0);
    assert_eq!(merged.getBorder(5).expect("style inside").getLineWidth(), 0.5);
}
