use super::super::NativeTypeface::NativeTypeface;
// java class TypefaceFactory
pub struct TypefaceFactory;
impl TypefaceFactory {
    // java public static ITypeface createTypeface(IFontFamily fontFamily, int style, String fileName) {
    // java Typeface typeFace = null;
    // java if (null != fileName && fileName.length() > 0) {
    // java File file = new File(fileName);
    // java if (file.exists() && file.length() > 0L) {
    // java try {
    // java typeFace = Typeface.create(Typeface.createFromFile(file), style);
    // java } catch (Throwable e) {
    // java KSLog.w("TypefaceFactory", "error in Typeface.createFromFile()", e);
    // java }
    // java }
    // java }
    // java
    // java if (typeFace != null) {
    // java return new AndroidTypeface(fontFamily, style, typeFace, true);
    // java } else {
    // java if (style != 0 && style != 1) {
    // java typeFace = Typeface.create("serif", style);
    // java } else {
    // java typeFace = Typeface.defaultFromStyle(style);
    // java }
    // java
    // java return new AndroidTypeface(fontFamily, style, typeFace, false);
    // java }
    // java }
    pub fn createTypeface(style: i32, file: Option<&str>) -> NativeTypeface {
        if let Some(file) = file {
            let path = std::path::Path::new(file);
            if path.is_file() && path.metadata().map(|m| m.len() > 0).unwrap_or(false) {
                return NativeTypeface {
                    family: String::new(),
                    style,
                    file: Some(file.into()),
                };
            }
        }
        if style != 0 && style != 1 {
            NativeTypeface {
                family: "serif".into(),
                style,
                file: None,
            }
        } else {
            NativeTypeface {
                family: String::new(),
                style,
                file: None,
            }
        }
    }
}
