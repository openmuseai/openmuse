// rust: Android Paint is the platform adapter, never a spreadsheet/layout implementation.
use jni::{
    objects::{GlobalRef, JClass, JValue},
    sys::{jint, jstring, JNI_VERSION_1_6},
    JNIEnv, JavaVM,
};
use serde_json::Value;
use std::sync::OnceLock;
static VM: OnceLock<JavaVM> = OnceLock::new();
static BRIDGE: OnceLock<GlobalRef> = OnceLock::new();
#[no_mangle]
pub unsafe extern "system" fn JNI_OnLoad(
    vm: *mut jni::sys::JavaVM,
    _reserved: *mut std::ffi::c_void,
) -> jint {
    let jvm = unsafe { JavaVM::from_raw(vm) }.expect("Android JavaVM");
    let _ = VM.set(jvm);
    JNI_VERSION_1_6
}
#[no_mangle]
pub extern "system" fn Java_io_openmuse_ss_1render_1host_NativePaintBridge_nativeInit(
    mut env: JNIEnv,
    class: JClass,
    cacheDir: jni::objects::JString,
) {
    let path: String = env
        .get_string(&cacheDir)
        .expect("Android cache path")
        .into();
    crate::api::SpreadsheetSession::set_native_temp_directory(path.into());
    let global = env.new_global_ref(class).expect("Paint bridge reference");
    let _ = BRIDGE.set(global);
}
pub struct AndroidPaintBackend;
impl super::native::PaintBackend for AndroidPaintBackend {
    fn call(&self, request: &Value) -> Result<Value, String> {
        let vm = VM.get().ok_or("Android Paint VM not initialized")?;
        let class = BRIDGE.get().ok_or("Android Paint bridge not initialized")?;
        let mut env = vm.attach_current_thread().map_err(|e| e.to_string())?;
        let json = env
            .new_string(request.to_string())
            .map_err(|e| e.to_string())?;
        let cls: &JClass = class.as_obj().into();
        let result = env.call_static_method(
            cls,
            "measure",
            "(Ljava/lang/String;)Ljava/lang/String;",
            &[JValue::Object(&json)],
        );
        let result = match result {
            Ok(r) => r,
            Err(e) => {
                let _ = env.exception_describe();
                let _ = env.exception_clear();
                return Err(e.to_string());
            }
        };
        let object = result.l().map_err(|e| e.to_string())?;
        let js = jni::objects::JString::from(object);
        let result: String = env.get_string(&js).map_err(|e| e.to_string())?.into();
        env.delete_local_ref(json).map_err(|e| e.to_string())?;
        env.delete_local_ref(js).map_err(|e| e.to_string())?;
        serde_json::from_str(&result).map_err(|e| e.to_string())
    }
}
