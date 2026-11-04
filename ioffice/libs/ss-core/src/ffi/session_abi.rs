use std::ffi::{CStr, CString};
use std::os::raw::c_char;

use serde_json::{json, Value};

use crate::api::SpreadsheetSession::{
    SpreadsheetLoadConfig, SpreadsheetSession, WorkbookSnapshot, ENGINE_ID,
};

pub struct SessionHandle {
    name: String,
    session: Option<SpreadsheetSession>,
}

#[no_mangle]
pub extern "C" fn ss_core_abi() -> u32 {
    1
}

#[no_mangle]
pub extern "C" fn ss_core_create() -> *mut SessionHandle {
    Box::into_raw(Box::new(SessionHandle {
        name: String::new(),
        session: None,
    }))
}

#[no_mangle]
pub extern "C" fn ss_core_destroy(handle: *mut SessionHandle) {
    if !handle.is_null() {
        unsafe {
            drop(Box::from_raw(handle));
        }
    }
}

#[no_mangle]
pub extern "C" fn ss_core_string_free(ptr: *mut c_char) {
    if !ptr.is_null() {
        unsafe {
            drop(CString::from_raw(ptr));
        }
    }
}

#[no_mangle]
pub extern "C" fn ss_core_open(
    handle: *mut SessionHandle,
    name: *const c_char,
    bytes: *const u8,
    len: isize,
) -> *mut c_char {
    let Some(handle) = (unsafe { handle.as_mut() }) else {
        return reply_err("null session");
    };
    let name = c_str(name).unwrap_or_else(|| "Book1.xlsx".to_string());
    if bytes.is_null() || len < 0 {
        return reply_err("open code 2");
    }
    let bytes = unsafe { std::slice::from_raw_parts(bytes, len as usize) };
    let session = SpreadsheetSession::open_bytes(bytes, SpreadsheetLoadConfig::default());
    let code = session.open_code();
    if code != 1 {
        handle.session = None;
        return reply_err(&format!("open code {code}"));
    }
    let sha = session.input_sha256().to_string();
    let sheet_count = session.snapshot(0, 0, 0, 0).sheet_count;
    handle.name = name;
    handle.session = Some(session);
    reply_ok(json!({
        "engineId": ENGINE_ID,
        "inputSha256": sha,
        "openCode": code,
        "sheetCount": sheet_count,
    }))
}

