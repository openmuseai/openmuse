// Java: import cn.wps.base.assertion.IOAssert
// Java: import cn.wps.base.log.IOLog
// Java: import java.io.IOException
// Java: import java.io.OutputStream
// Java: import java.nio.ByteBuffer
// Java: import java.nio.CharBuffer
// Java: import java.nio.charset.Charset
// Java: import java.util.ArrayList

use std::io::Write;

// Java: public class XmlWriter
pub struct XmlWriter {
    // Java field: String TAG
    // Java field: String MATH_ATTR_VAL
    pub(crate) math_attr_val: String,
    // Java field: OutputStream mDirectOutput
    pub(crate) mDirectOutput: Option<Box<dyn Write + Send>>,
    // Java field: String[] empty_strings
    // Java field: int SURROGATE_BEGIN
    // Java field: int SURROGATE_END
    // Java field: int UTF8_BUF_SIZE
    // Java field: byte[] utf8_buf
    pub(crate) utf8_buf: Vec<u8>,
    // Java field: int utf8_length
    pub(crate) utf8_length: usize,
    // Java field: int D_CHARBUF_SIZE
    // Java field: char[] d_char_buf
    pub(crate) d_char_buf: Vec<u8>,
    // Java field: char[] escape_lt
    // Java field: char[] escape_gt
    // Java field: char[] escape_amp
    // Java field: char[] escape_apos
    // Java field: char[] escape_quot
    // Java field: char[] escape_enter
    // Java field: char[] FACE_CHAR
    // Java field: Charset UTF8_CHARSET
}

const UTF8_BUF_SIZE: usize = 4096;
const D_CHARBUF_SIZE: usize = 512;
const ESCAPE_LT: &[u8] = b"&lt;";
const ESCAPE_GT: &[u8] = b"&gt;";
const ESCAPE_AMP: &[u8] = b"&amp;";
const ESCAPE_APOS: &[u8] = b"&apos;";
const ESCAPE_QUOT: &[u8] = b"&quot;";
const ESCAPE_ENTER: &[u8] = b"&#xA;";

// Java: public class XmlWriter
impl XmlWriter {
    // Java: XmlWriter(OutputStream os)
    pub fn new(os: Box<dyn Write + Send>) -> Self {
        // Java: super();
        // Java: IOAssert.assertNotNull("os should not be null", os);
        // Java: this.mDirectOutput = os;
        let mut w = Self {
            math_attr_val: "m:val".to_string(),
            mDirectOutput: Some(os),
            utf8_buf: vec![0u8; UTF8_BUF_SIZE],
            utf8_length: 0,
            d_char_buf: vec![0u8; D_CHARBUF_SIZE],
        };
        // Java: String header = "<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>\n";
        let header = "<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>\n";
        // Java: this.d_write_ascii(header);
        w.d_write_ascii_str(header);
        w
    }

    // Java: XmlWriter()
    pub fn new_empty() -> Self {
        // Java: super();
        Self {
            math_attr_val: "m:val".to_string(),
            mDirectOutput: None,
            utf8_buf: vec![0u8; UTF8_BUF_SIZE],
            utf8_length: 0,
            d_char_buf: vec![0u8; D_CHARBUF_SIZE],
        }
    }

    // Java: void dispose()
    fn dispose(&mut self) {
        // Java: this.mDirectOutput = null;
        self.mDirectOutput = None;
        // Java: this.utf8_buf = null;
        // Java: this.d_char_buf = null;
    }

    // Java: void close()
    pub fn close(&mut self) {
        // Java: this.d_flush();
        self.d_flush();
        // Java: this.dispose();
        self.dispose();
    }

