// Java: public class StringTokenizer implements Enumeration
pub struct StringTokenizer {
    // Java field: String string
    pub(crate) string: String,
    // Java field: String delimiters
    delimiters: Option<String>,
    // Java field: boolean returnDelimiters
    returnDelimiters: bool,
    // Java field: int position
    position: usize,
}

// Java: public class StringTokenizer implements Enumeration
impl StringTokenizer {
    // Java: void setString(String string)
    pub fn setString(&mut self, string: &str) {
        // Java: this.string = string;
        self.string = string.to_string();
        // Java: this.position = 0;
        self.position = 0;
    }

    // Java: StringTokenizer(String string)
    pub fn new(string: &str) -> Self {
        // Java: this(string, " \t\n\r\f", false);
        Self::new_with_delimiters_and_return(string, " \t\n\r\u{000C}", false)
    }

    // Java: StringTokenizer(String string, String delimiters)
    pub fn new_with_delimiters(string: &str, delimiters: &str) -> Self {
        // Java: this(string, delimiters, false);
        Self::new_with_delimiters_and_return(string, delimiters, false)
    }

    // Java: StringTokenizer(String string, String delimiters, boolean returnDelimiters)
    pub fn new_with_delimiters_and_return(string: &str, delimiters: &str, return_delimiters: bool) -> Self {
        // Java: super();
        // Java: this.string = string;
        // Java: this.delimiters = delimiters;
        // Java: this.returnDelimiters = returnDelimiters;
        // Java: this.position = 0;
        StringTokenizer {
            string: string.to_string(),
            delimiters: Some(delimiters.to_string()),
            returnDelimiters: return_delimiters,
            position: 0,
        }
    }

    // Java: int countTokens()
    pub fn countTokens(&self) -> i32 {
        // Java: int count = 0;
        let mut count: i32 = 0;
        // Java: boolean inToken = false;
        let mut in_token = false;
        // Java: int i = this.position;
        let mut i = self.position;
        let chars: Vec<char> = self.string.chars().collect();
        let delims = self.delimiters.as_deref().unwrap_or("");
        // Java: for(int length = this.string.length(); i < length; ++i)
        let length = chars.len();
        while i < length {
            // Java: if (this.delimiters.indexOf(this.string.charAt(i), 0) >= 0)
            if delims.contains(chars[i]) {
                // Java: if (this.returnDelimiters)
                if self.returnDelimiters {
                    // Java: ++count;
                    count += 1;
                }
                // Java: if (inToken)
                if in_token {
                    // Java: ++count;
                    count += 1;
                    // Java: inToken = false;
                    in_token = false;
                }
            }
            // Java: else
            else {
                // Java: inToken = true;
                in_token = true;
            }
            i += 1;
        }
        // Java: if (inToken)
        if in_token {
            // Java: ++count;
            count += 1;
        }
        // Java: return count;
        count
    }

    // Java: boolean hasMoreElements()
    pub fn hasMoreElements(&self) -> bool {
        // Java: return this.hasMoreTokens();
        self.hasMoreTokens()
    }

    // Java: boolean hasMoreTokens()
    pub fn hasMoreTokens(&self) -> bool {
        // Java: if (this.delimiters == null)
        if self.delimiters.is_none() {
            // Java: throw new NullPointerException();
            panic!("NullPointerException");
        }
        // Java: else
        {
            // Java: int length = this.string.length();
            let chars: Vec<char> = self.string.chars().collect();
            let length = chars.len();
            let delims = self.delimiters.as_deref().unwrap_or("");
            // Java: if (this.position < length)
            if self.position < length {
                // Java: if (this.returnDelimiters)
                if self.returnDelimiters {
                    // Java: return true;
                    return true;
                }
                // Java: for(int i = this.position; i < length; ++i)
                for i in self.position..length {
                    // Java: if (this.delimiters.indexOf(this.string.charAt(i), 0) == -1)
                    if !delims.contains(chars[i]) {
                        // Java: return true;
                        return true;
                    }
                }
            }
            // Java: return false;
            false
        }
    }

    // Java: Object nextElement()
    pub fn nextElement(&mut self) -> String {
        // Java: return this.nextToken();
        self.nextToken()
    }

    // Java: String nextToken()
    pub fn nextToken(&mut self) -> String {
        // Java: if (this.delimiters == null)
        if self.delimiters.is_none() {
            // Java: throw new NullPointerException();
            panic!("NullPointerException");
        }
        // Java: else
        {
            // Java: int i = this.position;
            let mut i = self.position;
            // Java: int length = this.string.length();
            let chars: Vec<char> = self.string.chars().collect();
            let length = chars.len();
            let delims = self.delimiters.clone().unwrap_or_default();
            // Java: if (i < length)
            if i < length {
                // Java: if (this.returnDelimiters)
                if self.returnDelimiters {
                    // Java: if (this.delimiters.indexOf(this.string.charAt(this.position), 0) >= 0)
                    if delims.contains(chars[self.position]) {
                        // Java: return String.valueOf(this.string.charAt(this.position++));
                        let ch = chars[self.position];
                        self.position += 1;
                        return ch.to_string();
                    }
                    // Java: ++this.position;
                    self.position += 1;
                    // Java: while(this.position < length)
                    while self.position < length {
                        // Java: if (this.delimiters.indexOf(this.string.charAt(this.position), 0) >= 0)
                        if delims.contains(chars[self.position]) {
                            // Java: return this.string.substring(i, this.position);
                            return chars[i..self.position].iter().collect();
                        }
                        // Java: ++this.position;
                        self.position += 1;
                    }
                    // Java: return this.string.substring(i);
                    return chars[i..].iter().collect();
                }
                // Java: while(i < length && this.delimiters.indexOf(this.string.charAt(i), 0) >= 0)
                while i < length && delims.contains(chars[i]) {
                    // Java: ++i;
                    i += 1;
                }
                // Java: this.position = i;
                self.position = i;
                // Java: if (i < length)
                if i < length {
                    // Java: ++this.position;
                    self.position += 1;
                    // Java: while(this.position < length)
                    while self.position < length {
                        // Java: if (this.delimiters.indexOf(this.string.charAt(this.position), 0) >= 0)
                        if delims.contains(chars[self.position]) {
                            // Java: return this.string.substring(i, this.position);
                            return chars[i..self.position].iter().collect();
                        }
                        // Java: ++this.position;
                        self.position += 1;
                    }
                    // Java: return this.string.substring(i);
                    return chars[i..].iter().collect();
                }
            }
            // Java: throw new NoSuchElementException();
            panic!("NoSuchElementException");
        }
    }

    // Java: String nextToken(String delims)
    pub fn nextToken_with_delims(&mut self, delims: &str) -> String {
        // Java: this.delimiters = delims;
        self.delimiters = Some(delims.to_string());
        // Java: return this.nextToken();
        self.nextToken()
    }
}
