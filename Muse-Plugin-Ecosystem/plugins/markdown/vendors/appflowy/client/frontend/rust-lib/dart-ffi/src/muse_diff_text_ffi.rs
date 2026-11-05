use muse_diff_text::{compare, CancellationToken, DiffOptions, TextSnapshot};
use serde::Serialize;
use std::collections::HashMap;
use std::ffi::{c_char, CStr, CString};
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::sync::{Mutex, OnceLock};
use std::time::Duration;

static REQUESTS: OnceLock<Mutex<HashMap<u64, CancellationToken>>> = OnceLock::new();

fn requests() -> &'static Mutex<HashMap<u64, CancellationToken>> {
  REQUESTS.get_or_init(|| Mutex::new(HashMap::new()))
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct FfiError<'a> {
  error: &'a str,
}

/// Returns a compact JSON change-block response. Ownership of the returned
/// C string is transferred to Dart and must be released with
/// `muse_diff_text_free_json`.
#[no_mangle]
pub unsafe extern "C" fn muse_diff_text_compare_json(
  request_id: u64,
  before: *const c_char,
  after: *const c_char,
  max_millis: u64,
  max_trace_bytes: usize,
) -> *mut c_char {
  let response = catch_unwind(AssertUnwindSafe(|| {
    if before.is_null() || after.is_null() {
      return serde_json::to_string(&FfiError {
        error: "null-input",
      })
      .unwrap();
    }
    let before = match CStr::from_ptr(before).to_str() {
      Ok(value) => value.to_owned(),
      Err(_) => {
        return serde_json::to_string(&FfiError {
          error: "invalid-before-utf8",
        })
        .unwrap()
      },
    };
    let after = match CStr::from_ptr(after).to_str() {
      Ok(value) => value.to_owned(),
      Err(_) => {
        return serde_json::to_string(&FfiError {
          error: "invalid-after-utf8",
        })
        .unwrap()
      },
    };
    let token = CancellationToken::default();
    requests().lock().unwrap().insert(request_id, token.clone());
    let result = compare(
      &TextSnapshot::new(before),
      &TextSnapshot::new(after),
      &DiffOptions {
        max_duration: Duration::from_millis(max_millis.max(1)),
        max_trace_bytes: max_trace_bytes.max(1),
      },
      &token,
    );
    requests().lock().unwrap().remove(&request_id);
    serde_json::to_string(&result).unwrap()
  }))
  .unwrap_or_else(|_| {
    serde_json::to_string(&FfiError {
      error: "runtime-panic",
    })
    .unwrap()
  });
  CString::new(response).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn muse_diff_text_cancel(request_id: u64) -> bool {
  let requests = requests().lock().unwrap();
  let Some(token) = requests.get(&request_id) else {
    return false;
  };
  token.cancel();
  true
}

#[no_mangle]
pub unsafe extern "C" fn muse_diff_text_free_json(value: *mut c_char) {
  if !value.is_null() {
    drop(CString::from_raw(value));
  }
}
