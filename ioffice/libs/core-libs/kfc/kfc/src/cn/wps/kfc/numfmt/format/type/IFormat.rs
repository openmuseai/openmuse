use crate::cn::wps::kfc::numfmt::{placeholder::{PieceContext::PieceContext,PlaceHolderToken::TokenRef},format::{NumFmtParam::NumFmtParam,NumFmtResult::NumFmtResult}};
use super::NumFmtBase::NumFmtBase;
#[derive(Clone,Copy)]
pub enum FormatValue<'a>{Number(f64),Text(&'a str)}
// java public interface IFormat
pub trait IFormat {
    // Rust protected vtable dispatch preserves the Java NumFmtBase hierarchy.
    fn base(&self)->&NumFmtBase {panic!("IFormat is not NumFmtBase")}
    fn base_mut(&mut self)->&mut NumFmtBase {panic!("IFormat is not NumFmtBase")}
    fn reset_context(&mut self,p:&PieceContext){self.base_mut().reset_context(p);}
    fn collectInfo(&mut self,_token:&TokenRef,_index:usize){}
    fn reset(&mut self,tokens:&[TokenRef],p:&PieceContext){
        // java this.reset(pieceContext);for(...) {this.mHolderTokens.add(holderToken);this.collectInfo(holderToken,index);++index;}
        self.reset_context(p);for (i,t) in tokens.iter().enumerate(){self.base_mut().mHolderTokens.push(t.clone());self.collectInfo(t,i);}
    }
    fn readyFormat(&mut self,v:FormatValue<'_>)->bool {self.base_mut().readyFormat(v)}
    fn doFormat(&mut self,result:&mut NumFmtResult){self.base_mut().doFormat(result);}
    fn finishFormat(&mut self,p:&NumFmtParam,result:&mut NumFmtResult){self.base_mut().finishFormat(p,result);}
    fn format(&mut self,v:FormatValue<'_>,p:&NumFmtParam,result:&mut NumFmtResult){
        // java final NumFmtBase.format: save param/style;clear value context and result;readyFormat;minus;doFormat;finishFormat.
        self.base_mut().prepare(p,result);
        if self.readyFormat(v){if self.base().mPHolderContext.mAddMinus{result.mNumFmtStr.push('-');}self.doFormat(result);}self.finishFormat(p,result);
    }
    // java NumFmtBase.formatValue returns d unchanged unless overridden.
    fn formatValue(&mut self,d:f64,_:&NumFmtParam)->f64 {d}
    fn getType(&self)->i16 {self.base().mNumFmtType}
    fn destroy(&mut self){}
}
pub type FormatRef=std::rc::Rc<std::cell::RefCell<dyn IFormat>>;
