// Java: import cn.wps.moffice.property.MutablePropertySet
use util::moffice::property::MutablePropertySet::*;
// Java: import cn.wps.moffice.property.PropertySet
use util::moffice::property::PropertySet::*;
// Java: import cn.wps.moffice.writer.data.value.PageSetup
use crate::cn::wps::moffice::writer::data::value::PageSetup::*;
// Java: import cn.wps.moffice.writer.data.PropertyId
use crate::cn::wps::moffice::writer::data::PropertyId::*;
use std::sync::Arc;

// java public static PropertySet properties;
// rust: ★Java 静态字段 + `if (null == properties)` 惰性初始化 ⇒ `OnceLock<PropertySet>`。
//   原模板的 `OnceLock<Arc<Option<PropertySet>>>` 是双重包裹，既无法表达 Java 的 null 判定，
//   也无法表达 `properties = mutablePropSet.freeze();` 的赋值。
pub(crate) static properties: std::sync::OnceLock<PropertySet> = std::sync::OnceLock::new();

// java public class DefaultSectionProperties implements PropertyId
#[derive(Default)]
pub struct DefaultSectionProperties {
}

// java public interface PropertyId
impl PropertyId for DefaultSectionProperties {
}

// java public class DefaultSectionProperties implements PropertyId
impl DefaultSectionProperties {
    // java public static PropertySet getProperties()
    pub fn getProperties() -> PropertySet {
      // java if (null == properties)
      if properties.get().is_none() {
        // java MutablePropertySet mutablePropSet = new MutablePropertySet();
        let mut mutablePropSet: MutablePropertySet = MutablePropertySet::new();
        // java initProperties(mutablePropSet);
        Self::initProperties(&mut mutablePropSet);
        // java properties = mutablePropSet.freeze();
        let _ = properties.set(mutablePropSet.freeze());
      }
      // java return properties;
      return properties.get().unwrap().clone();
    }

    // java private static void initProperties(MutablePropertySet mutablePropSet)
    pub fn initProperties(mutablePropSet: &mut MutablePropertySet) {
      // java mutablePropSet.setValue(676, new PageSetup(11906, 16838, 1800, 1800, 1440, 1440));
      mutablePropSet.setValue(
        676,
        Some(Arc::new(PageSetup::PageSetup(11906, 16838, 1800, 1800, 1440, 1440))),
      );
      // java mutablePropSet.setInt(678, 312);
      mutablePropSet.setInt(678, 312);
      // java mutablePropSet.setFloat(679, 10.5F);
      mutablePropSet.setFloat(679, 10.5);
      // java mutablePropSet.setInt(699, 1);
      mutablePropSet.setInt(699, 1);
      // java mutablePropSet.setInt(677, 2);
      mutablePropSet.setInt(677, 2);
    }

}
