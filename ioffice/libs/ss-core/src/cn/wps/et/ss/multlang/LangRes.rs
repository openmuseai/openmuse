use super::res::{SeparatorRes::SeparatorRes,CurrencyRes::CurrencyRes,DateRes::DateRes};
use util::org::apache::poi::util::LanguageType::*;
// java public class LangRes {
#[derive(serde::Serialize,serde::Deserialize)]
pub struct LangRes{
    // java public final SeparatorRes SEPERATORS;
    pub SEPERATORS:SeparatorRes,
    // java public final NegativeFormat NEGATIVE_FORMAT;
    pub NEGATIVE_FORMAT:NegativeFormat,
    // java public final DateRes DATE;
    pub DATE:DateRes,
    // java public final String AM;
    pub AM:Option<String>,
    // java public final String PM;
    pub PM:Option<String>,
    // java public final CurrencyRes[] CURRENCIES;
    pub CURRENCIES:Vec<CurrencyRes>,
    // java public final String[][] CUSTOMSERIES;
    pub CUSTOMSERIES:Vec<Vec<String>>,
}
impl LangRes{
    // java private LangRes(SeparatorRes sep, NegativeFormat negativeFormat, DateRes date, String am, String pm, CurrencyRes[] currencies) {
    fn LangRes(sep:SeparatorRes,negativeFormat:NegativeFormat,date:DateRes,am:Option<String>,pm:Option<String>,currencies:Vec<CurrencyRes>)->Self{
        // java this.SEPERATORS = sep;
        // java this.NEGATIVE_FORMAT = negativeFormat;
        // java this.DATE = date;
        // java this.AM = am;
        // java this.PM = pm;
        // java this.CURRENCIES = currencies;
        // java this.CUSTOMSERIES = new String[][]{{"Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"}, {"Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday"}, {"Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"}, {"January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"}};
        let series=[&["Sun","Mon","Tue","Wed","Thu","Fri","Sat"][..],&["Sunday","Monday","Tuesday","Wednesday","Thursday","Friday","Saturday"][..],&["Jan","Feb","Mar","Apr","May","Jun","Jul","Aug","Sep","Oct","Nov","Dec"][..],&["January","February","March","April","May","June","July","August","September","October","November","December"][..]];
        Self{SEPERATORS:sep,NEGATIVE_FORMAT:negativeFormat,DATE:date,AM:am,PM:pm,CURRENCIES:currencies,CUSTOMSERIES:series.into_iter().map(|a|a.iter().map(|v|(*v).into()).collect()).collect()}
    }
    // java private LangRes(SeparatorRes sep, NegativeFormat negativeFormat, DateRes date, String am, String pm, CurrencyRes[] currencies, String[][] customSeries) {
    fn LangRes__2(sep:SeparatorRes,negativeFormat:NegativeFormat,date:DateRes,am:Option<String>,pm:Option<String>,currencies:Vec<CurrencyRes>,customSeries:Vec<Vec<String>>)->Self{
        // java this.SEPERATORS = sep;
        // java this.NEGATIVE_FORMAT = negativeFormat;
        // java this.DATE = date;
        // java this.AM = am;
        // java this.PM = pm;
        // java this.CURRENCIES = currencies;
        // java this.CUSTOMSERIES = customSeries;
        Self{SEPERATORS:sep,NEGATIVE_FORMAT:negativeFormat,DATE:date,AM:am,PM:pm,CURRENCIES:currencies,CUSTOMSERIES:customSeries}
    }
    // java public static LangRes createByLangType(LanguageType lang) { switch (lang) {
    // rust: resource strings/arrays are literal payloads exported unchanged from
    // the original constructors. Keep the Java switch and constructor overload
    // choice here; the native literal decoder below makes no language decisions.
    pub fn createByLangType(lang:LanguageType)->Self{
        match lang{
            // java case LANGUAGE_ALSATIAN_FRANCE:
            // java return new LangRes(SeparatorRes.SPACE_GROUPER_SEPERATORS, LangRes.NegativeFormat.SN, new DateRes(false, "dd/MM/yyyy", '/', DateRes.ShortDateFmtType.DMY, DateRes.ALSATIAN_FRANCE_MONTHS), (String)null, (String)null, new CurrencyRes[]{new CurrencyRes("€", CurrencyRes.PositiveCurrencyFormat.SPACE_END, CurrencyRes.NegativeCurrencyFormat.SNPC, "#,##0 \"€\";[Red]-#,##0 \"€\"", "#,##0.00 \"€\";[Red]-#,##0.00 \"€\""), new CurrencyRes("€", CurrencyRes.PositiveCurrencyFormat.HEAD_SPACE, CurrencyRes.NegativeCurrencyFormat.SCPN, "[$€-2] #,##0;[Red]-[$€-2] #,##0", "[$€-2] #,##0.00;[Red]-[$€-2] #,##0.00")});
            LANGUAGE_ALSATIAN_FRANCE=>Self::literal("LANGUAGE_ALSATIAN_FRANCE",false),
            // java case LANGUAGE_ARABIC_ALGERIA:
            // java return new LangRes(SeparatorRes.AR_SEPERATORS, LangRes.NegativeFormat.NS, new DateRes(false, "dd-MM-yyyy", '-', DateRes.ShortDateFmtType.DMY, DateRes.ARABIC_2_MONTHS), "ص", "م", new CurrencyRes[]{new CurrencyRes("د.ج.", CurrencyRes.PositiveCurrencyFormat.HEAD_SPACE, CurrencyRes.NegativeCurrencyFormat.CPNS, "\"د.ج.\" #,##0_-;[Red]\"د.ج.\" #,##0-", "\"د.ج.\" #,##0.00_-;[Red]\"د.ج.\" #,##0.00-")});
            LANGUAGE_ARABIC_ALGERIA=>Self::literal("LANGUAGE_ARABIC_ALGERIA",false),
            // java case LANGUAGE_ARABIC_UAE:
            // java return new LangRes(SeparatorRes.AR_SEPERATORS, LangRes.NegativeFormat.NS, new DateRes(false, "dd/MM/yyyy", '/', DateRes.ShortDateFmtType.DMY, DateRes.ARABIC_1_MONTHS), "ص", "م", new CurrencyRes[]{new CurrencyRes("د.إ.", CurrencyRes.PositiveCurrencyFormat.HEAD_SPACE, CurrencyRes.NegativeCurrencyFormat.CPNS, "\"د.إ.\" #,##0_-;[Red]\"د.إ.\" #,##0-", "\"د.إ.\" #,##0.00_-;[Red]\"د.إ.\" #,##0.00-")});
            LANGUAGE_ARABIC_UAE=>Self::literal("LANGUAGE_ARABIC_UAE",false),
            // java case LANGUAGE_ARABIC_OMAN:
            // java return new LangRes(SeparatorRes.AR_SEPERATORS, LangRes.NegativeFormat.NS, new DateRes(false, "dd/MM/yyyy", '/', DateRes.ShortDateFmtType.DMY, DateRes.ARABIC_1_MONTHS), "ص", "م", new CurrencyRes[]{new CurrencyRes("ر.ع.", CurrencyRes.PositiveCurrencyFormat.HEAD_SPACE, CurrencyRes.NegativeCurrencyFormat.CPNS, "\"ر.ع.\" #,##0_-;[Red]\"ر.ع.\" #,##0-", "\"ر.ع.\" #,##0.00_-;[Red]\"ر.ع.\" #,##0.00-")});
            LANGUAGE_ARABIC_OMAN=>Self::literal("LANGUAGE_ARABIC_OMAN",false),
            // java case LANGUAGE_ARABIC_EGYPT:
            // java return new LangRes(SeparatorRes.AR_SEPERATORS, LangRes.NegativeFormat.NS, new DateRes(false, "dd/MM/yyyy", '/', DateRes.ShortDateFmtType.DMY, DateRes.ARABIC_1_MONTHS), "ص", "م", new CurrencyRes[]{new CurrencyRes("ج.م.", CurrencyRes.PositiveCurrencyFormat.HEAD_SPACE, CurrencyRes.NegativeCurrencyFormat.CPNS, "\"ج.م.\" #,##0_-;[Red]\"ج.م.\" #,##0-", "\"ج.م.\" #,##0.00_-;[Red]\"ج.م.\" #,##0.00-")});
            LANGUAGE_ARABIC_EGYPT=>Self::literal("LANGUAGE_ARABIC_EGYPT",false),
            // java case LANGUAGE_ARABIC_BAHRAIN:
            // java return new LangRes(SeparatorRes.AR_SEPERATORS, LangRes.NegativeFormat.NS, new DateRes(false, "dd/MM/yyyy", '/', DateRes.ShortDateFmtType.DMY, DateRes.ARABIC_1_MONTHS), "ص", "م", new CurrencyRes[]{new CurrencyRes("د.ب.", CurrencyRes.PositiveCurrencyFormat.HEAD_SPACE, CurrencyRes.NegativeCurrencyFormat.CPNS, "\"د.ب.\" #,##0_-;[Red]\"د.ب.\" #,##0-", "\"د.ب.\" #,##0.00_-;[Red]\"د.ب.\" #,##0.00-")});
            LANGUAGE_ARABIC_BAHRAIN=>Self::literal("LANGUAGE_ARABIC_BAHRAIN",false),
            // java case LANGUAGE_ARABIC_QATAR:
            // java return new LangRes(SeparatorRes.AR_SEPERATORS, LangRes.NegativeFormat.NS, new DateRes(false, "dd/MM/yyyy", '/', DateRes.ShortDateFmtType.DMY, DateRes.ARABIC_1_MONTHS), "ص", "م", new CurrencyRes[]{new CurrencyRes("ر.ق.", CurrencyRes.PositiveCurrencyFormat.HEAD_SPACE, CurrencyRes.NegativeCurrencyFormat.CPNS, "\"ر.ق.\" #,##0_-;[Red]\"ر.ق.\" #,##0-", "\"ر.ق.\" #,##0.00_-;[Red]\"ر.ق.\" #,##0.00-")});
            LANGUAGE_ARABIC_QATAR=>Self::literal("LANGUAGE_ARABIC_QATAR",false),
            // java case LANGUAGE_ARABIC_KUWAIT:
            // java return new LangRes(SeparatorRes.AR_SEPERATORS, LangRes.NegativeFormat.NS, new DateRes(false, "dd/MM/yyyy", '/', DateRes.ShortDateFmtType.DMY, DateRes.ARABIC_1_MONTHS), "ص", "م", new CurrencyRes[]{new CurrencyRes("د.ك.", CurrencyRes.PositiveCurrencyFormat.HEAD_SPACE, CurrencyRes.NegativeCurrencyFormat.CPNS, "\"د.ك.\" #,##0_-;[Red]\"د.ك.\" #,##0-", "\"د.ك.\" #,##0.00_-;[Red]\"د.ك.\" #,##0.00-")});
            LANGUAGE_ARABIC_KUWAIT=>Self::literal("LANGUAGE_ARABIC_KUWAIT",false),
            // java case LANGUAGE_ARABIC_LEBANON:
            // java return new LangRes(SeparatorRes.AR_SEPERATORS, LangRes.NegativeFormat.NS, new DateRes(false, "dd/MM/yyyy", '/', DateRes.ShortDateFmtType.DMY, DateRes.ARABIC_3_MONTHS), "ص", "م", new CurrencyRes[]{new CurrencyRes("ل.ل.", CurrencyRes.PositiveCurrencyFormat.HEAD_SPACE, CurrencyRes.NegativeCurrencyFormat.CPNS, "\"ل.ل.\" #,##0_-;[Red]\"ل.ل.\" #,##0-", "\"ل.ل.\" #,##0.00_-;[Red]\"ل.ل.\" #,##0.00-")});
            LANGUAGE_ARABIC_LEBANON=>Self::literal("LANGUAGE_ARABIC_LEBANON",false),
            // java case LANGUAGE_ARABIC_LIBYA:
            // java return new LangRes(SeparatorRes.AR_SEPERATORS, LangRes.NegativeFormat.NS, new DateRes(false, "dd/MM/yyyy", '/', DateRes.ShortDateFmtType.DMY, DateRes.ARABIC_1_MONTHS), "ص", "م", new CurrencyRes[]{new CurrencyRes("د.ل.", CurrencyRes.PositiveCurrencyFormat.HEAD, CurrencyRes.NegativeCurrencyFormat.CNS, "\"د.ل.\"#,##0_-;[Red]\"د.ل.\"#,##0-", "\"د.ل.\"#,##0.00_-;[Red]\"د.ل.\"#,##0.00-")});
            LANGUAGE_ARABIC_LIBYA=>Self::literal("LANGUAGE_ARABIC_LIBYA",false),
            // java case LANGUAGE_ARABIC_MOROCCO:
            // java return new LangRes(SeparatorRes.AR_SEPERATORS, LangRes.NegativeFormat.NS, new DateRes(false, "yyyy-MM-dd", '-', DateRes.ShortDateFmtType.YMD, DateRes.ARABIC_2_MONTHS), "ص", "م", new CurrencyRes[]{new CurrencyRes("د.م.", CurrencyRes.PositiveCurrencyFormat.HEAD_SPACE, CurrencyRes.NegativeCurrencyFormat.CPNS, "\"د.م.\" #,##0_-;[Red]\"د.م.\" #,##0-", "\"د.م.\" #,##0.00_-;[Red]\"د.م.\" #,##0.00-")});
            LANGUAGE_ARABIC_MOROCCO=>Self::literal("LANGUAGE_ARABIC_MOROCCO",false),
            // java case LANGUAGE_ARABIC_SAUDI_ARABIA:
            // java return new LangRes(SeparatorRes.AR_SEPERATORS, LangRes.NegativeFormat.NS, new DateRes(false, "dd/MM/yy", '/', DateRes.ShortDateFmtType.DMY, DateRes.ARABIC_1_MONTHS), "ص", "م", new CurrencyRes[]{new CurrencyRes("ر.س.", CurrencyRes.PositiveCurrencyFormat.HEAD_SPACE, CurrencyRes.NegativeCurrencyFormat.CPNS, "\"ر.س.\" #,##0_-;[Red]\"ر.س.\" #,##0-", "\"ر.س.\" #,##0.00_-;[Red]\"ر.س.\" #,##0.00-")});
            LANGUAGE_ARABIC_SAUDI_ARABIA=>Self::literal("LANGUAGE_ARABIC_SAUDI_ARABIA",false),
            // java case LANGUAGE_ARABIC_TUNISIA:
            // java return new LangRes(SeparatorRes.AR_SEPERATORS, LangRes.NegativeFormat.NS, new DateRes(false, "dd-MM-yyyy", '-', DateRes.ShortDateFmtType.DMY, DateRes.ARABIC_2_MONTHS), "ص", "م", new CurrencyRes[]{new CurrencyRes("د.ت.", CurrencyRes.PositiveCurrencyFormat.HEAD_SPACE, CurrencyRes.NegativeCurrencyFormat.CPNS, "\"د.ت.\" #,##0_-;[Red]\"د.ت.\" #,##0-", "\"د.ت.\" #,##0.00_-;[Red]\"د.ت.\" #,##0.00-")});
            LANGUAGE_ARABIC_TUNISIA=>Self::literal("LANGUAGE_ARABIC_TUNISIA",false),
            // java case LANGUAGE_ARABIC_SYRIA:
            // java return new LangRes(SeparatorRes.AR_SEPERATORS, LangRes.NegativeFormat.NS, new DateRes(false, "dd/MM/yyyy", '/', DateRes.ShortDateFmtType.DMY, DateRes.ARABIC_3_MONTHS), "ص", "م", new CurrencyRes[]{new CurrencyRes("ل.س.", CurrencyRes.PositiveCurrencyFormat.HEAD_SPACE, CurrencyRes.NegativeCurrencyFormat.CPNS, "\"ل.س.\" #,##0_-;[Red]\"ل.س.\" #,##0-", "\"ل.س.\" #,##0.00_-;[Red]\"ل.س.\" #,##0.00-")});
            LANGUAGE_ARABIC_SYRIA=>Self::literal("LANGUAGE_ARABIC_SYRIA",false),
            // java case LANGUAGE_ARABIC_YEMEN:
            // java return new LangRes(SeparatorRes.AR_SEPERATORS, LangRes.NegativeFormat.NS, new DateRes(false, "dd/MM/yyyy", '/', DateRes.ShortDateFmtType.DMY, DateRes.ARABIC_1_MONTHS), "ص", "م", new CurrencyRes[]{new CurrencyRes("ر.ي.", CurrencyRes.PositiveCurrencyFormat.HEAD_SPACE, CurrencyRes.NegativeCurrencyFormat.CPNS, "\"ر.ي.\" #,##0_-;[Red]\"ر.ي.\" #,##0-", "\"ر.ي.\" #,##0.00_-;[Red]\"ر.ي.\" #,##0.00-")});
            LANGUAGE_ARABIC_YEMEN=>Self::literal("LANGUAGE_ARABIC_YEMEN",false),
            // java case LANGUAGE_ARABIC_IRAQ:
            // java return new LangRes(SeparatorRes.AR_SEPERATORS, LangRes.NegativeFormat.NS, new DateRes(false, "dd/MM/yyyy", '/', DateRes.ShortDateFmtType.DMY, DateRes.ARABIC_3_MONTHS), "ص", "م", new CurrencyRes[]{new CurrencyRes("د.ع.", CurrencyRes.PositiveCurrencyFormat.HEAD_SPACE, CurrencyRes.NegativeCurrencyFormat.CPNS, "\"د.ع.\" #,##0_-;[Red]\"د.ع.\" #,##0-", "\"د.ع.\" #,##0.00_-;[Red]\"د.ع.\" #,##0.00-")});
            LANGUAGE_ARABIC_IRAQ=>Self::literal("LANGUAGE_ARABIC_IRAQ",false),
            // java case LANGUAGE_ARABIC_JORDAN:
            // java return new LangRes(SeparatorRes.AR_SEPERATORS, LangRes.NegativeFormat.NS, new DateRes(false, "dd/MM/yyyy", '/', DateRes.ShortDateFmtType.DMY, DateRes.ARABIC_3_MONTHS), "ص", "م", new CurrencyRes[]{new CurrencyRes("د.ا.", CurrencyRes.PositiveCurrencyFormat.HEAD_SPACE, CurrencyRes.NegativeCurrencyFormat.CPNS, "\"د.ا.\" #,##0_-;[Red]\"د.ا.\" #,##0-", "\"د.ا.\" #,##0.00_-;[Red]\"د.ا.\" #,##0.00-")});
            LANGUAGE_ARABIC_JORDAN=>Self::literal("LANGUAGE_ARABIC_JORDAN",false),
            // java case LANGUAGE_GAELIC_IRELAND:
            // java return new LangRes(SeparatorRes.COMMON_SEPERATORS, LangRes.NegativeFormat.SN, new DateRes(false, "dd/MM/yyyy", '/', DateRes.ShortDateFmtType.DMY, DateRes.GAELIC_IRELAND_MONTHS), "r.n.", "i.n.", new CurrencyRes[]{new CurrencyRes("€", CurrencyRes.PositiveCurrencyFormat.HEAD, CurrencyRes.NegativeCurrencyFormat.SCN, "\"€\"#,##0;[Red]-\"€\"#,##0", "\"€\"#,##0.00;[Red]-\"€\"#,##0.00"), new CurrencyRes("€", CurrencyRes.PositiveCurrencyFormat.SPACE_END, CurrencyRes.NegativeCurrencyFormat.SNPC, "#,##0 [$€-1];[Red]-#,##0 [$€-1]", "#,##0.00 [$€-1];[Red]-#,##0.00 [$€-1]")});
            LANGUAGE_GAELIC_IRELAND=>Self::literal("LANGUAGE_GAELIC_IRELAND",false),
            // java case LANGUAGE_OCCITAN_FRANCE:
            // java return new LangRes(SeparatorRes.SPACE_GROUPER_SEPERATORS, LangRes.NegativeFormat.SN, new DateRes(false, "dd/MM/yyyy", '/', DateRes.ShortDateFmtType.DMY, DateRes.OCCITAN_FRANCE_MONTHS), (String)null, (String)null, new CurrencyRes[]{new CurrencyRes("€", CurrencyRes.PositiveCurrencyFormat.SPACE_END, CurrencyRes.NegativeCurrencyFormat.SNPC, "#,##0 \"€\";[Red]-#,##0 \"€\"", "#,##0.00 \"€\";[Red]-#,##0.00 \"€\""), new CurrencyRes("€", CurrencyRes.PositiveCurrencyFormat.HEAD_SPACE, CurrencyRes.NegativeCurrencyFormat.SCPN, "[$€-2] #,##0;[Red]-[$€-2] #,##0", "[$€-2] #,##0.00;[Red]-[$€-2] #,##0.00")});
            LANGUAGE_OCCITAN_FRANCE=>Self::literal("LANGUAGE_OCCITAN_FRANCE",false),
            // java case LANGUAGE_BRETON_FRANCE:
            // java return new LangRes(SeparatorRes.SPACE_GROUPER_SEPERATORS, LangRes.NegativeFormat.SN, new DateRes(false, "dd/MM/yyyy", '/', DateRes.ShortDateFmtType.DMY, DateRes.BRETON_FRANCE_MONTHS), (String)null, (String)null, new CurrencyRes[]{new CurrencyRes("€", CurrencyRes.PositiveCurrencyFormat.SPACE_END, CurrencyRes.NegativeCurrencyFormat.SNPC, "#,##0 \"€\";[Red]-#,##0 \"€\"", "#,##0.00 \"€\";[Red]-#,##0.00 \"€\""), new CurrencyRes("€", CurrencyRes.PositiveCurrencyFormat.HEAD_SPACE, CurrencyRes.NegativeCurrencyFormat.SCPN, "[$€-2] #,##0;[Red]-[$€-2] #,##0", "[$€-2] #,##0.00;[Red]-[$€-2] #,##0.00")});
            LANGUAGE_BRETON_FRANCE=>Self::literal("LANGUAGE_BRETON_FRANCE",false),
            // java case LANGUAGE_KOREAN:
            // java case LANGUAGE_KOREAN_JOHAB:
            // java return new LangRes(SeparatorRes.COMMON_SEPERATORS, LangRes.NegativeFormat.SN, new DateRes(true, "yyyy-MM-dd", '-', DateRes.ShortDateFmtType.YMD, (String[])null), (String)null, (String)null, new CurrencyRes[]{new CurrencyRes("₩", CurrencyRes.PositiveCurrencyFormat.HEAD, CurrencyRes.NegativeCurrencyFormat.SCN, "\"₩\"#,##0;[Red]-\"₩\"#,##0", "\"₩\"#,##0.00;[Red]-\"₩\"#,##0.00")});
            LANGUAGE_KOREAN|LANGUAGE_KOREAN_JOHAB=>Self::literal("LANGUAGE_KOREAN",false),
            // java case LANGUAGE_GERMAN:
            // java return new LangRes(SeparatorRes.DOT_GROUPER_SEPERATORS, LangRes.NegativeFormat.SN, new DateRes(false, "dd.MM.yyyy", '.', DateRes.ShortDateFmtType.DMY, DateRes.GERMAN_MONTHS), (String)null, (String)null, new CurrencyRes[]{new CurrencyRes("€", CurrencyRes.PositiveCurrencyFormat.SPACE_END, CurrencyRes.NegativeCurrencyFormat.SNPC, "#,##0 \"€\";[Red]-#,##0 \"€\"", "#,##0.00 \"€\";[Red]-#,##0.00 \"€\""), new CurrencyRes("€", CurrencyRes.PositiveCurrencyFormat.HEAD_SPACE, CurrencyRes.NegativeCurrencyFormat.SCPN, "[$€-2] #,##0;[Red]-[$€-2] #,##0", "[$€-2] #,##0.00;[Red]-[$€-2] #,##0.00")});
            LANGUAGE_GERMAN=>Self::literal("LANGUAGE_GERMAN",false),
            // java case LANGUAGE_GERMAN_LUXEMBOURG:
            // java return new LangRes(SeparatorRes.DOT_GROUPER_SEPERATORS, LangRes.NegativeFormat.SN, new DateRes(false, "dd.MM.yyyy", '.', DateRes.ShortDateFmtType.DMY, DateRes.GERMAN_MONTHS), (String)null, (String)null, new CurrencyRes[]{new CurrencyRes("€", CurrencyRes.PositiveCurrencyFormat.SPACE_END, CurrencyRes.NegativeCurrencyFormat.SNPC, "#,##0 \"€\";[Red]-#,##0 \"€\"", "#,##0.00 \"€\";[Red]-#,##0.00 \"€\""), new CurrencyRes("€", CurrencyRes.PositiveCurrencyFormat.HEAD_SPACE, CurrencyRes.NegativeCurrencyFormat.SCPN, "[$€-2] #,##0;[Red]-[$€-2] #,##0", "[$€-2] #,##0.00;[Red]-[$€-2] #,##0.00")});
            LANGUAGE_GERMAN_LUXEMBOURG=>Self::literal("LANGUAGE_GERMAN_LUXEMBOURG",false),
            // java case LANGUAGE_GERMAN_SWISS:
            // java return new LangRes(SeparatorRes.QUOT_GROUPER_SEPERATORS, LangRes.NegativeFormat.SN, new DateRes(false, "dd.MM.yyyy", '.', DateRes.ShortDateFmtType.DMY, DateRes.GERMAN_MONTHS), (String)null, (String)null, new CurrencyRes[]{new CurrencyRes("Fr.", CurrencyRes.PositiveCurrencyFormat.HEAD_SPACE, CurrencyRes.NegativeCurrencyFormat.CSN, "\"Fr.\" #,##0;[Red]\"Fr.\" -#,##0", "\"Fr.\" #,##0.00;[Red]\"Fr.\" -#,##0.00")});
            LANGUAGE_GERMAN_SWISS=>Self::literal("LANGUAGE_GERMAN_SWISS",false),
            // java case LANGUAGE_RUSSIAN:
            // java case LANGUAGE_RUSSIAN_MOLDOVA:
            // java return new LangRes(SeparatorRes.SPACE_GROUPER_SEPERATORS, LangRes.NegativeFormat.SN, new DateRes(false, "dd.MM.yyyy", '.', DateRes.ShortDateFmtType.DMY, DateRes.RUSSIAN_MONTHS), (String)null, (String)null, new CurrencyRes[]{new CurrencyRes("р.", CurrencyRes.PositiveCurrencyFormat.END, CurrencyRes.NegativeCurrencyFormat.SNC, "#,##0\"р.\";[Red]-#,##0\"р.\"", "#,##0.00\"р.\";[Red]-#,##0.00\"р.\""), new CurrencyRes("p.", CurrencyRes.PositiveCurrencyFormat.END, CurrencyRes.NegativeCurrencyFormat.SNC, "#,##0\"р.\";[Red]-#,##0\"р.\"", "#,##0.00\"р.\";[Red]-#,##0.00\"р.\"")});
            LANGUAGE_RUSSIAN|LANGUAGE_RUSSIAN_MOLDOVA=>Self::literal("LANGUAGE_RUSSIAN",false),
            // java case LANGUAGE_FRENCH_BELGIAN:
            // java return new LangRes(SeparatorRes.DOT_GROUPER_SEPERATORS, LangRes.NegativeFormat.SN, new DateRes(false, "d/MM/yyyy", '/', DateRes.ShortDateFmtType.DMY, DateRes.FRENCH_MONTHS), (String)null, (String)null, new CurrencyRes[]{new CurrencyRes("€", CurrencyRes.PositiveCurrencyFormat.HEAD_SPACE, CurrencyRes.NegativeCurrencyFormat.CPSN, "\"€\" #,##0;[Red]\"€\" -#,##0", "\"€\" #,##0.00;[Red]\"€\" -#,##0.00"), new CurrencyRes("€", CurrencyRes.PositiveCurrencyFormat.SPACE_END, CurrencyRes.NegativeCurrencyFormat.NSPC, "#,##0_- [$€-1];[Red]#,##0- [$€-1]", "#,##0.00_- [$€-1];[Red]#,##0.00- [$€-1]")});
            LANGUAGE_FRENCH_BELGIAN=>Self::literal("LANGUAGE_FRENCH_BELGIAN",false),
            // java case LANGUAGE_FRENCH:
            // java return new LangRes(SeparatorRes.SPACE_GROUPER_SEPERATORS, LangRes.NegativeFormat.SN, new DateRes(false, "dd/MM/yyyy", '/', DateRes.ShortDateFmtType.DMY, DateRes.FRENCH_MONTHS), (String)null, (String)null, new CurrencyRes[]{new CurrencyRes("€", CurrencyRes.PositiveCurrencyFormat.SPACE_END, CurrencyRes.NegativeCurrencyFormat.SNPC, "#,##0 \"€\";[Red]-#,##0 \"€\"", "#,##0.00 \"€\";[Red]-#,##0.00 \"€\""), new CurrencyRes("€", CurrencyRes.PositiveCurrencyFormat.HEAD_SPACE, CurrencyRes.NegativeCurrencyFormat.SCPN, "[$€-2] #,##0;[Red]-[$€-2] #,##0", "[$€-2] #,##0.00;[Red]-[$€-2] #,##0.00")});
            LANGUAGE_FRENCH=>Self::literal("LANGUAGE_FRENCH",false),
            // java case LANGUAGE_FRENCH_LUXEMBOURG:
            // java return new LangRes(SeparatorRes.SPACE_GROUPER_SEPERATORS, LangRes.NegativeFormat.SN, new DateRes(false, "dd/MM/yyyy", '/', DateRes.ShortDateFmtType.DMY, DateRes.FRENCH_MONTHS), (String)null, (String)null, new CurrencyRes[]{new CurrencyRes("€", CurrencyRes.PositiveCurrencyFormat.SPACE_END, CurrencyRes.NegativeCurrencyFormat.SNPC, "#,##0 \"€\";[Red]-#,##0 \"€\"", "#,##0.00 \"€\";[Red]-#,##0.00 \"€\""), new CurrencyRes("€", CurrencyRes.PositiveCurrencyFormat.HEAD_SPACE, CurrencyRes.NegativeCurrencyFormat.SCPN, "[$€-2] #,##0;[Red]-[$€-2] #,##0", "[$€-2] #,##0.00;[Red]-[$€-2] #,##0.00")});
            LANGUAGE_FRENCH_LUXEMBOURG=>Self::literal("LANGUAGE_FRENCH_LUXEMBOURG",false),
            // java case LANGUAGE_FRENCH_SWISS:
            // java return new LangRes(SeparatorRes.QUOT_GROUPER_SEPERATORS, LangRes.NegativeFormat.SN, new DateRes(false, "dd.MM.yyyy", '.', DateRes.ShortDateFmtType.DMY, DateRes.FRENCH_MONTHS), (String)null, (String)null, new CurrencyRes[]{new CurrencyRes("fr.", CurrencyRes.PositiveCurrencyFormat.HEAD_SPACE, CurrencyRes.NegativeCurrencyFormat.CSN, "\"fr.\" #,##0;[Red]\"fr.\" -#,##0", "\"fr.\" #,##0.00;[Red]\"fr.\" -#,##0.00")});
            LANGUAGE_FRENCH_SWISS=>Self::literal("LANGUAGE_FRENCH_SWISS",false),
            // java case LANGUAGE_FRISIAN_NETHERLANDS:
            // java return new LangRes(SeparatorRes.DOT_GROUPER_SEPERATORS, LangRes.NegativeFormat.SN, new DateRes(false, "d-M-yyyy", '-', DateRes.ShortDateFmtType.DMY, DateRes.FRISIAN_NETHERLANDS_MONTHS), (String)null, (String)null, new CurrencyRes[]{new CurrencyRes("€", CurrencyRes.PositiveCurrencyFormat.HEAD_SPACE, CurrencyRes.NegativeCurrencyFormat.CPSN, "\"€\" #,##0_);[Red](\"€\" #,##0)", "\"€\" #,##0.00_);[Red](\"€\" #,##0.00)"), new CurrencyRes("€", CurrencyRes.PositiveCurrencyFormat.SPACE_END, CurrencyRes.NegativeCurrencyFormat.NSPC, "#,##0_- [$€-1];[Red]#,##0- [$€-1]", "#,##0.00_- [$€-1];[Red]#,##0.00- [$€-1]")});
            LANGUAGE_FRISIAN_NETHERLANDS=>Self::literal("LANGUAGE_FRISIAN_NETHERLANDS",false),
            // java case LANGUAGE_DUTCH_BELGIAN:
            // java return new LangRes(SeparatorRes.DOT_GROUPER_SEPERATORS, LangRes.NegativeFormat.SN, new DateRes(false, "d/MM/yyyy", '/', DateRes.ShortDateFmtType.DMY, DateRes.DUTCH_MONTHS), (String)null, (String)null, new CurrencyRes[]{new CurrencyRes("€", CurrencyRes.PositiveCurrencyFormat.HEAD_SPACE, CurrencyRes.NegativeCurrencyFormat.CPSN, "\"€\" #,##0_);[Red](\"€\" #,##0)", "\"€\" #,##0.00_);[Red](\"€\" #,##0.00)"), new CurrencyRes("€", CurrencyRes.PositiveCurrencyFormat.SPACE_END, CurrencyRes.NegativeCurrencyFormat.NSPC, "#,##0_- [$€-1];[Red]#,##0- [$€-1]", "#,##0.00_- [$€-1];[Red]#,##0.00- [$€-1]")});
            LANGUAGE_DUTCH_BELGIAN=>Self::literal("LANGUAGE_DUTCH_BELGIAN",false),
            // java case LANGUAGE_DUTCH:
            // java return new LangRes(SeparatorRes.DOT_GROUPER_SEPERATORS, LangRes.NegativeFormat.SN, new DateRes(false, "d-M-yyyy", '-', DateRes.ShortDateFmtType.DMY, DateRes.DUTCH_MONTHS), (String)null, (String)null, new CurrencyRes[]{new CurrencyRes("€", CurrencyRes.PositiveCurrencyFormat.HEAD_SPACE, CurrencyRes.NegativeCurrencyFormat.CPSN, "\"€\" #,##0_);[Red](\"€\" #,##0)", "\"€\" #,##0.00_);[Red](\"€\" #,##0.00)"), new CurrencyRes("€", CurrencyRes.PositiveCurrencyFormat.SPACE_END, CurrencyRes.NegativeCurrencyFormat.NSPC, "#,##0_- [$€-1];[Red]#,##0- [$€-1]", "#,##0.00_- [$€-1];[Red]#,##0.00- [$€-1]")});
            LANGUAGE_DUTCH=>Self::literal("LANGUAGE_DUTCH",false),
            // java case LANGUAGE_LUXEMBOURGISH_LUXEMBOURG:
            // java return new LangRes(SeparatorRes.SPACE_GROUPER_SEPERATORS, LangRes.NegativeFormat.SN, new DateRes(false, "dd/MM/yyyy", '/', DateRes.ShortDateFmtType.DMY, DateRes.LUXEMBOURGISH_LUXEMBOURG_MONTHS), (String)null, (String)null, new CurrencyRes[]{new CurrencyRes("€", CurrencyRes.PositiveCurrencyFormat.SPACE_END, CurrencyRes.NegativeCurrencyFormat.SNPC, "#,##0 \"€\";[Red]-#,##0 \"€\"", "#,##0.00 \"€\";[Red]-#,##0.00 \"€\""), new CurrencyRes("€", CurrencyRes.PositiveCurrencyFormat.HEAD_SPACE, CurrencyRes.NegativeCurrencyFormat.SCPN, "[$€-2] #,##0;[Red]-[$€-2] #,##0", "[$€-2] #,##0.00;[Red]-[$€-2] #,##0.00")});
            LANGUAGE_LUXEMBOURGISH_LUXEMBOURG=>Self::literal("LANGUAGE_LUXEMBOURGISH_LUXEMBOURG",false),
            // java case LANGUAGE_PORTUGUESE_BRAZILIAN:
            // java return new LangRes(SeparatorRes.DOT_GROUPER_SEPERATORS, LangRes.NegativeFormat.SN, new DateRes(false, "dd/MM/yyyy", '/', DateRes.ShortDateFmtType.DMY, DateRes.PORTUGUESE_BRAZILIAN_MONTHS), (String)null, (String)null, new CurrencyRes[]{new CurrencyRes("R$", CurrencyRes.PositiveCurrencyFormat.HEAD_SPACE, CurrencyRes.NegativeCurrencyFormat.SCPN, "\"R$\" #,##0;[Red]-\"R$\" #,##0", "\"R$\" #,##0.00;[Red]-\"R$\" #,##0.00")});
            LANGUAGE_PORTUGUESE_BRAZILIAN=>Self::literal("LANGUAGE_PORTUGUESE_BRAZILIAN",false),
            // java case LANGUAGE_PORTUGUESE:
            // java return new LangRes(SeparatorRes.DOT_GROUPER_SEPERATORS, LangRes.NegativeFormat.SN, new DateRes(false, "dd-MM-yyyy", '-', DateRes.ShortDateFmtType.DMY, DateRes.PORTUGUESE_MONTHS), (String)null, (String)null, new CurrencyRes[]{new CurrencyRes("€", CurrencyRes.PositiveCurrencyFormat.SPACE_END, CurrencyRes.NegativeCurrencyFormat.SNPC, "#,##0 \"€\";[Red]-#,##0 \"€\"", "#,##0.00 \"€\";[Red]-#,##0.00 \"€\""), new CurrencyRes("€", CurrencyRes.PositiveCurrencyFormat.HEAD_SPACE, CurrencyRes.NegativeCurrencyFormat.SCPN, "[$€-2] #,##0;[Red]-[$€-2] #,##0", "[$€-2] #,##0.00;[Red]-[$€-2] #,##0.00")});
            LANGUAGE_PORTUGUESE=>Self::literal("LANGUAGE_PORTUGUESE",false),
            // java case LANGUAGE_JAPANESE:
            // java return new LangRes(SeparatorRes.COMMON_SEPERATORS, LangRes.NegativeFormat.SN, new DateRes(true, "yyyy/MM/dd", '/', DateRes.ShortDateFmtType.YMD, (String[])null), (String)null, (String)null, new CurrencyRes[]{new CurrencyRes("¥", CurrencyRes.PositiveCurrencyFormat.HEAD, CurrencyRes.NegativeCurrencyFormat.SCN, "\"¥\"#,##0;[Red]-\"¥\"#,##0", "\"¥\"#,##0.00;[Red]-\"¥\"#,##0.00"), new CurrencyRes("$", CurrencyRes.PositiveCurrencyFormat.HEAD, CurrencyRes.NegativeCurrencyFormat.SCN, "$#,##0;[Red]-$#,##0", "$#,##0.00;[Red]-$#,##0.00")}, new String[][]{{"日", "月", "火", "水", "木", "金", "土"}, {"日曜日", "月曜日", "火曜日", "水曜日", "木曜日", "金曜日", "土曜日"}, {"1月", "2月", "3月", "4月", "5月", "6月", "7月", "8月", "9月", "10月", "11月", "12月"}, {"第1四半期", "第2四半期", "第3四半期", "第4四半期"}, {"睦月", "如月", "弥生", "卯月", "皐月", "水無月", "文月", "葉月", "長月", "神無月", "霜月", "師走"}, {"子", "丑", "寅", "卯", "辰", "巳", "午", "未", "申", "酉", "戌", "亥"}});
            LANGUAGE_JAPANESE=>Self::literal("LANGUAGE_JAPANESE",true),
            // java case LANGUAGE_SWEDISH_FINLAND:
            // java return new LangRes(SeparatorRes.SPACE_GROUPER_SEPERATORS, LangRes.NegativeFormat.SN, new DateRes(false, "d.M.yyyy", '.', DateRes.ShortDateFmtType.DMY, DateRes.SWEDISH_MONTHS), (String)null, (String)null, new CurrencyRes[]{new CurrencyRes("€", CurrencyRes.PositiveCurrencyFormat.SPACE_END, CurrencyRes.NegativeCurrencyFormat.SNPC, "#,##0 \"€\";[Red]-#,##0 \"€\"", "#,##0.00 \"€\";[Red]-#,##0.00 \"€\""), new CurrencyRes("€", CurrencyRes.PositiveCurrencyFormat.HEAD_SPACE, CurrencyRes.NegativeCurrencyFormat.SCPN, "[$€-2] #,##0;[Red]-[$€-2] #,##0", "[$€-2] #,##0.00;[Red]-[$€-2] #,##0.00")});
            LANGUAGE_SWEDISH_FINLAND=>Self::literal("LANGUAGE_SWEDISH_FINLAND",false),
            // java case LANGUAGE_SWEDISH:
            // java return new LangRes(SeparatorRes.SPACE_GROUPER_SEPERATORS, LangRes.NegativeFormat.SN, new DateRes(false, "yyyy-MM-dd", '-', DateRes.ShortDateFmtType.YMD, DateRes.SWEDISH_MONTHS), (String)null, (String)null, new CurrencyRes[]{new CurrencyRes("kr", CurrencyRes.PositiveCurrencyFormat.SPACE_END, CurrencyRes.NegativeCurrencyFormat.SNPC, "#,##0 \"kr\";[Red]-#,##0 \"kr\"", "#,##0.00 \"kr\";[Red]-#,##0.00 \"kr\"")});
            LANGUAGE_SWEDISH=>Self::literal("LANGUAGE_SWEDISH",false),
            // java case LANGUAGE_SPANISH_ARGENTINA:
            // java return new LangRes(SeparatorRes.DOT_GROUPER_SEPERATORS, LangRes.NegativeFormat.SN, new DateRes(false, "dd/MM/yyyy", '/', DateRes.ShortDateFmtType.DMY, DateRes.SPANISH_MONTHS), "a.m.", "p.m.", new CurrencyRes[]{new CurrencyRes("$", CurrencyRes.PositiveCurrencyFormat.HEAD_SPACE, CurrencyRes.NegativeCurrencyFormat.CSN, "$ #,##0;[Red]$-#,##0", "$ #,##0.00;[Red]$-#,##0.00")});
            LANGUAGE_SPANISH_ARGENTINA=>Self::literal("LANGUAGE_SPANISH_ARGENTINA",false),
            // java case LANGUAGE_SPANISH_PARAGUAY:
            // java return new LangRes(SeparatorRes.DOT_GROUPER_SEPERATORS, LangRes.NegativeFormat.SN, new DateRes(false, "dd/MM/yyyy", '/', DateRes.ShortDateFmtType.DMY, DateRes.SPANISH_MONTHS), "a.m.", "p.m.", new CurrencyRes[]{new CurrencyRes("Gs", CurrencyRes.PositiveCurrencyFormat.HEAD_SPACE, CurrencyRes.NegativeCurrencyFormat.BRACKET_HEAD_SPACE, "\"Gs\" #,##0;[Red](\"Gs\" #,##0)", "\"Gs\" #,##0.00;[Red](\"Gs\" #,##0.00)")});
            LANGUAGE_SPANISH_PARAGUAY=>Self::literal("LANGUAGE_SPANISH_PARAGUAY",false),
            // java case LANGUAGE_SPANISH_PANAMA:
            // java return new LangRes(SeparatorRes.COMMON_SEPERATORS, LangRes.NegativeFormat.SN, new DateRes(false, "MM/dd/yyyy", '/', DateRes.ShortDateFmtType.MDY, DateRes.SPANISH_MONTHS), "a.m.", "p.m.", new CurrencyRes[]{new CurrencyRes("B/.", CurrencyRes.PositiveCurrencyFormat.HEAD_SPACE, CurrencyRes.NegativeCurrencyFormat.BRACKET_HEAD_SPACE, "\"B/.\" #,##0;[Red](\"B/.\" #,##0)", "\"B/.\" #,##0.00;[Red](\"B/.\" #,##0.00)")});
            LANGUAGE_SPANISH_PANAMA=>Self::literal("LANGUAGE_SPANISH_PANAMA",false),
            // java case LANGUAGE_SPANISH_PUERTO_RICO:
            // java return new LangRes(SeparatorRes.COMMON_SEPERATORS, LangRes.NegativeFormat.SN, new DateRes(false, "dd/MM/yyyy", '/', DateRes.ShortDateFmtType.DMY, DateRes.SPANISH_MONTHS), "a.m.", "p.m.", new CurrencyRes[]{new CurrencyRes("$", CurrencyRes.PositiveCurrencyFormat.HEAD_SPACE, CurrencyRes.NegativeCurrencyFormat.BRACKET_HEAD_SPACE, "$ #,##0;[Red]($ #,##0)", "$ #,##0.00;[Red]($ #,##0.00)")});
            LANGUAGE_SPANISH_PUERTO_RICO=>Self::literal("LANGUAGE_SPANISH_PUERTO_RICO",false),
            // java case LANGUAGE_SPANISH_BOLIVIA:
            // java return new LangRes(SeparatorRes.DOT_GROUPER_SEPERATORS, LangRes.NegativeFormat.SN, new DateRes(false, "dd/MM/yyyy", '/', DateRes.ShortDateFmtType.DMY, DateRes.SPANISH_MONTHS), "a.m.", "p.m.", new CurrencyRes[]{new CurrencyRes("$b", CurrencyRes.PositiveCurrencyFormat.HEAD_SPACE, CurrencyRes.NegativeCurrencyFormat.BRACKET_HEAD_SPACE, "\"$b\" #,##0;[Red](\"$b\" #,##0)", "\"$b\" #,##0.00;[Red](\"$b\" #,##0.00)")});
            LANGUAGE_SPANISH_BOLIVIA=>Self::literal("LANGUAGE_SPANISH_BOLIVIA",false),
            // java case LANGUAGE_SPANISH_DOMINICAN_REPUBLIC:
            // java return new LangRes(SeparatorRes.COMMON_SEPERATORS, LangRes.NegativeFormat.SN, new DateRes(false, "dd/MM/yyyy", '/', DateRes.ShortDateFmtType.DMY, DateRes.SPANISH_MONTHS), "a.m.", "p.m.", new CurrencyRes[]{new CurrencyRes("RD$", CurrencyRes.PositiveCurrencyFormat.HEAD, CurrencyRes.NegativeCurrencyFormat.BRACKET_HEAD, "\"RD$\"#,##0;[Red](\"RD$\"#,##0)", "\"RD$\"#,##0.00;[Red](\"RD$\"#,##0.00)")});
            LANGUAGE_SPANISH_DOMINICAN_REPUBLIC=>Self::literal("LANGUAGE_SPANISH_DOMINICAN_REPUBLIC",false),
            // java case LANGUAGE_SPANISH_ECUADOR:
            // java return new LangRes(SeparatorRes.DOT_GROUPER_SEPERATORS, LangRes.NegativeFormat.SN, new DateRes(false, "dd/MM/yyyy", '/', DateRes.ShortDateFmtType.DMY, DateRes.SPANISH_MONTHS), (String)null, (String)null, new CurrencyRes[]{new CurrencyRes("$", CurrencyRes.PositiveCurrencyFormat.HEAD_SPACE, CurrencyRes.NegativeCurrencyFormat.BRACKET_HEAD_SPACE, "$ #,##0;[Red]($ #,##0)", "$ #,##0.00;[Red]($ #,##0.00)")});
            LANGUAGE_SPANISH_ECUADOR=>Self::literal("LANGUAGE_SPANISH_ECUADOR",false),
            // java case LANGUAGE_SPANISH_COLOMBIA:
            // java return new LangRes(SeparatorRes.DOT_GROUPER_SEPERATORS, LangRes.NegativeFormat.SN, new DateRes(false, "dd/MM/yyyy", '/', DateRes.ShortDateFmtType.DMY, DateRes.SPANISH_MONTHS), "a.m.", "p.m.", new CurrencyRes[]{new CurrencyRes("$", CurrencyRes.PositiveCurrencyFormat.HEAD_SPACE, CurrencyRes.NegativeCurrencyFormat.BRACKET_HEAD_SPACE, "$ #,##0;[Red]($ #,##0)", "$ #,##0.00;[Red]($ #,##0.00)")});
            LANGUAGE_SPANISH_COLOMBIA=>Self::literal("LANGUAGE_SPANISH_COLOMBIA",false),
            // java case LANGUAGE_SPANISH_COSTARICA:
            // java return new LangRes(SeparatorRes.DOT_GROUPER_SEPERATORS, LangRes.NegativeFormat.SN, new DateRes(false, "dd/MM/yyyy", '/', DateRes.ShortDateFmtType.DMY, DateRes.SPANISH_MONTHS), "a.m.", "p.m.", new CurrencyRes[]{new CurrencyRes("₡", CurrencyRes.PositiveCurrencyFormat.HEAD, CurrencyRes.NegativeCurrencyFormat.BRACKET_HEAD, "\"₡\"#,##0;[Red](\"₡\"#,##0)", "\"₡\"#,##0.00;[Red](\"₡\"#,##0.00)")});
            LANGUAGE_SPANISH_COSTARICA=>Self::literal("LANGUAGE_SPANISH_COSTARICA",false),
            // java case LANGUAGE_SPANISH_HONDURAS:
            // java return new LangRes(SeparatorRes.COMMON_SEPERATORS, LangRes.NegativeFormat.SN, new DateRes(false, "dd/MM/yyyy", '/', DateRes.ShortDateFmtType.DMY, DateRes.SPANISH_MONTHS), "a.m.", "p.m.", new CurrencyRes[]{new CurrencyRes("L.", CurrencyRes.PositiveCurrencyFormat.HEAD_SPACE, CurrencyRes.NegativeCurrencyFormat.CPSN, "\"L.\" #,##0;[Red]\"L.\" -#,##0", "\"L.\" #,##0.00;[Red]\"L.\" -#,##0.00")});
            LANGUAGE_SPANISH_HONDURAS=>Self::literal("LANGUAGE_SPANISH_HONDURAS",false),
            // java case LANGUAGE_SPANISH_UNITED_STATES:
            // java return new LangRes(SeparatorRes.COMMON_SEPERATORS, LangRes.NegativeFormat.SN, new DateRes(false, "M/d/yyyy", '/', DateRes.ShortDateFmtType.DMY, DateRes.SPANISH_MONTHS), "AM", "PM", new CurrencyRes[]{new CurrencyRes("$", CurrencyRes.PositiveCurrencyFormat.HEAD, CurrencyRes.NegativeCurrencyFormat.BRACKET_HEAD, "$#,##0;[Red]($#,##0)", "$#,##0.00;[Red]($#,##0.00)")});
            LANGUAGE_SPANISH_UNITED_STATES=>Self::literal("LANGUAGE_SPANISH_UNITED_STATES",false),
            // java case LANGUAGE_SPANISH_PERU:
            // java return new LangRes(SeparatorRes.COMMON_SEPERATORS, LangRes.NegativeFormat.SN, new DateRes(false, "dd/MM/yyyy", '/', DateRes.ShortDateFmtType.DMY, DateRes.SPANISH_MONTHS), "a.m.", "p.m.", new CurrencyRes[]{new CurrencyRes("S/.", CurrencyRes.PositiveCurrencyFormat.HEAD_SPACE, CurrencyRes.NegativeCurrencyFormat.CPSN, "\"S/.\" #,##0;[Red]\"S/.\" -#,##0", "\"S/.\" #,##0.00;[Red]\"S/.\" -#,##0.00")});
            LANGUAGE_SPANISH_PERU=>Self::literal("LANGUAGE_SPANISH_PERU",false),
            // java case LANGUAGE_SPANISH_MEXICAN:
            // java return new LangRes(SeparatorRes.COMMON_SEPERATORS, LangRes.NegativeFormat.SN, new DateRes(true, "dd/MM/yyyy", '/', DateRes.ShortDateFmtType.DMY, DateRes.SPANISH_MONTHS), "a.m.", "p.m.", new CurrencyRes[]{new CurrencyRes("$", CurrencyRes.PositiveCurrencyFormat.HEAD, CurrencyRes.NegativeCurrencyFormat.SCN, "$#,##0;[Red]-$#,##0", "$#,##0.00;[Red]-$#,##0.00")});
            LANGUAGE_SPANISH_MEXICAN=>Self::literal("LANGUAGE_SPANISH_MEXICAN",false),
            // java case LANGUAGE_SPANISH_NICARAGUA:
            // java return new LangRes(SeparatorRes.COMMON_SEPERATORS, LangRes.NegativeFormat.SN, new DateRes(false, "dd/MM/yyyy", '/', DateRes.ShortDateFmtType.DMY, DateRes.SPANISH_MONTHS), "a.m.", "p.m.", new CurrencyRes[]{new CurrencyRes("C$", CurrencyRes.PositiveCurrencyFormat.HEAD_SPACE, CurrencyRes.NegativeCurrencyFormat.BRACKET_HEAD_SPACE, "\"C$\" #,##0;[Red](\"C$\" #,##0)", "\"C$\" #,##0.00;[Red](\"C$\" #,##0.00)")});
            LANGUAGE_SPANISH_NICARAGUA=>Self::literal("LANGUAGE_SPANISH_NICARAGUA",false),
            // java case LANGUAGE_SPANISH_EL_SALVADOR:
            // java return new LangRes(SeparatorRes.COMMON_SEPERATORS, LangRes.NegativeFormat.SN, new DateRes(false, "dd/MM/yyyy", '/', DateRes.ShortDateFmtType.DMY, DateRes.SPANISH_MONTHS), "a.m.", "p.m.", new CurrencyRes[]{new CurrencyRes("$", CurrencyRes.PositiveCurrencyFormat.HEAD, CurrencyRes.NegativeCurrencyFormat.BRACKET_HEAD, "$#,##0;[Red]($#,##0)", "$#,##0.00;[Red]($#,##0.00)")});
            LANGUAGE_SPANISH_EL_SALVADOR=>Self::literal("LANGUAGE_SPANISH_EL_SALVADOR",false),
            // java case LANGUAGE_SPANISH_GUATEMALA:
            // java return new LangRes(SeparatorRes.COMMON_SEPERATORS, LangRes.NegativeFormat.SN, new DateRes(false, "dd/MM/yyyy", '/', DateRes.ShortDateFmtType.DMY, DateRes.SPANISH_MONTHS), "a.m.", "p.m.", new CurrencyRes[]{new CurrencyRes("Q", CurrencyRes.PositiveCurrencyFormat.HEAD, CurrencyRes.NegativeCurrencyFormat.BRACKET_HEAD, "\"Q\"#,##0;[Red](\"Q\"#,##0)", "\"Q\"#,##0.00;[Red](\"Q\"#,##0.00)")});
            LANGUAGE_SPANISH_GUATEMALA=>Self::literal("LANGUAGE_SPANISH_GUATEMALA",false),
            // java case LANGUAGE_SPANISH_VENEZUELA:
            // java return new LangRes(SeparatorRes.DOT_GROUPER_SEPERATORS, LangRes.NegativeFormat.SN, new DateRes(false, "dd/MM/yyyy", '/', DateRes.ShortDateFmtType.DMY, DateRes.SPANISH_MONTHS), "a.m.", "p.m.", new CurrencyRes[]{new CurrencyRes("Bs. F", CurrencyRes.PositiveCurrencyFormat.HEAD_SPACE, CurrencyRes.NegativeCurrencyFormat.CPSN, "Bs. F #,##0;[Red]Bs. F -#,##0", "Bs. F #,##0.00;[Red]Bs. F -#,##0.00")});
            LANGUAGE_SPANISH_VENEZUELA=>Self::literal("LANGUAGE_SPANISH_VENEZUELA",false),
            // java case LANGUAGE_SPANISH_URUGUAY:
            // java return new LangRes(SeparatorRes.DOT_GROUPER_SEPERATORS, LangRes.NegativeFormat.SN, new DateRes(false, "dd/MM/yyyy", '/', DateRes.ShortDateFmtType.DMY, DateRes.SPANISH_MONTHS), "a.m.", "p.m.", new CurrencyRes[]{new CurrencyRes("$U", CurrencyRes.PositiveCurrencyFormat.HEAD_SPACE, CurrencyRes.NegativeCurrencyFormat.BRACKET_HEAD_SPACE, "\"$U\" #,##0;[Red](\"$U\" #,##0)", "\"$U\" #,##0.00;[Red](\"$U\" #,##0.00)")});
            LANGUAGE_SPANISH_URUGUAY=>Self::literal("LANGUAGE_SPANISH_URUGUAY",false),
            // java case LANGUAGE_SPANISH:
            // java return new LangRes(SeparatorRes.DOT_GROUPER_SEPERATORS, LangRes.NegativeFormat.SN, new DateRes(false, "dd/MM/yyyy", '/', DateRes.ShortDateFmtType.DMY, DateRes.SPANISH_MONTHS), (String)null, (String)null, new CurrencyRes[]{new CurrencyRes("€", CurrencyRes.PositiveCurrencyFormat.SPACE_END, CurrencyRes.NegativeCurrencyFormat.SNPC, "#,##0 \"€\";[Red]-#,##0 \"€\"", "#,##0.00 \"€\";[Red]-#,##0.00 \"€\""), new CurrencyRes("€", CurrencyRes.PositiveCurrencyFormat.HEAD_SPACE, CurrencyRes.NegativeCurrencyFormat.SCPN, "[$€-2] #,##0;[Red]-[$€-2] #,##0", "[$€-2] #,##0.00;[Red]-[$€-2] #,##0.00")});
            LANGUAGE_SPANISH=>Self::literal("LANGUAGE_SPANISH",false),
            // java case LANGUAGE_SPANISH_CHILE:
            // java return new LangRes(SeparatorRes.DOT_GROUPER_SEPERATORS, LangRes.NegativeFormat.SN, new DateRes(false, "dd-MM-yyyy", '-', DateRes.ShortDateFmtType.DMY, DateRes.SPANISH_MONTHS), (String)null, (String)null, new CurrencyRes[]{new CurrencyRes("$", CurrencyRes.PositiveCurrencyFormat.HEAD_SPACE, CurrencyRes.NegativeCurrencyFormat.SCPN, "$ #,##0;[Red]-$ #,##0", "$ #,##0.00;[Red]-$ #,##0.00")});
            LANGUAGE_SPANISH_CHILE=>Self::literal("LANGUAGE_SPANISH_CHILE",false),
            // java case LANGUAGE_HEBREW:
            // java return new LangRes(SeparatorRes.COMMON_SEPERATORS, LangRes.NegativeFormat.SN, new DateRes(false, "dd/MM/yyyy", '/', DateRes.ShortDateFmtType.DMY, DateRes.HEBREW_MONTHS), "AM", "PM", new CurrencyRes[]{new CurrencyRes("₪", CurrencyRes.PositiveCurrencyFormat.HEAD_SPACE, CurrencyRes.NegativeCurrencyFormat.CSN, "₪ #,##0;[Red]₪ -#,##0", "₪ #,##0.00;[Red]₪ -#,##0.00")});
            LANGUAGE_HEBREW=>Self::literal("LANGUAGE_HEBREW",false),
            // java case LANGUAGE_ITALIAN_SWISS:
            // java return new LangRes(SeparatorRes.QUOT_GROUPER_SEPERATORS, LangRes.NegativeFormat.SN, new DateRes(false, "dd.MM.yyyy", '.', DateRes.ShortDateFmtType.DMY, DateRes.ITALIAN_MONTHS), (String)null, (String)null, new CurrencyRes[]{new CurrencyRes("fr.", CurrencyRes.PositiveCurrencyFormat.HEAD_SPACE, CurrencyRes.NegativeCurrencyFormat.CPSN, "\"fr.\" #,##0;[Red]\"fr.\" -#,##0", "\"fr.\" #,##0.00;[Red]\"fr.\" -#,##0.00")});
            LANGUAGE_ITALIAN_SWISS=>Self::literal("LANGUAGE_ITALIAN_SWISS",false),
            // java case LANGUAGE_ITALIAN:
            // java return new LangRes(SeparatorRes.DOT_GROUPER_SEPERATORS, LangRes.NegativeFormat.SN, new DateRes(false, "dd/MM/yyyy", '/', DateRes.ShortDateFmtType.DMY, DateRes.ITALIAN_MONTHS), (String)null, (String)null, new CurrencyRes[]{new CurrencyRes("€", CurrencyRes.PositiveCurrencyFormat.HEAD_SPACE, CurrencyRes.NegativeCurrencyFormat.SCPN, "\"€\" #,##0;[Red]-\"€\" #,##0", "\"€\" #,##0.00;[Red]-\"€\" #,##0.00"), new CurrencyRes("€", CurrencyRes.PositiveCurrencyFormat.SPACE_END, CurrencyRes.NegativeCurrencyFormat.SNPC, "#,##0 [$€-1];[Red]-#,##0 [$€-1]", "#,##0.00 [$€-1];[Red]-#,##0.00 [$€-1]")});
            LANGUAGE_ITALIAN=>Self::literal("LANGUAGE_ITALIAN",false),
            // java case LANGUAGE_ENGLISH_EIRE:
            // java return new LangRes(SeparatorRes.COMMON_SEPERATORS, LangRes.NegativeFormat.SN, new DateRes(false, "dd/MM/yyyy", '/', DateRes.ShortDateFmtType.DMY, (String[])null), (String)null, (String)null, new CurrencyRes[]{new CurrencyRes("€", CurrencyRes.PositiveCurrencyFormat.HEAD, CurrencyRes.NegativeCurrencyFormat.SCN, "\"€\"#,##0;[Red]-\"€\"#,##0", "\"€\"#,##0.00;[Red]-\"€\"#,##0.00"), new CurrencyRes("€", CurrencyRes.PositiveCurrencyFormat.SPACE_END, CurrencyRes.NegativeCurrencyFormat.SNPC, "#,##0 [$€-1];[Red]-#,##0 [$€-1]", "#,##0.00 [$€-1];[Red]-#,##0.00 [$€-1]")});
            LANGUAGE_ENGLISH_EIRE=>Self::literal("LANGUAGE_ENGLISH_EIRE",false),
            // java case LANGUAGE_ENGLISH:
            // java case LANGUAGE_ENGLISH_US:
            // java return new LangRes(SeparatorRes.COMMON_SEPERATORS, LangRes.NegativeFormat.SN, new DateRes(true, "M/d/yyyy", '/', DateRes.ShortDateFmtType.MDY, (String[])null), (String)null, (String)null, new CurrencyRes[]{new CurrencyRes("$", CurrencyRes.PositiveCurrencyFormat.HEAD, CurrencyRes.NegativeCurrencyFormat.BRACKET_HEAD, "$#,##0_);[Red]($#,##0)", "$#,##0.00_);[Red]($#,##0.00)")});
            LANGUAGE_ENGLISH|LANGUAGE_ENGLISH_US=>Self::literal("LANGUAGE_ENGLISH_US",false),
            // java case LANGUAGE_ENGLISH_UK:
            // java return new LangRes(SeparatorRes.COMMON_SEPERATORS, LangRes.NegativeFormat.SN, new DateRes(false, "dd/MM/yyyy", '/', DateRes.ShortDateFmtType.DMY, (String[])null), "AM", "PM", new CurrencyRes[]{new CurrencyRes("£", CurrencyRes.PositiveCurrencyFormat.HEAD, CurrencyRes.NegativeCurrencyFormat.SCN, "\"£\"#,##0;[Red]-\"£\"#,##0", "\"£\"#,##0.00;[Red]-\"£\"#,##0.00")});
            LANGUAGE_ENGLISH_UK=>Self::literal("LANGUAGE_ENGLISH_UK",false),
            // java case LANGUAGE_CHINESE_MACAU:
            // java return new LangRes(SeparatorRes.COMMON_SEPERATORS, LangRes.NegativeFormat.SN, new DateRes(true, "d/M/yyyy", '/', DateRes.ShortDateFmtType.DMY, DateRes.CHINESE_MONTHS), (String)null, (String)null, new CurrencyRes[]{new CurrencyRes("MOP", CurrencyRes.PositiveCurrencyFormat.HEAD, CurrencyRes.NegativeCurrencyFormat.BRACKET_HEAD, "\"MOP\"#,##0_);[Red](\"MOP\"#,##0)", "\"MOP\"#,##0.00_);[Red](\"MOP\"#,##0.00)"), new CurrencyRes("NT$", CurrencyRes.PositiveCurrencyFormat.HEAD, CurrencyRes.NegativeCurrencyFormat.SCN, "\"NT$\"#,##0;[Red]-\"NT$\"#,##0", "\"NT$\"#,##0.00;[Red]-\"NT$\"#,##0.00"), new CurrencyRes("US$", CurrencyRes.PositiveCurrencyFormat.HEAD, CurrencyRes.NegativeCurrencyFormat.BRACKET_HEAD, "\"US$\"#,##0_);[Red](\"US$\"#,##0)", "\"US$\"#,##0.00_);[Red](\"US$\"#,##0.00)"), new CurrencyRes("$", CurrencyRes.PositiveCurrencyFormat.HEAD, CurrencyRes.NegativeCurrencyFormat.BRACKET_HEAD, "\"US$\"#,##0_);[Red](\"US$\"#,##0)", "\"US$\"#,##0.00_);[Red](\"US$\"#,##0.00)")});
            LANGUAGE_CHINESE_MACAU=>Self::literal("LANGUAGE_CHINESE_MACAU",false),
            // java case LANGUAGE_CHINESE_TRADITIONAL:
            // java return new LangRes(SeparatorRes.COMMON_SEPERATORS, LangRes.NegativeFormat.SN, new DateRes(true, "yyyy/M/d", '/', DateRes.ShortDateFmtType.YMD, DateRes.CHINESE_MONTHS), (String)null, (String)null, new CurrencyRes[]{new CurrencyRes("NT$", CurrencyRes.PositiveCurrencyFormat.HEAD, CurrencyRes.NegativeCurrencyFormat.SCN, "\"NT$\"#,##0;[Red]-\"NT$\"#,##0", "\"NT$\"#,##0.00;[Red]-\"NT$\"#,##0.00"), new CurrencyRes("US$", CurrencyRes.PositiveCurrencyFormat.HEAD, CurrencyRes.NegativeCurrencyFormat.BRACKET_HEAD, "\"US$\"#,##0_);[Red](\"US$\"#,##0)", "\"US$\"#,##0.00_);[Red](\"US$\"#,##0.00)"), new CurrencyRes("$", CurrencyRes.PositiveCurrencyFormat.HEAD, CurrencyRes.NegativeCurrencyFormat.BRACKET_HEAD, "$#,##0_);[Red]($#,##0)", "$#,##0.00_);[Red]($#,##0.00)")});
            LANGUAGE_CHINESE_TRADITIONAL=>Self::literal("LANGUAGE_CHINESE_TRADITIONAL",false),
            // java case LANGUAGE_CHINESE_HONGKONG:
            // java return new LangRes(SeparatorRes.COMMON_SEPERATORS, LangRes.NegativeFormat.SN, new DateRes(true, "d/M/yyyy", '/', DateRes.ShortDateFmtType.DMY, DateRes.CHINESE_MONTHS), (String)null, (String)null, new CurrencyRes[]{new CurrencyRes("HK$", CurrencyRes.PositiveCurrencyFormat.HEAD, CurrencyRes.NegativeCurrencyFormat.BRACKET_HEAD, "\"HK$\"#,##0_);[Red](\"HK$\"#,##0)", "\"HK$\"#,##0.00_);[Red](\"HK$\"#,##0.00)"), new CurrencyRes("US$", CurrencyRes.PositiveCurrencyFormat.HEAD, CurrencyRes.NegativeCurrencyFormat.BRACKET_HEAD, "\"US$\"#,##0_);[Red](\"US$\"#,##0)", "\"US$\"#,##0.00_);[Red](\"US$\"#,##0.00)"), new CurrencyRes("$", CurrencyRes.PositiveCurrencyFormat.HEAD, CurrencyRes.NegativeCurrencyFormat.BRACKET_HEAD, "\"US$\"#,##0_);[Red](\"US$\"#,##0)", "\"US$\"#,##0.00_);[Red](\"US$\"#,##0.00)")});
            LANGUAGE_CHINESE_HONGKONG=>Self::literal("LANGUAGE_CHINESE_HONGKONG",false),
            // java case LANGUAGE_CHINESE_SINGAPORE:
            // java return new LangRes(SeparatorRes.COMMON_SEPERATORS, LangRes.NegativeFormat.SN, new DateRes(true, "d/M/yyyy", '/', DateRes.ShortDateFmtType.DMY, DateRes.CHINESE_MONTHS), "上午", "下午", new CurrencyRes[]{new CurrencyRes("$", CurrencyRes.PositiveCurrencyFormat.HEAD, CurrencyRes.NegativeCurrencyFormat.BRACKET_HEAD, "$#,##0_);[Red]($#,##0)", "$#,##0.00_);[Red]($#,##0.00)")}, new String[][]{{"日", "一", "二", "三", "四", "五", "六"}, {"星期天", "星期一", "星期二", "星期三", "星期四", "星期五", "星期六"}, {"星期日", "星期一", "星期二", "星期三", "星期四", "星期五", "星期六"}, {"一月", "二月", "三月", "四月", "五月", "六月", "七月", "八月", "九月", "十月", "十一月", "十二月"}, {"第一季", "第二季", "第三季", "第四季"}, {"正月", "二月", "三月", "四月", "五月", "六月", "七月", "八月", "九月", "十月", "十一月", "腊月"}, {"子", "丑", "寅", "卯", "辰", "巳", "午", "未", "申", "酉", "戌", "亥"}, {"甲", "乙", "丙", "丁", "戊", "己", "庚", "辛", "壬", "癸"}, {"Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"}, {"Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday"}, {"Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"}, {"January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"}});
            LANGUAGE_CHINESE_SINGAPORE=>Self::literal("LANGUAGE_CHINESE_SINGAPORE",true),
            // java case LANGUAGE_CHINESE:
            // java case LANGUAGE_CHINESE_SIMPLIFIED:
            // java return new LangRes(SeparatorRes.COMMON_SEPERATORS, LangRes.NegativeFormat.SN, new DateRes(true, "yyyy/M/d", '/', DateRes.ShortDateFmtType.YMD, DateRes.CHINESE_MONTHS), (String)null, (String)null, new CurrencyRes[]{new CurrencyRes("¥", CurrencyRes.PositiveCurrencyFormat.HEAD, CurrencyRes.NegativeCurrencyFormat.CSN, "\"¥\"#,##0;[Red]\"¥\"-#,##0", "\"¥\"#,##0.00;[Red]\"¥\"-#,##0.00"), new CurrencyRes("￥", CurrencyRes.PositiveCurrencyFormat.HEAD, CurrencyRes.NegativeCurrencyFormat.CSN, "\"￥\"#,##0;[Red]\"￥\"-#,##0", "\"￥\"#,##0.00;[Red]\"￥\"-#,##0.00"), new CurrencyRes("$", CurrencyRes.PositiveCurrencyFormat.HEAD, CurrencyRes.NegativeCurrencyFormat.BRACKET_HEAD, "$#,##0_);[Red]($#,##0)", "$#,##0.00_);[Red]($#,##0.00)")}, new String[][]{{"日", "一", "二", "三", "四", "五", "六"}, {"星期天", "星期一", "星期二", "星期三", "星期四", "星期五", "星期六"}, {"星期日", "星期一", "星期二", "星期三", "星期四", "星期五", "星期六"}, {"一月", "二月", "三月", "四月", "五月", "六月", "七月", "八月", "九月", "十月", "十一月", "十二月"}, {"第一季", "第二季", "第三季", "第四季"}, {"正月", "二月", "三月", "四月", "五月", "六月", "七月", "八月", "九月", "十月", "十一月", "腊月"}, {"子", "丑", "寅", "卯", "辰", "巳", "午", "未", "申", "酉", "戌", "亥"}, {"甲", "乙", "丙", "丁", "戊", "己", "庚", "辛", "壬", "癸"}, {"Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"}, {"Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday"}, {"Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"}, {"January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"}});
            LANGUAGE_CHINESE|LANGUAGE_CHINESE_SIMPLIFIED=>Self::literal("LANGUAGE_CHINESE_SIMPLIFIED",true),
            // java default:
            // java return new LangRes(SeparatorRes.COMMON_SEPERATORS, LangRes.NegativeFormat.SN, new DateRes(true, "d/M/yyyy", '/', DateRes.ShortDateFmtType.DMY, (String[])null), (String)null, (String)null, new CurrencyRes[]{new CurrencyRes("$", CurrencyRes.PositiveCurrencyFormat.HEAD, CurrencyRes.NegativeCurrencyFormat.BRACKET_HEAD, "$#,##0_);[Red]($#,##0)", "$#,##0.00_);[Red]($#,##0.00)")});
            _=>Self::literal("LANGUAGE_DONTKNOW",false),
        }
    }
    // rust: immutable literal transport only. These fields are constructor
    // arguments from the unchanged Java resource class, not recognised values.
    fn literal(name:&str,custom:bool)->Self{
        static LITERALS:std::sync::LazyLock<serde_json::Value>=std::sync::LazyLock::new(||serde_json::from_str(include_str!("resources/lang-res.json")).unwrap());
        let p:Self=serde_json::from_value(LITERALS[name].clone()).expect("original LangRes constructor literals");
        let a=p.SEPERATORS;let sep=SeparatorRes::SeparatorRes(a.GROUP_SEPARATOR,a.DECIMAL_SEPARATOR,a.PARAMETER_SEPARATOR,a.UNION_SEPARATOR,a.CONST_ARRAY_SEPARATOR,a.CONST_ARRAY_ROW_SEPARATOR);
        let a=p.DATE;let date=DateRes::DateRes(a.SUPPORT_MMMDY_DATE,a.SHORT_DATE_NUMFMT,a.PREFER_SEPERATOR_CHAR,a.SHORT_DATEFMT_TYPE,a.MONTHS);
        let currencies=p.CURRENCIES.into_iter().map(|a|CurrencyRes::CurrencyRes(a.CURRENCY_CHAR,a.POSITIVE_CURRENCY_FORMAT,a.NEGATIVE_CURRENCY_FORMAT,a.CURRENCY_NUMFMT,a.CURRENCY_NUMFMT_DECIMAL)).collect();
        if custom{Self::LangRes__2(sep,p.NEGATIVE_FORMAT,date,p.AM,p.PM,currencies,p.CUSTOMSERIES)}else{Self::LangRes(sep,p.NEGATIVE_FORMAT,date,p.AM,p.PM,currencies)}
    }
}
// java public static enum NegativeFormat { BRACKET, SN, SPN, NS, NPS; }
#[derive(Clone,Copy,PartialEq,Eq,serde::Serialize,serde::Deserialize)]
pub enum NegativeFormat{BRACKET,SN,SPN,NS,NPS}