    // Java: void startElementImpl(boolean bSimple, String qName, String[] attributes, int n)
    fn startElementImpl(&mut self, bSimple: bool, qName: &str, attributes: &[&str], mut n: usize) {
        // Java: if (attributes == null)
        {
            // Java: attributes = empty_strings;
        }
        // Java: if (n > attributes.length)
        {
            // Java: n = attributes.length;
            n = std::cmp::min(n, attributes.len());
        }
        // Java: this.d_write_ascii('<');
        self.d_write_ascii_char('<');
        // Java: this.d_write_ascii(qName);
        self.d_write_ascii_str(qName);
        // Java: for(int i = 0; i < n; i += 2)
        let mut i = 0;
        while i < n {
            // Java: String key = attributes[i];
            let key = attributes[i];
            // Java: if (key != null && key.length() != 0)
            if !key.is_empty() {
                // Java: String value = attributes[i + 1];
                let value = if i + 1 < attributes.len() { attributes[i + 1] } else { "" };
                // Java: IOAssert.assertNotNull("value should not be null.", value);
                // Java: this.d_write_ascii(' ');
                self.d_write_ascii_char(' ');
                // Java: this.d_write_ascii(key);
                self.d_write_ascii_str(key);
                // Java: this.d_write_ascii('=');
                self.d_write_ascii_char('=');
                // Java: this.d_write_ascii('"');
                self.d_write_ascii_char('"');
                // Java: this.d_write_escape(value);
                self.d_write_escape(value);
                // Java: this.d_write_ascii('"');
                self.d_write_ascii_char('"');
            }
            i += 2;
        }
        // Java: if (bSimple)
        if bSimple {
            // Java: this.d_write_ascii('/');
            self.d_write_ascii_char('/');
            // Java: this.d_write_ascii('>');
            self.d_write_ascii_char('>');
        }
        // Java: else
        else {
            // Java: this.d_write_ascii('>');
            self.d_write_ascii_char('>');
        }
    }

    // Java: void startElementAsciiImpl(boolean bSimple, String qName, String[] attributes, int n)
    fn startElementAsciiImpl(&mut self, bSimple: bool, qName: &str, attributes: &[&str], mut n: usize) {
        // Java: if (attributes == null)
        {
            // Java: attributes = empty_strings;
        }
        // Java: if (n > attributes.length)
        {
            // Java: n = attributes.length;
            n = std::cmp::min(n, attributes.len());
        }
        // Java: this.d_write_ascii('<');
        self.d_write_ascii_char('<');
        // Java: this.d_write_ascii(qName);
        self.d_write_ascii_str(qName);
        // Java: for(int i = 0; i < n; i += 2)
        let mut i = 0;
        while i < n {
            // Java: String key = attributes[i];
            let key = attributes[i];
            // Java: String value = attributes[i + 1];
            let value = if i + 1 < attributes.len() { attributes[i + 1] } else { "" };
            // Java: if (key != null && key.length() != 0)
            if !key.is_empty() {
                // Java: IOAssert.assertNotNull("value should not be null.", value);
                // Java: this.d_write_ascii(' ');
                self.d_write_ascii_char(' ');
                // Java: this.d_write_ascii(key);
                self.d_write_ascii_str(key);
                // Java: this.d_write_ascii('=');
                self.d_write_ascii_char('=');
                // Java: this.d_write_ascii('"');
                self.d_write_ascii_char('"');
                // Java: this.d_write_ascii(value);
                self.d_write_ascii_str(value);
                // Java: this.d_write_ascii('"');
                self.d_write_ascii_char('"');
            }
            i += 2;
        }
        // Java: if (bSimple)
        if bSimple {
            // Java: this.d_write_ascii('/');
            self.d_write_ascii_char('/');
            // Java: this.d_write_ascii('>');
            self.d_write_ascii_char('>');
        }
        // Java: else
        else {
            // Java: this.d_write_ascii('>');
            self.d_write_ascii_char('>');
        }
    }

    // Java: void startElement(String qName, String... attributes)
    pub fn startElement(&mut self, qName: &str, attributes: &[&str]) {
        // Java: this.startElementImpl(false, qName, attributes, attributes.length);
        let n = attributes.len();
        self.startElementImpl(false, qName, attributes, n);
    }

