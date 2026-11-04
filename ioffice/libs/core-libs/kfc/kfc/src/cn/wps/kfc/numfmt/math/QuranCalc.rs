#![allow(non_snake_case, non_camel_case_types, unused_variables, unused_imports, dead_code, unreachable_code, unused_mut)]

use super::TimeCalc::TimeStruct;
use std::sync::OnceLock;

// Java field: Calendar minDate
// Java field: Calendar maxDate
// Java field: int minQuranYear
pub(crate) const MIN_QURAN_YEAR: i32 = 1318;
// Java field: int[][] dateYearInfo
static DATE_YEAR_INFO: OnceLock<Vec<[i32; 4]>> = OnceLock::new();

// Java: public class QuranCalc
pub struct QuranCalc;

// Java: public class QuranCalc
impl QuranCalc {
    // Java: QuranCalc()
    pub fn new() -> Self {
        // Java: super();
        QuranCalc
    }

    // Java: static void greg2QuranTime(TimeCalc.TimeStruct greg, TimeCalc.TimeStruct quran)
    pub fn greg2QuranTime(_tm: &mut TimeStruct) {
        // Java: Calendar gregCalendar = Calendar.getInstance();
        // Java: gregCalendar.set(greg.mYear, greg.mMonth, greg.mMonthDay);
        // Java: int gregorianYear = gregCalendar.get(1);
        // Java: int index = -1;
        // Java: int nDays = 0;
        // Java: if (gregCalendar.getTimeInMillis() >= minDate.getTimeInMillis() && gregCalendar.getTimeInMillis() <= maxDate.getTimeInMillis())
        {
            // Java: for(int i = 0; i < dateYearInfo.length; ++i)
            {
                // Java: if (dateYearInfo[i][1] == gregorianYear)
                {
                    // Java: index = i;
                    // Java: break;
                }
            }
            // Java: Calendar comCalendar;
            // Java: do
            {
                // Java: comCalendar = Calendar.getInstance();
                // Java: comCalendar.set(dateYearInfo[index][1], dateYearInfo[index][2], dateYearInfo[index][3]);
                // Java: ++index;
            }
            // Java: while(gregCalendar.compareTo(comCalendar) > 0);
            // Java: --index;
            // Java: gregCalendar.get(14);
            // Java: gregCalendar.set(14, 0);
            // Java: comCalendar.get(14);
            // Java: comCalendar.set(14, 0);
            // Java: if (gregCalendar.compareTo(comCalendar) != 0)
            {
                // Java: --index;
            }
            // Java: String endTimeStr = greg.mYear + "-" + greg.mMonth + "-" + greg.mMonthDay;
            // Java: String startTimeStr = dateYearInfo[index][1] + "-" + dateYearInfo[index][2] + "-" + dateYearInfo[index][3];
            // Java: quran.mYear = index + 1318;
            // Java: quran.mMonth = 1;
            // Java: quran.mMonthDay = 1;
            // Java: nDays = getIntervalDays(endTimeStr, startTimeStr);
            // Java: int b = dateYearInfo[index][0];
            // Java: for(int DaysPerThisMonth = 29 + (b & 1); nDays >= DaysPerThisMonth; ++quran.mMonth)
            {
                // Java: nDays -= DaysPerThisMonth;
                // Java: b >>= 1;
                // Java: DaysPerThisMonth = 29 + (b & 1);
            }
            // Java: quran.mMonthDay += nDays;
            // Java: quran.mHour = greg.mHour;
            // Java: quran.mMinute = greg.mMinute;
            // Java: quran.mSecond = greg.mSecond;
        }
        // Java: else
        {
            // Java: HiJriCalc.greg2HiJri(greg, quran);
        }
        todo!("QuranCalc::greg2QuranTime")
    }

    // Java: static int getIntervalDays(String EndTimeStr, String startTimeStr)
    fn getIntervalDays(_end_time_str: &str, _start_time_str: &str) -> i32 {
        // Java: SimpleDateFormat sdformat = new SimpleDateFormat("yyyy-MM-dd", Locale.US);
        // Java: long endTime = 0L;
        // Java: long startTime = 0L;
        // Java: try
        {
            // Java: endTime = sdformat.parse(EndTimeStr).getTime() / 1000L;
            // Java: startTime = sdformat.parse(startTimeStr).getTime() / 1000L;
        }
        // Java: catch (ParseException e)
        {
            // Java: e.printStackTrace();
        }
        // Java: return (int)(endTime - startTime) / 86400;
        todo!("QuranCalc::getIntervalDays")
    }
}
