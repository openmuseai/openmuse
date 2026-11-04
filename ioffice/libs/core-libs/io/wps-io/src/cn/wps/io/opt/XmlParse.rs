#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unreachable_code)]

// Java: import cn.wps.io.dom.io.WMap
// Java: import java.io.IOException

use super::XmlStringBuilder::XmlStringBuilder;
use super::Token::Token;
use super::ContentHandler::ContentHandler;
use crate::io::dom::io::WMap::WMap;

// Java field: short INVLID
pub const INVLID: i16 = -1;
// Java field: short EMPTY_TAG
pub const EMPTY_TAG: i16 = 6;
// Java field: short NORMAL_START
pub const NORMAL_START: i16 = 2;
// Java field: short OPEN_TAG
pub const OPEN_TAG: i16 = 3;
// Java field: short CLOSE_TAG
pub const CLOSE_TAG: i16 = 4;
// Java field: short END_TAG
pub const END_TAG: i16 = 5;
// Java field: short TAG_NAME
pub const TAG_NAME: i16 = 7;
// Java field: short TAG_ATRNAME
pub const TAG_ATRNAME: i16 = 8;
// Java field: short TAG_ATRVAL_START
pub const TAG_ATRVAL_START: i16 = 9;
// Java field: short TAG_DOUBLE_QUOTES_ATRVAL
pub const TAG_DOUBLE_QUOTES_ATRVAL: i16 = 10;
// Java field: short DATA
pub const DATA: i16 = 11;
// Java field: short Escape
pub const ESCAPE: i16 = 12;
// Java field: short TAG_SINGLE_QUOTES_ATRVAL
pub const TAG_SINGLE_QUOTES_ATRVAL: i16 = 13;
// Java field: short EXCLAMATION
pub const EXCLAMATION: i16 = 14;
// Java field: short CDATA_START
pub const CDATA_START_CONST: i16 = 15;
// Java field: short CDATA_CONTENT
pub const CDATA_CONTENT: i16 = 16;
// Java field: String CDATA_START_VAL
const CDATA_START_VAL: &str = "[CDATA[";

// Java: public class XmlParse
pub struct XmlParse {
    // Java field: int lastSquareBracket
    lastSquareBracket: i32,
    // Java field: int state
    state: i32,
    // Java field: int preState
    preState: i32,
    // Java field: XmlStringBuilder tagName
    tagName: XmlStringBuilder,
    // Java field: XmlStringBuilder escape
    escape: XmlStringBuilder,
    // Java field: Token xmlToken
    xmlToken: Token,
    // Java field: int mMode
    mMode: i32,
    // Java field: ContentHandler contentHandler
    contentHandler: Box<dyn ContentHandler>,
    // rust: no-java —— `SAXReader` 替代 JDK `SAXParser` 时需要的空白保真开关（Java 侧 SAX 原样上报字符）
    preserveWhitespace: bool,
}

// Java: public class XmlParse
impl XmlParse {
    // Java: XmlParse(ContentHandler contentHandler, int mode)
    pub fn new(content_handler: Box<dyn ContentHandler>, mode: i32) -> Self {
        // Java: super();
        // Java: this.contentHandler = contentHandler;
        // Java: this.tagName = new XmlStringBuilder();
        // Java: this.escape = new XmlStringBuilder();
        // Java: this.xmlToken = new Token();
        // Java: this.mMode = mode;
        XmlParse {
            lastSquareBracket: 0,
            state: EMPTY_TAG as i32,
            preState: 0,
            tagName: XmlStringBuilder::new(),
            escape: XmlStringBuilder::new(),
            xmlToken: Token::new(),
            mMode: mode,
            contentHandler: content_handler,
            preserveWhitespace: false,
        }
    }

