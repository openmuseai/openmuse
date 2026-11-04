pub struct FontNameDecoder;
impl FontNameDecoder {
    // java public static String removeEncodingSuffix(String name) {
    // java if (null == name) {
    // java return null;
    // java } else {
    // java int index = name.lastIndexOf(95);
    // java if (index > 0) {
    // java for(String encoding : encodings) {
    // java if (startsWithNoCase(name, index + 1, encoding)) {
    // java name = name.substring(0, index);
    // java break;
    // java }
    // java }
    // java }
    // java
    // java return name;
    // java }
    // java }
    pub fn removeEncodingSuffix(name: &str) -> String {
        if let Some(i) = name.rfind('_') {
            if i > 0 {
                let suffix = &name[i + 1..];
                for encoding in ["GB18030", "GBK", "GB2312"] {
                    if suffix
                        .get(..encoding.len())
                        .is_some_and(|s| s.eq_ignore_ascii_case(encoding))
                    {
                        return name[..i].into();
                    }
                }
            }
        }
        name.into()
    }
}
