use super::{AnalyzerNonNumeric::AnalyzerNonNumeric,AnalyzerBase::{AnalyzerBase,AnalyzerDispatch}};
use crate::cn::wps::kfc::numfmt::placeholder::PlaceHolderToken::TokenRef;
// java public class AnalyzerGeneral extends AnalyzerNonNumeric {
pub struct AnalyzerGeneral{pub base:AnalyzerNonNumeric}
impl AnalyzerGeneral{
    // java public AnalyzerGeneral(){super();}
    pub fn new()->Self{Self{base:AnalyzerNonNumeric::new()}}
    // java protected boolean checkSyntax(){int i=0;for(int count=this.mTokens.size();i<count;++i){PlaceHolderToken token=this.mTokens.get(i);if(!token.isEmpty()&&token.getHolderID()==15){return false;}}return true;}
    pub fn checkSyntax(&self)->bool{let mut i=0;let count=self.base.base.mTokens.len();while i<count{let token=self.base.base.mTokens[i].lock().unwrap_or_else(std::sync::PoisonError::into_inner);if !token.isEmpty()&&token.getHolderID()==15{return false;}i+=1;}true}
}
impl AnalyzerDispatch for AnalyzerGeneral{fn base(&self)->&AnalyzerBase{&self.base.base}fn base_mut(&mut self)->&mut AnalyzerBase{&mut self.base.base}fn filter(&mut self,t:&[TokenRef]){self.base.filter(t)}fn setContext(&mut self){self.base.setContext()}fn checkSyntax(&self)->bool{Self::checkSyntax(self)}fn dealThouSepToken(&mut self){self.base.dealThouSepToken()}}