    // Java: void startElement(String qName, String[] attributes, int n)
    pub fn startElement_with_n(&mut self, qName: &str, attributes: &[&str], n: usize) {
        // Java: this.startElementImpl(false, qName, attributes, n);
        self.startElementImpl(false, qName, attributes, n);
    }

    // Java: void startElement(String qName, ArrayList<String> attributes)
    pub fn startElement_with_list(&mut self, qName: &str, attributes: &[String]) {
        // Java: IOAssert.assertNotNull("attributes should not be null", attributes);
        // Java: String[] a = new String[0];
        // Java: if (attributes != null)
        {
            // Java: a = (String[])attributes.toArray(a);
        }
        let refs: Vec<&str> = attributes.iter().map(|s| s.as_str()).collect();
        // Java: this.startElement(qName, a);
        self.startElement(qName, &refs);
    }

    // Java: void writeElement(String qName, ArrayList<String> attributes)
    pub fn writeElement_with_list(&mut self, qName: &str, attributes: &[String]) {
        // Java: IOAssert.assertNotNull("attributes should not be null", attributes);
        // Java: String[] a = new String[0];
        // Java: if (attributes != null)
        {
            // Java: a = (String[])attributes.toArray(a);
        }
        let refs: Vec<&str> = attributes.iter().map(|s| s.as_str()).collect();
        // Java: this.writeElement(qName, a);
        self.writeElement(qName, &refs);
    }

    // Java: void writeElement(String qName, String... attributes)
    pub fn writeElement(&mut self, qName: &str, attributes: &[&str]) {
        // Java: IOAssert.assertNotNull("attributes should not be null", attributes);
        // Java: this.startElementImpl(true, qName, attributes, attributes.length);
        let n = attributes.len();
        self.startElementImpl(true, qName, attributes, n);
    }

    // Java: void writeElementAscii(String qName, String... attributes)
    pub fn writeElementAscii(&mut self, qName: &str, attributes: &[&str]) {
        // Java: IOAssert.assertNotNull("attributes should not be null", attributes);
        // Java: this.startElementAsciiImpl(true, qName, attributes, attributes.length);
        let n = attributes.len();
        self.startElementAsciiImpl(true, qName, attributes, n);
    }

    // Java: void writeElement(String qName, String[] attributes, int n)
    pub fn writeElement_with_n(&mut self, qName: &str, attributes: &[&str], n: usize) {
        // Java: this.startElementImpl(true, qName, attributes, n);
        self.startElementImpl(true, qName, attributes, n);
    }

    // Java: void endElement(String qName)
    pub fn endElement(&mut self, qName: &str) {
        // Java: this.d_write_ascii('<');
        self.d_write_ascii_char('<');
        // Java: this.d_write_ascii('/');
        self.d_write_ascii_char('/');
        // Java: this.d_write_ascii(qName);
        self.d_write_ascii_str(qName);
        // Java: this.d_write_ascii('>');
        self.d_write_ascii_char('>');
    }

    // Java: void writeTextWithoutEscaping(String text)
    pub fn writeTextWithoutEscaping(&mut self, text: &str) {
        // Java: this.d_write_ascii(text);
        self.d_write_ascii_str(text);
    }

    // Java: void writeText(String text)
    pub fn writeText(&mut self, text: &str) {
        // Java: this.d_write_escape(text);
        self.d_write_escape(text);
    }

    // Java: void writeElement(String qName, boolean boolVal)
    pub fn writeElement_bool(&mut self, qName: &str, boolVal: bool) {
        // Java: if (boolVal)
        if boolVal {
            // Java: this.writeElement(qName);
            self.writeElement(qName, &[]);
        }
        // Java: else
        else {
            // Java: this.writeElement(qName, "w:val", "false");
            self.writeElement(qName, &["w:val", "false"]);
        }
    }

    // Java: void writeElement(String qName, String val)
    pub fn writeElement_str(&mut self, qName: &str, val: &str) {
        // Java: this.writeElement(qName, "w:val", val);
        self.writeElement(qName, &["w:val", val]);
    }

