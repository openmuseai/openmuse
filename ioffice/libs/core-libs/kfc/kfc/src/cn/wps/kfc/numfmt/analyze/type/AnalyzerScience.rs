use super::{AnalyzerNumeric::{AnalyzerNumeric,NumericDispatch},AnalyzerBase::{AnalyzerBase,AnalyzerDispatch}};
use crate::cn::wps::kfc::numfmt::placeholder::{PlaceHolderToken::TokenRef,PlaceHolder::ulen};
// java public class AnalyzerScience extends AnalyzerNumeric {
pub struct AnalyzerScience{pub base:AnalyzerNumeric}
impl AnalyzerScience{
    // java public AnalyzerScience(){super();}
    pub fn new()->Self{Self{base:AnalyzerNumeric::new()}}
    // java protected void collectTokenInfo(PlaceHolderToken token,int tokenPos) {
    pub fn collectTokenInfo(&mut self,token:&TokenRef,tokenPos:i32){let token=token.lock().unwrap_or_else(std::sync::PoisonError::into_inner);let c=&mut self.base.base.mContext;match token.getHolderID(){
        // java case 2:case 3:case 4:if(this.mContext.firstDeciPointPos==-1&&this.mContext.scienceEPos==-1){this.mContext.leftDigital+=token.mToken.length();if(this.mContext.firstIntPartPos==-1){this.mContext.firstIntPartPos=tokenPos;}this.mContext.lastIntPartPos=tokenPos;}else if(this.mContext.scienceEPos!=-1){if(this.mContext.firstDigitAfterE==-1){this.mContext.firstDigitAfterE=tokenPos;}this.mContext.lastDigitAfterE=tokenPos;}if(this.mContext.scienceEPos==-1){this.mContext.lastDigitBeforeE=tokenPos;}this.mContext.lastDigitalPos=tokenPos;case 5:case 6:default:break;
        2|3|4=>{if c.firstDeciPointPos==-1&&c.scienceEPos==-1{c.leftDigital=c.leftDigital.wrapping_add(ulen(&token.mToken)as i32);if c.firstIntPartPos==-1{c.firstIntPartPos=tokenPos;}c.lastIntPartPos=tokenPos;}else if c.scienceEPos!=-1{if c.firstDigitAfterE==-1{c.firstDigitAfterE=tokenPos;}c.lastDigitAfterE=tokenPos;}if c.scienceEPos==-1{c.lastDigitBeforeE=tokenPos;}c.lastDigitalPos=tokenPos;},
        // java case 7:if(this.mContext.scienceEPos!=-1){throw new AnalyzeException("more than one E+");}this.mContext.scienceEPos=tokenPos;break;
        7=>{assert!(c.scienceEPos==-1,"AnalyzeException: more than one E+");c.scienceEPos=tokenPos;},
        // java case 8:this.mContext.hasComma=true;break;
        8=>c.hasComma=true,
        // java case 9:if(this.mContext.scienceEPos==-1){if(this.mContext.firstDeciPointPos==-1){this.mContext.firstDeciPointPos=tokenPos;}}else if(this.mContext.secondDeciPointPos==-1){this.mContext.secondDeciPointPos=tokenPos;}
        9=>{if c.scienceEPos==-1{if c.firstDeciPointPos==-1{c.firstDeciPointPos=tokenPos;}}else if c.secondDeciPointPos==-1{c.secondDeciPointPos=tokenPos;}},_=>{}
    }}
    // java protected boolean checkSyntax() {
    pub fn checkSyntax(&self)->bool{
        // java boolean hasDigBeforE=this.mContext.firstIntPartPos!=-1||this.mContext.firstDeciPointPos!=-1;if(!hasDigBeforE){return false;}else{
        let c=&self.base.base.mContext;let hasDigBeforE=c.firstIntPartPos!=-1||c.firstDeciPointPos!=-1;if !hasDigBeforE{return false;}
        // java boolean hasDigAfterE=this.mContext.firstDigitAfterE!=-1||this.mContext.secondDeciPointPos!=-1;if(!hasDigAfterE){return false;}else if(!this.checkColon(this.mContext.lastDigitAfterE)){return false;}else{
        let hasDigAfterE=c.firstDigitAfterE!=-1||c.secondDeciPointPos!=-1;if !hasDigAfterE{return false;}else if !self.base.checkColon(c.lastDigitAfterE){return false;}
        // java boolean hasDeciBeforeE=this.mContext.firstDeciPointPos!=-1;if(hasDeciBeforeE){return true;}else{for(int i=this.mContext.firstDigitAfterE+1;i<this.mContext.lastDigitAfterE;++i){PlaceHolderToken token=this.mTokens.get(i);if(token.getHolderID()==8&&this.mTokens.get(i+1).getHolderID()!=9){return false;}}return true;}
        let hasDeciBeforeE=c.firstDeciPointPos!=-1;if hasDeciBeforeE{true}else{for i in c.firstDigitAfterE+1..c.lastDigitAfterE{let id=self.base.base.mTokens[i as usize].lock().unwrap_or_else(std::sync::PoisonError::into_inner).getHolderID();if id==8&&self.base.base.mTokens[i as usize+1].lock().unwrap_or_else(std::sync::PoisonError::into_inner).getHolderID()!=9{return false;}}true}
    }
    // java protected boolean decideThouSep(int thouSepPos){return thouSepPos>this.mContext.scienceEPos;}
    pub fn decideThouSep(&self,thouSepPos:i32)->bool{thouSepPos>self.base.base.mContext.scienceEPos}
}
impl NumericDispatch for AnalyzerScience{fn numeric(&self)->&AnalyzerNumeric{&self.base}fn numeric_mut(&mut self)->&mut AnalyzerNumeric{&mut self.base}fn collectTokenInfo(&mut self,t:&TokenRef,p:i32){Self::collectTokenInfo(self,t,p)}fn decideThouSep(&self,p:i32)->bool{Self::decideThouSep(self,p)}}
impl AnalyzerDispatch for AnalyzerScience{fn base(&self)->&AnalyzerBase{&self.base.base}fn base_mut(&mut self)->&mut AnalyzerBase{&mut self.base.base}fn filter(&mut self,t:&[TokenRef]){self.base.filter(t)}fn setContext(&mut self){self.setContextNumeric()}fn checkSyntax(&self)->bool{Self::checkSyntax(self)}fn dealThouSepToken(&mut self){self.dealThouSepTokenNumeric()}}
