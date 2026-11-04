use crate::cn::wps::et::ss::formula::{eval::{ValueRef,NumberEval::NumberEval,StringEval::StringEval,ErrorEval::ErrorEval,MissingArgEval::MissingArgEval},evaluator::{EvalResult,EvaluationContext::EvaluationContext},functions::{base::{MixOpFunction::MixOpFunction,checked::{self,FunctionResult}},util::LookupUtils::LookupUtils}};use std::sync::Arc;
// java public abstract class TextFunction
pub struct TextFunction{kind:u8}
impl TextFunction{
// java public static final IFunction LEN= ...
pub const LEN:Self=Self{kind:0};
// java public static final IFunction TRIM= ...
pub const TRIM:Self=Self{kind:1};
// java public static final IFunction LEFT= ...
pub const LEFT:Self=Self{kind:2};
// java public static final IFunction RIGHT= ...
pub const RIGHT:Self=Self{kind:3};
// java public static final IFunction FIND= ...
pub const FIND:Self=Self{kind:4};
// java public static final IFunction SEARCH= ...
pub const SEARCH:Self=Self{kind:5};
// java public static final String evaluateStringArg(IValueEval eval, EvaluationContext ec) throws EvaluationException {
// java IValueEval ve = OperandResolver.getSingleValue(eval, ec);
// java return OperandResolver.coerceValueToString(ve);
// java }
pub fn evaluateStringArg(v:&ValueRef,ec:&EvaluationContext)->FunctionResult<Vec<u16>> {
checked::string(&checked::single(v,ec)?)
}
// java protected static final int evaluateIntArg(IValueEval arg, EvaluationContext ec) throws EvaluationException {
// java IValueEval ve = OperandResolver.getSingleValue(arg, ec);
// java return OperandResolver.coerceValueToInt(ve);
// java }
pub fn evaluateIntArg(v:&ValueRef,ec:&EvaluationContext)->FunctionResult<i32> {
checked::int(&checked::single(v,ec)?)
}
// java String excel_trim(String s) {
// java StringBuilder sb = new StringBuilder(s.length());
// java char first_blank = 0;
// java boolean has_letter = false;
// java
// java for(int i = 0; i < s.length(); ++i) {
// java char c = s.charAt(i);
// java if (this.isBlank(c)) {
// java if (first_blank == 0) {
// java first_blank = c;
// java }
// java } else {
// java if (first_blank != 0 && has_letter) {
// java sb.append(first_blank);
// java }
// java
// java sb.append(c);
// java first_blank = 0;
// java has_letter = true;
// java }
// java }
// java
// java return sb.toString();
// java }
fn excel_trim(s:&[u16])->Vec<u16>{let mut out=vec![];let(mut first,mut letter)=(0,false);for &c in s{if c==32||c==12288{if first==0{first=c;}}else{if first!=0&&letter{out.push(first);}out.push(c);first=0;letter=true;}}out}
fn invoke(&self,args:&[ValueRef],ec:&EvaluationContext)->FunctionResult<ValueRef>{if self.kind<=1{if args.len()!=1{return Err(ErrorEval::VALUE_INVALID.into());}let s=Self::evaluateStringArg(&args[0],ec)?;return Ok(if self.kind==0{Arc::new(NumberEval::NumberEval(s.len()as f64))}else{Arc::new(StringEval::from_utf16(Self::excel_trim(&s)))});}if self.kind<=3{if args.is_empty()||args.len()>2{return Err(ErrorEval::VALUE_INVALID.into());}let s=Self::evaluateStringArg(&args[0],ec)?;let index=if args.len()==1{1}else{Self::evaluateIntArg(&args[1],ec)?};if index<0{return Err(ErrorEval::VALUE_INVALID.into());}let n=(index as usize).min(s.len());return Ok(Arc::new(StringEval::from_utf16(if self.kind==2{s[..n].to_vec()}else{s[s.len()-n..].to_vec()})));}if args.len()!=2&&args.len()!=3{return Err(ErrorEval::VALUE_INVALID.into());}let start=if args.len()==3&&!args[2].as_any().is::<MissingArgEval>(){Self::evaluateIntArg(&args[2],ec)?.wrapping_sub(1)}else{0};if start<0||args[0].as_any().is::<MissingArgEval>()||args[1].as_any().is::<MissingArgEval>(){return Err(ErrorEval::VALUE_INVALID.into());}let needle=Self::evaluateStringArg(&args[0],ec)?;let haystack=Self::evaluateStringArg(&args[1],ec)?;self.search(&haystack,&needle,start as usize)}
// java private IValueEval eval(String haystack, String needle, int startIndex) {
// java if (startIndex != 0 && startIndex >= haystack.length()) {
// java return ErrorEval.VALUE_INVALID;
// java } else {
// java int result;
// java if (this._isSearch) {
// java Pattern p = LookupUtils.wildCardToPattern(needle, false);
// java if (p == null) {
// java result = haystack.toUpperCase().indexOf(needle.toUpperCase(), startIndex);
// java } else {
// java Matcher m = p.matcher(haystack);
// java if (m.find(startIndex)) {
// java result = m.start();
// java } else {
// java result = -1;
// java }
// java }
// java } else {
// java result = haystack.indexOf(needle, startIndex);
// java }
// java
// java return (IValueEval)(result == -1 ? ErrorEval.VALUE_INVALID : new NumberEval((double)(result + 1)));
// java }
// java }
fn search(&self,haystack:&[u16],needle:&[u16],start:usize)->FunctionResult<ValueRef>{if start!=0&&start>=haystack.len(){return Err(ErrorEval::VALUE_INVALID.into());}let result=if self.kind==5{if let Some(pattern)=LookupUtils::wildCardToPattern(needle,false)?{let s=String::from_utf16(haystack).map_err(|_|crate::cn::wps::et::ss::formula::evaluator::EvalFailure::Unported("UTF16 regex input surrogate"))?;let byte=String::from_utf16(&haystack[..start]).map_err(|_|crate::cn::wps::et::ss::formula::evaluator::EvalFailure::Unported("UTF16 regex split surrogate"))?.len();pattern.find_at(&s,byte).map(|m|s[..m.start()].encode_utf16().count())}else{let upper=|s:&[u16]|std::char::decode_utf16(s.iter().copied()).flat_map(|c|match c { Ok(c)=>c.to_uppercase().flat_map(|v|v.encode_utf16(&mut [0;2]).to_vec()).collect::<Vec<_>>(), Err(e)=>vec![e.unpaired_surrogate()] }).collect::<Vec<_>>();Self::indexOf(&upper(haystack),&upper(needle),start)}}else{Self::indexOf(haystack,needle,start)};match result{Some(i)=>Ok(Arc::new(NumberEval::NumberEval((i+1)as f64))),None=>Err(ErrorEval::VALUE_INVALID.into())}}
fn indexOf(h:&[u16],n:&[u16],start:usize)->Option<usize>{if n.is_empty(){return Some(start.min(h.len()));}if start>h.len()||n.len()>h.len()-start{return None;}(start..=h.len()-n.len()).find(|i|h[*i..*i+n.len()]==*n)}
}
impl MixOpFunction for TextFunction{fn valueArgRequired(&self,_i:usize,_a:&[ValueRef],_e:&EvaluationContext)->bool{true}fn evaluateOnce(&self,_r:i8,args:&[ValueRef],ec:&EvaluationContext)->EvalResult<ValueRef>{checked::catch(self.invoke(args,ec))}}