    // rust: no-java —— Java 侧 SAX 通路由 JDK `SAXParser` 提供字符（**不做**空白裁剪）；
    //       本 crate 的 `XmlParse` 在 DATA 态按 WPS 原语义「非 `xml:space=preserve` 即丢空白」
    //       ⇒ 给 SAX 替代通路一个显式开关（默认关闭 = 既有调用方行为不变；登记 §1.7.1 ⑩）
    pub fn setPreserveWhitespace(&mut self, preserve: bool) {
        self.preserveWhitespace = preserve;
    }

    // Java: void setIgnoreElement(int elementId)
    pub fn setIgnoreElement(&mut self, element_id: i32) {
        // Java: this.xmlToken.setIgnoreElementId(elementId);
        self.xmlToken.setIgnoreElementId(element_id);
    }

    // Java: void initNamespace4HwMemo()
    pub fn initNamespace4HwMemo(&mut self) {
        // Java: if (this.xmlToken != null)
        // Java: this.xmlToken.addNamespace("w", WMap.MAIN_NAMESPACING);
        self.xmlToken.addNamespace("w", WMap::MAIN_NAMESPACING());
    }

    // Java: void parse(char[] buf, int start, int len)
    pub fn parse(&mut self, buf: &[char], start: usize, len: usize) {
        // Java: for(int i = start; i < len; ++i)
        for i in start..len {
            // Java: char ch = buf[i];
            let ch = buf[i];
            // Java: switch (this.state)
            // Java: case -1:
            // Java: this.state = this.parseInvalidTag(ch);
            // Java: case 0:
            // Java: case 1:
            // Java: case 3:
            // Java: case 4:
            // Java: default:
            // Java: break;
            // Java: case 2:
            // Java: this.state = this.parseTag(ch);
            // Java: break;
            // Java: case 5:
            // Java: this.state = this.parseEndTag(ch);
            // Java: break;
            // Java: case 6:
            // Java: this.state = this.parseEmptyTag(ch);
            // Java: break;
            // Java: case 7:
            // Java: this.state = this.parseTagName(ch);
            // Java: break;
            // Java: case 8:
            // Java: this.state = this.parseTagAttributeName(ch);
            // Java: break;
            // Java: case 9:
            // Java: this.state = this.parseTagAttributeValueStart(ch);
            // Java: break;
            // Java: case 10:
            // Java: this.state = this.parseDoubleQuotesTagAttrVal(ch);
            // Java: break;
            // Java: case 11:
            // Java: this.state = this.parseData(ch);
            // Java: break;
            // Java: case 12:
            // Java: this.state = this.parseEscape(ch);
            // Java: break;
            // Java: case 13:
            // Java: this.state = this.parseSingleQuotesTagAttrVal(ch);
            // Java: break;
            // Java: case 14:
            // Java: this.state = this.parseExclamation(ch);
            // Java: break;
            // Java: case 15:
            // Java: this.state = this.parseCDATAStart(ch);
            // Java: break;
            // Java: case 16:
            // Java: this.state = this.parseCDataContent(ch);
            match self.state {
                -1 => { self.state = self.parseInvalidTag(ch); }
                0 | 1 | 3 | 4 => {}
                2 => { self.state = self.parseTag(ch); }
                5 => { self.state = self.parseEndTag(ch); }
                6 => { self.state = self.parseEmptyTag(ch); }
                7 => { self.state = self.parseTagName(ch); }
                8 => { self.state = self.parseTagAttributeName(ch); }
                9 => { self.state = self.parseTagAttributeValueStart(ch); }
                10 => { self.state = self.parseDoubleQuotesTagAttrVal(ch); }
                11 => { self.state = self.parseData(ch); }
                12 => { self.state = self.parseEscape(ch); }
                13 => { self.state = self.parseSingleQuotesTagAttrVal(ch); }
                14 => { self.state = self.parseExclamation(ch); }
                15 => { self.state = self.parseCDATAStart(ch); }
                16 => { self.state = self.parseCDataContent(ch); }
                _ => {}
            }
        }
    }

