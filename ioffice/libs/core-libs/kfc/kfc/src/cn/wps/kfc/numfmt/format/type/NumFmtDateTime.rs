use super::{IFormat::{IFormat,FormatValue},NumFmtBase::NumFmtBase};
use crate::cn::wps::kfc::numfmt::{math::TimeCalc::{TimeCalc,TimeStruct},format::{NumFmtParam::NumFmtParam,NumFmtResult::NumFmtResult},placeholder::{PieceContext::PieceContext,PlaceHolderToken::TokenRef,PlaceHolderID::PlaceHolderID,PlaceHolder::ulen},global::GlobalEnv::{GlobalEnv,localeStrManager}};
// java public class NumFmtDateTime extends NumFmtBase
pub struct NumFmtDateTime {pub base:NumFmtBase,mTime:TimeStruct,mIsValid:bool,mRealMinute:bool,mHasMillsecond:bool,mMillDecCount:i32,mHolderIdx:i32}
impl NumFmtDateTime {
    pub fn new()->Self{Self{base:NumFmtBase::new(8192),mTime:TimeStruct::default(),mIsValid:false,mRealMinute:false,mHasMillsecond:false,mMillDecCount:0,mHolderIdx:0}}
    // java private int roundMillSecond(int millisecond)
    fn roundMillSecond(&self,mut millis:i32)->i32 {if millis!=0&&self.mMillDecCount<=2{if self.mMillDecCount>1{millis=(millis as f32/10.0+0.5) as i32*10;}else if self.mMillDecCount>0{millis=(millis as f32/100.0+0.5) as i32*100;}}millis}
    // java getMillisecondStr: pad to three digits when len>=minReserved, otherwise pad to minReserved.
    fn getMillisecondStr(value:i32,minReserved:usize)->String{let digits=value.to_string();let len=digits.len();if len>=minReserved{"0".repeat(3usize.saturating_sub(len))+&digits}else{"0".repeat(minReserved-len)+&digits}}
}
impl IFormat for NumFmtDateTime {
    fn base(&self)->&NumFmtBase{&self.base}fn base_mut(&mut self)->&mut NumFmtBase{&mut self.base}
    fn reset_context(&mut self,p:&PieceContext){
        // java super.reset(pieceContext);mIsValid=false;mRealMinute=false;mHasMillsecond=false;mHolderIdx=-1;mTime.mEra=-1;mMillDecCount=0;
        self.base.reset_context(p);self.mIsValid=false;self.mRealMinute=false;self.mHasMillsecond=false;self.mHolderIdx=-1;self.mTime.mEra=-1;self.mMillDecCount=0;
    }
    fn readyFormat(&mut self,v:FormatValue<'_>)->bool{
        if let FormatValue::Number(value)=v{
            // java mFormatValue=value;mIsValid=TimeCalc.double2Time(value,mTime,mUse1904,mCalendarType);mFormatterContext.mTime=mTime;
            self.base.mFormatterContext.borrow_mut().mFormatValue=value;self.mIsValid=TimeCalc::double2Time(value,&mut self.mTime,self.base.mNumFmtStyle.mUse1904,self.base.mNumFmtStyle.mCalendarType);
            if self.mIsValid&&self.mHasMillsecond{
                // java mTime.mMillisecond=roundMillSecond(...);mTime.mSecond-=mSecond>0&&mMillisecond>500?1:0;mDecimal=mMillisecond>0?getMillisecondStr(...,1):"";mRDigitHolderCount=mMillDecCount;
                self.mTime.mMillisecond=self.roundMillSecond(self.mTime.mMillisecond);if self.mTime.mSecond>0&&self.mTime.mMillisecond>500{self.mTime.mSecond-=1;}self.base.mFormatterContext.borrow_mut().mDecimal=if self.mTime.mMillisecond>0{Self::getMillisecondStr(self.mTime.mMillisecond,1)}else{String::new()};self.base.mPHolderContext.mRDigitHolderCount=self.mMillDecCount;
            }
            self.base.mFormatterContext.borrow_mut().mTime=Some(self.mTime.clone());
        }self.mIsValid
    }
    fn doFormat(&mut self,r:&mut NumFmtResult){
        for i in 0..self.base.mHolderTokens.len(){
            // java if(mHasMillsecond&&holderToken.mPlaceHolder.mPlaceHolderID==9)mRDigitOffset=0;holderToken.doMutexHolder().format(...);
            if self.mHasMillsecond&&self.base.mHolderTokens[i].lock().unwrap_or_else(std::sync::PoisonError::into_inner).getHolderID()==9{self.base.mPHolderContext.mRDigitOffset=0;}self.base.formatToken(i,r);
        }
    }
    fn finishFormat(&mut self,p:&NumFmtParam,r:&mut NumFmtResult){
        // java if(mIsValid)super.finishFormat(param,result);else result.mNumFmtStr.append(invalidNumberSignString);
        if self.mIsValid{self.base.finishFormat(p,r);}else{r.mNumFmtStr.push_str(&"#".repeat(255));}
    }
    fn collectInfo(&mut self,token:&TokenRef,index:usize){
        let mut token=token.lock().unwrap_or_else(std::sync::PoisonError::into_inner);let id=token.getHolderID();
        match id {
            // java case 3: if(mHolderIdx> -1) previous token becomes minute;realMinute=false;hasMillis=true;millDecCount+=token.length();
            3=>{if self.mHolderIdx> -1{self.base.mHolderTokens[self.mHolderIdx as usize].lock().unwrap_or_else(std::sync::PoisonError::into_inner).setPlaceHolder(PlaceHolderID::getPlaceHolder(27));self.mHolderIdx=-1;}self.mRealMinute=false;self.mHasMillsecond=true;self.mMillDecCount+=ulen(&token.mToken) as i32;},
            // java case 5: set plain holder and replace slash by repeated local slash if available.
            5=>{token.setPlaceHolder(PlaceHolderID::getPlaceHolder(0));let slash=localeStrManager.lock().unwrap_or_else(std::sync::PoisonError::into_inner).getSlash();if let Some(ch)=slash.chars().next(){token.mToken=ch.to_string().repeat(ulen(&token.mToken));}},
            9=>self.mMillDecCount=0,
            20|21=>{if GlobalEnv::getDefaultLCID()==1041&&!self.base.mNumFmtStyle.mHasNCL{self.base.mNumFmtStyle.mCalendarType=3;}},
            22=>{if !self.base.mNumFmtStyle.mHasNCL&&token.mToken=="B2"{self.base.mNumFmtStyle.mCalendarType=6;}},
            23|25|30|31|32|33|36|37|38|39|40|41=>{self.mHolderIdx=-1;self.mRealMinute=false;},
            24=>{if ulen(&token.mToken)>2&&GlobalEnv::getDefaultLCID()==1025&&!self.base.mNumFmtStyle.mHasNCL{self.base.mNumFmtStyle.mHasNCL=true;self.base.mNumFmtStyle.mNumeralShape=2;self.base.mNumFmtStyle.mLocaleCodeID=1025;}self.mHolderIdx=-1;self.mRealMinute=false;},
            26|29=>{self.mHolderIdx=-1;self.mRealMinute=true;},
            // java case 27: if(!mRealMinute&&contextType==2) change token to month and remember index;else reset index;realMinute=false;
            27=>{if !self.mRealMinute&&self.base.mPHolderContext.mContextType==2{token.setPlaceHolder(PlaceHolderID::getPlaceHolder(24));self.mHolderIdx=index as i32;}else{self.mHolderIdx=-1;}self.mRealMinute=false;},
            // java case 28: turn remembered month into minute when followed by seconds.
            28=>{if self.mHolderIdx> -1{self.base.mHolderTokens[self.mHolderIdx as usize].lock().unwrap_or_else(std::sync::PoisonError::into_inner).setPlaceHolder(PlaceHolderID::getPlaceHolder(27));self.mHolderIdx=-1;self.mRealMinute=false;}},
            34=>self.base.mPHolderContext.mIs24HR=false,_=>{}
        }
    }
}
