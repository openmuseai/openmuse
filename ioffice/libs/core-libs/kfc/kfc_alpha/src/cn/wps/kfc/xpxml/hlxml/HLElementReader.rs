use std::io::Read;

use super::HLInnerAccepter::HLInnerAccepter;

// Java: public final class HLElementReader
pub struct HLElementReader<A: HLInnerAccepter> {
    // Java field: HLInnerAccepter mAccept
    mAccept: A,
    // Java field: char[] mBuffer
    mBuffer: Vec<char>,
    // Java field: int mLength
    mLength: usize,
    // Java field: int mStart
    mStart: usize,
    // Java field: int mTop
    mTop: usize,
    // Java field: boolean bInSingleQuote
    bInSingleQuote: bool,
    // Java field: boolean bInDoubleQuote
    bInDoubleQuote: bool,
}

impl<A: HLInnerAccepter> HLElementReader<A> {
    // Java: HLElementReader(HLInnerAccepter accept)
    pub fn new(accept: A) -> Self {
        HLElementReader {
            mAccept: accept,
            mBuffer: Vec::new(),
            mLength: 0,
            mStart: 0,
            mTop: 0,
            bInSingleQuote: false,
            bInDoubleQuote: false,
        }
    }

    pub fn into_accept(self) -> A {
        self.mAccept
    }

    // Java: void parser(InputStream stream)
    pub fn parser(&mut self, stream: &mut dyn Read) -> std::io::Result<()> {
        // Java: if (stream != null && this.isSimpleUTF8(stream)) this.parser(stream, "UTF-8");
        // `isSimpleUTF8` 在声明不是固定 55 字节时已经把全文按探测到的编码解析完。
        if self.is_simple_utf8(stream)? {
            self.parser_encoding(stream, "UTF-8")?;
        }
        Ok(())
    }

    // Java: void parser(InputStream stream, String encoding)
    pub fn parser_encoding(&mut self, stream: &mut dyn Read, encoding: &str) -> std::io::Result<()> {
        let mut bytes = Vec::new();
        stream.read_to_end(&mut bytes)?;
        let chars = decode_chars(&bytes, encoding);
        self.scan(&chars);
        Ok(())
    }

    fn scan(&mut self, chars: &[char]) {
        self.mBuffer = chars.to_vec();
        self.mLength = chars.len();
        self.mTop = 0;
        self.mStart = 0;
        while self.mTop < self.mLength {
            if self.mBuffer[self.mStart] == '<' {
                if self.mStart + 1 < self.mLength
                    && (self.mBuffer[self.mStart + 1] == '?' || self.mBuffer[self.mStart + 1] == '!')
                {
                    if !self.filter_invalid_label() {
                        break;
                    }
                } else if !self.parser_element() {
                    break;
                }
            } else if !self.parser_characters() {
                break;
            }
        }
    }

    // Java: boolean parserElement()
    fn parser_element(&mut self) -> bool {
        while self.mTop < self.mLength {
            match self.mBuffer[self.mTop] {
                '"' => {
                    if !self.bInSingleQuote {
                        self.bInDoubleQuote = !self.bInDoubleQuote;
                    }
                }
                '\'' => {
                    if !self.bInDoubleQuote {
                        self.bInSingleQuote = !self.bInSingleQuote;
                    }
                }
                '>' => {
                    if !self.bInDoubleQuote && !self.bInSingleQuote {
                        let start = self.mStart;
                        let end = self.mTop;
                        let piece: Vec<char> = self.mBuffer[start..=end].to_vec();
                        let last = piece.len() - 1;
                        self.mAccept.label(&piece, 0, last);
                        self.mTop += 1;
                        self.mStart = self.mTop;
                        return true;
                    }
                }
                _ => {}
            }
            self.mTop += 1;
        }
        false
    }

    // Java: boolean parserCharacters()
    fn parser_characters(&mut self) -> bool {
        while self.mTop < self.mLength {
            if self.mBuffer[self.mTop] == '<' {
                let start = self.mStart;
                let end = self.mTop - 1;
                let piece: Vec<char> = self.mBuffer[start..=end].to_vec();
                let last = piece.len() - 1;
                self.mAccept.characters(&piece, 0, last, true);
                self.mStart = self.mTop;
                return true;
            }
            self.mTop += 1;
        }
        false
    }

    // Java: boolean filterInvalidLabel()
    fn filter_invalid_label(&mut self) -> bool {
        loop {
            if self.mTop >= self.mLength {
                return false;
            }
            if self.mBuffer[self.mTop] != '>' {
                self.mTop += 1;
                continue;
            }
            if self.is_declaration_end() || self.is_doc_type_end() || self.is_annotation_end() {
                self.mTop += 1;
                self.mStart = self.mTop;
                return true;
            }
            if self.is_cdata_end() {
                let start = self.mStart + 9;
                let end = self.mTop - 3;
                if start <= end {
                    let piece: Vec<char> = self.mBuffer[start..=end].to_vec();
                    let last = piece.len() - 1;
                    self.mAccept.characters(&piece, 0, last, false);
                }
                self.mTop += 1;
                self.mStart = self.mTop;
                return true;
            }
            self.mTop += 1;
        }
    }

