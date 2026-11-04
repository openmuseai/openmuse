use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::cell::RefCell;
use std::sync::Arc;
// rust: Platform boundary for Android Paint / CoreText / Flutter host measurement.
// No font widths or layout decisions are fabricated when a host is absent.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct FontMetrics {
    pub top: f32,
    pub ascent: f32,
    pub descent: f32,
    pub bottom: f32,
    pub leading: f32,
}
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct FontMetricsInt {
    pub top: i32,
    pub ascent: i32,
    pub descent: i32,
    pub bottom: i32,
    pub leading: i32,
    pub width: i32,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Paint {
    pub baselineShift: i32,
    pub nativeFontFamily: String,
    pub nativeFontStyle: i32,
    pub nativeFontFile: Option<String>,
    pub shader: Option<Value>,
    pub pathEffect: Option<Value>,
    pub strokeCap: i32,
    pub strokeJoin: i32,
    pub useFontHost: bool,
    pub fontName: String,
    pub textSize: f32,
    pub color: i32,
    pub flags: i32,
    pub fakeBold: bool,
    pub skewX: f32,
    pub scaleX: f32,
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
    pub strike: bool,
    pub strokeWidth: f32,
    pub style: i32,
    pub textAlign: i32,
}
impl Default for Paint {
    fn default() -> Self {
        Self {
            baselineShift: 0,
            nativeFontFamily: String::new(),
            nativeFontStyle: 0,
            nativeFontFile: None,
            shader: None,
            pathEffect: None,
            strokeCap: 0,
            strokeJoin: 0,
            useFontHost: false,
            fontName: "".into(),
            textSize: paint_defaults().1,
            color: -16777216,
            flags: paint_defaults().0,
            fakeBold: false,
            skewX: 0.0,
            scaleX: 1.0,
            bold: false,
            italic: false,
            underline: false,
            strike: false,
            strokeWidth: 0.0,
            style: 0,
            textAlign: 0,
        }
    }
}
pub trait PaintBackend {
    fn call(&self, request: &Value) -> Result<Value, String>;
}
thread_local! {static HOST:RefCell<Option<Arc<dyn PaintBackend>>>=RefCell::new(None);}
thread_local! {static DEFAULTS:std::cell::Cell<(i32,f32)>=const{std::cell::Cell::new((1280,16.0))};}
fn paint_defaults() -> (i32, f32) {
    DEFAULTS.with(|v| v.get())
}
pub struct PaintHostScope(Option<Arc<dyn PaintBackend>>, (i32, f32));
impl PaintHostScope {
    pub fn enter(backend: Arc<dyn PaintBackend>) -> Self {
        let v = backend
            .call(&serde_json::json!({"op":"paintDefaults"}))
            .expect("native Paint defaults");
        let defaults = (
            v["flags"].as_i64().expect("Paint flags") as i32,
            v["textSize"].as_f64().expect("Paint text size") as f32,
        );
        Self(
            HOST.with(|h| h.replace(Some(backend))),
            DEFAULTS.with(|v| v.replace(defaults)),
        )
    }
}
impl Drop for PaintHostScope {
    fn drop(&mut self) {
        HOST.with(|h| {
            h.replace(self.0.take());
        });
        DEFAULTS.with(|v| v.set(self.1));
    }
}
pub fn host(request: Value) -> Value {
    HOST.with(|h| {
        let backend = h
            .borrow()
            .as_ref()
            .cloned()
            .expect("M3 native Paint host is not configured");
        backend
            .call(&request)
            .unwrap_or_else(|e| panic!("M3 native Paint: {e}"))
    })
}
impl Paint {
    pub fn setTypeface(&mut self, face: &crate::cn::wps::font::NativeTypeface::NativeTypeface) {
        self.nativeFontFamily = face.family.clone();
        self.nativeFontStyle = face.style;
        self.nativeFontFile = face.file.clone();
    }
    pub fn getFontMetrics(&self) -> FontMetrics {
        serde_json::from_value(self.measure(&[])["metricsFloat"].clone())
            .expect("native Paint float metrics")
    }
    pub fn measure(&self, text: &[u16]) -> Value {
        host(serde_json::json!({"op":"paint","paint":self,"textUtf16":text}))
    }
    pub fn measureText(&self, text: &[u16]) -> f32 {
        self.measure(text)["width"]
            .as_f64()
            .expect("native Paint width") as f32
    }
    pub fn getTextWidths(&self, text: &[u16]) -> Vec<f32> {
        self.measure(text)["widths"]
            .as_array()
            .expect("native Paint widths")
            .iter()
            .map(|v| v.as_f64().expect("native glyph width") as f32)
            .collect()
    }
    pub fn getFontMetricsInt(&self) -> FontMetricsInt {
        serde_json::from_value(self.measure(&[])["metrics"].clone()).expect("native Paint metrics")
    }
}

// rust: Native-only C ABI callback. Caller retains the response bytes until the next callback.
pub type PaintCallback =
    unsafe extern "C" fn(*const std::os::raw::c_char) -> *const std::os::raw::c_char;
thread_local! {static CALLBACK:std::cell::Cell<Option<PaintCallback>>=const{std::cell::Cell::new(None)};}
#[no_mangle]
pub extern "C" fn ss_core_set_paint_callback(callback: Option<PaintCallback>) {
    CALLBACK.with(|c| c.set(callback));
}
struct CallbackBackend(PaintCallback);
impl PaintBackend for CallbackBackend {
    fn call(&self, request: &Value) -> Result<Value, String> {
        let input = std::ffi::CString::new(request.to_string()).map_err(|e| e.to_string())?;
        let p = unsafe { (self.0)(input.as_ptr()) };
        if p.is_null() {
            return Err("Native Paint callback failed".into());
        }
        let text = unsafe { std::ffi::CStr::from_ptr(p) }
            .to_str()
            .map_err(|e| e.to_string())?;
        let out: Value = serde_json::from_str(text).map_err(|e| e.to_string())?;
        if let Some(e) = out.get("error") {
            return Err(e.to_string());
        }
        Ok(out)
    }
}
pub fn platform_backend() -> Result<Arc<dyn PaintBackend>, String> {
    #[cfg(target_os = "android")]
    {
        return Ok(Arc::new(super::android::AndroidPaintBackend));
    }
    #[cfg(not(target_os = "android"))]
    {
        CALLBACK.with(|c| {
            c.get()
                .map(|f| Arc::new(CallbackBackend(f)) as Arc<dyn PaintBackend>)
                .ok_or_else(|| "M3 native Paint host is not configured".into())
        })
    }
}
