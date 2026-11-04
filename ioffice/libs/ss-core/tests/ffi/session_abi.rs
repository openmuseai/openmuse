use std::ffi::{CStr, CString};

use serde_json::Value;
use ss_core::ffi::session_abi::{
    ss_core_abi, ss_core_call, ss_core_create, ss_core_destroy, ss_core_open, ss_core_string_free,
};

fn fixture(name: &str) -> Vec<u8> {
    let path = format!(
        "{}/rewrite-review/milestones/m1-xlsx-grid-read/fixtures/{name}",
        env!("CARGO_MANIFEST_DIR")
    );
    std::fs::read(path).unwrap()
}

fn take(ptr: *mut std::os::raw::c_char) -> Value {
    assert!(!ptr.is_null());
    let text = unsafe { CStr::from_ptr(ptr) }.to_str().unwrap().to_string();
    ss_core_string_free(ptr);
    serde_json::from_str(&text).unwrap()
}

fn call(handle: *mut ss_core::ffi::session_abi::SessionHandle, request: &str) -> Value {
    let request = CString::new(request).unwrap();
    take(ss_core_call(handle, request.as_ptr()))
}

#[test]
fn ffi1_open_and_snapshot_use_ss_core() {
    let _case = super::SESSION_SERIAL.lock().unwrap();
    assert_eq!(ss_core_abi(), 1);
    let handle = ss_core_create();
    let bytes = fixture("basic-types.xlsx");
    let name = CString::new("basic-types.xlsx").unwrap();
    let opened = take(ss_core_open(
        handle,
        name.as_ptr(),
        bytes.as_ptr(),
        bytes.len() as isize,
    ));
    assert_eq!(opened["ok"], true);
    assert_eq!(opened["result"]["engineId"], "ss-core");
    assert_eq!(
        opened["result"]["inputSha256"],
        "c48f21bb6c390da8ce6b9a6a3e284593ed5942d2b2bf5ea49c897bc0297a20a9"
    );
    assert_eq!(opened["result"]["sheetCount"], 1);

    let snap = call(
        handle,
        r#"{"op":"snapshot","row":0,"col":0,"rows":2,"cols":7,"sheet":0}"#,
    );
    assert_eq!(snap["ok"], true);
    let result = &snap["result"];
    assert_eq!(result["engineId"], "ss-core");
    assert_eq!(result["sheetCount"], 1);
    assert!(result["cellCount"].as_i64().unwrap() >= 6);
    assert_eq!(result["sheets"][0], "Sheet1");
    let a1 = result["cells"]
        .as_array()
        .unwrap()
        .iter()
        .find(|cell| cell["row"] == 0 && cell["col"] == 0)
        .unwrap();
    assert_eq!(a1["text"], "1.5");
    assert_eq!(a1["rawType"], 1);
    assert_eq!(a1["ixf"], 15);
    ss_core_destroy(handle);
}

#[test]
fn ffi2_edit_export_and_bad_package_fail_explicitly() {
    let _case = super::SESSION_SERIAL.lock().unwrap();
    let handle = ss_core_create();
    let bytes = fixture("not-workbook.zip");
    let name = CString::new("not-workbook.zip").unwrap();
    let opened = take(ss_core_open(
        handle,
        name.as_ptr(),
        bytes.as_ptr(),
        bytes.len() as isize,
    ));
    assert_eq!(opened["ok"], false);
    assert!(opened["error"].as_str().unwrap().contains("open code 2"));

    let bytes = fixture("empty.xlsx");
    let opened = take(ss_core_open(
        handle,
        name.as_ptr(),
        bytes.as_ptr(),
        bytes.len() as isize,
    ));
    assert_eq!(opened["ok"], true);
    let edit = call(handle, r#"{"op":"execute","id":"edit.paste"}"#);
    assert_eq!(edit["ok"], false);
    assert!(edit["error"]
        .as_str()
        .unwrap()
        .contains("unsupported capability"));
    let export = call(handle, r#"{"op":"export","format":"xlsx"}"#);
    assert_eq!(export["ok"], false);
    let created = call(handle, r#"{"op":"new"}"#);
    assert_eq!(created["ok"], false);
    ss_core_destroy(handle);
}
