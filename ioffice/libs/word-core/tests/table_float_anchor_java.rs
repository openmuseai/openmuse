use util::moffice::property::MutablePropertySet::MutablePropertySet;
use word_core::cn::wps::moffice::writer::data::value::PositionCode::PositionCode;
use word_core::cn::wps::moffice::writer::io::reader::docxReader::importer::helpers::ConvertHelper::Attributes;
use word_core::cn::wps::moffice::writer::io::reader::docxReader::importer::helpers::TablePropertyBaseConverterSimple::TablePropertyBaseConverterSimple;

struct TblpPr;
impl Attributes for TblpPr {
    fn getLength(&self) -> i32 { 5 }
    fn getValue(&self, _uri: &str, local: &str) -> Option<String> {
        match local {
            "horzAnchor" => Some("margin".into()),
            "vertAnchor" => Some("text".into()),
            "tblpY" => Some("-45".into()),
            "leftFromText" | "rightFromText" => Some("180".into()),
            _ => None,
        }
    }
}

#[test]
fn floating_table_import_retains_java_anchor_position_code() {
    let mut props = MutablePropertySet::new();
    TablePropertyBaseConverterSimple::default().setFloatTablePosition(&mut props, &TblpPr);
    let frozen = props.freeze();
    let position = frozen.getValue(312).unwrap().as_ref().as_any()
        .downcast_ref::<PositionCode>().unwrap();
    assert_eq!((position.getVertPos(), position.getHorzPos()), (2, 1));
    assert_eq!(frozen.getInt__int_int(314, 0), -45);
}
