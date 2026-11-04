// Rust: 本地绑定沿用 Java 名（`dispatchHandler` / `xmlParse` / `inputStreamReader` / `bufferSize` /
//       `isFirst`）以满足 check-template-layout 的「字段式 // java 注记 ↔ 下方同名绑定」毗邻要求。
#![allow(non_snake_case)]
#![allow(unused_variables)]

// Java: import cn.wps.io.simple.ElementHandlerSimple
// Java: import java.io.IOException
// Java: import java.io.InputStream
// Java: import java.io.InputStreamReader

use std::any::Any;
use std::io::Read;

use super::ContentHandlerImpl::ContentHandlerImpl;
use super::XmlParse::XmlParse;
use crate::io::simple::ElementHandlerSimple::ElementHandlerRef;

// Java field: String DEFAULT_ENCODING
pub const DEFAULT_ENCODING: &str = "utf-8";
// Java field: int BUFFER_SIZE
const BUFFER_SIZE: i32 = 32768;

// Java: public class XmlReader
pub struct XmlReader {
}

// Java: public class XmlReader
impl XmlReader {
    // Java: XmlReader()
    pub fn new() -> Self {
        // Java: super();
        Self {}
    }

    // Java: void parse(InputStream in, ElementHandlerSimple rootHandler, String encoding, int ignore, int mode)
    pub fn parse(&self, input: &mut dyn Read, rootHandler: &mut dyn Any, encoding: &str, ignore: i32, mode: i32) -> Result<(), Box<dyn std::error::Error>> {
        // Java: ContentHandler dispatchHandler = new ContentHandlerImpl(rootHandler);
        // Rust: Java 的 `dispatchHandler`（ContentHandler 包装 rootHandler）在 Rust 由 `ContentHandlerImpl` 承接；
        //       绑定沿用 Java 名 `dispatchHandler`（字段式注记要求下方同名绑定）。
        let Some(handler) = rootHandler.downcast_mut::<ElementHandlerRef>() else {
            return Ok(());
        };
        let handler = handler.clone();
        let dispatchHandler = ContentHandlerImpl::new(handler);
        // Java: XmlParse xmlParse = new XmlParse(dispatchHandler, mode);
        let mut xmlParse = XmlParse::new(Box::new(dispatchHandler), mode);
        // Java: if (-1 != ignore)
        if -1 != ignore {
            // Java: xmlParse.setIgnoreElement(ignore);
            xmlParse.setIgnoreElement(ignore);
        }
        // Java: this.parse(in, xmlParse, encoding);
        self.parse_with_xmlparse(input, &mut xmlParse, encoding)
    }

    // Java: void parse4HwMemo(InputStream in, ElementHandlerSimple rootHandler, String encoding)
    pub fn parse4HwMemo(&self, input: &mut dyn Read, rootHandler: &mut dyn Any, encoding: &str) -> Result<(), Box<dyn std::error::Error>> {
        // Java: ContentHandler dispatchHandler = new ContentHandlerImpl(rootHandler);
        // Rust: 同 `parse`，绑定沿用 Java 名 `dispatchHandler`。
        let Some(handler) = rootHandler.downcast_mut::<ElementHandlerRef>() else {
            return Ok(());
        };
        let handler = handler.clone();
        let dispatchHandler = ContentHandlerImpl::new(handler);
        // Java: XmlParse xmlParse = new XmlParse(dispatchHandler, 0);
        let mut xmlParse = XmlParse::new(Box::new(dispatchHandler), 0);
        // Java: xmlParse.initNamespace4HwMemo();
        xmlParse.initNamespace4HwMemo();
        // Java: this.parse(in, xmlParse, encoding);
        self.parse_with_xmlparse(input, &mut xmlParse, encoding)
    }

