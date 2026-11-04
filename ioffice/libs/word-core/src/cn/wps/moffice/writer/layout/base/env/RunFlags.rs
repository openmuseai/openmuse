#![allow(non_snake_case)]

// Java: import cn.wps.moffice.property.PropertySet
use util::moffice::property::PropertySet::PropertySet;

use super::FontSize::FontSize;

/// Layout adapter: CHP bold/italic flags. Not a full FontStyleFactory port.
pub struct RunFlags {}

impl RunFlags {
    // java public static boolean getBold(PropertySet propSet, int langType)
    pub fn getBold(propSet: &PropertySet, langType: i32) -> bool {
        if FontSize::isComplexScript(langType) {
            propSet.getBoolean__int_boolean(38, false)
        } else {
            propSet.getBoolean__int_boolean(5, false)
        }
    }

    // java public static boolean getItalic(PropertySet propSet, int langType)
    pub fn getItalic(propSet: &PropertySet, langType: i32) -> bool {
        if FontSize::isComplexScript(langType) {
            propSet.getBoolean__int_boolean(39, false)
        } else {
            propSet.getBoolean__int_boolean(6, false)
        }
    }
}
