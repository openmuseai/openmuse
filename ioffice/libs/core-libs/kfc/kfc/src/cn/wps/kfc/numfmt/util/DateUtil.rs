#![allow(non_snake_case, non_camel_case_types, unused_variables, unused_imports, dead_code, unreachable_code, unused_mut)]
// rust: Native java.util.Calendar field adapter; format decisions use the original DateUtil/GregCalc methods.
pub trait CalendarFields{fn get(&self,field:i32)->i32;}

// Java: import cn.wps.kfc.numfmt.builtin.BuiltinFormats
use crate::cn::wps::kfc::numfmt::builtin::BuiltinFormats;
// Java: import cn.wps.kfc.numfmt.math.GregCalc
use crate::cn::wps::kfc::numfmt::math::GregCalc;
// Java: import cn.wps.kfc.numfmt.math.TimeCalc
use crate::cn::wps::kfc::numfmt::math::TimeCalc;
// Java: import java.util.Calendar
// Java: import java.util.Date
// Java: import java.util.regex.Matcher
// Java: import java.util.regex.Pattern

// Java field: int DateFormatType
pub(crate) const DateFormatType: i32 = 1;
// Java field: int TimeFormatType
pub(crate) const TimeFormatType: i32 = 2;
// Java field: double MIN_DATE_VALUE
pub(crate) const MIN_DATE_VALUE: f64 = 0.0;
// Java field: double MAX_DATE_VALUE
pub(crate) const MAX_DATE_VALUE: f64 = 2958465.999988426;
// Java field: int MAX_YEAR
pub(crate) const MAX_YEAR: i32 = 9999;
// Java field: int MIN_YEAR
pub(crate) const MIN_YEAR: i32 = 1899;
// Java field: String F800
pub(crate) const F800: &str = "[$-F800]";
// Java field: String F400
pub(crate) const F400: &str = "[$-F400]";
// Java field: Pattern ampmPattern

// Java: public class DateUtil
pub struct DateUtil {
}

// Java: public class DateUtil
impl DateUtil {
    // Java: DateUtil()
    pub fn new() -> Self {
        // Java: super();
        DateUtil {}
    }

    // Java: static boolean isADateFormat(int formatIndex, String formatString)
    pub fn isADateFormat(formatIndex: i32, formatString: &str) -> bool {
        // Java: if (isInternalDateTimeFormat(formatIndex))
        if Self::isInternalDateTimeFormat(formatIndex) {
            // Java: return true;
            true
        }
        // Java: else
        else {
            // Java: return isDateTimeFormat(formatIndex, formatString) != 0;
            Self::isDateTimeFormat(formatIndex, formatString) != 0
        }
    }

    // Java: static int isDateTimeFormat(int formatIndex, String formatString)
    pub fn isDateTimeFormat(formatIndex: i32, mut formatString: &str) -> i32 {
        // Java: if (isBuildInNotDateFormat(formatIndex))
        if Self::isBuildInNotDateFormat(formatIndex) {
            // Java: return 0;
            0
        }
        // Java: else
        else {
            // Java: switch (formatIndex)
            match formatIndex {
                // Java: case 14:
                // Java: case 15:
                // Java: case 16:
                // Java: case 17:
                    // Java: return 1;
                14 | 15 | 16 | 17 => 1,
                // Java: case 18:
                // Java: case 19:
                // Java: case 20:
                // Java: case 21:
                // Java: case 45:
                // Java: case 46:
                // Java: case 47:
                    // Java: return 2;
                18 | 19 | 20 | 21 | 45 | 46 | 47 => 2,
                // Java: case 22:
                    // Java: return 3;
                22 => 3,
                // Java: case 23:
                // Java: case 24:
                // Java: case 25:
                // Java: case 26:
                // Java: case 27:
                // Java: case 28:
                // Java: case 29:
                // Java: case 30:
                // Java: case 31:
                // Java: case 32:
                // Java: case 33:
                // Java: case 34:
                // Java: case 35:
                // Java: case 36:
                // Java: case 37:
                // Java: case 38:
                // Java: case 39:
                // Java: case 40:
                // Java: case 41:
                // Java: case 42:
                // Java: case 43:
                // Java: case 44:
                // Java: default:
                _ => {
                    // Java: int groupIndex = formatString.indexOf(59);
                    let groupIndex = formatString.find(';');
                    // Java: if (groupIndex > 0 && groupIndex < formatString.length() - 1)
                    if groupIndex.is_some_and(|i| i > 0 && i < formatString.len() - 1) {
                        // Java: formatString = formatString.substring(0, groupIndex);
                        formatString = &formatString[..groupIndex.unwrap()];
                    }
                    // Java: return getDateTimeFormatType(formatString);
                    Self::getDateTimeFormatType(formatString)
                }
            }
        }
    }

