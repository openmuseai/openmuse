#![allow(non_snake_case, non_camel_case_types, unused_variables, unused_imports, dead_code, unreachable_code, unused_mut)]

use super::TimeCalc::TimeStruct;
use std::sync::OnceLock;

// Java field: int[] lunarInfo
static LUNAR_INFO: OnceLock<Vec<i32>> = OnceLock::new();

// Java: public class LunarCalc
pub struct LunarCalc;

// Java: public class LunarCalc
impl LunarCalc {
    // Java: LunarCalc()
    pub fn new() -> Self {
        // Java: super();
        LunarCalc
    }

    // Java: void greg2lunar(TimeCalc.TimeStruct greg, TimeCalc.TimeStruct lunar)
    pub fn greg2lunar(&mut self, _tm: &mut TimeStruct) {
        // Java: int year = greg.mYear;
        // Java: if (year >= 1900 && year < 10000)
        {
            // Java: int month = greg.mMonth;
            // Java: int date = greg.mMonthDay;
            // Java: long[] l = this.calElement(year, month, date);
            // Java: lunar.mYear = (int)l[0];
            // Java: lunar.mMonth = (int)l[1];
            // Java: lunar.mMonthDay = (int)l[2];
            // Java: lunar.mHour = greg.mHour;
            // Java: lunar.mMinute = greg.mMinute;
            // Java: lunar.mSecond = greg.mSecond;
        }
        // Java: else
        {
            // Java: lunar.mYear = greg.mYear;
            // Java: lunar.mMonth = greg.mMonth;
            // Java: lunar.mMonthDay = greg.mMonthDay;
            // Java: lunar.mHour = greg.mHour;
            // Java: lunar.mMinute = greg.mMinute;
            // Java: lunar.mSecond = greg.mSecond;
        }
        todo!("LunarCalc::greg2lunar")
    }

    // Java: int lYearDays(int y)
    fn lYearDays(&self, _y: i32) -> i32 {
        // Java: int sum = 348;
        // Java: for(int i = 32768; i > 8; i >>= 1)
        {
            // Java: if ((lunarInfo[y - 1900] & i) != 0)
            {
                // Java: ++sum;
            }
        }
        // Java: return sum + this.leapDays(y);
        todo!("LunarCalc::lYearDays")
    }

    // Java: int leapDays(int y)
    fn leapDays(&self, _y: i32) -> i32 {
        // Java: if (this.leapMonth(y) != 0)
        {
            // Java: return (lunarInfo[y - 1900] & 65536) != 0 ? 30 : 29;
        }
        // Java: else
        {
            // Java: return 0;
        }
        todo!("LunarCalc::leapDays")
    }

    // Java: int leapMonth(int y)
    fn leapMonth(&self, _y: i32) -> i32 {
        // Java: return lunarInfo[y - 1900] & 15;
        todo!("LunarCalc::leapMonth")
    }

    // Java: int monthDays(int y, int m)
    fn monthDays(&self, _y: i32, _m: i32) -> i32 {
        // Java: return (lunarInfo[y - 1900] & 65536 >> m) == 0 ? 29 : 30;
        todo!("LunarCalc::monthDays")
    }

    // Java: long[] calElement(int y, int m, int d)
    fn calElement(&self, _y: i32, _m: i32, _d: i32) -> Vec<i64> {
        // Java: long[] nongDate = new long[7];
        // Java: int year = 0;
        // Java: int month = 0;
        // Java: int dayOfYear = 0;
        // Java: int dayOfMonth = 0;
        // Java: int leap = 0;
        // Java: TimeZone.getDefault();
        // Java: TimeZone.setDefault(TimeZone.getTimeZone("GMT+8"));
        // Java: Date baseDate = (new GregorianCalendar(1900, 0, 31)).getTime();
        // Java: Date objDate = (new GregorianCalendar(y, m - 1, d)).getTime();
        // Java: long offset = (objDate.getTime() - baseDate.getTime()) / 86400000L;
        // Java: if (offset < 0L && offset > -30L)
        {
            // Java: nongDate[0] = (long)(y - 1);
            // Java: nongDate[1] = 12L;
            // Java: nongDate[2] = (long)d;
            // Java: return nongDate;
        }
        // Java: else
        {
            // Java: nongDate[5] = offset + 40L;
            // Java: nongDate[4] = 14L;
            // Java: for(year = 1900; year < 10000 && offset > 0L; ++year)
            {
                // Java: dayOfYear = this.lYearDays(year);
                // Java: offset -= (long)dayOfYear;
                // Java: nongDate[4] += 12L;
            }
            // Java: if (offset < 0L)
            {
                // Java: offset += (long)dayOfYear;
                // Java: --year;
                // Java: nongDate[4] -= 12L;
            }
            // Java: nongDate[0] = (long)year;
            // Java: nongDate[3] = (long)(year - 1864);
            // Java: leap = this.leapMonth(year);
            // Java: nongDate[6] = 0L;
            // Java: boolean afterLeap = false;
            // Java: for(month = 1; month < 13 && offset > 0L; ++month)
            {
                // Java: if (leap > 0 && month == leap + 1 && nongDate[6] == 0L)
                {
                    // Java: afterLeap = true;
                    // Java: --month;
                    // Java: nongDate[6] = 1L;
                    // Java: dayOfMonth = this.leapDays((int)nongDate[0]);
                }
                // Java: else
                {
                    // Java: dayOfMonth = this.monthDays((int)nongDate[0], month);
                }
                // Java: if (nongDate[6] == 1L && month == leap + 1)
                {
                    // Java: nongDate[6] = 0L;
                }
                // Java: offset -= (long)dayOfMonth;
                // Java: if (nongDate[6] == 0L)
                {
                    // Java: nongDate[4]++;
                }
            }
            // Java: if (offset == 0L && leap > 0 && month == leap + 1)
            {
                // Java: afterLeap = true;
                // Java: if (nongDate[6] == 1L)
                {
                    // Java: nongDate[6] = 0L;
                }
                // Java: else
                {
                    // Java: nongDate[6] = 1L;
                    // Java: --month;
                    // Java: nongDate[4]--;
                }
            }
            // Java: if (offset < 0L)
            {
                // Java: offset += (long)dayOfMonth;
                // Java: --month;
                // Java: nongDate[4]--;
            }
            // Java: nongDate[1] = afterLeap ? (long)(month + 1) : (long)month;
            // Java: nongDate[2] = offset + 1L;
            // Java: return nongDate;
        }
        todo!("LunarCalc::calElement")
    }
}
