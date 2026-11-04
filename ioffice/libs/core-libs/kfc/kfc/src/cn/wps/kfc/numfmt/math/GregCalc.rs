// Java field: double MAX_DATE
pub(crate) const MAX_DATE: f64 = 2958465.999988426;
// Java field: double MIN_DATE
pub(crate) const MIN_DATE: f64 = 0.0;
// Java field: double INVALID_DATE
pub(crate) const INVALID_DATE: f64 = -1.0;
// Java field: boolean EXCEL_1900
pub(crate) const EXCEL_1900: bool = true;
// Java field: long INTERVAL_1904
pub(crate) const INTERVAL_1904: i64 = 1462;
// Java field: double HALF_SECOND
pub(crate) const HALF_SECOND: f64 = 5.787037037037037E-6;
// Java field: int[] afxMonthDays
pub(crate) static afxMonthDays: [i32; 13] = [0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334, 365];

// Java: public class GregCalc
pub struct GregCalc {
}

// Java: public class GregCalc
impl GregCalc {
    // Java: GregCalc()
    pub fn new() -> Self {
      // Java: super();
      GregCalc {}
    }

    // Java: static boolean double2GregTimeExcel(double dtSrc, TimeCalc.TimeStruct tmDest, boolean b1904)
    pub fn double2GregTimeExcel(dtSrc: f64, tmDest: &mut super::TimeCalc::TimeStruct, b1904: bool) -> bool {
      // Java: double src = dtSrc;
      let mut src = dtSrc;
        // Java: if (!b1904 && dtSrc >= (double)1.0F && dtSrc < (double)60.0F)
      if !b1904 && dtSrc >= 1.0 && dtSrc < 60.0 {
         // Java: src = dtSrc + (double)1.0F;
         src = dtSrc + 1.0;
      }
      // Java: boolean ok = double2GregTime(src, tmDest, b1904);
      let ok = Self::double2GregTime(src, tmDest, b1904);
        // Java: if (ok && !b1904)
      if ok && !b1904 {
            // Java: if (!b1904 && (long)dtSrc == 0L)
         if !b1904 && (dtSrc as i64) == 0 {
            // Java: tmDest.mYear = 1900;
            tmDest.mYear = 1900;
            // Java: tmDest.mYearDay = 0;
            tmDest.mYearDay = 0;
            // Java: tmDest.mMonth = 1;
            tmDest.mMonth = 1;
            // Java: tmDest.mMonthDay = 0;
            tmDest.mMonthDay = 0;
            // Java: tmDest.mWeekDay = 6;
            tmDest.mWeekDay = 6;
         }
            // Java: if (dtSrc >= (double)60.0F && dtSrc < (double)61.0F)
         if dtSrc >= 60.0 && dtSrc < 61.0 {
            // Java: ++tmDest.mMonthDay;
            tmDest.mMonthDay += 1;
         }
            // Java: if (dtSrc >= (double)1.0F && dtSrc < (double)60.0F)
         if dtSrc >= 1.0 && dtSrc < 60.0 {
            // Java: tmDest.mWeekDay = tmDest.mWeekDay > 0 ? tmDest.mWeekDay - 1 : 6;
            tmDest.mWeekDay = if tmDest.mWeekDay > 0 { tmDest.mWeekDay - 1 } else { 6 };
         }
         // Java: return ok;
         return ok;
      }
        // Java: else
      else {
         // Java: return ok;
         return ok;
      }
    }

