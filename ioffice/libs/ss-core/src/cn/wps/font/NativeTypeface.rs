use serde::{Deserialize, Serialize};
// rust: Value handle for the platform Typeface object; construction remains a platform graphics leaf.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct NativeTypeface {
    pub family: String,
    pub style: i32,
    pub file: Option<String>,
}
