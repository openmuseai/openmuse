//! Read-only Word-style word count over the live main-story text rope.
//!
//! The product may request this on open and after an idle edit/selection; it
//! never runs in the layout loop or in a plugin extension process.

use crate::cn::wps::moffice::writer::core::TextDocument::TextDocument;
use serde::Serialize;

#[derive(Clone, Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WordTextStatistics {
    pub schema_version: u32,
    pub words: i32,
    pub asian_chars: i32,
    pub non_asian_words: i32,
    pub chars_no_space: i32,
    pub chars_with_space: i32,
    pub paragraphs: i32,
}

fn asian(ch: char) -> bool {
    matches!(ch as u32,
        0x1100..=0x11FF | 0x2E80..=0x2FDF | 0x3001..=0x303F |
        0x3040..=0x30FF | 0x3100..=0x312F | 0x3130..=0x318F |
        0x31C0..=0x31FF | 0x3400..=0x4DBF | 0x4E00..=0x9FFF |
        0xAC00..=0xD7AF | 0xF900..=0xFAFF | 0xFF01..=0xFFA0 |
        0xFFE0..=0xFFE6 | 0x20000..=0x3FFFF)
}

fn non_asian_word_char(ch: char) -> bool {
    matches!(ch as u32,
        0x30..=0x39 | 0x41..=0x5A | 0x61..=0x7A |
        0x00C0..=0x024F | 0x0370..=0x03FF | 0x0400..=0x04FF |
        0x0530..=0x058F | 0x0590..=0x05FF | 0x0600..=0x06FF |
        0x0900..=0x097F | 0x0E00..=0x0EFF | 0x0F00..=0x0FFF |
        0x1000..=0x109F | 0x10A0..=0x10FF | 0x1780..=0x17FF |
        0x1E00..=0x1EFF)
}

#[derive(Default)]
struct Counter {
    stats: WordTextStatistics,
    in_word: bool,
    join_pending: bool,
    paragraph_nonempty: bool,
}

impl Counter {
    fn push(&mut self, ch: char) {
        if ch == '\r' || ch == '\n' {
            if self.paragraph_nonempty {
                self.stats.paragraphs += 1;
                self.paragraph_nonempty = false;
            }
        } else if !ch.is_control() {
            self.stats.chars_with_space += ch.len_utf16() as i32;
            if !ch.is_whitespace() {
                self.stats.chars_no_space += ch.len_utf16() as i32;
                self.paragraph_nonempty = true;
            }
        }
        if asian(ch) {
            self.stats.asian_chars += 1;
            self.in_word = false;
            self.join_pending = false;
        } else if non_asian_word_char(ch) {
            if !self.in_word && !self.join_pending {
                self.stats.non_asian_words += 1;
            }
            self.in_word = true;
            self.join_pending = false;
        } else if (ch == '\'' || ch == '-') && self.in_word {
            // GenOffice treats an apostrophe/hyphen between two word groups
            // as one word. A second joiner or delimiter ends the group.
            self.in_word = false;
            self.join_pending = true;
        } else {
            self.in_word = false;
            self.join_pending = false;
        }
    }

    fn finish(mut self) -> WordTextStatistics {
        if self.paragraph_nonempty {
            self.stats.paragraphs += 1;
        }
        self.stats.words = self.stats.asian_chars + self.stats.non_asian_words;
        self.stats.schema_version = 1;
        self.stats
    }
}

/// Count a half-open main-story range using bounded scratch memory. The
/// caller holds the document session lock while reading the rope.
pub fn text_statistics(doc: &TextDocument, start: i32, end: i32) -> WordTextStatistics {
    let Some(main) = doc.getMainDocument() else {
        return Counter::default().finish();
    };
    let len = main.getLength();
    let lo = start.clamp(0, len);
    let hi = end.clamp(lo, len);
    let Some(ptr) = main.getTextRope() else {
        return Counter::default().finish();
    };
    if ptr.is_null() {
        return Counter::default().finish();
    }
    let rope = unsafe { &*ptr };
    let mut counter = Counter::default();
    let mut cursor = lo;
    while cursor < hi {
        let next = (cursor + 4096).min(hi);
        let mut chars = vec!['\0'; (next - cursor) as usize];
        rope.getText(cursor, next, &mut chars, 0);
        for ch in chars {
            counter.push(ch);
        }
        cursor = next;
    }
    counter.finish()
}

pub fn count_words(doc: &TextDocument, start: i32, end: i32) -> i32 {
    text_statistics(doc, start, end).words
}

#[cfg(test)]
mod tests {
    use super::*;

    fn count(text: &str) -> i32 {
        let mut counter = Counter::default();
        for ch in text.chars() {
            counter.push(ch);
        }
        counter.finish().words
    }

    #[test]
    fn counts_genoffice_cjk_and_joined_non_asian_words() {
        assert_eq!(count("中文，ABC-def don't 😊 123"), 6);
        assert_eq!(count("hello--world 你好"), 4);
        assert_eq!(count("— \r\n"), 0);
    }

    #[test]
    fn counts_dialog_fields_without_scanning_whole_document_into_memory() {
        let mut counter = Counter::default();
        for ch in "中文，ABC-def don't 😊 123\r\nA B".chars() {
            counter.push(ch);
        }
        let stats = counter.finish();
        assert_eq!(stats.schema_version, 1);
        assert_eq!(stats.words, 8);
        assert_eq!(stats.asian_chars, 3);
        assert_eq!(stats.non_asian_words, 5);
        assert_eq!(stats.chars_no_space, 22);
        assert_eq!(stats.chars_with_space, 26);
        assert_eq!(stats.paragraphs, 2);
    }
}
