//! Versioned, process-local bridge between the Flutter grid and GridCraft's engine.
//! Product resources never call this library directly; the Host owns the session.

use std::ffi::{CStr, CString, c_char};
use std::panic::{AssertUnwindSafe, catch_unwind};

use gridcraft_chart::{ApproxMeasure, render, resolve};
use gridcraft_engine::{Session, core::CellRef, display};
use serde_json::{Value, json};

const MAX_DOCUMENT_BYTES: usize = 128 * 1024 * 1024;
const MAX_REQUEST_BYTES: usize = 1024 * 1024;
const MAX_IMAGE_TRANSFER_BYTES: usize = 2 * 1024 * 1024;

fn reply(value: Value) -> *mut c_char {
    let encoded = value.to_string().replace('\0', "");
    CString::new(encoded).map_or(std::ptr::null_mut(), CString::into_raw)
}

fn guarded(f: impl FnOnce() -> Result<Value, String>) -> *mut c_char {
    let value = match catch_unwind(AssertUnwindSafe(f)) {
        Ok(Ok(value)) => json!({"ok": true, "result": value}),
        Ok(Err(error)) => json!({"ok": false, "error": error}),
        Err(_) => json!({"ok": false, "error": "Spreadsheet engine failed safely"}),
    };
    reply(value)
}

fn session<'a>(handle: *mut Session) -> Result<&'a mut Session, String> {
    // The Host owns the opaque handle and never shares one handle across concurrent calls.
    unsafe { handle.as_mut() }.ok_or_else(|| "Spreadsheet session is closed".into())
}

fn string(ptr: *const c_char) -> Result<String, String> {
    if ptr.is_null() {
        return Err("Missing string argument".into());
    }
    unsafe { CStr::from_ptr(ptr) }
        .to_str()
        .map(str::to_owned)
        .map_err(|_| "Invalid UTF-8 argument".into())
}

#[unsafe(no_mangle)]
pub extern "C" fn om_sheet_abi() -> u32 {
    1
}

#[unsafe(no_mangle)]
pub extern "C" fn om_sheet_create() -> *mut Session {
    Box::into_raw(Box::new(Session::new()))
}