#[no_mangle]
pub extern "C" fn ss_core_call(handle: *mut SessionHandle, request: *const c_char) -> *mut c_char {
    match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| call_body(handle, request))) {
        Ok(reply) => reply,
        Err(error) => {
            let message = error
                .downcast_ref::<String>()
                .cloned()
                .or_else(|| error.downcast_ref::<&str>().map(|s| s.to_string()))
                .unwrap_or_else(|| "engine exception".into());
            reply_err(&message)
        }
    }
}
fn call_body(handle: *mut SessionHandle, request: *const c_char) -> *mut c_char {
    let Some(handle) = (unsafe { handle.as_mut() }) else {
        return reply_err("null session");
    };
    let Some(text) = c_str(request) else {
        return reply_err("empty request");
    };
    let request: Value = match serde_json::from_str(&text) {
        Ok(value) => value,
        Err(_) => return reply_err("request is not json"),
    };
    let op = request.get("op").and_then(Value::as_str).unwrap_or("");
    match op {
        "configureViewHost" => {
            use crate::cn::wps::moffice::spreadsheet::control::grid::theme::DefaultGridTheme::DefaultGridTheme;
            let phone = request["usePhoneTheme"].as_bool().unwrap_or(false);
            let night = request["nightMode"].as_bool().unwrap_or(false);
            DefaultGridTheme::setThemeStyle(phone);
            DefaultGridTheme::setIsNightMode(night);
            // rust: Host assignment to the original public IconSetData static fields; initialize the class before assigning, as the JVM does.
            if request.get("maxFilterIconWidth").is_some()
                || request.get("minFilterIconWidth").is_some()
            {
                use crate::cn::wps::moffice::spreadsheet::control::grid::render::data::IconSetData;
                use std::sync::atomic::Ordering;
                IconSetData::IconSetData::init();
                if let Some(v) = request["maxFilterIconWidth"].as_i64() {
                    IconSetData::MAX_FILTER_ICON_WIDTH.store(v as i32, Ordering::Relaxed);
                }
                if let Some(v) = request["minFilterIconWidth"].as_i64() {
                    IconSetData::MIN_FILTER_ICON_WIDTH.store(v as i32, Ordering::Relaxed);
                }
            }
            reply_ok(json!({"usePhoneTheme":phone,"nightMode":night}))
        }
        "configureLocale" => {
            let Some(locale) = request
                .get("locale")
                .and_then(Value::as_str)
                .filter(|s| !s.is_empty())
            else {
                return reply_err("locale is required");
            };
            // Host context uses the two original Locale facades and the original notification chain.
            crate::cn::wps::moss::app::KmoLocale::KmoLocale::setLocale(locale.replace('-', "_"));
            kfc::cn::wps::kfc::numfmt::i18n::KmoLang::KmoLang::setLocale(locale.replace('-', "_"));
            kfc::cn::wps::kfc::numfmt::global::NumFmtLocaleHandler::NumFmtLocaleHandler::new()
                .localeChanged_impl();
            reply_ok(json!({"locale":locale}))
        }
        "snapshot" => {
            let Some(session) = handle.session.as_ref() else {
                return reply_err("workbook is not open");
            };
            let row = json_i32(&request, "row");
            let col = json_i32(&request, "col");
            let rows = json_i32(&request, "rows");
            let cols = json_i32(&request, "cols");
            let active = json_i32(&request, "sheet");
            let snap = session.snapshot(row, col, rows, cols);
            reply_ok(product_view(
                &snap,
                active,
                session.is_dirty(),
                &handle.name,
            ))
        }
        "layoutGeometry" | "layoutEvidence" | "layoutMeasure" => {
            use crate::cn::wps::moffice::spreadsheet::control::grid::render::native::{
                platform_backend, PaintHostScope,
            };
            let Some(session) = handle.session.as_ref() else {
                return reply_err("no open workbook");
            };
            let backend = match platform_backend() {
                Ok(b) => b,
                Err(e) => return reply_err(&e),
            };
            let _scope = PaintHostScope::enter(backend);
            let sheet = json_i32_default(&request, "sheet", 0);
            let rows = json_i32_default(&request, "rows", 40);
            let cols = json_i32_default(&request, "cols", 24);
            let observe = |v: &Value| {
                let dpi = v["dpi"].as_f64().unwrap_or(96.0) as f32;
                let zoom = json_i32_default(v, "zoom", 100);
                if op == "layoutMeasure" {
                    session.layout_measure(sheet, dpi, zoom, rows, cols)
                } else {
                    session.layout_evidence(sheet, dpi, zoom, rows, cols, op == "layoutEvidence")
                }
            };
            if let Some(variants) = request["variants"].as_array() {
                reply_ok(json!({"variants":variants.iter().map(observe).collect::<Vec<_>>()}))
            } else {
                reply_ok(observe(&request))
            }
        }
        "renderGrid" => {
            use crate::cn::wps::moffice::spreadsheet::control::grid::render::native::{
                platform_backend, PaintHostScope,
            };
            let Some(session) = handle.session.as_ref() else {
                return reply_err("no open workbook");
            };
            let backend = match platform_backend() {
                Ok(b) => b,
                Err(e) => return reply_err(&e),
            };
            let _scope = PaintHostScope::enter(backend);
            let width = json_i32_default(&request, "width", 1024);
            let height = json_i32_default(&request, "height", 768);
            if width <= 0 || height <= 0 || width > 8192 || height > 8192 {
                return reply_err("invalid viewport");
            }
            let render = |v: &Value| {
                session.render_grid(
                    json_i32_default(&request, "sheet", 0),
                    v["dpi"].as_f64().unwrap_or(96.0) as f32,
                    json_i32_default(v, "zoom", 100),
                    json_i32_default(&request, "rows", 40),
                    json_i32_default(&request, "cols", 24),
                    width,
                    height,
                )
            };
            if let Some(views) = request["variants"].as_array() {
                reply_ok(json!({"variants":views.iter().map(render).collect::<Vec<_>>()}))
            } else {
                reply_ok(render(&request))
            }
        }
        "measureSpan" => {
            use crate::cn::wps::moffice::spreadsheet::control::grid::{
                layout::{data::SpanInfo::SpanInfo, typo::PaintFontHelper::PaintFontHelper},
                render::native::{platform_backend, PaintHostScope},
            };
            let backend = match platform_backend() {
                Ok(b) => b,
                Err(e) => return reply_err(&e),
            };
            let _scope = PaintHostScope::enter(backend);
            let span: SpanInfo = match serde_json::from_value(request["span"].clone()) {
                Ok(s) => s,
                Err(e) => return reply_err(&e.to_string()),
            };
            let text = request["text"]
                .as_str()
                .unwrap_or("0")
                .encode_utf16()
                .collect::<Vec<_>>();
            let metrics = PaintFontHelper::getFontMetricsInt(&span);
            reply_ok(
                json!({"fontMetrics":{"top":metrics.top,"ascent":metrics.ascent,"descent":metrics.descent,"bottom":metrics.bottom,"leading":metrics.leading},"charWidth":PaintFontHelper::getCharWidth__span(&text,&span) as f64}),
            )
        }
        "commands" => reply_ok(json!([])),
        "imageBytes" => reply_ok(json!({"available": false})),
        "execute" => {
            let id = request.get("id").and_then(Value::as_str).unwrap_or("");
            reply_err(&format!("unsupported capability: {id}"))
        }
        "export" => {
            let format = request.get("format").and_then(Value::as_str).unwrap_or("");
            reply_err(&format!("unsupported capability: export {format}"))
        }
        "new" => reply_err("unsupported capability: new"),
        other => reply_err(&format!("unsupported capability: {other}")),
    }
}