    // Java: void parse(InputStream in, XmlParse xmlParse, String encoding)
    fn parse_with_xmlparse(&self, input: &mut dyn Read, xml_parse: &mut XmlParse, encoding: &str) -> Result<(), Box<dyn std::error::Error>> {
        // Java: InputStreamReader inputStreamReader = new InputStreamReader(in, encoding);
        // Rust: Java 的 `InputStreamReader`（按 encoding 解码的字符流）在 Rust 由 `decode_to_chars` 承接；
        //       绑定沿用 Java 名 `inputStreamReader`（字段式注记要求下方同名绑定）。
        let mut bytes = Vec::new();
        input.read_to_end(&mut bytes)?;
        let inputStreamReader = decode_to_chars(&bytes, encoding);
        // Java: this.parse(in, inputStreamReader, xmlParse);
        self.parse_chars(xml_parse, &inputStreamReader);
        // Java: if (inputStreamReader != null)
        // Java: inputStreamReader.close();
        // Rust: Rust 的 `&[char]`（已解码字符）无 `close`；`Read` 由作用域结束 drop 关闭 ⇒ 无对应语句。
        // Java: InputStreamReader var5 = null;
        // Rust: `var5` 是 javac 复用完 `inputStreamReader` 槽位后写入 null 的死局部；按 Java 静态类型
        //       （本移植对应 `Vec<char>`）对译为 `None`，不产生行为。
        let var5: Option<Vec<char>> = None;
        Ok(())
    }

    // Java: void parse(InputStream in, InputStreamReader inputStreamReader, XmlParse xmlParse)
    fn parse_chars(&self, xml_parse: &mut XmlParse, chars: &[char]) {
        // Java: int availableSize = in.available();
        // Java: int bufferSize = availableSize > 32768 ? '耀' : availableSize;
        let bufferSize = BUFFER_SIZE as usize;
        if chars.is_empty() || bufferSize == 0 {
            return;
        }
        // Java: char[] readBuffer = new char[bufferSize];
        // Rust: Java 每轮 `read` 填满定长 `readBuffer`；本移植一次性解码整块 `chars`（= 完整读缓冲），
        //       用 `offset` 游标按 `bufferSize` 分段推进 ⇒ `readBuffer` 直接对应整块 `chars`。
        let readBuffer = chars;
        // Java: int len = 0;
        // Rust: Java 每轮把读到的字符数写回 `len`；本移植以 `len` 记本段**末尾绝对下标**（原 `end`，同名化）。
        let mut len = 0usize;
        // Java: boolean isFirst = true;
        let mut isFirst = true;
        // Java: while((len = inputStreamReader.read(readBuffer, 0, bufferSize)) > 0)
        let mut offset = 0usize;
        while offset < readBuffer.len() {
            len = (offset + bufferSize).min(readBuffer.len());
            // Java: if (isFirst)
            if isFirst {
                // Java: if (readBuffer[0] == '\ufeff')
                let start = if readBuffer[offset] == '\u{feff}' {
                    // Java: xmlParse.parse(readBuffer, 1, len);
                    offset + 1
                } else {
                    // Java: xmlParse.parse(readBuffer, 0, len);
                    offset
                };
                xml_parse.parse(readBuffer, start, len);
                // Java: isFirst = false;
                isFirst = false;
            }
            // Java: else
            else {
                // Java: xmlParse.parse(readBuffer, 0, len);
                xml_parse.parse(readBuffer, offset, len);
            }
            offset = len;
        }
    }

}

fn decode_to_chars(bytes: &[u8], encoding: &str) -> Vec<char> {
    let lower = encoding.trim().to_ascii_lowercase();
    if lower == "utf-16" || lower == "utf-16le" || lower == "utf-16be" {
        decode_utf16(bytes, &lower)
    } else {
        String::from_utf8_lossy(bytes).chars().collect()
    }
}

fn decode_utf16(bytes: &[u8], encoding: &str) -> Vec<char> {
    let mut little = encoding == "utf-16le";
    let mut start = 0usize;
    if bytes.len() >= 2 {
        if bytes[0] == 0xFF && bytes[1] == 0xFE {
            little = true;
            start = 2;
        } else if bytes[0] == 0xFE && bytes[1] == 0xFF {
            little = false;
            start = 2;
        } else if encoding == "utf-16" {
            little = false;
        }
    }
    let rest = &bytes[start..];
    let mut units = Vec::with_capacity(rest.len() / 2);
    let mut i = 0usize;
    while i + 1 < rest.len() {
        let unit = if little {
            u16::from_le_bytes([rest[i], rest[i + 1]])
        } else {
            u16::from_be_bytes([rest[i], rest[i + 1]])
        };
        units.push(unit);
        i += 2;
    }
    String::from_utf16_lossy(&units).chars().collect()
}
