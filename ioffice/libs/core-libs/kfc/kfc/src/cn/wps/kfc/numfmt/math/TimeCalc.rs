// Java field: int GREG_BUDDHIST
pub(crate) const GREG_BUDDHIST: i32 = 543;
// Java field: int GREG_KOREAN
pub(crate) const GREG_KOREAN: i32 = 2333;
// Java field: int GREG_TAIWAN
pub(crate) const GREG_TAIWAN: i32 = 1911;

// Java: public class TimeCalc
pub struct TimeCalc {
}

// Java: public class TimeStruct
#[derive(Clone,Default)]
pub struct TimeStruct {
    // Java field: int mMillisecond
    pub mMillisecond: i32,
    // Java field: int mSecond
    pub mSecond: i32,
    // Java field: int mMinute
    pub mMinute: i32,
    // Java field: int mHour
    pub mHour: i32,
    // Java field: int mMonth
    pub mMonth: i32,
    // Java field: int mYear
    pub mYear: i32,
    // Java field: int mWeekDay
    pub mWeekDay: i32,
    // Java field: int mMonthDay
    pub mMonthDay: i32,
    // Java field: int mYearDay
    pub mYearDay: i32,
    // Java field: int mEra
    pub mEra: i32,
}

pub(crate) struct PInt {
    pub(crate) mInteger: i32,
}

// Java: public class TimeCalc
impl TimeCalc {
    // Java: TimeCalc()
    pub fn new() -> Self {
      // Java: super();
      TimeCalc {}
    }

    // Java: static boolean double2Time(double dtSrc, TimeStruct tmDest, boolean use1904, byte calendarType)
    pub fn double2Time(dt_src: f64, tm_dest: &mut TimeStruct, use1904: bool, calendar_type: i8) -> bool {
        // Java: if (!GregCalc.double2GregTimeExcel(dtSrc, tmDest, use1904))
      if !super::GregCalc::GregCalc::double2GregTimeExcel(dt_src, tm_dest, use1904) {
         // Java: return false;
         return false;
      }
        // Java: else
      else {
            // Java: switch (calendarType)
         match calendar_type {
            // Java: case 3:
            3 => {
               // Java: PNumber.PInt pYear = new PNumber.PInt();
               let mut p_year = PInt { mInteger: 0 };
               // Java: tmDest.mEra = greTime2JpTime(tmDest, pYear);
               tm_dest.mEra = Self::greTime2JpTime(tm_dest, &mut p_year);
               // Java: tmDest.mYear = pYear.mInteger;
               tm_dest.mYear = p_year.mInteger;
            }
            // Java: case 4:
            // LLM Rust match: combined with case 9-22, default into one arm
            // Java: case 9:
            // LLM Rust match: combined into 4|9|...|22 arm
            // Java: case 10:
            // LLM Rust match: combined into 4|9|...|22 arm
            // Java: case 11:
            // LLM Rust match: combined into 4|9|...|22 arm
            // Java: case 12:
            // LLM Rust match: combined into 4|9|...|22 arm
            // Java: case 13:
            // LLM Rust match: combined into 4|9|...|22 arm
            // Java: case 14:
            // LLM Rust match: combined into 4|9|...|22 arm
            // Java: case 15:
            // LLM Rust match: combined into 4|9|...|22 arm
            // Java: case 16:
            // LLM Rust match: combined into 4|9|...|22 arm
            // Java: case 17:
            // LLM Rust match: combined into 4|9|...|22 arm
            // Java: case 18:
            // LLM Rust match: combined into 4|9|...|22 arm
            // Java: case 20:
            // LLM Rust match: combined into 4|9|...|22 arm
            // Java: case 21:
            // LLM Rust match: combined into 4|9|...|22 arm
            // Java: case 22:
            // LLM Rust match: combined into 4|9|...|22 arm
            // Java: default:
            4 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 20 | 21 | 22 => {
               // Java: break;
               // LLM Rust match: implicit break at end of arm
            }
            // Java: case 5:
            5 => {
               // Java: tmDest.mYear = greYear2KoreanYear(tmDest.mYear);
               tm_dest.mYear = Self::greYear2KoreanYear(tm_dest.mYear);
               // Java: break;
               // LLM Rust match: implicit break at end of arm
            }
            // Java: case 6:
            6 => {
               // Java: HiJriCalc.greg2HiJri(tmDest, tmDest);
               super::HiJriCalc::HiJriCalc::greg2HiJri(tm_dest);
               // Java: break;
               // LLM Rust match: implicit break at end of arm
            }
            // Java: case 7:
            7 => {
               // Java: tmDest.mYear = greYear2BuddYear(tmDest.mYear);
               tm_dest.mYear = Self::greYear2BuddYear(tm_dest.mYear);
               // Java: break;
               // LLM Rust match: implicit break at end of arm
            }
            // Java: case 8:
            8 => {
               // Java: HebrewCalc.greg2HebrewTime(tmDest, tmDest);
               super::HebrewCalc::HebrewCalc::greg2HebrewTime(tm_dest);
               // Java: break;
               // LLM Rust match: implicit break at end of arm
            }
            // Java: case 19:
            19 => {
               // Java: LunarCalc lunarCalc = new LunarCalc();
               let mut lunarCalc = super::LunarCalc::LunarCalc::new();
               // Java: lunarCalc.greg2lunar(tmDest, tmDest);
               lunarCalc.greg2lunar(tm_dest);
               // Java: break;
               // LLM Rust match: implicit break at end of arm
            }
            // Java: case 23:
            23 => {
               // Java: QuranCalc.greg2QuranTime(tmDest, tmDest);
               super::QuranCalc::QuranCalc::greg2QuranTime(tm_dest);
            }
            _ => {
               // LLM Rust match: default case covered by 4|9|...|22 arm above
            }
         }
         // Java: return true;
         return true;
      }
    }

