#![allow(non_snake_case, non_camel_case_types, unused_variables, unused_imports, dead_code, unreachable_code, unused_mut)]

// Java: import cn.wps.kfc.numfmt.resource.ResConstString
use crate::cn::wps::kfc::numfmt::resource::ResConstString;
use super::CalendarCache::CalendarCache;
use super::TimeCalc::TimeStruct;

use std::sync::{Mutex, OnceLock};

// Java field: long HOUR_PARTS
pub(crate) const HOUR_PARTS: i64 = 1080;
// Java field: long DAY_PARTS
pub(crate) const DAY_PARTS: i64 = 25920;
// Java field: int TISHRI
pub const TISHRI: i32 = 0;
// Java field: int CHESHVAN
pub const CHESHVAN: i32 = 1;
// Java field: int KISLEV
pub const KISLEV: i32 = 2;
// Java field: int TEVET
pub const TEVET: i32 = 3;
// Java field: int SHEVAT
pub const SHEVAT: i32 = 4;
// Java field: int ADAR_1
pub const ADAR_1: i32 = 5;
// Java field: int ADAR
pub const ADAR: i32 = 6;
// Java field: int NISAN
pub const NISAN: i32 = 7;
// Java field: int IYAR
pub const IYAR: i32 = 8;
// Java field: int SIVAN
pub const SIVAN: i32 = 9;
// Java field: int TAMUZ
pub const TAMUZ: i32 = 10;
// Java field: int AV
pub const AV: i32 = 11;
// Java field: int ELUL
pub const ELUL: i32 = 12;
// Java field: int MONTH_DAYS
pub(crate) const MONTH_DAYS: i32 = 29;
// Java field: long MONTH_FRACT
pub(crate) const MONTH_FRACT: i64 = 13753;
// Java field: long MONTH_PARTS
pub(crate) const MONTH_PARTS: i64 = 765433;
// Java field: long BAHARAD
pub(crate) const BAHARAD: i64 = 12084;
// Java field: CalendarCache cache
static CACHE: OnceLock<Mutex<CalendarCache>> = OnceLock::new();
// Java field: int[][] MONTH_START
static MONTH_START: OnceLock<Vec<Vec<i32>>> = OnceLock::new();
// Java field: int[][] LEAP_MONTH_START
static LEAP_MONTH_START: OnceLock<Vec<Vec<i32>>> = OnceLock::new();

// Java: public class HebrewCalc
pub struct HebrewCalc;

// Java: public class HebrewCalc
impl HebrewCalc {
    // Java: HebrewCalc()
    pub fn new() -> Self {
        // Java: super();
        HebrewCalc
    }

    // Java: static void greg2HebrewTime(TimeCalc.TimeStruct src, TimeCalc.TimeStruct tm)
    pub fn greg2HebrewTime(_tm: &mut TimeStruct) {
        // Java: double julianDay = HiJriCalc.julianDays(src.mYear, src.mMonth, src.mMonthDay, src.mHour, src.mMinute, src.mSecond);
        // Java: double d = (double)Math.round(julianDay - (double)347997.0F);
        // Java: double m = d * (double)25920.0F / (double)765433.0F;
        // Java: int year = (int)(((double)19.0F * m + (double)234.0F) / (double)235.0F) + 1;
        // Java: long ys = startOfYear(year);
        // Java: int dayOfYear;
        // Java: for(dayOfYear = (int)(d - (double)ys); dayOfYear < 1; dayOfYear = (int)(d - (double)ys))
        {
            // Java: --year;
            // Java: ys = startOfYear(year);
        }
        // Java: int yearType = yearType(year);
        // Java: int[][] monthStart = isLeapYear(year) ? LEAP_MONTH_START : MONTH_START;
        // Java: int month;
        // Java: for(month = 0; dayOfYear > monthStart[month][yearType]; ++month)
        {
        }
        // Java: --month;
        // Java: int dayOfMonth = dayOfYear - monthStart[month][yearType];
        // Java: tm.mYear = year;
        // Java: tm.mMonth = base1Month(month, year);
        // Java: tm.mMonthDay = dayOfMonth;
        // Java: tm.mYearDay = dayOfYear;
        // Java: tm.mHour = src.mHour;
        // Java: tm.mMinute = src.mMinute;
        // Java: tm.mSecond = src.mSecond;
        todo!("HebrewCalc::greg2HebrewTime")
    }

    // Java: static String getHebrewMonth(int year, int month)
    pub fn getHebrewMonth(_year: i32, _month: i32) -> String {
        // Java: boolean leap = isLeapYear(year);
        // Java: if (month <= 5 || leap)
        {
            // Java: --month;
        }
        // Java: String strMonth = ResConstString.HEBREW_MONTH[month];
        // Java: if (!leap && month == 6)
        {
            // Java: strMonth = "Adar";
        }
        // Java: return strMonth;
        todo!("HebrewCalc::getHebrewMonth")
    }

    // Java: static String getHebrewWeek(int weekIndex)
    pub fn getHebrewWeek(_week_index: i32) -> String {
        // Java: return ResConstString.HEBREW_WEEK[weekIndex];
        todo!("HebrewCalc::getHebrewWeek")
    }

    // Java: static int base1Month(int month, int year)
    fn base1Month(_month: i32, _year: i32) -> i32 {
        // Java: if (month <= 5 || isLeapYear(year))
        {
            // Java: ++month;
        }
        // Java: return month;
        todo!("HebrewCalc::base1Month")
    }

    // Java: static long startOfYear(int year)
    fn startOfYear(_year: i32) -> i64 {
        // Java: long day = cache.get((long)year);
        // Java: if (day == CalendarCache.EMPTY)
        {
            // Java: int months = (235 * year - 234) / 19;
            // Java: long frac = (long)months * 13753L + 12084L;
            // Java: day = (long)(months * 29) + frac / 25920L;
            // Java: frac %= 25920L;
            // Java: int wd = (int)(day % 7L);
            // Java: if (wd == 2 || wd == 4 || wd == 6)
            {
                // Java: ++day;
                // Java: wd = (int)(day % 7L);
            }
            // Java: if (wd == 1 && frac > 16404L && !isLeapYear(year))
            {
                // Java: day += 2L;
            }
            // Java: else if (wd == 0 && frac > 23269L && isLeapYear(year - 1))
            {
                // Java: ++day;
            }
            // Java: cache.put((long)year, day);
        }
        // Java: return day;
        todo!("HebrewCalc::startOfYear")
    }

    // Java: static int yearType(int year)
    fn yearType(_year: i32) -> i32 {
        // Java: int yearLength = (int)(startOfYear(year + 1) - startOfYear(year));
        // Java: if (yearLength > 380)
        {
            // Java: yearLength -= 30;
        }
        // Java: int type = 0;
        // Java: switch (yearLength)
        {
            // Java: case 353:
                // Java: type = 0;
                // Java: break;
            // Java: case 354:
                // Java: type = 1;
                // Java: break;
            // Java: case 355:
                // Java: type = 2;
                // Java: break;
            // Java: default:
                // Java: throw new IllegalArgumentException("Illegal year length " + yearLength + " in year " + year);
        }
        // Java: return type;
        todo!("HebrewCalc::yearType")
    }

    // Java: static boolean isLeapYear(int year)
    fn isLeapYear(_year: i32) -> bool {
        // Java: int x = (year * 12 + 17) % 19;
        // Java: return x >= (x < 0 ? -7 : 12);
        todo!("HebrewCalc::isLeapYear")
    }
}
