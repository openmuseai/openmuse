#![allow(non_snake_case, dead_code, unused_imports)]
use crate::cn::wps::moffice::writer::core::Document::Document;
use crate::cn::wps::moffice::writer::data::IPLC_Words::IPLC_Words;
use crate::cn::wps::moffice::writer::data::PLC_Char::PLC_Char;

// java public class KPLC_Words extends PLC_Char implements IPLC_Words
pub struct KPLC_Words {
    // java PLC_Char super_
    pub(crate) super_: PLC_Char,
}

impl Default for KPLC_Words {
    fn default() -> Self {
        Self { super_: PLC_Char::default() }
    }
}

impl IPLC_Words for KPLC_Words {
    fn size(&self) -> i32 {
        self.super_.size()
    }
}

impl KPLC_Words {
    // java public  KPLC_Words(Document document)
    pub fn KPLC_Words(document: &dyn Document) -> Self {
      // java super(document);
      Self { super_: PLC_Char::new(document) }
    }
}

impl std::ops::Deref for KPLC_Words {
    type Target = PLC_Char;
    fn deref(&self) -> &Self::Target { &self.super_ }
}