    // Java: static int greYear2BuddYear(int greYear)
    pub fn greYear2BuddYear(gre_year: i32) -> i32 {
      // Java: return greYear + 543;
      gre_year + 543
    }

    // Java: static int buddYear2greYear(int buddYear)
    pub fn buddYear2greYear(budd_year: i32) -> i32 {
      // Java: return buddYear - 543;
      budd_year - 543
    }

    // Java: static int greYear2KoreanYear(int greYear)
    pub fn greYear2KoreanYear(gre_year: i32) -> i32 {
      // Java: return greYear + 2333;
      gre_year + 2333
    }

    // Java: static int koreanYear2greYear(int koreanYear)
    pub fn koreanYear2greYear(korean_year: i32) -> i32 {
      // Java: return koreanYear - 2333;
      korean_year - 2333
    }

    // Java: static int gregYear2TaiwanYear(int gregYear)
    pub fn gregYear2TaiwanYear(greg_year: i32) -> i32 {
      // Java: int taiwanYear = gregYear - 1911;
      let mut taiwanYear = greg_year - 1911;
        // Java: if (taiwanYear <= 0)
      if taiwanYear <= 0 {
         // Java: --taiwanYear;
         taiwanYear -= 1;
      }
      // Java: return taiwanYear;
      taiwanYear
    }

    // Java: static int taiwanYear2gregYear(int taiwanYear)
    pub fn taiwanYear2gregYear(taiwan_year: i32) -> i32 {
      // Java: int gregYear = taiwanYear < 0 ? 1 : 0;
      let mut gregYear = if taiwan_year < 0 { 1 } else { 0 };
      // Java: gregYear += 1911;
      gregYear += 1911;
      // Java: return gregYear;
      gregYear
    }

    // Java: static int greTime2JpTime(TimeStruct dtm, PNumber.PInt year)
    pub(crate) fn greTime2JpTime(dtm: &TimeStruct, year: &mut PInt) -> i32 {
      // Java: int eraIdx = 2;
      let eraIdx;
      // Java: int jpYear = 0;
      let jpYear;
        // Java: if (dtm.mYear >= 1912 && (1912 != dtm.mYear || dtm.mMonth >= 7) && (1912 != dtm.mYear || 7 != dtm.mMonth || dtm.mMonthDay > 29))
      if dtm.mYear >= 1912 && (1912 != dtm.mYear || dtm.mMonth >= 7) && (1912 != dtm.mYear || 7 != dtm.mMonth || dtm.mMonthDay > 29) {
            // Java: if (dtm.mYear >= 1926 && (1926 != dtm.mYear || dtm.mMonth >= 12) && (1926 != dtm.mYear || 12 != dtm.mMonth || dtm.mMonthDay > 24))
         if dtm.mYear >= 1926 && (1926 != dtm.mYear || dtm.mMonth >= 12) && (1926 != dtm.mYear || 12 != dtm.mMonth || dtm.mMonthDay > 24) {
                // Java: if (dtm.mYear >= 1989 && (1989 != dtm.mYear || dtm.mMonth >= 1) && (1989 != dtm.mYear || 1 != dtm.mMonth || dtm.mMonthDay > 7))
            if dtm.mYear >= 1989 && (1989 != dtm.mYear || dtm.mMonth >= 1) && (1989 != dtm.mYear || 1 != dtm.mMonth || dtm.mMonthDay > 7) {
               // Java: eraIdx = 3;
               eraIdx = 3;
               // Java: jpYear = dtm.mYear - 1989 + 1;
               jpYear = dtm.mYear - 1989 + 1;
            }
                // Java: else
            else {
               // Java: eraIdx = 2;
               eraIdx = 2;
               // Java: jpYear = dtm.mYear - 1926 + 1;
               jpYear = dtm.mYear - 1926 + 1;
            }
         }
            // Java: else
         else {
            // Java: eraIdx = 1;
            eraIdx = 1;
            // Java: jpYear = dtm.mYear - 1912 + 1;
            jpYear = dtm.mYear - 1912 + 1;
         }
      }
        // Java: else
      else {
         // Java: eraIdx = 0;
         eraIdx = 0;
         // Java: jpYear = dtm.mYear - 1867;
         jpYear = dtm.mYear - 1867;
      }
        // Java: if (year != null)
        // LLM Rust Null-Safety: year is always a valid reference in Rust, no null check needed
      if true {
         // Java: year.mInteger = jpYear;
         year.mInteger = jpYear;
      }
      // Java: return eraIdx;
      eraIdx
    }

