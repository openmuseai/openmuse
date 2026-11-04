// function_id=03_function_measure_color
// cargo test --manifest-path libs/word-core/Cargo.toml --test 03_function_measure_color
// src:  libs/word-core/src/cn/wps/moffice/writer/io/reader/docxReader/importer/helpers/ConvertHelper.rs
// test: libs/word-core/tests/cn/wps/moffice/writer/io/reader/docxReader/importer/helpers/ConvertHelper_measure.rs
// Same src Type as 02; separate [[test]] path so attr vs measure stay independently runnable.

#[test]
#[ignore = "spec: ConvertHelper.getLong missing"]
fn MC1_get_long_missing_is_null() {
    unimplemented!("getLong(attrs, \"left\") absent => None");
}

#[test]
#[ignore = "spec: ConvertHelper.getLong raw magnitude"]
fn MC2_get_long_unitless_1440() {
    unimplemented!("getLong left=\"1440\" => Some(1440); must not *20");
}

#[test]
#[ignore = "spec: ConvertHelper.getLong pt -> Points2Twips"]
fn MC3_get_long_72pt_is_1440_twips() {
    unimplemented!("getLong left=\"72pt\" => Some(1440) via WmlUnit PT + MetricUtility.Points2Twips");
}

#[test]
#[ignore = "spec: ConvertHelper.getLong 12pt"]
fn MC4_get_long_12pt_is_240_twips() {
    unimplemented!("getLong left=\"12pt\" => Some(240)");
}

#[test]
#[ignore = "spec: ConvertHelper.getFloat 12pt"]
fn MC5_get_float_12pt_is_240() {
    unimplemented!("getFloat(attrs, \"sz\") with 12pt => Some(240.0) (Points2Twips, not 12.0)");
}

#[test]
#[ignore = "spec: ConvertHelper.getFloat missing"]
fn MC6_get_float_missing_is_null() {
    unimplemented!("getFloat missing name => None");
}

#[test]
#[ignore = "spec: ConvertHelper.getInt 72pt"]
fn MC7_get_int_72pt_is_1440() {
    unimplemented!("getInt left=\"72pt\" => Some(1440)");
}

#[test]
#[ignore = "spec: ConvertHelper.toColor missing"]
fn MC8_to_color_missing_is_auto_black() {
    unimplemented!("toColor(attrs, \"color\") missing => -16777216 (not private convert2coreColor default -2)");
}

#[test]
#[ignore = "spec: ConvertHelper.toColor auto"]
fn MC9_to_color_auto() {
    unimplemented!("color=auto => -16777216");
}

#[test]
#[ignore = "spec: ConvertHelper.toColor hex"]
fn MC10_to_color_hex_ff0000() {
    unimplemented!("color=FF0000 => 16711680 (TypeConversion.hexString2Integer)");
}

#[test]
#[ignore = "spec: ConvertHelper.toColor invalid hex"]
fn MC11_to_color_invalid_is_minus_two() {
    unimplemented!("color=xyz => -2 (convert2coreColor dstVal default when hex parse fails)");
}

#[test]
#[ignore = "spec: ConvertHelper.hex2Int missing"]
fn MC12_hex2int_missing_is_null() {
    unimplemented!("hex2Int(attrs, \"color\") absent => None, not -16777216");
}
