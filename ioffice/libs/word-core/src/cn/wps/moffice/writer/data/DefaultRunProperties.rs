// rust: `MutablePropertySet::setValue` 取 `Option<PropertyValue>`；静态用 `OnceLock`。
use std::sync::Arc;
// Java: import cn.wps.moffice.property.MutablePropertySet
use util::moffice::property::MutablePropertySet::*;
// Java: import cn.wps.moffice.property.PropertySet
use util::moffice::property::PropertySet::*;
// Java: import java.lang.Math
use stubs::java::lang::Math::*;
// Java: import cn.wps.moffice.writer.data.DefaultValue
use crate::cn::wps::moffice::writer::data::DefaultValue::*;
// Java: import cn.wps.moffice.writer.data.PropertyId
use crate::cn::wps::moffice::writer::data::PropertyId::*;

// java public static PropertySet properties;
pub(crate) static properties: std::sync::OnceLock<std::sync::Arc<Option<PropertySet>>> =
                std::sync::OnceLock::new();
// java public static final float[] fontSizes = new float[]{5.0F, 5.5F, 6.5F, 7.5F, 8.0F, 9.0F, 10.0F, 10.5F, 11.0F, 12.0F, 14.0F, 16.0F, 18.0F, 20.0F, 22.0F, 24.0F, 26.0F, 28.0F, 36.0F, 48.0F, 72.0F};
pub(crate) static fontSizes: std::sync::LazyLock<Option<Vec<f32>>> =
                    std::sync::LazyLock::new(|| Some(vec![5.0f32, 5.5, 6.5, 7.5, 8.0, 9.0, 10.0, 10.5,
                                                        11.0, 12.0, 14.0, 16.0, 18.0, 20.0, 22.0, 24.0,
                                                        26.0, 28.0, 36.0, 48.0, 72.0]));
// java public static final float FONTSIZEMAX = 300.0F;
pub(crate) const FONTSIZEMAX: f32 = 300.0f32;
// java public static final float FONTSIZEMIN = 1.0F;
pub(crate) const FONTSIZEMIN: f32 = 1.0f32;

// java public class DefaultRunProperties implements PropertyId
#[derive(Default)]
pub struct DefaultRunProperties {
}

// java public interface PropertyId
impl PropertyId for DefaultRunProperties {
}

// java public class DefaultRunProperties implements PropertyId
impl DefaultRunProperties {
    // java public static PropertySet getProperties()
    pub fn getProperties() -> PropertySet {
      // java if (null == properties)
      if properties.get().is_none() {
        // java MutablePropertySet mutablePropSet = new MutablePropertySet();
        let mut mutablePropSet: MutablePropertySet = MutablePropertySet::new();
        // java initProperties(mutablePropSet);
        Self::initProperties(&mut mutablePropSet);
        // java properties = mutablePropSet.freeze();
        let _ = properties.set(Arc::new(Some(mutablePropSet.freeze())));
      }
      // java return properties;
      return properties.get().and_then(|a| (**a).clone()).unwrap_or_default();
    }

    // java private static void initProperties(MutablePropertySet mutablePropSet)
    pub fn initProperties(mutablePropSet: &mut MutablePropertySet) {
      // java mutablePropSet.setValue(3, "Times New Roman");
      mutablePropSet.setValue(3, Some(Arc::new("Times New Roman")));
      // java mutablePropSet.setValue(4, "宋体");
      mutablePropSet.setValue(4, Some(Arc::new("宋体")));
      // java mutablePropSet.setValue(5, false);
      mutablePropSet.setValue(5, Some(Arc::new(false)));
      // java mutablePropSet.setValue(38, false);
      mutablePropSet.setValue(38, Some(Arc::new(false)));
      // java mutablePropSet.setValue(6, false);
      mutablePropSet.setValue(6, Some(Arc::new(false)));
      // java mutablePropSet.setValue(39, false);
      mutablePropSet.setValue(39, Some(Arc::new(false)));
      // java mutablePropSet.setValue(7, 0);
      mutablePropSet.setValue(7, Some(Arc::new(0)));
      // java mutablePropSet.setValue(8, 0);
      mutablePropSet.setValue(8, Some(Arc::new(0)));
      // java mutablePropSet.setValue(9, -16777216);
      mutablePropSet.setValue(9, Some(Arc::new(-16777216)));
      // java mutablePropSet.setValue(10, 10.0F);
      mutablePropSet.setValue(10, Some(Arc::new(10)));
      // java mutablePropSet.setValue(37, 10.0F);
      mutablePropSet.setValue(37, Some(Arc::new(10)));
      // java mutablePropSet.setValue(11, -16777216);
      mutablePropSet.setValue(11, Some(Arc::new(-16777216)));
      // java mutablePropSet.setValue(12, 0);
      mutablePropSet.setValue(12, Some(Arc::new(0)));
      // java mutablePropSet.setValue(13, 0);
      mutablePropSet.setValue(13, Some(Arc::new(0)));
      // java mutablePropSet.setValue(15, 0);
      mutablePropSet.setValue(15, Some(Arc::new(0)));
    }

