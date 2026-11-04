use super::{IFormat::IFormat,NumFmtBase::NumFmtBase};
use crate::cn::wps::kfc::numfmt::{format::{NumFmtResult::NumFmtResult,NumFmtParam::NumFmtParam},placeholder::PlaceHolderToken::TokenRef};
// java public class NumFmtUnknown extends NumFmtBase
pub struct NumFmtUnknown {pub base:NumFmtBase}
impl NumFmtUnknown {pub fn new()->Self {Self{base:NumFmtBase::new(0)}}}
impl IFormat for NumFmtUnknown {fn base(&self)->&NumFmtBase{&self.base}fn base_mut(&mut self)->&mut NumFmtBase{&mut self.base}
}
