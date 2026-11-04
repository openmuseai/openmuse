use super::RecogniseResult::RecogniseResult;
use crate::{java_runtime::BigDecimal,cn::wps::et::ss::{manual::{FloatRecognisor::FloatRecognisor,IntegerRecognisor::IntegerRecognisor,SenticRecognisor::SenticRecognisor},statemachine::RecogniseResultRl::Value}};
// java public class PercentRecognisor {
pub struct PercentRecognisor;
impl PercentRecognisor {
    // java public static RecogniseResult getPercent(String s, int beginIndex, int endIndex, char groupSeparator, char decimalSeparator) {
    pub fn getPercent(s:&[u16],mut beginIndex:usize,mut endIndex:usize,groupSeparator:u16,decimalSeparator:u16)->Option<RecogniseResult>{
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
        // java } else if (s.charAt(beginIndex) == '+' || s.charAt(beginIndex) == '-') {
        }else if s[beginIndex]==43||s[beginIndex]==45{
            // java if (s.charAt(beginIndex) == '-') { negative = true; }
            if s[beginIndex]==45{negative=true;}
            // java ++beginIndex;
            beginIndex+=1;
            // java while(beginIndex < endIndex && s.charAt(beginIndex) == ' ') { ++beginIndex; }
            while beginIndex<endIndex&&s[beginIndex]==32{beginIndex+=1;}
            // java if (beginIndex >= endIndex) { return null; }
            if beginIndex>=endIndex{return None;}
        }
        // java if (s.charAt(endIndex - 1) != '%') { return null; }
        if s[endIndex-1]!=37{return None;}
        // java --endIndex;
        endIndex-=1;
        // java while(beginIndex < endIndex && s.charAt(endIndex - 1) == ' ') { --endIndex; }
        while beginIndex<endIndex&&s[endIndex-1]==32{endIndex-=1;}
        // java if (beginIndex >= endIndex) { return null; }
        if beginIndex>=endIndex{return None;}
        // java RecogniseResult rs = IntegerRecognisor.getLocalInt(s, beginIndex, endIndex, groupSeparator, decimalSeparator, true);
        let rs=IntegerRecognisor::getLocalInt(s,beginIndex,endIndex,groupSeparator,decimalSeparator,true);
        // java if (rs != null) {
        if let Some(rs)=rs{
            // java double v = BigDecimal.valueOf(rs.getDblValue()).divide(BigDecimal.valueOf((double)100.0F)).doubleValue();
            let mut v=BigDecimal::valueOf(rs.getDblValue().expect("Double")).divide(&BigDecimal::valueOf(100.0)).doubleValue();
            // java if (v < (double)0.0F) { return null; }
            if v<0.0{return None;}
            // java if (negative) { v = -v; }
            if negative{v=-v;}
            // java RecogniseResult ret = new RecogniseResult(v); ret.setPercentageFlage(); ret.setSuggestedNumfmt(9); return ret;
            let mut ret=RecogniseResult::fromValue(Value::Number(v));ret.setPercentageFlage();ret.setSuggestedNumfmtI(9);Some(ret)
        }else{
            // java rs = FloatRecognisor.getFloat(s, beginIndex, endIndex, groupSeparator, decimalSeparator);
            let mut rs=FloatRecognisor::getFloat(s,beginIndex,endIndex,groupSeparator,decimalSeparator);
            // java if (rs == null) { rs = SenticRecognisor.getSentic(s, beginIndex, endIndex, groupSeparator, decimalSeparator); }
            if rs.is_none(){rs=SenticRecognisor::getSentic(s,beginIndex,endIndex,groupSeparator,decimalSeparator);}
            // java if (rs == null) { return null; }
            let rs=rs?;
            // java double v = BigDecimal.valueOf(rs.getDblValue()).divide(BigDecimal.valueOf((double)100.0F)).doubleValue();
            let mut v=BigDecimal::valueOf(rs.getDblValue().expect("Double")).divide(&BigDecimal::valueOf(100.0)).doubleValue();
            // java if (v < (double)0.0F) { return null; }
            if v<0.0{return None;}
            // java if (negative) { v = -v; }
            if negative{v=-v;}
            // java RecogniseResult ret = new RecogniseResult(v); ret.setPercentageFlage(); ret.setSuggestedNumfmt(10); return ret;
            let mut ret=RecogniseResult::fromValue(Value::Number(v));ret.setPercentageFlage();ret.setSuggestedNumfmtI(10);Some(ret)
        }
    }
}