    // Java: static boolean double2GregTime(double dtSrc, TimeCalc.TimeStruct tmDest, boolean use1904)
    pub fn double2GregTime(dtSrc: f64, tmDest: &mut super::TimeCalc::TimeStruct, use1904: bool) -> bool {
      // Java: long add1904 = use1904 ? 1462L : 0L;
      let add1904: i64 = if use1904 { 1462 } else { 0 };
        // Java: if (!((double)((long)dtSrc + add1904) > 2958465.999988426) && !(dtSrc < (double)0.0F))
      if !((dtSrc as i64 + add1904) as f64 > MAX_DATE) && !(dtSrc < 0.0) {
         // Java: boolean leap4 = true;
         let mut leap4 = true;
         // Java: double dblDate = dtSrc + (dtSrc > (double)0.0F ? 5.787037037037037E-6 : -5.787037037037037E-6);
         let mut dblDate: f64 = dtSrc + if dtSrc > 0.0 { HALF_SECOND } else { -HALF_SECOND };
         // Java: long daysAbsolute = (long)dblDate + 693959L + add1904;
         let mut daysAbsolute: i64 = dblDate as i64 + 693959 + add1904;
         // Java: dblDate = Math.abs(dblDate);
         dblDate = dblDate.abs();
         // Java: long secsInDay = (long)((dblDate - Math.floor(dblDate)) * (double)86400.0F);
         let secsInDay: i64 = ((dblDate - dblDate.floor()) * 86400.0) as i64;
         // Java: tmDest.mWeekDay = (int)((daysAbsolute - 1L) % 7L);
         tmDest.mWeekDay = ((daysAbsolute - 1) % 7) as i32;
         // Java: long n400Years = daysAbsolute / 146097L;
         let n400Years: i64 = daysAbsolute / 146097;
         // Java: daysAbsolute %= 146097L;
         daysAbsolute %= 146097;
         // Java: long n400Century = (daysAbsolute - 1L) / 36524L;
         let n400Century: i64 = (daysAbsolute - 1) / 36524;
         // Java: long n4Years;
         let n4Years: i64;
         // Java: long n4Day;
         let mut n4Day: i64;
            // Java: if (n400Century != 0L)
         if n400Century != 0 {
            // Java: daysAbsolute = (daysAbsolute - 1L) % 36524L;
            daysAbsolute = (daysAbsolute - 1) % 36524;
            // Java: n4Years = (daysAbsolute + 1L) / 1461L;
            n4Years = (daysAbsolute + 1) / 1461;
                // Java: if (n4Years != 0L)
            if n4Years != 0 {
               // Java: n4Day = (daysAbsolute + 1L) % 1461L;
               n4Day = (daysAbsolute + 1) % 1461;
            }
                // Java: else
            else {
               // Java: leap4 = false;
               leap4 = false;
               // Java: n4Day = daysAbsolute;
               n4Day = daysAbsolute;
            }
         }
            // Java: else
         else {
            // Java: n4Years = daysAbsolute / 1461L;
            n4Years = daysAbsolute / 1461;
            // Java: n4Day = daysAbsolute % 1461L;
            n4Day = daysAbsolute % 1461;
         }
         // Java: long n4Yr;
         let n4Yr: i64;
            // Java: if (leap4)
         if leap4 {
            // Java: n4Yr = (n4Day - 1L) / 365L;
            n4Yr = (n4Day - 1) / 365;
                // Java: if (n4Yr != 0L)
            if n4Yr != 0 {
               // Java: n4Day = (n4Day - 1L) % 365L;
               n4Day = (n4Day - 1) % 365;
            }
         }
            // Java: else
         else {
            // Java: n4Yr = n4Day / 365L;
            n4Yr = n4Day / 365;
            // Java: n4Day %= 365L;
            n4Day %= 365;
         }
         // Java: tmDest.mYearDay = (int)n4Day + 1;
         tmDest.mYearDay = n4Day as i32 + 1;
         // Java: tmDest.mYear = (int)(n400Years * 400L + n400Century * 100L + n4Years * 4L + n4Yr);
         tmDest.mYear = (n400Years * 400 + n400Century * 100 + n4Years * 4 + n4Yr) as i32;
         // Java: boolean skip = false;
         let mut skip = false;
            // Java: if (n4Yr == 0L && leap4)
         if n4Yr == 0 && leap4 {
                // Java: if (n4Day == 59L)
            if n4Day == 59 {
               // Java: tmDest.mMonth = 2;
               tmDest.mMonth = 2;
               // Java: tmDest.mMonthDay = 29;
               tmDest.mMonthDay = 29;
               // Java: skip = true;
               skip = true;
            }
                // Java: else if (n4Day >= 60L)
            else if n4Day >= 60 {
               // Java: --n4Day;
               n4Day -= 1;
            }
         }
            // Java: if (!skip)
         if !skip {
            // Java: ++n4Day;
            n4Day += 1;
                // Java: for(tmDest.mMonth = (int)(n4Day >> 5) + 1; n4Day > (long)afxMonthDays[tmDest.mMonth]; ++tmDest.mMonth)
            tmDest.mMonth = (n4Day >> 5) as i32 + 1;
            while n4Day > afxMonthDays[tmDest.mMonth as usize] as i64 {
                tmDest.mMonth += 1;
            }
            // Java: tmDest.mMonthDay = (int)(n4Day - (long)afxMonthDays[tmDest.mMonth - 1]);
            tmDest.mMonthDay = (n4Day - afxMonthDays[(tmDest.mMonth - 1) as usize] as i64) as i32;
         }
            // Java: if (secsInDay == 0L)
         if secsInDay == 0 {
            // Java: tmDest.mHour = tmDest.mMinute = tmDest.mSecond = 0;
            tmDest.mHour = 0;
            tmDest.mMinute = 0;
            tmDest.mSecond = 0;
         }
            // Java: else
         else {
            // Java: tmDest.mSecond = (int)(secsInDay % 60L);
            tmDest.mSecond = (secsInDay % 60) as i32;
            // Java: long minutesInDay = secsInDay / 60L;
            let minutesInDay: i64 = secsInDay / 60;
            // Java: tmDest.mMinute = (int)minutesInDay % 60;
            tmDest.mMinute = (minutesInDay as i32) % 60;
            // Java: tmDest.mHour = (int)minutesInDay / 60;
            tmDest.mHour = (minutesInDay as i32) / 60;
         }
         // Java: double hundredths = dtSrc * (double)86400.0F;
         let hundredths: f64 = dtSrc * 86400.0;
         // Java: tmDest.mMillisecond = (int)((hundredths - Math.floor(hundredths)) * (double)1000.0F + (double)0.5F);
         tmDest.mMillisecond = ((hundredths - hundredths.floor()) * 1000.0 + 0.5) as i32;
            // Java: if (tmDest.mMillisecond >= 1000)
         if tmDest.mMillisecond >= 1000 {
            // Java: tmDest.mMillisecond %= 1000;
            tmDest.mMillisecond %= 1000;
         }
         // Java: return true;
         return true;
      }
        // Java: else
      else {
         // Java: return false;
         return false;
      }
    }

