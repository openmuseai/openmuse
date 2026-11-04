// Java: public class XmlValueBuilder
pub struct XmlValueBuilder {
    // Java field: char[] mBuffer
    pub(crate) mBuffer: Vec<char>,
    // Java field: int mSize
    mSize: usize,
    // Java field: int mTop
    pub(crate) mTop: usize,
}

// Java: public class XmlValueBuilder
impl XmlValueBuilder {
    // Java int-index / nullable byte[] facade, retaining the original bounds tests.
    // java public static int index(byte[] str,int start,int end,char ch){if(str!=null&&str.length>=end&&start>=0){while(start<end){if(str[start]==ch){return start;}++start;}return -1;}else{return -1;}}
    pub fn index_bytes__int(str:Option<&[u8]>,mut start:i32,end:i32,ch:u8)->i32{if let Some(str)=str.filter(|s|(s.len()as i64)>=end as i64&&start>=0){while start<end{if str[start as usize]==ch{return start;}start=start.wrapping_add(1);}-1}else{-1}}
    // java public static int hashcode(byte[] str,int start,int end){while(end>0&&str[end-1]==32){--end;}if(str!=null&&str.length>=end&&start>=0){int h;for(h=0;start<end;h=31*h+str[start++]){}return h;}else{return 0;}}
    pub fn hashcode_bytes__int(str:Option<&[u8]>,mut start:i32,mut end:i32)->i32{while end>0&&*str.expect("NullPointerException").get(end.wrapping_sub(1)as usize).expect("ArrayIndexOutOfBoundsException")==32{end=end.wrapping_sub(1);}if let Some(str)=str.filter(|s|(s.len()as i64)>=end as i64&&start>=0){let mut h=0i32;while start<end{h=h.wrapping_mul(31).wrapping_add(str[start as usize]as i8 as i32);start=start.wrapping_add(1);}h}else{0}}
    // Java: XmlValueBuilder()
    pub fn new() -> Self {
        // Java: this(32);
        Self::new_with_size(32)
    }

    // Java: XmlValueBuilder(int size)
    pub fn new_with_size(size: usize) -> Self {
        // Java: super();
        // Java: this.mBuffer = new char[size];
        // Java: this.mSize = size;
        // Java: this.mTop = 0;
        XmlValueBuilder {
            mBuffer: vec!['\0'; size],
            mSize: size,
            mTop: 0,
        }
    }

    // Java: void append(char ch)
    pub fn append_char(&mut self, ch: char) {
        // Java: if (this.mTop >= this.mSize)
        if self.mTop >= self.mSize {
            // Java: int newSize = this.mSize * 2;
            let new_size = self.mSize * 2;
            // Java: char[] tmp = new char[newSize];
            // Java: System.arraycopy(this.mBuffer, 0, tmp, 0, this.mTop);
            self.mBuffer.resize(new_size, '\0');
            // Java: this.mBuffer = tmp;
            // Java: this.mSize = newSize;
            self.mSize = new_size;
        }
        // Java: this.mBuffer[this.mTop++] = ch;
        self.mBuffer[self.mTop] = ch;
        self.mTop += 1;
    }

    // Java: void append(String str)
    pub fn append_str(&mut self, str: &str) {
        // Java: if (str != null && str.length() != 0)
        if !str.is_empty() {
            // Java: int i = 0;
            // Java: for(int size = str.length(); i < size; ++i)
            for ch in str.chars() {
                // Java: this.append(str.charAt(i));
                self.append_char(ch);
            }
        }
    }

    // Java: int length()
    pub fn length(&self) -> i32 {
        // Java: return this.mTop;
        self.mTop as i32
    }

    // Java: char charAt(int index)
    pub fn char_at(&self, index: usize) -> char {
        // Java: return this.mBuffer[index];
        self.mBuffer[index]
    }

    // Java: void clear()
    pub fn clear(&mut self) {
        // Java: this.mTop = 0;
        self.mTop = 0;
    }

    // Java: int getPrefixHashCode(int defaultValue)
    pub fn get_prefix_hash_code(&self, default_value: i32) -> i32 {
        // Java: int pos = index((char[])this.mBuffer, 0, this.mTop, ':');
        let pos = Self::index_chars(&self.mBuffer, 0, self.mTop, ':');
        // Java: return pos == -1 ? defaultValue : hashcode((char[])this.mBuffer, 0, pos);
        if pos == -1 { default_value } else { Self::hashcode_chars(&self.mBuffer, 0, pos as usize) }
    }

