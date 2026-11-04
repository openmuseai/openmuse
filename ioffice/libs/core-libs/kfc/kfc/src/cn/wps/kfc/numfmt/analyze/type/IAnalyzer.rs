use crate::cn::wps::kfc::numfmt::placeholder::{PieceContext::PieceContext,PlaceHolderToken::TokenRef};
// java public interface IAnalyzer {
pub trait IAnalyzer{
    // java void reset(ArrayList<PlaceHolderToken> var1,PieceContext var2,short var3);
    fn reset(&mut self,var1:&[TokenRef],var2:&PieceContext,var3:i16);
    // java boolean analyze(AnalyzeResult var1);
    fn analyze(&mut self,var1:&mut AnalyzeResult)->bool;
}
// java public static class AnalyzeResult {
pub struct AnalyzeResult{
    // java public StringBuffer mFormatCode = new StringBuffer();
    pub mFormatCode:String,
    // java public boolean mAddGeneralSection = false;
    pub mAddGeneralSection:bool,
    // java public short mCurSectIndex;
    pub mCurSectIndex:i16,
}
impl AnalyzeResult{
    // java public AnalyzeResult() { super(); }
    pub fn new()->Self{Self{mFormatCode:String::new(),mAddGeneralSection:false,mCurSectIndex:0}}
}