    // Java: int parseEmptyTag(char ch)
    fn parseEmptyTag(&mut self, ch: char) -> i32 {
        // Java: switch (ch)
        match ch {
            // Java: case '&':
            '&' => {
                // Java: this.escape.clear();
                self.escape.clear();
                // Java: this.preState = 11;
                self.preState = 11;
                // Java: return 12;
                12
            }
            // Java: case '<':
            '<' => {
                // Java: return 2;
                2
            }
            // Java: case '>':
            '>' => {
                // Java: return 6;
                6
            }
            // Java: default:
            _ => {
                // Java: if (this.isPreserveSpace())
                if self.preserveWhitespace || self.isPreserveSpace() {
                    // Java: this.tagName.append(ch);
                    self.tagName.append_char(ch);
                    // Java: return 11;
                    return 11;
                }
                // Java: else if (!this.isSpace(ch))
                else if !self.isSpace(ch) {
                    // Java: this.tagName.append(ch);
                    self.tagName.append_char(ch);
                    // Java: return 11;
                    return 11;
                }
                // Java: else
                else {
                    // Java: return this.state;
                    self.state
                }
            }
        }
    }

    // Java: int parseEscape(char ch)
    fn parseEscape(&mut self, ch: char) -> i32 {
        // Java: switch (ch)
        match ch {
            // Java: case ';':
            ';' => {
                // Java: return this.convertEscape();
                self.convertEscape()
            }
            // Java: default:
            _ => {
                // Java: this.escape.append(ch);
                self.escape.append_char(ch);
                // Java: return this.state;
                self.state
            }
        }
    }

    // Java: int parseEndTag(char ch)
    fn parseEndTag(&mut self, ch: char) -> i32 {
        // Java: switch (ch)
        match ch {
            // Java: case ':':
            ':' => {
                // Java: this.tagName.append(ch, true);
                self.tagName.append_char_spec(ch, true);
                // Java: return this.state;
                self.state
            }
            // Java: case '>':
            '>' => {
                // Java: this.addElementName();
                self.addElementName();
                // Java: this.reportEndElement();
                self.reportEndElement();
                // Java: return 6;
                6
            }
            // Java: default:
            _ => {
                // Java: this.tagName.append(ch);
                self.tagName.append_char(ch);
                // Java: return this.state;
                self.state
            }
        }
    }

    // Java: int parseCDATAStart(char ch)
    fn parseCDATAStart(&mut self, ch: char) -> i32 {
        // Java: switch (ch)
        match ch {
            // Java: case '[':
            '[' => {
                // Java: this.tagName.append(ch);
                self.tagName.append_char(ch);
                // Java: if (this.isCDATA())
                if self.isCDATA() {
                    // Java: return 16;
                    return 16;
                }
                // Java: this.tagName.clear();
                self.tagName.clear();
                // Java: return -1;
                -1
            }
            // Java: default:
            _ => {
                // Java: this.tagName.append(ch);
                self.tagName.append_char(ch);
                // Java: return this.state;
                self.state
            }
        }
    }

    // Java: int parseCDataContent(char ch)
    fn parseCDataContent(&mut self, ch: char) -> i32 {
        // Java: switch (ch)
        match ch {
            // Java: case '>':
            '>' => {
                // Java: if (this.lastSquareBracket == 2)
                if self.lastSquareBracket == 2 {
                    // Java: this.addText();
                    self.addText();
                    // Java: this.lastSquareBracket = 0;
                    self.lastSquareBracket = 0;
                    // Java: return 6;
                    return 6;
                }
                // Java: this.appendSquareBracket();
                self.appendSquareBracket();
                // Java: this.tagName.append(ch);
                self.tagName.append_char(ch);
                // Java: return this.state;
                self.state
            }
            // Java: case ']':
            ']' => {
                // Java: ++this.lastSquareBracket;
                self.lastSquareBracket += 1;
                // Java: return this.state;
                self.state
            }
            // Java: default:
            _ => {
                // Java: this.appendSquareBracket();
                self.appendSquareBracket();
                // Java: this.tagName.append(ch);
                self.tagName.append_char(ch);
                // Java: return this.state;
                self.state
            }
        }
    }

