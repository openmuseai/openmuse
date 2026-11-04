#![allow(non_snake_case)]
#[derive(Default, Debug, Clone)]
pub struct StringBuilder {
    pub buf: String,
}
impl StringBuilder {
    pub fn new() -> Self { Self { buf: String::new() } }
    pub fn new__string(s: &str) -> Self { Self { buf: s.to_string() } }
    pub fn setLength(&mut self, len: i32) {
        let len = len.max(0) as usize;
        if len < self.buf.len() {
            self.buf.truncate(len);
        } else {
            self.buf.push_str(&" ".repeat(len - self.buf.len()));
        }
        if len == 0 { self.buf.clear(); }
    }
    pub fn append__char(&mut self, ch: char) -> &mut Self { self.buf.push(ch); self }
    pub fn append__string(&mut self, s: &str) -> &mut Self { self.buf.push_str(s); self }
    pub fn append(&mut self, ch: char) -> &mut Self { self.append__char(ch) }
    pub fn charAt(&self, index: i32) -> char {
        self.buf.chars().nth(index as usize).unwrap_or('\0')
    }
    pub fn length(&self) -> i32 { self.buf.chars().count() as i32 }
    pub fn toString(&self) -> String { self.buf.clone() }
    pub fn delete(&mut self, start: i32, end: i32) -> &mut Self {
        let s = start.max(0) as usize;
        let e = (end as usize).min(self.buf.len());
        if s < e {
            // char-index approximate via byte truncate for ASCII-heavy field codes
            let chars: Vec<char> = self.buf.chars().collect();
            let s = s.min(chars.len());
            let e = e.min(chars.len());
            self.buf = chars[..s].iter().chain(chars[e..].iter()).collect();
        }
        self
    }
}
impl std::fmt::Display for StringBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "{}", self.buf) }
}