    // Java: int getNameHashCode()
    pub fn get_name_hash_code(&self) -> i32 {
        // Java: int start = 0;
        let mut start = 0usize;
        // Java: int pos = index((char[])this.mBuffer, 0, this.mTop, ':');
        let pos = Self::index_chars(&self.mBuffer, 0, self.mTop, ':');
        // Java: if (pos != -1)
        if pos != -1 {
            // Java: start = pos + 1;
            start = (pos + 1) as usize;
        }
        // Java: return hashcode(this.mBuffer, start, this.mTop);
        Self::hashcode_chars(&self.mBuffer, start, self.mTop)
    }

    // Java: char getEscape(char defaultValue)
    pub fn get_escape(&self, default_value: char) -> char {
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

    // Java: XmlValueBuilder backup()
    pub fn backup(&self) -> XmlValueBuilder {
        // Java: XmlValueBuilder obj = new XmlValueBuilder(this.mTop);
        let mut obj = XmlValueBuilder::new_with_size(self.mTop);
        // Java: System.arraycopy(this.mBuffer, 0, obj.mBuffer, 0, this.mTop);
        obj.mBuffer[..self.mTop].copy_from_slice(&self.mBuffer[..self.mTop]);
        // Java: obj.mTop = this.mTop;
        obj.mTop = self.mTop;
        // Java: return obj;
        obj
    }

    // Java: void restore(XmlValueBuilder obj)
    pub fn restore(&mut self, obj: &XmlValueBuilder) {
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
    }

    // Java: String toString()
    pub fn to_string(&self) -> String {
        // Java: return new String(this.mBuffer, 0, this.mTop);
        self.mBuffer[..self.mTop].iter().collect()
    }

    // Java: boolean isCDATAEnd()
    pub fn is_cdata_end(&self) -> bool {
        // Java: return this.mTop >= 12 && this.mBuffer[0] == '<' && this.mBuffer[1] == '!' && this.mBuffer[2] == '[' && this.mBuffer[this.mTop - 3] == ']' && this.mBuffer[this.mTop - 2] == ']' && this.mBuffer[this.mTop - 1] == '>';
        self.mTop >= 12
            && self.mBuffer[0] == '<'
            && self.mBuffer[1] == '!'
            && self.mBuffer[2] == '['
            && self.mBuffer[self.mTop - 3] == ']'
            && self.mBuffer[self.mTop - 2] == ']'
            && self.mBuffer[self.mTop - 1] == '>'
    }

    // Java: String toCDATAString()
    pub fn to_cdata_string(&self) -> String {
        // Java: return new String(this.mBuffer, 9, this.mTop - 12);
        self.mBuffer[9..9 + (self.mTop - 12)].iter().collect()
    }

    // Java: boolean isAnnotationEnd()
    pub fn is_annotation_end(&self) -> bool {
        // Java: return this.mTop >= 7 && this.mBuffer[0] == '<' && this.mBuffer[1] == '!' && this.mBuffer[2] == '-' && this.mBuffer[3] == '-' && this.mBuffer[this.mTop - 3] == '-' && this.mBuffer[this.mTop - 2] == '-' && this.mBuffer[this.mTop - 1] == '>';
        self.mTop >= 7
            && self.mBuffer[0] == '<'
            && self.mBuffer[1] == '!'
            && self.mBuffer[2] == '-'
            && self.mBuffer[3] == '-'
            && self.mBuffer[self.mTop - 3] == '-'
            && self.mBuffer[self.mTop - 2] == '-'
            && self.mBuffer[self.mTop - 1] == '>'
    }

    // Java: static int index(char[] str, int start, int end, char ch)
    pub fn index_chars(str: &[char], start: usize, end: usize, ch: char) -> i32 {
        // Java: if (str != null && str.length >= end && start >= 0)
        if str.len() >= end {
            // Java: while(start < end)
            let mut s = start;
            while s < end {
                // Java: if (str[start] == ch)
                if str[s] == ch {
                    // Java: return start;
                    return s as i32;
                }
                // Java: ++start;
                s += 1;
            }
            // Java: return -1;
            return -1;
        }
        // Java: else
        // Java: return -1;
        -1
    }

    // Java: static int index(byte[] str, int start, int end, char ch)
    pub fn index_bytes(str: &[u8], start: usize, end: usize, ch: char) -> i32 {
        // Java: if (str != null && str.length >= end && start >= 0)
        if str.len() >= end {
            // Java: while(start < end)
            let mut s = start;
            while s < end {
                // Java: if (str[start] == ch)
                if str[s] == ch as u8 {
                    // Java: return start;
                    return s as i32;
                }
                // Java: ++start;
                s += 1;
            }
            // Java: return -1;
            return -1;
        }
        // Java: else
        // Java: return -1;
        -1
    }

    // Java: static int hashcode(char[] str, int start, int end)
    pub fn hashcode_chars(str: &[char], start: usize, end: usize) -> i32 {
        let mut end = end;
        // Java: while(end > 0 && str[end - 1] == ' ')
        while end > 0 && str[end - 1] == ' ' {
            // Java: --end;
            end -= 1;
        }
        // Java: if (str != null && str.length >= end && start >= 0)
        if str.len() >= end {
            // Java: int h;
            let mut h: i32 = 0;
            // Java: for(h = 0; start < end; h = 31 * h + str[start++])
            let mut s = start;
            while s < end {
                h = h.wrapping_mul(31).wrapping_add(str[s] as i32);
                s += 1;
            }
            // Java: return h;
            return h;
        }
        // Java: else
        // Java: return 0;
        0
    }

    // Java: static int hashcode(byte[] str, int start, int end)
    pub fn hashcode_bytes(str: &[u8], start: usize, end: usize) -> i32 {
        let mut end = end;
        // Java: while(end > 0 && str[end - 1] == 32)
        while end > 0 && str[end - 1] == 32 {
            // Java: --end;
            end -= 1;
        }
        // Java: if (str != null && str.length >= end && start >= 0)
        if str.len() >= end {
            // Java: int h;
            let mut h: i32 = 0;
            // Java: for(h = 0; start < end; h = 31 * h + str[start++])
            let mut s = start;
            while s < end {
                h = h.wrapping_mul(31).wrapping_add(str[s] as i32);
                s += 1;
            }
            // Java: return h;
            return h;
        }
        // Java: else
        // Java: return 0;
        0
    }

    // Java: static Integer parserHexToInt(char[] chars, int start, int end, int radix)
    pub fn parser_hex_to_int(chars: &[char], start: usize, end: usize, radix: u32) -> Option<i32> {
        // Java: try
        match Self::hex_to_int(chars, start, end, radix) {
            // Java: return hexToInt(chars, start, end, radix);
            Ok(v) => Some(v),
            // Java: catch (Throwable var5)
            // Java: return null;
            Err(_) => None,
        }
    }

    // Java: static int hexToInt(char[] chars, int start, int end, int radix)
    fn hex_to_int(chars: &[char], start: usize, end: usize, radix: u32) -> Result<i32, String> {
        // Java: if (radix >= 2 && radix <= 36)
        if radix >= 2 && radix <= 36 {
            // Java: int i = 0;
            // Java: if (end - start == 0)
            if end <= start {
                // Java: invalidInt(new String(chars, start, end));
                return Err(format!("Invalid int: \"{}\"", chars[start..end].iter().collect::<String>()));
            }
            // Java: int max = Integer.MIN_VALUE / radix;
            let max = i32::MIN / radix as i32;
            // Java: int result;
            let mut result: i32 = 0;
            // Java: int next;
            let mut s = start;
            // Java: for(result = 0; start < end; result = next)
            while s < end {
                // Java: int digit = Character.digit(chars[start++], radix);
                let digit = chars[s].to_digit(radix).map(|d| d as i32).unwrap_or(-1);
                s += 1;
                // Java: if (digit == -1)
                if digit == -1 {
                    // Java: invalidInt(new String(chars, start, end));
                    return Err(format!("Invalid int: \"{}\"", chars[start..end].iter().collect::<String>()));
                }
                // Java: if (max > result)
                if max > result {
                    // Java: invalidInt(new String(chars, start, end));
                    return Err(format!("Invalid int: \"{}\"", chars[start..end].iter().collect::<String>()));
                }
                // Java: next = result * radix - digit;
                let next = result * radix as i32 - digit;
                // Java: if (next > result)
                if next > result {
                    // Java: invalidInt(new String(chars, start, end));
                    return Err(format!("Invalid int: \"{}\"", chars[start..end].iter().collect::<String>()));
                }
                result = next;
            }
            // Java: result = -result;
            result = -result;
            // Java: if (result < 0)
            if result < 0 {
                // Java: invalidInt(new String(chars, start, end));
                return Err(format!("Invalid int: \"{}\"", chars[start..end].iter().collect::<String>()));
            }
            // Java: return result;
            return Ok(result);
        }
        // Java: else
        // Java: throw new NumberFormatException("Invalid radix: " + radix);
        Err(format!("Invalid radix: {}", radix))
    }

    // Java: static NumberFormatException invalidInt(String s)
    fn invalid_int(s: &str) -> String {
        // Java: throw new NumberFormatException("Invalid int: \"" + s + "\"");
        format!("Invalid int: \"{}\"", s)
    }
}
