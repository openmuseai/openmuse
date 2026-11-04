use super::{IFormat::IFormat,NumFmtBase::NumFmtBase};
use crate::cn::wps::kfc::numfmt::{format::{NumFmtResult::NumFmtResult,NumFmtParam::NumFmtParam},placeholder::PlaceHolderToken::TokenRef};
// java public class NumFmtText extends NumFmtBase
pub struct NumFmtText {pub base:NumFmtBase}
impl NumFmtText {pub fn new()->Self {Self{base:NumFmtBase::new(64)}}}
impl IFormat for NumFmtText {fn base(&self)->&NumFmtBase{&self.base}fn base_mut(&mut self)->&mut NumFmtBase{&mut self.base}
fn doFormat(&mut self,r:&mut NumFmtResult){
// java if(valueIsText())super.doFormat(result);else{convertGeneral(0,mFormatValue,maxLength);replaceDecPointLocal;result.mNumericAsText=true;}
let is_text=self.base.mFormatterContext.borrow().valueIsText();if is_text{self.base.doFormat(r);}else{let value=self.base.mFormatterContext.borrow().mFormatValue;let text=crate::cn::wps::kfc::numfmt::format::NumConverter::NumConverter::convertGeneral(0,value,self.base.mNumFmtStyle.mMaxLength);crate::cn::wps::kfc::numfmt::global::GlobalEnv::localeStrManager.lock().unwrap_or_else(std::sync::PoisonError::into_inner).replaceDecPointLocal(&mut r.mNumFmtStr,&text);r.mNumericAsText=true;}}
// java if(mPHolderContext.mFillPos<0)this.recycle();else super.finishFormat(param,result);
fn finishFormat(&mut self,p:&NumFmtParam,r:&mut NumFmtResult){if self.base.mPHolderContext.mFillPos<0{self.base.recycle();}else{self.base.finishFormat(p,r);}}
}
