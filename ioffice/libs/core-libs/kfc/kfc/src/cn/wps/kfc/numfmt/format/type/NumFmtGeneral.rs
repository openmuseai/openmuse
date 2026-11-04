use super::{IFormat::IFormat,NumFmtBase::NumFmtBase};
use crate::cn::wps::kfc::numfmt::{format::{NumFmtResult::NumFmtResult,NumFmtParam::NumFmtParam},placeholder::PlaceHolderToken::TokenRef};
// java public class NumFmtGeneral extends NumFmtBase
pub struct NumFmtGeneral {pub base:NumFmtBase}
impl NumFmtGeneral {pub fn new()->Self {Self{base:NumFmtBase::new(1)}}}
impl IFormat for NumFmtGeneral {fn base(&self)->&NumFmtBase{&self.base}fn base_mut(&mut self)->&mut NumFmtBase{&mut self.base}
// java if(holderToken.mPlaceHolder.mPlaceHolderID==18)this.mNumFmtStyle.mIsRMBScale=holderToken.mToken.indexOf("RMB")> -1;
fn collectInfo(&mut self,token:&TokenRef,_:usize){let token=token.lock().unwrap_or_else(std::sync::PoisonError::into_inner);if token.getHolderID()==18{self.base.mNumFmtStyle.mIsRMBScale=token.mToken.contains("RMB");}}
// java if(mPHolderContext.mFillPos<0)this.recycle();else super.finishFormat(param,result);
fn finishFormat(&mut self,p:&NumFmtParam,r:&mut NumFmtResult){if self.base.mPHolderContext.mFillPos<0{self.base.recycle();}else{self.base.finishFormat(p,r);}}
}