    // Java: void appendSquareBracket()
    fn appendSquareBracket(&mut self) {
        // Java: for(int i = 0; i < this.lastSquareBracket; ++i)
        for _i in 0..self.lastSquareBracket {
            // Java: this.tagName.append(']');
            self.tagName.append_char(']');
        }
        // Java: this.lastSquareBracket = 0;
        self.lastSquareBracket = 0;
    }

    // Java: int parseData(char ch)
    fn parseData(&mut self, ch: char) -> i32 {
        // Java: switch (ch)
        match ch {
            // Java: case '&':
            '&' => {
                // Java: this.escape.clear();
                self.escape.clear();
                // Java: this.preState = this.state;
                self.preState = self.state;
                // Java: return 12;
                12
            }
            // Java: case '<':
            '<' => {
                // Java: this.addText();
                self.addText();
                // Java: return 2;
                2
            }
            // Java: case '>':
            '>' => {
                // Java: return 6;
                6
            }
            // Java: default:
            _ => {
                // Java: this.tagName.append(ch);
                self.tagName.append_char(ch);
                // Java: return this.state;
                self.state
            }
        }
    }

    // Java: boolean isCDATA()
    fn isCDATA(&mut self) -> bool {
        // Java: if (this.tagName.length() > 0)
        if self.tagName.length() > 0 {
            // Java: String buffer = new String(this.tagName.getChar(), 0, this.tagName.length());
            let buffer = self.tagName.to_string();
            // Java: if ("[CDATA[".equals(buffer))
            if buffer == CDATA_START_VAL {
                // Java: this.tagName.clear();
                self.tagName.clear();
                // Java: return true;
                return true;
            }
        }
        // Java: return false;
        false
    }

    // Java: int parseTag(char ch)
    fn parseTag(&mut self, ch: char) -> i32 {
        // Java: switch (ch)
        match ch {
            // Java: case '!':
            '!' => {
                // Java: return 14;
                14
            }
            // Java: case '/':
            '/' => {
                // Java: return 5;
                5
            }
            // Java: case '<':
            '<' => {
                // Java: this.clearTag();
                self.clearTag();
                // Java: return 8;
                8
            }
            // Java: case '>':
            '>' => {
                // Java: return 6;
                6
            }
            // Java: case '?':
            '?' => {
                // Java: return -1;
                -1
            }
            // Java: default:
            _ => {
                // Java: if (!this.isSpace(ch))
                if !self.isSpace(ch) {
                    // Java: this.clearTag();
                    self.clearTag();
                    // Java: this.tagName.append(ch);
                    self.tagName.append_char(ch);
                    // Java: return 7;
                    return 7;
                }
                // Java: else
                else {
                    // Java: return this.state;
                    self.state
                }
            }
        }
    }

    // Java: int parseExclamation(char ch)
    fn parseExclamation(&mut self, ch: char) -> i32 {
        // Java: switch (ch)
        match ch {
            // Java: case '[':
            '[' => {
                // Java: this.tagName.append(ch);
                self.tagName.append_char(ch);
                // Java: return 15;
                15
            }
            // Java: default:
            _ => {
                // Java: return -1;
                -1
            }
        }
    }

    // Java: int parseInvalidTag(char ch)
    fn parseInvalidTag(&mut self, ch: char) -> i32 {
        // Java: switch (ch)
        match ch {
            // Java: case '>':
            '>' => {
                // Java: return 6;
                6
            }
            // Java: default:
            _ => {
                // Java: return this.state;
                self.state
            }
        }
    }