    // Java: static int getDateTimeFormatType(String formatString)
    pub fn getDateTimeFormatType(formatString: &str) -> i32 {
        // Java: if (formatString != null && formatString.length() != 0)
        if !formatString.is_empty() {
            // Java: if (formatString.indexOf("[$-F400]") != -1)
            if formatString.contains("[$-F400]") {
                // Java: return 2;
                2
            }
            // Java: else if (formatString.indexOf("[$-F800]") != -1)
            else if formatString.contains("[$-F800]") {
                // Java: return 1;
                1
            }
            // Java: else
            else {
                // Java: String fs = NumFmtUtil.filterInvalidChar(formatString);
                let fs = super::NumFmtUtil::NumFmtUtil::filterInvalidChar(formatString).expect("non-null format");
                // Java: if (fs.indexOf("General") != -1)
                if fs.contains("General") {
                    // Java: return 0;
                    0
                }
                // Java: else
                else {
                    // Java: int len = fs.length();
                    let fsChars: Vec<u16> = fs.encode_utf16().collect();
                    let len = fsChars.len() as i32;
                    // Java: int dateTime = 0;
                    let mut dateTime = 0;
                    // Java: int hIndex = len;
                    let mut hIndex = len;
                    // Java: int mIndex = -1;
                    let mut mIndex = -1;
                    // Java: int sIndex = -1;
                    let mut sIndex = -1;
                    // Java: int charANum = 0;
                    let mut charANum = 0;
                    // Java: Matcher m = ampmPattern.matcher(fs);
                    // Java: if (m.find())
                    if fs.contains("A/P") || fs.contains("AM/PM") {
                        // Java: dateTime |= 2;
                        dateTime |= 2;
                    }
                    // Java: for(int i = 0; i < len; ++i)
                    for i in 0..len {
                        // Java: char c = fs.charAt(i);
                        let c = fsChars[i as usize];
                        // Java: switch (c)
                        match c {
                            // Java: case 'A':
                            // Java: case 'a':
                                // Java: ++charANum;
                            65 | 97 => { charANum += 1; },
                                // Java: break;
                            // Java: case 'D':
                            // Java: case 'Y':
                            // Java: case 'd':
                            // Java: case 'e':
                            // Java: case 'y':
                                // Java: dateTime |= 1;
                            68 | 89 | 100 | 101 | 121 => { dateTime |= 1; },
                                // Java: break;
                            // Java: case 'H':
                            // Java: case 'h':
                                // Java: dateTime |= 2;
                                // Java: hIndex = i;
                            72 | 104 => { dateTime |= 2; hIndex = i; },
                                // Java: break;
                            // Java: case 'M':
                            // Java: case 'm':
                                // Java: mIndex = i;
                            77 | 109 => { mIndex = i; },
                                // Java: break;
                            // Java: case 'S':
                            // Java: case 's':
                                // Java: dateTime |= 2;
                                // Java: sIndex = i;
                            83 | 115 => { dateTime |= 2; sIndex = i; },
                            _ => {},
                        }
                        // Java: if (mIndex > hIndex || mIndex < sIndex)
                        if mIndex > hIndex || mIndex < sIndex {
                            // Java: dateTime |= 2;
                            dateTime |= 2;
                        }
                        // Java: if (c != 'a' && c != 'A')
                        if c != 97 && c != 65 {
                            // Java: charANum = 0;
                            charANum = 0;
                        }
                        // Java: if (charANum > 2)
                        if charANum > 2 {
                            // Java: dateTime |= 1;
                            dateTime |= 1;
                        }
                        // Java: if (dateTime == 3)
                        if dateTime == 3 {
                            // Java: break;
                            break;
                        }
                    }
                    // Java: if (dateTime == 0 && mIndex != -1)
                    if dateTime == 0 && mIndex != -1 {
                        // Java: dateTime |= 1;
                        dateTime |= 1;
                    }
                    // Java: return dateTime;
                    dateTime
                }
            }
        }
        // Java: else
        else {
            // Java: return 0;
            0
        }
    }

