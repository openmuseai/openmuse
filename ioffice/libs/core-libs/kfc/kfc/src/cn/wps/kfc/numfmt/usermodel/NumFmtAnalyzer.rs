use std::{rc::Rc,cell::RefCell,panic::{catch_unwind,AssertUnwindSafe,resume_unwind}};
use crate::cn::wps::kfc::numfmt::{analyze::{AnalyzerFactory::AnalyzerFactory,r#type::IAnalyzer::AnalyzeResult},format::NumFmtSection::NumFmtSection,placeholder::{PieceContext::PieceContext,PlaceHolderToken::{PlaceHolderToken,TokenRef},VariantPieceDic::VariantPieceDic}};
// java public class NumFmtAnalyzer {
pub struct NumFmtAnalyzer{
    // java private ArrayList<PlaceHolderToken> mCompileHolders=new ArrayList();
    mCompileHolders:Vec<TokenRef>,
    // java private PieceContext mContext=new PieceContext();
    mContext:PieceContext,
    // java private IAnalyzer.AnalyzeResult mAnalyzeResult=new IAnalyzer.AnalyzeResult();
    pub(crate) mAnalyzeResult:Rc<RefCell<AnalyzeResult>>,
    // java private String[] mSectionGroups=new String[4];
    mSectionGroups:[Option<String>;4],
    // java private AnalyzerFactory factory=new AnalyzerFactory();
    factory:AnalyzerFactory,
    // java private VariantPieceDic mTokenDic=new VariantPieceDic();
    mTokenDic:VariantPieceDic,
}
impl NumFmtAnalyzer{
    // java public NumFmtAnalyzer(){super();}
    pub fn new()->Self{Self{mCompileHolders:vec![],mContext:PieceContext::new(),mAnalyzeResult:Rc::new(RefCell::new(AnalyzeResult::new())),mSectionGroups:std::array::from_fn(|_|None),factory:AnalyzerFactory::new(),mTokenDic:VariantPieceDic::new()}}
    // java public void resetDic(short lcid){this.mTokenDic.reset(lcid);}
    pub fn resetDic(&mut self,lcid:i16){self.mTokenDic.reset(lcid);}
    // java public synchronized String analyse(String formatCode) {
    pub fn analyse(&mut self,formatCode:&str)->Option<String>{
        // java if(formatCode!=null&&formatCode.length()!=0){this.mAnalyzeResult.mFormatCode.setLength(0);short sectCount=NumFmtSection.splitGroup(formatCode,this.mSectionGroups);
        if !formatCode.is_empty(){self.mAnalyzeResult.borrow_mut().mFormatCode.clear();let sectCount=NumFmtSection::splitGroup(formatCode,&mut self.mSectionGroups);
            // java if(sectCount>4){return null;}else{for(short i=0;i<sectCount;++i){this.mAnalyzeResult.mCurSectIndex=i;if(i>0){this.mAnalyzeResult.mFormatCode.append(";");}
            if sectCount>4{return None;}for i in 0..sectCount{self.mAnalyzeResult.borrow_mut().mCurSectIndex=i;if i>0{self.mAnalyzeResult.borrow_mut().mFormatCode.push(';');}
                // java try{String formatSection=this.mSectionGroups[i];if(formatSection.length()>0){this.analyzeSection(formatSection,this.mAnalyzeResult,i,sectCount);}}catch(AnalyzeException var5){this.clear();return null;}
                let result=catch_unwind(AssertUnwindSafe(||{let formatSection=self.mSectionGroups[i as usize].clone().expect("NullPointerException");if !formatSection.is_empty(){self.analyzeSection(&formatSection,self.mAnalyzeResult.clone(),i,sectCount);}}));if let Err(error)=result{let message=error.downcast_ref::<String>().map(String::as_str).or_else(||error.downcast_ref::<&str>().copied()).unwrap_or("");if message.starts_with("AnalyzeException"){self.clear();return None;}resume_unwind(error);}
            }
            // java if(sectCount==1&&this.mAnalyzeResult.mAddGeneralSection){this.mAnalyzeResult.mFormatCode.append(';').append("General");}return this.mAnalyzeResult.mFormatCode.toString();
            if sectCount==1&&self.mAnalyzeResult.borrow().mAddGeneralSection{self.mAnalyzeResult.borrow_mut().mFormatCode.push_str(";General");}Some(self.mAnalyzeResult.borrow().mFormatCode.clone())
        // java }else{return "General";}
        }else{Some("General".into())}
    }
    // java protected void clear(){int i=0;for(int size=this.mCompileHolders.size();i<size;++i){this.mCompileHolders.get(i).recycle();}this.mCompileHolders.clear();this.mContext.clear();this.mAnalyzeResult.mAddGeneralSection=false;}
    pub fn clear(&mut self){let mut i=0;let size=self.mCompileHolders.len();while i<size{PlaceHolderToken::recycle(&self.mCompileHolders[i]);i+=1;}self.mCompileHolders.clear();self.mContext.clear();self.mAnalyzeResult.borrow_mut().mAddGeneralSection=false;}
    // java protected boolean analyzeSection(String formatSection,IAnalyzer.AnalyzeResult result,short sectIndex,short sectCount){this.clear();this.collectPieces(formatSection);if(!this.checkType(this.mContext,sectIndex,sectCount)){throw new AnalyzeException();}else{short type=this.getMainType(this.mContext);IAnalyzer analyzer=this.factory.getAnalyzer(type,this.mCompileHolders,this.mContext);analyzer.analyze(result);return true;}}
    pub fn analyzeSection(&mut self,formatSection:&str,result:Rc<RefCell<AnalyzeResult>>,sectIndex:i16,sectCount:i16)->bool{self.clear();self.collectPieces(formatSection);assert!(self.checkType(&self.mContext,sectIndex,sectCount),"AnalyzeException");let r#type=self.getMainType(&self.mContext);let analyzer=self.factory.getAnalyzer(r#type,&self.mCompileHolders,&self.mContext);analyzer.analyze(&mut result.borrow_mut());true}
    // java private void collectPieces(String formatPart){int i=0;for(int length=formatPart.length();i<length;++i){PlaceHolderPiece piece=this.mTokenDic.searchPiece(formatPart.charAt(i),formatPart,i);int len=piece.analyze(formatPart,i,length,this.mContext);i+=len-1;if(piece.mHolderToken!=null){this.mCompileHolders.add(piece.mHolderToken);}}}
    fn collectPieces(&mut self,formatPart:&str){let units:Vec<_>=formatPart.encode_utf16().collect();let mut i=0i32;let length=units.len()as i32;while i<length{let piece=self.mTokenDic.searchPiece(units[i as usize],&units,i as usize);let mut piece=piece.borrow_mut();let len=piece.analyze(&units,i as usize,length as usize,&mut self.mContext);i+=len-1;if let Some(token)=piece.base().mHolderToken.clone(){self.mCompileHolders.push(token);}i+=1;}}
    // java private boolean checkType(PieceContext context,short sectIndex,short sectCount) {
    fn checkType(&self,context:&PieceContext,sectIndex:i16,sectCount:i16)->bool{
        // java short type=context.mPieceType;short typeCount=0;if((type&512)<=0||(type&8192)<=0&&!context.mHasTextPlaceHolder&&(type&1)<=0){
        let r#type=context.mPieceType;let mut typeCount=0;if (r#type&512)<=0||(r#type&8192)<=0&&!context.mHasTextPlaceHolder&&(r#type&1)<=0{
            // java if(context.mHasTextPlaceHolder&&(type&1)>0){return false;}else if(context.mHasTextPlaceHolder&&sectIndex!=sectCount-1){return false;}else{
            if context.mHasTextPlaceHolder&&(r#type&1)>0{false}else if context.mHasTextPlaceHolder&&sectIndex!=sectCount-1{false}else{
                // java if((type&8192)>0){++typeCount;}if(context.mHasTextPlaceHolder){++typeCount;}if((type&512)>0&&(type&8)>0){++typeCount;}if((type&16)>0){++typeCount;}return typeCount<=1;
                if (r#type&8192)>0{typeCount+=1;}if context.mHasTextPlaceHolder{typeCount+=1;}if (r#type&512)>0&&(r#type&8)>0{typeCount+=1;}if (r#type&16)>0{typeCount+=1;}typeCount<=1
            }
        // java }else{return false;}
        }else{false}
    }
    // java private short getMainType(PieceContext context){short type=context.mPieceType;if((type&8192)>0){return 8192;}else if((type&8)>0){return 8;}else if((type&16)>0){return 16;}else if((type&512)>0){return 512;}else if((type&1)>0){return 1;}else{return (short)((type&64)<=0&&(type&128)<=0?0:64);}}
    fn getMainType(&self,context:&PieceContext)->i16{let r#type=context.mPieceType;if (r#type&8192)>0{8192}else if (r#type&8)>0{8}else if (r#type&16)>0{16}else if (r#type&512)>0{512}else if (r#type&1)>0{1}else if (r#type&64)<=0&&(r#type&128)<=0{0}else{64}}
}
