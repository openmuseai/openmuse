use super::IntegerRecognisor::IntegerRecognisor;
use crate::{java_runtime::BigDecimal,cn::wps::et::ss::{constant::RecogniseResult::RecogniseResult,statemachine::RecogniseResultRl::Value}};
// java public class FloatRecognisor {
pub struct FloatRecognisor;
impl FloatRecognisor {
    // java public static RecogniseResult getFloat(String s, int beginIndex, int endIndex, char groupSeparator, char decimalSeparator) {
    pub fn getFloat(s:&[u16],mut beginIndex:usize,mut endIndex:usize,groupSeparator:u16,decimalSeparator:u16)->Option<RecogniseResult>{
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
        // java String subs = s.substring(beginIndex, endIndex);
        let subs=String::from_utf16(&s[beginIndex..endIndex]).ok()?;
        // java Pattern float_ptrn1 = Pattern.compile((new StringBuilder(100)).append("(\\d+(").append(ConstRecognisor._wrapRegChar(groupSeparator)).append("\\d{3,})*)").append(ConstRecognisor._wrapRegChar(decimalSeparator)).append("(\\d+)").toString());
        let group=regex::escape(&String::from_utf16(&[groupSeparator]).ok()?);
        let decimal=regex::escape(&String::from_utf16(&[decimalSeparator]).ok()?);
        let float_ptrn1=crate::java_runtime::Pattern::compile(&format!(r"\A(\d+({group}\d{{3,}})*){decimal}(\d+)\z")).unwrap();
        // java Matcher m = float_ptrn1.matcher(subs); try { if (m.matches()) {
        if let Some(m)=float_ptrn1.captures(&subs){
            // java String istr = m.group(1);
            let istr:Vec<u16>=m[1].encode_utf16().collect();
            // java RecogniseResult rs = IntegerRecognisor.getLocalInt(istr, 0, istr.length(), groupSeparator, decimalSeparator, true);
            let rs=IntegerRecognisor::getLocalInt(&istr,0,istr.len(),groupSeparator,decimalSeparator,true).expect("integer group");
            // java Double v = rs.getDblValue(); Double flo = Double.valueOf("0." + m.group(3));
            let mut v=rs.getDblValue().expect("Double");let flo=crate::java_runtime::parseDouble(&format!("0.{}",&m[3]))?;
            // java v = BigDecimal.valueOf(v).add(BigDecimal.valueOf(flo)).doubleValue();
            if !v.is_finite(){return None;}
            v=BigDecimal::valueOf(v).add(&BigDecimal::valueOf(flo)).doubleValue();
            // java if (negative) { v = -v; }
            if negative{v=-v;}
            // java RecogniseResult ret = new RecogniseResult(v);
            let mut ret=RecogniseResult::fromValue(Value::Number(v));
            // java if (rs.hasGroupedFlag()) { ret.setSuggestedNumfmt(4); ret.setGroupedFlag(); } else { ret.setSuggestedNumfmt(0); }
            if rs.hasGroupedFlag(){ret.setSuggestedNumfmtI(4);ret.setGroupedFlag();}else{ret.setSuggestedNumfmtI(0);}
            // java return ret;
            Some(ret)
        }else{
            // java Pattern float_ptrn2 = Pattern.compile((new StringBuilder(100)).append(ConstRecognisor._wrapRegChar(decimalSeparator)).append("(\\d+)").toString()); m = float_ptrn2.matcher(subs);
            let float_ptrn2=crate::java_runtime::Pattern::compile(&format!(r"\A{decimal}(\d+)\z")).unwrap();
            // java if (m.matches()) {
            if let Some(m)=float_ptrn2.captures(&subs){
                // java Double flo = Double.valueOf("0." + m.group(1));
                let flo=crate::java_runtime::parseDouble(&format!("0.{}",&m[1]))?;
                // java RecogniseResult ret = new RecogniseResult(negative ? -flo : flo); ret.setSuggestedNumfmt(0); return ret;
                let mut ret=RecogniseResult::fromValue(Value::Number(if negative{-flo}else{flo}));ret.setSuggestedNumfmtI(0);Some(ret)
            // java } else { return null; }
            }else{None}
        }
        // java } catch (NumberFormatException var14) { return null; }
        // rust: parseDouble's Option and the finite check above transport this catch.
    }
}
