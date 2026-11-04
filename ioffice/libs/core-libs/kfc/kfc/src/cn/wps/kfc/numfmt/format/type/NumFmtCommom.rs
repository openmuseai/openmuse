use super::{IFormat::IFormat,NumFmtBase::NumFmtBase};
use crate::cn::wps::kfc::numfmt::{format::{NumFmtResult::NumFmtResult,NumFmtParam::NumFmtParam},placeholder::PlaceHolderToken::TokenRef};
// java public class NumFmtCommom extends NumFmtBase
pub struct NumFmtCommom {pub base:NumFmtBase}
impl NumFmtCommom {pub fn new()->Self {Self{base:NumFmtBase::new(128)}}}
impl IFormat for NumFmtCommom {fn base(&self)->&NumFmtBase{&self.base}fn base_mut(&mut self)->&mut NumFmtBase{&mut self.base}
}
