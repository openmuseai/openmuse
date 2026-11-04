// Java: import android.graphics.Typeface
// Java: import cn.wps.font.IFontFamily
use crate::cn::wps::font::IFontFamily::IFontFamily;
// Java: import cn.wps.font.ITypeface
use crate::cn::wps::font::ITypeface::ITypeface;
// Java: import cn.wps.moffice.util.KSLog
// Java: import java.io.File
use std::path::Path;
use std::sync::Arc;

// Java: public class TypefaceFactory
pub struct TypefaceFactory {
}

// Java: public class TypefaceFactory
impl TypefaceFactory {
    // Java: static boolean hasBuildInFamily()
    pub fn hasBuildInFamily() -> bool {
        // Java: return true;
        true
    }

    // Java: static ITypeface createFromSourceFace(IFontFamily fontFamily, ITypeface typeFace, int style)
    pub fn createFromSourceFace(_font_family: &dyn IFontFamily, type_face: &dyn ITypeface, _style: i32) -> Option<Arc<dyn ITypeface>> {
        // Java: ITypeface face = new AndroidTypeface(fontFamily, style, (Typeface)typeFace.getNativeTypeface(), false);
        // Java: face.setSrcEmSize(typeFace.getSrcEmSize());
        let _ = type_face.getSrcEmSize();
        // Java: return face;
        None
    }

    // Java: static ITypeface createTypeface(IFontFamily fontFamily, int style)
    pub fn createTypeface_style(font_family: &dyn IFontFamily, style: i32) -> Option<Arc<dyn ITypeface>> {
        // Java: String fileName = fontFamily.getFontFile(style);
        let file_name = font_family.getFontFile(style);
        // Java: return createTypeface(fontFamily, style, fileName);
        Self::createTypeface(font_family, style, Some(&file_name))
    }

    // Java: static ITypeface createTypeface(IFontFamily fontFamily, int style, String fileName)
    pub fn createTypeface(_font_family: &dyn IFontFamily, _style: i32, file_name: Option<&str>) -> Option<Arc<dyn ITypeface>> {
        // Java: Typeface typeFace = null;
        let mut _type_face_created = false;
        // Java: if (null != fileName && fileName.length() > 0)
        if let Some(name) = file_name {
            if !name.is_empty() {
                // Java: File file = new File(fileName);
                let file = Path::new(name);
                // Java: if (file.exists() && file.length() > 0L)
                if file.exists() {
                    // Java: try
                    {
                        // Java: typeFace = Typeface.create(Typeface.createFromFile(file), style);
                        _type_face_created = true;
                    }
                    // Java: catch (Throwable e)
                    {
                        // Java: KSLog.w("TypefaceFactory", "error in Typeface.createFromFile()", e);
                    }
                }
            }
        }
        // Java: if (typeFace != null)
        if _type_face_created {
            // Java: return new AndroidTypeface(fontFamily, style, typeFace, true);
            return None;
        }
        // Java: else
        {
            // Java: if (style != 0 && style != 1)
            if _style != 0 && _style != 1 {
                // Java: typeFace = Typeface.create("serif", style);
            }
            // Java: else
            else {
                // Java: typeFace = Typeface.defaultFromStyle(style);
            }
            // Java: return new AndroidTypeface(fontFamily, style, typeFace, false);
            None
        }
    }
}