    // Java: void writeElement(String qName, int val)
    pub fn writeElement_int(&mut self, qName: &str, val: i32) {
        // Java: this.writeElement(qName, "w:val", Integer.toString(val));
        let val_str = val.to_string();
        self.writeElement(qName, &["w:val", &val_str]);
    }

    // Java: void writeMathElement(String qName, String val)
    pub fn writeMathElement(&mut self, qName: &str, val: &str) {
        // Java: this.writeElement(qName, this.MATH_ATTR_VAL, val);
        let attr = self.math_attr_val.clone();
        self.writeElement(qName, &[&attr, val]);
    }

    // Java: void writeMathElement(String qName, int val)
    pub fn writeMathElement_int(&mut self, qName: &str, val: i32) {
        // Java: this.writeMathElement(qName, Integer.toString(val));
        let val_str = val.to_string();
        self.writeMathElement(qName, &val_str);
    }

    // Java: void writeMathElement(String qName, char val)
    pub fn writeMathElement_char(&mut self, qName: &str, val: char) {
        // Java: this.writeMathElement(qName, String.valueOf(val));
        let val_str = val.to_string();
        self.writeMathElement(qName, &val_str);
    }

    // Java: void writeMathBooleanToInt(String qName, boolean boolVal)
    pub fn writeMathBooleanToInt(&mut self, qName: &str, boolVal: bool) {
        // Java: if (boolVal)
        if boolVal {
            // Java: this.writeMathElement(qName, "1");
            self.writeMathElement(qName, "1");
        }
        // Java: else
        else {
            // Java: this.writeMathElement(qName, "0");
            self.writeMathElement(qName, "0");
        }
    }

    // Java: void writeMathBooleanToOnOff(String qName, boolean boolVal)
    pub fn writeMathBooleanToOnOff(&mut self, qName: &str, boolVal: bool) {
        // Java: if (boolVal)
        if boolVal {
            // Java: this.writeMathElement(qName, "on");
            self.writeMathElement(qName, "on");
        }
        // Java: else
        else {
            // Java: this.writeMathElement(qName, "off");
            self.writeMathElement(qName, "off");
        }
    }

    // Java: void d_flush()
    pub fn d_flush(&mut self) {
        // Java: if (this.utf8_length > 0)
        if self.utf8_length > 0 {
            // Java: this.flush();
            self.flush();
        }
    }

    // Java: void flush()
    pub fn flush(&mut self) {
        // Java: try
        if let Some(ref mut output) = self.mDirectOutput {
            // Java: this.mDirectOutput.write(this.utf8_buf, 0, this.utf8_length);
            let _ = output.write_all(&self.utf8_buf[..self.utf8_length]);
            // Java: this.utf8_length = 0;
            self.utf8_length = 0;
        }
        // Java: catch (IOException e)
        // Java: IOLog.e(TAG, "IOException", e);
        // Java: throw new RuntimeException(e);
    }

    // Java: void d_write_ascii(char c)
    fn d_write_ascii_char(&mut self, c: char) {
        // Java: this.ensureCapacity();
        self.ensureCapacity();
        // Java: this.utf8_buf[this.utf8_length++] = (byte)c;
        self.utf8_buf[self.utf8_length] = c as u8;
        self.utf8_length += 1;
    }

    // Java: void writeBuf()
    pub fn writeBuf(&mut self) {
        // Java: try
        if let Some(ref mut output) = self.mDirectOutput {
            // Java: this.mDirectOutput.write(this.utf8_buf);
            let _ = output.write_all(&self.utf8_buf);
            // Java: this.utf8_length = 0;
            self.utf8_length = 0;
        }
        // Java: catch (IOException e)
        // Java: IOLog.e(TAG, "IOException", e);
        // Java: throw new RuntimeException(e);
    }

