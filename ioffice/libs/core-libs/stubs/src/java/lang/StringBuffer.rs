#![allow(non_snake_case)]
/// Java `java.lang.StringBuffer` — mutable string buffer (same surface as StringBuilder for this codebase).
#[derive(Default, Debug, Clone)]
pub struct StringBuffer {
    pub buf: String,
}
impl StringBuffer {
    pub fn new() -> Self { Self { buf: String::new() } }
    pub fn new__string(s: &str) -> Self { Self { buf: s.to_string() } }
    pub fn setLength(&mut self, len: i32) {
        let len = len.max(0) as usize;
        if len == 0 {
            self.buf.clear();
        } else if len < self.buf.chars().count() {
            self.buf = self.buf.chars().take(len).collect();
        } else {
            let n = len - self.buf.chars().count();
            self.buf.push_str(&"\0".repeat(n));
        }
    }
    pub fn append__char(&mut self, ch: char) -> &mut Self { self.buf.push(ch); self }
    pub fn append__string(&mut self, s: &str) -> &mut Self { self.buf.push_str(s); self }
    pub fn append(&mut self, ch: char) -> &mut Self { self.append__char(ch) }
    pub fn append_str(&mut self, s: &str) -> &mut Self { self.append__string(s) }
    pub fn charAt(&self, index: i32) -> char {
        self.buf.chars().nth(index as usize).unwrap_or('\0')
    }
    pub fn length(&self) -> i32 { self.buf.chars().count() as i32 }
    pub fn toString(&self) -> String { self.buf.clone() }
    pub fn delete(&mut self, start: i32, end: i32) -> &mut Self {
        let chars: Vec<char> = self.buf.chars().collect();
        let s = (start.max(0) as usize).min(chars.len());
        let e = (end.max(0) as usize).min(chars.len());
        if s < e {
            self.buf = chars[..s].iter().chain(chars[e..].iter()).collect();
        }
        self
    }
    pub fn deleteCharAt(&mut self, index: i32) -> &mut Self {
        let i = index as usize;
        let chars: Vec<char> = self.buf.chars().collect();
        if i < chars.len() {
            self.buf = chars[..i].iter().chain(chars[i + 1..].iter()).collect();
        }
        self
    }
    pub fn insert(&mut self, offset: i32, ch: char) -> &mut Self {
        let chars: Vec<char> = self.buf.chars().collect();
        let o = (offset.max(0) as usize).min(chars.len());
        self.buf = chars[..o].iter().copied().chain(std::iter::once(ch)).chain(chars[o..].iter().copied()).collect();
        self
    }
    /// Java `insert(int offset, String str)`
    pub fn insert_str(&mut self, offset: i32, s: &str) -> &mut Self {
        let chars: Vec<char> = self.buf.chars().collect();
        let o = (offset.max(0) as usize).min(chars.len());
        self.buf = chars[..o].iter().copied().chain(s.chars()).chain(chars[o..].iter().copied()).collect();
        self
    }
    pub fn replace(&mut self, start: i32, end: i32, str: &str) -> &mut Self {
        let chars: Vec<char> = self.buf.chars().collect();
        let s = (start.max(0) as usize).min(chars.len());
        let e = (end.max(0) as usize).min(chars.len());
        self.buf = chars[..s].iter().copied().chain(str.chars()).chain(chars[e..].iter().copied()).collect();
        self
    }
    pub fn setCharAt(&mut self, index: i32, ch: char) {
        let chars: Vec<char> = self.buf.chars().collect();
        let i = index as usize;
        if i < chars.len() {
            let mut chars = chars;
            chars[i] = ch;
            self.buf = chars.into_iter().collect();
        }
    }
    pub fn reverse(&mut self) -> &mut Self {
        self.buf = self.buf.chars().rev().collect();
        self
    }
}
impl std::fmt::Display for StringBuffer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.buf)
    }
}
impl From<&StringBuffer> for String {
    fn from(s: &StringBuffer) -> String { s.buf.clone() }
}
