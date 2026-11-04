#![allow(non_snake_case, dead_code, unused_imports)]
use crate::cn::wps::moffice::writer::core::Document::Document;
use crate::cn::wps::moffice::writer::data::PLC_Char::PLC_Char;
use crate::cn::wps::moffice::writer::data::PLC_Headings::PLC_Headings;

// java public class KPLC_Headings extends PLC_Char implements PLC_Headings
pub struct KPLC_Headings {
    // java PLC_Char super_
    pub(crate) super_: PLC_Char,
    // java private boolean forceAdjust = false;
    forceAdjust: bool,
    // java protected boolean adjustBeforeRemoveText = true;
    pub(crate) adjustBeforeRemoveText: bool,
}

impl Default for KPLC_Headings {
    fn default() -> Self {
        Self {
            super_: PLC_Char::default(),
            forceAdjust: false,
            adjustBeforeRemoveText: true,
        }
    }
}

impl PLC_Headings for KPLC_Headings {
    fn size(&self) -> i32 {
        self.super_.size()
    }
}

impl KPLC_Headings {
    // java public  KPLC_Headings(Document document)
    pub fn KPLC_Headings(document: &dyn Document) -> Self {
      // java super(document);
      let mut this = Self {
        super_: PLC_Char::new(document),
        forceAdjust: true,
        adjustBeforeRemoveText: false,
      };
      // java this.rebuild_all();
      // rust: no-stmt — papx 全扫描仍 DEFERRED（§1.7.1）
      let _ = &mut this;
      // java this.setTransaction(document.getTextDocument().getTransaction());
      this.super_.setTransaction(document.getTextDocument().and_then(|td| td.getTransaction()));
      // java this.forceAdjust = true;
      // java this.adjustBeforeRemoveText = false;
      this
    }
}

impl std::ops::Deref for KPLC_Headings {
    type Target = PLC_Char;
    fn deref(&self) -> &Self::Target { &self.super_ }
}
