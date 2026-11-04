// Java: import cn.wps.base.assertion.Assert
use util::base::assertion::Assert::Assert;
use super::FontVariant::FontVariant;

// Java: public class CssHelper
pub struct CssHelper {
}

// Java: public class CssHelper
impl CssHelper {
    // Java: CssHelper()
    pub fn new() -> Self {
      // Java: super();
      CssHelper {}
    }

    // Java: static boolean convertFontVariant2SmallCaps(FontVariant fontVariant)
    pub fn convertFontVariant2SmallCaps(fontVariant: FontVariant) -> bool {
      // Java: Assert.assertNotNull("fontVariant should not be null!", fontVariant);
      // Java: return FontVariant.SMALLCAPS == fontVariant;
      FontVariant::SMALLCAPS == fontVariant
    }

}
