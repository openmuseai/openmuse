#![allow(non_snake_case)]

// Java: import cn.wps.moffice.property.PropertySet
use util::moffice::property::PropertySet::PropertySet;

use super::FontStyleFactory::FontStyleFactory;

/// wr-10 I/O facade over `FontStyleFactory::getFontSize` (Java twin).
pub struct FontSize {}

impl FontSize {
    pub fn setMaxFontSize(size: i32) {
        FontStyleFactory::setMaxFontSize(size);
    }

    pub fn maxFontSize() -> i32 {
        FontStyleFactory::maxFontSize()
    }

    pub fn isComplexScript(langType: i32) -> bool {
        FontStyleFactory::isComplexScript(langType)
    }

    pub fn getFontSize(propSet: &PropertySet, langType: i32) -> i32 {
        FontStyleFactory::getFontSize(propSet, langType)
    }

    pub fn getFontSizeLimited(propSet: &PropertySet, langType: i32, maxFontSize: i32) -> i32 {
        let prev = Self::maxFontSize();
        FontStyleFactory::setMaxFontSize(maxFontSize);
        let n = FontStyleFactory::getFontSize(propSet, langType);
        FontStyleFactory::setMaxFontSize(prev);
        n
    }
}