    // Java: static int jpYear2greYear(int jpYear, int jpEra)
    pub fn jpYear2greYear(jp_year: i32, jp_era: i32) -> i32 {
      // Java: int greYear = jpYear;
      let mut greYear = jp_year;
        // Java: switch (jpEra)
      match jp_era {
         // Java: case 0:
         0 => {
            // Java: greYear = jpYear + 1867;
            greYear = jp_year + 1867;
            // Java: break;
            // LLM Rust match: implicit break at end of arm
         }
         // Java: case 1:
         1 => {
            // Java: greYear = jpYear + 1911;
            greYear = jp_year + 1911;
            // Java: break;
            // LLM Rust match: implicit break at end of arm
         }
         // Java: case 2:
         2 => {
            // Java: greYear = jpYear + 1925;
            greYear = jp_year + 1925;
            // Java: break;
            // LLM Rust match: implicit break at end of arm
         }
         // Java: case 3:
         3 => {
            // Java: greYear = jpYear + 1988;
            greYear = jp_year + 1988;
         }
         _ => {}
      }
      // Java: return greYear;
      greYear
    }

    // Java: static int otherTime2greYear(TimeStruct otherTm, byte calendar)
    pub fn otherTime2greYear(other_tm: &TimeStruct, calendar: i8) -> i32 {
        // Java: switch (calendar)
      match calendar {
         // Java: case 3:
         3 => {
            // Java: return jpYear2greYear(otherTm.mYear, otherTm.mEra);
            return Self::jpYear2greYear(other_tm.mYear, other_tm.mEra);
         }
         // Java: case 4:
         // LLM Rust match: combined with default into 4 | _ arm
         // Java: default:
         4 => {
            // Java: return otherTm.mYear;
            return other_tm.mYear;
         }
         // Java: case 5:
         5 => {
            // Java: return koreanYear2greYear(otherTm.mYear);
            return Self::koreanYear2greYear(other_tm.mYear);
         }
         // Java: case 6:
         6 => {
            // Java: return HiJriCalc.hiJriYear2GregYear(otherTm.mYear);
            return super::HiJriCalc::HiJriCalc::hiJriYear2GregYear(other_tm.mYear);
         }
         // Java: case 7:
         7 => {
            // Java: return buddYear2greYear(otherTm.mYear);
            return Self::buddYear2greYear(other_tm.mYear);
         }
         // Java: case 8:
         8 => {
            // Java: return -1;
            return -1;
         }
         _ => {
            // LLM Rust match: default case covered by case 4 arm above
            return other_tm.mYear;
         }
      }
    }

    // Java: static int getElapsedHours(double date)
    pub fn getElapsedHours(date: f64) -> i32 {
      // Java: return (int)((date * (double)86400.0F + (double)0.5F) / (double)3600.0F);
      ((date * 86400.0 + 0.5) / 3600.0) as i32
    }

    // Java: static int getElapsedMinutes(double date)
    pub fn getElapsedMinutes(date: f64) -> i32 {
      // Java: return (int)((date * (double)86400.0F + (double)0.5F) / (double)60.0F);
      ((date * 86400.0 + 0.5) / 60.0) as i32
    }

    // Java: static long getElapsedSeconds(double date)
    pub fn getElapsedSeconds(date: f64) -> i64 {
      // Java: return (long)(date * (double)86400.0F + (double)0.5F);
      (date * 86400.0 + 0.5) as i64
    }

    // Java: static boolean isAm(int hour24)
    pub fn isAm(hour24: i32) -> bool {
      // Java: return hour24 >= 0 && hour24 < 12;
      hour24 >= 0 && hour24 < 12
    }

    // Java: static int hour24to12(int hour24)
    pub fn hour24to12(hour24: i32) -> i32 {
      // Java: int hour = hour24 % 12;
      let hour = hour24 % 12;
      // Java: return hour != 0 ? hour : 12;
      if hour != 0 { hour } else { 12 }
    }

}

// Java: public class TimeStruct
impl TimeStruct {
    // Java: TimeStruct()
    pub fn new() -> Self {
         // Java: super();
         TimeStruct {
             mMillisecond: 0,
             mSecond: 0,
             mMinute: 0,
             mHour: 0,
             mMonth: 0,
             mYear: 0,
             mWeekDay: 0,
             mMonthDay: 0,
             mYearDay: 0,
             mEra: 0,
         }
    }

}
