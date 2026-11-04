// Java: public class Token
pub struct Token {
    // Java field: int tokenType
    pub(crate) tokenType: i32,
    // Java field: String parseText
    pub(crate) parseText: String,
    // Java field: int tokenBegin
    pub(crate) tokenBegin: i32,
    // Java field: int tokenEnd
    pub(crate) tokenEnd: i32,
}

// Java: public class Token
impl Token {
    // Java: Token(int tokenType, String parseText, int tokenBegin, int tokenEnd)
    pub fn new(tokenType: i32, parseText: String, tokenBegin: i32, tokenEnd: i32) -> Self {
      // Java: super();
      // Java: this.setTokenType(tokenType);
      // Java: this.setParseText(parseText);
      // Java: this.setTokenBegin(tokenBegin);
      // Java: this.setTokenEnd(tokenEnd);
      Token { tokenType, parseText, tokenBegin, tokenEnd }
    }

    // Java: void setTokenType(int tokenType)
    fn setTokenType(&mut self, tokenType: i32) {
      // Java: this.tokenType = tokenType;
      self.tokenType = tokenType;
    }

    // Java: int getTokenType()
    pub fn getTokenType(&self) -> i32 {
      // Java: return this.tokenType;
      self.tokenType
    }

    // Java: void setParseText(String parseText)
    fn setParseText(&mut self, parseText: String) {
      // Java: this.parseText = parseText;
      self.parseText = parseText;
    }

    // Java: String getTokenText()
    pub fn getTokenText(&self) -> &str {
      // Java: return this.parseText.substring(this.getTokenBegin(), this.getTokenEnd());
      &self.parseText[self.tokenBegin as usize..self.tokenEnd as usize]
    }

    // Java: void setTokenBegin(int tokenBegin)
    fn setTokenBegin(&mut self, tokenBegin: i32) {
      // Java: this.tokenBegin = tokenBegin;
      self.tokenBegin = tokenBegin;
    }

    // Java: int getTokenBegin()
    pub fn getTokenBegin(&self) -> i32 {
      // Java: return this.tokenBegin;
      self.tokenBegin
    }

    // Java: void setTokenEnd(int tokenEnd)
    fn setTokenEnd(&mut self, tokenEnd: i32) {
      // Java: this.tokenEnd = tokenEnd;
      self.tokenEnd = tokenEnd;
    }

    // Java: int getTokenEnd()
    pub fn getTokenEnd(&self) -> i32 {
      // Java: return this.tokenEnd;
      self.tokenEnd
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
      // Java: return "[ (" + this.tokenType + ") (" + this.getTokenText() + ")";
      format!("[ ({}) ({})", self.tokenType, self.getTokenText())
    }
}