    // Java: int parseTagName(char ch)
    fn parseTagName(&mut self, ch: char) -> i32 {
        // Java: switch (ch)
        match ch {
            // Java: case '\n':
            // Java: case '\r':
            // Java: case ' ':
            '\n' | '\r' | ' ' => {
                // Java: if (this.tagName.length() > 0)
                if self.tagName.length() > 0 {
                    // Java: this.addElementName();
                    self.addElementName();
                    // Java: return 8;
                    return 8;
                }
                // Java: return this.state;
                self.state
            }
            // Java: case '!':
            // Java: case '/':
            // Java: case '?':
            '!' | '/' | '?' => {
                // Java: this.addElementName();
                self.addElementName();
                // Java: this.reportStartElement();
                self.reportStartElement();
                // Java: this.reportEndElement();
                self.reportEndElement();
                // Java: return 6;
                6
            }
            // Java: case ':':
            ':' => {
                // Java: this.tagName.append(ch, true);
                self.tagName.append_char_spec(ch, true);
                // Java: return this.state;
                self.state
            }
            // Java: case '<':
            '<' => {
                // Java: return 2;
                2
            }
            // Java: case '>':
            '>' => {
                // Java: this.addElementName();
                self.addElementName();
                // Java: this.reportStartElement();
                self.reportStartElement();
                // Java: return 6;
                6
            }
            // Java: default:
            _ => {
                // Java: this.tagName.append(ch);
                self.tagName.append_char(ch);
                // Java: return this.state;
                self.state
            }
        }
    }

    // Java: int parseTagAttributeName(char ch)
    fn parseTagAttributeName(&mut self, ch: char) -> i32 {
        // Java: switch (ch)
        match ch {
            // Java: case '\t':
            // Java: case '\n':
            // Java: case '\r':
            // Java: case ' ':
            '\t' | '\n' | '\r' | ' ' => {
                // Java: return this.state;
                self.state
            }
            // Java: case '!':
            // Java: case '/':
            // Java: case '?':
            '!' | '/' | '?' => {
                // Java: this.reportStartElement();
                self.reportStartElement();
                // Java: this.reportEndElement();
                self.reportEndElement();
                // Java: return 6;
                6
            }
            // Java: case ':':
            ':' => {
                // Java: this.tagName.append(ch, true);
                self.tagName.append_char_spec(ch, true);
                // Java: return this.state;
                self.state
            }
            // Java: case '<':
            '<' => {
                // Java: return 2;
                2
            }
            // Java: case '=':
            '=' => {
                // Java: this.addAttrName();
                self.addAttrName();
                // Java: return 9;
                9
            }
            // Java: case '>':
            '>' => {
                // Java: this.reportStartElement();
                self.reportStartElement();
                // Java: return 6;
                6
            }
            // Java: default:
            _ => {
                // Java: this.tagName.append(ch);
                self.tagName.append_char(ch);
                // Java: return this.state;
                self.state
            }
        }
    }

    // Java: int parseTagAttributeValueStart(char ch)
    fn parseTagAttributeValueStart(&mut self, ch: char) -> i32 {
        // Java: switch (ch)
        match ch {
            // Java: case '"':
            '"' => {
                // Java: return 10;
                10
            }
            // Java: case '\'':
            '\'' => {
                // Java: return 13;
                13
            }
            // Java: case '>':
            '>' => {
                // Java: this.reportStartElement();
                self.reportStartElement();
                // Java: return 6;
                6
            }
            // Java: default:
            _ => {
                // Java: return this.state;
                self.state
            }
        }
    }

    // Java: int parseDoubleQuotesTagAttrVal(char ch)
    fn parseDoubleQuotesTagAttrVal(&mut self, ch: char) -> i32 {
        // Java: switch (ch)
        match ch {
            // Java: case '"':
            '"' => {
                // Java: this.addAttribute();
                self.addAttribute();
                // Java: return 8;
                8
            }
            // Java: case '&':
            '&' => {
                // Java: this.escape.clear();
                self.escape.clear();
                // Java: this.preState = this.state;
                self.preState = self.state;
                // Java: return 12;
                12
            }
            // Java: case '>':
            '>' => {
                // Java: this.reportStartElement();
                self.reportStartElement();
                // Java: return 6;
                6
            }
            // Java: default:
            _ => {
                // Java: this.tagName.append(ch);
                self.tagName.append_char(ch);
                // Java: return this.state;
                self.state
            }
        }
    }

