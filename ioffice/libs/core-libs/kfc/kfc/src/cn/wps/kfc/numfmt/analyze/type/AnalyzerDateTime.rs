use super::{AnalyzerNonNumeric::AnalyzerNonNumeric,AnalyzerBase::{AnalyzerBase,AnalyzerDispatch}};
use crate::cn::wps::kfc::numfmt::placeholder::PlaceHolderToken::TokenRef;
// java public class AnalyzerDateTime extends AnalyzerNonNumeric {
pub struct AnalyzerDateTime{pub base:AnalyzerNonNumeric}
impl AnalyzerDateTime{
    // java public AnalyzerDateTime(){super();}
    pub fn new()->Self{Self{base:AnalyzerNonNumeric::new()}}
    // java protected boolean checkSyntax(){int countElapseTime=0;int i=0;for(int len=this.mTokens.size();i<len;++i){if(this.isElapseTimeToken(this.mTokens.get(i))){++countElapseTime;if(countElapseTime>1){return false;}}}return true;}
    pub fn checkSyntax(&self)->bool{let mut countElapseTime=0;let mut i=0;let len=self.base.base.mTokens.len();while i<len{if self.isElapseTimeToken(&self.base.base.mTokens[i]){countElapseTime+=1;if countElapseTime>1{return false;}}i+=1;}true}
    // java private final boolean isElapseTimeToken(PlaceHolderToken token){switch(token.getHolderID()){case 29:case 30:case 31:return true;default:return false;}}
    fn isElapseTimeToken(&self,token:&TokenRef)->bool{matches!(token.lock().unwrap_or_else(std::sync::PoisonError::into_inner).getHolderID(),29|30|31)}
}
impl AnalyzerDispatch for AnalyzerDateTime{fn base(&self)->&AnalyzerBase{&self.base.base}fn base_mut(&mut self)->&mut AnalyzerBase{&mut self.base.base}fn filter(&mut self,t:&[TokenRef]){self.base.filter(t)}fn setContext(&mut self){self.base.setContext()}fn checkSyntax(&self)->bool{Self::checkSyntax(self)}fn dealThouSepToken(&mut self){self.base.dealThouSepToken()}}