    // Java: boolean isDeclarationEnd()
    fn is_declaration_end(&self) -> bool {
        self.mTop - self.mStart >= 3
            && self.mBuffer[self.mStart] == '<'
            && self.mBuffer[self.mStart + 1] == '?'
            && self.mBuffer[self.mTop] == '>'
    }

    // Java: boolean isDocTypeEnd()
    fn is_doc_type_end(&self) -> bool {
        self.mTop - self.mStart >= 10
            && self.mBuffer[self.mStart] == '<'
            && self.mBuffer[self.mStart + 1] == '!'
            && self.mBuffer[self.mStart + 2] == 'D'
            && self.mBuffer[self.mStart + 3] == 'O'
            && self.mBuffer[self.mTop] == '>'
    }

    // Java: boolean isCDATAEnd()
    fn is_cdata_end(&self) -> bool {
        self.mTop - self.mStart >= 12
            && self.mBuffer[self.mStart] == '<'
            && self.mBuffer[self.mStart + 1] == '!'
            && self.mBuffer[self.mStart + 2] == '['
            && self.mBuffer[self.mTop - 2] == ']'
            && self.mBuffer[self.mTop - 1] == ']'
            && self.mBuffer[self.mTop] == '>'
    }

    // Java: boolean isAnnotationEnd()
    fn is_annotation_end(&self) -> bool {
        self.mTop - self.mStart >= 7
            && self.mBuffer[self.mStart] == '<'
            && self.mBuffer[self.mStart + 1] == '!'
            && self.mBuffer[self.mStart + 2] == '-'
            && self.mBuffer[self.mStart + 3] == '-'
            && self.mBuffer[self.mTop - 2] == '-'
            && self.mBuffer[self.mTop - 1] == '-'
            && self.mBuffer[self.mTop] == '>'
    }

    // Java: boolean isSimpleUTF8(InputStream stream)
    /// 声明恰好是 55 字节且以 `?>` 结尾时返回 true，调用方接着用 UTF-8 解析剩余字节。
    /// 其余情况在这里按探测到的编码把全文解析完，并返回 false。
    fn is_simple_utf8(&mut self, stream: &mut dyn Read) -> std::io::Result<bool> {
        let mut header = [0u8; 55];
        let mut filled = 0usize;
        while filled < header.len() {
            match stream.read(&mut header[filled..]) {
                Ok(0) => break,
                Ok(n) => filled += n,
                Err(e) if e.kind() == std::io::ErrorKind::Interrupted => continue,
                Err(e) => return Err(e),
            }
        }
        if filled == header.len()
            && header[0] == b'<'
            && header[1] == b'?'
            && header[34] == b'8'
            && header[53] == b'?'
            && header[54] == b'>'
        {
            return Ok(true);
        }
        let encoding = detect_encoding(&header[..filled]);
        let mut rest = Vec::new();
        stream.read_to_end(&mut rest)?;
        let mut all = header[..filled].to_vec();
        all.extend(rest);
        let chars = decode_chars(&all, &encoding);
        self.scan(&chars);
        Ok(false)
    }
}

fn detect_encoding(header: &[u8]) -> String {
    if header.len() >= 2 && header[0] == 0xFF && header[1] == 0xFE {
        return "UTF-16".to_string();
    }
    if header.len() >= 2 && header[0] == b'<' && header[1] == b'?' {
        let tmp = String::from_utf8_lossy(header);
        if let Some(index) = tmp.find("encoding=") {
            let sep_at = index + "encoding=".len();
            let rest: Vec<char> = tmp[sep_at..].chars().collect();
            if let Some(separator) = rest.first().copied() {
                if let Some(end) = rest.iter().skip(1).position(|c| *c == separator) {
                    return rest[1..=end].iter().collect();
                }
            }
        }
    }
    "UTF-8".to_string()
}

fn decode_chars(bytes: &[u8], encoding: &str) -> Vec<char> {
    if encoding.eq_ignore_ascii_case("UTF-16")
        || encoding.eq_ignore_ascii_case("UTF-16LE")
    {
        let mut raw = bytes;
        if raw.len() >= 2 && raw[0] == 0xFF && raw[1] == 0xFE {
            raw = &raw[2..];
        }
        return raw
            .chunks(2)
            .filter_map(|c| c.get(1).map(|b| u16::from_le_bytes([c[0], *b])))
            .map(|u| char::from_u32(u as u32).unwrap_or('\u{FFFD}'))
            .collect();
    }
    String::from_utf8_lossy(bytes).chars().collect()
}