    // Java: int parseSingleQuotesTagAttrVal(char ch)
    fn parseSingleQuotesTagAttrVal(&mut self, ch: char) -> i32 {
        // Java: switch (ch)
        match ch {
            // Java: case '&':
            '&' => {
                // Java: this.escape.clear();
                self.escape.clear();
                // Java: this.preState = this.state;
                self.preState = self.state;
                // Java: return 12;
                12
            }
            // Java: case '\'':
            '\'' => {
                // Java: this.addAttribute();
                self.addAttribute();
                // Java: return 8;
                8
            }
            // Java: case '>':
            '>' => {
                // Java: this.reportStartElement();
                self.reportStartElement();
                // Java: return 6;
                6
            }
            // Java: default:
            _ => {
                // Java: this.tagName.append(ch);
                self.tagName.append_char(ch);
                // Java: return this.state;
                self.state
            }
        }
    }

    // Java: int convertEscape()
    fn convertEscape(&mut self) -> i32 {
        // Java: char chEsq = ' ';
        let mut chEsq: char = ' ';
        // Java: String temp = this.escape.toString();
        let temp = self.escape.to_string();
        // Java: if (temp.equals("amp"))
        if temp == "amp" {
            // Java: chEsq = '&';
            chEsq = '&';
        }
        // Java: else if (temp.equals("lt"))
        else if temp == "lt" {
            // Java: chEsq = '<';
            chEsq = '<';
        }
        // Java: else if (temp.equals("gt"))
        else if temp == "gt" {
            // Java: chEsq = '>';
            chEsq = '>';
        }
        // Java: else if (temp.equals("quot"))
        else if temp == "quot" {
            // Java: chEsq = '"';
            chEsq = '"';
        }
        // Java: else if (temp.equals("apos"))
        else if temp == "apos" {
            // Java: chEsq = '\'';
            chEsq = '\'';
        }
        // Java: else if (!temp.equals("#xA") && !temp.equals("#x0A"))
        else if temp != "#xA" && temp != "#x0A" {
            // Java: if (!temp.equals("#xD") && !temp.equals("#x0D"))
            if temp != "#xD" && temp != "#x0D" {
                // Java: if (temp.length() <= 0 || temp.charAt(0) != '#')
                if temp.is_empty() || temp.chars().next() != Some('#') {
                    // Java: throw new IllegalStateException();
                    panic!("IllegalStateException");
                }
                // Java: int radix = 10;
                let mut radix = 10u32;
                let mut num_str = &temp[1..];
                // Java: if (temp.length() > 1 && temp.charAt(1) == 'x')
                if temp.len() > 1 && temp.chars().nth(1) == Some('x') {
                    // Java: temp = temp.substring(2);
                    num_str = &temp[2..];
                    // Java: radix = 16;
                    radix = 16;
                }
                // Java: else
                else {
                    // Java: temp = temp.substring(1);
                    num_str = &temp[1..];
                }
                // Java: try
                // Java: chEsq = (char)Integer.parseInt(temp, radix);
                match i32::from_str_radix(num_str, radix) {
                    Ok(v) => chEsq = char::from_u32(v as u32).unwrap_or(' '),
                    // Java: catch (NumberFormatException var5)
                    Err(_) => {
                        // Java: throw new IllegalStateException();
                        panic!("IllegalStateException");
                    }
                }
            }
            // Java: else
            else {
                // Java: chEsq = '\r';
                chEsq = '\r';
            }
        }
        // Java: else
        else {
            // Java: chEsq = '\n';
            chEsq = '\n';
        }
        // Java: this.tagName.append(chEsq);
        self.tagName.append_char(chEsq);
        // Java: this.escape.clear();
        self.escape.clear();
        // Java: return this.preState;
        self.preState
    }

    // Java: boolean isPreserveSpace()
    fn isPreserveSpace(&self) -> bool {
        // Java: return this.xmlToken.attributes.isPreserveSpace();
        self.xmlToken.attributes.isPreserveSpace()
    }