    // Java: static boolean isInternalDateTimeFormat(int format)
    pub fn isInternalDateTimeFormat(format: i32) -> bool {
        // Java: switch (format)
        match format {
            // Java: case 14:
            // Java: case 15:
            // Java: case 16:
            // Java: case 17:
            // Java: case 18:
            // Java: case 19:
            // Java: case 20:
            // Java: case 21:
            // Java: case 22:
            // Java: case 45:
            // Java: case 46:
            // Java: case 47:
                // Java: return true;
            14..=22 | 45..=47 => true,
            // Java: case 23:
            // Java: case 24:
            // Java: case 25:
            // Java: case 26:
            // Java: case 27:
            // Java: case 28:
            // Java: case 29:
            // Java: case 30:
            // Java: case 31:
            // Java: case 32:
            // Java: case 33:
            // Java: case 34:
            // Java: case 35:
            // Java: case 36:
            // Java: case 37:
            // Java: case 38:
            // Java: case 39:
            // Java: case 40:
            // Java: case 41:
            // Java: case 42:
            // Java: case 43:
            // Java: case 44:
            // Java: default:
                // Java: return format >= 27 && format <= 36 || format >= 50 && format <= 58;
            _ => (27..=36).contains(&format) || (50..=58).contains(&format),
        }
    }

    // Java: static boolean isBuildInNotDateFormat(int format)
    pub fn isBuildInNotDateFormat(format: i32) -> bool {
        // Java: return format >= 0 && format <= 13 || format >= 37 && format <= 44 || format >= 48 && format <= 49;
        (0..=13).contains(&format) || (37..=44).contains(&format) || (48..=49).contains(&format)
    }

    // Java: static String performSpecialDateFormat(String formatCode)
    pub fn performSpecialDateFormat(_format_code: &str) -> String {
        // Java: if (formatCode.indexOf("[$-F400]") != -1)
        {
            // Java: return BuiltinFormats.getAll()[33];
        }
        // Java: else
        {
            // Java: return formatCode.indexOf("[$-F800]") != -1 ? BuiltinFormats.getAll()[31] : formatCode;
        }
        todo!("DateUtil::performSpecialDateFormat")
    }

    // Java: static boolean isValidExcelDate(double value)
    pub fn isValidExcelDate(_value: f64) -> bool {
        // Java: return value >= (double)0.0F && value <= 2958465.999988426;
        todo!("DateUtil::isValidExcelDate")
    }

// java public static Date getJavaDate(double date, boolean is1904) {
// java TimeCalc.TimeStruct tm = getGregDate(date, false);
// java Date javaDate = new Date(tm.mYear - 1900, tm.mMonth - 1, tm.mMonthDay, tm.mHour, tm.mMinute, tm.mSecond);
// java return javaDate;
// java }
pub fn getJavaDate(date:f64,_is1904:bool)->i64 {
use chrono::TimeZone;let tm=Self::getGregDate(date,false);let year=tm.mYear+(tm.mMonth-1).div_euclid(12);let month=(tm.mMonth-1).rem_euclid(12)+1;let base=chrono::NaiveDate::from_ymd_opt(year,month as u32,1).expect("Date range").and_hms_opt(0,0,0).unwrap();let value=base+chrono::Duration::days(tm.mMonthDay as i64-1)+chrono::Duration::hours(tm.mHour as i64)+chrono::Duration::minutes(tm.mMinute as i64)+chrono::Duration::seconds(tm.mSecond as i64);chrono::Local.from_local_datetime(&value).earliest().expect("Date local time").timestamp_millis()
}
    // Java: static double getExcelDate(Date date, boolean use1904)
    pub fn getExcelDate_from_date(_date: i64, _use1904: bool) -> f64 {
        // Java: int year = date.getYear();
        // Java: int month = date.getMonth();
        // Java: int monthDay = date.getDate();
        // Java: int hours = date.getHours();
        // Java: int minutes = date.getMinutes();
        // Java: int seconds = date.getSeconds();
        // Java: return GregCalc.greTime2DoubleExcel(year + 1900, month + 1, monthDay, hours, minutes, seconds, 0, use1904);
        todo!("DateUtil::getExcelDate_from_date")
    }