pub fn product_view(snap: &WorkbookSnapshot, active: i32, dirty: bool, title: &str) -> Value {
    let active = if (0..snap.sheets.len() as i32).contains(&active) {
        active
    } else {
        0
    };
    let cells = snap
        .sheets
        .iter()
        .find(|sheet| sheet.index == active)
        .map(|sheet| {
            sheet
                .cells
                .iter()
                .map(|cell| {
                    json!({
                        "row": cell.row,
                        "col": cell.col,
                        "text": cell.display_text,
                        "displayText":cell.display_text,
                        "rawValue":cell.raw_value,
                        "style":cell.style,
                        "formatColor":cell.format_color,
                        "fillPos":cell.fill_pos,"fillChar":cell.fill_char,
                        "numericAsText":cell.numeric_as_text,"richFormatInvalid":cell.rich_format_invalid,
                        "rawType": cell.raw_type,
                        "ixf": cell.ixf,
                        "formulaType": cell.formula_type,
                    })
                })
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    let row_heights = snap
        .sheets
        .iter()
        .find(|sheet| sheet.index == active)
        .map(|sheet| sheet.row_heights.clone())
        .unwrap_or_default();
    let column_widths = snap
        .sheets
        .iter()
        .find(|sheet| sheet.index == active)
        .map(|sheet| sheet.column_widths.clone())
        .unwrap_or_default();
    let merged_ranges = snap
        .sheets
        .iter()
        .find(|sheet| sheet.index == active)
        .map(|sheet| sheet.merged_ranges.clone())
        .unwrap_or_default();
    json!({
        "locale":kfc::cn::wps::kfc::numfmt::i18n::KmoLang::KmoLang::getSystemLocale(),
        "lcid":kfc::cn::wps::kfc::numfmt::i18n::KmoLang::KmoLang::getSystemLCID(),
        "engineId": snap.engine_id,
        "inputSha256": snap.input_sha256,
        "openCode": snap.open_code,
        "sheetCount": snap.sheet_count,
        "xfCount": snap.xf_count,
        "cellCount": snap.cell_count,
        "title": title,
        "activeSheet": active,
        "dirty": dirty,
        "sheets": snap.sheets.iter().map(|sheet| sheet.name.clone()).collect::<Vec<_>>(),
        "sheetStates": snap.sheets.iter().map(|sheet| json!({"id": sheet.sheet_id, "hidden": sheet.hidden})).collect::<Vec<_>>(),
        "cells": cells,
        "rowHeights": row_heights,
        "columnWidths": column_widths,
        "mergedRanges": merged_ranges,
        "selection": "A1",
        "contexts": [],
        "visuals": [],
    })
}

fn json_i32(value: &Value, key: &str) -> i32 {
    value.get(key).and_then(Value::as_i64).unwrap_or(0) as i32
}

fn c_str(ptr: *const c_char) -> Option<String> {
    if ptr.is_null() {
        return None;
    }
    unsafe {
        CStr::from_ptr(ptr)
            .to_str()
            .ok()
            .map(|text| text.to_string())
    }
}

fn reply_ok(result: Value) -> *mut c_char {
    reply(json!({"ok": true, "result": result}))
}

fn reply_err(message: &str) -> *mut c_char {
    reply(json!({"ok": false, "error": message}))
}

fn reply(value: Value) -> *mut c_char {
    CString::new(value.to_string())
        .unwrap_or_else(|_| CString::new(r#"{"ok":false,"error":"json"}"#).unwrap())
        .into_raw()
}

fn json_i32_default(v: &Value, k: &str, d: i32) -> i32 {
    v.get(k)
        .and_then(Value::as_i64)
        .map(|n| n as i32)
        .unwrap_or(d)
}
