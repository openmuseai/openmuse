use super::super::super::render::native::host;
pub struct BidiTools;
impl BidiTools {
    // java public static boolean needRtl(String source) {
    // java return !TextUtils.isEmpty(source) && Bidi.requiresBidi(source.toCharArray(), 0, source.length()) && !(new Bidi(source, -1)).baseIsLeftToRight();
    // java }
    pub fn needRtl(source: &[u16]) -> bool {
        !source.is_empty()
            && host(serde_json::json!({"op":"bidi","textUtf16":source}))["needRtl"]
                .as_bool()
                .expect("native java.text.Bidi result")
    }
}
