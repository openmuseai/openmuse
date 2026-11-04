#![allow(non_snake_case)]

// Java: import cn.wps.collections.textrope.ITextRope
use crate::cn::wps::collections::textrope::ITextRope::ITextRope;
// Java: import cn.wps.moffice.writer.core.Document
use crate::cn::wps::moffice::writer::core::Document::Document;

/// Layout adapter: cp-range → rope text. Not a GraphReader port.
pub struct CpText {}

impl CpText {
    // java public int getText(int start, int end, char[] buffer, int nBufferStart)
    pub fn getText(doc: &dyn Document, start: i32, end: i32) -> String {
        let len = doc.getLength();
        let mut start = start;
        let mut end = end;
        if start < 0 {
            start = 0;
        }
        if end < 0 || end > len {
            end = len;
        }
        if start > end {
            start = end;
        }
        if start == end {
            return String::new();
        }
        let n = (end - start) as usize;
        let mut buffer = vec!['\u{0}'; n];
        match doc.getTextRope() {
            Some(r) if !r.is_null() => unsafe {
                (*r).getText(start, end, &mut buffer, 0);
            },
            _ => return String::new(),
        }
        buffer.into_iter().collect()
    }
}
