#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

// Java: import cn.wps.base.assertion.IOAssert
// Java: import cn.wps.base.log.IOLog
// Java: import cn.wps.core.runtime.Platform
// Java: import cn.wps.moffice.kfs.File
// Java: import cn.wps.moffice.kfs.FileOutputStream
// Java: import cn.wps.moffice.util.FileUtil
// Java: import cn.wps.util.IOUtils

use std::io::Write;

// Java field: String TAG
const TAG: Option<&str> = None;
// Java field: int MEMORY_LIMIT
const MEMORY_LIMIT: i32 = 52428800;

// Java: public class XmlStringBuilder
pub struct XmlStringBuilder {
    // Java field: String mPersistPath
    pub(crate) mPersistPath: Option<String>,
    // Java field: BufferedOutputStream mPersistOs
    mPersistOs: Option<Box<dyn Write + Send>>,
    // Java field: char[] mBuffer
    mBuffer: Vec<char>,
    // Java field: int mSize
    mSize: usize,
    // Java field: int mTop
    mTop: usize,
    // Java field: int mURISpe
    mURISpe: usize,
}

// Java: public class XmlStringBuilder
impl XmlStringBuilder {
    // Java: XmlStringBuilder()
    pub fn new() -> Self {
        // Java: this(32);
        Self::new_with_size(32)
    }

    // Java: XmlStringBuilder(int size)
    pub fn new_with_size(size: usize) -> Self {
        // Java: super();
        // Java: this.mBuffer = new char[size];
        // Java: this.mSize = size;
        // Java: this.mTop = 0;
        // Java: this.mURISpe = 0;
        // Java: this.mPersistOs = null;
        // Java: this.mPersistPath = null;
        XmlStringBuilder {
            mPersistPath: None,
            mPersistOs: None,
            mBuffer: vec!['\0'; size],
            mSize: size,
            mTop: 0,
            mURISpe: 0,
        }
    }

    // Java: XmlStringBuilder(XmlStringBuilder strBuilder)
    pub fn new_from(other: &XmlStringBuilder) -> Self {
        // Java: super();
        // Java: this.mSize = strBuilder.mSize;
        // Java: this.mTop = strBuilder.mTop;
        // Java: this.mURISpe = strBuilder.mURISpe;
        // Java: this.mPersistOs = strBuilder.mPersistOs;
        // Java: this.mPersistPath = strBuilder.mPersistPath;
        // Java: this.mBuffer = new char[strBuilder.mBuffer.length];
        // Java: System.arraycopy(strBuilder.mBuffer, 0, this.mBuffer, 0, strBuilder.mBuffer.length);
        XmlStringBuilder {
            mPersistPath: other.mPersistPath.clone(),
            mPersistOs: None,
            mBuffer: other.mBuffer.clone(),
            mSize: other.mSize,
            mTop: other.mTop,
            mURISpe: other.mURISpe,
        }
    }

    // Java: void append(char ch)
    pub fn append_char(&mut self, ch: char) {
        // Java: this.ensureCapacity();
        self.ensureCapacity();
        // Java: if (null == this.mPersistPath)
        if self.mPersistPath.is_none() {
            // Java: this.mBuffer[this.mTop++] = ch;
            if self.mTop < self.mBuffer.len() {
                self.mBuffer[self.mTop] = ch;
            }
            self.mTop += 1;
        }
        // Java: else
        else {
            // Java: this.appendByFile(ch);
            self.appendByFile(ch);
        }
    }

    // Java: void appendByFile(char ch)
    fn appendByFile(&mut self, ch: char) {
        // Java: try
        {
            // Java: if (null != this.mPersistOs)
            if let Some(ref mut os) = self.mPersistOs {
                // Java: this.mPersistOs.write((byte)ch);
                let _ = os.write(&[ch as u8]);
            }
        }
        // Java: catch (IOException e)
        {
            // Java: IOLog.e(TAG, "IOException", e);
        }
    }

    // Java: void ensureCapacity()
    fn ensureCapacity(&mut self) {
        // Java: if (this.mTop >= this.mSize)
        if self.mTop >= self.mSize {
            // Java: this.grow();
            self.grow();
        }
    }

