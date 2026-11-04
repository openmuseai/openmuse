use super::AnalyzerBase::{AnalyzerBase,AnalyzerDispatch};
use crate::cn::wps::kfc::numfmt::placeholder::{PlaceHolderToken::TokenRef,PlaceHolderID::PlaceHolderID,PlaceHolder::{ulen,usub}};
// java public class AnalyzerNumeric extends AnalyzerBase {
pub struct AnalyzerNumeric{pub base:AnalyzerBase}
impl AnalyzerNumeric{
    // java public AnalyzerNumeric(){super();}
    pub fn new()->Self{Self{base:AnalyzerBase::new()}}
    // java protected void filter(ArrayList<PlaceHolderToken> tokens) {
    pub fn filter(&mut self,tokens:&[TokenRef]){
        // java int lastDigitalPos=-1;boolean hasComma=false;int i=0;for(int count=tokens.size();i<count;++i){PlaceHolderToken token=tokens.get(i);if(this.isDigital(token)){lastDigitalPos=i;}if(token.getHolderID()==8){hasComma=true;}}
        let mut lastDigitalPos=-1;let mut hasComma=false;let mut i=0;let count=tokens.len();while i<count{let token=&tokens[i];if self.base.isDigital(token){lastDigitalPos=i as i32;}if token.lock().unwrap_or_else(std::sync::PoisonError::into_inner).getHolderID()==8{hasComma=true;}i+=1;}
        // java if(hasComma){i=0;for(int count=tokens.size();i<count;++i){PlaceHolderToken token=tokens.get(i);if(token.getHolderID()==8){
        if hasComma{i=0;let count=tokens.len();while i<count{let token=&tokens[i];let id=token.lock().unwrap_or_else(std::sync::PoisonError::into_inner).getHolderID();if id==8{
            // java if(i-1>=0){PlaceHolderToken preToken=tokens.get(i-1);if(!this.isDigitalAndPoint(preToken)){token.setPlaceHolder(PlaceHolderID.getPlaceHolder((short)0));token.mToken=token.mToken.substring(0,1);}else if(i+1<count){PlaceHolderToken nextToken=tokens.get(i+1);if(!this.isDigitalAndPoint(nextToken)&&i<lastDigitalPos){token.setPlaceHolder((PlaceHolder)null);}}}else{token.setPlaceHolder(PlaceHolderID.getPlaceHolder((short)0));token.mToken=token.mToken.substring(0,1);}
            if i>0{let preToken=&tokens[i-1];if !self.base.isDigitalAndPoint(preToken){let mut token=token.lock().unwrap_or_else(std::sync::PoisonError::into_inner);token.mPlaceHolder=Some(PlaceHolderID::getPlaceHolder(0));token.mToken=usub(&token.mToken,0,1);}else if i+1<count{let nextToken=&tokens[i+1];if !self.base.isDigitalAndPoint(nextToken)&&(i as i32)<lastDigitalPos{token.lock().unwrap_or_else(std::sync::PoisonError::into_inner).mPlaceHolder=None;}}}else{let mut token=token.lock().unwrap_or_else(std::sync::PoisonError::into_inner);token.mPlaceHolder=Some(PlaceHolderID::getPlaceHolder(0));token.mToken=usub(&token.mToken,0,1);}
        }i+=1;}}
    }
    // java protected void setContext() { for(int i=0,count=this.mTokens.size();i<count;++i){this.collectTokenInfo(this.mTokens.get(i),i);}this.mContext.leftDigiRemind=this.mContext.leftDigital;this.checkThouSep(); }
    pub fn setContext(&mut self){NumericDispatch::setContextNumeric(self)}
    // java protected void collectTokenInfo(PlaceHolderToken token,int tokenPos) {
    pub fn collectTokenInfo(&mut self,token:&TokenRef,tokenPos:i32){let token=token.lock().unwrap_or_else(std::sync::PoisonError::into_inner);let c=&mut self.base.mContext;match token.getHolderID(){
        // java case 2:case 3:case 4:if((tokenPos<this.mContext.fractionStartPos||this.mContext.fractionPos==-1)&&!this.mContext.hasDecimal){this.mContext.leftDigital+=token.mToken.length();if(this.mContext.firstIntPartPos==-1){this.mContext.firstIntPartPos=tokenPos;}this.mContext.lastIntPartPos=tokenPos;}else{if(this.mContext.firstDeciPartPos==-1){this.mContext.firstDeciPartPos=tokenPos;}this.mContext.lastDeciPartPos=tokenPos;}this.mContext.lastDigitalPos=tokenPos;case 5:case 7:default:break;
        2|3|4=>{if (tokenPos<c.fractionStartPos||c.fractionPos==-1)&&!c.hasDecimal{c.leftDigital=c.leftDigital.wrapping_add(ulen(&token.mToken)as i32);if c.firstIntPartPos==-1{c.firstIntPartPos=tokenPos;}c.lastIntPartPos=tokenPos;}else{if c.firstDeciPartPos==-1{c.firstDeciPartPos=tokenPos;}c.lastDeciPartPos=tokenPos;}c.lastDigitalPos=tokenPos;},
        // java case 6:if(this.mContext.percentPos!=-1){throw new AnalyzeException();}this.mContext.percentPos=tokenPos;break;
        6=>{assert!(c.percentPos==-1,"AnalyzeException");c.percentPos=tokenPos;},
        // java case 8:this.mContext.hasComma=true;break;
        8=>c.hasComma=true,
        // java case 9:if(!this.mContext.hasDecimal){this.mContext.firstDeciPointPos=tokenPos;}this.mContext.hasDecimal=true;
        9=>{if !c.hasDecimal{c.firstDeciPointPos=tokenPos;}c.hasDecimal=true;},_=>{}
    }}
    // java protected boolean checkSyntax(){return this.checkColon(this.mContext.lastDigitalPos);}
    pub fn checkSyntax(&self)->bool{self.checkColon(self.base.mContext.lastDigitalPos)}
    // java protected boolean checkColon(int stopPos){for(int i=0;i<stopPos;++i){PlaceHolderToken token=this.mTokens.get(i);if(token.getHolderID()==35){return false;}}return true;}
    pub fn checkColon(&self,stopPos:i32)->bool{for i in 0..stopPos{let token=self.base.mTokens[i as usize].lock().unwrap_or_else(std::sync::PoisonError::into_inner);if token.getHolderID()==35{return false;}}true}
    // java protected void dealThouSepToken() {
    pub fn dealThouSepToken(&mut self){NumericDispatch::dealThouSepTokenNumeric(self)}
    // java protected boolean decideThouSep(int thouSepPos){if(this.mContext.percentPos==-1){return true;}else{return this.mContext.percentPos>=this.mContext.lastDigitalPos||thouSepPos>=this.mContext.percentPos;}}
    pub fn decideThouSep(&self,thouSepPos:i32)->bool{let c=&self.base.mContext;if c.percentPos==-1{true}else{c.percentPos>=c.lastDigitalPos||thouSepPos>=c.percentPos}}
    // java protected void checkThouSep(){if(this.mContext.hasComma){for(int i=this.mContext.firstIntPartPos+1;i<this.mContext.lastIntPartPos;++i){PlaceHolderToken token=this.mTokens.get(i);if(token.getHolderID()==8){this.mContext.thouSepInIntPart=true;break;}}}}
    pub fn checkThouSep(&mut self){let c=&mut self.base.mContext;if c.hasComma{for i in c.firstIntPartPos+1..c.lastIntPartPos{let token=self.base.mTokens[i as usize].lock().unwrap_or_else(std::sync::PoisonError::into_inner);if token.getHolderID()==8{c.thouSepInIntPart=true;break;}}}}
}
// Native dispatch of the original overridable numeric hooks.
pub trait NumericDispatch{
    fn numeric(&self)->&AnalyzerNumeric;
    fn numeric_mut(&mut self)->&mut AnalyzerNumeric;
    fn collectTokenInfo(&mut self,t:&TokenRef,p:i32){self.numeric_mut().collectTokenInfo(t,p)}
    fn decideThouSep(&self,p:i32)->bool{self.numeric().decideThouSep(p)}
    fn setContextNumeric(&mut self){
        // java int i=0;for(int count=this.mTokens.size();i<count;++i){this.collectTokenInfo(this.mTokens.get(i),i);}
        let mut i=0;let count=self.numeric().base.mTokens.len();while i<count{let token=self.numeric().base.mTokens[i].clone();self.collectTokenInfo(&token,i as i32);i+=1;}
        // java this.mContext.leftDigiRemind=this.mContext.leftDigital;this.checkThouSep();
        let n=self.numeric_mut();n.base.mContext.leftDigiRemind=n.base.mContext.leftDigital;n.checkThouSep();
    }
    fn dealThouSepTokenNumeric(&mut self){
        // java if(this.mContext.hasComma){int startPos=this.mContext.lastIntPartPos!=-1?this.mContext.lastIntPartPos+1:this.mContext.firstDeciPointPos+1;
        let c=&self.numeric().base.mContext;if c.hasComma{let startPos=if c.lastIntPartPos!=-1{c.lastIntPartPos+1}else{c.firstDeciPointPos+1};
            // java for(int i=startPos;i<this.mTokens.size();++i){PlaceHolderToken token=this.mTokens.get(i);if(token.getHolderID()==8){
            for i in startPos..self.numeric().base.mTokens.len()as i32{let token=self.numeric().base.mTokens[i as usize].clone();let id=token.lock().unwrap_or_else(std::sync::PoisonError::into_inner).getHolderID();if id==8{
                // java if(i>this.mContext.lastDigitalPos){this.mContext.insertContent.append(token.mToken);}else{PlaceHolderToken nextToken=this.mTokens.get(i+1);if(nextToken.getHolderID()==9&&this.decideThouSep(i)){this.mContext.insertContent.append(token.mToken);}else{token.setPlaceHolder((PlaceHolder)null);}}
                if i>self.numeric().base.mContext.lastDigitalPos{let text=token.lock().unwrap_or_else(std::sync::PoisonError::into_inner).mToken.clone();self.numeric_mut().base.mContext.insertContent.push_str(&text);}else{let next=self.numeric().base.mTokens[i as usize+1].clone();let nextId=next.lock().unwrap_or_else(std::sync::PoisonError::into_inner).getHolderID();if nextId==9&&self.decideThouSep(i){let text=token.lock().unwrap_or_else(std::sync::PoisonError::into_inner).mToken.clone();self.numeric_mut().base.mContext.insertContent.push_str(&text);}else{token.lock().unwrap_or_else(std::sync::PoisonError::into_inner).mPlaceHolder=None;}}
            }}
        }
    }
}
impl NumericDispatch for AnalyzerNumeric{fn numeric(&self)->&AnalyzerNumeric{self}fn numeric_mut(&mut self)->&mut AnalyzerNumeric{self}}
impl AnalyzerDispatch for AnalyzerNumeric{fn base(&self)->&AnalyzerBase{&self.base}fn base_mut(&mut self)->&mut AnalyzerBase{&mut self.base}fn filter(&mut self,t:&[TokenRef]){Self::filter(self,t)}fn setContext(&mut self){Self::setContext(self)}fn checkSyntax(&self)->bool{Self::checkSyntax(self)}fn dealThouSepToken(&mut self){Self::dealThouSepToken(self)}}