    // java static int indexOfSize(float nSize)
    pub fn indexOfSize(nSize: f32) -> i32 {
      let mut i: i32 = 0;
      // java for(int i = 0; i < fontSizes.length - 1; ++i)
      while i < (fontSizes.as_ref().map(|v| v.len()).unwrap_or(0) as i32) - 1 {
        // java if (fontSizes[i] <= nSize && nSize < fontSizes[i + 1])
        if fontSizes.as_ref().unwrap()[(i) as usize] <= nSize && nSize < fontSizes.as_ref().unwrap()[(i + 1) as usize] {
          // java return i;
          return i;
        }
      }
      // java float nLast = fontSizes[fontSizes.length - 1];
      let mut nLast: f32 = fontSizes.as_ref().unwrap()[((fontSizes.as_ref().map(|v| v.len()).unwrap_or(0) as i32) - 1) as usize];
      // java if (nSize >= nLast && nSize < nLast + 0.01F)
      if nSize >= nLast && nSize < nLast + 0.01 {
        // java return fontSizes.length - 1;
        return (fontSizes.as_ref().map(|v| v.len()).unwrap_or(0) as i32) - 1;
      }
      // java else
      else {
        // java return -1;
        return -1;
      }
    }

    // java public static float growSize(float nSize)
    pub fn growSize(nSize: f32) -> f32 {
      // java int iSize = indexOfSize(nSize);
      let mut iSize: i32 = Self::indexOfSize(nSize);
      // java if (iSize != -1 && iSize != fontSizes.length - 1)
      if iSize != -1 && iSize != (fontSizes.as_ref().map(|v| v.len()).unwrap_or(0) as i32) - 1 {
        // java return fontSizes[iSize + 1];
        return fontSizes.as_ref().unwrap()[(iSize + 1) as usize];
      }
      // java else
      else {
        // java return nSize > 70.0F ? Math.min((float)((int)nSize / 10 * 10 + 10), 300.0F) : (float)((int)nSize + 1);
        // rust: 模板吞掉了浮点字面量后缀（`70.0F`→`70`、`300.0F`→`300`）⇒ 按 Java 原文补回。
        return if nSize > 70.0f32 { Math::min(((((nSize) as i32) / 10 * 10 + 10) as f32), 300.0f32) } else { ((((nSize) as i32) + 1) as f32) };
      }
    }

    // java public static float shrinkSize(float nSize)
    pub fn shrinkSize(nSize: f32) -> f32 {
      // java int iSize = indexOfSize(nSize);
      let mut iSize: i32 = Self::indexOfSize(nSize);
      // java if (iSize != -1 && iSize != 0)
      if iSize != -1 && iSize != 0 {
        // java return fontSizes[iSize - 1];
        return fontSizes.as_ref().unwrap()[(iSize - 1) as usize];
      }
      // java else if (nSize < 5.1F)
      else if nSize < 5.1 {
        // java return Math.max((float)((int)nSize - 1), 1.0F);
        return Math::max(((((nSize) as i32) - 1) as f32), 1.0f32);
      }
      // java else
      else {
        // java return nSize > 70.0F && nSize <= 80.0F ? 72.0F : (float)((int)nSize / 10 * 10 - 10);
        return if nSize > 70.0f32 && nSize <= 80.0f32 { 72.0f32 } else { ((((nSize) as i32) / 10 * 10 - 10) as f32) };
      }
    }

}