    // Java: void clearTag()
    fn clearTag(&mut self) {
        // Java: this.tagName.clear();
        self.tagName.clear();
        // Java: this.xmlToken.clear();
        self.xmlToken.clear();
    }

    // Java: void addElementName()
    fn addElementName(&mut self) {
        // Java: this.xmlToken.addElementName(this.tagName, this.mMode);
        self.xmlToken.addElementName(&mut self.tagName, self.mMode);
        // Java: this.tagName.clear();
        self.tagName.clear();
    }

    // Java: void addAttrName()
    fn addAttrName(&mut self) {
        // Java: this.xmlToken.addAttrName(this.tagName, this.mMode);
        self.xmlToken.addAttrName(&mut self.tagName, self.mMode);
        // Java: this.tagName.clear();
        self.tagName.clear();
    }

    // Java: void addAttribute()
    fn addAttribute(&mut self) {
        // Java: this.xmlToken.addAttribute(this.tagName);
        self.xmlToken.addAttribute(&self.tagName);
        // Java: this.tagName.clear();
        self.tagName.clear();
    }

    // Java: void addText()
    fn addText(&mut self) {
        // Java: if (this.tagName.length() > 0)
        if self.tagName.length() > 0 {
            // Java: this.contentHandler.character(this.tagName.getChar(), 0, this.tagName.length());
            let len = self.tagName.length() as i32;
            self.contentHandler.character_buf(self.tagName.getChar(), 0, len);
            // Java: this.tagName.clear();
            self.tagName.clear();
        }
        // Java: else if (null != this.tagName.getPersistPath())
        else if self.tagName.getPersistPath().is_some() {
            // Java: String path = this.tagName.getPersistPath();
            let path = self.tagName.getPersistPath().unwrap().to_string();
            // Java: this.tagName.clear();
            self.tagName.clear();
            // Java: this.contentHandler.character(path);
            self.contentHandler.character_str(&path);
        }
    }

    // Java: void reportStartElement()
    fn reportStartElement(&mut self) {
        // Java: this.xmlToken.startElement();
        self.xmlToken.startElement();
        // Java: this.contentHandler.startElement(this.xmlToken.elementId, this.xmlToken.getElementNameSpacing(), this.xmlToken.localName, this.xmlToken.qualifiedName, this.xmlToken.attributes);
        let element_id = self.xmlToken.elementId;
        let ns = self.xmlToken.getElementNameSpacing().unwrap_or("").to_string();
        let local = self.xmlToken.localName.clone().unwrap_or_default();
        let qname = self.xmlToken.qualifiedName.clone().unwrap_or_default();
        let XmlParse {
            contentHandler,
            xmlToken,
            ..
        } = self;
        contentHandler.startElement(element_id, &ns, &local, &qname, &xmlToken.attributes);
    }

    // Java: void reportEndElement()
    fn reportEndElement(&mut self) {
        // Java: this.xmlToken.endElement();
        self.xmlToken.endElement();
        // Java: this.contentHandler.endElement(this.xmlToken.elementId, this.xmlToken.getElementNameSpacing(), this.xmlToken.localName, this.xmlToken.qualifiedName);
        let element_id = self.xmlToken.elementId;
        let ns = self.xmlToken.getElementNameSpacing().unwrap_or("").to_string();
        let local = self.xmlToken.localName.clone().unwrap_or_default();
        let qname = self.xmlToken.qualifiedName.clone().unwrap_or_default();
        self.contentHandler.endElement(element_id, &ns, &local, &qname);
    }

    // Java: boolean isSpace(char ch)
    fn isSpace(&self, ch: char) -> bool {
        // Java: switch (ch)
        match ch {
            // Java: case '\t':
            // Java: case '\n':
            // Java: case '\r':
            '\t' | '\n' | '\r' => {
                // Java: return true;
                true
            }
            // Java: case ' ':
            ' ' => {
                // Java: return 3 != this.mMode;
                3 != self.mMode
            }
            // Java: default:
            _ => {
                // Java: return false;
                false
            }
        }
    }
}
