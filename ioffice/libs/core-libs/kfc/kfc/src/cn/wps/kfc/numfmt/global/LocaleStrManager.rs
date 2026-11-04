#![allow(non_snake_case, non_camel_case_types, unused_variables, unused_imports, dead_code, unused_assignments)]

use crate::cn::wps::kfc::numfmt::resource::LocaleResAccessor;
use crate::cn::wps::kfc::numfmt::builtin::BuiltinColors::BuiltinColors;
use crate::cn::wps::kfc::numfmt::resource::ResourceLoader;
use crate::cn::wps::kfc::numfmt::global::LocaleResID;

// Java: public class LocaleStrManager
pub struct LocaleStrManager {
    // Java field: Properties mNumeralShape
    mNumeralShape: Option<std::collections::HashMap<String,Vec<String>>>,
    // Java field: LocaleResAccessor.LocaleResCache mDefaultLocaleRes
    mDefaultLocaleRes: Option<std::sync::Arc<LocaleResAccessor::LocaleResCache>>,
}

impl LocaleStrManager {
    // Java: LocaleStrManager()
    pub fn new() -> Self {
        // Java: super();
        LocaleStrManager { mNumeralShape: None, mDefaultLocaleRes: None }
    }

    // Java: void destroy()
    pub fn destroy(&mut self) {
        // Java: this.mNumeralShape = null;
        self.mNumeralShape = None;
        // Java: this.mDefaultLocaleRes = null;
        self.mDefaultLocaleRes = None;
    }

    // Java: void loadDefaultRes(short lcid)
    pub fn loadDefaultRes(&mut self, lcid: i16) {
        // Java: this.mDefaultLocaleRes = LocaleResAccessor.getLocaleResource(lcid);
        self.mDefaultLocaleRes = Some(LocaleResAccessor::LocaleResAccessor::getLocaleResource(lcid));
        // Java: BuiltinColors.loadColorNames(this.mDefaultLocaleRes.getItem(LocaleResID.LR_COLOR_NAME));
        BuiltinColors::loadColorNames(self.mDefaultLocaleRes.as_ref().unwrap().getItem(LocaleResID::LR_COLOR_NAME).mContent.as_deref());
    }

    // Java: Properties getNumShapeCache()
    pub fn getNumShapeCache(&mut self) -> std::collections::HashMap<String,Vec<String>> {
        // Java: if (this.mNumeralShape == null)
        if self.mNumeralShape.is_none() {
            // Java: this.mNumeralShape = ResourceLoader.getSaxParserCache("numeral_shape");
            self.mNumeralShape = ResourceLoader::ResourceLoader::getSaxParserCache("numeral_shape");
        }
        // Java: return this.mNumeralShape;
        self.mNumeralShape.clone().expect("numeral shape resource")
    }

    // Java: String getDecimalPoint()
    pub fn getDecimalPoint(&self) -> String {
        // Java: return this.mDefaultLocaleRes.getString(LocaleResID.LR_DECIMAL_POINT);
        self.mDefaultLocaleRes.as_ref().expect("default locale").getString(LocaleResID::LR_DECIMAL_POINT)
    }

    // Java: String getThousSepator()
    pub fn getThousSepator(&self) -> String {
        // Java: return this.mDefaultLocaleRes.getString(LocaleResID.LR_THOUS_SEP);
        self.mDefaultLocaleRes.as_ref().expect("default locale").getString(LocaleResID::LR_THOUS_SEP)
    }

    // Java: String getGeneral()
    pub fn getGeneral(&self) -> String {
        // Java: return this.mDefaultLocaleRes.getString(LocaleResID.LR_GENERAL);
        self.mDefaultLocaleRes.as_ref().expect("default locale").getString(LocaleResID::LR_GENERAL)
    }

    // Java: String getYear()
    pub fn getYear(&self) -> String {
        // Java: return this.mDefaultLocaleRes.getString(LocaleResID.LR_YEAR);
        self.mDefaultLocaleRes.as_ref().expect("default locale").getString(LocaleResID::LR_YEAR)
    }

