use std::{rc::Rc,cell::RefCell};
use super::IFormat::{IFormat,FormatValue};
use crate::cn::wps::kfc::numfmt::{format::{FormatContext::FormatContext,FormatLocale::FormatLocale,NumFmtStyle::NumFmtStyle,NumFmtParam::NumFmtParam,NumFmtResult::NumFmtResult},placeholder::{PlaceHolderContext::PlaceHolderContext,PieceContext::PieceContext,PlaceHolderToken::{PlaceHolderToken,TokenRef},PlaceHolder::ulen}};
// java public class NumFmtBase implements IFormat
pub struct NumFmtBase {pub mNumFmtType:i16,pub mHolderTokens:Vec<TokenRef>,pub mPHolderContext:PlaceHolderContext,pub mFormatterContext:Rc<RefCell<FormatContext>>,pub mNumFmtStyle:NumFmtStyle,pub mFormatLocale:FormatLocale,pub mNumFmtParam:Option<NumFmtParam>}
impl NumFmtBase {
    // java initialize contexts/style/locale;this.mPHolderContext.mFormatterContext=this.mFormatterContext;
    pub fn new(numFmtType:i16)->Self{let f=Rc::new(RefCell::new(FormatContext::new()));let mut p=PlaceHolderContext::new();p.mFormatterContext=f.clone();Self{mNumFmtType:numFmtType,mHolderTokens:Vec::new(),mPHolderContext:p,mFormatterContext:f,mNumFmtStyle:NumFmtStyle::new(),mFormatLocale:FormatLocale::new(),mNumFmtParam:None}}
    pub fn reset_context(&mut self,p:&PieceContext){
        // java mPHolderContext.clear();mNumFmtStyle.clear();mHolderTokens.clear();mPHolderContext.mContextType=pieceContext.mContextType;mPHolderContext.mAddMinus=pieceContext.mAddMinus;mFormatLocale.reset(pieceContext,mNumFmtStyle);
        self.mPHolderContext.clear();self.mNumFmtStyle.clear();self.mHolderTokens.clear();self.mPHolderContext.mContextType=p.mContextType;self.mPHolderContext.mAddMinus=p.mAddMinus;self.mFormatLocale.reset(p,&mut self.mNumFmtStyle);
    }
    pub fn prepare(&mut self,p:&NumFmtParam,r:&mut NumFmtResult){
        // java mNumFmtParam=param;mNumFmtStyle.mMaxLength=param.mMaxLength> -1?param.mMaxLength:255;mNumFmtStyle.mUse1904=param.isUse1904;mFormatterContext.clear();result.clear();
        self.mNumFmtParam=Some(p.clone());self.mNumFmtStyle.mMaxLength=if p.mMaxLength> -1{p.mMaxLength}else{255};self.mNumFmtStyle.mUse1904=p.isUse1904;self.mFormatterContext.borrow_mut().clear();r.clear();
    }
    pub fn readyFormat(&mut self,v:FormatValue<'_>)->bool{
        // java if(value instanceof Number) mFormatValue=((Number)value).doubleValue();else mFormatText=value.toString();return true;
        let mut f=self.mFormatterContext.borrow_mut();match v{FormatValue::Number(n)=>f.mFormatValue=n,FormatValue::Text(s)=>f.mFormatText=Some(s.into())}true
    }
    pub fn formatToken(&mut self,index:usize,result:&mut NumFmtResult){
        self.mPHolderContext.mPos=index as i32;let token=self.mHolderTokens[index].clone();let data=token.lock().unwrap_or_else(std::sync::PoisonError::into_inner);let mut holder=data.doMutexHolder();holder.format_value(&mut self.mPHolderContext,&self.mNumFmtStyle,result);
    }
    // java iterate holder tokens and invoke holderToken.doMutexHolder().format(...).
    pub fn doFormat(&mut self,result:&mut NumFmtResult){for i in 0..self.mHolderTokens.len(){self.formatToken(i,result);}}
    pub fn finishFormat(&mut self,p:&NumFmtParam,r:&mut NumFmtResult){
        // java this.recycle();if(param.mMaxLength>=0){int fillLength=param.mMaxLength-result.mNumFmtStr.length();if(mFillPos> -1&&fillLength>= -1){result.mFillPos=mFillPos;result.mFillChar=mFillChar;}else if(fillLength<0){result.clear text;append fillChar('#',maxLength);}}
        self.recycle();if p.mMaxLength>=0{let len=p.mMaxLength-ulen(&r.mNumFmtStr) as i32;if self.mPHolderContext.mFillPos> -1&&len>= -1{r.mFillPos=self.mPHolderContext.mFillPos;r.mFillChar=self.mPHolderContext.mFillChar;}else if len<0{r.mNumFmtStr="#".repeat(p.mMaxLength as usize);}}
    }
    pub fn recycle(&mut self){for token in &self.mHolderTokens{PlaceHolderToken::recycle(token);}self.mHolderTokens.clear();}
    // java protected final boolean isZero(CharSequence str): all code units equal '0' (empty is true).
    pub fn isZero(s:&str)->bool {s.chars().all(|c|c=='0')}
}
impl IFormat for NumFmtBase {fn base(&self)->&Self{self}fn base_mut(&mut self)->&mut Self{self}}
