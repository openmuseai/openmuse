use super::AnalyzerBase::AnalyzerBase;
use crate::cn::wps::kfc::numfmt::placeholder::{PlaceHolderToken::TokenRef,PlaceHolderID::PlaceHolderID};
// java public abstract class AnalyzerNonNumeric extends AnalyzerBase {
pub struct AnalyzerNonNumeric{pub base:AnalyzerBase}
impl AnalyzerNonNumeric{
    // java public AnalyzerNonNumeric(){super();}
    pub fn new()->Self{Self{base:AnalyzerBase::new()}}
    // java protected void filter(ArrayList<PlaceHolderToken> tokens) {
    pub fn filter(&mut self,tokens:&[TokenRef]){
        // java int i=0;for(int count=tokens.size();i<count;++i){PlaceHolderToken token=tokens.get(i);switch(token.getHolderID()){
        let mut i=0;let count=tokens.len();while i<count{let id=tokens[i].lock().unwrap_or_else(std::sync::PoisonError::into_inner).getHolderID();match id{
            // java case 6:token.setPlaceHolder(PlaceHolderID.getPlaceHolder((short)0));case 7:default:break;
            6=>{tokens[i].lock().unwrap_or_else(std::sync::PoisonError::into_inner).mPlaceHolder=Some(PlaceHolderID::getPlaceHolder(0));},
            // java case 8:token.setPlaceHolder(PlaceHolderID.getPlaceHolder((short)0));token.mToken=token.mToken.substring(0,1);case 9:if(i+1>=count||tokens.get(i+1).getHolderID()!=3){token.setPlaceHolder(PlaceHolderID.getPlaceHolder((short)0));}
            8|9=>{if id==8{let mut token=tokens[i].lock().unwrap_or_else(std::sync::PoisonError::into_inner);token.mPlaceHolder=Some(PlaceHolderID::getPlaceHolder(0));token.mToken=super::super::super::placeholder::PlaceHolder::usub(&token.mToken,0,1);}if i+1>=count||tokens[i+1].lock().unwrap_or_else(std::sync::PoisonError::into_inner).getHolderID()!=3{tokens[i].lock().unwrap_or_else(std::sync::PoisonError::into_inner).mPlaceHolder=Some(PlaceHolderID::getPlaceHolder(0));}},_=>{}
        }i+=1;}
    }
    // java protected void setContext() { }
    pub fn setContext(&mut self){}
    // java protected void dealThouSepToken() { }
    pub fn dealThouSepToken(&mut self){}
}
