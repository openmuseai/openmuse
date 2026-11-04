use crate::{java_runtime::Calendar,cn::wps::et::ss::{constant::RecogniseResult::RecogniseResult,multlang::{LangRes::LangRes,res::DateRes::ShortDateFmtType}}};
use kfc::cn::wps::kfc::numfmt::util::DateUtil::DateUtil;
use util::org::apache::poi::util::LanguageType::*;
use std::sync::LazyLock;
// java public class DateRecognisor {
pub struct DateRecognisor;
// java private static final String en_months = "jan|january|feb|february|mar|march|apr|april|may|jun|june|jul|july|aug|august|sep|september|oct|october|nov|november|dec|december";
const en_months:&str="jan|january|feb|february|mar|march|apr|april|may|jun|june|jul|july|aug|august|sep|september|oct|october|nov|november|dec|december";
// java private static final String[] en_mprefixs = new String[]{"jan", "feb", "mar", "apr", "may", "jun", "jul", "aug", "sep", "oct", "nov", "dec"};
const en_mprefixs:[&str;12]=["jan","feb","mar","apr","may","jun","jul","aug","sep","oct","nov","dec"];
// java static Pattern zh_date_ptrn1 = Pattern.compile("(\\d{1,2})\\s*月\\s*(\\d{1,2})\\s*日");
static zh_date_ptrn1:LazyLock<regex::Regex>=LazyLock::new(||pattern(r"(\d{1,2})\s*月\s*(\d{1,2})\s*日",false));
// java static Pattern zh_date_ptrn2 = Pattern.compile("(\\d{1,4})\\s*年\\s*(\\d{1,2})\\s*月");
static zh_date_ptrn2:LazyLock<regex::Regex>=LazyLock::new(||pattern(r"(\d{1,4})\s*年\s*(\d{1,2})\s*月",false));
// java static Pattern zh_date_ptrn3 = Pattern.compile("(\\d{1,4})\\s*年\\s*(\\d{1,2})\\s*月\\s*(\\d{1,2})\\s*日");
static zh_date_ptrn3:LazyLock<regex::Regex>=LazyLock::new(||pattern(r"(\d{1,4})\s*年\s*(\d{1,2})\s*月\s*(\d{1,2})\s*日",false));
// java static Pattern jp_date_ptrn1 = Pattern.compile("(h|s|t|m)(\\d{1,2})(\\.|/|\\-)(\\d{1,2})(\\.|/|\\-)(\\d{1,2})", 2);
static jp_date_ptrn1:LazyLock<regex::Regex>=LazyLock::new(||pattern(r"(h|s|t|m)(\d{1,2})(\.|/|\-)(\d{1,2})(\.|/|\-)(\d{1,2})",true));
// java static Pattern jp_date_ptrn2 = Pattern.compile("(平成|昭和|大正|明治)(\\d{1,2})年(\\d{1,2})月(\\d{1,2})日");
static jp_date_ptrn2:LazyLock<regex::Regex>=LazyLock::new(||pattern(r"(平成|昭和|大正|明治)(\d{1,2})年(\d{1,2})月(\d{1,2})日",false));
// Native leaf adapters for Pattern/Integer/String only; ET branch decisions stay below.
fn pattern(p:&str,ci:bool)->regex::Regex{crate::java_runtime::Pattern::compileMatches(p,ci).expect("PatternSyntaxException")}
fn integer(s:&str)->i32{crate::java_runtime::DecimalInteger::parseInt(&s.encode_utf16().collect::<Vec<_>>()).expect("NumberFormatException")}
fn lower(s:&str)->String{String::from_utf16(&s.encode_utf16().map(crate::java_runtime::Character::toLowerCase).collect::<Vec<_>>()).unwrap()}
impl DateRecognisor {
    // java static RecogniseResult getEnglishDMMMY(String s, int beginIndex, int endIndex, boolean usingDate1904) {
    pub fn getEnglishDMMMY(s:&[u16],beginIndex:usize,endIndex:usize,usingDate1904:bool)->Option<RecogniseResult>{
        // java String subs = s.substring(beginIndex, endIndex);
        let subs=String::from_utf16(&s[beginIndex..endIndex]).ok()?;
        // java Calendar c = Calendar.getInstance();
        let mut c=Calendar::getInstance();
        // java Pattern en_dmmmy_ptrn = Pattern.compile((new StringBuilder(100)).append("(\\d{1,2})\\s*(\\-|/| )\\s*(").append("jan|january|feb|february|mar|march|apr|april|may|jun|june|jul|july|aug|august|sep|september|oct|october|nov|november|dec|december").append(")\\s*(\\-|/| )\\s*(\\d{1,4})").toString(), 2);
        let en_dmmmy_ptrn=pattern(&format!(r"(\d{{1,2}})\s*(\-|/| )\s*({en_months})\s*(\-|/| )\s*(\d{{1,4}})"),true);
        // java Matcher m = en_dmmmy_ptrn.matcher(subs); if (m.matches()) {
        if let Some(m)=en_dmmmy_ptrn.captures(&subs){
            // java int day = Integer.parseInt(m.group(1)); String smonth = m.group(3); int year = Integer.parseInt(m.group(5));
            let day=integer(&m[1]);let smonth=&m[3];let mut year=integer(&m[5]);
            // java if (year < 30) { year += 2000; } else if (year < 100) { year += 1900; }
            if year<30{year+=2000;}else if year<100{year+=1900;}
            // java int month = 13;
            let mut month=13;
            // java for(int i = 0; i < en_mprefixs.length; ++i) { if (smonth.toLowerCase().startsWith(en_mprefixs[i])) { month = i; break; } }
            for i in 0..en_mprefixs.len(){if lower(smonth).starts_with(en_mprefixs[i]){month=i as i32;break;}}
            // java if (1900 <= year && year < 10000 && month <= 11 && 1 <= day && day <= 31) {
            if 1900<=year&&year<10000&&month<=11&&1<=day&&day<=31{
                // java c.set(year, month, day, 0, 0, 0); c.set(14, 0);
                c.set(year,month,day,0,0,0);c.setField(14,0);
                // java if (c.get(5) == day) {
                if c.get(5)==day{
                    // java RecogniseResult ret = new RecogniseResult(DateUtil.getExcelDate(c, usingDate1904)); ret.setSuggestedNumfmt(15); ret.setDateFlag(); return ret;
                    let mut ret=RecogniseResult::RecogniseResult__2(Some(DateUtil::getExcelDate_from_calendar(&c,usingDate1904)));ret.setSuggestedNumfmt__2(15);ret.setDateFlag();return Some(ret);
                }
            }
        }
        // java Pattern en_short_dmmmy_ptrn1 = Pattern.compile((new StringBuilder(100)).append("(").append("jan|january|feb|february|mar|march|apr|april|may|jun|june|jul|july|aug|august|sep|september|oct|october|nov|november|dec|december").append(")\\s*(\\-|/| )\\s*(\\d{1,4})").toString(), 2);
        let en_short_dmmmy_ptrn1=pattern(&format!(r"({en_months})\s*(\-|/| )\s*(\d{{1,4}})"),true);
        // java m = en_short_dmmmy_ptrn1.matcher(subs); if (m.matches()) {
        if let Some(m)=en_short_dmmmy_ptrn1.captures(&subs){
            // java String smonth = m.group(1); int year = Integer.parseInt(m.group(3)); int month = 13;
            let smonth=&m[1];let mut year=integer(&m[3]);let mut month=13;
            // java for(int i = 0; i < en_mprefixs.length; ++i) { if (smonth.toLowerCase().startsWith(en_mprefixs[i])) { month = i; break; } }
            for i in 0..en_mprefixs.len(){if lower(smonth).starts_with(en_mprefixs[i]){month=i as i32;break;}}
            // java if (year < 30) { year += 2000; } else if (year < 100) { year += 1900; }
            if year<30{year+=2000;}else if year<100{year+=1900;}
            // java if (1900 <= year && year < 10000 && month <= 11) {
            if 1900<=year&&year<10000&&month<=11{
                // java c.set(year, month, 1, 0, 0, 0); c.set(14, 0);
                c.set(year,month,1,0,0,0);c.setField(14,0);
                // java RecogniseResult ret = new RecogniseResult(DateUtil.getExcelDate(c, usingDate1904)); ret.setSuggestedNumfmt(17); ret.setDateFlag(); return ret;
                let mut ret=RecogniseResult::RecogniseResult__2(Some(DateUtil::getExcelDate_from_calendar(&c,usingDate1904)));ret.setSuggestedNumfmt__2(17);ret.setDateFlag();return Some(ret);
            }
        }
        // java Pattern en_short_dmmmy_ptrn2 = Pattern.compile((new StringBuilder(100)).append("(\\d{1,2})\\s*(\\-|/| )\\s*(").append("jan|january|feb|february|mar|march|apr|april|may|jun|june|jul|july|aug|august|sep|september|oct|october|nov|november|dec|december").append(")").toString(), 2);
        let en_short_dmmmy_ptrn2=pattern(&format!(r"(\d{{1,2}})\s*(\-|/| )\s*({en_months})"),true);
        // java m = en_short_dmmmy_ptrn2.matcher(subs); if (m.matches()) {
        if let Some(m)=en_short_dmmmy_ptrn2.captures(&subs){
            // java int day = Integer.parseInt(m.group(1)); String smonth = m.group(3); int month = 13;
            let day=integer(&m[1]);let smonth=&m[3];let mut month=13;
            // java for(int i = 0; i < en_mprefixs.length; ++i) { if (smonth.toLowerCase().startsWith(en_mprefixs[i])) { month = i; break; } }
            for i in 0..en_mprefixs.len(){if lower(smonth).starts_with(en_mprefixs[i]){month=i as i32;break;}}
            // java if (month <= 11 && 1 <= day && day <= 31) {
            if month<=11&&1<=day&&day<=31{
                // java c.set(c.get(1), month, day, 0, 0, 0); c.set(14, 0);
                c.set(c.get(1),month,day,0,0,0);c.setField(14,0);
                // java if (c.get(5) == day) {
                if c.get(5)==day{
                    // java RecogniseResult ret = new RecogniseResult(DateUtil.getExcelDate(c, usingDate1904)); ret.setSuggestedNumfmt(16); ret.setDateFlag(); return ret;
                    let mut ret=RecogniseResult::RecogniseResult__2(Some(DateUtil::getExcelDate_from_calendar(&c,usingDate1904)));ret.setSuggestedNumfmt__2(16);ret.setDateFlag();return Some(ret);
                }
            }
        }
        // java return null;
        None
    }
    // java static RecogniseResult getEnglishMMMDY(String s, int beginIndex, int endIndex, boolean usingDate1904) {
    pub fn getEnglishMMMDY(s:&[u16],beginIndex:usize,endIndex:usize,usingDate1904:bool)->Option<RecogniseResult>{
        // java String subs = s.substring(beginIndex, endIndex);
        let subs=String::from_utf16(&s[beginIndex..endIndex]).ok()?;
        // java Pattern en_mmmdy_ptrn = Pattern.compile((new StringBuilder(100)).append("(").append("jan|january|feb|february|mar|march|apr|april|may|jun|june|jul|july|aug|august|sep|september|oct|october|nov|november|dec|december").append(")\\s*(\\-|/| )\\s*(\\d{1,2})\\s*,\\s+(\\d{1,4})").toString(), 2);
        let en_mmmdy_ptrn=pattern(&format!(r"({en_months})\s*(\-|/| )\s*(\d{{1,2}})\s*,\s+(\d{{1,4}})"),true);
        // java Matcher m = en_mmmdy_ptrn.matcher(subs); if (m.matches()) {
        if let Some(m)=en_mmmdy_ptrn.captures(&subs){
            // java String smonth = m.group(1); int day = Integer.parseInt(m.group(3)); int year = Integer.parseInt(m.group(4)); int month = 13;
            let smonth=&m[1];let day=integer(&m[3]);let mut year=integer(&m[4]);let mut month=13;
            // java for(int i = 0; i < en_mprefixs.length; ++i) { if (smonth.toLowerCase().startsWith(en_mprefixs[i])) { month = i; break; } }
            for i in 0..en_mprefixs.len(){if lower(smonth).starts_with(en_mprefixs[i]){month=i as i32;break;}}
            // java if (year < 30) { year += 2000; } else if (year < 100) { year += 1900; }
            if year<30{year+=2000;}else if year<100{year+=1900;}
            // java if (1900 <= year && year < 10000 && month <= 11 && 1 <= day && day <= 31) {
            if 1900<=year&&year<10000&&month<=11&&1<=day&&day<=31{
                // java Calendar c = Calendar.getInstance(); c.set(year, month, day, 0, 0, 0); c.set(14, 0);
                let mut c=Calendar::getInstance();c.set(year,month,day,0,0,0);c.setField(14,0);
                // java if (c.get(5) == day) {
                if c.get(5)==day{
                    // java RecogniseResult ret = new RecogniseResult(DateUtil.getExcelDate(c, usingDate1904)); ret.setSuggestedNumfmt(15); ret.setDateFlag(); return ret;
                    let mut ret=RecogniseResult::RecogniseResult__2(Some(DateUtil::getExcelDate_from_calendar(&c,usingDate1904)));ret.setSuggestedNumfmt__2(15);ret.setDateFlag();return Some(ret);
                }
            }
        }
        // java return null;
        None
    }
    // java static RecogniseResult getDMMMY(String s, int beginIndex, int endIndex, boolean usingDate1904, String[] months) {
    pub fn getDMMMY(s:&[u16],beginIndex:usize,endIndex:usize,usingDate1904:bool,months:&[String])->Option<RecogniseResult>{
        // java Calendar c = Calendar.getInstance(); int day = -1; int dayEnd = -1; int i = beginIndex;
        let mut c=Calendar::getInstance();let mut day=-1;let mut dayEnd=-1isize;let mut i=beginIndex;
        // java while(true) {
        loop{
            // java if (i < beginIndex + 2 && i < endIndex) {
            if i<beginIndex+2&&i<endIndex{
                // java char ch = s.charAt(i);
                let ch=s[i];
                // java if (ch >= '0' && ch <= '9') { ++i; continue; }
                if ch>=48&&ch<=57{i+=1;continue;}
                // java if (i == beginIndex) { day = 1; dayEnd = -1; } else {
                if i==beginIndex{day=1;dayEnd=-1;}else{
                    // java if (ch != ' ' && ch != '-' && ch != '/') { return null; }
                    if ch!=32&&ch!=45&&ch!=47{return None;}
                    // java String dayStr = s.substring(beginIndex, i); day = Integer.parseInt(dayStr); dayEnd = i;
                    let dayStr=String::from_utf16(&s[beginIndex..i]).ok()?;day=integer(&dayStr);dayEnd=i as isize;
                    // java if (day < 1 || day > 31) { return null; }
                    if day<1||day>31{return None;}
                }
            }
            // java if (day < 0) { return null; }
            if day<0{return None;}
            // java int year = -1; int yearBegin = -1; int ri = endIndex - 1;
            let mut year=-1;let mut yearBegin=-1isize;let mut ri=endIndex as isize-1;
            // java while (true) {
            loop{
                // java if (ri >= endIndex - 1 - 4 && ri >= beginIndex) {
                if ri>=endIndex as isize-1-4&&ri>=beginIndex as isize{
                    // java char ch = s.charAt(ri);
                    let ch=s[ri as usize];
                    // java if (ch >= '0' && ch <= '9') { --ri; continue; }
                    if ch>=48&&ch<=57{ri-=1;continue;}
                    // java if (ri == endIndex - 1) { year = c.get(Calendar.YEAR); yearBegin = endIndex + 1; } else {
                    if ri==endIndex as isize-1{year=c.get(1);yearBegin=endIndex as isize+1;}else{
                        // java if (ch != ' ' && ch != '-' && ch != '/') { return null; }
                        if ch!=32&&ch!=45&&ch!=47{return None;}
                        // java String yearStr = s.substring(ri + 1, endIndex); year = Integer.parseInt(yearStr); yearBegin = ri + 1;
                        let yearStr=String::from_utf16(&s[(ri+1)as usize..endIndex]).ok()?;year=integer(&yearStr);yearBegin=ri+1;
                        // java if (year < 30) { year += 2000; } else if (year < 100) { year += 1900; }
                        if year<30{year+=2000;}else if year<100{year+=1900;}
                        // java if (year < 1900 || year >= 10000) { return null; }
                        if year<1900||year>=10000{return None;}
                    }
                // java } else { return null; } break;
                }else{return None;}break;
            }
            // java if (year >= 0 && dayEnd + 1 < yearBegin - 1) {
            if year>=0&&dayEnd+1<yearBegin-1{
                // java String monthStr = s.substring(dayEnd + 1, yearBegin - 1); int month = -1;
                let monthStr=&s[(dayEnd+1)as usize..(yearBegin-1)as usize];let mut month=-1;
                // java for (int mi = 0; mi < 12; ++mi) { if (months[mi].equalsIgnoreCase(monthStr)) { month = mi; break; } }
                for mi in 0..12{if crate::java_runtime::Character::equalsIgnoreCase(&months[mi].encode_utf16().collect::<Vec<_>>(),monthStr){month=mi as i32;break;}}
                // java if (month < 0) { return null; }
                if month<0{return None;}
                // java c.set(year, month, day, 0, 0, 0); c.set(14, 0);
                c.set(year,month,day,0,0,0);c.setField(14,0);
                // java if (c.get(5) != day) { return null; }
                if c.get(5)!=day{return None;}
                // java RecogniseResult ret = new RecogniseResult(DateUtil.getExcelDate(c, usingDate1904));
                let mut ret=RecogniseResult::RecogniseResult__2(Some(DateUtil::getExcelDate_from_calendar(&c,usingDate1904)));
                // java if (dayEnd == -1 && yearBegin == endIndex + 1) { return null; }
                if dayEnd==-1&&yearBegin==endIndex as isize+1{return None;}
                // java if (dayEnd == -1) { ret.setSuggestedNumfmt(17); } else if (yearBegin == endIndex + 1) { ret.setSuggestedNumfmt(16); } else { ret.setSuggestedNumfmt(15); }
                if dayEnd==-1{ret.setSuggestedNumfmt__2(17);}else if yearBegin==endIndex as isize+1{ret.setSuggestedNumfmt__2(16);}else{ret.setSuggestedNumfmt__2(15);}
                // java ret.setDateFlag(); return ret;
                ret.setDateFlag();return Some(ret);
            }
            // java return null;
            return None;
        }
    }
}
impl DateRecognisor {
    // java static RecogniseResult getYMD(String s, int beginIndex, int endIndex, LanguageType lang, boolean usingDate1904, char dateSeparator) {
    pub fn getYMD(s:&[u16],beginIndex:usize,endIndex:usize,lang:LanguageType,usingDate1904:bool,dateSeparator:u16)->Option<RecogniseResult>{
        // java String subs = s.substring(beginIndex, endIndex); Calendar c = Calendar.getInstance();
        let subs=String::from_utf16(&s[beginIndex..endIndex]).ok()?;let mut c=Calendar::getInstance();
        // java Pattern ymd_ptrn = Pattern.compile((new StringBuilder(100)).append("(\\d{1,4})\\s*\\").append(dateSeparator).append("\\s*(\\d{1,2})\\s*\\").append(dateSeparator).append("\\s*(\\d{1,2})").toString());
        let sep=char::from_u32(dateSeparator as u32).unwrap();let ymd_ptrn=pattern(&format!(r"(\d{{1,4}})\s*\{sep}\s*(\d{{1,2}})\s*\{sep}\s*(\d{{1,2}})"),false);
        // java Matcher m = ymd_ptrn.matcher(subs); if (m.matches()) {
        if let Some(m)=ymd_ptrn.captures(&subs){
            // java int year = Integer.parseInt(m.group(1)); int month = Integer.parseInt(m.group(2)); int day = Integer.parseInt(m.group(3));
            let mut year=integer(&m[1]);let month=integer(&m[2]);let day=integer(&m[3]);
            // java if (year < 30) { year += 2000; } else if (year < 100) { year += 1900; }
            if year<30{year+=2000;}else if year<100{year+=1900;}
            // java if (1900 <= year && year < 10000 && 1 <= month && month <= 12 && 1 <= day && day <= 31) {
            if 1900<=year&&year<10000&&1<=month&&month<=12&&1<=day&&day<=31{
                // java c.set(year, month - 1, day, 0, 0, 0); c.set(14, 0);
                c.set(year,month-1,day,0,0,0);c.setField(14,0);
                // java if (c.get(5) == day) {
                if c.get(5)==day{
                    // java RecogniseResult ret = new RecogniseResult(DateUtil.getExcelDate(c, usingDate1904)); ret.setSuggestedNumfmt(14); ret.setDateFlag(); return ret;
                    let mut ret=RecogniseResult::RecogniseResult__2(Some(DateUtil::getExcelDate_from_calendar(&c,usingDate1904)));ret.setSuggestedNumfmt__2(14);ret.setDateFlag();return Some(ret);
                }
            }
        }
        // java Pattern short_ymd_ptrn1 = Pattern.compile((new StringBuilder(100)).append("(\\d{1,2})\\s*\\").append(dateSeparator).append("\\s*(\\d{1,2})").toString());
        let short_ymd_ptrn1=pattern(&format!(r"(\d{{1,2}})\s*\{sep}\s*(\d{{1,2}})"),false);
        // java m = short_ymd_ptrn1.matcher(subs); if (m.matches()) {
        if let Some(m)=short_ymd_ptrn1.captures(&subs){
            // java int v1 = Integer.parseInt(m.group(1)); int v2 = Integer.parseInt(m.group(2));
            let v1=integer(&m[1]);let v2=integer(&m[2]);
            // java if (1 <= v1 && v1 <= 31 && 1 <= v2 && v2 <= 31) {
            if 1<=v1&&v1<=31&&1<=v2&&v2<=31{
                // java if (v1 <= 12) {
                if v1<=12{
                    // java c.set(c.get(1), v1 - 1, v2, 0, 0, 0); c.set(14, 0);
                    c.set(c.get(1),v1-1,v2,0,0,0);c.setField(14,0);
                    // java if (c.get(5) == v2) {
                    if c.get(5)==v2{
                        // java Double v = DateUtil.getExcelDate(c, usingDate1904); RecogniseResult ret = new RecogniseResult(v);
                        let v=DateUtil::getExcelDate_from_calendar(&c,usingDate1904);let mut ret=RecogniseResult::RecogniseResult__2(Some(v));
                        // java if (lang.getPrimaryValue() == LanguageType.LANGUAGE_CHINESE.getPrimaryValue()) {
                        if lang.getPrimaryValue()==LANGUAGE_CHINESE.getPrimaryValue(){
                            // java if (lang == LanguageType.LANGUAGE_CHINESE_SIMPLIFIED) { ret.setSuggestedNumfmt(28); } else { ret.setSuggestedNumfmt("m\"月\"d\"日\""); }
                            if lang==LANGUAGE_CHINESE_SIMPLIFIED{ret.setSuggestedNumfmt__2(28);}else{ret.setSuggestedNumfmt(Some("m\"月\"d\"日\"".into()));}
                        // java } else if (lang.getPrimaryValue() == LanguageType.LANGUAGE_JAPANESE.getPrimaryValue()) { ret.setSuggestedNumfmt(35); }
                        }else if lang.getPrimaryValue()==LANGUAGE_JAPANESE.getPrimaryValue(){ret.setSuggestedNumfmt__2(35);}
                        // java else if (lang.getPrimaryValue() == LanguageType.LANGUAGE_KOREAN.getPrimaryValue()) { ret.setSuggestedNumfmt("mm\"월\" dd\"일\""); } else { ret.setSuggestedNumfmt(16); }
                        else if lang.getPrimaryValue()==LANGUAGE_KOREAN.getPrimaryValue(){ret.setSuggestedNumfmt(Some("mm\"월\" dd\"일\"".into()));}else{ret.setSuggestedNumfmt__2(16);}
                        // java ret.setDateFlag(); return ret;
                        ret.setDateFlag();return Some(ret);
                    }
                }
                // java if (v2 <= 12) {
                if v2<=12{
                    // java c.set(c.get(1), v2 - 1, v1, 0, 0, 0); c.set(14, 0);
                    c.set(c.get(1),v2-1,v1,0,0,0);c.setField(14,0);
                    // java if (c.get(5) == v1) {
                    if c.get(5)==v1{
                        // java Double v = DateUtil.getExcelDate(c, usingDate1904); RecogniseResult ret = new RecogniseResult(v);
                        let v=DateUtil::getExcelDate_from_calendar(&c,usingDate1904);let mut ret=RecogniseResult::RecogniseResult__2(Some(v));
                        // java if (lang.getPrimaryValue() == LanguageType.LANGUAGE_CHINESE.getPrimaryValue()) {
                        if lang.getPrimaryValue()==LANGUAGE_CHINESE.getPrimaryValue(){
                            // java if (lang == LanguageType.LANGUAGE_CHINESE_SIMPLIFIED) { ret.setSuggestedNumfmt(28); } else { ret.setSuggestedNumfmt("m\"月\"d\"日\""); }
                            if lang==LANGUAGE_CHINESE_SIMPLIFIED{ret.setSuggestedNumfmt__2(28);}else{ret.setSuggestedNumfmt(Some("m\"月\"d\"日\"".into()));}
                        // java } else if (lang.getPrimaryValue() == LanguageType.LANGUAGE_JAPANESE.getPrimaryValue()) { ret.setSuggestedNumfmt(35); }
                        }else if lang.getPrimaryValue()==LANGUAGE_JAPANESE.getPrimaryValue(){ret.setSuggestedNumfmt__2(35);}
                        // java else if (lang.getPrimaryValue() == LanguageType.LANGUAGE_KOREAN.getPrimaryValue()) { ret.setSuggestedNumfmt("mm\"월\" dd\"일\""); } else { ret.setSuggestedNumfmt(16); }
                        else if lang.getPrimaryValue()==LANGUAGE_KOREAN.getPrimaryValue(){ret.setSuggestedNumfmt(Some("mm\"월\" dd\"일\"".into()));}else{ret.setSuggestedNumfmt__2(16);}
                        // java ret.setDateFlag(); return ret;
                        ret.setDateFlag();return Some(ret);
                    }
                }
            }
        }
        // java Pattern short_ymd_ptrn2 = Pattern.compile((new StringBuilder(100)).append("(\\d{1,4})\\s*\\").append(dateSeparator).append("\\s*(\\d{1,2})").toString());
        let short_ymd_ptrn2=pattern(&format!(r"(\d{{1,4}})\s*\{sep}\s*(\d{{1,2}})"),false);
        // java m = short_ymd_ptrn2.matcher(subs); if (m.matches()) {
        if let Some(m)=short_ymd_ptrn2.captures(&subs){
            // java int year = Integer.parseInt(m.group(1)); int month = Integer.parseInt(m.group(2));
            let mut year=integer(&m[1]);let month=integer(&m[2]);
            // java if (0 < year && year < 30) { year += 2000; } else if (30 <= year && year < 100) { year += 1900; }
            if 0<year&&year<30{year+=2000;}else if 30<=year&&year<100{year+=1900;}
            // java if (1900 <= year && year < 10000 && 1 <= month && month <= 12) {
            if 1900<=year&&year<10000&&1<=month&&month<=12{
                // java c.set(year, month - 1, 1, 0, 0, 0); c.set(14, 0);
                c.set(year,month-1,1,0,0,0);c.setField(14,0);
                // java RecogniseResult ret = new RecogniseResult(DateUtil.getExcelDate(c, usingDate1904)); ret.setSuggestedNumfmt(17); ret.setDateFlag(); return ret;
                let mut ret=RecogniseResult::RecogniseResult__2(Some(DateUtil::getExcelDate_from_calendar(&c,usingDate1904)));ret.setSuggestedNumfmt__2(17);ret.setDateFlag();return Some(ret);
            }
        }
        // java return null;
        None
    }
    // java static RecogniseResult getDMY(String s, int beginIndex, int endIndex, LanguageType lang, boolean usingDate1904, char dateSeparator) {
    pub fn getDMY(s:&[u16],beginIndex:usize,endIndex:usize,lang:LanguageType,usingDate1904:bool,dateSeparator:u16)->Option<RecogniseResult>{
        // java String subs = s.substring(beginIndex, endIndex); Calendar c = Calendar.getInstance();
        let subs=String::from_utf16(&s[beginIndex..endIndex]).ok()?;let mut c=Calendar::getInstance();
        // java Pattern dmy_ptrn = Pattern.compile((new StringBuilder(100)).append("(\\d{1,2})\\s*\\").append(dateSeparator).append("\\s*(\\d{1,2})\\s*\\").append(dateSeparator).append("\\s*(\\d{1,4})").toString());
        let sep=char::from_u32(dateSeparator as u32).unwrap();let dmy_ptrn=pattern(&format!(r"(\d{{1,2}})\s*\{sep}\s*(\d{{1,2}})\s*\{sep}\s*(\d{{1,4}})"),false);
        // java Matcher m = dmy_ptrn.matcher(s.substring(beginIndex, endIndex)); if (m.matches()) {
        if let Some(m)=dmy_ptrn.captures(&subs){
            // java int year = Integer.parseInt(m.group(3)); int month = Integer.parseInt(m.group(2)); int day = Integer.parseInt(m.group(1));
            let mut year=integer(&m[3]);let month=integer(&m[2]);let day=integer(&m[1]);
            // java if (year < 30) { year += 2000; } else if (year < 100) { year += 1900; }
            if year<30{year+=2000;}else if year<100{year+=1900;}
            // java if (1900 <= year && year < 10000 && 1 <= month && month <= 12 && 1 <= day && day <= 31) {
            if 1900<=year&&year<10000&&1<=month&&month<=12&&1<=day&&day<=31{
                // java c.set(year, month - 1, day, 0, 0, 0); c.set(14, 0);
                c.set(year,month-1,day,0,0,0);c.setField(14,0);
                // java if (c.get(5) == day) {
                if c.get(5)==day{
                    // java RecogniseResult ret = new RecogniseResult(DateUtil.getExcelDate(c, usingDate1904)); ret.setSuggestedNumfmt(14); ret.setDateFlag(); return ret;
                    let mut ret=RecogniseResult::RecogniseResult__2(Some(DateUtil::getExcelDate_from_calendar(&c,usingDate1904)));ret.setSuggestedNumfmt__2(14);ret.setDateFlag();return Some(ret);
                }
            }
        }
        // java Pattern short_dmy_ptrn1 = Pattern.compile((new StringBuilder(100)).append("(\\d{1,2})\\s*\\").append(dateSeparator).append("\\s*(\\d{1,2})").toString());
        let short_dmy_ptrn1=pattern(&format!(r"(\d{{1,2}})\s*\{sep}\s*(\d{{1,2}})"),false);
        // java m = short_dmy_ptrn1.matcher(subs); if (m.matches()) {
        if let Some(m)=short_dmy_ptrn1.captures(&subs){
            // java int day = Integer.parseInt(m.group(1)); int month = Integer.parseInt(m.group(2));
            let day=integer(&m[1]);let month=integer(&m[2]);
            // java if (1 <= day && day <= 31 && 1 <= month && month <= 12) {
            if 1<=day&&day<=31&&1<=month&&month<=12{
                // java c.set(c.get(1), month - 1, day, 0, 0, 0); c.set(14, 0);
                c.set(c.get(1),month-1,day,0,0,0);c.setField(14,0);
                // java if (c.get(5) == day) {
                if c.get(5)==day{
                    // java Double v = DateUtil.getExcelDate(c, usingDate1904); RecogniseResult ret = new RecogniseResult(v);
                    let v=DateUtil::getExcelDate_from_calendar(&c,usingDate1904);let mut ret=RecogniseResult::RecogniseResult__2(Some(v));
                    // java if (lang.getPrimaryValue() == LanguageType.LANGUAGE_CHINESE.getPrimaryValue()) {
                    if lang.getPrimaryValue()==LANGUAGE_CHINESE.getPrimaryValue(){
                        // java if (lang == LanguageType.LANGUAGE_CHINESE_SIMPLIFIED) { ret.setSuggestedNumfmt(28); } else { ret.setSuggestedNumfmt("m\"月\"d\"日\""); }
                        if lang==LANGUAGE_CHINESE_SIMPLIFIED{ret.setSuggestedNumfmt__2(28);}else{ret.setSuggestedNumfmt(Some("m\"月\"d\"日\"".into()));}
                    // java } else if (lang.getPrimaryValue() == LanguageType.LANGUAGE_JAPANESE.getPrimaryValue()) { ret.setSuggestedNumfmt(35); } else { ret.setSuggestedNumfmt(16); }
                    }else if lang.getPrimaryValue()==LANGUAGE_JAPANESE.getPrimaryValue(){ret.setSuggestedNumfmt__2(35);}else{ret.setSuggestedNumfmt__2(16);}
                    // java ret.setDateFlag(); return ret;
                    ret.setDateFlag();return Some(ret);
                }
            }
        }
        // java Pattern short_dmy_ptrn2 = Pattern.compile((new StringBuilder(100)).append("(\\d{1,2})\\s*\\").append(dateSeparator).append("\\s*(\\d{1,4})").toString());
        let short_dmy_ptrn2=pattern(&format!(r"(\d{{1,2}})\s*\{sep}\s*(\d{{1,4}})"),false);
        // java m = short_dmy_ptrn2.matcher(subs); if (m.matches()) {
        if let Some(m)=short_dmy_ptrn2.captures(&subs){
            // java int year = Integer.parseInt(m.group(2)); int month = Integer.parseInt(m.group(1));
            let mut year=integer(&m[2]);let month=integer(&m[1]);
            // java if (0 < year && year < 30) { year += 2000; } else if (30 <= year && year < 100) { year += 1900; }
            if 0<year&&year<30{year+=2000;}else if 30<=year&&year<100{year+=1900;}
            // java if (1900 <= year && year < 10000 && 1 <= month && month <= 12) {
            if 1900<=year&&year<10000&&1<=month&&month<=12{
                // java c.set(year, month - 1, 1, 0, 0, 0); c.set(14, 0);
                c.set(year,month-1,1,0,0,0);c.setField(14,0);
                // java RecogniseResult ret = new RecogniseResult(DateUtil.getExcelDate(c, usingDate1904)); ret.setSuggestedNumfmt(17); ret.setDateFlag(); return ret;
                let mut ret=RecogniseResult::RecogniseResult__2(Some(DateUtil::getExcelDate_from_calendar(&c,usingDate1904)));ret.setSuggestedNumfmt__2(17);ret.setDateFlag();return Some(ret);
            }
        }
        // java return null;
        None
    }
    // java static RecogniseResult getMDY(String s, int beginIndex, int endIndex, LanguageType lang, boolean usingDate1904, char dateSeparator) {
    pub fn getMDY(s:&[u16],beginIndex:usize,endIndex:usize,lang:LanguageType,usingDate1904:bool,dateSeparator:u16)->Option<RecogniseResult>{
        // java String subs = s.substring(beginIndex, endIndex); Calendar c = Calendar.getInstance();
        let subs=String::from_utf16(&s[beginIndex..endIndex]).ok()?;let mut c=Calendar::getInstance();
        // java Pattern mdy_ptrn = Pattern.compile((new StringBuilder(100)).append("(\\d{1,2})\\s*\\").append(dateSeparator).append("\\s*(\\d{1,2})\\s*\\").append(dateSeparator).append("\\s*(\\d{1,4})").toString());
        let sep=char::from_u32(dateSeparator as u32).unwrap();let mdy_ptrn=pattern(&format!(r"(\d{{1,2}})\s*\{sep}\s*(\d{{1,2}})\s*\{sep}\s*(\d{{1,4}})"),false);
        // java Matcher m = mdy_ptrn.matcher(subs); if (m.matches()) {
        if let Some(m)=mdy_ptrn.captures(&subs){
            // java int year = Integer.parseInt(m.group(3)); int month = Integer.parseInt(m.group(1)); int day = Integer.parseInt(m.group(2));
            let mut year=integer(&m[3]);let month=integer(&m[1]);let day=integer(&m[2]);
            // java if (year < 30) { year += 2000; } else if (year < 100) { year += 1900; }
            if year<30{year+=2000;}else if year<100{year+=1900;}
            // java if (1900 <= year && year < 10000 && 1 <= month && month <= 12 && 1 <= day && day <= 31) {
            if 1900<=year&&year<10000&&1<=month&&month<=12&&1<=day&&day<=31{
                // java c.set(year, month - 1, day, 0, 0, 0); c.set(14, 0);
                c.set(year,month-1,day,0,0,0);c.setField(14,0);
                // java if (c.get(5) == day) {
                if c.get(5)==day{
                    // java RecogniseResult ret = new RecogniseResult(DateUtil.getExcelDate(c, usingDate1904)); ret.setSuggestedNumfmt(14); ret.setDateFlag(); return ret;
                    let mut ret=RecogniseResult::RecogniseResult__2(Some(DateUtil::getExcelDate_from_calendar(&c,usingDate1904)));ret.setSuggestedNumfmt__2(14);ret.setDateFlag();return Some(ret);
                }
            }
        }
        // java Pattern short_mdy_ptrn1 = Pattern.compile((new StringBuilder(100)).append("(\\d{1,2})\\s*\\").append(dateSeparator).append("\\s*(\\d{1,2})").toString());
        let short_mdy_ptrn1=pattern(&format!(r"(\d{{1,2}})\s*\{sep}\s*(\d{{1,2}})"),false);
        // java m = short_mdy_ptrn1.matcher(subs); if (m.matches()) {
        if let Some(m)=short_mdy_ptrn1.captures(&subs){
            // java int day = Integer.parseInt(m.group(2)); int month = Integer.parseInt(m.group(1));
            let day=integer(&m[2]);let month=integer(&m[1]);
            // java if (1 <= day && day <= 31 && 1 <= month && month <= 12) {
            if 1<=day&&day<=31&&1<=month&&month<=12{
                // java c.set(c.get(1), month - 1, day, 0, 0, 0); c.set(14, 0);
                c.set(c.get(1),month-1,day,0,0,0);c.setField(14,0);
                // java if (c.get(5) == day) {
                if c.get(5)==day{
                    // java Double v = DateUtil.getExcelDate(c, usingDate1904); RecogniseResult ret = new RecogniseResult(v);
                    let v=DateUtil::getExcelDate_from_calendar(&c,usingDate1904);let mut ret=RecogniseResult::RecogniseResult__2(Some(v));
                    // java if (lang.getPrimaryValue() == LanguageType.LANGUAGE_CHINESE.getPrimaryValue()) {
                    if lang.getPrimaryValue()==LANGUAGE_CHINESE.getPrimaryValue(){
                        // java if (lang == LanguageType.LANGUAGE_CHINESE_SIMPLIFIED) { ret.setSuggestedNumfmt(28); } else { ret.setSuggestedNumfmt("m\"月\"d\"日\""); }
                        if lang==LANGUAGE_CHINESE_SIMPLIFIED{ret.setSuggestedNumfmt__2(28);}else{ret.setSuggestedNumfmt(Some("m\"月\"d\"日\"".into()));}
                    // java } else if (lang.getPrimaryValue() == LanguageType.LANGUAGE_JAPANESE.getPrimaryValue()) { ret.setSuggestedNumfmt(35); } else { ret.setSuggestedNumfmt(16); }
                    }else if lang.getPrimaryValue()==LANGUAGE_JAPANESE.getPrimaryValue(){ret.setSuggestedNumfmt__2(35);}else{ret.setSuggestedNumfmt__2(16);}
                    // java ret.setDateFlag(); return ret;
                    ret.setDateFlag();return Some(ret);
                }
            }
        }
        // java Pattern short_mdy_ptrn2 = Pattern.compile((new StringBuilder(100)).append("(\\d{1,2})\\s*\\").append(dateSeparator).append("\\s*(\\d{1,4})").toString());
        let short_mdy_ptrn2=pattern(&format!(r"(\d{{1,2}})\s*\{sep}\s*(\d{{1,4}})"),false);
        // java m = short_mdy_ptrn2.matcher(subs); if (m.matches()) {
        if let Some(m)=short_mdy_ptrn2.captures(&subs){
            // java int year = Integer.parseInt(m.group(2)); int month = Integer.parseInt(m.group(1));
            let mut year=integer(&m[2]);let month=integer(&m[1]);
            // java if (0 < year && year < 30) { year += 2000; } else if (30 <= year && year < 100) { year += 1900; }
            if 0<year&&year<30{year+=2000;}else if 30<=year&&year<100{year+=1900;}
            // java if (1900 <= year && year < 10000 && 1 <= month && month <= 12) {
            if 1900<=year&&year<10000&&1<=month&&month<=12{
                // java c.set(year, month - 1, 1, 0, 0, 0); c.set(14, 0);
                c.set(year,month-1,1,0,0,0);c.setField(14,0);
                // java RecogniseResult ret = new RecogniseResult(DateUtil.getExcelDate(c, usingDate1904)); ret.setSuggestedNumfmt(17); ret.setDateFlag(); return ret;
                let mut ret=RecogniseResult::RecogniseResult__2(Some(DateUtil::getExcelDate_from_calendar(&c,usingDate1904)));ret.setSuggestedNumfmt__2(17);ret.setDateFlag();return Some(ret);
            }
        }
        // java return null;
        None
    }
    // java static RecogniseResult getChineseDate(String s, int beginIndex, int endIndex, boolean date1904, LanguageType lang) {
    pub fn getChineseDate(s:&[u16],beginIndex:usize,endIndex:usize,date1904:bool,lang:LanguageType)->Option<RecogniseResult>{
        // java String subs = s.substring(beginIndex, endIndex); Calendar c = Calendar.getInstance(); Matcher m = zh_date_ptrn1.matcher(subs);
        let subs=String::from_utf16(&s[beginIndex..endIndex]).ok()?;let mut c=Calendar::getInstance();
        // java if (m.matches()) {
        if let Some(m)=zh_date_ptrn1.captures(&subs){
            // java String monthStr = m.group(1); String dayStr = m.group(2); int month = Integer.parseInt(m.group(1)); int day = Integer.parseInt(m.group(2));
            let monthStr=&m[1];let dayStr=&m[2];let month=integer(&m[1]);let day=integer(&m[2]);
            // java if (1 <= month && month <= 12 && 1 <= day && day <= 31) {
            if 1<=month&&month<=12&&1<=day&&day<=31{
                // java c.set(c.get(1), month - 1, day, 0, 0, 0); c.set(14, 0);
                c.set(c.get(1),month-1,day,0,0,0);c.setField(14,0);
                // java if (c.get(5) == day) {
                if c.get(5)==day{
                    // java Double v = DateUtil.getExcelDate(c, date1904); RecogniseResult ret = new RecogniseResult(v);
                    let v=DateUtil::getExcelDate_from_calendar(&c,date1904);let mut ret=RecogniseResult::RecogniseResult__2(Some(v));
                    // java if (lang.getPrimaryValue() == LanguageType.LANGUAGE_CHINESE.getPrimaryValue()) {
                    if lang.getPrimaryValue()==LANGUAGE_CHINESE.getPrimaryValue(){
                        // java if (!monthStr.startsWith("0") && !dayStr.startsWith("0")) {
                        if !monthStr.starts_with('0')&&!dayStr.starts_with('0'){
                            // java if (lang == LanguageType.LANGUAGE_CHINESE_SIMPLIFIED) { ret.setSuggestedNumfmt(28); } else { ret.setSuggestedNumfmt("m\"月\"d\"日\""); }
                            if lang==LANGUAGE_CHINESE_SIMPLIFIED{ret.setSuggestedNumfmt__2(28);}else{ret.setSuggestedNumfmt(Some("m\"月\"d\"日\"".into()));}
                        // java } else { ret.setSuggestedNumfmt("mm\"月\"dd\"日\""); }
                        }else{ret.setSuggestedNumfmt(Some("mm\"月\"dd\"日\"".into()));}
                    // java } else if (lang.getPrimaryValue() == LanguageType.LANGUAGE_JAPANESE.getPrimaryValue()) { ret.setSuggestedNumfmt(35); } else { ret.setSuggestedNumfmt("m\"月\"d\"日\""); }
                    }else if lang.getPrimaryValue()==LANGUAGE_JAPANESE.getPrimaryValue(){ret.setSuggestedNumfmt__2(35);}else{ret.setSuggestedNumfmt(Some("m\"月\"d\"日\"".into()));}
                    // java ret.setDateFlag(); return ret;
                    ret.setDateFlag();return Some(ret);
                }
            }
        }
        // java m = zh_date_ptrn2.matcher(subs); if (m.matches()) {
        if let Some(m)=zh_date_ptrn2.captures(&subs){
            // java int year = Integer.parseInt(m.group(1)); int month = Integer.parseInt(m.group(2));
            let mut year=integer(&m[1]);let month=integer(&m[2]);
            // java if (year < 30) { year += 2000; } else if (year < 100) { year += 1900; }
            if year<30{year+=2000;}else if year<100{year+=1900;}
            // java if (1900 <= year && year < 10000 && 1 <= month && month <= 12) {
            if 1900<=year&&year<10000&&1<=month&&month<=12{
                // java c.set(year, month - 1, 1, 0, 0, 0); c.set(14, 0);
                c.set(year,month-1,1,0,0,0);c.setField(14,0);
                // java Double v = DateUtil.getExcelDate(c, date1904); RecogniseResult ret = new RecogniseResult(v);
                let v=DateUtil::getExcelDate_from_calendar(&c,date1904);let mut ret=RecogniseResult::RecogniseResult__2(Some(v));
                // java if (lang.getPrimaryValue() == LanguageType.LANGUAGE_CHINESE.getPrimaryValue()) {
                if lang.getPrimaryValue()==LANGUAGE_CHINESE.getPrimaryValue(){
                    // java if (lang == LanguageType.LANGUAGE_CHINESE_SIMPLIFIED) { ret.setSuggestedNumfmt(27); } else { ret.setSuggestedNumfmt("yyyy\"年\"m\"月\""); }
                    if lang==LANGUAGE_CHINESE_SIMPLIFIED{ret.setSuggestedNumfmt__2(27);}else{ret.setSuggestedNumfmt(Some("yyyy\"年\"m\"月\"".into()));}
                // java } else if (lang.getPrimaryValue() == LanguageType.LANGUAGE_JAPANESE.getPrimaryValue()) { ret.setSuggestedNumfmt(34); } else { ret.setSuggestedNumfmt("yyyy\"年\"m\"月\""); }
                }else if lang.getPrimaryValue()==LANGUAGE_JAPANESE.getPrimaryValue(){ret.setSuggestedNumfmt__2(34);}else{ret.setSuggestedNumfmt(Some("yyyy\"年\"m\"月\"".into()));}
                // java ret.setDateFlag(); return ret;
                ret.setDateFlag();return Some(ret);
            }
        }
        // java m = zh_date_ptrn3.matcher(subs); if (m.matches()) {
        if let Some(m)=zh_date_ptrn3.captures(&subs){
            // java int year = Integer.parseInt(m.group(1)); int month = Integer.parseInt(m.group(2)); int day = Integer.parseInt(m.group(3));
            let mut year=integer(&m[1]);let month=integer(&m[2]);let day=integer(&m[3]);
            // java if (year < 30) { year += 2000; } else if (year < 100) { year += 1900; }
            if year<30{year+=2000;}else if year<100{year+=1900;}
            // java if (1900 <= year && year < 10000 && 1 <= month && month <= 12 && 1 <= day && day <= 31) {
            if 1900<=year&&year<10000&&1<=month&&month<=12&&1<=day&&day<=31{
                // java c.set(year, month - 1, day, 0, 0, 0); c.set(14, 0);
                c.set(year,month-1,day,0,0,0);c.setField(14,0);
                // java if (c.get(5) == day) {
                if c.get(5)==day{
                    // java Double v = DateUtil.getExcelDate(c, date1904); RecogniseResult ret = new RecogniseResult(v);
                    let v=DateUtil::getExcelDate_from_calendar(&c,date1904);let mut ret=RecogniseResult::RecogniseResult__2(Some(v));
                    // java if (lang.getPrimaryValue() == LanguageType.LANGUAGE_CHINESE.getPrimaryValue()) { ret.setSuggestedNumfmt(31); } else if (lang.getPrimaryValue() == LanguageType.LANGUAGE_JAPANESE.getPrimaryValue()) { ret.setSuggestedNumfmt(31); } else { ret.setSuggestedNumfmt("yyyy\"年\"m\"月\"d\"日\""); }
                    if lang.getPrimaryValue()==LANGUAGE_CHINESE.getPrimaryValue(){ret.setSuggestedNumfmt__2(31);}else if lang.getPrimaryValue()==LANGUAGE_JAPANESE.getPrimaryValue(){ret.setSuggestedNumfmt__2(31);}else{ret.setSuggestedNumfmt(Some("yyyy\"年\"m\"月\"d\"日\"".into()));}
                    // java ret.setDateFlag(); return ret;
                    ret.setDateFlag();return Some(ret);
                }
            }
        }
        // java return null;
        None
    }
    // java static RecogniseResult getJapaneseDate(String s, int beginIndex, int endIndex, boolean date1904, LanguageType lang) {
    pub fn getJapaneseDate(s:&[u16],beginIndex:usize,endIndex:usize,date1904:bool,lang:LanguageType)->Option<RecogniseResult>{
        // java RecogniseResult rs = getChineseDate(s, beginIndex, endIndex, date1904, lang);
        let rs=Self::getChineseDate(s,beginIndex,endIndex,date1904,lang);
        // java if (rs != null) { return rs; } else {
        if rs.is_some(){rs}else{
            // java String subs = s.substring(beginIndex, endIndex).toLowerCase(); Calendar c = Calendar.getInstance(); Matcher m = jp_date_ptrn1.matcher(subs);
            let subs=lower(&String::from_utf16(&s[beginIndex..endIndex]).ok()?);let mut c=Calendar::getInstance();
            // java if (m.matches()) {
            if let Some(m)=jp_date_ptrn1.captures(&subs){
                // java int year = Integer.parseInt(m.group(2)); int month = Integer.parseInt(m.group(4)); int day = Integer.parseInt(m.group(6));
                let mut year=integer(&m[2]);let month=integer(&m[4]);let day=integer(&m[6]);
                // java switch (m.group(1).charAt(0)) { case 'h': year += 1988; break; case 'm': year += 1867; break; case 's': year += 1925; break; case 't': year += 1912; }
                match m[1].as_bytes()[0]{b'h'=>year+=1988,b'm'=>year+=1867,b's'=>year+=1925,b't'=>year+=1912,_=>{}}
                // java if (1900 <= year && year < 10000 && 1 <= month && month <= 12 && 1 <= day && day <= 31) {
                if 1900<=year&&year<10000&&1<=month&&month<=12&&1<=day&&day<=31{
                    // java c.set(year, month - 1, day, 0, 0, 0); c.set(14, 0);
                    c.set(year,month-1,day,0,0,0);c.setField(14,0);
                    // java if (c.get(5) == day) {
                    if c.get(5)==day{
                        // java Double v = DateUtil.getExcelDate(c, date1904); RecogniseResult ret = new RecogniseResult(v); ret.setSuggestedNumfmt(50); ret.setDateFlag(); return ret;
                        let v=DateUtil::getExcelDate_from_calendar(&c,date1904);let mut ret=RecogniseResult::RecogniseResult__2(Some(v));ret.setSuggestedNumfmt__2(50);ret.setDateFlag();return Some(ret);
                    }
                }
            }
            // java m = jp_date_ptrn2.matcher(subs); if (m.matches()) {
            if let Some(m)=jp_date_ptrn2.captures(&subs){
                // java int year = Integer.parseInt(m.group(2)); int month = Integer.parseInt(m.group(3)); int day = Integer.parseInt(m.group(4)); String syear = m.group(1);
                let mut year=integer(&m[2]);let month=integer(&m[3]);let day=integer(&m[4]);let syear=&m[1];
                // java if (syear.equals("平成")) { year += 1988; } else if (syear.equals("昭和")) { year += 1925; } else if (syear.equals("大正")) { year += 1912; } else if (syear.equals("明治")) { year += 1867; }
                if syear=="平成"{year+=1988;}else if syear=="昭和"{year+=1925;}else if syear=="大正"{year+=1912;}else if syear=="明治"{year+=1867;}
                // java if (1900 <= year && year < 10000 && 1 <= month && month <= 12 && 1 <= day && day <= 31) {
                if 1900<=year&&year<10000&&1<=month&&month<=12&&1<=day&&day<=31{
                    // java c.set(year, month - 1, day, 0, 0, 0); c.set(14, 0);
                    c.set(year,month-1,day,0,0,0);c.setField(14,0);
                    // java if (c.get(5) == day) {
                    if c.get(5)==day{
                        // java Double v = DateUtil.getExcelDate(c, date1904); RecogniseResult ret = new RecogniseResult(v); ret.setSuggestedNumfmt(51); ret.setDateFlag(); return ret;
                        let v=DateUtil::getExcelDate_from_calendar(&c,date1904);let mut ret=RecogniseResult::RecogniseResult__2(Some(v));ret.setSuggestedNumfmt__2(51);ret.setDateFlag();return Some(ret);
                    }
                }
            }
            // java return null;
            None
        }
    }
    // java public static RecogniseResult getDate(String s, int beginIndex, int endIndex, boolean date1904, LanguageType lang, LangRes langRes) {
    pub fn getDate(s:&[u16],beginIndex:usize,endIndex:usize,date1904:bool,lang:LanguageType,langRes:&LangRes)->Option<RecogniseResult>{
        // java RecogniseResult rs = null;
        let mut rs=None;
        // java if (lang.getPrimaryValue() == LanguageType.LANGUAGE_CHINESE.getPrimaryValue()) {
        if lang.getPrimaryValue()==LANGUAGE_CHINESE.getPrimaryValue(){
            // java rs = getChineseDate(s, 0, s.length(), date1904, lang); if (rs != null) { return rs; }
            rs=Self::getChineseDate(s,0,s.len(),date1904,lang);if rs.is_some(){return rs;}
        // java } else if (lang == LanguageType.LANGUAGE_JAPANESE) {
        }else if lang==LANGUAGE_JAPANESE{
            // java rs = getJapaneseDate(s, 0, s.length(), date1904, lang); if (rs != null) { return rs; }
            rs=Self::getJapaneseDate(s,0,s.len(),date1904,lang);if rs.is_some(){return rs;}
        }
        // java if (langRes.DATE.MONTHS != null) {
        if let Some(months)=&langRes.DATE.MONTHS{
            // java rs = getDMMMY(s, 0, s.length(), date1904, langRes.DATE.MONTHS); if (rs != null) { return rs; }
            rs=Self::getDMMMY(s,0,s.len(),date1904,months);if rs.is_some(){return rs;}
        }
        // java if (langRes.DATE.SUPPORT_MMMDY_DATE) {
        if langRes.DATE.SUPPORT_MMMDY_DATE{
            // java rs = getEnglishMMMDY(s, beginIndex, endIndex, date1904); if (rs != null) { return rs; }
            rs=Self::getEnglishMMMDY(s,beginIndex,endIndex,date1904);if rs.is_some(){return rs;}
        }
        // java rs = getEnglishDMMMY(s, beginIndex, endIndex, date1904);
        rs=Self::getEnglishDMMMY(s,beginIndex,endIndex,date1904);
        // java if (rs != null) { return rs; } else {
        if rs.is_some(){rs}else{
            // java if (langRes.DATE.SHORT_DATEFMT_TYPE == DateRes.ShortDateFmtType.DMY) {
            if langRes.DATE.SHORT_DATEFMT_TYPE==ShortDateFmtType::DMY{
                // java rs = getDMY(s, beginIndex, endIndex, lang, date1904, '/');
                rs=Self::getDMY(s,beginIndex,endIndex,lang,date1904,47);
                // java if (rs == null) { rs = getDMY(s, beginIndex, endIndex, lang, date1904, '-'); }
                if rs.is_none(){rs=Self::getDMY(s,beginIndex,endIndex,lang,date1904,45);}
                // java if (rs == null && langRes.DATE.SHORT_DATE_NUMFMT.indexOf(46) >= 0) { rs = getDMY(s, beginIndex, endIndex, lang, date1904, '.'); }
                if rs.is_none()&&langRes.DATE.SHORT_DATE_NUMFMT.contains('.') {rs=Self::getDMY(s,beginIndex,endIndex,lang,date1904,46);}
                // java if (rs != null) { return rs; }
                if rs.is_some(){return rs;}
            // java } else if (langRes.DATE.SHORT_DATEFMT_TYPE == DateRes.ShortDateFmtType.MDY) {
            }else if langRes.DATE.SHORT_DATEFMT_TYPE==ShortDateFmtType::MDY{
                // java rs = getMDY(s, beginIndex, endIndex, lang, date1904, '/');
                rs=Self::getMDY(s,beginIndex,endIndex,lang,date1904,47);
                // java if (rs == null) { rs = getMDY(s, beginIndex, endIndex, lang, date1904, '-'); }
                if rs.is_none(){rs=Self::getMDY(s,beginIndex,endIndex,lang,date1904,45);}
                // java if (rs == null && langRes.DATE.SHORT_DATE_NUMFMT.indexOf(46) >= 0) { rs = getMDY(s, beginIndex, endIndex, lang, date1904, '.'); }
                if rs.is_none()&&langRes.DATE.SHORT_DATE_NUMFMT.contains('.') {rs=Self::getMDY(s,beginIndex,endIndex,lang,date1904,46);}
                // java if (rs != null) { return rs; }
                if rs.is_some(){return rs;}
            }
            // java if (rs == null || langRes.DATE.SHORT_DATEFMT_TYPE == DateRes.ShortDateFmtType.YMD) {
            if rs.is_none()||langRes.DATE.SHORT_DATEFMT_TYPE==ShortDateFmtType::YMD{
                // java rs = getYMD(s, beginIndex, endIndex, lang, date1904, '/');
                rs=Self::getYMD(s,beginIndex,endIndex,lang,date1904,47);
                // java if (rs == null) { rs = getYMD(s, beginIndex, endIndex, lang, date1904, '-'); }
                if rs.is_none(){rs=Self::getYMD(s,beginIndex,endIndex,lang,date1904,45);}
                // java if (rs == null && langRes.DATE.SHORT_DATE_NUMFMT.indexOf(46) >= 0) { rs = getYMD(s, beginIndex, endIndex, lang, date1904, '.'); }
                if rs.is_none()&&langRes.DATE.SHORT_DATE_NUMFMT.contains('.') {rs=Self::getYMD(s,beginIndex,endIndex,lang,date1904,46);}
                // java if (rs != null) { return rs; }
                if rs.is_some(){return rs;}
            }
            // java return null;
            None
        }
    }
}