    // Java: static double getExcelDate(Calendar c, boolean use1904)
    pub fn getExcelDate_from_calendar(c: &dyn CalendarFields, use1904: bool) -> f64 {
        // Java: int year = c.get(1);
        // Java: int month = c.get(2);
        // Java: int monthDay = c.get(5);
        // Java: int hours = c.get(11);
        // Java: int minutes = c.get(12);
        // Java: int seconds = c.get(13);
        // Java: return GregCalc.greTime2DoubleExcel(year, month + 1, monthDay, hours, minutes, seconds, 0, use1904);
        super::super::math::GregCalc::GregCalc::greTime2DoubleExcel(c.get(1),c.get(2)+1,c.get(5),c.get(11),c.get(12),c.get(13),0,use1904)
    }

    // Java: static double getExcelDate(TimeCalc.TimeStruct tm, boolean use1904)
    pub fn getExcelDate_from_timestruct(_tm: i32, _use1904: bool) -> f64 {
        // Java: return GregCalc.greTime2DoubleExcel(tm.mYear, tm.mMonth, tm.mMonthDay, tm.mHour, tm.mMinute, tm.mSecond, tm.mMillisecond, use1904);
        todo!("DateUtil::getExcelDate_from_timestruct")
    }

    // Java: static double getExcelDate(int year, int month, int monthDay, int hour, int minute, int second, boolean use1904)
    pub fn getExcelDate_from_parts(_year: i32, _month: i32, _month_day: i32, _hour: i32, _minute: i32, _second: i32, _use1904: bool) -> f64 {
        // Java: return GregCalc.greTime2DoubleExcel(year, month, monthDay, hour, minute, second, 0, use1904);
        todo!("DateUtil::getExcelDate_from_parts")
    }

    // Java: static double getExcelDate(int year, int month, int monthDay, boolean use1904)
    pub fn getExcelDate_from_ymd(_year: i32, _month: i32, _month_day: i32, _use1904: bool) -> f64 {
        // Java: return GregCalc.greTime2DoubleExcel(year, month, monthDay, 0, 0, 0, 0, use1904);
        super::super::math::GregCalc::GregCalc::greTime2DoubleExcel(_year,_month,_month_day,0,0,0,0,_use1904)
    }

    // Java: static double getExcelTime(int hour, int minute, int second, int millsecond, boolean use1904)
    pub fn getExcelTime(_hour: i32, _minute: i32, _second: i32, _millsecond: i32, _use1904: bool) -> f64 {
        // Java: return GregCalc.greTime2DoubleExcel(0, 0, 0, hour, minute, second, millsecond, use1904);
        todo!("DateUtil::getExcelTime")
    }
}

impl DateUtil{// java public static TimeCalc.TimeStruct getGregDate(double date, boolean use1904) {
// java TimeCalc.TimeStruct tm = new TimeCalc.TimeStruct();
// java GregCalc.double2GregTimeExcel(date, tm, use1904);
// java return tm;
// java }
pub fn getGregDate(date:f64,use1904:bool)->super::super::math::TimeCalc::TimeStruct {
let mut tm=super::super::math::TimeCalc::TimeStruct::default();super::super::math::GregCalc::GregCalc::double2GregTimeExcel(date,&mut tm,use1904);tm
}
}
