use super::{IFormat::{IFormat,FormatValue},NumFmtBase::NumFmtBase};
use crate::cn::wps::kfc::numfmt::{format::{NumFmtParam::NumFmtParam,NumFmtResult::NumFmtResult},placeholder::{PieceContext::PieceContext,PlaceHolderToken::TokenRef,PlaceHolder::ulen},util::NumberToStr::NumberToStr};
// java public class NumFmtNumberic extends NumFmtBase
pub struct NumFmtNumberic {pub base:NumFmtBase,pub mIntHolderCount:i32,pub mDecHolderCount:i32,pub mLastThousCount:i32,pub mThousSepCount:i32,pub mHasDecPoint:bool,pub mHasIntXCut:bool}
impl NumFmtNumberic {
    pub fn new()->Self{Self::new_with_type(512)}
    pub fn new_with_type(ty:i16)->Self{Self{base:NumFmtBase::new(ty),mIntHolderCount:0,mDecHolderCount:0,mLastThousCount:0,mThousSepCount:0,mHasDecPoint:false,mHasIntXCut:false}}
}
impl IFormat for NumFmtNumberic {
    fn base(&self)->&NumFmtBase{&self.base}fn base_mut(&mut self)->&mut NumFmtBase{&mut self.base}
    fn reset_context(&mut self,p:&PieceContext){
        // java super.reset(pieceContext); counts zero, flags false.
        self.base.reset_context(p);self.mIntHolderCount=0;self.mDecHolderCount=0;self.mLastThousCount=0;self.mThousSepCount=0;self.mHasDecPoint=false;self.mHasIntXCut=false;
    }
    fn reset(&mut self,tokens:&[TokenRef],p:&PieceContext){
        // java super.reset(holderTokens,pieceContext);if(mHasThousSep&&mThousSepCount-mLastThousCount==0)mHasThousSep=false;
        self.reset_context(p);for (i,t) in tokens.iter().enumerate(){self.base.mHolderTokens.push(t.clone());self.collectInfo(t,i);}if self.base.mNumFmtStyle.mHasThousSep&&self.mThousSepCount-self.mLastThousCount==0{self.base.mNumFmtStyle.mHasThousSep=false;}
    }
    fn collectInfo(&mut self,token:&TokenRef,_:usize){
        let token=token.lock().unwrap_or_else(std::sync::PoisonError::into_inner);let id=token.getHolderID();
        match id {
            // java case 6:mHasPercent=true;break;case 8:last+=tokenLen;count+=tokenLen;break;case 9:mHasDecPoint=true;break;
            6=>self.base.mNumFmtStyle.mHasPercent=true,8=>{let len=ulen(&token.mToken) as i32;self.mLastThousCount+=len;self.mThousSepCount+=len;},9=>self.mHasDecPoint=true,
            // java case 42 sets mHasIntXCut then falls through to digit handling.
            42|2|3|4=>{if id==42&&self.mIntHolderCount==0&&!self.mHasIntXCut{self.mHasIntXCut=true;}if self.mLastThousCount>0{self.mLastThousCount-=1;self.base.mNumFmtStyle.mHasThousSep=true;}if self.mHasDecPoint{self.mDecHolderCount+=ulen(&token.mToken) as i32;}else{self.mIntHolderCount+=ulen(&token.mToken) as i32;}},_=>{}
        }
    }
    fn readyFormat(&mut self,v:FormatValue<'_>)->bool{
        self.base.readyFormat(v);
        // java double formatValue=mFormatValue*Math.pow(10,-3*mLastThousCount);String doubleStr=NumberToStr.double2String(formatValue,mDecHolderCount,mHasPercent);
        let value=self.base.mFormatterContext.borrow().mFormatValue*10f64.powi(-3*self.mLastThousCount);let text=NumberToStr::double2String_percent(value,self.mDecHolderCount,self.base.mNumFmtStyle.mHasPercent);
        // java if(mAddMinus&&isZero(doubleStr))mAddMinus=false;
        if self.base.mPHolderContext.mAddMinus&&NumFmtBase::isZero(&text){self.base.mPHolderContext.mAddMinus=false;}
        let mut f=self.base.mFormatterContext.borrow_mut();let (int,dec)=text.split_once('.').unwrap_or((&text,""));f.mInteger=if int=="0"{String::new()}else{int.into()};f.mDecimal=dec.into();let mut len=ulen(&f.mInteger) as i32;
        // java if(mHasIntXCut&&integerLen>mIntHolderCount) keep rightmost mIntHolderCount digits.
        if self.mHasIntXCut&&len>self.mIntHolderCount{f.mInteger=f.mInteger[(len-self.mIntHolderCount) as usize..].into();len=self.mIntHolderCount;}
        let c=&mut self.base.mPHolderContext;c.mOffset=len;c.mLDigitHolderCount=self.mIntHolderCount;c.mLDigitOffset=c.mLDigitHolderCount;c.mRDigitHolderCount=self.mDecHolderCount;true
    }
    fn formatValue(&mut self,d:f64,p:&NumFmtParam)->f64{
        if !self.readyFormat(FormatValue::Number(d)){return d;}
        // java result=new NumFmtResult();if(mAddMinus) append('-');doFormat;finishFormat;
        let mut result=NumFmtResult::new();if self.base.mPHolderContext.mAddMinus{result.mNumFmtStr.push('-');}self.doFormat(&mut result);self.finishFormat(p,&mut result);
        // java parse real value;preserve original d sign;on NumberFormatException construct sign+integer+"."+decimal; return +0 for zero.
        let ret=if let Ok(mut ret)=result.mNumRealValue.parse::<f64>(){if ret>0.0&&d<0.0||ret<0.0&&d>0.0{ret*= -1.0;}ret}else{let f=self.base.mFormatterContext.borrow();format!("{}{}.{}",if d<0.0{"-"}else{""},if f.mInteger.is_empty(){"0"}else{&f.mInteger},f.mDecimal).parse().expect("NumberFormatException")};if ret==0.0{0.0}else{ret}
    }
}
