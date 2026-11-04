#![allow(non_snake_case, dead_code, unused_variables, unused_imports)]
//! KRange — inheritance surface used by HelperHeaderFooter.RangeHeaderFooter.
//! Full KRange.java (4924 lines) is not rewritten here.

use crate::cn::wps::moffice::writer::core::Document::Document;
use util::moffice::property::PropertySet::PropertySet;

pub struct KRange {
    document: Option<*mut (dyn Document + 'static)>,
    start: i32,
    end: i32,
}

impl Default for KRange {
    fn default() -> Self {
        Self {
            document: None,
            start: 0,
            end: 0,
        }
    }
}

impl KRange {
    // java: public static int fieldFlags(PLCField.FieldNode fieldNode)
    pub fn fieldFlags(_fieldNode: *mut ()) -> i32 {
        0
    }

    pub fn new__document_int_int(document: &dyn Document, start: i32, end: i32) -> Self {
        let p = unsafe {
            std::mem::transmute::<*const dyn Document, *mut (dyn Document + 'static)>(
                document as *const dyn Document,
            )
        };
        Self {
            document: Some(p),
            start: std::cmp::max(0, start),
            end,
        }
    }

    pub fn dispose(&mut self) {
        self.document = None;
        self.start = 0;
        self.end = 0;
    }

    pub fn getStart(&self) -> i32 {
        self.start
    }

    pub fn getEnd(&self) -> i32 {
        self.end
    }

    // java public long getRange()
    pub fn getRange(&self) -> i64 {
      // java return RANGEFactory.getRANGE(this.range.start, this.range.end);
      crate::cn::wps::moffice::writer::data::RANGEFactory::RANGEFactory::getRANGE(self.start, self.end)
    }

    pub fn setRange(&mut self, start: i32, end: i32) {
        let len = self
            .document
            .map(|p| unsafe { (*p).getLength() })
            .unwrap_or(0);
        if start >= 0 && start <= end && end <= len {
            self.start = start;
            self.end = end;
        }
    }

    pub fn setParagraphProperty(&mut self, _prop: PropertySet, _mode: i32) {}

    pub fn insertParagraph(&mut self) {}
}

// java: public static enum DeleteRangeReason (nested in KRange.java)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DeleteRangeReason {
    #[default]
    delete_key,
    backspace_key,
    typetext,
}