    // Java: void grow()
    fn grow(&mut self) {
        // Java: int newSize = this.mSize * 2;
        let new_size = self.mSize * 2;
        // Java: if (newSize >= 52428800)
        if new_size >= MEMORY_LIMIT as usize {
            // Java: this.growByFile();
            self.growByFile();
            // Java: this.clearBuffer();
            self.clearBuffer();
        }
        // Java: else
        else {
            // Java: char[] tmp = new char[newSize];
            let mut tmp = vec!['\0'; new_size];
            // Java: System.arraycopy(this.mBuffer, 0, tmp, 0, this.mTop);
            tmp[..self.mTop].copy_from_slice(&self.mBuffer[..self.mTop]);
            // Java: this.mBuffer = tmp;
            self.mBuffer = tmp;
            // Java: this.mSize = newSize;
            self.mSize = new_size;
        }
    }

    // Java: void growByFile()
    fn growByFile(&mut self) {
        // Java: try
        {
            // Java: File file = Platform.createTempFile("xml_chars", "persist");
            // Java: if (null == file)
            {
                // Java: return;
            }
            // Java: this.mPersistPath = file.getAbsolutePath();
            // Java: this.mPersistOs = new BufferedOutputStream(new FileOutputStream(this.mPersistPath));
            // Java: byte[] bytes = IOUtils.getBytes(this.mBuffer, 0, this.mTop);
            // Java: this.mPersistOs.write(bytes);
        }
        // Java: catch (IOException e)
        {
            // Java: IOLog.e(TAG, "IOException", e);
        }
    }

    // Java: void append(char ch, boolean isSpec)
    pub fn append_char_spec(&mut self, ch: char, _is_spec: bool) {
        // Java: this.mURISpe = this.mTop;
        self.mURISpe = self.mTop;
        // Java: this.append(ch);
        self.append_char(ch);
    }

    // Java: int length()
    pub fn length(&self) -> usize {
        // Java: return this.mTop;
        self.mTop
    }

    // Java: int prefixLength()
    pub fn prefixLength(&self) -> usize {
        // Java: return this.mURISpe;
        self.mURISpe
    }

    // Java: char charAt(int index)
    pub fn charAt(&self, index: usize) -> char {
        // Java: return this.mBuffer[index];
        self.mBuffer[index]
    }

    // Java: void clear()
    pub fn clear(&mut self) {
        // Java: this.clearBuffer();
        self.clearBuffer();
        // Java: this.mPersistPath = null;
        self.mPersistPath = None;
        // Java: FileUtil.closeQuietly(this.mPersistOs);
        // Java: this.mPersistOs = null;
        self.mPersistOs = None;
    }

    // Java: void clearBuffer()
    fn clearBuffer(&mut self) {
        // Java: this.mTop = 0;
        self.mTop = 0;
        // Java: this.mURISpe = 0;
        self.mURISpe = 0;
    }

    // Java: int getPrefixHashCode(int defaultValue)
    pub fn getPrefixHashCode_default(&self, default_value: i32) -> i32 {
        // Java: int pos = index((char[])this.mBuffer, 0, this.mTop, ':');
        let pos = Self::index_char(&self.mBuffer, 0, self.mTop, ':');
        // Java: return pos == -1 ? defaultValue : hashcode((char[])this.mBuffer, 0, pos);
        if pos == -1 { default_value } else { Self::hashcode_char(&self.mBuffer, 0, pos as usize) }
    }

    // Java: int getPrefixHashCode()
    pub fn getPrefixHashCode(&self) -> i32 {
        // Java: return hashcode((char[])this.mBuffer, 0, this.mURISpe);
        Self::hashcode_char(&self.mBuffer, 0, self.mURISpe)
    }

