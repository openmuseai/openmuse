use std::collections::HashMap;use crate::cn::wps::moffice::spreadsheet::control::grid::render::native::host;use serde_json::json;
#[derive(Clone)]pub struct NativeBitmap{pub name:String,pub width:i32,pub height:i32}
pub struct RenderDrawablePool;static ICONS:std::sync::LazyLock<std::sync::Mutex<HashMap<String,Option<NativeBitmap>>>>=std::sync::LazyLock::new(||std::sync::Mutex::new(HashMap::new()));
impl RenderDrawablePool{
// java public static Bitmap getIconBitmap(String name) {
// java if (mIconSetList == null) {
// java mIconSetList = new HashMap();
// java }
// java
// java Bitmap bitmap = (Bitmap)mIconSetList.get(name);
// java if (bitmap == null) {
// java IResourceManager rm = Platform.getResourceManager();
// java if (rm != null) {
// java bitmap = BitmapFactory.decodeResource(mContext.getResources(), rm.getDrawableId(name));
// java }
// java
// java mIconSetList.put(name, bitmap);
// java }
// java
// java return bitmap;
// java }
pub fn getIconBitmap(name:&str)->Option<NativeBitmap>{let mut icons=ICONS.lock().unwrap();if let Some(Some(bitmap))=icons.get(name){return Some(bitmap.clone());}let v=host(json!({"op":"bitmapResource","resource":name}));let bitmap=if v["available"].as_bool().unwrap_or(false){Some(NativeBitmap{name:name.into(),width:v["width"].as_i64().expect("bitmap width")as i32,height:v["height"].as_i64().expect("bitmap height")as i32})}else{None};icons.insert(name.into(),bitmap.clone());bitmap}
}