    // Java: static double greTime2DoubleExcel(int year, int month, int day, int hour, int minute, int second, int millisecond, boolean use1904)
    pub fn greTime2DoubleExcel(year: i32, month: i32, day: i32, hour: i32, minute: i32, second: i32, millisecond: i32, use1904: bool) -> f64 {
        // Java: if (year <= 9999 && month >= 1 && month <= 12)
      if year <= 9999 && month >= 1 && month <= 12 {
         // Java: boolean leapYear = (year & 3) == 0 && (year % 100 != 0 || year % 400 == 0);
         let mut leapYear = (year & 3) == 0 && (year % 100 != 0 || year % 400 == 0);
            // Java: if (year == 1900)
         if year == 1900 {
            // Java: leapYear = true;
            leapYear = true;
         }
         // Java: int daysInMonth = afxMonthDays[month] - afxMonthDays[month - 1] + (leapYear && day == 29 && month == 2 ? 1 : 0);
         let daysInMonth = afxMonthDays[month as usize] - afxMonthDays[(month - 1) as usize] + if leapYear && day == 29 && month == 2 { 1 } else { 0 };
            // Java: if (day >= 0 && day <= daysInMonth && hour <= 23 && minute <= 59 && second <= 59)
         if day >= 0 && day <= daysInMonth && hour <= 23 && minute <= 59 && second <= 59 {
            // Java: long nDate = (long)year * 365L + (long)(year / 4) - (long)(year / 100) + (long)(year / 400) + (long)afxMonthDays[month - 1] + (long)day;
            let mut nDate: i64 = year as i64 * 365 + (year / 4) as i64 - (year / 100) as i64 + (year / 400) as i64 + afxMonthDays[(month - 1) as usize] as i64 + day as i64;
                // Java: if (month <= 2 && leapYear)
            if month <= 2 && leapYear {
               // Java: --nDate;
               nDate -= 1;
            }
            // Java: nDate -= 693959L;
            nDate -= 693959;
                // Java: if (use1904)
            if use1904 {
               // Java: nDate -= 1462L;
               nDate -= 1462;
            }
            // Java: double dblTime = ((double)((long)hour * 3600L + (long)minute * 60L) + (double)second + (double)millisecond * 0.001) / (double)86400.0F;
            let dblTime: f64 = (hour as i64 * 3600 + minute as i64 * 60) as f64 + second as f64 + millisecond as f64 * 0.001;
            let dblTime = dblTime / 86400.0;
            // Java: return (double)nDate + (nDate >= 0L ? dblTime : -dblTime);
            return nDate as f64 + if nDate >= 0 { dblTime } else { -dblTime };
         }
            // Java: else
         else {
            // Java: return (double)-1.0F;
            return -1.0;
         }
      }
        // Java: else
      else {
         // Java: return (double)-1.0F;
         return -1.0;
      }
    }