    // Java: void d_write_ascii(char[] buf, int start, int end)
    fn d_write_ascii_buf(&mut self, buf: &[u8], mut start: usize, end: usize) {
        // Java: while(start < end)
        while start < end {
            // Java: int n = end - start;
            let mut n = end - start;
            // Java: int nCap = 4096 - this.utf8_length;
            let n_cap = UTF8_BUF_SIZE - self.utf8_length;
            // Java: if (n > nCap)
            if n > n_cap {
                // Java: n = nCap;
                n = n_cap;
            }
            // Java: if (n == 0)
            if n == 0 {
                // Java: this.writeBuf();
                self.writeBuf();
            }
            // Java: else
            else {
                // Java: byte[] _utf8_buf = this.utf8_buf;
                // Java: int _utf8_length;
                // Java: for(_utf8_length = this.utf8_length; n-- > 0; _utf8_buf[_utf8_length++] = (byte)buf[start++])
                for _ in 0..n {
                    self.utf8_buf[self.utf8_length] = buf[start];
                    self.utf8_length += 1;
                    start += 1;
                }
                // Java: this.utf8_length = _utf8_length;
            }
        }
    }

    // Java: void d_write_ascii(String str)
    fn d_write_ascii_str(&mut self, str: &str) {
        // Java: int len;
        // Java: if (str != null && (len = str.length()) != 0)
        if !str.is_empty() {
            let len = str.len();
            // Java: int n;
            let mut n;
            // Java: for(int i = 0; i < len; i += n)
            let mut i = 0;
            while i < len {
                // Java: n = len - i;
                n = len - i;
                // Java: if (n > 512)
                if n > D_CHARBUF_SIZE {
                    // Java: n = 512;
                    n = D_CHARBUF_SIZE;
                }
                // Java: str.getChars(i, i + n, this.d_char_buf, 0);
                // Java: this.d_write_ascii(this.d_char_buf, 0, n);
                let bytes = &str.as_bytes()[i..i + n];
                self.d_write_ascii_buf(bytes, 0, n);
                i += n;
            }
        }
    }

    // Java: void d_write(char cc)
    fn d_write(&mut self, cc: char) {
        // Java: if (cc >= ' ' || cc == '\t' || cc == '\n' || cc == '\r')
        if cc >= ' ' || cc == '\t' || cc == '\n' || cc == '\r' {
            // Java: if (cc != '\uffff')
            if cc != '\u{FFFF}' {
                // Java: this.ensureCapacity();
                // Java: if (cc < 128)
                if (cc as u32) < 128 {
                    self.ensureCapacity();
                    // Java: this.utf8_buf[this.utf8_length++] = (byte)cc;
                    self.utf8_buf[self.utf8_length] = cc as u8;
                    self.utf8_length += 1;
                }
                // Java: else if (cc < 2048)
                else if (cc as u32) < 2048 {
                    self.ensureCapacity();
                    // Java: this.utf8_buf[this.utf8_length++] = (byte)(cc >> 6 | 192);
                    self.utf8_buf[self.utf8_length] = ((cc as u32 >> 6) | 192) as u8;
                    self.utf8_length += 1;
                    // Java: this.ensureCapacity();
                    self.ensureCapacity();
                    // Java: this.utf8_buf[this.utf8_length++] = (byte)(cc & 63 | 128);
                    self.utf8_buf[self.utf8_length] = ((cc as u32 & 63) | 128) as u8;
                    self.utf8_length += 1;
                }
                // Java: else if (cc >= '\ud800' && cc < '\ue000')
                else if cc as u32 >= 0xD800 && (cc as u32) < 0xE000 {
                    self.ensureCapacity();
                    // Java: this.utf8_buf[this.utf8_length++] = 32;
                    self.utf8_buf[self.utf8_length] = 32;
                    self.utf8_length += 1;
                }
                // Java: else
                else {
                    self.ensureCapacity();
                    // Java: this.utf8_buf[this.utf8_length++] = (byte)(cc >> 12 | 224);
                    self.utf8_buf[self.utf8_length] = ((cc as u32 >> 12) | 224) as u8;
                    self.utf8_length += 1;
                    // Java: this.ensureCapacity();
                    self.ensureCapacity();
                    // Java: this.utf8_buf[this.utf8_length++] = (byte)(128 | cc >> 6 & 63);
                    self.utf8_buf[self.utf8_length] = (128 | (cc as u32 >> 6) & 63) as u8;
                    self.utf8_length += 1;
                    // Java: this.ensureCapacity();
                    self.ensureCapacity();
                    // Java: this.utf8_buf[this.utf8_length++] = (byte)(cc & 63 | 128);
                    self.utf8_buf[self.utf8_length] = ((cc as u32 & 63) | 128) as u8;
                    self.utf8_length += 1;
                }
            }
        }
    }

