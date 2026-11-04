use crate::cn::wps::et::ss::{constant::{ConstRecognisor::ConstRecognisor,NumfmtCategory::NumfmtCategory,RecogniseResult::RecogniseResult},multlang::LangRes::LangRes};
use super::IntegerRecognisor::IntegerRecognisor;
// java public class FractionRecognisor {
pub struct FractionRecognisor;
impl FractionRecognisor {
    // java static RecogniseResult getPureFraction(String s, int beginIndex, int endIndex) {
    pub fn getPureFraction(s:&[u16],mut beginIndex:usize,mut endIndex:usize)->Option<RecogniseResult>{
        // java while(beginIndex < endIndex && s.charAt(beginIndex) == ' ') { ++beginIndex; }
        while beginIndex<endIndex&&s[beginIndex]==32{beginIndex+=1;}
        // java while(beginIndex < endIndex && s.charAt(endIndex - 1) == ' ') { --endIndex; }
        while beginIndex<endIndex&&s[endIndex-1]==32{endIndex-=1;}
        // java if (beginIndex >= endIndex) { return null; } else {
        if beginIndex>=endIndex{return None;}else{
            // java boolean negative = false;
            let mut negative=false;
            // java if (s.charAt(beginIndex) == '-' || s.charAt(beginIndex) == '+') {
            if s[beginIndex]==45||s[beginIndex]==43{
                // java if (s.charAt(beginIndex) == '-') { negative = true; }
                if s[beginIndex]==45{negative=true;}
                // java ++beginIndex;
                beginIndex+=1;
                // java while(beginIndex < endIndex && s.charAt(beginIndex) == ' ') { ++beginIndex; }
                while beginIndex<endIndex&&s[beginIndex]==32{beginIndex+=1;}
                // java if (beginIndex >= endIndex) { return null; }
                if beginIndex>=endIndex{return None;}
            }
            // java Pattern pure_fraction_ptrn = Pattern.compile((new StringBuilder(100)).append("(\\d{1,5})\\s*/\\s*(\\d{1,5})").toString());
            let pure_fraction_ptrn=crate::java_runtime::Pattern::compileMatches(r"(\d{1,5})\s*/\s*(\d{1,5})",false).unwrap();
            // java Matcher m = pure_fraction_ptrn.matcher(s.substring(beginIndex, endIndex));
            let text=String::from_utf16(&s[beginIndex..endIndex]).ok()?;
            let m=pure_fraction_ptrn.captures(&text);
            // java if (!m.matches()) { return null; } else {
            if m.is_none(){None}else{
                let m=m.unwrap();
                // java Long numerator = Long.valueOf(m.group(1));
                let numerator=crate::java_runtime::DecimalInteger::parseLong(&m[1].encode_utf16().collect::<Vec<_>>()).expect("NumberFormatException");
                // java Long denominator = Long.valueOf(m.group(2));
                let denominator=crate::java_runtime::DecimalInteger::parseLong(&m[2].encode_utf16().collect::<Vec<_>>()).expect("NumberFormatException");
                // java double fra = (double)numerator / (double)denominator;
                let fra=numerator as f64/denominator as f64;
                // java RecogniseResult ret = new RecogniseResult(negative ? -fra : fra);
                let mut ret=RecogniseResult::RecogniseResult__2(Some(if negative{-fra}else{fra}));
                // java ret.setSuggestedNumfmt(13);
                ret.setSuggestedNumfmt__2(13);
                // java ret.setFractionFlag();
                ret.setFractionFlag();
                // java return ret;
                Some(ret)
            }
        }
    }
    // java public static RecogniseResult getFractionWithInteger(String s, int beginIndex, int endIndex, char groupSeparator, char decimalSeparator) {
    pub fn getFractionWithInteger(s:&[u16],mut beginIndex:usize,mut endIndex:usize,groupSeparator:u16,decimalSeparator:u16)->Option<RecogniseResult>{
        // java while(beginIndex < endIndex && s.charAt(beginIndex) == ' ') { ++beginIndex; }
        while beginIndex<endIndex&&s[beginIndex]==32{beginIndex+=1;}
        // java while(beginIndex < endIndex && s.charAt(endIndex - 1) == ' ') { --endIndex; }
        while beginIndex<endIndex&&s[endIndex-1]==32{endIndex-=1;}
        // java if (beginIndex >= endIndex) { return null; } else {
        if beginIndex>=endIndex{return None;}else{
            // java boolean negative = false;
            let mut negative=false;
            // java if (s.charAt(beginIndex) == '(' && s.charAt(endIndex - 1) == ')') {
            if s[beginIndex]==40&&s[endIndex-1]==41{
                // java negative = true; ++beginIndex; --endIndex;
                negative=true;beginIndex+=1;endIndex-=1;
                // java while(beginIndex < endIndex && s.charAt(beginIndex) == ' ') { ++beginIndex; }
                while beginIndex<endIndex&&s[beginIndex]==32{beginIndex+=1;}
                // java while(beginIndex < endIndex && s.charAt(endIndex - 1) == ' ') { --endIndex; }
                while beginIndex<endIndex&&s[endIndex-1]==32{endIndex-=1;}
                // java if (beginIndex >= endIndex) { return null; }
                if beginIndex>=endIndex{return None;}
            // java } else if (s.charAt(beginIndex) == '-' || s.charAt(beginIndex) == '+') {
            }else if s[beginIndex]==45||s[beginIndex]==43{
                // java if (s.charAt(beginIndex) == '-') { negative = true; }
                if s[beginIndex]==45{negative=true;}
                // java ++beginIndex;
                beginIndex+=1;
                // java while(beginIndex < endIndex && s.charAt(beginIndex) == ' ') { ++beginIndex; }
                while beginIndex<endIndex&&s[beginIndex]==32{beginIndex+=1;}
                // java if (beginIndex >= endIndex) { return null; }
                if beginIndex>=endIndex{return None;}
            }
            // java StringBuilder int_fraction_ptrn = (new StringBuilder(100)).append("(\\d+(").append(ConstRecognisor._wrapRegChar(groupSeparator)).append("\\d{3,})*)\\s+(\\d{1,5}\\s*/\\s*\\d{1,5})");
            let int_fraction_ptrn=format!(r"(\d+({}\d{{3,}})*)\s+(\d{{1,5}}\s*/\s*\d{{1,5}})",String::from_utf16(&ConstRecognisor::_wrapRegChar(groupSeparator)).unwrap());
            // java Matcher m = Pattern.compile(int_fraction_ptrn.toString()).matcher(s.subSequence(beginIndex, endIndex));
            let pattern=crate::java_runtime::Pattern::compileMatches(&int_fraction_ptrn,false).unwrap();
            let text=String::from_utf16(&s[beginIndex..endIndex]).ok()?;
            let m=pattern.captures(&text);
            // java if (!m.matches()) { return null; } else {
            if m.is_none(){None}else{
                let m=m.unwrap();
                // java String int_str = m.group(1);
                let int_str:Vec<_>=m[1].encode_utf16().collect();
                // java RecogniseResult rs = IntegerRecognisor.getLocalInt(int_str, 0, int_str.length(), groupSeparator, decimalSeparator, true);
                let rs=IntegerRecognisor::getLocalInt(&int_str,0,int_str.len(),groupSeparator,decimalSeparator,true);
                // java if (rs == null) { return null; } else {
                if rs.is_none(){None}else{
                    // java Double v = rs.getDblValue();
                    let mut v=rs.unwrap().getDblValue().unwrap();
                    // java String fra_str = m.group(3);
                    let fra_str:Vec<_>=m[3].encode_utf16().collect();
                    // java rs = getPureFraction(fra_str, 0, fra_str.length());
                    let rs=Self::getPureFraction(&fra_str,0,fra_str.len());
                    // java if (rs == null) { return null; } else {
                    if rs.is_none(){None}else{
                        // java v = v + rs.getDblValue();
                        v=v+rs.unwrap().getDblValue().unwrap();
                        // java RecogniseResult ret = new RecogniseResult(negative ? -v : v);
                        let mut ret=RecogniseResult::RecogniseResult__2(Some(if negative{-v}else{v}));
                        // java ret.setSuggestedNumfmt(13); ret.setFractionFlag(); return ret;
                        ret.setSuggestedNumfmt__2(13);ret.setFractionFlag();Some(ret)
                    }
                }
            }
        }
    }
    // java public static RecogniseResult getFraction(String s, int beginIndex, int endIndex, LangRes langRes, NumfmtCategory numfmtCat) {
    pub fn getFraction(s:&[u16],beginIndex:usize,endIndex:usize,langRes:&LangRes,numfmtCat:NumfmtCategory)->Option<RecogniseResult>{
        // java RecogniseResult rs = getFractionWithInteger(s, beginIndex, endIndex, langRes.SEPERATORS.GROUP_SEPARATOR, langRes.SEPERATORS.DECIMAL_SEPARATOR);
        let rs=Self::getFractionWithInteger(s,beginIndex,endIndex,langRes.SEPERATORS.GROUP_SEPARATOR,langRes.SEPERATORS.DECIMAL_SEPARATOR);
        // java if (rs != null) { return rs; } else {
        if rs.is_some(){rs}else{
            // java if (numfmtCat == NumfmtCategory.NUMBER || numfmtCat == NumfmtCategory.ACCOUNTING || numfmtCat == NumfmtCategory.CURRENCY || numfmtCat == NumfmtCategory.PERCENTAGE || numfmtCat == NumfmtCategory.FRACTION || numfmtCat == NumfmtCategory.SCIENTIC) {
            if matches!(numfmtCat,NumfmtCategory::NUMBER|NumfmtCategory::ACCOUNTING|NumfmtCategory::CURRENCY|NumfmtCategory::PERCENTAGE|NumfmtCategory::FRACTION|NumfmtCategory::SCIENTIC){
                // java rs = getPureFraction(s, beginIndex, endIndex);
                let rs=Self::getPureFraction(s,beginIndex,endIndex);
                // java if (rs != null) { return rs; }
                if rs.is_some(){return rs;}
            }
            // java return null;
            None
        }
    }
}
