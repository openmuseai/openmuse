use ss_core::api::GridDiff::{evidence_json, first_diff, require_java_dump};
use ss_core::api::SpreadsheetSession::{SpreadsheetLoadConfig, SpreadsheetSession};

fn fixture(name: &str) -> String {
    format!(
        "{}/rewrite-review/milestones/m1-xlsx-grid-read/fixtures/{name}",
        env!("CARGO_MANIFEST_DIR")
    )
}

#[test]
fn diff1_same_book_has_no_difference() {
    let path = fixture("basic-types.xlsx");
    let left = SpreadsheetSession::open_path(&path, SpreadsheetLoadConfig::default()).snapshot(0, 0, 2, 7);
    let right = SpreadsheetSession::open_path(&path, SpreadsheetLoadConfig::default()).snapshot(0, 0, 2, 7);
    assert!(first_diff(&left, &right).is_none());
}

#[test]
fn diff2_first_difference_is_the_cell_value() {
    let path = fixture("basic-types.xlsx");
    let left = SpreadsheetSession::open_path(&path, SpreadsheetLoadConfig::default()).snapshot(0, 0, 2, 7);
    let mut right = left.clone();
    right.sheets[0].cells[0].raw_value = "9".to_string();
    let diff = first_diff(&left, &right).unwrap();
    assert_eq!(diff.path, "sheets[0].r0c0.raw_value");
    assert_eq!(diff.left, "1.5");
    assert_eq!(diff.right, "9");
}

#[test]
fn diff3_sha_is_checked_before_cells() {
    let path = fixture("basic-types.xlsx");
    let left = SpreadsheetSession::open_path(&path, SpreadsheetLoadConfig::default()).snapshot(0, 0, 1, 1);
    let mut right = left.clone();
    right.input_sha256 = "different".to_string();
    right.sheets[0].cells[0].raw_value = "9".to_string();
    let diff = first_diff(&left, &right).unwrap();
    assert_eq!(diff.path, "input_sha256");
}

#[test]
fn diff5_row_height_is_before_cells() {
    let path = fixture("sparse-multisheet.xlsx");
    let left =
        SpreadsheetSession::open_path(&path, SpreadsheetLoadConfig::default()).snapshot(0, 0, 5, 3);
    let mut right = left.clone();
    right.sheets[0].row_heights[0] = 1;
    right.sheets[0].cells[0].raw_value = "9".to_string();
    let diff = first_diff(&left, &right).unwrap();
    assert_eq!(diff.path, "sheets[0].row_heights[0]");
    assert_eq!(diff.left, "600");
}

#[test]
fn diff6_ixf_follows_the_raw_value() {
    let path = fixture("basic-types.xlsx");
    let left =
        SpreadsheetSession::open_path(&path, SpreadsheetLoadConfig::default()).snapshot(0, 0, 1, 1);
    let mut right = left.clone();
    right.sheets[0].cells[0].ixf = 0;
    let diff = first_diff(&left, &right).unwrap();
    assert_eq!(diff.path, "sheets[0].r0c0.ixf");
    assert_eq!(diff.left, "15");
}

#[test]
fn diff7_rust_dump_records_stored_cells_and_java_file_is_present() {
    let path = fixture("basic-types.xlsx");
    let snap =
        SpreadsheetSession::open_path(&path, SpreadsheetLoadConfig::default()).snapshot(0, 0, 2, 7);
    let dump = evidence_json(&snap);
    assert_eq!(
        dump["input_sha256"],
        "c48f21bb6c390da8ce6b9a6a3e284593ed5942d2b2bf5ea49c897bc0297a20a9"
    );
    assert_eq!(dump["engine_version"], "ss-core");
    assert_eq!(dump["sheets"][0]["name"], "Sheet1");
    let a1 = dump["sheets"][0]["rows"][0]["cells"]
        .as_array()
        .unwrap()
        .iter()
        .find(|cell| cell["col"] == 0)
        .unwrap();
    assert_eq!(a1["raw_value"], "1.5");
    assert_eq!(a1["raw_type"], 1);
    assert_eq!(a1["xf"], 15);
    let dir = format!(
        "{}/rewrite-review/milestones/m1-xlsx-grid-read/evidence/rust",
        env!("CARGO_MANIFEST_DIR")
    );
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(
        format!("{dir}/basic-types.json"),
        serde_json::to_string_pretty(&dump).unwrap(),
    )
    .unwrap();
    let java = format!(
        "{}/rewrite-review/milestones/m1-xlsx-grid-read/evidence/java/basic-types.json",
        env!("CARGO_MANIFEST_DIR")
    );
    require_java_dump(&java).unwrap();
    let java_dump: serde_json::Value = serde_json::from_slice(&std::fs::read(java).unwrap()).unwrap();
    assert_eq!(java_dump["inputSha256"], dump["input_sha256"]);
}

#[test]
fn diff4_missing_java_dump_is_not_a_match() {
    let missing = format!(
        "{}/rewrite-review/milestones/m1-xlsx-grid-read/evidence/java-basic-types.json",
        env!("CARGO_MANIFEST_DIR")
    );
    let err = require_java_dump(&missing).unwrap_err();
    assert!(err.contains("java dump absent"));
}
