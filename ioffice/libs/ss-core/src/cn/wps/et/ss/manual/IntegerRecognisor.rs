use crate::cn::wps::et::ss::{constant::RecogniseResult::RecogniseResult,statemachine::RecogniseResultRl::Value};
// java public class IntegerRecognisor {
pub struct IntegerRecognisor;
impl IntegerRecognisor {
    // java public static RecogniseResult getLocalInt(String s, int beginIndex, int endIndex, char groupSeparator, char decimalSeparator, boolean signAhead) {
    pub fn getLocalInt(s:&[u16],mut beginIndex:usize,mut endIndex:usize,groupSeparator:u16,decimalSeparator:u16,signAhead:bool)->Option<RecogniseResult>{
        // java while(beginIndex < endIndex && s.charAt(beginIndex) == ' ') { ++beginIndex; }
        while beginIndex<endIndex&&s[beginIndex]==32{beginIndex+=1;}
        // java while(beginIndex < endIndex && s.charAt(endIndex - 1) == ' ') { --endIndex; }
        while beginIndex<endIndex&&s[endIndex-1]==32{endIndex-=1;}
        // java if (beginIndex >= endIndex) { return null; }
        if beginIndex>=endIndex{return None;}
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
        // java } else if (!signAhead || s.charAt(beginIndex) != '-' && s.charAt(beginIndex) != '+') {
        }else if !signAhead||s[beginIndex]!=45&&s[beginIndex]!=43{
            // java if (!signAhead && (s.charAt(endIndex - 1) == '-' || s.charAt(endIndex - 1) == '+')) {
            if !signAhead&&(s[endIndex-1]==45||s[endIndex-1]==43){
                // java if (s.charAt(endIndex - 1) == '-') { negative = true; }
                if s[endIndex-1]==45{negative=true;}
                // java --endIndex;
                endIndex-=1;
                // java while(beginIndex < endIndex && s.charAt(endIndex - 1) == ' ') { --endIndex; }
                while beginIndex<endIndex&&s[endIndex-1]==32{endIndex-=1;}
                // java if (beginIndex >= endIndex) { return null; }
                if beginIndex>=endIndex{return None;}
            }
        }else{
            // java if (s.charAt(beginIndex) == '-') { negative = true; }
            if s[beginIndex]==45{negative=true;}
            // java ++beginIndex;
            beginIndex+=1;
            // java while(beginIndex < endIndex && s.charAt(beginIndex) == ' ') { ++beginIndex; }
            while beginIndex<endIndex&&s[beginIndex]==32{beginIndex+=1;}
            // java if (beginIndex >= endIndex) { return null; }
            if beginIndex>=endIndex{return None;}
        }
        // java StringBuilder int_ptrn = (new StringBuilder(100)).append("\\d+(").append(ConstRecognisor._wrapRegChar(groupSeparator)).append("\\d{3,})*").append(ConstRecognisor._wrapRegChar(decimalSeparator)).append('?');
        // rust: Android java.util.regex uses Unicode decimal digits for \d.
        let group=regex::escape(&String::from_utf16(&[groupSeparator]).ok()?);
        let decimal=regex::escape(&String::from_utf16(&[decimalSeparator]).ok()?);
        let int_ptrn=crate::java_runtime::Pattern::compile(&format!(r"\A\d+({group}\d{{3,}})*{decimal}?\z")).unwrap();
        // java if (!Pattern.matches(int_ptrn.toString(), s.subSequence(beginIndex, endIndex))) { return null; }
        if !int_ptrn.is_match(&String::from_utf16(&s[beginIndex..endIndex]).ok()?){return None;}
        // java StringBuilder num = new StringBuilder(endIndex - beginIndex); boolean grouped = false;
        let mut num=vec![];let mut grouped=false;
        // java for(int i = beginIndex; i < endIndex; ++i) {
        for i in beginIndex..endIndex{
            // java char c = s.charAt(i);
            let c=s[i];
            // java if (c == groupSeparator) { grouped = true; }
            if c==groupSeparator{grouped=true;}
            // java else if (c != ' ' && c != '\t' && c != decimalSeparator) { num.append(c); }
            else if c!=32&&c!=9&&c!=decimalSeparator{num.push(c);}
        }
        // java Double v = TypeConversion.string2Double(num.toString()); v = null == v ? (double)0.0F : v;
        let mut v=crate::java_runtime::parseDouble(&String::from_utf16(&num).ok()?).unwrap_or(0.0);
        // java if (negative) { v = -v; }
        if negative{v=-v;}
        // java RecogniseResult ret = new RecogniseResult(v);
        let mut ret=RecogniseResult::fromValue(Value::Number(v));
        // java if (grouped) { ret.setSuggestedNumfmt(3); ret.setGroupedFlag(); } else { ret.setSuggestedNumfmt(0); }
        if grouped{ret.setSuggestedNumfmtI(3);ret.setGroupedFlag();}else{ret.setSuggestedNumfmtI(0);}
        // java return ret;
        Some(ret)
    }
    // java public static RecogniseResult getInt(String s, int beginIndex, int endIndex, char groupSeparator, char decimalSeparator) {
    pub fn getInt(s:&[u16],beginIndex:usize,endIndex:usize,groupSeparator:u16,decimalSeparator:u16)->Option<RecogniseResult>{
        // java RecogniseResult rs = getLocalInt(s, beginIndex, endIndex, groupSeparator, decimalSeparator, true);
        let rs=Self::getLocalInt(s,beginIndex,endIndex,groupSeparator,decimalSeparator,true);
        // java return rs;
        rs
    }
}
