use std::{rc::Rc,cell::RefCell,collections::HashMap};
use super::r#type::{IFormat::{IFormat,FormatRef},NumFmtGeneral::NumFmtGeneral,NumFmtPercent::NumFmtPercent,NumFmtScience::NumFmtScience,NumFmtFraction::NumFmtFraction,NumFmtText::NumFmtText,NumFmtNumberic::NumFmtNumberic,NumFmtDateTime::NumFmtDateTime,NumFmtCommom::NumFmtCommom,NumFmtUnknown::NumFmtUnknown,NumFmtInvalid::NumFmtInvalid};
use crate::cn::wps::kfc::numfmt::placeholder::{PieceContext::PieceContext,PlaceHolderToken::TokenRef};
// java public class FormatFactory
pub struct FormatFactory {mFormatters:HashMap<i16,FormatRef>,mWordDateTimeFormatter:Option<FormatRef>}
impl FormatFactory {
    pub fn new()->Self{Self{mFormatters:HashMap::new(),mWordDateTimeFormatter:None}}
    // java public IFormat getFormatter(Short formatterType,ArrayList<PlaceHolderToken> compilePieces,PieceContext pieceContext)
    pub fn getFormatter(&mut self,ty:i16,tokens:&[TokenRef],p:&PieceContext)->Option<FormatRef>{
        let formatter=if let Some(formatter)=self.mFormatters.get(&ty){formatter.clone()}else{
            let formatter:FormatRef=match ty {
                -1=>Rc::new(RefCell::new(NumFmtInvalid::new())),1=>Rc::new(RefCell::new(NumFmtGeneral::new())),4=>Rc::new(RefCell::new(NumFmtPercent::new())),8=>Rc::new(RefCell::new(NumFmtFraction::new())),16=>Rc::new(RefCell::new(NumFmtScience::new())),64=>Rc::new(RefCell::new(NumFmtText::new())),128=>Rc::new(RefCell::new(NumFmtCommom::new())),512=>Rc::new(RefCell::new(NumFmtNumberic::new())),
                // java case 8192: Word uses its separate formatter; Excel uses NumFmtDateTime; other context returns null.
                8192=>{if p.mContextType==1{panic!("DEFERRED_NON_FUNCTION: WordFmtDateTime");}else if p.mContextType==2{Rc::new(RefCell::new(NumFmtDateTime::new()))}else{return None;}},
                _=>Rc::new(RefCell::new(NumFmtUnknown::new())),
            };
            // java if(shouldPutInCollections)this.mFormatters.put(formatterType,numFormatter);
            self.mFormatters.insert(ty,formatter.clone());formatter
        };
        // java if(numFormatter!=null)numFormatter.reset(compilePieces,pieceContext);return numFormatter;
        formatter.borrow_mut().reset(tokens,p);Some(formatter)
    }
}
