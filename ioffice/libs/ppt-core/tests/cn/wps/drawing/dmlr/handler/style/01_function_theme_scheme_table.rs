use ppt_core::cn::wps::drawing::dmlr::util::DGMap::DGMap;
use ppt_core::cn::wps::show::pptxr::PptxReader::parse_theme_scheme_table;

#[test]
fn t01_accent1_and_dk1() {
    let xml = r#"<a:theme xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main"><a:themeElements><a:clrScheme name="Office"><a:dk1><a:srgbClr val="000000"/></a:dk1><a:lt1><a:srgbClr val="FFFFFF"/></a:lt1><a:accent1><a:srgbClr val="4472C4"/></a:accent1></a:clrScheme></a:themeElements></a:theme>"#;
    let table = parse_theme_scheme_table(xml);
    let accent1 = *DGMap::schemeColorMap().get("accent1").unwrap();
    assert_eq!(table.get(&accent1), Some(&0xFF4472C4));
    let dk1 = *DGMap::schemeColorMap().get("dk1").unwrap();
    assert_eq!(table.get(&dk1), Some(&0xFF000000));
}

#[test]
fn t02_empty_theme_is_empty_table() {
    let table = parse_theme_scheme_table("<a:theme/>");
    assert!(table.is_empty());
}

#[test]
fn t03_sysclr_lastclr_and_tx1_alias() {
    let xml = r#"<a:theme xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main"><a:dk1><a:sysClr val="windowText" lastClr="111111"/></a:dk1></a:theme>"#;
    let table = parse_theme_scheme_table(xml);
    let dk1 = *DGMap::schemeColorMap().get("dk1").unwrap();
    assert_eq!(table.get(&dk1), Some(&0xFF111111));
    let tx1 = *DGMap::schemeColorMap().get("tx1").unwrap();
    assert_eq!(table.get(&tx1), Some(&0xFF111111));
}
