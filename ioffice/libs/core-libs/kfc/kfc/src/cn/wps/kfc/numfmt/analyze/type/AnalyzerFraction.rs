use super::{AnalyzerNumeric::{AnalyzerNumeric,NumericDispatch},AnalyzerBase::{AnalyzerBase,AnalyzerDispatch}};
use crate::cn::wps::kfc::numfmt::placeholder::{PlaceHolderToken::TokenRef,PlaceHolder::ulen};
// java public class AnalyzerFraction extends AnalyzerNumeric {
pub struct AnalyzerFraction{pub base:AnalyzerNumeric}
impl AnalyzerFraction{
    // java public AnalyzerFraction(){super();}
    pub fn new()->Self{Self{base:AnalyzerNumeric::new()}}
    // java private boolean checkFractionPart(){return this.mContext.fractionStartPos!=-1&&this.mContext.fractionEndPos!=-1;}
    fn checkFractionPart(&self)->bool{self.base.base.mContext.fractionStartPos!=-1&&self.base.base.mContext.fractionEndPos!=-1}
    // java private boolean checkFractionDecimal(){if(!this.mContext.hasDecimal){return true;}else{if(this.mContext.firstDeciPointPos<this.mContext.fractionStartPos){for(int i=this.mContext.firstDeciPointPos+1;i<this.mContext.fractionStartPos;++i){PlaceHolderToken token=this.mTokens.get(i);if(this.isDigitalPlaceHolder(token)){return false;}}}return true;}}
    fn checkFractionDecimal(&self)->bool{let c=&self.base.base.mContext;if !c.hasDecimal{true}else{if c.firstDeciPointPos<c.fractionStartPos{for i in c.firstDeciPointPos+1..c.fractionStartPos{let token=&self.base.base.mTokens[i as usize];if self.base.base.isDigitalPlaceHolder(token){return false;}}}true}}
    // java protected boolean checkSyntax(){if(this.mContext.fractionPos==-1){return false;}else if(!this.checkColon(this.mContext.lastDigitalPos)){return false;}else{return this.checkFractionPart()&&this.checkFractionDecimal();}}
    pub fn checkSyntax(&self)->bool{if self.base.base.mContext.fractionPos==-1{false}else if !self.base.checkColon(self.base.base.mContext.lastDigitalPos){false}else{self.checkFractionPart()&&self.checkFractionDecimal()}}
    // java protected void setContext(){this.collectFractionOrPercent();super.setContext();}
    pub fn setContext(&mut self){self.collectFractionOrPercent();NumericDispatch::setContextNumeric(self);}
    // java protected boolean decideThouSep(int i){int pos=this.mContext.fractionPos>this.mContext.percentPos?this.mContext.fractionPos:this.mContext.percentPos;return i>pos;}
    pub fn decideThouSep(&self,i:i32)->bool{let c=&self.base.base.mContext;let pos=if c.fractionPos>c.percentPos{c.fractionPos}else{c.percentPos};i>pos}
    // java private void collectFractionOrPercent() {
    fn collectFractionOrPercent(&mut self){
        // java int index=0;for(int count=this.mTokens.size();index<count;++index){PlaceHolderToken token=this.mTokens.get(index);short id=token.getHolderID();if(id==5){if(this.mContext.fractionPos!=-1||token.mToken.length()>1){throw new AnalyzeException();}this.mContext.fractionPos=index;}}
        let mut index=0;let count=self.base.base.mTokens.len();while index<count{let token=self.base.base.mTokens[index].lock().unwrap_or_else(std::sync::PoisonError::into_inner);let id=token.getHolderID();if id==5{assert!(self.base.base.mContext.fractionPos==-1&&ulen(&token.mToken)<=1,"AnalyzeException");self.base.base.mContext.fractionPos=index as i32;}index+=1;}
        // java if(this.mContext.fractionPos!=-1){this.collectNumerator();this.collectDenominator();}
        if self.base.base.mContext.fractionPos!=-1{self.collectNumerator();self.collectDenominator();}
    }
    // java private void collectNumerator() {
    fn collectNumerator(&mut self){
        // java int temp=this.mContext.fractionPos;while(true){--temp;if(temp<0){break;}PlaceHolderToken token=this.mTokens.get(temp);if(this.isDigital(token)){this.mContext.fractionStartPos=temp;}else if(this.mContext.fractionStartPos!=-1){break;}}
        let mut temp=self.base.base.mContext.fractionPos;loop{temp-=1;if temp<0{break;}let token=&self.base.base.mTokens[temp as usize];if self.base.base.isDigital(token){self.base.base.mContext.fractionStartPos=temp;}else if self.base.base.mContext.fractionStartPos!=-1{break;}}
        // java if(this.mContext.fractionStartPos==-1){throw new AnalyzeException();}else{if(this.mContext.fractionStartPos-2>=0){PlaceHolderToken token=this.mTokens.get(this.mContext.fractionStartPos-1);if(token.getHolderID()==9||token.getHolderID()==8){token=this.mTokens.get(this.mContext.fractionStartPos-2);if(this.isDigital(token)){throw new AnalyzeException();}}}}
        let c=&self.base.base.mContext;assert!(c.fractionStartPos!=-1,"AnalyzeException");if c.fractionStartPos-2>=0{let token=&self.base.base.mTokens[(c.fractionStartPos-1)as usize];let id=token.lock().unwrap_or_else(std::sync::PoisonError::into_inner).getHolderID();if id==9||id==8{let token=&self.base.base.mTokens[(c.fractionStartPos-2)as usize];assert!(!self.base.base.isDigital(token),"AnalyzeException");}}
    }
    // java private void collectDenominator() {
    fn collectDenominator(&mut self){
        // java int temp=this.mContext.fractionPos;int count=this.mTokens.size();while(true){++temp;if(temp>=count){break;}PlaceHolderToken token=this.mTokens.get(temp);if(this.isDigital(token)){this.mContext.fractionEndPos=temp;}else if(this.mContext.fractionEndPos!=-1){break;}}
        let mut temp=self.base.base.mContext.fractionPos;let count=self.base.base.mTokens.len()as i32;loop{temp+=1;if temp>=count{break;}let token=&self.base.base.mTokens[temp as usize];if self.base.base.isDigital(token){self.base.base.mContext.fractionEndPos=temp;}else if self.base.base.mContext.fractionEndPos!=-1{break;}}
        // java if(this.mContext.fractionEndPos==-1){throw new AnalyzeException();}else{byte[] tempArray=new byte[2];for(int i=this.mContext.fractionPos+1;i<=this.mContext.fractionEndPos;++i){PlaceHolderToken token=this.mTokens.get(i);short id=token.getHolderID();if(id==2||id==4){tempArray[0]=1;}if(id==15){tempArray[1]=1;}}
        let c=&self.base.base.mContext;assert!(c.fractionEndPos!=-1,"AnalyzeException");let mut tempArray=[0i8;2];for i in c.fractionPos+1..=c.fractionEndPos{let token=self.base.base.mTokens[i as usize].lock().unwrap_or_else(std::sync::PoisonError::into_inner);let id=token.getHolderID();if id==2||id==4{tempArray[0]=1;}if id==15{tempArray[1]=1;}}
        // java int total=tempArray[0]+tempArray[1];byte[] var8=null;if(total>1){throw new AnalyzeException();}else{if(this.mContext.fractionEndPos+2<this.mTokens.size()){PlaceHolderToken token=this.mTokens.get(this.mContext.fractionEndPos+1);if(token.getHolderID()==9||token.getHolderID()==8){token=this.mTokens.get(this.mContext.fractionEndPos+2);if(this.isDigital(token)){throw new AnalyzeException();}}}}
        let total=tempArray[0]+tempArray[1];assert!(total<=1,"AnalyzeException");if c.fractionEndPos+2<(self.base.base.mTokens.len()as i32){let token=&self.base.base.mTokens[(c.fractionEndPos+1)as usize];let id=token.lock().unwrap_or_else(std::sync::PoisonError::into_inner).getHolderID();if id==9||id==8{let token=&self.base.base.mTokens[(c.fractionEndPos+2)as usize];assert!(!self.base.base.isDigital(token),"AnalyzeException");}}
    }
}
impl NumericDispatch for AnalyzerFraction{fn numeric(&self)->&AnalyzerNumeric{&self.base}fn numeric_mut(&mut self)->&mut AnalyzerNumeric{&mut self.base}fn decideThouSep(&self,p:i32)->bool{Self::decideThouSep(self,p)}}
impl AnalyzerDispatch for AnalyzerFraction{fn base(&self)->&AnalyzerBase{&self.base.base}fn base_mut(&mut self)->&mut AnalyzerBase{&mut self.base.base}fn filter(&mut self,t:&[TokenRef]){self.base.filter(t)}fn setContext(&mut self){Self::setContext(self)}fn checkSyntax(&self)->bool{Self::checkSyntax(self)}fn dealThouSepToken(&mut self){self.dealThouSepTokenNumeric()}}
