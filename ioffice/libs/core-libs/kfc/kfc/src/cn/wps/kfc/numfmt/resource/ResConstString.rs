#![allow(non_snake_case, non_camel_case_types, unused_variables, unused_imports, dead_code, unreachable_code, unused_mut)]

use std::sync::OnceLock;

// Java field: String[][] JP_ERA
static JP_ERA: OnceLock<Vec<Vec<String>>> = OnceLock::new();

pub(crate) fn get_jp_era() -> &'static Vec<Vec<String>> {
    JP_ERA.get_or_init(|| {
        vec![
            vec!["M".to_string(), "T".to_string(), "S".to_string(), "H".to_string()],
            vec!["\u{660E}".to_string(), "\u{5927}".to_string(), "\u{662D}".to_string(), "\u{5E73}".to_string()],
            vec!["\u{660E}\u{6CBB}".to_string(), "\u{5927}\u{6B63}".to_string(), "\u{662D}\u{548C}".to_string(), "\u{5E73}\u{6210}".to_string()],
        ]
    })
}

// Java field: String[] TW_ERA
static TW_ERA: OnceLock<Vec<String>> = OnceLock::new();

pub(crate) fn get_tw_era() -> &'static Vec<String> {
    TW_ERA.get_or_init(|| {
        vec!["\u{516C}\u{5143}".to_string(), "\u{6C11}\u{56FD}".to_string()]
    })
}

// Java field: String[] HEBREW_MONTH
static HEBREW_MONTH: OnceLock<Vec<String>> = OnceLock::new();

pub(crate) fn get_hebrew_month() -> &'static Vec<String> {
    HEBREW_MONTH.get_or_init(|| {
        vec![
            "Tishrei".to_string(), "Cheshvan".to_string(), "Kislev".to_string(),
            "Tevet".to_string(), "Shvat".to_string(), "AdarI".to_string(),
            "AdarII".to_string(), "Nisan".to_string(), "Iyar".to_string(),
            "Sivan".to_string(), "Tamuz".to_string(), "Av".to_string(), "Elul".to_string(),
        ]
    })
}

// Java field: String[] HEBREW_WEEK
static HEBREW_WEEK: OnceLock<Vec<String>> = OnceLock::new();

pub(crate) fn get_hebrew_week() -> &'static Vec<String> {
    HEBREW_WEEK.get_or_init(|| {
        vec![
            "Yom Rishon".to_string(), "Yom Sheni".to_string(), "Yom Shlishi".to_string(),
            "Yom Revi'i".to_string(), "Yom Chamishi".to_string(), "Yom Shishi".to_string(),
            "Shabat".to_string(),
        ]
    })
}

// Java field: String HEBREW_MONTH_ADAR
pub(crate) const HEBREW_MONTH_ADAR: &str = "Adar";

// Java: public class ResConstString
pub struct ResConstString {
}

// Java: public class ResConstString
impl ResConstString {
    // Java: ResConstString()
    pub fn new() -> Self {
        // Java: super();
        ResConstString {}
    }
}