    // Java: int getNameHashCode()
    pub fn getNameHashCode(&self) -> i32 {
        // Java: int start = 0;
        let mut start: usize = 0;
        // Java: if (this.mURISpe != 0)
        if self.mURISpe != 0 {
            // Java: start = this.mURISpe + 1;
            start = self.mURISpe + 1;
        }
        // Java: int end = this.mTop;
        let end = self.mTop;
        // Java: char[] str = this.mBuffer;
        let str = &self.mBuffer;
        // Java: if (str != null && str.length >= end && start >= 0)
        if str.len() >= end {
            // Java: int h;
            let mut h: i32 = 0;
            // Java: for(h = 0; start < end; h = 31 * h + str[start++])
            while start < end {
                h = h.wrapping_mul(31).wrapping_add(str[start] as i32);
                start += 1;
            }
            // Java: return h;
            h
        }
        // Java: else
        else {
            // Java: return 0;
            0
        }
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
        // Java: return this.getHashCode();
        self.getHashCode()
    }

    // Java: int getHashCode()
    pub fn getHashCode(&self) -> i32 {
        // Java: return hashcode((char[])this.mBuffer, 0, this.mTop);
        Self::hashcode_char(&self.mBuffer, 0, self.mTop)
    }

    // Java: char getEscape(char defaultValue)
    pub fn getEscape(&self, default_value: char) -> char {
        // Java: char[] val = this.mBuffer;
        let val = &self.mBuffer;
        // Java: switch (this.mTop)
        match self.mTop {
            // Java: case 2:
            2 => {
                // Java: if (val[0] == 'l' && val[1] == 't')
                if val[0] == 'l' && val[1] == 't' {
                    // Java: return '<';
                    return '<';
                }
                // Java: if (val[0] == 'g' && val[1] == 't')
                if val[0] == 'g' && val[1] == 't' {
                    // Java: return '>';
                    return '>';
                }
                // Java: break;
            }
            // Java: case 3:
            3 => {
                // Java: if (val[0] == 'a' && val[1] == 'm' && val[2] == 'p')
                if val[0] == 'a' && val[1] == 'm' && val[2] == 'p' {
                    // Java: return '&';
                    return '&';
                }
                // Java: break;
            }
            // Java: case 4:
            4 => {
                // Java: if (val[0] == 'q' && val[1] == 'u' && val[2] == 'o' && val[3] == 't')
                if val[0] == 'q' && val[1] == 'u' && val[2] == 'o' && val[3] == 't' {
                    // Java: return '"';
                    return '"';
                }
                // Java: if (val[0] == 'a' && val[1] == 'p' && val[2] == 'o' && val[3] == 's')
                if val[0] == 'a' && val[1] == 'p' && val[2] == 'o' && val[3] == 's' {
                    // Java: return '\'';
                    return '\'';
                }
            }
            _ => {}
        }
        // Java: return defaultValue;
        default_value
    }

    // Java: String getPrefixName()
    pub fn getPrefixName(&self) -> String {
        // Java: return new String(this.mBuffer, 0, this.mURISpe);
        self.mBuffer[..self.mURISpe].iter().collect()
    }

    // Java: boolean isPrefixEqual(String str)
    pub fn isPrefixEqual(&self, s: &str) -> bool {
        // Java: if (this.prefixLength() != str.length())
        if self.prefixLength() != s.len() {
            // Java: return false;
            return false;
        }
        // Java: else
        else {
            // Java: for(int i = 0; i < this.mURISpe; ++i)
            for i in 0..self.mURISpe {
                // Java: if (str.charAt(i) != this.mBuffer[i])
                if s.chars().nth(i) != Some(self.mBuffer[i]) {
                    // Java: return false;
                    return false;
                }
            }
            // Java: return true;
            true
        }
    }

    // Java: String getName()
    pub fn getName(&self) -> String {
        // Java: int start = this.mURISpe == 0 ? 0 : this.mURISpe + 1;
        let start = if self.mURISpe == 0 { 0 } else { self.mURISpe + 1 };
        // Java: return new String(this.mBuffer, start, this.mTop - start);
        self.mBuffer[start..self.mTop].iter().collect()
    }

