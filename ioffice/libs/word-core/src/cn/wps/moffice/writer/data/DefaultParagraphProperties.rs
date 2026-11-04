// rust: `MutablePropertySet::setValue` 取 `Option<PropertyValue>`。
use std::sync::Arc;
// Java: import cn.wps.moffice.property.MutablePropertySet
use util::moffice::property::MutablePropertySet::*;
// Java: import cn.wps.moffice.property.PropertySet
use util::moffice::property::PropertySet::*;
// Java: import cn.wps.moffice.writer.data.DefaultValue
use crate::cn::wps::moffice::writer::data::DefaultValue::*;
// Java: import cn.wps.moffice.writer.data.PropertyId
use crate::cn::wps::moffice::writer::data::PropertyId::*;

// java public static PropertySet properties;
pub(crate) static properties: std::sync::OnceLock<std::sync::Arc<Option<PropertySet>>> =
                std::sync::OnceLock::new();

// java public class DefaultParagraphProperties implements PropertyId
#[derive(Default)]
pub struct DefaultParagraphProperties {
}

// java public interface PropertyId
impl PropertyId for DefaultParagraphProperties {
}

// java public class DefaultParagraphProperties implements PropertyId
impl DefaultParagraphProperties {
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
      // java mutablePropSet.setValue(193, 0);
      mutablePropSet.setValue(193, Some(Arc::new(0)));
      // java mutablePropSet.setValue(214, 0);
      mutablePropSet.setValue(214, Some(Arc::new(0)));
      // java mutablePropSet.setValue(216, false);
      mutablePropSet.setValue(216, Some(Arc::new(false)));
      // java mutablePropSet.setValue(217, 0);
      mutablePropSet.setValue(217, Some(Arc::new(0)));
      // java mutablePropSet.setValue(219, false);
      mutablePropSet.setValue(219, Some(Arc::new(false)));
      // java mutablePropSet.setValue(220, DefaultValue.paragraph_lineSpacing);
      // rust: `paragraph_lineSpacing` 在 `DefaultValue` 中是**模块级静态**
      //   （`LazyLock<Arc<Option<LineSpacing>>>`），非关联常量 ⇒ 解引用并解包。
      mutablePropSet.setValue(220, Some(Arc::new((**paragraph_lineSpacing).clone().unwrap_or_default())));
    }

}
