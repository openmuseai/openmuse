// Java: import cn.wps.moffice.define.UILanguage
// Java: import java.text.DateFormat
// Java: import java.text.ParseException
// Java: import java.text.SimpleDateFormat
// Java: import java.util.Calendar
// Java: import java.util.Date
// Java: import java.util.HashMap
// Java: import java.util.Locale
// Java: import java.util.Map
// Java: import java.util.TimeZone

// Java field: String DATE_FORMAT_DAY
pub(crate) const DATE_FORMAT_DAY: &str = "yyyyMMdd";
// Java field: String DATE_FORMAT_DAY_1
pub(crate) const DATE_FORMAT_DAY_1: &str = "yyyy-MM-dd";
// Java field: String DATE_FORMAT_DAY_2
pub(crate) const DATE_FORMAT_DAY_2: &str = "yyyy.MM.dd";
// Java field: String DATE_FORMAT_DAY_3
pub(crate) const DATE_FORMAT_DAY_3: &str = "dd/MM/yyyy";
// Java field: String DATE_FORMAT_DAY_4
pub(crate) const DATE_FORMAT_DAY_4: &str = "MM/dd/yyyy";
// Java field: String DATE_FORMAT_DAY_5
pub(crate) const DATE_FORMAT_DAY_5: &str = "dd/MM";
// Java field: String DATE_FORMAT_DAY_6
pub(crate) const DATE_FORMAT_DAY_6: &str = "MM-dd";
// Java field: String DATE_FORMAT_VERBOSE_WITHOUT_SPACE
pub(crate) const DATE_FORMAT_VERBOSE_WITHOUT_SPACE: &str = "yyyy-MM-dd HH:mm";
// Java field: String DATE_FORMAT_VERBOSE
pub(crate) const DATE_FORMAT_VERBOSE: &str = "yyyy-MM-dd   HH:mm";
// Java field: String DATE_FORMAT_DAY_USA
pub(crate) const DATE_FORMAT_DAY_USA: &str = "MM/dd/yyyy   HH:mm";
// Java field: String DATE_FORMAT_DAY_UK_RUSSIA
pub(crate) const DATE_FORMAT_DAY_UK_RUSSIA: &str = "dd/MM/yyyy   HH:mm";
// Java field: String DATE_FORMAT_FILE_NAME
pub(crate) const DATE_FORMAT_FILE_NAME: &str = "yyyyMMddHHmm";
// Java field: String DATE_FORMAT_CALENDAR_WIDGET_DATE
pub(crate) const DATE_FORMAT_CALENDAR_WIDGET_DATE: &str = "MM-dd";
// Java field: String DATE_FORMAT_CALENDAR_WIDGET_TIME
pub(crate) const DATE_FORMAT_CALENDAR_WIDGET_TIME: &str = "HH:mm";
// Java field: ThreadLocal<Map<String, DateFormat>> smDateFormatMap
// Java field: ThreadLocal<Locale> mDateFormatLocale
// Java field: DateFormat smDefaultDateFormat

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

    // Java: static String formatDate(Date date)
    pub fn formatDate(_date: i64) -> String {
      // Java: return formatDate(date, "yyyy-MM-dd   HH:mm");
      todo!()
    }

    // Java: static String formatDate(Date date, String date_str_format)
    pub fn formatDate_with_format(_date: i64, _date_str_format: &str) -> String {
      // Java: DateFormat format = getFormat(date_str_format);
      // Java: return format.format(date);
      todo!()
    }

    // Java: static DateFormat getFormat(String dateFormat)
    pub fn getFormat(_dateFormat: &str) -> Box<dyn std::any::Any> {
      // Java: Map<String, DateFormat> map = (Map)smDateFormatMap.get();
      // Java: Locale locale = (Locale)mDateFormatLocale.get();
      // Java: boolean localeExpired = !Locale.getDefault().equals(locale);
      // Java: if (localeExpired || map == null)
      // Java: map = new HashMap();
      // Java: smDateFormatMap.set(map);
      // Java: mDateFormatLocale.set(Locale.getDefault());
      // Java: DateFormat format = (DateFormat)map.get(dateFormat);
      // Java: if (null == format)
      // Java: format = new SimpleDateFormat(dateFormat, Locale.getDefault());
      // Java: map.put(dateFormat, format);
      // Java: return format;
      todo!()
    }

    // Java: static String formatDate(long millisecond, String formatStr)
    pub fn formatDate_with_millis(_millisecond: i64, _formatStr: &str) -> String {
      // Java: return formatDate(new Date(millisecond), formatStr);
      todo!()
    }

    // Java: static Date getDateByDays(int days)
    pub fn getDateByDays(_days: i32) -> i64 {
      // Java: Calendar c = Calendar.getInstance();
      // Java: c.add(5, days);
      // Java: return c.getTime();
      todo!()
    }

    // Java: static long getDaysBetween(Date startDate, Date endDate)
    pub fn getDaysBetween(_startDate: i64, _endDate: i64) -> i64 {
      // Java: Calendar fromCalendar = Calendar.getInstance();
      // Java: fromCalendar.setTime(startDate);
      // Java: fromCalendar.set(11, 0);
      // Java: fromCalendar.set(12, 0);
      // Java: fromCalendar.set(13, 0);
      // Java: fromCalendar.set(14, 0);
      // Java: Calendar toCalendar = Calendar.getInstance();
      // Java: toCalendar.setTime(endDate);
      // Java: toCalendar.set(11, 0);
      // Java: toCalendar.set(12, 0);
      // Java: toCalendar.set(13, 0);
      // Java: toCalendar.set(14, 0);
      // Java: return (toCalendar.getTime().getTime() - fromCalendar.getTime().getTime()) / 86400000L;
      todo!()
    }

    // Java: static boolean isSameDay(Date date1, Date date2)
    pub fn isSameDay(_date1: i64, _date2: i64) -> bool {
      // Java: if (date1 != null && date2 != null)
      // Java: Calendar cal1 = Calendar.getInstance();
      // Java: Calendar cal2 = Calendar.getInstance();
      // Java: cal1.setTime(date1);
      // Java: cal2.setTime(date2);
      // Java: return cal1.get(6) == cal2.get(6) && cal1.get(1) == cal2.get(1);
      // Java: else
      // Java: return false;
      todo!()
    }

    // Java: static Date parseDate(String str)
    pub fn parseDate(_str: &str) -> Option<i64> {
      // Java: return parseDate(str, "yyyy-MM-dd   HH:mm");
      todo!()
    }

    // Java: static Date parseDate(String str, String date_str_format)
    pub fn parseDate_with_format(_str: &str, _date_str_format: &str) -> Option<i64> {
      // Java: try
      // Java: DateFormat format = getFormat(date_str_format);
      // Java: return format.parse(str);
      // Java: catch (ParseException var3)
      // Java: return null;
      todo!()
    }

    // Java: static long convertToDateTime(String dateStr, long defaultTime)
    pub fn convertToDateTime(_dateStr: Option<&str>, _defaultTime: i64) -> i64 {
      // Java: if (dateStr != null && dateStr.length() != 0)
      // Java: Date date = parseDate(dateStr, "yyyy-MM-dd HH:mm");
      // Java: return null == date ? defaultTime : date.getTime();
      // Java: else
      // Java: return defaultTime;
      todo!()
    }

    // Java: static String formatDateByLangNLocale(Date date, UILanguage currentLanguage)
    pub fn formatDateByLangNLocale(_date: i64, _currentLanguage: &dyn std::any::Any) -> String {
      // Java: if (currentLanguage == UILanguage.UILanguage_english)
      // Java: return Locale.getDefault().equals(Locale.UK) ? formatDate(date, "dd/MM/yyyy   HH:mm") : formatDate(date, "MM/dd/yyyy   HH:mm");
      // Java: else
      // Java: return currentLanguage == UILanguage.UILanguage_russian ? formatDate(date, "dd/MM/yyyy   HH:mm") : formatDate(date, "yyyy-MM-dd   HH:mm");
      todo!()
    }

    // Java: static String formatSimpleDateByLangNLocale(Date date, UILanguage currentLanguage, boolean showYear)
    pub fn formatSimpleDateByLangNLocale(_date: i64, _currentLanguage: &dyn std::any::Any, _showYear: bool) -> String {
      // Java: if (showYear)
      // Java: if (currentLanguage == UILanguage.UILanguage_english)
      // Java: return Locale.getDefault().equals(Locale.UK) ? formatDate(date, "dd/MM/yyyy") : formatDate(date, "MM/dd/yyyy");
      // Java: else
      // Java: return currentLanguage == UILanguage.UILanguage_russian ? formatDate(date, "dd/MM/yyyy") : formatDate(date, "yyyy-MM-dd");
      // Java: else if (currentLanguage == UILanguage.UILanguage_english)
      // Java: return Locale.getDefault().equals(Locale.UK) ? formatDate(date, "dd/MM") : formatDate(date, "MM/dd");
      // Java: else
      // Java: return currentLanguage == UILanguage.UILanguage_russian ? formatDate(date, "dd/MM") : formatDate(date, "MM-dd");
      todo!()
    }

    // Java: static String getGMTDateString(Date date)
    pub fn getGMTDateString(_date: i64) -> String {
      // Java: return getGMTDateString(date, Locale.US);
      todo!()
    }

    // Java: static String getGMTDateString(Date date, Locale locale)
    pub fn getGMTDateString_with_locale(_date: i64, _locale: &dyn std::any::Any) -> String {
      // Java: SimpleDateFormat sdf = new SimpleDateFormat("EEE, dd MMM yyyy HH:mm:ss ", locale);
      // Java: sdf.setTimeZone(TimeZone.getTimeZone("GMT+00:00"));
      // Java: return sdf.format(date).concat("GMT");
      todo!()
    }

    // Java: static long dayStart(long millis)
    pub fn dayStart(_millis: i64) -> i64 {
      // Java: Calendar calendar = Calendar.getInstance();
      // Java: calendar.setTimeInMillis(millis);
      // Java: calendar.set(14, 0);
      // Java: calendar.set(13, 0);
      // Java: calendar.set(12, 0);
      // Java: calendar.set(11, 0);
      // Java: return calendar.getTimeInMillis();
      todo!()
    }

    // Java: static String getGMTDateStringEdu(Date date)
    pub fn getGMTDateStringEdu(_date: i64) -> String {
      // Java: SimpleDateFormat sdf = new SimpleDateFormat("dd/MM/yyyy", Locale.US);
      // Java: return sdf.format(date);
      todo!()
    }

    // Java: static long dayEnd(long millis)
    pub fn dayEnd(_millis: i64) -> i64 {
      // Java: Calendar calendar = Calendar.getInstance();
      // Java: calendar.setTimeInMillis(millis);
      // Java: calendar.set(14, 0);
      // Java: calendar.set(13, 0);
      // Java: calendar.set(12, 59);
      // Java: calendar.set(11, 23);
      // Java: return calendar.getTimeInMillis();
      todo!()
    }

}
