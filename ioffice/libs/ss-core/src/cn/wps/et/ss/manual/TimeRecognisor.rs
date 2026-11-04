use crate::cn::wps::et::ss::{constant::{ConstRecognisor::ConstRecognisor,RecogniseResult::RecogniseResult},multlang::LangRes::LangRes};
use util::org::apache::poi::util::LanguageType::*;
use std::sync::LazyLock;
// java public final class TimeRecognisor {
pub struct TimeRecognisor;
// Native Pattern field declaration; each literal and declaration remains in Java order.
macro_rules! pat{($name:ident,$value:literal)=>{static $name:LazyLock<regex::Regex>=LazyLock::new(||pattern($value));};}
// java static Pattern jp_time_ptrn1 = Pattern.compile("(\\d{1,4})\\s*時\\s*(\\d{1,4})\\s*分");
pat!(jp_time_ptrn1,r"(\d{1,4})\s*時\s*(\d{1,4})\s*分");
// java static Pattern jp_time_ptrn2 = Pattern.compile("(\\d{1,4})\\s*時\\s*(\\d{1,4})\\s*分\\s*(\\d{1,4})\\s*秒");
pat!(jp_time_ptrn2,r"(\d{1,4})\s*時\s*(\d{1,4})\s*分\s*(\d{1,4})\s*秒");
// java static Pattern zh_td_time_ptrn1 = Pattern.compile("(上午|下午)(\\d{1,2})時(\\d{1,2})分");
pat!(zh_td_time_ptrn1,r"(上午|下午)(\d{1,2})時(\d{1,2})分");
// java static Pattern zh_td_time_ptrn2 = Pattern.compile("(上午|下午)(\\d{1,2})時(\\d{1,2})分(\\d{1,2})秒");
pat!(zh_td_time_ptrn2,r"(上午|下午)(\d{1,2})時(\d{1,2})分(\d{1,2})秒");
// java private static final Pattern zh_time_ptrn1 = Pattern.compile("(\\d{1,4})\\s*时\\s*(\\d{1,4})\\s*分");
pat!(zh_time_ptrn1,r"(\d{1,4})\s*时\s*(\d{1,4})\s*分");
// java private static final Pattern zh_time_ptrn2 = Pattern.compile("(\\d{1,4})\\s*时\\s*(\\d{1,4})\\s*分\\s*(\\d{1,4})\\s*秒");
pat!(zh_time_ptrn2,r"(\d{1,4})\s*时\s*(\d{1,4})\s*分\s*(\d{1,4})\s*秒");
// java private static final Pattern zh_time_ptrn3 = Pattern.compile("(\\d{1,4})\\s*:\\s*(\\d{1,4})\\s+(上午|下午)");
pat!(zh_time_ptrn3,r"(\d{1,4})\s*:\s*(\d{1,4})\s+(上午|下午)");
// java private static final Pattern zh_time_ptrn4 = Pattern.compile("(\\d{1,4})\\s*:\\s*(\\d{1,4})\\s*:\\s*(\\d{1,4})\\s+(上午|下午)");
pat!(zh_time_ptrn4,r"(\d{1,4})\s*:\s*(\d{1,4})\s*:\s*(\d{1,4})\s+(上午|下午)");
// java private static final Pattern zh_time_ptrn5 = Pattern.compile("(\\d{1,4})\\s*:\\s*(\\d{1,4}\\.\\d+)\\s+(上午|下午)");
pat!(zh_time_ptrn5,r"(\d{1,4})\s*:\s*(\d{1,4}\.\d+)\s+(上午|下午)");
// java private static final Pattern zh_time_ptrn6 = Pattern.compile("(\\d{1,4})\\s*:\\s*(\\d{1,4})\\s*:\\s*(\\d{1,4}\\.\\d+)\\s+(上午|下午)");
pat!(zh_time_ptrn6,r"(\d{1,4})\s*:\s*(\d{1,4})\s*:\s*(\d{1,4}\.\d+)\s+(上午|下午)");
// java private static final Pattern zh_time_ptrn7 = Pattern.compile("(上午|下午)(\\d{1,2})时(\\d{1,2})分");
pat!(zh_time_ptrn7,r"(上午|下午)(\d{1,2})时(\d{1,2})分");
// java private static final Pattern zh_time_ptrn8 = Pattern.compile("(上午|下午)(\\d{1,2})时(\\d{1,2})分(\\d{1,2})秒");
pat!(zh_time_ptrn8,r"(上午|下午)(\d{1,2})时(\d{1,2})分(\d{1,2})秒");
// Native Pattern/Integer/Double adapters. Parsing failures propagate as Java exceptions.
fn pattern(p:&str)->regex::Regex{crate::java_runtime::Pattern::compileMatches(p,false).expect("PatternSyntaxException")}
fn integer(s:&str)->i32{crate::java_runtime::DecimalInteger::parseInt(&s.encode_utf16().collect::<Vec<_>>()).expect("NumberFormatException")}
fn double(s:&str)->f64{crate::java_runtime::parseDouble(s).expect("NumberFormatException")}
impl TimeRecognisor {
    // java private static double makeTime(int hour, double min, double sec) { return (double)hour / (double)24.0F + min / (double)1440.0F + sec / (double)86400.0F; }
    pub fn makeTime(hour:i32,min:f64,sec:f64)->f64{hour as f64/24.0+min/1440.0+sec/86400.0}
    // java static RecogniseResult getTraditionalChineseTime(String s, int beginIndex, int endIndex) {
    pub fn getTraditionalChineseTime(s:&[u16],beginIndex:usize,endIndex:usize)->Option<RecogniseResult>{
        // java String subs = s.substring(beginIndex, endIndex); Matcher m = jp_time_ptrn1.matcher(subs); if (m.matches()) {
        let subs=String::from_utf16(&s[beginIndex..endIndex]).ok()?;
        if let Some(m)=jp_time_ptrn1.captures(&subs){
            // java int hour = Integer.parseInt(m.group(1)); int min = Integer.parseInt(m.group(2));
            let hour=integer(&m[1]);let min=integer(&m[2]);
            // java if (hour < 24 && min < 60) {
            if hour<24&&min<60{
                // java RecogniseResult ret = new RecogniseResult(makeTime(hour, (double)min, (double)0.0F)); ret.setSuggestedNumfmt(32); ret.setTimeFlag(); return ret;
                let mut ret=RecogniseResult::RecogniseResult__2(Some(Self::makeTime(hour,min as f64,0.0)));ret.setSuggestedNumfmt__2(32);ret.setTimeFlag();Some(ret)
            // java } else { return null; }
            }else{None}
        // java } else { m = jp_time_ptrn2.matcher(subs); if (m.matches()) {
        }else if let Some(m)=jp_time_ptrn2.captures(&subs){
            // java int hour = Integer.parseInt(m.group(1)); int min = Integer.parseInt(m.group(2)); int sec = Integer.parseInt(m.group(3));
            let hour=integer(&m[1]);let min=integer(&m[2]);let sec=integer(&m[3]);
            // java if (hour < 24 && min < 60 && sec < 60) {
            if hour<24&&min<60&&sec<60{
                // java RecogniseResult ret = new RecogniseResult(makeTime(hour, (double)min, (double)sec)); ret.setSuggestedNumfmt(33); ret.setTimeFlag(); return ret;
                let mut ret=RecogniseResult::RecogniseResult__2(Some(Self::makeTime(hour,min as f64,sec as f64)));ret.setSuggestedNumfmt__2(33);ret.setTimeFlag();Some(ret)
            // java } else { return null; }
            }else{None}
        // java } else { m = zh_td_time_ptrn1.matcher(subs); if (m.matches()) {
        }else if let Some(m)=zh_td_time_ptrn1.captures(&subs){
            // java boolean afternoon = m.group(1).equals("下午"); int hour = Integer.parseInt(m.group(2)); int min = Integer.parseInt(m.group(3));
            let afternoon=&m[1]=="下午";let mut hour=integer(&m[2]);let min=integer(&m[3]);
            // java if (hour <= 12 && min < 60) {
            if hour<=12&&min<60{
                // java if (hour == 12) { hour = 0; } if (afternoon) { hour += 12; }
                if hour==12{hour=0;}if afternoon{hour+=12;}
                // java RecogniseResult ret = new RecogniseResult(makeTime(hour, (double)min, (double)0.0F)); ret.setSuggestedNumfmt("上午/下午h\"時\"mm\"分\""); ret.setTimeFlag(); return ret;
                let mut ret=RecogniseResult::RecogniseResult__2(Some(Self::makeTime(hour,min as f64,0.0)));ret.setSuggestedNumfmt(Some("上午/下午h\"時\"mm\"分\"".into()));ret.setTimeFlag();Some(ret)
            // java } else { return null; }
            }else{None}
        // java } else { m = zh_td_time_ptrn2.matcher(subs); if (m.matches()) {
        }else if let Some(m)=zh_td_time_ptrn2.captures(&subs){
            // java boolean afternoon = m.group(1).equals("下午"); int hour = Integer.parseInt(m.group(2)); int min = Integer.parseInt(m.group(3)); int sec = Integer.parseInt(m.group(4));
            let afternoon=&m[1]=="下午";let mut hour=integer(&m[2]);let min=integer(&m[3]);let sec=integer(&m[4]);
            // java if (hour <= 12 && min < 60 && sec < 60) {
            if hour<=12&&min<60&&sec<60{
                // java if (hour == 12) { hour = 0; } if (afternoon) { hour += 12; }
                if hour==12{hour=0;}if afternoon{hour+=12;}
                // java RecogniseResult ret = new RecogniseResult(makeTime(hour, (double)min, (double)sec)); ret.setSuggestedNumfmt("上午/下午h\"時\"mm\"分\"ss\"秒\""); ret.setTimeFlag(); return ret;
                let mut ret=RecogniseResult::RecogniseResult__2(Some(Self::makeTime(hour,min as f64,sec as f64)));ret.setSuggestedNumfmt(Some("上午/下午h\"時\"mm\"分\"ss\"秒\"".into()));ret.setTimeFlag();Some(ret)
            // java } else { return null; }
            }else{None}
        // java } else { return null; }
        }else{None}
    }
    // java static RecogniseResult getSimplifiedChineseTime(String s, int beginIndex, int endIndex) {
    pub fn getSimplifiedChineseTime(s:&[u16],beginIndex:usize,endIndex:usize)->Option<RecogniseResult>{
        // java String subs = s.substring(beginIndex, endIndex); Matcher m = zh_time_ptrn1.matcher(subs); if (m.matches()) {
        let subs=String::from_utf16(&s[beginIndex..endIndex]).ok()?;
        if let Some(m)=zh_time_ptrn1.captures(&subs){
            // java int hour = Integer.parseInt(m.group(1)); int min = Integer.parseInt(m.group(2));
            let hour=integer(&m[1]);let min=integer(&m[2]);
            // java if (hour < 24 && min < 60) {
            if hour<24&&min<60{
                // java RecogniseResult ret = new RecogniseResult(makeTime(hour, (double)min, (double)0.0F)); ret.setSuggestedNumfmt(32); ret.setTimeFlag(); return ret;
                let mut ret=RecogniseResult::RecogniseResult__2(Some(Self::makeTime(hour,min as f64,0.0)));ret.setSuggestedNumfmt__2(32);ret.setTimeFlag();Some(ret)
            // java } else { return null; }
            }else{None}
        // java } else { m = zh_time_ptrn2.matcher(subs); if (m.matches()) {
        }else if let Some(m)=zh_time_ptrn2.captures(&subs){
            // java int hour = Integer.parseInt(m.group(1)); int min = Integer.parseInt(m.group(2)); int sec = Integer.parseInt(m.group(3));
            let hour=integer(&m[1]);let min=integer(&m[2]);let sec=integer(&m[3]);
            // java if (hour < 24 && min < 60 && sec < 60) {
            if hour<24&&min<60&&sec<60{
                // java RecogniseResult ret = new RecogniseResult(makeTime(hour, (double)min, (double)sec)); ret.setSuggestedNumfmt(33); ret.setTimeFlag(); return ret;
                let mut ret=RecogniseResult::RecogniseResult__2(Some(Self::makeTime(hour,min as f64,sec as f64)));ret.setSuggestedNumfmt__2(33);ret.setTimeFlag();Some(ret)
            // java } else { return null; }
            }else{None}
        // java } else { m = zh_time_ptrn3.matcher(subs); if (m.matches()) {
        }else if let Some(m)=zh_time_ptrn3.captures(&subs){
            // java int hour = Integer.parseInt(m.group(1)); int min = Integer.parseInt(m.group(2));
            let mut hour=integer(&m[1]);let min=integer(&m[2]);
            // java if (hour <= 12 && min < 60) {
            if hour<=12&&min<60{
                // java if (hour == 12) { hour = 0; } String apm = m.group(3); if (apm.charAt(0) == 19979) { hour += 12; }
                if hour==12{hour=0;}let apm=&m[3];if apm.encode_utf16().next()==Some(19979){hour+=12;}
                // java RecogniseResult ret = new RecogniseResult(makeTime(hour, (double)min, (double)0.0F)); ret.setSuggestedNumfmt(18); ret.setTimeFlag(); return ret;
                let mut ret=RecogniseResult::RecogniseResult__2(Some(Self::makeTime(hour,min as f64,0.0)));ret.setSuggestedNumfmt__2(18);ret.setTimeFlag();Some(ret)
            // java } else { return null; }
            }else{None}
        // java } else { m = zh_time_ptrn4.matcher(subs); if (m.matches()) {
        }else if let Some(m)=zh_time_ptrn4.captures(&subs){
            // java int hour = Integer.parseInt(m.group(1)); int min = Integer.parseInt(m.group(2)); int sec = Integer.parseInt(m.group(3));
            let mut hour=integer(&m[1]);let min=integer(&m[2]);let sec=integer(&m[3]);
            // java if (hour <= 12 && min < 60 && sec < 60) {
            if hour<=12&&min<60&&sec<60{
                // java if (hour == 12) { hour = 0; } String apm = m.group(4); if (apm.charAt(0) == 19979) { hour += 12; }
                if hour==12{hour=0;}let apm=&m[4];if apm.encode_utf16().next()==Some(19979){hour+=12;}
                // java RecogniseResult ret = new RecogniseResult(makeTime(hour, (double)min, (double)sec)); ret.setSuggestedNumfmt(19); ret.setTimeFlag(); return ret;
                let mut ret=RecogniseResult::RecogniseResult__2(Some(Self::makeTime(hour,min as f64,sec as f64)));ret.setSuggestedNumfmt__2(19);ret.setTimeFlag();Some(ret)
            // java } else { return null; }
            }else{None}
        // java } else { m = zh_time_ptrn5.matcher(subs); if (m.matches()) {
        }else if let Some(m)=zh_time_ptrn5.captures(&subs){
            // java int min = Integer.parseInt(m.group(1)); double sec = Double.parseDouble(m.group(2));
            let min=integer(&m[1]);let sec=double(&m[2]);
            // java if (min < 60 && !(sec >= (double)60.0F)) {
            if min<60&&!(sec>=60.0){
                // java String apm = m.group(3); int hour = apm.charAt(0) == 19979 ? 12 : 0;
                let apm=&m[3];let hour=if apm.encode_utf16().next()==Some(19979){12}else{0};
                // java RecogniseResult ret = new RecogniseResult(makeTime(hour, (double)min, sec)); ret.setSuggestedNumfmt(47); ret.setTimeFlag(); return ret;
                let mut ret=RecogniseResult::RecogniseResult__2(Some(Self::makeTime(hour,min as f64,sec)));ret.setSuggestedNumfmt__2(47);ret.setTimeFlag();Some(ret)
            // java } else { return null; }
            }else{None}
        // java } else { m = zh_time_ptrn6.matcher(subs); if (m.matches()) {
        }else if let Some(m)=zh_time_ptrn6.captures(&subs){
            // java int hour = Integer.parseInt(m.group(1)); int min = Integer.parseInt(m.group(2)); double sec = Double.parseDouble(m.group(3));
            let mut hour=integer(&m[1]);let min=integer(&m[2]);let sec=double(&m[3]);
            // java if (hour <= 12 && min < 60 && !(sec >= (double)60.0F)) {
            if hour<=12&&min<60&&!(sec>=60.0){
                // java if (hour == 12) { hour = 0; } String apm = m.group(4); if (apm.charAt(0) == 19979) { hour += 12; }
                if hour==12{hour=0;}let apm=&m[4];if apm.encode_utf16().next()==Some(19979){hour+=12;}
                // java RecogniseResult ret = new RecogniseResult(makeTime(hour, (double)min, sec)); ret.setSuggestedNumfmt(47); ret.setTimeFlag(); return ret;
                let mut ret=RecogniseResult::RecogniseResult__2(Some(Self::makeTime(hour,min as f64,sec)));ret.setSuggestedNumfmt__2(47);ret.setTimeFlag();Some(ret)
            // java } else { return null; }
            }else{None}
        // java } else { m = zh_time_ptrn7.matcher(subs); if (m.matches()) {
        }else if let Some(m)=zh_time_ptrn7.captures(&subs){
            // java boolean afternoon = m.group(1).equals("下午"); int hour = Integer.parseInt(m.group(2)); int min = Integer.parseInt(m.group(3));
            let afternoon=&m[1]=="下午";let mut hour=integer(&m[2]);let min=integer(&m[3]);
            // java if (hour <= 12 && min < 60) {
            if hour<=12&&min<60{
                // java if (hour == 12) { hour = 0; } if (afternoon) { hour += 12; }
                if hour==12{hour=0;}if afternoon{hour+=12;}
                // java RecogniseResult ret = new RecogniseResult(makeTime(hour, (double)min, (double)0.0F)); ret.setSuggestedNumfmt("上午/下午h\"时\"mm\"分\""); ret.setTimeFlag(); return ret;
                let mut ret=RecogniseResult::RecogniseResult__2(Some(Self::makeTime(hour,min as f64,0.0)));ret.setSuggestedNumfmt(Some("上午/下午h\"时\"mm\"分\"".into()));ret.setTimeFlag();Some(ret)
            // java } else { return null; }
            }else{None}
        // java } else { m = zh_time_ptrn8.matcher(subs); if (m.matches()) {
        }else if let Some(m)=zh_time_ptrn8.captures(&subs){
            // java boolean afternoon = m.group(1).equals("下午"); int hour = Integer.parseInt(m.group(2)); int min = Integer.parseInt(m.group(3)); int sec = Integer.parseInt(m.group(4));
            let afternoon=&m[1]=="下午";let mut hour=integer(&m[2]);let min=integer(&m[3]);let sec=integer(&m[4]);
            // java if (hour <= 12 && min < 60 && sec < 60) {
            if hour<=12&&min<60&&sec<60{
                // java if (hour == 12) { hour = 0; } if (afternoon) { hour += 12; }
                if hour==12{hour=0;}if afternoon{hour+=12;}
                // java RecogniseResult ret = new RecogniseResult(makeTime(hour, (double)min, (double)sec)); ret.setSuggestedNumfmt("上午/下午h\"时\"mm\"分\"ss\"秒\""); ret.setTimeFlag(); return ret;
                let mut ret=RecogniseResult::RecogniseResult__2(Some(Self::makeTime(hour,min as f64,sec as f64)));ret.setSuggestedNumfmt(Some("上午/下午h\"时\"mm\"分\"ss\"秒\"".into()));ret.setTimeFlag();Some(ret)
            // java } else { return null; }
            }else{None}
        // java } else { return null; }
        }else{None}
    }
    // java static RecogniseResult getKoreanTime(String s, int beginIndex, int endIndex) {
    pub fn getKoreanTime(s:&[u16],beginIndex:usize,endIndex:usize)->Option<RecogniseResult>{
        // java String subs = s.substring(beginIndex, endIndex); Pattern korean_time_ptrn1 = Pattern.compile("(\\d{1,4})시 (\\d{1,4})분 (\\d{1,4})초"); Matcher m = korean_time_ptrn1.matcher(subs);
        let subs=String::from_utf16(&s[beginIndex..endIndex]).ok()?;let korean_time_ptrn1=pattern(r"(\d{1,4})시 (\d{1,4})분 (\d{1,4})초");
        // java if (m.matches()) {
        if let Some(m)=korean_time_ptrn1.captures(&subs){
            // java int hour = Integer.parseInt(m.group(1)); int min = Integer.parseInt(m.group(2)); int sec = Integer.parseInt(m.group(3));
            let hour=integer(&m[1]);let min=integer(&m[2]);let sec=integer(&m[3]);
            // java if (hour < 24 && min < 60 && sec < 60) {
            if hour<24&&min<60&&sec<60{
                // java RecogniseResult ret = new RecogniseResult(makeTime(hour, (double)min, (double)sec)); ret.setSuggestedNumfmt("h\"시\" mm\"분\" ss\"초\""); ret.setTimeFlag(); return ret;
                let mut ret=RecogniseResult::RecogniseResult__2(Some(Self::makeTime(hour,min as f64,sec as f64)));ret.setSuggestedNumfmt(Some("h\"시\" mm\"분\" ss\"초\"".into()));ret.setTimeFlag();Some(ret)
            // java } else { return null; }
            }else{None}
        // java } else {
        }else{
            // java Pattern korean_time_ptrn2 = Pattern.compile("(\\d{1,4})시 (\\d{1,4})분"); m = korean_time_ptrn2.matcher(subs); if (m.matches()) {
            let korean_time_ptrn2=pattern(r"(\d{1,4})시 (\d{1,4})분");
            if let Some(m)=korean_time_ptrn2.captures(&subs){
                // java int hour = Integer.parseInt(m.group(1)); int min = Integer.parseInt(m.group(2));
                let hour=integer(&m[1]);let min=integer(&m[2]);
                // java if (hour < 24 && min < 60) {
                if hour<24&&min<60{
                    // java RecogniseResult ret = new RecogniseResult(makeTime(hour, (double)min, (double)0.0F)); ret.setSuggestedNumfmt("h\"시\" mm\"분\""); ret.setTimeFlag(); return ret;
                    let mut ret=RecogniseResult::RecogniseResult__2(Some(Self::makeTime(hour,min as f64,0.0)));ret.setSuggestedNumfmt(Some("h\"시\" mm\"분\"".into()));ret.setTimeFlag();Some(ret)
                // java } else { return null; }
                }else{None}
            // java } else { return null; }
            }else{None}
        }
    }
    // java public static RecogniseResult getCommonTime(String s, int beginIndex, int endIndex, char decimalSeparator) {
    pub fn getCommonTime(s:&[u16],beginIndex:usize,endIndex:usize,decimalSeparator:u16)->Option<RecogniseResult>{
        // java String subs = s.substring(beginIndex, endIndex);
        let subs=String::from_utf16(&s[beginIndex..endIndex]).ok()?;
        // java Pattern common_time_ptrn1 = Pattern.compile((new StringBuilder(40)).append("(\\d{1,4})\\s*:\\s*(\\d{1,4})").append(ConstRecognisor._wrapRegChar(decimalSeparator)).append('?').toString());
        let dec=String::from_utf16(&ConstRecognisor::_wrapRegChar(decimalSeparator)).unwrap();let common_time_ptrn1=pattern(&format!(r"(\d{{1,4}})\s*:\s*(\d{{1,4}}){dec}?"));
        // java Matcher m = common_time_ptrn1.matcher(subs); if (m.matches()) {
        if let Some(m)=common_time_ptrn1.captures(&subs){
            // java int hour = Integer.parseInt(m.group(1)); int min = Integer.parseInt(m.group(2));
            let hour=integer(&m[1]);let min=integer(&m[2]);
            // java if (hour >= 24 && min >= 60) { return null; } else {
            if hour>=24&&min>=60{None}else{
                // java RecogniseResult ret = new RecogniseResult(makeTime(hour, (double)min, (double)0.0F));
                let mut ret=RecogniseResult::RecogniseResult__2(Some(Self::makeTime(hour,min as f64,0.0)));
                // java if (hour >= 24) { ret.setSuggestedNumfmt(46); } else { ret.setSuggestedNumfmt(20); }
                if hour>=24{ret.setSuggestedNumfmt__2(46);}else{ret.setSuggestedNumfmt__2(20);}
                // java ret.setTimeFlag(); return ret;
                ret.setTimeFlag();Some(ret)
            }
        // java } else {
        }else{
            // java Pattern common_time_ptrn2 = Pattern.compile((new StringBuilder(40)).append("(\\d{1,4})\\s*:\\s*(\\d{1,4})\\s*:\\s*(\\d{1,4})").append(ConstRecognisor._wrapRegChar(decimalSeparator)).append('?').toString()); m = common_time_ptrn2.matcher(subs);
            let common_time_ptrn2=pattern(&format!(r"(\d{{1,4}})\s*:\s*(\d{{1,4}})\s*:\s*(\d{{1,4}}){dec}?"));
            // java if (m.matches()) {
            if let Some(m)=common_time_ptrn2.captures(&subs){
                // java int hour = Integer.parseInt(m.group(1)); int min = Integer.parseInt(m.group(2)); int sec = Integer.parseInt(m.group(3));
                let hour=integer(&m[1]);let min=integer(&m[2]);let sec=integer(&m[3]);
                // java if ((hour < 24 || min < 60 && sec < 60) && (min < 60 || sec < 60)) {
                if (hour<24||min<60&&sec<60)&&(min<60||sec<60){
                    // java RecogniseResult ret = new RecogniseResult(makeTime(hour, (double)min, (double)sec));
                    let mut ret=RecogniseResult::RecogniseResult__2(Some(Self::makeTime(hour,min as f64,sec as f64)));
                    // java if (hour >= 24) { ret.setSuggestedNumfmt(46); } else if (sec % 60 == 0) { ret.setSuggestedNumfmt(20); } else { ret.setSuggestedNumfmt(21); }
                    if hour>=24{ret.setSuggestedNumfmt__2(46);}else if sec%60==0{ret.setSuggestedNumfmt__2(20);}else{ret.setSuggestedNumfmt__2(21);}
                    // java ret.setTimeFlag(); return ret;
                    ret.setTimeFlag();Some(ret)
                // java } else { return null; }
                }else{None}
            // java } else {
            }else{
                // java Pattern common_time_ptrn5 = Pattern.compile((new StringBuilder(50)).append("(\\d{1,4})\\s*:\\s*(\\d{1,4})").append(ConstRecognisor._wrapRegChar(decimalSeparator)).append("(\\d+)").toString()); m = common_time_ptrn5.matcher(subs);
                let common_time_ptrn5=pattern(&format!(r"(\d{{1,4}})\s*:\s*(\d{{1,4}}){dec}(\d+)"));
                // java if (m.matches()) {
                if let Some(m)=common_time_ptrn5.captures(&subs){
                    // java int min = Integer.parseInt(m.group(1)); double sec = Double.parseDouble(m.group(2) + "." + m.group(3));
                    let min=integer(&m[1]);let sec=double(&format!("{}.{}",&m[2],&m[3]));
                    // java if (min >= 60 && sec >= (double)60.0F) { return null; } else {
                    if min>=60&&sec>=60.0{None}else{
                        // java RecogniseResult ret = new RecogniseResult(makeTime(0, (double)min, sec)); ret.setSuggestedNumfmt(47); ret.setTimeFlag(); return ret;
                        let mut ret=RecogniseResult::RecogniseResult__2(Some(Self::makeTime(0,min as f64,sec)));ret.setSuggestedNumfmt__2(47);ret.setTimeFlag();Some(ret)
                    }
                // java } else {
                }else{
                    // java Pattern common_time_ptrn6 = Pattern.compile((new StringBuilder(90)).append("(\\d{1,4})\\s*:\\s*(\\d{1,4})\\s*:\\s*(\\d{1,4})").append(ConstRecognisor._wrapRegChar(decimalSeparator)).append("(\\d+)").toString()); m = common_time_ptrn6.matcher(subs);
                    let common_time_ptrn6=pattern(&format!(r"(\d{{1,4}})\s*:\s*(\d{{1,4}})\s*:\s*(\d{{1,4}}){dec}(\d+)"));let m=common_time_ptrn6.captures(&subs);
                    // java if (!m.matches()) { return null; } else {
                    if m.is_none(){None}else{
                        let m=m.unwrap();
                        // java int hour = Integer.parseInt(m.group(1)); int min = Integer.parseInt(m.group(2)); double sec = Double.parseDouble(m.group(3) + "." + m.group(4));
                        let hour=integer(&m[1]);let min=integer(&m[2]);let sec=double(&format!("{}.{}",&m[3],&m[4]));
                        // java if ((hour < 24 || min < 60 && !(sec >= (double)60.0F)) && (min < 60 || !(sec >= (double)60.0F))) {
                        if (hour<24||min<60&&!(sec>=60.0))&&(min<60||!(sec>=60.0)){
                            // java RecogniseResult ret = new RecogniseResult(makeTime(hour, (double)min, sec));
                            let mut ret=RecogniseResult::RecogniseResult__2(Some(Self::makeTime(hour,min as f64,sec)));
                            // java if (hour >= 24) { ret.setSuggestedNumfmt(46); } else { ret.setSuggestedNumfmt(47); }
                            if hour>=24{ret.setSuggestedNumfmt__2(46);}else{ret.setSuggestedNumfmt__2(47);}
                            // java ret.setTimeFlag(); return ret;
                            ret.setTimeFlag();Some(ret)
                        // java } else { return null; }
                        }else{None}
                    }
                }
            }
        }
    }
    // java public static RecogniseResult getEnglishAMPMTime(String s, int beginIndex, int endIndex, char decimalSeparator) {
    pub fn getEnglishAMPMTime(s:&[u16],beginIndex:usize,endIndex:usize,decimalSeparator:u16)->Option<RecogniseResult>{
        // java String subs = s.substring(beginIndex, endIndex);
        let subs=String::from_utf16(&s[beginIndex..endIndex]).ok()?;
        // java Pattern en_ampm_time_ptrn1 = Pattern.compile((new StringBuilder(100)).append("(\\d{1,4})\\s*:\\s*(\\d{1,4})\\s+(am|pm|a|p|AM|PM)").toString()); Matcher m = en_ampm_time_ptrn1.matcher(subs);
        let en_ampm_time_ptrn1=pattern(r"(\d{1,4})\s*:\s*(\d{1,4})\s+(am|pm|a|p|AM|PM)");
        // java if (m.matches()) {
        if let Some(m)=en_ampm_time_ptrn1.captures(&subs){
            // java int hour = Integer.parseInt(m.group(1)); int min = Integer.parseInt(m.group(2));
            let mut hour=integer(&m[1]);let min=integer(&m[2]);
            // java if (hour <= 12 && min < 60) {
            if hour<=12&&min<60{
                // java if (hour == 12) { hour = 0; } String apm = m.group(3); if (apm.startsWith("p") || apm.startsWith("P")) { hour += 12; }
                if hour==12{hour=0;}let apm=&m[3];if apm.starts_with('p')||apm.starts_with('P'){hour+=12;}
                // java RecogniseResult ret = new RecogniseResult(makeTime(hour, (double)min, (double)0.0F)); ret.setSuggestedNumfmt(18); ret.setTimeFlag(); return ret;
                let mut ret=RecogniseResult::RecogniseResult__2(Some(Self::makeTime(hour,min as f64,0.0)));ret.setSuggestedNumfmt__2(18);ret.setTimeFlag();Some(ret)
            // java } else { return null; }
            }else{None}
        // java } else {
        }else{
            // java Pattern en_ampm_time_ptrn2 = Pattern.compile((new StringBuilder(100)).append("(\\d{1,4})\\s*:\\s*(\\d{1,4})\\s*:\\s*(\\d{1,4})\\s+(am|pm|a|p|AM|PM)").toString()); m = en_ampm_time_ptrn2.matcher(subs);
            let en_ampm_time_ptrn2=pattern(r"(\d{1,4})\s*:\s*(\d{1,4})\s*:\s*(\d{1,4})\s+(am|pm|a|p|AM|PM)");
            // java if (m.matches()) {
            if let Some(m)=en_ampm_time_ptrn2.captures(&subs){
                // java int hour = Integer.parseInt(m.group(1)); int min = Integer.parseInt(m.group(2)); int sec = Integer.parseInt(m.group(3));
                let mut hour=integer(&m[1]);let min=integer(&m[2]);let sec=integer(&m[3]);
                // java if (hour <= 12 && min < 60 && sec < 60) {
                if hour<=12&&min<60&&sec<60{
                    // java if (hour == 12) { hour = 0; } String apm = m.group(4); if (apm.startsWith("p") || apm.startsWith("P")) { hour += 12; }
                    if hour==12{hour=0;}let apm=&m[4];if apm.starts_with('p')||apm.starts_with('P'){hour+=12;}
                    // java RecogniseResult ret = new RecogniseResult(makeTime(hour, (double)min, (double)sec));
                    let mut ret=RecogniseResult::RecogniseResult__2(Some(Self::makeTime(hour,min as f64,sec as f64)));
                    // java if (sec % 60 == 0) { ret.setSuggestedNumfmt(18); } else { ret.setSuggestedNumfmt(19); }
                    if sec%60==0{ret.setSuggestedNumfmt__2(18);}else{ret.setSuggestedNumfmt__2(19);}
                    // java ret.setTimeFlag(); return ret;
                    ret.setTimeFlag();Some(ret)
                // java } else { return null; }
                }else{None}
            // java } else {
            }else{
                // java Pattern common_time_ptrn7 = Pattern.compile((new StringBuilder(100)).append("(\\d{1,4})\\s*:\\s*(\\d{1,4})").append(ConstRecognisor._wrapRegChar(decimalSeparator)).append("(\\d+)\\s+(am|pm|a|p|AM|PM)").toString()); m = common_time_ptrn7.matcher(subs);
                let dec=String::from_utf16(&ConstRecognisor::_wrapRegChar(decimalSeparator)).unwrap();let common_time_ptrn7=pattern(&format!(r"(\d{{1,4}})\s*:\s*(\d{{1,4}}){dec}(\d+)\s+(am|pm|a|p|AM|PM)"));
                // java if (m.matches()) {
                if let Some(m)=common_time_ptrn7.captures(&subs){
                    // java int min = Integer.parseInt(m.group(1)); double sec = Double.parseDouble(m.group(2) + "." + m.group(3));
                    let min=integer(&m[1]);let sec=double(&format!("{}.{}",&m[2],&m[3]));
                    // java if (min < 60 && !(sec >= (double)60.0F)) {
                    if min<60&&!(sec>=60.0){
                        // java int hour = 0; String apm = m.group(3); if (apm.startsWith("p") || apm.startsWith("P")) { hour += 12; }
                        let mut hour=0;let apm=&m[3];if apm.starts_with('p')||apm.starts_with('P'){hour+=12;}
                        // java RecogniseResult ret = new RecogniseResult(makeTime(hour, (double)min, sec)); ret.setSuggestedNumfmt(47); ret.setTimeFlag(); return ret;
                        let mut ret=RecogniseResult::RecogniseResult__2(Some(Self::makeTime(hour,min as f64,sec)));ret.setSuggestedNumfmt__2(47);ret.setTimeFlag();Some(ret)
                    // java } else { return null; }
                    }else{None}
                // java } else {
                }else{
                    // java Pattern common_time_ptrn8 = Pattern.compile((new StringBuilder(90)).append("(\\d{1,4})\\s*:\\s*(\\d{1,4})\\s*:\\s*(\\d{1,4})").append(ConstRecognisor._wrapRegChar(decimalSeparator)).append("(\\d+)\\s+(am|pm|a|p|AM|PM)").toString()); m = common_time_ptrn8.matcher(subs);
                    let common_time_ptrn8=pattern(&format!(r"(\d{{1,4}})\s*:\s*(\d{{1,4}})\s*:\s*(\d{{1,4}}){dec}(\d+)\s+(am|pm|a|p|AM|PM)"));let m=common_time_ptrn8.captures(&subs);
                    // java if (!m.matches()) { return null; } else {
                    if m.is_none(){None}else{
                        let m=m.unwrap();
                        // java int hour = Integer.parseInt(m.group(1)); int min = Integer.parseInt(m.group(2)); double sec = Double.parseDouble(m.group(3) + "." + m.group(4));
                        let mut hour=integer(&m[1]);let min=integer(&m[2]);let sec=double(&format!("{}.{}",&m[3],&m[4]));
                        // java if (hour <= 12 && min < 60 && !(sec >= (double)60.0F)) {
                        if hour<=12&&min<60&&!(sec>=60.0){
                            // java if (hour == 12) { hour = 0; } String apm = m.group(4); if (apm.startsWith("p") || apm.startsWith("P")) { hour += 12; }
                            if hour==12{hour=0;}let apm=&m[4];if apm.starts_with('p')||apm.starts_with('P'){hour+=12;}
                            // java RecogniseResult ret = new RecogniseResult(makeTime(hour, (double)min, sec)); ret.setSuggestedNumfmt(47); ret.setTimeFlag(); return ret;
                            let mut ret=RecogniseResult::RecogniseResult__2(Some(Self::makeTime(hour,min as f64,sec)));ret.setSuggestedNumfmt__2(47);ret.setTimeFlag();Some(ret)
                        // java } else { return null; }
                        }else{None}
                    }
                }
            }
        }
    }
    // java public static RecogniseResult getAMPMTime(String s, int beginIndex, int endIndex, char decimalSeparator, String am, String pm) {
    pub fn getAMPMTime(s:&[u16],beginIndex:usize,endIndex:usize,decimalSeparator:u16,am:&str,pm:&str)->Option<RecogniseResult>{
        // java String subs = s.substring(beginIndex, endIndex);
        let subs=String::from_utf16(&s[beginIndex..endIndex]).ok()?;
        // java Pattern ampm_time_ptrn1 = Pattern.compile((new StringBuilder(100)).append("(\\d{1,4})\\s*:\\s*(\\d{1,4})\\s+(").append(am).append("|").append(pm).append(")").toString()); Matcher m = ampm_time_ptrn1.matcher(subs);
        let ampm_time_ptrn1=pattern(&format!(r"(\d{{1,4}})\s*:\s*(\d{{1,4}})\s+({am}|{pm})"));
        // java if (m.matches()) {
        if let Some(m)=ampm_time_ptrn1.captures(&subs){
            // java int hour = Integer.parseInt(m.group(1)); int min = Integer.parseInt(m.group(2));
            let mut hour=integer(&m[1]);let min=integer(&m[2]);
            // java if (hour <= 12 && min < 60) {
            if hour<=12&&min<60{
                // java if (hour == 12) { hour = 0; } String apm = m.group(3); if (apm.equalsIgnoreCase(pm)) { hour += 12; }
                if hour==12{hour=0;}let apm=&m[3];if crate::java_runtime::Character::equalsIgnoreCase(&apm.encode_utf16().collect::<Vec<_>>(),&pm.encode_utf16().collect::<Vec<_>>()){hour+=12;}
                // java RecogniseResult ret = new RecogniseResult(makeTime(hour, (double)min, (double)0.0F)); ret.setSuggestedNumfmt(18); ret.setTimeFlag(); return ret;
                let mut ret=RecogniseResult::RecogniseResult__2(Some(Self::makeTime(hour,min as f64,0.0)));ret.setSuggestedNumfmt__2(18);ret.setTimeFlag();Some(ret)
            // java } else { return null; }
            }else{None}
        // java } else {
        }else{
            // java Pattern ampm_time_ptrn2 = Pattern.compile((new StringBuilder(100)).append("(\\d{1,4})\\s*:\\s*(\\d{1,4})\\s*:\\s*(\\d{1,4})\\s+(").append(am).append("|").append(pm).append(")").toString()); m = ampm_time_ptrn2.matcher(subs);
            let ampm_time_ptrn2=pattern(&format!(r"(\d{{1,4}})\s*:\s*(\d{{1,4}})\s*:\s*(\d{{1,4}})\s+({am}|{pm})"));
            // java if (m.matches()) {
            if let Some(m)=ampm_time_ptrn2.captures(&subs){
                // java int hour = Integer.parseInt(m.group(1)); int min = Integer.parseInt(m.group(2)); int sec = Integer.parseInt(m.group(3));
                let mut hour=integer(&m[1]);let min=integer(&m[2]);let sec=integer(&m[3]);
                // java if (hour <= 12 && min < 60 && sec < 60) {
                if hour<=12&&min<60&&sec<60{
                    // java if (hour == 12) { hour = 0; } String apm = m.group(4); if (apm.equalsIgnoreCase(pm)) { hour += 12; }
                    if hour==12{hour=0;}let apm=&m[4];if crate::java_runtime::Character::equalsIgnoreCase(&apm.encode_utf16().collect::<Vec<_>>(),&pm.encode_utf16().collect::<Vec<_>>()){hour+=12;}
                    // java RecogniseResult ret = new RecogniseResult(makeTime(hour, (double)min, (double)sec));
                    let mut ret=RecogniseResult::RecogniseResult__2(Some(Self::makeTime(hour,min as f64,sec as f64)));
                    // java if (sec % 60 == 0) { ret.setSuggestedNumfmt(18); } else { ret.setSuggestedNumfmt(19); }
                    if sec%60==0{ret.setSuggestedNumfmt__2(18);}else{ret.setSuggestedNumfmt__2(19);}
                    // java ret.setTimeFlag(); return ret;
                    ret.setTimeFlag();Some(ret)
                // java } else { return null; }
                }else{None}
            // java } else {
            }else{
                // java Pattern ampm_time_ptr3 = Pattern.compile((new StringBuilder(100)).append("(\\d{1,4})\\s*:\\s*(\\d{1,4})").append(ConstRecognisor._wrapRegChar(decimalSeparator)).append("(\\d+)\\s+(").append(am).append("|").append(pm).append(")").toString()); m = ampm_time_ptr3.matcher(subs);
                let dec=String::from_utf16(&ConstRecognisor::_wrapRegChar(decimalSeparator)).unwrap();let ampm_time_ptr3=pattern(&format!(r"(\d{{1,4}})\s*:\s*(\d{{1,4}}){dec}(\d+)\s+({am}|{pm})"));
                // java if (m.matches()) {
                if let Some(m)=ampm_time_ptr3.captures(&subs){
                    // java int min = Integer.parseInt(m.group(1)); double sec = Double.parseDouble(m.group(2) + '.' + m.group(3));
                    let min=integer(&m[1]);let sec=double(&format!("{}.{}",&m[2],&m[3]));
                    // java if (min < 60 && !(sec >= (double)60.0F)) {
                    if min<60&&!(sec>=60.0){
                        // java int hour = 0; String apm = m.group(3); if (apm.equalsIgnoreCase(pm)) { hour += 12; }
                        let mut hour=0;let apm=&m[3];if crate::java_runtime::Character::equalsIgnoreCase(&apm.encode_utf16().collect::<Vec<_>>(),&pm.encode_utf16().collect::<Vec<_>>()){hour+=12;}
                        // java RecogniseResult ret = new RecogniseResult(makeTime(hour, (double)min, sec)); ret.setSuggestedNumfmt(47); ret.setTimeFlag(); return ret;
                        let mut ret=RecogniseResult::RecogniseResult__2(Some(Self::makeTime(hour,min as f64,sec)));ret.setSuggestedNumfmt__2(47);ret.setTimeFlag();Some(ret)
                    // java } else { return null; }
                    }else{None}
                // java } else {
                }else{
                    // java Pattern ampm_time_ptr4 = Pattern.compile((new StringBuilder(100)).append("(\\d{1,4})\\s*:\\s*(\\d{1,4})\\s*:\\s*(\\d{1,4})").append(ConstRecognisor._wrapRegChar(decimalSeparator)).append("(\\d+)\\s+(").append(am).append("|").append(pm).append(")").toString()); m = ampm_time_ptr4.matcher(subs);
                    let ampm_time_ptr4=pattern(&format!(r"(\d{{1,4}})\s*:\s*(\d{{1,4}})\s*:\s*(\d{{1,4}}){dec}(\d+)\s+({am}|{pm})"));
                    // java if (m.matches()) {
                    if let Some(m)=ampm_time_ptr4.captures(&subs){
                        // java int hour = Integer.parseInt(m.group(1)); int min = Integer.parseInt(m.group(2)); double sec = Double.parseDouble(m.group(3) + '.' + m.group(4));
                        let mut hour=integer(&m[1]);let min=integer(&m[2]);let sec=double(&format!("{}.{}",&m[3],&m[4]));
                        // java if (hour <= 12 && min < 60 && !(sec >= (double)60.0F)) {
                        if hour<=12&&min<60&&!(sec>=60.0){
                            // java if (hour == 12) { hour = 0; } String apm = m.group(4); if (apm.equalsIgnoreCase(pm)) { hour += 12; }
                            if hour==12{hour=0;}let apm=&m[4];if crate::java_runtime::Character::equalsIgnoreCase(&apm.encode_utf16().collect::<Vec<_>>(),&pm.encode_utf16().collect::<Vec<_>>()){hour+=12;}
                            // java RecogniseResult ret = new RecogniseResult(makeTime(hour, (double)min, sec)); ret.setSuggestedNumfmt(47); ret.setTimeFlag(); return ret;
                            let mut ret=RecogniseResult::RecogniseResult__2(Some(Self::makeTime(hour,min as f64,sec)));ret.setSuggestedNumfmt__2(47);ret.setTimeFlag();Some(ret)
                        // java } else { return null; }
                        }else{None}
                    // java } else { return null; }
                    }else{None}
                }
            }
        }
    }
    // java public static RecogniseResult getTime(String s, int beginIndex, int endIndex, LanguageType lang, LangRes langRes) {
    pub fn getTime(s:&[u16],beginIndex:usize,endIndex:usize,lang:LanguageType,langRes:&LangRes)->Option<RecogniseResult>{
        // java RecogniseResult rs = null;
        let mut rs=None;
        // java if (lang == LanguageType.LANGUAGE_CHINESE_SIMPLIFIED) {
        if lang==LANGUAGE_CHINESE_SIMPLIFIED{
            // java rs = getSimplifiedChineseTime(s, 0, s.length()); if (rs != null) { return rs; }
            rs=Self::getSimplifiedChineseTime(s,0,s.len());if rs.is_some(){return rs;}
        // java } else if (lang != LanguageType.LANGUAGE_JAPANESE && lang != LanguageType.LANGUAGE_CHINESE_TRADITIONAL) {
        }else if lang!=LANGUAGE_JAPANESE&&lang!=LANGUAGE_CHINESE_TRADITIONAL{
            // java if (lang == LanguageType.LANGUAGE_KOREAN || lang == LanguageType.LANGUAGE_KOREAN_JOHAB) {
            if lang==LANGUAGE_KOREAN||lang==LANGUAGE_KOREAN_JOHAB{
                // java rs = getKoreanTime(s, 0, s.length()); if (rs != null) { return rs; }
                rs=Self::getKoreanTime(s,0,s.len());if rs.is_some(){return rs;}
            }
        // java } else { rs = getTraditionalChineseTime(s, 0, s.length()); if (rs != null) { return rs; } }
        }else{rs=Self::getTraditionalChineseTime(s,0,s.len());if rs.is_some(){return rs;}}
        // java rs = getCommonTime(s, 0, s.length(), langRes.SEPERATORS.DECIMAL_SEPARATOR);
        rs=Self::getCommonTime(s,0,s.len(),langRes.SEPERATORS.DECIMAL_SEPARATOR);
        // java if (rs != null) { return rs; } else {
        if rs.is_some(){rs}else{
            // java rs = getEnglishAMPMTime(s, beginIndex, endIndex, langRes.SEPERATORS.DECIMAL_SEPARATOR);
            rs=Self::getEnglishAMPMTime(s,beginIndex,endIndex,langRes.SEPERATORS.DECIMAL_SEPARATOR);
            // java if (rs != null) { return rs; } else {
            if rs.is_some(){rs}else{
                // java if (langRes.AM != null && !langRes.AM.equalsIgnoreCase("AM")) {
                if langRes.AM.as_ref().is_some_and(|am|!crate::java_runtime::Character::equalsIgnoreCaseAscii(&am.encode_utf16().collect::<Vec<_>>(),b"AM")){
                    // java rs = getAMPMTime(s, beginIndex, endIndex, langRes.SEPERATORS.DECIMAL_SEPARATOR, langRes.AM, langRes.PM); if (rs != null) { return rs; }
                    rs=Self::getAMPMTime(s,beginIndex,endIndex,langRes.SEPERATORS.DECIMAL_SEPARATOR,langRes.AM.as_ref().unwrap(),langRes.PM.as_ref().unwrap());if rs.is_some(){return rs;}
                }
                // java return null;
                None
            }
        }
    }
}