    // Java: XmlStringBuilder backup()
    pub fn backup(&self) -> XmlStringBuilder {
        // Java: XmlStringBuilder obj = new XmlStringBuilder(this.mTop);
        let mut obj = XmlStringBuilder::new_with_size(self.mTop);
        // Java: System.arraycopy(this.mBuffer, 0, obj.mBuffer, 0, this.mTop);
        obj.mBuffer[..self.mTop].copy_from_slice(&self.mBuffer[..self.mTop]);
        // Java: obj.mTop = this.mTop;
        obj.mTop = self.mTop;
        // Java: return obj;
        obj
    }

    // Java: void restore(XmlStringBuilder obj)
    pub fn restore(&mut self, obj: &XmlStringBuilder) {
        // Java: if (obj.mTop > this.mSize)
        if obj.mTop > self.mSize {
            // Java: int newSize = obj.mTop * 2;
            let new_size = obj.mTop * 2;
            // Java: this.mBuffer = new char[newSize];
            self.mBuffer = vec!['\0'; new_size];
            // Java: this.mSize = newSize;
            self.mSize = new_size;
        }
        // Java: System.arraycopy(obj.mBuffer, 0, this.mBuffer, 0, obj.mTop);
        self.mBuffer[..obj.mTop].copy_from_slice(&obj.mBuffer[..obj.mTop]);
        // Java: this.mTop = obj.mTop;
        self.mTop = obj.mTop;
        // Java: this.mURISpe = obj.mURISpe;
        self.mURISpe = obj.mURISpe;
    }

    // Java: String toString()
    pub fn to_string(&self) -> String {
        // Java: return new String(this.mBuffer, 0, this.mTop);
        self.mBuffer[..self.mTop].iter().collect()
    }

    // Java: static int index(char[] str, int start, int end, char ch)
    pub fn index_char(str: &[char], mut start: usize, end: usize, ch: char) -> i32 {
        // Java: if (str != null && str.length >= end && start >= 0)
        if str.len() >= end {
            // Java: while(start < end)
            while start < end {
                // Java: if (str[start] == ch)
                if str[start] == ch {
                    // Java: return start;
                    return start as i32;
                }
                // Java: ++start;
                start += 1;
            }
            // Java: return -1;
            -1
        }
        // Java: else
        else {
            // Java: return -1;
            -1
        }
    }

    // Java: static int index(byte[] str, int start, int end, char ch)
    pub fn index_byte(str: &[u8], mut start: usize, end: usize, ch: char) -> i32 {
        // Java: if (str != null && str.length >= end && start >= 0)
        if str.len() >= end {
            // Java: while(start < end)
            while start < end {
                // Java: if (str[start] == ch)
                if str[start] == ch as u8 {
                    // Java: return start;
                    return start as i32;
                }
                // Java: ++start;
                start += 1;
            }
            // Java: return -1;
            -1
        }
        // Java: else
        else {
            // Java: return -1;
            -1
        }
    }

    // Java: static int hashcode(char[] str, int start, int end)
    pub fn hashcode_char(str: &[char], mut start: usize, end: usize) -> i32 {
        // Java: IOAssert.assertTrue(start <= end);
        assert!(start <= end);
        // Java: if (str != null && str.length >= end && start >= 0)
        if str.len() >= end {
            // Java: int h;
            let mut h: i32 = 0;
            // Java: for(h = 0; start < end; h = 31 * h + str[start++])
            while start < end {
                h = h.wrapping_mul(31).wrapping_add(str[start] as i32);
                start += 1;
            }
            // Java: return h;
            h
        }
        // Java: else
        else {
            // Java: return 0;
            0
        }
    }

    // Java: static int hashcode(byte[] str, int start, int end)
    pub fn hashcode_byte(str: &[u8], mut start: usize, end: usize) -> i32 {
        // Java: if (str != null && str.length >= end && start >= 0)
        if str.len() >= end {
            // Java: int h;
            let mut h: i32 = 0;
            // Java: for(h = 0; start < end; h = 31 * h + str[start++])
            while start < end {
                h = h.wrapping_mul(31).wrapping_add(str[start] as i32);
                start += 1;
            }
            // Java: return h;
            h
        }
        // Java: else
        else {
            // Java: return 0;
            0
        }
    }

    // Java: char[] getChar()
    pub fn getChar(&self) -> &[char] {
        // Java: return this.mBuffer;
        &self.mBuffer
    }