    // Java: String getMonth()
    pub fn getMonth(&self) -> String {
        // Java: return this.mDefaultLocaleRes.getString(LocaleResID.LR_MONTH);
        self.mDefaultLocaleRes.as_ref().expect("default locale").getString(LocaleResID::LR_MONTH)
    }

    // Java: String getDay()
    pub fn getDay(&self) -> String {
        // Java: return this.mDefaultLocaleRes.getString(LocaleResID.LR_DAY);
        self.mDefaultLocaleRes.as_ref().expect("default locale").getString(LocaleResID::LR_DAY)
    }

    // Java: String getHour()
    pub fn getHour(&self) -> String {
        // Java: return this.mDefaultLocaleRes.getString(LocaleResID.LR_HOUR);
        self.mDefaultLocaleRes.as_ref().expect("default locale").getString(LocaleResID::LR_HOUR)
    }

    // Java: String getMinute()
    pub fn getMinute(&self) -> String {
        // Java: return this.mDefaultLocaleRes.getString(LocaleResID.LR_MINUTE);
        self.mDefaultLocaleRes.as_ref().expect("default locale").getString(LocaleResID::LR_MINUTE)
    }

    // Java: String getSecond()
    pub fn getSecond(&self) -> String {
        // Java: return this.mDefaultLocaleRes.getString(LocaleResID.LR_SECOND);
        self.mDefaultLocaleRes.as_ref().expect("default locale").getString(LocaleResID::LR_SECOND)
    }

    // Java: String getEscape()
    pub fn getEscape(&self) -> String {
        // Java: return this.mDefaultLocaleRes.getString(LocaleResID.LR_ESCAPE);
        self.mDefaultLocaleRes.as_ref().expect("default locale").getString(LocaleResID::LR_ESCAPE)
    }

    // Java: String getSlash()
    pub fn getSlash(&self) -> String {
        // Java: return this.mDefaultLocaleRes.getString(LocaleResID.LR_SLASH);
        self.mDefaultLocaleRes.as_ref().expect("default locale").getString(LocaleResID::LR_SLASH)
    }

    // Java: String[] getF4800()
    pub fn getF4800(&self) -> Vec<String> {
        // Java: return this.mDefaultLocaleRes.getArray(LocaleResID.LR_F400_F800);
        self.mDefaultLocaleRes.as_ref().expect("default locale").getArray(LocaleResID::LR_F400_F800)
    }

    // Java: void replaceDecPointLocal(StringBuffer result, String src)
    pub fn replaceDecPointLocal(&self, result: &mut String, src: &str) {
        // Java: char localDecPoint = this.getDecimalPoint().charAt(0);
        let localDecPoint = self.getDecimalPoint().chars().next().unwrap_or('.');
        // Java: char innerDecPoint = '.';
        let innerDecPoint = '.';
        // Java: int srcLength = src.length();
        let srcLength = src.len();
        // Java: int index = 0;
        let mut index: i32 = 0;
        // Java: if (localDecPoint != '.' && (index = src.indexOf(46)) > -1)
        index = src.find('.').map(|i| i as i32).unwrap_or(-1);
        if localDecPoint != '.' && index > -1 {
            {
                // Java: int cursor;
                let mut cursor: usize = 0;
                // Java: for(cursor = 0; index > -1; index = src.indexOf(46, cursor))
                while index > -1 {
                    // Java: result.append(src, cursor, index);
                    result.push_str(&src[cursor..index as usize]);
                    // Java: result.append(localDecPoint);
                    result.push(localDecPoint);
                    // Java: cursor = index + 1;
                    cursor = (index + 1) as usize;
                    index = src[cursor..].find('.').map(|i| (i + cursor) as i32).unwrap_or(-1);
                }
                // Java: if (cursor < srcLength)
                if cursor < srcLength {
                    // Java: result.append(src, cursor, srcLength);
                    result.push_str(&src[cursor..srcLength]);
                }
            }
        }
        // Java: else
        else {
            // Java: result.append(src);
            result.push_str(src);
        }
    }
}
