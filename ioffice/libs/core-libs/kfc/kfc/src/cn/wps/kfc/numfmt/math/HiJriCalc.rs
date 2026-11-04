#![allow(non_snake_case, non_camel_case_types, unused_variables, unused_imports, dead_code, unreachable_code, unused_mut)]

use super::TimeCalc::TimeStruct;

// Java: public class HiJriCalc
pub struct HiJriCalc;

// Java: public class HiJriCalc
impl HiJriCalc {
    // Java: HiJriCalc()
    pub fn new() -> Self {
        // Java: super();
        HiJriCalc
    }

    // Java: static void greg2HiJri(TimeCalc.TimeStruct greg, TimeCalc.TimeStruct hiJri)
    pub fn greg2HiJri(_tm: &mut TimeStruct) {
        // Java: double julianDays = julianDays(greg.mYear, greg.mMonth, greg.mMonthDay, greg.mHour, greg.mMinute, greg.mSecond);
        // Java: double sumDays = Math.floor(julianDays - (double)1948439.5F);
        // Java: double weekOffset = Math.floor((sumDays + 0.1) / (double)10631.0F);
        // Java: double dayWOffset = sumDays - weekOffset * (double)10631.0F;
        // Java: double dayYOffset = Math.floor((dayWOffset + (double)0.5F) / 354.366);
        // Java: double dayMOffset = dayWOffset - Math.floor(dayYOffset * 354.366 + (double)0.5F);
        // Java: int m = (int)Math.floor((dayMOffset + 0.11) / 29.51);
        // Java: int d = (int)(dayMOffset - Math.floor(29.5001 * (double)m + (double)0.5F));
        // Java: int hy0 = (int)(dayYOffset + (double)1.0F + weekOffset * (double)30.0F);
        // Java: hiJri.mYear = hy0;
        // Java: if (hy0 <= 0)
        {
            // Java: hiJri.mYear = -hy0 + 1;
        }
        // Java: hiJri.mMonth = m + 1;
        // Java: hiJri.mMonthDay = d + 1;
        // Java: hiJri.mHour = greg.mHour;
        // Java: hiJri.mMinute = greg.mMinute;
        // Java: hiJri.mSecond = greg.mSecond;
        todo!("HiJriCalc::greg2HiJri")
    }

    // Java: static double julianDays(int Y, int M, int D, int h, int m, int s)
    pub fn julianDays(_y: i32, _m: i32, _d: i32, _h: i32, _min: i32, _s: i32) -> f64 {
        // Java: int year = Y;
        // Java: int month = M;
        // Java: double days = (double)0.0F;
        // Java: if (M <= 2)
        {
            // Java: month = M + 12;
            // Java: year = Y - 1;
        }
        // Java: if (Y * 372 + M * 31 + D >= 588829)
        {
            // Java: days = Math.floor((double)(year / 100));
            // Java: days = (double)2.0F - days + days / (double)4.0F;
        }
        // Java: days += Math.floor((double)365.25F * (double)(year + 4716));
        // Java: days += Math.floor(30.6001 * (double)(month + 1)) + (double)D;
        // Java: days += (double)(((s / 60 + m) / 60 + h) / 24) - (double)1524.5F;
        // Java: return days;
        todo!("HiJriCalc::julianDays")
    }

    // Java: static int hiJriYear2GregYear(int hiJriYear)
    pub fn hiJriYear2GregYear(_hi_jri_year: i32) -> i32 {
        // Java: return (int)(0.970223 * (double)(hiJriYear - 1) + 622.5479);
        todo!("HiJriCalc::hiJriYear2GregYear")
    }
}