    // Java: String getPersistPath()
    pub fn getPersistPath(&self) -> Option<&str> {
        // Java: return this.mPersistPath;
        self.mPersistPath.as_deref()
    }

    // Java: boolean isNameEqual(String name)
    pub fn isNameEqual(&self, name: &str) -> bool {
        // Java: int start = this.mURISpe == 0 ? 0 : this.mURISpe + 1;
        let start = if self.mURISpe == 0 { 0 } else { self.mURISpe + 1 };
        // Java: if (this.mTop - start == name.length())
        if self.mTop - start == name.len() {
            // Java: return this.getNameHashCode() == name.hashCode();
            let name_hash = Self::hashcode_char(&name.chars().collect::<Vec<_>>(), 0, name.len());
            self.getNameHashCode() == name_hash
        }
        // Java: else
        else {
            // Java: return false;
            false
        }
    }

    // Java: Integer toInteger()
    pub fn toInteger(&self) -> Option<i32> {
        // Java: int radix = 10;
        // Java: int result = 0;
        let mut result: i32 = 0;
        // Java: boolean negative = false;
        let mut negative = false;
        // Java: int i = 0;
        let mut i: usize = 0;
        // Java: int max = this.length();
        let max = self.length();
        // Java: if (max > 0)
        if max > 0 {
            // Java: int limit;
            let limit: i32;
            // Java: if (this.mBuffer[i] == '-')
            if self.mBuffer[i] == '-' {
                // Java: negative = true;
                negative = true;
                // Java: limit = Integer.MIN_VALUE;
                limit = i32::MIN;
                // Java: ++i;
                i += 1;
            }
            // Java: else
            else {
                // Java: limit = -2147483647;
                limit = -2147483647;
            }
            // Java: int multmin = limit / radix;
            let multmin = limit / 10;
            // Java: if (i < max)
            if i < max {
                // Java: int digit = Character.digit(this.mBuffer[i++], radix);
                let digit = self.mBuffer[i].to_digit(10).map(|d| d as i32).unwrap_or(-1);
                i += 1;
                // Java: if (digit < 0)
                if digit < 0 {
                    // Java: IOAssert.fail();
                    panic!("invalid digit");
                }
                // Java: else
                else {
                    // Java: result = -digit;
                    result = -digit;
                }
            }
            // Java: int digit;
            // Java: for(; i < max; result -= digit)
            while i < max {
                // Java: digit = Character.digit(this.mBuffer[i++], radix);
                let digit = self.mBuffer[i].to_digit(10).map(|d| d as i32).unwrap_or(-1);
                i += 1;
                // Java: if (digit < 0)
                if digit < 0 {
                    // Java: IOAssert.fail();
                    panic!("invalid digit");
                }
                // Java: if (result < multmin)
                if result < multmin {
                    // Java: IOAssert.fail();
                    panic!("overflow");
                }
                // Java: result *= radix;
                result *= 10;
                // Java: if (result < limit + digit)
                if result < limit + digit {
                    // Java: IOAssert.fail();
                    panic!("overflow");
                }
                result -= digit;
            }
        }
        // Java: else
        else {
            // Java: IOAssert.fail();
            panic!("empty string");
        }
        // Java: if (negative)
        if negative {
            // Java: if (i > 1)
            if i > 1 {
                // Java: return result;
                Some(result)
            }
            // Java: else
            else {
                // Java: IOAssert.fail();
                // Java: return null;
                None
            }
        }
        // Java: else
        else {
            // Java: return -result;
            Some(-result)
        }
    }

    // Java: boolean toBoolean()
    pub fn toBoolean(&self) -> bool {
        // Java: int hashCode = this.hashCode();
        let hash_code = self.hashCode();
        // Java: switch (hashCode)
        match hash_code {
            // Java: case 48:
            // Java: case 102:
            // Java: case 109935:
            // Java: case 97196323:
            48 | 102 | 109935 | 97196323 => {
                // Java: return false;
                false
            }
            // Java: default:
            _ => {
                // Java: return true;
                true
            }
        }
    }
}
