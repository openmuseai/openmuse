use super::{IFormat::{IFormat,FormatValue},NumFmtBase::NumFmtBase,NumFmtNumberic::NumFmtNumberic};
use crate::cn::wps::kfc::numfmt::{format::NumFmtParam::NumFmtParam,placeholder::{PieceContext::PieceContext,PlaceHolderToken::TokenRef},util::NumberToStr::NumberToStr};
// java public class NumFmtPercent extends NumFmtNumberic
pub struct NumFmtPercent {pub base:NumFmtNumberic}
impl NumFmtPercent {pub fn new()->Self{Self{base:NumFmtNumberic::new_with_type(4)}}}
impl IFormat for NumFmtPercent {
fn base(&self)->&NumFmtBase{&self.base.base}fn base_mut(&mut self)->&mut NumFmtBase{&mut self.base.base}
fn reset_context(&mut self,p:&PieceContext){self.base.reset_context(p);}fn reset(&mut self,t:&[TokenRef],p:&PieceContext){self.base.reset(t,p);}fn collectInfo(&mut self,t:&TokenRef,i:usize){self.base.collectInfo(t,i);}fn readyFormat(&mut self,v:FormatValue<'_>)->bool{self.base.readyFormat(v)}
// java int pre=d<0? -1:1;double ret=Double.parseDouble(NumberToStr.double2String(d,mDecHolderCount+2,false));return ret*pre;
fn formatValue(&mut self,d:f64,_:&NumFmtParam)->f64{let pre=if d<0.0{-1.0}else{1.0};NumberToStr::double2String_percent(d,self.base.mDecHolderCount+2,false).parse::<f64>().unwrap()*pre}
}
