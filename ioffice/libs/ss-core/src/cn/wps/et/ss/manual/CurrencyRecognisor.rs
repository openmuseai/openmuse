use crate::cn::wps::et::ss::{constant::RecogniseResult::RecogniseResult,multlang::{LangRes::LangRes,res::CurrencyRes::{CurrencyRes,PositiveCurrencyFormat}}};
use super::{IntegerRecognisor::IntegerRecognisor,FloatRecognisor::FloatRecognisor,SenticRecognisor::SenticRecognisor};
// java public class CurrencyRecognisor {
pub struct CurrencyRecognisor;
impl CurrencyRecognisor {
    // java static RecogniseResult getHead(String s, int beginIndex, int endIndex, String currencyChar, char groupSeparator, char decimalSeparator, CurrencyRes c) {
    pub fn getHead(s:&[u16],mut beginIndex:usize,mut endIndex:usize,currencyChar:&[u16],groupSeparator:u16,decimalSeparator:u16,c:&CurrencyRes)->Option<RecogniseResult>{
        // java while(beginIndex < endIndex && s.charAt(beginIndex) == ' ') { ++beginIndex; }
        while beginIndex<endIndex&&s[beginIndex]==32{beginIndex+=1;}
        // java while(beginIndex < endIndex && s.charAt(endIndex - 1) == ' ') { --endIndex; }
        while beginIndex<endIndex&&s[endIndex-1]==32{endIndex-=1;}
        // java if (beginIndex >= endIndex) { return null; } else {
        if beginIndex>=endIndex{return None;}else{
            // java boolean negative = false;
            let mut negative=false;
            // java if (s.charAt(beginIndex) == '-') {
            if s[beginIndex]==45{
                // java negative = true; ++beginIndex;
                negative=true;beginIndex+=1;
                // java while(beginIndex < endIndex && s.charAt(beginIndex) == ' ') { ++beginIndex; }
                while beginIndex<endIndex&&s[beginIndex]==32{beginIndex+=1;}
                // java if (beginIndex >= endIndex) { return null; }
                if beginIndex>=endIndex{return None;}
            // java } else if (s.charAt(beginIndex) == '(' && s.charAt(endIndex - 1) == ')') {
            }else if s[beginIndex]==40&&s[endIndex-1]==41{
                // java negative = true; ++beginIndex; --endIndex;
                negative=true;beginIndex+=1;endIndex-=1;
                // java while(beginIndex < endIndex && s.charAt(beginIndex) == ' ') { ++beginIndex; }
                while beginIndex<endIndex&&s[beginIndex]==32{beginIndex+=1;}
                // java while(beginIndex < endIndex && s.charAt(endIndex - 1) == ' ') { --endIndex; }
                while beginIndex<endIndex&&s[endIndex-1]==32{endIndex-=1;}
                // java if (beginIndex >= endIndex) { return null; }
                if beginIndex>=endIndex{return None;}
            }
            // java int i = 0;
            let mut i=0;
            // java for(int len = currencyChar.length(); i < len; ++i) {
            let len=currencyChar.len();while i<len{
                // java int index = beginIndex + i;
                let index=beginIndex+i;
                // java if (index >= s.length()) { return null; }
                if index>=s.len(){return None;}
                // java if (s.charAt(index) != currencyChar.charAt(i)) { return null; }
                if s[index]!=currencyChar[i]{return None;}
                i+=1;
            }
            // java for(beginIndex += currencyChar.length(); beginIndex < endIndex && s.charAt(beginIndex) == ' '; ++beginIndex) { }
            beginIndex+=currencyChar.len();while beginIndex<endIndex&&s[beginIndex]==32{beginIndex+=1;}
            // java if (beginIndex >= endIndex) { return null; } else {
            if beginIndex>=endIndex{return None;}else{
                // java if (!negative && s.charAt(beginIndex) == '-') {
                if !negative&&s[beginIndex]==45{
                    // java negative = true; ++beginIndex;
                    negative=true;beginIndex+=1;
                    // java while(beginIndex < endIndex && s.charAt(beginIndex) == ' ') { ++beginIndex; }
                    while beginIndex<endIndex&&s[beginIndex]==32{beginIndex+=1;}
                    // java if (beginIndex >= endIndex) { return null; }
                    if beginIndex>=endIndex{return None;}
                }
                // java i = 0;
                i=0;
                // java RecogniseResult rs = IntegerRecognisor.getInt(s, beginIndex, endIndex, groupSeparator, decimalSeparator);
                let mut rs=IntegerRecognisor::getInt(s,beginIndex,endIndex,groupSeparator,decimalSeparator);
                // java if (rs == null) {
                if rs.is_none(){
                    // java rs = FloatRecognisor.getFloat(s, beginIndex, endIndex, groupSeparator, decimalSeparator); i = 1;
                    rs=FloatRecognisor::getFloat(s,beginIndex,endIndex,groupSeparator,decimalSeparator);i=1;
                }
                // java if (rs != null) {
                if let Some(rs)=rs{
                    // java double dblValue = rs.getDblValue();
                    let mut dblValue=rs.getDblValue().unwrap();
                    // java if (dblValue < (double)0.0F) { return null; } else {
                    if dblValue<0.0{None}else{
                        // java if (negative) { dblValue = -dblValue; }
                        if negative{dblValue=-dblValue;}
                        // java RecogniseResult ret = new RecogniseResult(dblValue);
                        let mut ret=RecogniseResult::RecogniseResult__2(Some(dblValue));
                        // java if (i != 0) { ret.setSuggestedNumfmt(c.CURRENCY_NUMFMT_DECIMAL); } else { ret.setSuggestedNumfmt(c.CURRENCY_NUMFMT); }
                        if i!=0{ret.setSuggestedNumfmt(Some(c.CURRENCY_NUMFMT_DECIMAL.clone()));}else{ret.setSuggestedNumfmt(Some(c.CURRENCY_NUMFMT.clone()));}
                        // java ret.setCurrencyFlag(); return ret;
                        ret.setCurrencyFlag();Some(ret)
                    }
                // java } else {
                }else{
                    // java rs = SenticRecognisor.getSentic(s, beginIndex, endIndex, groupSeparator, decimalSeparator);
                    let rs=SenticRecognisor::getSentic(s,beginIndex,endIndex,groupSeparator,decimalSeparator);
                    // java if (rs != null) {
                    if let Some(rs)=rs{
                        // java double dblValue = rs.getDblValue();
                        let mut dblValue=rs.getDblValue().unwrap();
                        // java if (dblValue < (double)0.0F) { return null; } else {
                        if dblValue<0.0{None}else{
                            // java if (negative) { dblValue = -dblValue; }
                            if negative{dblValue=-dblValue;}
                            // java RecogniseResult ret = new RecogniseResult(dblValue);
                            let mut ret=RecogniseResult::RecogniseResult__2(Some(dblValue));
                            // java ret.setSuggestedNumfmt(11); return ret;
                            ret.setSuggestedNumfmt__2(11);Some(ret)
                        }
                    // java } else { return null; }
                    }else{None}
                }
            }
        }
    }
    // java static RecogniseResult getEnd(String s, int beginIndex, int endIndex, String currencyChar, char groupSeparator, char decimalSeparator, CurrencyRes c) {
    pub fn getEnd(s:&[u16],mut beginIndex:usize,mut endIndex:usize,currencyChar:&[u16],groupSeparator:u16,decimalSeparator:u16,c:&CurrencyRes)->Option<RecogniseResult>{
        // java while(beginIndex < endIndex && s.charAt(beginIndex) == ' ') { ++beginIndex; }
        while beginIndex<endIndex&&s[beginIndex]==32{beginIndex+=1;}
        // java while(beginIndex < endIndex && s.charAt(endIndex - 1) == ' ') { --endIndex; }
        while beginIndex<endIndex&&s[endIndex-1]==32{endIndex-=1;}
        // java if (beginIndex >= endIndex) { return null; } else {
        if beginIndex>=endIndex{return None;}else{
            // java boolean negative = false;
            let mut negative=false;
            // java if (s.charAt(beginIndex) == '-') {
            if s[beginIndex]==45{
                // java negative = true; ++beginIndex;
                negative=true;beginIndex+=1;
                // java while(beginIndex < endIndex && s.charAt(beginIndex) == ' ') { ++beginIndex; }
                while beginIndex<endIndex&&s[beginIndex]==32{beginIndex+=1;}
                // java if (beginIndex >= endIndex) { return null; }
                if beginIndex>=endIndex{return None;}
            // java } else if (s.charAt(beginIndex) == '(' && s.charAt(endIndex - 1) == ')') {
            }else if s[beginIndex]==40&&s[endIndex-1]==41{
                // java negative = true; ++beginIndex; --endIndex;
                negative=true;beginIndex+=1;endIndex-=1;
                // java while(beginIndex < endIndex && s.charAt(beginIndex) == ' ') { ++beginIndex; }
                while beginIndex<endIndex&&s[beginIndex]==32{beginIndex+=1;}
                // java while(beginIndex < endIndex && s.charAt(endIndex - 1) == ' ') { --endIndex; }
                while beginIndex<endIndex&&s[endIndex-1]==32{endIndex-=1;}
                // java if (beginIndex >= endIndex) { return null; }
                if beginIndex>=endIndex{return None;}
            }
            // java int i = 0;
            let mut i=0;
            // java for(int len = currencyChar.length(); i < len; ++i) {
            let len=currencyChar.len();while i<len{
                // java int sourceIndex = endIndex - 1 - i;
                let sourceIndex=endIndex as isize-1-i as isize;
                // java if (sourceIndex < 0) { return null; }
                if sourceIndex<0{return None;}
                // java if (s.charAt(sourceIndex) != currencyChar.charAt(len - 1 - i)) { return null; }
                if s[sourceIndex as usize]!=currencyChar[len-1-i]{return None;}
                i+=1;
            }
            // java for(endIndex -= currencyChar.length(); beginIndex < endIndex && s.charAt(endIndex - 1) == ' '; --endIndex) { }
            endIndex-=currencyChar.len();while beginIndex<endIndex&&s[endIndex-1]==32{endIndex-=1;}
            // java if (beginIndex >= endIndex) { return null; } else {
            if beginIndex>=endIndex{return None;}else{
                // java i = 0;
                i=0;
                // java RecogniseResult rs = IntegerRecognisor.getInt(s, beginIndex, endIndex, groupSeparator, decimalSeparator);
                let mut rs=IntegerRecognisor::getInt(s,beginIndex,endIndex,groupSeparator,decimalSeparator);
                // java if (rs == null) {
                if rs.is_none(){
                    // java rs = FloatRecognisor.getFloat(s, beginIndex, endIndex, groupSeparator, decimalSeparator); i = 1;
                    rs=FloatRecognisor::getFloat(s,beginIndex,endIndex,groupSeparator,decimalSeparator);i=1;
                }
                // java if (rs != null) {
                if let Some(rs)=rs{
                    // java double dblValue = rs.getDblValue();
                    let mut dblValue=rs.getDblValue().unwrap();
                    // java if (dblValue < (double)0.0F) { return null; } else {
                    if dblValue<0.0{None}else{
                        // java if (negative) { dblValue = -dblValue; }
                        if negative{dblValue=-dblValue;}
                        // java RecogniseResult ret = new RecogniseResult(dblValue);
                        let mut ret=RecogniseResult::RecogniseResult__2(Some(dblValue));
                        // java if (i != 0) { ret.setSuggestedNumfmt(c.CURRENCY_NUMFMT_DECIMAL); } else { ret.setSuggestedNumfmt(c.CURRENCY_NUMFMT); }
                        if i!=0{ret.setSuggestedNumfmt(Some(c.CURRENCY_NUMFMT_DECIMAL.clone()));}else{ret.setSuggestedNumfmt(Some(c.CURRENCY_NUMFMT.clone()));}
                        // java ret.setCurrencyFlag(); return ret;
                        ret.setCurrencyFlag();Some(ret)
                    }
                // java } else {
                }else{
                    // java rs = SenticRecognisor.getSentic(s, beginIndex, endIndex, groupSeparator, decimalSeparator);
                    let rs=SenticRecognisor::getSentic(s,beginIndex,endIndex,groupSeparator,decimalSeparator);
                    // java if (rs != null) {
                    if let Some(rs)=rs{
                        // java double dblValue = rs.getDblValue();
                        let mut dblValue=rs.getDblValue().unwrap();
                        // java if (dblValue < (double)0.0F) { return null; } else {
                        if dblValue<0.0{None}else{
                            // java if (negative) { dblValue = -dblValue; }
                            if negative{dblValue=-dblValue;}
                            // java RecogniseResult ret = new RecogniseResult(dblValue);
                            let mut ret=RecogniseResult::RecogniseResult__2(Some(dblValue));
                            // java ret.setSuggestedNumfmt(11); return ret;
                            ret.setSuggestedNumfmt__2(11);Some(ret)
                        }
                    // java } else { return null; }
                    }else{None}
                }
            }
        }
    }
    // java public static RecogniseResult getCurrency(String s, int beginIndex, int endIndex, LangRes langRes) {
    pub fn getCurrency(s:&[u16],beginIndex:usize,endIndex:usize,langRes:&LangRes)->Option<RecogniseResult>{
        // java RecogniseResult rs = null;
        let mut rs=None;
        // java CurrencyRes[] res = langRes.CURRENCIES;
        let res=&langRes.CURRENCIES;
        // java for(int i = 0; i < res.length; ++i) {
        for i in 0..res.len(){
            // java CurrencyRes c = res[i];
            let c=&res[i];
            // java switch (c.POSITIVE_CURRENCY_FORMAT) {
            match c.POSITIVE_CURRENCY_FORMAT{
                // java case HEAD: case HEAD_SPACE:
                PositiveCurrencyFormat::HEAD|PositiveCurrencyFormat::HEAD_SPACE=>{
                    // java rs = getHead(s, beginIndex, endIndex, c.CURRENCY_CHAR.toLowerCase(), langRes.SEPERATORS.GROUP_SEPARATOR, langRes.SEPERATORS.DECIMAL_SEPARATOR, c); break;
                    let currency:Vec<u16>=c.CURRENCY_CHAR.encode_utf16().map(crate::java_runtime::Character::toLowerCase).collect();
                    rs=Self::getHead(s,beginIndex,endIndex,&currency,langRes.SEPERATORS.GROUP_SEPARATOR,langRes.SEPERATORS.DECIMAL_SEPARATOR,c);
                }
                // java case END: case SPACE_END:
                PositiveCurrencyFormat::END|PositiveCurrencyFormat::SPACE_END=>{
                    // java rs = getEnd(s, beginIndex, endIndex, c.CURRENCY_CHAR.toLowerCase(), langRes.SEPERATORS.GROUP_SEPARATOR, langRes.SEPERATORS.DECIMAL_SEPARATOR, c);
                    let currency:Vec<u16>=c.CURRENCY_CHAR.encode_utf16().map(crate::java_runtime::Character::toLowerCase).collect();
                    rs=Self::getEnd(s,beginIndex,endIndex,&currency,langRes.SEPERATORS.GROUP_SEPARATOR,langRes.SEPERATORS.DECIMAL_SEPARATOR,c);
                }
            }
            // java if (rs != null) { return rs; }
            if rs.is_some(){return rs;}
        }
        // java return null;
        None
    }
}
