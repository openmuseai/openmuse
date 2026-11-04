use super::{IFormat::{IFormat,FormatValue},NumFmtBase::NumFmtBase};
use crate::cn::wps::kfc::numfmt::{format::{NumFmtParam::NumFmtParam,NumFmtResult::NumFmtResult},placeholder::{PieceContext::PieceContext,PlaceHolderToken::TokenRef,PlaceHolder::ulen},util::NumberToStr::NumberToStr,math::Numeric::BigDecimal};
// java public class NumFmtScience extends NumFmtBase
pub struct NumFmtScience {pub base:NumFmtBase,mScience:[String;4],mIntHolderCount:i32,mDecHolderCount:i32,mExpHolderCount:i32,mDecPointPos:i32,mExponentPos:i32,realValue:f64}
impl NumFmtScience {pub fn new()->Self{Self{base:NumFmtBase::new(16),mScience:std::array::from_fn(|_|String::new()),mIntHolderCount:0,mDecHolderCount:0,mExpHolderCount:0,mDecPointPos:0,mExponentPos:0,realValue:0.0}}}
impl IFormat for NumFmtScience {
fn base(&self)->&NumFmtBase{&self.base}fn base_mut(&mut self)->&mut NumFmtBase{&mut self.base}
fn reset_context(&mut self,p:&PieceContext){self.base.reset_context(p);self.mDecPointPos=-1;self.mExponentPos=-1;self.mIntHolderCount=0;self.mDecHolderCount=0;self.mExpHolderCount=0;}
fn collectInfo(&mut self,token:&TokenRef,index:usize){let token=token.lock().unwrap_or_else(std::sync::PoisonError::into_inner);match token.getHolderID(){2|3|4=>{let count=ulen(&token.mToken) as i32;if self.mDecPointPos== -1&&self.mExponentPos== -1{self.mIntHolderCount+=count;}else if self.mDecPointPos> -1&&self.mExponentPos== -1{self.mDecHolderCount+=count;}else{self.mExpHolderCount+=count;}},7=>self.mExponentPos=index as i32,8=>self.base.mNumFmtStyle.mHasThousSep=true,9=>{if self.mDecPointPos== -1{self.mDecPointPos=index as i32;}},_=>{}}}
fn readyFormat(&mut self,v:FormatValue<'_>)->bool{
self.base.readyFormat(v);let value=self.base.mFormatterContext.borrow().mFormatValue;let mut intPartCount=self.mIntHolderCount;
// java engineering form chooses integerCount % mIntHolderCount for nonzero integer values.
if self.mIntHolderCount>1&&value as i64!=0{let exponent=value.abs().log10() as i32;let integerCount=if exponent<1{1}else{exponent+1};intPartCount=integerCount%self.mIntHolderCount;if intPartCount==0{intPartCount=self.mIntHolderCount;}}
if value!=0.0{self.realValue=NumberToStr::double2Science(value,self.mDecHolderCount,intPartCount,&mut self.mScience);}else{self.realValue=0.0;self.mScience=["0".repeat(self.mIntHolderCount as usize),String::new(),"+".into(),"0".into()];}true
}
fn doFormat(&mut self,r:&mut NumFmtResult){
{let mut f=self.base.mFormatterContext.borrow_mut();f.mInteger=self.mScience[0].clone();f.mDecimal=self.mScience[1].clone();}
let c=&mut self.base.mPHolderContext;c.mFirstDigitPass=false;c.mOffset=if self.mDecPointPos==0{0}else{ulen(&self.mScience[0]) as i32};c.mLDigitHolderCount=self.mIntHolderCount;c.mLDigitOffset=c.mLDigitHolderCount;c.mRDigitHolderCount=self.mDecHolderCount;
let mut index=0;while (index as i32)<self.mExponentPos{self.base.formatToken(index,r);index+=1;}
// java positive exponent with E- prints E; negative exponent with E+ prints E-; otherwise preserve token.
{let token=self.base.mHolderTokens[index].lock().unwrap_or_else(std::sync::PoisonError::into_inner);if self.mScience[2]=="+"&&token.mToken.contains('-'){r.mNumFmtStr.push('E');}else if self.mScience[2]=="-"&&token.mToken.contains('+'){r.mNumFmtStr.push_str("E-");}else{r.mNumFmtStr.push_str(&token.mToken);}}
index+=1;{let mut f=self.base.mFormatterContext.borrow_mut();f.mInteger=self.mScience[3].clone();f.mDecimal.clear();}
let c=&mut self.base.mPHolderContext;c.mFirstDigitPass=false;c.mOffset=ulen(&self.mScience[3]) as i32;c.mLDigitHolderCount=self.mExpHolderCount;c.mLDigitOffset=c.mLDigitHolderCount;c.mRDigitHolderCount=0;c.mRDigitOffset=0;
while index<self.base.mHolderTokens.len(){self.base.formatToken(index,r);index+=1;}
}
fn formatValue(&mut self,d:f64,_:&NumFmtParam)->f64{
if self.readyFormat(FormatValue::Number(d)){let exp=self.mScience[3].parse::<i32>().expect("NumberFormatException");let mut base=1.0;for _ in 0..exp{base*=10.0;}let pre=if d<0.0{-1.0}else{1.0};if self.mScience[2]=="-"{return BigDecimal::valueOf(self.realValue).divide(&BigDecimal::valueOf(base)).doubleValue()*pre;}if self.mScience[2]=="+"{return BigDecimal::valueOf(self.realValue).multiply(&BigDecimal::valueOf(base)).doubleValue()*pre;}}0.0
}
}
