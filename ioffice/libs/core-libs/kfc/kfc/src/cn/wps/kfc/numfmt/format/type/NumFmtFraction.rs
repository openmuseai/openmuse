use super::{IFormat::{IFormat,FormatValue},NumFmtBase::NumFmtBase};
use crate::cn::wps::kfc::numfmt::{format::{NumFmtParam::NumFmtParam,NumFmtResult::NumFmtResult},placeholder::{PieceContext::PieceContext,PlaceHolderToken::TokenRef,PlaceHolderID::PlaceHolderID,PlaceHolder::ulen},util::NumberToStr::NumberToStr};
// java public class NumFmtFraction extends NumFmtBase
pub struct NumFmtFraction {pub base:NumFmtBase,mIntPartCount:i32,mNumeratorCount:i32,mDenominator:i32,mIntPartEndPos:i32,mFractionPos:i32,mDenomStartPos:i32,mDenomEndPos:i32,mDigitDenom:bool,mDigitDenomPos:Vec<usize>,mFractions:[String;3]}
impl NumFmtFraction {pub fn new()->Self{Self{base:NumFmtBase::new(8),mIntPartCount:0,mNumeratorCount:0,mDenominator:0,mIntPartEndPos:0,mFractionPos:0,mDenomStartPos:0,mDenomEndPos:0,mDigitDenom:false,mDigitDenomPos:Vec::new(),mFractions:std::array::from_fn(|_|String::new())}}}
impl IFormat for NumFmtFraction {
fn base(&self)->&NumFmtBase{&self.base}fn base_mut(&mut self)->&mut NumFmtBase{&mut self.base}
fn reset_context(&mut self,p:&PieceContext){self.base.reset_context(p);self.mIntPartCount=0;self.mNumeratorCount=0;self.mDenominator=0;self.mIntPartEndPos=-1;self.mFractionPos=-1;self.mDenomStartPos=-1;self.mDenomEndPos=-1;self.mDigitDenom=false;self.mDigitDenomPos.clear();}
fn collectInfo(&mut self,t:&TokenRef,index:usize){
let t=t.lock().unwrap_or_else(std::sync::PoisonError::into_inner);let id=t.getHolderID();match id{
2|3|4=>{if self.mFractionPos> -1&&self.mIntPartEndPos== -1{self.mNumeratorCount+=ulen(&t.mToken) as i32;}else if self.mIntPartEndPos> -1{self.mIntPartCount+=ulen(&t.mToken) as i32;}},5=>self.mFractionPos=index as i32,8=>self.base.mNumFmtStyle.mHasThousSep=true,15=>{if self.mFractionPos== -1{self.mDigitDenom=true;}},_=>{if id==6{self.base.mNumFmtStyle.mHasPercent=true;}if self.mFractionPos> -1&&self.mIntPartEndPos== -1&&self.mNumeratorCount>0{self.mIntPartEndPos=index as i32;}}}
}
fn reset(&mut self,tokens:&[TokenRef],p:&PieceContext){
// java this.reset(pieceContext);this.mFormatLocale.reset(...); reverse insertion and collectInfo.
self.reset_context(p);self.base.mFormatLocale.reset(p,&mut self.base.mNumFmtStyle);for index in (0..tokens.len()).rev(){self.base.mHolderTokens.insert(0,tokens[index].clone());self.collectInfo(&tokens[index],index);}
let mut denominator=String::new();let mut hasFind=false;let mut i=self.mFractionPos+1;
while (i as usize)<tokens.len(){let mut t=self.base.mHolderTokens[i as usize].lock().unwrap_or_else(std::sync::PoisonError::into_inner);let id=t.getHolderID();
if self.mDigitDenom{if id!=15&&id!=3{if hasFind{break;}}else{denominator.push_str(&t.mToken);self.mDigitDenomPos.push(i as usize);if self.mDenomStartPos== -1{self.mDenomStartPos=i;}hasFind=true;}if id==3{t.setPlaceHolder(PlaceHolderID::getPlaceHolder(15));}}
else if !matches!(id,2|3|4){if hasFind{break;}}else{self.mDenominator+=ulen(&t.mToken) as i32;if self.mDenomStartPos== -1{self.mDenomStartPos=i;}hasFind=true;}i+=1;
}
self.mDenomEndPos=i;if self.mDigitDenom{self.mDenominator=denominator.parse().expect("NumberFormatException");}
}
fn readyFormat(&mut self,v:FormatValue<'_>)->bool{self.base.readyFormat(v);let mut value=self.base.mFormatterContext.borrow().mFormatValue;if self.base.mNumFmtStyle.mHasPercent{value*=100.0;}NumberToStr::double2Fraction(value,self.mDenominator,self.mIntPartCount>0,self.mDigitDenom,&mut self.mFractions);true}
fn formatValue(&mut self,d:f64,_:&NumFmtParam)->f64{
self.readyFormat(FormatValue::Number(d));let mut integer=0i32;let mut numerator=1i32;let mut denominator=1i32;
// java sequential assignments in one try retain earlier parsed fields on failure.
'parse:{integer=if self.mFractions[0].is_empty(){0}else{let Ok(v)=self.mFractions[0].parse()else{break 'parse};v};numerator=if self.mFractions[1].is_empty(){0}else{let Ok(v)=self.mFractions[1].parse()else{break 'parse};v};denominator=if self.mFractions[2].is_empty(){1}else{let Ok(v)=self.mFractions[2].parse()else{break 'parse};v};}
(integer.wrapping_abs() as f64+(numerator as f64/denominator as f64).abs())*if d<0.0{-1.0}else{1.0}
}
fn doFormat(&mut self,r:&mut NumFmtResult){
if self.mFractions[0]=="#".repeat(255){r.mNumFmtStr.push_str(&self.mFractions[0]);return;}
let mut index=0;
if self.mIntPartEndPos>0{self.base.mFormatterContext.borrow_mut().mInteger=self.mFractions[0].clone();let c=&mut self.base.mPHolderContext;c.mOffset=ulen(&self.mFractions[0]) as i32;c.mLDigitHolderCount=self.mIntPartCount;c.mLDigitOffset=c.mLDigitHolderCount;while (index as i32)<self.mIntPartEndPos{self.base.formatToken(index,r);index+=1;}}
self.base.mNumFmtStyle.mHasThousSep=false;self.base.mFormatterContext.borrow_mut().mInteger=self.mFractions[1].clone();let c=&mut self.base.mPHolderContext;c.mFirstDigitPass=false;c.mOffset=ulen(&self.mFractions[1]) as i32;c.mLDigitHolderCount=self.mNumeratorCount;c.mLDigitOffset=c.mLDigitHolderCount;while index as i32<=self.mFractionPos{self.base.formatToken(index,r);index+=1;}
let hasDenom=!self.mFractions[2].is_empty();let firstAsk=self.base.mHolderTokens[self.mDenomStartPos as usize].lock().unwrap_or_else(std::sync::PoisonError::into_inner).mToken.starts_with('?');
if firstAsk{let mut f=self.base.mFormatterContext.borrow_mut();f.mInteger.clear();f.mDecimal=self.mFractions[2].clone();let c=&mut self.base.mPHolderContext;c.mOffset=-1;c.mRDigitOffset=0;c.mRDigitHolderCount=if self.mDigitDenom{0}else{self.mDenominator};}else{self.base.mFormatterContext.borrow_mut().mInteger=self.mFractions[2].clone();let c=&mut self.base.mPHolderContext;c.mFirstDigitPass=false;c.mOffset=ulen(&self.mFractions[2]) as i32;c.mLDigitHolderCount=if self.mDigitDenom{0}else{self.mDenominator};c.mLDigitOffset=c.mLDigitHolderCount;c.mRDigitHolderCount=0;}
while (index as i32)<self.mDenomEndPos{self.base.mPHolderContext.mPos=index as i32;let token=self.base.mHolderTokens[index].clone();let mut token=token.lock().unwrap_or_else(std::sync::PoisonError::into_inner);if !hasDenom&&self.mDigitDenom&&self.mDigitDenomPos.contains(&index){token.mToken=" ".into();}else if !hasDenom&&token.getHolderID()==3{r.mNumFmtStr.push_str(&" ".repeat(ulen(&token.mToken)));index+=1;continue;}drop(token);self.base.formatToken(index,r);index+=1;}
self.base.mPHolderContext.mOffset=-1;self.base.mPHolderContext.mRDigitHolderCount=0;
while index<self.base.mHolderTokens.len(){let token=self.base.mHolderTokens[index].clone();let token=token.lock().unwrap_or_else(std::sync::PoisonError::into_inner);match token.getHolderID(){2=>r.mNumFmtStr.push_str(&"0".repeat(ulen(&token.mToken))),3=>r.mNumFmtStr.push_str(&token.mToken),4=>r.mNumFmtStr.push_str(&" ".repeat(ulen(&token.mToken))),_=>{drop(token);self.base.formatToken(index,r);}}index+=1;}
}
}
