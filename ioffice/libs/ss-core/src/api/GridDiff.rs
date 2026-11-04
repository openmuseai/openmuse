use serde_json::{json, Value};

use super::SpreadsheetSession::WorkbookSnapshot;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GridDiff {
    pub path: String,
    pub left: String,
    pub right: String,
}

pub fn first_diff(left: &WorkbookSnapshot, right: &WorkbookSnapshot) -> Option<GridDiff> {
    if let Some(diff) = cmp("input_sha256", &left.input_sha256, &right.input_sha256) {
        return Some(diff);
    }
    if let Some(diff) = cmp("open_code", &left.open_code.to_string(), &right.open_code.to_string())
    {
        return Some(diff);
    }
    if let Some(diff) = cmp(
        "sheet_count",
        &left.sheet_count.to_string(),
        &right.sheet_count.to_string(),
    ) {
        return Some(diff);
    }
    let sheets = left.sheets.len().max(right.sheets.len());
    for index in 0..sheets {
        let lsheet = left.sheets.get(index);
        let rsheet = right.sheets.get(index);
        let base = format!("sheets[{index}]");
        match (lsheet, rsheet) {
            (None, Some(_)) => {
                return Some(GridDiff {
                    path: format!("{base}.missing"),
                    left: "absent".to_string(),
                    right: "present".to_string(),
                });
            }
            (Some(_), None) => {
                return Some(GridDiff {
                    path: format!("{base}.missing"),
                    left: "present".to_string(),
                    right: "absent".to_string(),
                });
            }
            (None, None) => {}
            (Some(lsheet), Some(rsheet)) => {
                if let Some(diff) = cmp(&format!("{base}.name"), &lsheet.name, &rsheet.name) {
                    return Some(diff);
                }
                if let Some(diff) = cmp(
                    &format!("{base}.hidden"),
                    &lsheet.hidden.to_string(),
                    &rsheet.hidden.to_string(),
                ) {
                    return Some(diff);
                }
                let rows = lsheet.row_heights.len().max(rsheet.row_heights.len());
                for row in 0..rows {
                    let lv = lsheet.row_heights.get(row).map(|h| h.to_string());
                    let rv = rsheet.row_heights.get(row).map(|h| h.to_string());
                    if lv != rv {
                        return Some(GridDiff {
                            path: format!("{base}.row_heights[{row}]"),
                            left: lv.unwrap_or_else(|| "absent".to_string()),
                            right: rv.unwrap_or_else(|| "absent".to_string()),
                        });
                    }
                }
                let cells = lsheet.cells.len().max(rsheet.cells.len());
                for cell in 0..cells {
                    let lc = lsheet.cells.get(cell);
                    let rc = rsheet.cells.get(cell);
                    match (lc, rc) {
                        (None, Some(rc)) => {
                            return Some(GridDiff {
                                path: format!("{base}.r{}c{}.missing", rc.row, rc.col),
                                left: "absent".to_string(),
                                right: "present".to_string(),
                            });
                        }
                        (Some(lc), None) => {
                            return Some(GridDiff {
                                path: format!("{base}.r{}c{}.missing", lc.row, lc.col),
                                left: "present".to_string(),
                                right: "absent".to_string(),
                            });
                        }
                        (None, None) => {}
                        (Some(lc), Some(rc)) => {
                            let coord = format!("{base}.r{}c{}", lc.row, lc.col);
                            if lc.row != rc.row || lc.col != rc.col {
                                return Some(GridDiff {
                                    path: format!("{coord}.coord"),
                                    left: format!("r{}c{}", lc.row, lc.col),
                                    right: format!("r{}c{}", rc.row, rc.col),
                                });
                            }
                            if let Some(diff) = cmp(
                                &format!("{coord}.raw_type"),
                                &lc.raw_type.to_string(),
                                &rc.raw_type.to_string(),
                            ) {
                                return Some(diff);
                            }
                            if let Some(diff) =
                                cmp(&format!("{coord}.raw_value"), &lc.raw_value, &rc.raw_value)
                            {
                                return Some(diff);
                            }
                            if let Some(diff) =
                                cmp(&format!("{coord}.ixf"), &lc.ixf.to_string(), &rc.ixf.to_string())
                            {
                                return Some(diff);
                            }
                        }
                    }
                }
            }
        }
    }
    None
}

pub fn evidence_json(snap: &WorkbookSnapshot) -> Value {
    json!({
        "input_sha256": snap.input_sha256,
        "engine_version": snap.engine_id,
        "environment": "rust-ss-core",
        "open_code": snap.open_code,
        "sheets": snap.sheets.iter().map(|sheet| {
            let mut rows = Vec::new();
            for (offset, height) in sheet.row_heights.iter().enumerate() {
                let row = snap.window_row + offset as i32;
                let cells: Vec<_> = sheet.cells.iter().filter(|cell| cell.row == row).map(|cell| {
                    json!({
                        "col": cell.col,
                        "raw_type": cell.raw_type,
                        "raw_value": cell.raw_value,
                        "xf": cell.ixf,
                        "formula_cache_state": if cell.raw_type == 0 { "blank" } else { "cached" },
                    })
                }).collect();
                if cells.is_empty() && *height == 0 {
                    continue;
                }
                rows.push(json!({
                    "index": row,
                    "height_twip": height,
                    "cells": cells,
                }));
            }
            json!({
                "index": sheet.index,
                "id": sheet.sheet_id,
                "name": sheet.name,
                "hidden": sheet.hidden,
                "rows": rows,
            })
        }).collect::<Vec<_>>(),
    })
}

pub fn require_java_dump(path: &str) -> Result<(), String> {
    if std::path::Path::new(path).is_file() {
        Ok(())
    } else {
        Err(format!("java dump absent: {path}"))
    }
}

fn cmp(path: &str, left: &str, right: &str) -> Option<GridDiff> {
    if left == right {
        None
    } else {
        Some(GridDiff {
            path: path.to_string(),
            left: left.to_string(),
            right: right.to_string(),
        })
    }
}
