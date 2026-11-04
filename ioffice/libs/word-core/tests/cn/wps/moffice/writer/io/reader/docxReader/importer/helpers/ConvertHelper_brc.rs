// function_id=04_function_brc_shd
// cargo test --manifest-path libs/word-core/Cargo.toml --test 04_function_brc_shd
// src:  libs/word-core/src/cn/wps/moffice/writer/io/reader/docxReader/importer/helpers/ConvertHelper.rs
// test: libs/word-core/tests/cn/wps/moffice/writer/io/reader/docxReader/importer/helpers/ConvertHelper_brc.rs
// Same src Type as 02/03; separate [[test]] path so BRC/SHD stay independently runnable.

#[test]
#[ignore = "spec: ConvertHelper.toBRC; ImporterHelper.getLineWidthBRC null => 0.5"]
fn BS1_missing_sz_line_width_half_point() {
    unimplemented!("toBRC(empty, isXml03=false).getLineWidth() == 0.5");
}

#[test]
#[ignore = "spec: EighthPoints2Points sz=8"]
fn BS2_sz_eight_is_one_point() {
    unimplemented!("sz=8 => getLineWidth() == 1.0 (value/8), not 8.0");
}

#[test]
#[ignore = "spec: BorderConvertSimple none hash 3387192 => type 0"]
fn BS3_val_none_type_zero() {
    unimplemented!("val=none => brcType == 0 (\"none\".hashCode()==3387192)");
}

#[test]
#[ignore = "spec: BorderConvertSimple single hash -902265784 => type 1"]
fn BS4_val_single_type_one() {
    unimplemented!("val=single => brcType == 1 (\"single\".hashCode()==-902265784)");
}

#[test]
#[ignore = "spec: getBorderType null => DEFAULT_BORDER_TYPE 0"]
fn BS5_missing_val_type_zero() {
    unimplemented!("no val attr => brcType == 0");
}

#[test]
#[ignore = "spec: toBRC missing color uses toColor default"]
fn BS6_missing_color_is_auto_black() {
    unimplemented!("no color => BRC.color == -16777216");
}

#[test]
#[ignore = "spec: ImporterHelper.getSpace null => 0.0"]
fn BS7_missing_space_is_zero() {
    unimplemented!("no space => dptSpace == 0.0");
}

#[test]
#[ignore = "spec: getBorderShadow/getBorderFrame null => false"]
fn BS8_missing_shadow_and_frame_are_false() {
    unimplemented!("no shadow/frame => fShadow==false && fFrame==false");
}

#[test]
#[ignore = "spec: ConvertHelper.toSHD missing attrs"]
fn BS9_shd_missing_auto_and_pat_minus_two() {
    unimplemented!("toSHD(empty): colorFore/Back == -16777216; ipat == -2 (toPat hash -1)");
}

#[test]
#[ignore = "spec: toPat solid hash 109618859 => 1"]
fn BS10_shd_val_solid_ipat_one() {
    unimplemented!("val=solid => getPattern()/ipat == 1");
}

#[test]
#[ignore = "spec: toPat clear hash 94746189 => 0"]
fn BS11_shd_val_clear_ipat_zero() {
    unimplemented!("val=clear => ipat == 0");
}

#[test]
#[ignore = "spec: toSHD fill hex"]
fn BS12_shd_fill_ff0000() {
    unimplemented!("fill=FF0000 => getColorBack() == 16711680 (must not write to colorFore)");
}
