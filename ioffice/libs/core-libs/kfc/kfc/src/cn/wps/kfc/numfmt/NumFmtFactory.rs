#![allow(non_snake_case, non_camel_case_types, unused_variables, unused_imports, dead_code, unreachable_code, unused_mut)]

// Java: import cn.wps.kfc.numfmt.i18n.KmoLang
use crate::cn::wps::kfc::numfmt::i18n::KmoLang::KmoLang;
// Java: import cn.wps.kfc.numfmt.usermodel.NumberFormatter
use crate::cn::wps::kfc::numfmt::usermodel::NumberFormatter;
// Java: import cn.wps.kfc.numfmt.usermodel.WordFormatter
use crate::cn::wps::kfc::numfmt::usermodel::WordFormatter::WordFormatter;

// Java: public class NumFmtFactory
pub struct NumFmtFactory {
}

// Java: public class NumFmtFactory
impl NumFmtFactory {
    // Java: NumFmtFactory()
    pub fn new() -> Self {
        // Java: super();
        NumFmtFactory {}
    }

    // Java: static INumFormatter createNumFormatter(short languageType)
    pub fn createNumFormatter(language_type: i16) -> Box<dyn super::INumFormatter::INumFormatterTrait> {
        // Java: languageType = check(languageType);
        // Java: return NumberFormatter.createInstance(languageType);
        NumberFormatter::NumberFormatter::createInstance(Self::check(language_type))
    }

    // Java: static IWordFormatter createWordFormatter(short languageType)
    pub fn createWordFormatter(language_type: i16) -> WordFormatter {
        // Java: languageType = check(languageType);
        let language_type = Self::check(language_type);
        // Java: return WordFormatter.createInstance(languageType);
        WordFormatter::createInstance(language_type)
    }

    // Java: static short check(short languageType)
    pub fn check(mut language_type: i16) -> i16 {
        // Java: if (languageType == 0)
        if language_type == 0 {
            // Java: languageType = KmoLang.getSystemLCID();
            language_type = KmoLang::getSystemLCID();
        }
        // Java: return languageType;
        language_type
    }
}