#[unsafe(no_mangle)]
pub extern "C" fn om_sheet_destroy(handle: *mut Session) {
    if !handle.is_null() {
        unsafe {
            drop(Box::from_raw(handle));
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn om_sheet_string_free(ptr: *mut c_char) {
    if !ptr.is_null() {
        unsafe {
            drop(CString::from_raw(ptr));
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn om_sheet_open(
    handle: *mut Session,
    name: *const c_char,
    bytes: *const u8,
    len: usize,
) -> *mut c_char {
    guarded(|| {
        let name = string(name)?;
        if len > MAX_DOCUMENT_BYTES || (len > 0 && bytes.is_null()) {
            return Err("Spreadsheet document size or pointer invalid".into());
        }
        let data = if len == 0 {
            &[]
        } else {
            unsafe { std::slice::from_raw_parts(bytes, len) }
        };
        let (book, warnings) =
            gridcraft_engine::io::open_bytes(&name, data).map_err(|e| e.to_string())?;
        let title = name.rsplit('/').next().unwrap_or(&name).to_string();
        let index =
            session(handle)?.add_document(gridcraft_engine::DocState::new(book, None, title));
        Ok(json!({"index": index, "warnings": warnings}))
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn om_sheet_call(handle: *mut Session, request: *const c_char) -> *mut c_char {
    guarded(|| {
        let raw = string(request)?;
        if raw.len() > MAX_REQUEST_BYTES {
            return Err("Spreadsheet request too large".into());
        }
        let request: Value = serde_json::from_str(&raw).map_err(|e| e.to_string())?;
        let op = request
            .get("op")
            .and_then(Value::as_str)
            .ok_or("Missing operation")?;
        let state = session(handle)?;
        match op {
            "new" => Ok(json!({"index": state.new_workbook()})),
            "snapshot" => snapshot(state, &request),
            "imageBytes" => {
                let id = request.get("id").and_then(Value::as_u64)
                    .and_then(|value| u32::try_from(value).ok()).ok_or("Missing image ID")?;
                let doc = state.doc().map_err(|e| e.to_string())?;
                let sheet = doc.wb.active().ok_or("Workbook has no active sheet")?;
                let image = sheet.images.iter().find(|image| image.id == id)
                    .ok_or("Workbook image not found")?;
                let supported = matches!(image.mime.as_str(), "image/png" | "image/jpeg" | "image/webp" | "image/gif");
                if !supported || image.data.len() > MAX_IMAGE_TRANSFER_BYTES {
                    Ok(json!({"available": false, "bytes": image.data.len(), "mime": image.mime}))
                } else {
                    Ok(json!({"available": true, "mime": image.mime,
                        "base64": gridcraft_engine::io::base64_encode(&image.data)}))
                }
            }
            "commands" => serde_json::to_value(state.commands()).map_err(|e| e.to_string()),
            "execute" => {
                let id = request.get("id").and_then(Value::as_str).ok_or("Missing command ID")?;
                let params = request.get("params").cloned().unwrap_or_else(|| json!({}));
                state.execute(id, params).map_err(|e| e.to_string())
            }
            "export" => state.execute("file.saveBytes", json!({"format": request.get("format").and_then(Value::as_str).unwrap_or("xlsx")})).map_err(|e| e.to_string()),
            _ => Err(format!("Unknown spreadsheet operation: {op}")),
        }
    })
}

fn snapshot(state: &Session, request: &Value) -> Result<Value, String> {
    let doc = state.doc().map_err(|e| e.to_string())?;
    let wb = &doc.wb;
    let sheet = wb.active().ok_or("Workbook has no active sheet")?;
    let row = request
        .get("row")
        .and_then(Value::as_u64)
        .unwrap_or(0)
        .min(1_048_575) as u32;
    let col = request
        .get("col")
        .and_then(Value::as_u64)
        .unwrap_or(0)
        .min(16_383) as u32;
    let rows = request
        .get("rows")
        .and_then(Value::as_u64)
        .unwrap_or(35)
        .clamp(1, 512) as u32;
    let cols = request
        .get("cols")
        .and_then(Value::as_u64)
        .unwrap_or(16)
        .clamp(1, 160) as u32;
    let selection_stats = display::stats(sheet, &doc.selection.ranges);
    let selection = doc.selection.current();
    let selection_extent = json!({
        "width": (sheet.col_left(selection.end.col + 1)
            - sheet.col_left(selection.start.col)).max(1.0),
        "height": (sheet.row_top(selection.end.row + 1)
            - sheet.row_top(selection.start.row)).max(1.0),
        "cols": selection.end.col - selection.start.col + 1,
        "rows": selection.end.row - selection.start.row + 1,
    });
    // A default size plus sparse deviations lets the Flutter surface position
    // far-away cells without transferring a million row records on each snapshot.
    // Bound pathological workbooks and report the limit rather than implying
    // that a partial axis is exact.
    const MAX_ROW_GEOMETRY_OVERRIDES: usize = 65_536;
    let mut row_sizes = sheet.rows.iter().filter_map(|(&index, info)| {
        let size = if info.hidden { 0.0 } else { info.size.unwrap_or(sheet.default_row_height) };
        (size != sheet.default_row_height).then_some((index, size))
    }).take(MAX_ROW_GEOMETRY_OVERRIDES + 1).collect::<Vec<_>>();
    let rows_complete = row_sizes.len() <= MAX_ROW_GEOMETRY_OVERRIDES;
    row_sizes.truncate(MAX_ROW_GEOMETRY_OVERRIDES);
    let col_sizes = sheet.cols.iter().filter_map(|(&index, info)| {
        let size = if info.hidden { 0.0 } else { info.size.unwrap_or(sheet.default_col_width) };
        (size != sheet.default_col_width).then_some((index, size))
    }).collect::<Vec<_>>();
    let contexts = if sheet
        .tables
        .iter()
        .any(|table| table.range.contains(doc.selection.active))
    {
        vec!["table"]
    } else {
        vec![]
    };
    let mut cells = Vec::new();
    for r in row..row.saturating_add(rows).min(1_048_576) {
        for c in col..col.saturating_add(cols).min(16_384) {
            let at = CellRef::new(r, c);
            if sheet.cell(at).is_none() && sheet.value_ref(at).is_none() {
                continue;
            }
            let style = wb.styles.get(sheet.style_id(at));
            cells.push(json!({
                "row": r, "col": c,
                "text": display::cell_text(wb, sheet, at),
                "input": sheet.input_text(at),
                "bold": style.font.bold,
                "italic": style.font.italic,
                "fontSize": style.font.size,
                "fontColor": style.font.color.hex(&wb.theme),
                "fillColor": style.fill.fg.hex(&wb.theme),
            }));
        }
    }
    // The workbook owns floating objects. Keep this transport bounded; the UI
    // paints only the active sheet and never stores a second chart model.
    let mut visuals = sheet
        .charts
        .iter()
        .take(16)
        .map(|chart| {
            let anchor = chart.anchor;
            let width = anchor.width.clamp(20.0, 1000.0);
            let height = anchor.height.clamp(20.0, 800.0);
            let data = resolve(wb, wb.active_sheet, chart);
            let primitives = render(chart, &data, width, height, &ApproxMeasure);
            let too_complex = primitives.len() > 1500;
            json!({
                "id": chart.id,
                "kind": "chart",
                "chartKind": format!("{:?}", chart.kind),
                "title": chart.title,
                "anchor": {"row": anchor.cell.row, "col": anchor.cell.col,
                    "dx": anchor.dx, "dy": anchor.dy, "width": width, "height": height},
                "primitives": if too_complex { json!([]) } else { json!(primitives) },
                "tooComplex": too_complex,
            })
        })
        .collect::<Vec<_>>();
    visuals.extend(sheet.shapes.iter().take(16).map(|shape| {
        let anchor = shape.anchor;
        let text = shape.text.chars().take(2048).collect::<String>();
        let text_truncated = text.len() < shape.text.len();
        json!({
            "id": shape.id, "kind": "shape", "shapeKind": format!("{:?}", shape.kind),
            "anchor": {"row": anchor.cell.row, "col": anchor.cell.col,
                "dx": anchor.dx, "dy": anchor.dy, "width": anchor.width, "height": anchor.height},
            "fillColor": shape.fill.hex(&wb.theme), "lineColor": shape.line.hex(&wb.theme),
            "text": text, "textTruncated": text_truncated,
            "format": {"noLine": shape.format.no_line,
                "fontSize": shape.format.font_size, "bold": shape.format.bold,
                "italic": shape.format.italic, "font": shape.format.font,
                "textColor": shape.format.text_color.and_then(|color| color.hex(&wb.theme)),
                "align": shape.format.align, "anchor": shape.format.anchor},
        })
    }));
    visuals.extend(sheet.images.iter().take(8).map(|image| {
        let anchor = image.anchor;
        let supported = matches!(
            image.mime.as_str(),
            "image/png" | "image/jpeg" | "image/webp" | "image/gif"
        );
        json!({
            "id": image.id, "kind": "image", "alt": image.alt,
            "mime": image.mime, "bytes": image.data.len(),
            "mediaAvailable": supported && image.data.len() <= MAX_IMAGE_TRANSFER_BYTES,
            "anchor": {"row": anchor.cell.row, "col": anchor.cell.col,
                "dx": anchor.dx, "dy": anchor.dy, "width": anchor.width, "height": anchor.height},
        })
    }));
    Ok(json!({
        "engine": "gridcraft", "abi": 1,
        "revision": doc.revision,
        "dirty": doc.is_dirty(),
        "title": doc.title,
        "sheets": wb.sheets.iter().map(|s| &s.name).collect::<Vec<_>>(),
        "sheetStates": wb.sheets.iter().map(|s| json!({
            "name": s.name, "visibility": s.visibility,
            "tabColor": s.tab_color.and_then(|color| color.hex(&wb.theme)),
        })).collect::<Vec<_>>(),
        "activeSheet": wb.active_sheet,
        "view": sheet.view.as_str(),
        "selection": doc.selection.active.a1(),
        "selectionInput": sheet.input_text(doc.selection.active),
        "selectionRanges": doc.selection.ranges.iter().map(|range| json!({
            "startRow": range.start.row,
            "startCol": range.start.col,
            "endRow": range.end.row,
            "endCol": range.end.col,
        })).collect::<Vec<_>>(),
        "selectionStats": selection_stats,
        "selectionExtent": selection_extent,
        "sheetGeometry": {
            "defaultRowHeight": sheet.default_row_height,
            "defaultColWidth": sheet.default_col_width,
            "rowSizes": row_sizes,
            "colSizes": col_sizes,
            "rowsComplete": rows_complete,
        },
        "contexts": contexts,
        "zoom": sheet.zoom,
        "cells": cells,
        "visuals": visuals,
        "visualsOmitted": sheet.charts.len().saturating_sub(16)
            + sheet.shapes.len().saturating_sub(16)
            + sheet.images.len().saturating_sub(8),
    }))
}
