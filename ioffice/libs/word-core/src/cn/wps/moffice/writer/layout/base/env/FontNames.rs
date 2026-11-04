#![allow(non_snake_case)]

// Java: import cn.wps.moffice.property.PropertySet
use util::moffice::property::PropertySet::PropertySet;

/// Layout adapter: rFonts ascii/eastAsia + lang/hint resolve. No FontHost.
pub struct FontNames {}

impl FontNames {
    // java public static String getAscii(PropertySet propSet)
    pub fn getAscii(propSet: &PropertySet) -> String {
        propSet.getString__int_string(3, "Times New Roman")
    }

    // java public static String getEastAsia(PropertySet propSet)
    pub fn getEastAsia(propSet: &PropertySet) -> String {
        propSet.getString__int_string(4, "宋体")
    }

    // java public static int applyHint(PropertySet propSet, int langType)
    pub fn applyHint(propSet: &PropertySet, mut langType: i32) -> i32 {
        let hint = propSet.getInt__int_int(40, -1);
        match hint {
            1 => {
                langType = 2;
            }
            2 => {
                if langType != 6 && langType != 7 {
                    langType = 5;
                }
            }
            _ => {}
        }
        langType
    }

    // java public static String getResolvedFontName(PropertySet propSet, int langType)
    pub fn getResolvedFontName(propSet: &PropertySet, langType: i32) -> String {
        let langType = Self::applyHint(propSet, langType);
        match langType {
            2 | 3 => propSet.getString__int_string(4, "宋体"),
            4 | 7 | 8 => propSet.getString__int_string(36, "Times New Roman"),
            5 => propSet.getString__int_string(36, "Times New Roman"),
            6 => {
                if propSet.contains(36) {
                    propSet.getString(36)
                } else if propSet.contains(35) {
                    propSet.getString(35)
                } else {
                    propSet.getString__int_string(3, "Times New Roman")
                }
            }
            9 => propSet.getString__int_string(35, "Times New Roman"),
            _ => propSet.getString__int_string(3, "Times New Roman"),
        }
    }
}
