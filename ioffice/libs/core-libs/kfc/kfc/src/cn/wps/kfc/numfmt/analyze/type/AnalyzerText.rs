use super::{AnalyzerNonNumeric::AnalyzerNonNumeric,AnalyzerBase::{AnalyzerBase,AnalyzerDispatch}};
use crate::cn::wps::kfc::numfmt::placeholder::PlaceHolderToken::TokenRef;
// java public class AnalyzerText extends AnalyzerNonNumeric {
pub struct AnalyzerText{pub base:AnalyzerNonNumeric}
impl AnalyzerText{
    // java public AnalyzerText(){super();}
    pub fn new()->Self{Self{base:AnalyzerNonNumeric::new()}}
    // java protected boolean checkSyntax(){boolean hasPlainDigtal=false;boolean hasText=false;int i=0;for(int count=this.mTokens.size();i<count;++i){PlaceHolderToken token=this.mTokens.get(i);if(!token.isEmpty()){if(token.getHolderID()==15){hasPlainDigtal=true;}if(token.getHolderID()==12){hasText=true;}}}return !hasPlainDigtal||!hasText;}
    pub fn checkSyntax(&self)->bool{let mut hasPlainDigtal=false;let mut hasText=false;let mut i=0;let count=self.base.base.mTokens.len();while i<count{let token=self.base.base.mTokens[i].lock().unwrap_or_else(std::sync::PoisonError::into_inner);if !token.isEmpty(){if token.getHolderID()==15{hasPlainDigtal=true;}if token.getHolderID()==12{hasText=true;}}i+=1;}!hasPlainDigtal||!hasText}
}
impl AnalyzerDispatch for AnalyzerText{fn base(&self)->&AnalyzerBase{&self.base.base}fn base_mut(&mut self)->&mut AnalyzerBase{&mut self.base.base}fn filter(&mut self,t:&[TokenRef]){self.base.filter(t)}fn setContext(&mut self){self.base.setContext()}fn checkSyntax(&self)->bool{Self::checkSyntax(self)}fn dealThouSepToken(&mut self){self.base.dealThouSepToken()}}