    // Java: void ensureCapacity()
    fn ensureCapacity(&mut self) {
        // Java: if (this.utf8_length >= 4096)
        if self.utf8_length >= UTF8_BUF_SIZE {
            // Java: try
            if let Some(ref mut output) = self.mDirectOutput {
                // Java: this.mDirectOutput.write(this.utf8_buf);
                let _ = output.write_all(&self.utf8_buf);
                // Java: this.utf8_length = 0;
                self.utf8_length = 0;
            }
            // Java: catch (IOException e)
            // Java: throw new RuntimeException(e);
        }
    }

    // Java: void d_write_escape(String str)
    fn d_write_escape(&mut self, str: &str) {
        // Java: int len;
        // Java: if (str != null && (len = str.length()) != 0)
        if !str.is_empty() {
            let chars: Vec<char> = str.chars().collect();
            let len = chars.len();
            // Java: int i = 0;
            let mut i = 0;
            // Java: while(i < len)
            while i < len {
                // Java: char c = str.charAt(i++);
                let c = chars[i];
                i += 1;
                // Java: if (this.isEmoji(i, len, c, str))
                if i < len && c.is_ascii() == false && Self::is_high_surrogate(c) && Self::is_low_surrogate(chars[i]) {
                    // Java: this.d_write_emoji(c, str.charAt(i++));
                    let low = chars[i];
                    i += 1;
                    self.d_write_emoji(c, low);
                }
                // Java: else
                else {
                    // Java: this.d_write_singleEscapeChar2(c);
                    self.d_write_singleEscapeChar2(c);
                }
            }
        }
    }

    // Java: boolean isEmoji(int start, int end, char c, String str)
    fn is_high_surrogate(c: char) -> bool {
        // Java: return Character.isHighSurrogate(c) && start < end && Character.isLowSurrogate(str.charAt(start));
        let v = c as u32;
        v >= 0xD800 && v <= 0xDBFF
    }

    fn is_low_surrogate(c: char) -> bool {
        let v = c as u32;
        v >= 0xDC00 && v <= 0xDFFF
    }

    // Java: void d_write_emoji(char high, char low)
    fn d_write_emoji(&mut self, high: char, low: char) {
        // Java: FACE_CHAR[0] = high;
        // Java: FACE_CHAR[1] = low;
        // Java: ByteBuffer emojiBuf = UTF8_CHARSET.encode(CharBuffer.wrap(FACE_CHAR));
        // Java: byte[] emojiByteArr = new byte[emojiBuf.limit()];
        // Java: emojiBuf.get(emojiByteArr);
        // Java: int len = emojiByteArr.length;
        let code_point = ((high as u32 - 0xD800) << 10) + (low as u32 - 0xDC00) + 0x10000;
        if let Some(ch) = char::from_u32(code_point) {
            let mut buf = [0u8; 4];
            let encoded = ch.encode_utf8(&mut buf);
            // Java: for(int i = 0; i < len; ++i)
            for b in encoded.bytes() {
                // Java: this.ensureCapacity();
                self.ensureCapacity();
                // Java: this.utf8_buf[this.utf8_length++] = emojiByteArr[i];
                self.utf8_buf[self.utf8_length] = b;
                self.utf8_length += 1;
            }
        }
    }

