// function_id=07_function_correct_prop
// cargo test --manifest-path libs/word-core/Cargo.toml --test 07_function_correct_prop
// src:  libs/word-core/src/cn/wps/moffice/writer/io/reader/docxReader/importer/helpers/ParaPropsConverterSimple.rs
// test: libs/word-core/tests/cn/wps/moffice/writer/io/reader/docxReader/importer/helpers/ParaPropsConverterSimple_correct.rs
// Same src Type as 06; separate [[test]] path so convert vs correctProp stay independently runnable.

#[test]
#[ignore = "spec: ParaPropsConverterSimple.correctProp / correntStyle"]
fn CP1_empty_dst_gets_191_zero() {
    unimplemented!("correctProp(new MutablePropertySet) => contains(191) && getInt(191)==0");
}

#[test]
#[ignore = "spec: correntStyle does not overwrite existing 191"]
fn CP2_existing_191_is_kept() {
    unimplemented!("setInt(191, 5) then correctProp => getInt(191)==5 (must not force Normal)");
}

#[test]
#[ignore = "spec: correctProp null dst"]
fn CP3_null_dst_does_not_panic() {
    unimplemented!("Java if (null != dst); Rust Option: correctProp(None) returns");
}

#[test]
#[ignore = "spec: correctTabs no-op without 280/281"]
fn CP4_no_raw_tabs_does_not_invent_236() {
    unimplemented!("after CP1, !contains(236) — empty para must not gain TxTabStops");
}

#[test]
#[ignore = "spec: correctProp idempotent when 191 already 0"]
fn CP5_second_correct_keeps_zero() {
    unimplemented!("setInt(191, 0) then correctProp still getInt(191)==0");
}

#[test]
#[ignore = "spec: correctTabs synthesizes 236 when 280 and 281 are present"]
fn CP6_tabs_280_281_write_236_and_clear_raw() {
    unimplemented!(
        "setValue(280) and setValue(281) non-null then correctProp => contains(236); 280 and 281 become null"
    );
}
