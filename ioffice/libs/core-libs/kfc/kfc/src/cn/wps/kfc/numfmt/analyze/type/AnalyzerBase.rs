use super::IAnalyzer::{IAnalyzer,AnalyzeResult};
use crate::cn::wps::kfc::numfmt::{analyze::AnalyzeContext::AnalyzeContext,placeholder::{PieceContext::PieceContext,PlaceHolderToken::{PlaceHolderToken,TokenRef}}};
// java public abstract class AnalyzerBase implements IAnalyzer {
pub struct AnalyzerBase{
    // java protected AnalyzeContext mContext = new AnalyzeContext();
    pub mContext:AnalyzeContext,
    // java protected ArrayList<PlaceHolderToken> mTokens = new ArrayList();
    pub mTokens:Vec<TokenRef>,
    // java protected StringBuffer mBuffer = new StringBuffer();
    pub mBuffer:String,
}
impl AnalyzerBase{
    // java public AnalyzerBase() { super(); }
    pub fn new()->Self{Self{mContext:AnalyzeContext::new(),mTokens:vec![],mBuffer:String::new()}}
    // java private void addTokens(ArrayList<PlaceHolderToken> tokens) {
    fn addTokens(&mut self,tokens:&[TokenRef]){
        // java int i=0;for(int initCount=tokens.size();i<initCount;++i){PlaceHolderToken token=tokens.get(i);if(!token.isEmpty()){this.mTokens.add(token);}else{token.recycle();}}
        let mut i=0;let initCount=tokens.len();while i<initCount{let token=&tokens[i];let empty=token.lock().unwrap_or_else(std::sync::PoisonError::into_inner).isEmpty();if !empty{self.mTokens.push(token.clone());}else{PlaceHolderToken::recycle(token);}i+=1;}
    }
    // java protected boolean formattedBracket(IAnalyzer.AnalyzeResult result) {
    pub fn formattedBracket(&mut self,result:&mut AnalyzeResult)->bool{
        // java boolean hasCondition=false;boolean hasText=false;PlaceHolderToken colorToken=null;PlaceHolderToken conditionToken=null;PlaceHolderToken dbnumToken=null;int i=0;
        let mut hasCondition=false;let mut hasText=false;let mut colorToken=None;let mut conditionToken=None;let mut dbnumToken=None;let mut i=0;
        // java for(int count=this.mTokens.size();i<count;++i){PlaceHolderToken token=this.mTokens.get(i);if(!token.isEmpty()){switch(token.getHolderID()){
        let count=self.mTokens.len();while i<count{let token=&self.mTokens[i];let t=token.lock().unwrap_or_else(std::sync::PoisonError::into_inner);if !t.isEmpty(){match t.getHolderID(){
            // java case 12:hasText=true;case 13:case 14:case 15:case 18:default:break;
            12=>hasText=true,
            // java case 16:colorToken=this.checkRepeat(colorToken,token);break;
            16=>colorToken=Some(self.checkRepeat(colorToken,token.clone())),
            // java case 17:hasCondition=true;conditionToken=this.checkRepeat(conditionToken,token);break;
            17=>{hasCondition=true;conditionToken=Some(self.checkRepeat(conditionToken,token.clone()));},
            // java case 19:dbnumToken=this.checkRepeat(dbnumToken,token);
            19=>dbnumToken=Some(self.checkRepeat(dbnumToken,token.clone())),_=>{}
        }}i+=1;}
        // java if(colorToken!=null){colorToken.doMutexHolder().compile(result.mFormatCode,this.mContext);colorToken.setPlaceHolder((PlaceHolder)null);}
        if let Some(colorToken)=colorToken{let mut t=colorToken.lock().unwrap_or_else(std::sync::PoisonError::into_inner);t.doMutexHolder().compile_value(&mut result.mFormatCode,&mut self.mContext);t.mPlaceHolder=None;}
        // java if(conditionToken!=null){if(result.mCurSectIndex>=2||hasText){throw new AnalyzeException();}conditionToken.doMutexHolder().compile(result.mFormatCode,this.mContext);conditionToken.setPlaceHolder((PlaceHolder)null);}
        if let Some(conditionToken)=conditionToken{assert!(result.mCurSectIndex<2&&!hasText,"AnalyzeException");let mut t=conditionToken.lock().unwrap_or_else(std::sync::PoisonError::into_inner);t.doMutexHolder().compile_value(&mut result.mFormatCode,&mut self.mContext);t.mPlaceHolder=None;}
        // java if(dbnumToken!=null){dbnumToken.doMutexHolder().compile(result.mFormatCode,this.mContext);dbnumToken.setPlaceHolder((PlaceHolder)null);}
        if let Some(dbnumToken)=dbnumToken{let mut t=dbnumToken.lock().unwrap_or_else(std::sync::PoisonError::into_inner);t.doMutexHolder().compile_value(&mut result.mFormatCode,&mut self.mContext);t.mPlaceHolder=None;}
        // java return hasCondition;
        hasCondition
    }
    // java private PlaceHolderToken checkRepeat(PlaceHolderToken dest,PlaceHolderToken src){if(dest!=null){throw new AnalyzeException("repeat the same bracket type");}else{return src;}}
    fn checkRepeat(&self,dest:Option<TokenRef>,src:TokenRef)->TokenRef{assert!(dest.is_none(),"AnalyzeException: repeat the same bracket type");src}
    // java protected final boolean isDigitalPlaceHolder(PlaceHolderToken token){switch(token.getHolderID()){case 2:case 3:case 4:return true;default:return false;}}
    pub fn isDigitalPlaceHolder(&self,token:&TokenRef)->bool{matches!(token.lock().unwrap_or_else(std::sync::PoisonError::into_inner).getHolderID(),2|3|4)}
    // java protected final boolean isDigital(PlaceHolderToken token){return this.isDigitalPlaceHolder(token)?true:token.getHolderID()==15;}
    pub fn isDigital(&self,token:&TokenRef)->bool{self.isDigitalPlaceHolder(token)||token.lock().unwrap_or_else(std::sync::PoisonError::into_inner).getHolderID()==15}
    // java protected final boolean isDigitalAndPoint(PlaceHolderToken token){return this.isDigital(token)?true:token.getHolderID()==9;}
    pub fn isDigitalAndPoint(&self,token:&TokenRef)->bool{self.isDigital(token)||token.lock().unwrap_or_else(std::sync::PoisonError::into_inner).getHolderID()==9}
}
// Native inherited virtual dispatch; the hooks below correspond to the four
// original abstract protected methods, preserving subclass call order.
pub trait AnalyzerDispatch{
    fn base(&self)->&AnalyzerBase;
    fn base_mut(&mut self)->&mut AnalyzerBase;
    // java protected abstract void filter(ArrayList<PlaceHolderToken> var1);
    fn filter(&mut self,tokens:&[TokenRef]);
    // java protected abstract void setContext() throws AnalyzeException;
    fn setContext(&mut self);
    // java protected abstract boolean checkSyntax();
    fn checkSyntax(&self)->bool;
    // java protected abstract void dealThouSepToken();
    fn dealThouSepToken(&mut self);
    // java protected void collectInfo(ArrayList<PlaceHolderToken> tokens){this.filter(tokens);this.addTokens(tokens);this.setContext();}
    fn collectInfo(&mut self,tokens:&[TokenRef]){self.filter(tokens);self.base_mut().addTokens(tokens);self.setContext();}
}
impl<T:AnalyzerDispatch> IAnalyzer for T{
    // java public void reset(ArrayList<PlaceHolderToken> tokens,PieceContext context,short type){this.mContext.clear();this.mTokens.clear();this.mBuffer.setLength(0);this.mContext.type=type;this.collectInfo(tokens);}
    fn reset(&mut self,tokens:&[TokenRef],_context:&PieceContext,r#type:i16){let b=self.base_mut();b.mContext.clear();b.mTokens.clear();b.mBuffer.clear();b.mContext.r#type=r#type;self.collectInfo(tokens);}
    // java public boolean analyze(IAnalyzer.AnalyzeResult result) {
    fn analyze(&mut self,result:&mut AnalyzeResult)->bool{
        // java this.mBuffer.setLength(0);if(!this.checkSyntax()){throw new AnalyzeException();}else{
        self.base_mut().mBuffer.clear();assert!(self.checkSyntax(),"AnalyzeException");
        // java this.dealThouSepToken();boolean hasCondition=this.formattedBracket(result);PlaceHolderToken token=null;
        self.dealThouSepToken();let hasCondition=self.base_mut().formattedBracket(result);
        // java for(int i=0;i<this.mTokens.size();++i){token=this.mTokens.get(i);if(!token.isEmpty()){this.mContext.curPos=i;token.doMutexHolder().compile(this.mBuffer,this.mContext);}}
        for i in 0..self.base().mTokens.len(){let token=self.base().mTokens[i].clone();let t=token.lock().unwrap_or_else(std::sync::PoisonError::into_inner);if !t.isEmpty(){let b=self.base_mut();b.mContext.curPos=i as i32;t.doMutexHolder().compile_value(&mut b.mBuffer,&mut b.mContext);}}
        // java if(this.mBuffer.length()>0){result.mFormatCode.append(this.mBuffer);if(hasCondition&&!this.mBuffer.toString().equals("General")){result.mAddGeneralSection=true;}else{result.mAddGeneralSection=false;}}else{result.mFormatCode.append("General");}return true;
        if !self.base().mBuffer.is_empty(){result.mFormatCode.push_str(&self.base().mBuffer);if hasCondition&&self.base().mBuffer!="General"{result.mAddGeneralSection=true;}else{result.mAddGeneralSection=false;}}else{result.mFormatCode.push_str("General");}true
    }
}
