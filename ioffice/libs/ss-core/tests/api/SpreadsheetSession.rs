#[path = "../ffi/session_abi.rs"]
mod session_abi_tests;

// Match the other workbook fixture suites: KmoApp / XLSX reader globals are shared
// across cases. Each open still uses the original concurrent reader workers.
static SESSION_SERIAL: std::sync::Mutex<()> = std::sync::Mutex::new(());

use ss_core::api::SpreadsheetSession::{
    SpreadsheetLoadConfig, SpreadsheetSession, ENGINE_ID,
};

fn fixture(name: &str) -> String {
    format!(
        "{}/rewrite-review/milestones/m1-xlsx-grid-read/fixtures/{name}",
        env!("CARGO_MANIFEST_DIR")
    )
}

fn cell<'a>(
    snap: &'a ss_core::api::SpreadsheetSession::SheetSnapshot,
    row: i32,
    col: i32,
) -> &'a ss_core::api::SpreadsheetSession::CellSnapshot {
    snap.cells
        .iter()
        .find(|cell| cell.row == row && cell.col == col)
        .unwrap_or_else(|| panic!("missing cell {row},{col}"))
}

#[test]
fn sess1_basic_types_snapshot_matches_stored_cells() {
    let _case = SESSION_SERIAL.lock().unwrap();
    let path = fixture("basic-types.xlsx");
    let session = SpreadsheetSession::open_path(&path, SpreadsheetLoadConfig::default());
    let snap = session.snapshot(0, 0, 2, 7);
    assert_eq!(snap.engine_id, ENGINE_ID);
    assert_eq!(snap.open_code, 1);
    assert_eq!(snap.sheet_count, 1);
    assert_eq!(
        snap.input_sha256,
        "c48f21bb6c390da8ce6b9a6a3e284593ed5942d2b2bf5ea49c897bc0297a20a9"
    );
    assert_eq!(snap.sheets[0].name, "Sheet1");
    assert!(!snap.sheets[0].hidden);
    let a1 = cell(&snap.sheets[0], 0, 0);
    assert_eq!(a1.raw_type, 1);
    assert_eq!(a1.raw_value, "1.5");
    assert_eq!(a1.ixf, 15);
    let b1 = cell(&snap.sheets[0], 0, 1);
    assert_eq!(b1.raw_type, 5);
    assert_eq!(b1.raw_value, "true");
    assert!(snap.sheets[0]
        .cells
        .iter()
        .all(|item| !(item.row == 0 && item.col == 3)));
    assert_eq!(cell(&snap.sheets[0], 1, 0).raw_value, "中文");
    assert!(snap.cell_count >= 6);
}

#[test]
fn sess2_sparse_sheets_keep_height_and_hidden() {
    let _case = SESSION_SERIAL.lock().unwrap();
    let session = SpreadsheetSession::open_path(
        &fixture("sparse-multisheet.xlsx"),
        SpreadsheetLoadConfig::default(),
    );
    let snap = session.snapshot(0, 0, 5, 3);
    assert_eq!(snap.sheet_count, 3);
    assert_eq!(snap.sheets[0].name, "Visible");
    assert_eq!(snap.sheets[0].row_heights[0], 600);
    assert_eq!(cell(&snap.sheets[0], 0, 0).raw_value, "1");
    assert_eq!(cell(&snap.sheets[0], 4, 2).raw_value, "9");
    assert!(snap.sheets[1].hidden);
    assert_eq!(cell(&snap.sheets[1], 0, 0).raw_value, "2");
    assert!(snap.sheets[2].cells.iter().all(|item| item.row != 0));
    assert_eq!(cell(&snap.sheets[2], 1, 1).raw_value, "3");
}

#[test]
fn sess3_formula_cache_and_async_error() {
    let _case = SESSION_SERIAL.lock().unwrap();
    let session = SpreadsheetSession::open_path(
        &fixture("formula-cache.xlsx"),
        SpreadsheetLoadConfig::default(),
    );
    let snap = session.snapshot(0, 0, 1, 3);
    assert_eq!(cell(&snap.sheets[0], 0, 0).raw_value, "2");
    assert_eq!(cell(&snap.sheets[0], 0, 1).raw_value, "ab");
    // Java open returns while recalculation is running; assert the eventual value.
    let deadline=std::time::Instant::now()+std::time::Duration::from_secs(5);
    loop {
        let late=session.snapshot(0,0,1,3);let c1=cell(&late.sheets[0],0,2);
        if c1.raw_type==6 {assert_eq!(c1.raw_value,"7");break;}
        assert!(std::time::Instant::now()<deadline,"C1 remained blank after async open");
        std::thread::sleep(std::time::Duration::from_millis(2));
    }
}

#[test]
fn sess4_missing_file_has_no_sheets() {
    let _case = SESSION_SERIAL.lock().unwrap();
    let session =
        SpreadsheetSession::open_path("/tmp/m1-no-such-session.xlsx", SpreadsheetLoadConfig::default());
    let snap = session.snapshot(0, 0, 2, 2);
    assert_eq!(snap.open_code, 2);
    assert_eq!(snap.sheet_count, 0);
    assert_eq!(snap.cell_count, 0);
    assert!(snap.sheets.is_empty());
}

#[test]
fn sess5_same_bytes_match_path_snapshot() {
    let _case = SESSION_SERIAL.lock().unwrap();
    let path = fixture("basic-types.xlsx");
    let bytes = std::fs::read(&path).unwrap();
    let from_path = SpreadsheetSession::open_path(&path, SpreadsheetLoadConfig::default());
    let from_bytes = SpreadsheetSession::open_bytes(&bytes, SpreadsheetLoadConfig::default());
    let left = from_path.snapshot(0, 0, 2, 7);
    let right = from_bytes.snapshot(0, 0, 2, 7);
    assert_eq!(left.input_sha256, right.input_sha256);
    assert_eq!(left.sheets, right.sheets);
}

#[test]
fn sess6_edit_and_export_are_unsupported() {
    let _case = SESSION_SERIAL.lock().unwrap();
    let session = SpreadsheetSession::open_path(
        &fixture("empty.xlsx"),
        SpreadsheetLoadConfig::default(),
    );
    assert!(session.execute("edit.paste").unwrap_err().contains("unsupported capability"));
    assert!(session.export("xlsx").unwrap_err().contains("unsupported capability"));
    let snap = session.snapshot(0, 0, 1, 1);
    assert_eq!(snap.open_code, 1);
    assert_eq!(snap.cell_count, 0);
}
