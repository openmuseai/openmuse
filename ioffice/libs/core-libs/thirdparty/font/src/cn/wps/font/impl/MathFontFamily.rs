// Java: import cn.wps.font.IFontFamily
use crate::cn::wps::font::IFontFamily::IFontFamily;
// Java: import cn.wps.font.IMathFontFamily
use crate::cn::wps::font::IMathFontFamily::IMathFontFamily;
// Java: import cn.wps.font.ITypeface
use crate::cn::wps::font::ITypeface::ITypeface;
use std::sync::Arc;
use super::FontFamilyImpl::FontFamilyImpl;

// Java: public class MathFontFamily implements IMathFontFamily
pub struct MathFontFamily {
    // Java field: FontFamilyImpl mImpl
    mImpl: Arc<FontFamilyImpl>,
}

// Java: public class MathFontFamily implements IMathFontFamily
impl MathFontFamily {
    // Java: MathFontFamily(IFontFamily fontFamily)
    pub fn new(font_family: Arc<FontFamilyImpl>) -> Self {
        // Java: super();
        // Java: this.mImpl = (FontFamilyImpl)fontFamily;
        MathFontFamily {
            mImpl: font_family,
        }
    }

    // Java: ITypeface getTypeface(int mathStyle, int style)
    pub fn getTypeface_impl(&self, math_style: i32, style: i32) -> Option<Arc<dyn ITypeface>> {
        // Java: ITypeface face = null;
        let mut face: Option<Arc<dyn ITypeface>> = None;
        // Java: switch (mathStyle)
        match math_style {
            // Java: case 0:
            0 => {
                // Java: face = this.mImpl.createTypeface(0);
                face = self.mImpl.createTypeface(0);
                // Java: if (style > 0)
                if style > 0 {
                    // Java: AbstractTypeface copy = ((AbstractTypeface)face).clone();
                    // Java: if ((style & 1) > 0)
                    // Java: copy.setNeedBold(true);
                    // Java: face = copy;
                }
                // Java: break;
            }
            // Java: case 1:
            // Java: case 3:
            1 | 3 => {
                // Java: face = this.mImpl.createTypeface(0);
                face = self.mImpl.createTypeface(0);
                // Java: break;
            }
            // Java: case 2:
            // Java: case 4:
            2 | 4 => {
                // Java: face = this.mImpl.createTypeface(1);
                face = self.mImpl.createTypeface(1);
            }
            _ => {}
        }
        // Java: return face;
        face
    }
}

impl IMathFontFamily for MathFontFamily {
    fn getTypeface(&self, var1: i32, var2: i32) -> Option<Arc<dyn ITypeface>> {
        self.getTypeface_impl(var1, var2)
    }
}
