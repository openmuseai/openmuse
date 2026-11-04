#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]

// Java: import java.util.HashMap
use std::collections::HashMap;
use std::sync::OnceLock;

// Java field: int BASE_ID
const BASE_ID: i32 = 19;

// Java field: int LR_LONG_WEEK
pub(crate) const LR_LONG_WEEK: i32 = 0;
// Java field: int LR_SHORT_WEEK
pub(crate) const LR_SHORT_WEEK: i32 = 1;
// Java field: int LR_LONG_MONTH
pub(crate) const LR_LONG_MONTH: i32 = 2;
// Java field: int LR_SHORT_MONTH
pub(crate) const LR_SHORT_MONTH: i32 = 3;
// Java field: int LR_SHORTEST_MONTH
pub(crate) const LR_SHORTEST_MONTH: i32 = 4;
// Java field: int LR_AM_PM
pub(crate) const LR_AM_PM: i32 = 5;
// Java field: int LR_COLOR_NAME
pub(crate) const LR_COLOR_NAME: i32 = 6;
// Java field: int LR_F400_F800
pub(crate) const LR_F400_F800: i32 = 7;
// Java field: int ARRAY_END_INDEX
pub(crate) const ARRAY_END_INDEX: i32 = 8;
// Java field: int LR_GENERAL
pub(crate) const LR_GENERAL: i32 = 8;
// Java field: int LR_THOUS_SEP
pub(crate) const LR_THOUS_SEP: i32 = 9;
// Java field: int LR_DECIMAL_POINT
pub(crate) const LR_DECIMAL_POINT: i32 = 10;
// Java field: int LR_YEAR
pub(crate) const LR_YEAR: i32 = 11;
// Java field: int LR_MONTH
pub(crate) const LR_MONTH: i32 = 12;
// Java field: int LR_DAY
pub(crate) const LR_DAY: i32 = 13;
// Java field: int LR_HOUR
pub(crate) const LR_HOUR: i32 = 14;
// Java field: int LR_MINUTE
pub(crate) const LR_MINUTE: i32 = 15;
// Java field: int LR_SECOND
pub(crate) const LR_SECOND: i32 = 16;
// Java field: int LR_ESCAPE
pub(crate) const LR_ESCAPE: i32 = 17;
// Java field: int LR_SLASH
pub(crate) const LR_SLASH: i32 = 18;
// Java field: HashMap<Integer, String> mIDMaps
static mIDMaps: OnceLock<HashMap<i32, &str>> = OnceLock::new();

fn get_id_maps() -> &'static HashMap<i32, &'static str> {
    mIDMaps.get_or_init(|| {
        let mut m = HashMap::with_capacity(19);
        m.insert(LR_LONG_WEEK, "long-week");
        m.insert(LR_SHORT_WEEK, "short-week");
        m.insert(LR_LONG_MONTH, "long-month");
        m.insert(LR_SHORT_MONTH, "short-month");
        m.insert(LR_SHORTEST_MONTH, "shortest-month");
        m.insert(LR_AM_PM, "am-pm");
        m.insert(LR_COLOR_NAME, "color-name");
        m.insert(LR_F400_F800, "f400-f800");
        m.insert(LR_GENERAL, "general");
        m.insert(LR_THOUS_SEP, "thous-separator");
        m.insert(LR_DECIMAL_POINT, "decimal-point");
        m.insert(LR_YEAR, "year");
        m.insert(LR_MONTH, "month");
        m.insert(LR_DAY, "day");
        m.insert(LR_HOUR, "hour");
        m.insert(LR_MINUTE, "minute");
        m.insert(LR_SECOND, "second");
        m.insert(LR_ESCAPE, "escape");
        m.insert(LR_SLASH, "date-slash");
        m
    })
}

// Java: public class LocaleResID
pub struct LocaleResID {
}

// Java: public class LocaleResID
impl LocaleResID {
    // Java: LocaleResID()
    pub fn new() -> Self {
      // Java: super();
      LocaleResID {}
    }

    // Java: static int getIDCount()
    pub fn getIDCount() -> i32 {
      // Java: return BASE_ID;
      BASE_ID
    }

    // Java: static String getName(int id)
    pub fn getName(id: i32) -> Option<&'static str> {
      // Java: return (String)mIDMaps.get(id);
      get_id_maps().get(&id).copied()
    }

    // Java: static boolean isArray(int id)
    pub fn isArray(id: i32) -> bool {
      // Java: return id < ARRAY_END_INDEX;
      id < ARRAY_END_INDEX
    }

}
