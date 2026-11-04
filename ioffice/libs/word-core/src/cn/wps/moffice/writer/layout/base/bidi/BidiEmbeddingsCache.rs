// Java: cn.wps.moffice.writer.layout.base.bidi.BidiEmbeddingsCache
#![allow(dead_code, non_snake_case)]

use crate::cn::wps::moffice::writer::core::Document::Document;

// java public class BidiEmbeddingsCache
pub struct BidiEmbeddingsCache {
    mLineStartCP: i32,
    mCharBuff: Vec<char>,
    mCurEmbeddints: i32,
    mEmbeddingCharStack: Vec<char>,
}

impl Default for BidiEmbeddingsCache {
    fn default() -> Self { Self::new() }
}

impl BidiEmbeddingsCache {
    pub fn new() -> Self {
        Self {
            mLineStartCP: 0,
            mCharBuff: vec!['\0'; 128],
            mCurEmbeddints: 0,
            mEmbeddingCharStack: Vec::new(),
        }
    }

    pub fn dispose(&mut self) {
        self.mEmbeddingCharStack.clear();
    }

    pub fn clear(&mut self) {
        self.mLineStartCP = -1;
        self.mEmbeddingCharStack.clear();
        self.mCurEmbeddints = 0;
    }

    // java public void init(Document doc, int paraStart, int lineStart)
    pub fn init(&mut self, doc: &dyn Document, paraStart: i32, lineStart: i32) {
        if self.mLineStartCP != lineStart && lineStart != 0 && paraStart != lineStart {
            self.clear();
            let mut curReadStart = paraStart.max(0);
            let max = lineStart.min(doc.getLength());
            while curReadStart < max {
                let end = (curReadStart + 128).min(max);
                let readThisTime = doc.getText(curReadStart, end, &mut self.mCharBuff, 0);
                let n = readThisTime.max(0) as usize;
                let chars: Vec<char> = self.mCharBuff.iter().take(n.min(self.mCharBuff.len())).copied().collect();
                for c in chars {
                    self.processChar(c);
                }
                if readThisTime <= 0 {
                    break;
                }
                curReadStart = end;
            }
        }
    }

    // java public void processChar(char c)
    pub fn processChar(&mut self, c: char) {
        match c {
            '\u{200e}' | '\u{200f}' | '\u{202a}' | '\u{202b}' | '\u{202d}' | '\u{202e}' => {
                self.mEmbeddingCharStack.push(c);
                self.updateEmbedding();
            }
            '\u{202c}' => {
                self.mEmbeddingCharStack.pop();
                self.updateEmbedding();
            }
            _ => {}
        }
    }

    fn updateEmbedding(&mut self) {
        self.mCurEmbeddints = 0;
        let mut i = 0;
        while i < self.mEmbeddingCharStack.len() {
            match self.mEmbeddingCharStack[i] {
                '\u{200e}' => self.apply_mark(false),
                '\u{200f}' => self.apply_mark(true),
                '\u{202a}' => self.apply_embed(false),
                '\u{202b}' => self.apply_embed(true),
                '\u{202c}' => {
                    self.mEmbeddingCharStack.pop();
                    self.updateEmbedding();
                    return;
                }
                '\u{202d}' => self.apply_override(false),
                '\u{202e}' => self.apply_override(true),
                _ => {}
            }
            i += 1;
        }
    }

    fn apply_mark(&mut self, rtl: bool) {
        let odd = self.mCurEmbeddints.abs() % 2 != 0;
        let want_odd = rtl;
        if odd != want_odd {
            if self.mCurEmbeddints >= 0 {
                self.mCurEmbeddints += 1;
            } else if self.mCurEmbeddints < 0 {
                self.mCurEmbeddints = 1 - self.mCurEmbeddints;
            }
        } else if self.mCurEmbeddints < 0 {
            self.mCurEmbeddints = -self.mCurEmbeddints;
        }
    }

    fn apply_embed(&mut self, rtl: bool) {
        let odd = self.mCurEmbeddints.abs() % 2 != 0;
        let want_odd = rtl;
        if odd != want_odd {
            if self.mCurEmbeddints >= 0 {
                self.mCurEmbeddints += 1;
            } else if self.mCurEmbeddints < 0 {
                self.mCurEmbeddints -= 1;
            }
        } else if self.mCurEmbeddints >= 0 {
            self.mCurEmbeddints += 2;
        } else {
            self.mCurEmbeddints -= 2;
        }
    }

    fn apply_override(&mut self, rtl: bool) {
        let odd = self.mCurEmbeddints.abs() % 2 != 0;
        let want_odd = rtl;
        if odd != want_odd {
            if self.mCurEmbeddints >= 0 {
                self.mCurEmbeddints = -(self.mCurEmbeddints + 1);
            } else if self.mCurEmbeddints < 0 {
                self.mCurEmbeddints -= 1;
            }
        } else if self.mCurEmbeddints == 0 {
            self.mCurEmbeddints = if rtl { -1 } else { -2 };
        } else if self.mCurEmbeddints > 0 {
            self.mCurEmbeddints = -self.mCurEmbeddints;
        }
    }

    pub fn setLineStart(&mut self, lineStart: i32) {
        self.mLineStartCP = lineStart;
    }

    pub fn getEmbedding(&self) -> i8 {
        self.mCurEmbeddints as i8
    }
}
