use super::{FloatRecognisor::FloatRecognisor,IntegerRecognisor::IntegerRecognisor};
use crate::{java_runtime::BigDecimal,cn::wps::et::ss::{constant::RecogniseResult::RecogniseResult,statemachine::RecogniseResultRl::Value}};
// java public class SenticRecognisor {
pub struct SenticRecognisor;
impl SenticRecognisor {
    // java public static RecogniseResult getSentic(String s, int beginIndex, int endIndex, char groupSeparator, char decimalSeparator) {
    pub fn getSentic(s:&[u16],mut beginIndex:usize,mut endIndex:usize,groupSeparator:u16,decimalSeparator:u16)->Option<RecogniseResult>{
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
        // java int eb = -1; int ee = -1; int i = beginIndex;
        let mut eb=None;let mut ee=0;let mut i=beginIndex;
        // java while(i < endIndex) {
        while i<endIndex{
            // java char c = s.charAt(i);
            let c=s[i];
            // java if (c != 'e' && c != 'E') {
            if c!=101&&c!=69{
                // java if (c != '*') {
                if c!=42{
                    // java if (c == groupSeparator || c == decimalSeparator || c == '+' || c == '-' || c >= '0' && c <= '9') { ++i; continue; }
                    if c==groupSeparator||c==decimalSeparator||c==43||c==45||(48..=57).contains(&c){i+=1;continue;}
                    // java return null;
                    return None;
                }
                // java if (i <= beginIndex || i + 1 >= endIndex - 1 || s.charAt(i + 1) != '*') { return null; }
                if i<=beginIndex||i+1>=endIndex-1||s[i+1]!=42{return None;}
                // java eb = i; ee = i + 2; break;
                eb=Some(i);ee=i+2;break;
            }
            // java if (i <= beginIndex || i >= endIndex - 1) { return null; }
            if i<=beginIndex||i>=endIndex-1{return None;}
            // java eb = i; ee = i + 1; break;
            eb=Some(i);ee=i+1;break;
        }
        // java if (eb == -1) { return null; }
        let eb=eb?;
        // java i = 1;
        let mut i=1;
        // java if (s.charAt(ee) == '-') { i = 0; ++ee; } else if (s.charAt(ee) == '+') { ++ee; }
        if s[ee]==45{i=0;ee+=1;}else if s[ee]==43{ee+=1;}
        // java RecogniseResult rs = FloatRecognisor.getFloat(s, beginIndex, eb, groupSeparator, decimalSeparator);
        let mut rs=FloatRecognisor::getFloat(s,beginIndex,eb,groupSeparator,decimalSeparator);
        // java if (rs == null) { rs = IntegerRecognisor.getLocalInt(s, beginIndex, eb, groupSeparator, decimalSeparator, true); }
        if rs.is_none(){rs=IntegerRecognisor::getLocalInt(s,beginIndex,eb,groupSeparator,decimalSeparator,true);}
        // java if (rs == null) { return null; } else if (rs.getDblValue() < (double)0.0F) { return null; }
        let rs=rs?;if rs.getDblValue().expect("Double")<0.0{return None;}
        // java BigDecimal base = BigDecimal.valueOf(rs.getDblValue());
        let base=BigDecimal::valueOf(rs.getDblValue().expect("Double"));
        // java if (ee >= endIndex) { return null; }
        if ee>=endIndex{return None;}
        // java StringBuilder sb = new StringBuilder(endIndex - ee);
        let mut sb=String::new();
        // java for (int pos = ee; pos < endIndex; ++pos) {
        for pos in ee..endIndex{
            // java char c = s.charAt(pos); if (c < '0' || c > '9') { return null; } sb.append(c);
            let c=s[pos];if !(48..=57).contains(&c){return None;}sb.push(char::from_u32(c as u32).unwrap());
        }
        // java String subs = sb.toString(); int e = 0; try { e = Integer.parseInt(subs); } catch (RuntimeException var17) { return null; }
        let subs=sb;let e=subs.parse::<i32>().ok()?;
        // java if (e >= 308) { return null; }
        if e>=308{return None;}
        // java double v; if (i != 0) { v = base.multiply(BigDecimal.TEN.pow(e)).doubleValue(); } else { v = base.divide(BigDecimal.TEN.pow(e)).doubleValue(); }
        let ten=BigDecimal::valueOfLong(10).pow(e);
        let mut v=if i!=0{base.multiply(&ten).doubleValue()}else{base.divide(&ten).doubleValue()};
        // java if (negative) { v = -v; }
        if negative{v=-v;}
        // java if (!(v >= 1.0E308) && !(v <= -1.0E308) && (base.doubleValue() == (double)0.0F || !(Math.abs(v) <= Double.MIN_NORMAL))) {
        if !(v>=1.0e308)&&!(v<=-1.0e308)&&(base.doubleValue()==0.0||!(v.abs()<=f64::MIN_POSITIVE)){
            // java RecogniseResult ret = new RecogniseResult(v); ret.setSuggestedNumfmt(11); ret.setSenticFlag(); return ret;
            let mut ret=RecogniseResult::fromValue(Value::Number(v));ret.setSuggestedNumfmtI(11);ret.setSenticFlag();Some(ret)
        // java } else { return null; }
        }else{None}
    }
}
