use super::IFormat::{IFormat,FormatValue};
use crate::cn::wps::kfc::numfmt::{format::{NumFmtParam::NumFmtParam,NumFmtResult::NumFmtResult},placeholder::{PlaceHolderToken::TokenRef,PieceContext::PieceContext}};
// java public class NumFmtInvalid implements IFormat
pub struct NumFmtInvalid;impl NumFmtInvalid{pub fn new()->Self{Self}}
impl IFormat for NumFmtInvalid{
fn reset(&mut self,_:&[TokenRef],_:&PieceContext){}
// java result.clear();result.mNumFmtStr.append(NumFmtUtil.invalidNumberSignString);
fn format(&mut self,_:FormatValue<'_>,_:&NumFmtParam,r:&mut NumFmtResult){r.clear();r.mNumFmtStr="#".repeat(255);}
fn getType(&self)->i16{-1}
}