    // Java: static double greTime2Double(int year, int month, int day, int hour, int minute, int second, int millisecond, boolean use1904)
    pub fn greTime2Double(year: i32, month: i32, day: i32, hour: i32, minute: i32, second: i32, millisecond: i32, use1904: bool) -> f64 {
        // Java: if (year <= 9999 && month >= 1 && month <= 12)
      if year <= 9999 && month >= 1 && month <= 12 {
         // Java: boolean leapYear = (year & 3) == 0 && (year % 100 != 0 || year % 400 == 0);
         let leapYear = (year & 3) == 0 && (year % 100 != 0 || year % 400 == 0);
         // Java: int daysInMonth = afxMonthDays[month] - afxMonthDays[month - 1] + (leapYear && day == 29 && month == 2 ? 1 : 0);
         let daysInMonth = afxMonthDays[month as usize] - afxMonthDays[(month - 1) as usize] + if leapYear && day == 29 && month == 2 { 1 } else { 0 };
            // Java: if (day >= 1 && day <= daysInMonth && hour <= 23 && minute <= 59 && second <= 59)
         if day >= 1 && day <= daysInMonth && hour <= 23 && minute <= 59 && second <= 59 {
            // Java: long nDate = (long)year * 365L + (long)(year / 4) - (long)(year / 100) + (long)(year / 400) + (long)afxMonthDays[month - 1] + (long)day;
            let mut nDate: i64 = year as i64 * 365 + (year / 4) as i64 - (year / 100) as i64 + (year / 400) as i64 + afxMonthDays[(month - 1) as usize] as i64 + day as i64;
                // Java: if (month <= 2 && leapYear)
            if month <= 2 && leapYear {
               // Java: --nDate;
               nDate -= 1;
            }
            // Java: nDate -= 693959L;
            nDate -= 693959;
                // Java: if (use1904)
            if use1904 {
               // Java: nDate -= 1462L;
               nDate -= 1462;
            }
            // Java: double dblTime = ((double)((long)hour * 3600L + (long)minute * 60L) + (double)second + (double)millisecond * 0.001) / (double)86400.0F;
            let dblTime: f64 = (hour as i64 * 3600 + minute as i64 * 60) as f64 + second as f64 + millisecond as f64 * 0.001;
            let dblTime = dblTime / 86400.0;
            // Java: return (double)nDate + (nDate >= 0L ? dblTime : -dblTime);
            return nDate as f64 + if nDate >= 0 { dblTime } else { -dblTime };
         }
            // Java: else
         else {
            // Java: return (double)-1.0F;
            return -1.0;
         }
      }
        // Java: else
      else {
         // Java: return (double)-1.0F;
         return -1.0;
      }
    }

}
