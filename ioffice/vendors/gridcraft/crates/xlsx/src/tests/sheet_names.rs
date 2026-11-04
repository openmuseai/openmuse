//! Sheet names XLSX cannot hold are refused on export instead of breaking formulas on reopen.

use std::sync::Arc;

use gridcraft_model::{Sheet, Workbook};

use crate::{IoError, write_xlsx};

fn book(names: &[&str]) -> Workbook {
    let mut wb = Workbook::new();
    wb.sheets = names.iter().map(|n| Arc::new(Sheet::new(*n))).collect();
    wb
}

#[test]
fn unrepresentable_sheet_names_are_refused() {
    let long_a = format!("{}X", "A".repeat(31));
    let long_b = format!("{}Y", "A".repeat(31));
    for names in [vec!["Data", "a/b"], vec![long_a.as_str(), long_b.as_str()], vec!["Data", "data"], vec!["Data", ""]] {
        assert!(matches!(write_xlsx(&book(&names)), Err(IoError::Format(_))), "{names:?}");
    }
    assert!(write_xlsx(&book(&["Data", "Other"])).is_ok());
}
