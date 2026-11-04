// java public class DateRes {
#[derive(Clone,serde::Serialize,serde::Deserialize)]
pub struct DateRes{
    // java public final boolean SUPPORT_MMMDY_DATE;
    pub SUPPORT_MMMDY_DATE:bool,
    // java public final String SHORT_DATE_NUMFMT;
    pub SHORT_DATE_NUMFMT:String,
    // java public final char PREFER_SEPERATOR_CHAR;
    pub PREFER_SEPERATOR_CHAR:u16,
    // java public final ShortDateFmtType SHORT_DATEFMT_TYPE;
    pub SHORT_DATEFMT_TYPE:ShortDateFmtType,
    // java public final String[] MONTHS;
    pub MONTHS:Option<Vec<String>>,
}
// java public static String[] ALSATIAN_FRANCE_MONTHS = new String[]{"Jän", "Fev", "März", "Apr", "Mai", "Jüni", "Jüli", "Aug", "Sept", "Okt", "Now", "Dez"};
pub const ALSATIAN_FRANCE_MONTHS:[&str;12]=["Jän","Fev","März","Apr","Mai","Jüni","Jüli","Aug","Sept","Okt","Now","Dez"];
// java public static String[] ARABIC_1_MONTHS = new String[]{"يناير", "فبراير", "مارس", "أبريل", "مايو", "يونيو", "يوليو", "أغسطس", "سبتمبر", "أكتوبر", "نوفمبر", "ديسمبر"};
pub const ARABIC_1_MONTHS:[&str;12]=["يناير","فبراير","مارس","أبريل","مايو","يونيو","يوليو","أغسطس","سبتمبر","أكتوبر","نوفمبر","ديسمبر"];
// java public static String[] ARABIC_2_MONTHS = new String[]{"جانفييه", "فيفرييه", "مارس", "أفريل", "مي", "جوان", "جوييه", "أوت", "سبتمبر", "أكتوبر", "نوفمبر", "ديسمبر"};
pub const ARABIC_2_MONTHS:[&str;12]=["جانفييه","فيفرييه","مارس","أفريل","مي","جوان","جوييه","أوت","سبتمبر","أكتوبر","نوفمبر","ديسمبر"];
// java public static String[] ARABIC_3_MONTHS = new String[]{"كانون الثاني", "شباط", "آذار", "نيسان", "أيار", "حزيران", "تموز", "آب", "أيلول", "تشرين الأول", "تشرين الثاني", "كانون الأول"};
pub const ARABIC_3_MONTHS:[&str;12]=["كانون الثاني","شباط","آذار","نيسان","أيار","حزيران","تموز","آب","أيلول","تشرين الأول","تشرين الثاني","كانون الأول"];
// java public static String[] GAELIC_IRELAND_MONTHS = new String[]{"Ean", "Feabh", "Már", "Aib", "Bealt", "Meith", "Iúil", "Lún", "M.Fómh", "D.Fómh", "Samh", "Noll"};
pub const GAELIC_IRELAND_MONTHS:[&str;12]=["Ean","Feabh","Már","Aib","Bealt","Meith","Iúil","Lún","M.Fómh","D.Fómh","Samh","Noll"];
// java public static String[] OCCITAN_FRANCE_MONTHS = new String[]{"Gen", "C'hwe", "Meur", "Ebr", "Mae", "Mezh", "Goue", "Eost", "Gwen", "Here", "Du", "Kzu"};
pub const OCCITAN_FRANCE_MONTHS:[&str;12]=["Gen","C'hwe","Meur","Ebr","Mae","Mezh","Goue","Eost","Gwen","Here","Du","Kzu"];
// java public static String[] BRETON_FRANCE_MONTHS = new String[]{"Gen", "C'hwe", "Meur", "Ebr", "Mae", "Mezh", "Goue", "Eost", "Gwen", "Here", "Du", "Kzu"};
pub const BRETON_FRANCE_MONTHS:[&str;12]=["Gen","C'hwe","Meur","Ebr","Mae","Mezh","Goue","Eost","Gwen","Here","Du","Kzu"];
// java public static String[] GERMAN_MONTHS = new String[]{"Jan", "Feb", "Mrz", "Apr", "Mai", "Jun", "Jul", "Aug", "Sep", "Okt", "Nov", "Dez"};
pub const GERMAN_MONTHS:[&str;12]=["Jan","Feb","Mrz","Apr","Mai","Jun","Jul","Aug","Sep","Okt","Nov","Dez"];
// java public static String[] RUSSIAN_MONTHS = new String[]{"янв", "фев", "мар", "апр", "май", "июн", "июл", "авг", "сен", "окт", "ноя", "дек"};
pub const RUSSIAN_MONTHS:[&str;12]=["янв","фев","мар","апр","май","июн","июл","авг","сен","окт","ноя","дек"];
// java public static String[] FRENCH_MONTHS = new String[]{"janv", "févr", "mars", "avr", "mai", "juin", "juil", "août", "sept", "oct", "nov", "déc"};
pub const FRENCH_MONTHS:[&str;12]=["janv","févr","mars","avr","mai","juin","juil","août","sept","oct","nov","déc"];
// java public static String[] FRISIAN_NETHERLANDS_MONTHS = new String[]{"jann", "febr", "mrt", "apr", "maaie", "jun", "jul", "aug", "sept", "okt", "nov", "des"};
pub const FRISIAN_NETHERLANDS_MONTHS:[&str;12]=["jann","febr","mrt","apr","maaie","jun","jul","aug","sept","okt","nov","des"];
// java public static String[] DUTCH_MONTHS = new String[]{"jan", "feb", "mrt", "apr", "mei", "jun", "jul", "aug", "sep", "okt", "nov", "dec"};
pub const DUTCH_MONTHS:[&str;12]=["jan","feb","mrt","apr","mei","jun","jul","aug","sep","okt","nov","dec"];
// java public static String[] LUXEMBOURGISH_LUXEMBOURG_MONTHS = new String[]{"Jan", "Feb", "Mäe", "Abr", "Mee", "Jun", "Jul", "Aug", "Sep", "Okt", "Nov", "Dez"};
pub const LUXEMBOURGISH_LUXEMBOURG_MONTHS:[&str;12]=["Jan","Feb","Mäe","Abr","Mee","Jun","Jul","Aug","Sep","Okt","Nov","Dez"];
// java public static String[] PORTUGUESE_BRAZILIAN_MONTHS = new String[]{"jan", "fev", "mar", "abr", "mai", "jun", "jul", "ago", "set", "out", "nov", "dez"};
pub const PORTUGUESE_BRAZILIAN_MONTHS:[&str;12]=["jan","fev","mar","abr","mai","jun","jul","ago","set","out","nov","dez"];
// java public static String[] PORTUGUESE_MONTHS = new String[]{"Jan", "Fev", "Mar", "Abr", "Mai", "Jun", "Jul", "Ago", "Set", "Out", "Nov", "Dez"};
pub const PORTUGUESE_MONTHS:[&str;12]=["Jan","Fev","Mar","Abr","Mai","Jun","Jul","Ago","Set","Out","Nov","Dez"];
// java public static String[] SWEDISH_MONTHS = new String[]{"jan", "feb", "mar", "apr", "maj", "jun", "jul", "aug", "sep", "okt", "nov", "dec"};
pub const SWEDISH_MONTHS:[&str;12]=["jan","feb","mar","apr","maj","jun","jul","aug","sep","okt","nov","dec"];
// java public static String[] SPANISH_MONTHS = new String[]{"ene", "feb", "mar", "abr", "may", "jun", "jul", "ago", "sep", "oct", "nov", "dic"};
pub const SPANISH_MONTHS:[&str;12]=["ene","feb","mar","abr","may","jun","jul","ago","sep","oct","nov","dic"];
// java public static String[] ITALIAN_MONTHS = new String[]{"gen", "feb", "mar", "apr", "mag", "giu", "lug", "ago", "set", "ott", "nov", "dic"};
pub const ITALIAN_MONTHS:[&str;12]=["gen","feb","mar","apr","mag","giu","lug","ago","set","ott","nov","dic"];
// java public static String[] CHINESE_MONTHS = new String[]{"一月", "二月", "三月", "四月", "五月", "六月", "七月", "八月", "九月", "十月", "十一月", "十二月"};
pub const CHINESE_MONTHS:[&str;12]=["一月","二月","三月","四月","五月","六月","七月","八月","九月","十月","十一月","十二月"];
// java public static String[] HEBREW_MONTHS = new String[]{"ינו", "פבר", "מרץ", "אפר", "מאי", "יונ", "יול", "אוג", "ספט", "אוק", "נוב", "דצמ"};
pub const HEBREW_MONTHS:[&str;12]=["ינו","פבר","מרץ","אפר","מאי","יונ","יול","אוג","ספט","אוק","נוב","דצמ"];
impl DateRes{
    // java public DateRes(boolean supportMMDYDate, String shortDateNumfmt, char preferSeperator, ShortDateFmtType shortDateFmtType, String[] months) {
    pub fn DateRes(supportMMDYDate:bool,shortDateNumfmt:String,preferSeperator:u16,shortDateFmtType:ShortDateFmtType,months:Option<Vec<String>>)->Self{
        // java this.SUPPORT_MMMDY_DATE = supportMMDYDate;
        // java this.SHORT_DATE_NUMFMT = shortDateNumfmt;
        // java this.PREFER_SEPERATOR_CHAR = preferSeperator;
        // java this.SHORT_DATEFMT_TYPE = shortDateFmtType;
        // java this.MONTHS = months;
        Self{SUPPORT_MMMDY_DATE:supportMMDYDate,SHORT_DATE_NUMFMT:shortDateNumfmt,PREFER_SEPERATOR_CHAR:preferSeperator,SHORT_DATEFMT_TYPE:shortDateFmtType,MONTHS:months}
    }
}
// java public static enum ShortDateFmtType { DMY, YMD, MDY; }
#[derive(Clone,Copy,PartialEq,Eq,serde::Serialize,serde::Deserialize)]
pub enum ShortDateFmtType{DMY,YMD,MDY}