    // Java: boolean isEmoji(int start, int end, char c, char[] buf)
    // Java: return Character.isHighSurrogate(c) && start < end && Character.isLowSurrogate(buf[start]);

    // Java: void d_write_singleEscapeChar2(char c)
    fn d_write_singleEscapeChar2(&mut self, c: char) {
        // Java: switch (c)
        match c {
            // Java: case '\n':
            '\n' => {
                // Java: this.d_write_ascii(escape_enter, 0, escape_enter.length);
                self.d_write_ascii_buf(ESCAPE_ENTER, 0, ESCAPE_ENTER.len());
            }
            // Java: case '\r':
            '\r' => {
                // Java: break;
            }
            // Java: case '"':
            '"' => {
                // Java: this.d_write_ascii(escape_quot, 0, escape_quot.length);
                self.d_write_ascii_buf(ESCAPE_QUOT, 0, ESCAPE_QUOT.len());
                // Java: break;
            }
            // Java: case '&':
            '&' => {
                // Java: this.d_write_ascii(escape_amp, 0, escape_amp.length);
                self.d_write_ascii_buf(ESCAPE_AMP, 0, ESCAPE_AMP.len());
                // Java: break;
            }
            // Java: case '\'':
            '\'' => {
                // Java: this.d_write_ascii(escape_apos, 0, escape_apos.length);
                self.d_write_ascii_buf(ESCAPE_APOS, 0, ESCAPE_APOS.len());
                // Java: break;
            }
            // Java: case '<':
            '<' => {
                // Java: this.d_write_ascii(escape_lt, 0, escape_lt.length);
                self.d_write_ascii_buf(ESCAPE_LT, 0, ESCAPE_LT.len());
                // Java: break;
            }
            // Java: case '>':
            '>' => {
                // Java: this.d_write_ascii(escape_gt, 0, escape_gt.length);
                self.d_write_ascii_buf(ESCAPE_GT, 0, ESCAPE_GT.len());
                // Java: break;
            }
            // Java: default:
            _ => {
                // Java: this.d_write(c);
                self.d_write(c);
            }
        }
    }

    // Java: void d_write_singleEscapeChar(char c)
    fn d_write_singleEscapeChar(&mut self, c: char) {
        // Java: switch (c)
        match c {
            // Java: case '\n':
            '\n' => {
                // Java: this.d_write_ascii(escape_enter, 0, escape_enter.length);
                self.d_write_ascii_buf(ESCAPE_ENTER, 0, ESCAPE_ENTER.len());
                // Java: break;
            }
            // Java: case '"':
            '"' => {
                // Java: this.d_write_ascii(escape_quot, 0, escape_quot.length);
                self.d_write_ascii_buf(ESCAPE_QUOT, 0, ESCAPE_QUOT.len());
                // Java: break;
            }
            // Java: case '&':
            '&' => {
                // Java: this.d_write_ascii(escape_amp, 0, escape_amp.length);
                self.d_write_ascii_buf(ESCAPE_AMP, 0, ESCAPE_AMP.len());
                // Java: break;
            }
            // Java: case '\'':
            '\'' => {
                // Java: this.d_write_ascii(escape_apos, 0, escape_apos.length);
                self.d_write_ascii_buf(ESCAPE_APOS, 0, ESCAPE_APOS.len());
                // Java: break;
            }
            // Java: case '<':
            '<' => {
                // Java: this.d_write_ascii(escape_lt, 0, escape_lt.length);
                self.d_write_ascii_buf(ESCAPE_LT, 0, ESCAPE_LT.len());
                // Java: break;
            }
            // Java: case '>':
            '>' => {
                // Java: this.d_write_ascii(escape_gt, 0, escape_gt.length);
                self.d_write_ascii_buf(ESCAPE_GT, 0, ESCAPE_GT.len());
                // Java: break;
            }
            // Java: default:
            _ => {
                // Java: this.d_write(c);
                self.d_write(c);
            }
        }
    }

}
